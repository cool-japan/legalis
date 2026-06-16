//! Customizable output verbosity for the Legalis CLI.
//!
//! The CLI already exposes a `-v`/`--verbose` counter (for tracing log levels)
//! and a `--quiet` flag. This module turns those raw inputs into a single,
//! ordered [`Verbosity`] value that *commands* can consult to decide how much
//! human-facing detail to print, independently of the `tracing` log filter.
//!
//! The level is also influenced by the environment so that scripts and CI can
//! pin verbosity without touching flags:
//!
//! - `LEGALIS_VERBOSITY=silent|quiet|normal|verbose|debug|trace`
//! - `LEGALIS_QUIET=1` forces [`Verbosity::Quiet`]
//!
//! Precedence (highest wins): explicit `--quiet` flag → `-v` count (when > 0)
//! → `LEGALIS_VERBOSITY` → `LEGALIS_QUIET` → default ([`Verbosity::Normal`]).

use std::fmt;
use std::str::FromStr;
use std::sync::OnceLock;

/// Process-global resolved verbosity, set once early in `main`.
static GLOBAL_VERBOSITY: OnceLock<Verbosity> = OnceLock::new();

/// Installs the process-global verbosity (no-op if already set).
pub fn set_global(verbosity: Verbosity) {
    let _ = GLOBAL_VERBOSITY.set(verbosity);
}

/// Returns the process-global verbosity, defaulting to [`Verbosity::Normal`]
/// when it has not been installed (e.g. in unit tests).
pub fn global() -> Verbosity {
    GLOBAL_VERBOSITY.get().copied().unwrap_or_default()
}

/// Ordered verbosity levels controlling human-facing output detail.
///
/// The ordering is meaningful: `Silent < Quiet < Normal < Verbose < Debug <
/// Trace`. Use the [`PartialOrd`]/[`Ord`] comparisons (or the convenience
/// predicates) to gate output, e.g. `if verbosity >= Verbosity::Verbose { ... }`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Verbosity {
    /// Suppress everything except hard errors (and explicitly requested data).
    Silent,
    /// Errors and warnings only; suppress progress/info chatter.
    Quiet,
    /// The default: status, results, warnings, and errors.
    #[default]
    Normal,
    /// Adds extra contextual detail (per-item progress, timings, hints).
    Verbose,
    /// Adds internal diagnostic detail useful when debugging behaviour.
    Debug,
    /// Maximum detail, including low-level traces.
    Trace,
}

impl Verbosity {
    /// All levels in ascending order (useful for tests and iteration).
    pub const ALL: [Verbosity; 6] = [
        Verbosity::Silent,
        Verbosity::Quiet,
        Verbosity::Normal,
        Verbosity::Verbose,
        Verbosity::Debug,
        Verbosity::Trace,
    ];

    /// Resolves the effective verbosity from CLI flags and the environment.
    ///
    /// `verbose_count` is the raw `-v` repetition count and `quiet` is the
    /// `--quiet` flag. See the module docs for the precedence rules.
    pub fn resolve(verbose_count: u8, quiet: bool) -> Self {
        if quiet {
            return Verbosity::Quiet;
        }
        if verbose_count > 0 {
            return Self::from_verbose_count(verbose_count);
        }
        if let Ok(raw) = std::env::var("LEGALIS_VERBOSITY")
            && let Ok(level) = raw.parse::<Verbosity>()
        {
            return level;
        }
        if std::env::var("LEGALIS_QUIET").is_ok() {
            return Verbosity::Quiet;
        }
        Verbosity::Normal
    }

    /// Maps a raw `-v` repetition count onto a level (`0` stays [`Normal`]).
    ///
    /// `1 => Verbose`, `2 => Debug`, `>=3 => Trace`.
    pub fn from_verbose_count(count: u8) -> Self {
        match count {
            0 => Verbosity::Normal,
            1 => Verbosity::Verbose,
            2 => Verbosity::Debug,
            _ => Verbosity::Trace,
        }
    }

    /// The canonical lowercase name (round-trips through [`FromStr`]).
    pub fn as_str(self) -> &'static str {
        match self {
            Verbosity::Silent => "silent",
            Verbosity::Quiet => "quiet",
            Verbosity::Normal => "normal",
            Verbosity::Verbose => "verbose",
            Verbosity::Debug => "debug",
            Verbosity::Trace => "trace",
        }
    }

    /// Whether normal status/info lines should be printed.
    pub fn shows_status(self) -> bool {
        self >= Verbosity::Normal
    }

    /// Whether warnings should be printed.
    pub fn shows_warnings(self) -> bool {
        self >= Verbosity::Quiet
    }

    /// Whether extra/verbose detail should be printed.
    pub fn shows_detail(self) -> bool {
        self >= Verbosity::Verbose
    }

    /// Whether debug-level diagnostic detail should be printed.
    pub fn shows_debug(self) -> bool {
        self >= Verbosity::Debug
    }

    /// Whether maximum-detail trace output should be printed.
    pub fn shows_trace(self) -> bool {
        self >= Verbosity::Trace
    }

    /// The `tracing` env-filter directive that best matches this level.
    pub fn tracing_directive(self) -> &'static str {
        match self {
            Verbosity::Silent => "error",
            Verbosity::Quiet => "error",
            Verbosity::Normal => "warn",
            Verbosity::Verbose => "info",
            Verbosity::Debug => "debug",
            Verbosity::Trace => "trace",
        }
    }
}

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Error returned when parsing an unknown verbosity name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseVerbosityError(String);

impl fmt::Display for ParseVerbosityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unknown verbosity '{}' (expected one of: silent, quiet, normal, verbose, debug, trace)",
            self.0
        )
    }
}

impl std::error::Error for ParseVerbosityError {}

impl FromStr for Verbosity {
    type Err = ParseVerbosityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "silent" | "0" => Ok(Verbosity::Silent),
            "quiet" | "1" => Ok(Verbosity::Quiet),
            "normal" | "info" | "2" => Ok(Verbosity::Normal),
            "verbose" | "3" => Ok(Verbosity::Verbose),
            "debug" | "4" => Ok(Verbosity::Debug),
            "trace" | "5" => Ok(Verbosity::Trace),
            other => Err(ParseVerbosityError(other.to_string())),
        }
    }
}

/// A clap value enum mirroring [`Verbosity`] for an explicit `--verbosity` flag.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum VerbosityArg {
    /// Suppress everything except hard errors.
    Silent,
    /// Errors and warnings only.
    Quiet,
    /// Default verbosity.
    #[default]
    Normal,
    /// Extra contextual detail.
    Verbose,
    /// Internal diagnostic detail.
    Debug,
    /// Maximum detail.
    Trace,
}

impl From<VerbosityArg> for Verbosity {
    fn from(arg: VerbosityArg) -> Self {
        match arg {
            VerbosityArg::Silent => Verbosity::Silent,
            VerbosityArg::Quiet => Verbosity::Quiet,
            VerbosityArg::Normal => Verbosity::Normal,
            VerbosityArg::Verbose => Verbosity::Verbose,
            VerbosityArg::Debug => Verbosity::Debug,
            VerbosityArg::Trace => Verbosity::Trace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run a closure with a set of environment variables removed, restoring the
    /// previous values afterwards. Avoids cross-test contamination.
    fn with_clean_env<F: FnOnce()>(keys: &[&str], f: F) {
        let saved: Vec<(String, Option<String>)> = keys
            .iter()
            .map(|k| ((*k).to_string(), std::env::var(k).ok()))
            .collect();
        for k in keys {
            unsafe {
                std::env::remove_var(k);
            }
        }
        f();
        for (k, v) in saved {
            unsafe {
                match v {
                    Some(value) => std::env::set_var(&k, value),
                    None => std::env::remove_var(&k),
                }
            }
        }
    }

    #[test]
    fn test_ordering() {
        assert!(Verbosity::Silent < Verbosity::Quiet);
        assert!(Verbosity::Quiet < Verbosity::Normal);
        assert!(Verbosity::Normal < Verbosity::Verbose);
        assert!(Verbosity::Verbose < Verbosity::Debug);
        assert!(Verbosity::Debug < Verbosity::Trace);
    }

    #[test]
    fn test_default_is_normal() {
        assert_eq!(Verbosity::default(), Verbosity::Normal);
    }

    #[test]
    fn test_from_verbose_count() {
        assert_eq!(Verbosity::from_verbose_count(0), Verbosity::Normal);
        assert_eq!(Verbosity::from_verbose_count(1), Verbosity::Verbose);
        assert_eq!(Verbosity::from_verbose_count(2), Verbosity::Debug);
        assert_eq!(Verbosity::from_verbose_count(3), Verbosity::Trace);
        assert_eq!(Verbosity::from_verbose_count(99), Verbosity::Trace);
    }

    #[test]
    fn test_quiet_flag_wins() {
        with_clean_env(&["LEGALIS_VERBOSITY", "LEGALIS_QUIET"], || {
            assert_eq!(Verbosity::resolve(3, true), Verbosity::Quiet);
        });
    }

    #[test]
    fn test_verbose_count_over_env() {
        with_clean_env(&["LEGALIS_VERBOSITY", "LEGALIS_QUIET"], || {
            unsafe {
                std::env::set_var("LEGALIS_VERBOSITY", "trace");
            }
            // Explicit -v count beats the env var.
            assert_eq!(Verbosity::resolve(1, false), Verbosity::Verbose);
            unsafe {
                std::env::remove_var("LEGALIS_VERBOSITY");
            }
        });
    }

    #[test]
    fn test_env_verbosity() {
        with_clean_env(&["LEGALIS_VERBOSITY", "LEGALIS_QUIET"], || {
            unsafe {
                std::env::set_var("LEGALIS_VERBOSITY", "debug");
            }
            assert_eq!(Verbosity::resolve(0, false), Verbosity::Debug);
            unsafe {
                std::env::remove_var("LEGALIS_VERBOSITY");
            }
        });
    }

    #[test]
    fn test_env_quiet_fallback() {
        with_clean_env(&["LEGALIS_VERBOSITY", "LEGALIS_QUIET"], || {
            unsafe {
                std::env::set_var("LEGALIS_QUIET", "1");
            }
            assert_eq!(Verbosity::resolve(0, false), Verbosity::Quiet);
            unsafe {
                std::env::remove_var("LEGALIS_QUIET");
            }
        });
    }

    #[test]
    fn test_default_resolution() {
        with_clean_env(&["LEGALIS_VERBOSITY", "LEGALIS_QUIET"], || {
            assert_eq!(Verbosity::resolve(0, false), Verbosity::Normal);
        });
    }

    #[test]
    fn test_predicates() {
        assert!(!Verbosity::Silent.shows_status());
        assert!(!Verbosity::Silent.shows_warnings());
        assert!(Verbosity::Quiet.shows_warnings());
        assert!(!Verbosity::Quiet.shows_status());
        assert!(Verbosity::Normal.shows_status());
        assert!(!Verbosity::Normal.shows_detail());
        assert!(Verbosity::Verbose.shows_detail());
        assert!(!Verbosity::Verbose.shows_debug());
        assert!(Verbosity::Debug.shows_debug());
        assert!(!Verbosity::Debug.shows_trace());
        assert!(Verbosity::Trace.shows_trace());
    }

    #[test]
    fn test_parse_roundtrip() {
        for level in Verbosity::ALL {
            let parsed: Verbosity = level.as_str().parse().expect("canonical name should parse");
            assert_eq!(parsed, level);
        }
    }

    #[test]
    fn test_parse_aliases_and_errors() {
        assert_eq!("INFO".parse::<Verbosity>(), Ok(Verbosity::Normal));
        assert_eq!("  Trace  ".parse::<Verbosity>(), Ok(Verbosity::Trace));
        assert_eq!("4".parse::<Verbosity>(), Ok(Verbosity::Debug));
        assert!("nonsense".parse::<Verbosity>().is_err());
    }

    #[test]
    fn test_tracing_directive() {
        assert_eq!(Verbosity::Silent.tracing_directive(), "error");
        assert_eq!(Verbosity::Normal.tracing_directive(), "warn");
        assert_eq!(Verbosity::Verbose.tracing_directive(), "info");
        assert_eq!(Verbosity::Trace.tracing_directive(), "trace");
    }

    #[test]
    fn test_arg_conversion() {
        assert_eq!(Verbosity::from(VerbosityArg::Silent), Verbosity::Silent);
        assert_eq!(Verbosity::from(VerbosityArg::Trace), Verbosity::Trace);
        assert_eq!(Verbosity::from(VerbosityArg::Normal), Verbosity::Normal);
    }
}
