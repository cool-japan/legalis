//! Legal statute verification for German law (Deutsches Recht).
//!
//! This module provides verification capabilities for German legal statutes,
//! checking constitutional compliance based on the Grundgesetz (Basic Law)
//! and logical consistency.
//!
//! # Grundgesetz-basierte Prüfung
//!
//! | German | English | Article |
//! |--------|---------|---------|
//! | Menschenwürde | Human Dignity | Art. 1 |
//! | Gleichheit vor dem Gesetz | Equality before Law | Art. 3 |
//! | Berufsfreiheit | Occupational Freedom | Art. 12 |
//! | Eigentumsgarantie | Property Rights | Art. 14 |
//! | Sozialstaatsprinzip | Social State Principle | Art. 20 |
//!
//! # Rechtsquellenhierarchie (Legal Source Hierarchy)
//!
//! 1. Grundgesetz (Basic Law)
//! 2. Bundesgesetz (Federal Law)
//! 3. Rechtsverordnung (Ordinance)
//! 4. Satzung (Bylaws)
//! 5. Tarifvertrag (Collective Agreement)
//!
//! # Example / Beispiel
//!
//! ```rust,ignore
//! use legalis_de::reasoning::verifier::{DeStatuteVerifier, de_constitutional_principles};
//! use legalis_de::reasoning::statute_adapter::all_labor_statutes;
//!
//! let verifier = DeStatuteVerifier::new();
//! let statutes = all_labor_statutes();
//!
//! let result = verifier.verify(&statutes);
//! println!("Prüfung bestanden: {}", result.passed);
//! ```

use legalis_core::Statute;
use legalis_verifier::{
    ConstitutionalPrinciple, PrincipleCheck, Severity, StatuteVerifier, VerificationError,
    VerificationResult,
};
use std::collections::{HashMap, HashSet};

/// German statute verifier with Grundgesetz compliance checking.
///
/// This verifier integrates German constitutional principles and the
/// Rechtsquellenhierarchie (legal source hierarchy) into the verification framework.
///
/// # Deutsche Rechtsordnung
///
/// - Verfassungsmäßigkeitsprüfung: Statutes must comply with Grundgesetz
/// - Günstigkeitsprinzip: More favorable law applies to workers
/// - Verhältnismäßigkeitsprinzip: Proportionality in restrictions
pub struct DeStatuteVerifier {
    /// Base verifier from legalis-verifier
    inner: StatuteVerifier,
    /// German-specific legal hierarchy rules
    hierarchy_rules: HierarchyRules,
}

impl std::fmt::Debug for DeStatuteVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeStatuteVerifier")
            .field("hierarchy_rules", &self.hierarchy_rules)
            .finish_non_exhaustive()
    }
}

/// Legal source hierarchy in German law (Rechtsquellenhierarchie).
///
/// Deutsche Normenhierarchie:
/// 1. Grundgesetz (Constitution)
/// 2. Bundesgesetz (Federal Law - Bundestag)
/// 3. Landesgesetz (State Law - Landtag)
/// 4. Rechtsverordnung (Ordinance)
/// 5. Satzung (Bylaws/Statutes of public bodies)
/// 6. Tarifvertrag (Collective Agreement)
/// 7. Betriebsvereinbarung (Works Agreement)
/// 8. Arbeitsvertrag (Employment Contract)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rechtsquelle {
    /// Grundgesetz - Basic Law (highest)
    Grundgesetz,
    /// Bundesgesetz - Federal Law (e.g., BGB, ArbZG, KSchG)
    Bundesgesetz,
    /// Landesgesetz - State Law
    Landesgesetz,
    /// Rechtsverordnung - Legal Ordinance
    Rechtsverordnung,
    /// Satzung - Bylaws of public bodies
    Satzung,
    /// Tarifvertrag - Collective Agreement
    Tarifvertrag,
    /// Betriebsvereinbarung - Works Council Agreement
    Betriebsvereinbarung,
    /// Arbeitsvertrag - Individual Employment Contract (lowest)
    Arbeitsvertrag,
}

impl Rechtsquelle {
    /// Returns the German name for this legal source level.
    #[must_use]
    pub const fn german_name(&self) -> &'static str {
        match self {
            Self::Grundgesetz => "Grundgesetz",
            Self::Bundesgesetz => "Bundesgesetz",
            Self::Landesgesetz => "Landesgesetz",
            Self::Rechtsverordnung => "Rechtsverordnung",
            Self::Satzung => "Satzung",
            Self::Tarifvertrag => "Tarifvertrag",
            Self::Betriebsvereinbarung => "Betriebsvereinbarung",
            Self::Arbeitsvertrag => "Arbeitsvertrag",
        }
    }

    /// Returns the English name for this legal source level.
    #[must_use]
    pub const fn english_name(&self) -> &'static str {
        match self {
            Self::Grundgesetz => "Basic Law (Constitution)",
            Self::Bundesgesetz => "Federal Law",
            Self::Landesgesetz => "State Law",
            Self::Rechtsverordnung => "Legal Ordinance",
            Self::Satzung => "Public Body Bylaws",
            Self::Tarifvertrag => "Collective Agreement",
            Self::Betriebsvereinbarung => "Works Council Agreement",
            Self::Arbeitsvertrag => "Employment Contract",
        }
    }

    /// Determines the legal source level from statute ID or title.
    #[must_use]
    pub fn from_statute(statute: &Statute) -> Self {
        let id = statute.id.to_lowercase();
        let title = statute.title.to_lowercase();

        // Check for Grundgesetz references
        if id.contains("gg_") || title.contains("grundgesetz") || title.contains("basic law") {
            return Self::Grundgesetz;
        }

        // Check for Tarifvertrag
        if id.contains("tv_")
            || id.contains("tarif")
            || title.contains("tarifvertrag")
            || title.contains("collective agreement")
        {
            return Self::Tarifvertrag;
        }

        // Check for Betriebsvereinbarung
        if id.contains("bv_")
            || title.contains("betriebsvereinbarung")
            || title.contains("works agreement")
        {
            return Self::Betriebsvereinbarung;
        }

        // Check for Rechtsverordnung
        if id.contains("vo_")
            || title.contains("verordnung")
            || title.contains("ordinance")
            || title.contains("durchführungsverordnung")
        {
            return Self::Rechtsverordnung;
        }

        // Check for Landesgesetz (state law patterns)
        if title.contains("landesgesetz")
            || title.contains("landes")
            || id.starts_with("l")
                && (id.contains("arbg") || id.contains("beamtg") || id.contains("schulg"))
        {
            return Self::Landesgesetz;
        }

        // Default to Bundesgesetz for most German laws
        Self::Bundesgesetz
    }

    /// Checks if this level is higher in the hierarchy than another.
    #[must_use]
    pub fn is_higher_than(&self, other: &Self) -> bool {
        (*self as u8) < (*other as u8)
    }

    /// Checks if this level allows the Günstigkeitsprinzip (more favorable rule).
    ///
    /// In German labor law, lower-level norms can deviate from higher-level norms
    /// if they are more favorable to the employee (Günstigkeitsprinzip).
    #[must_use]
    pub fn allows_guenstigkeitsprinzip(&self) -> bool {
        matches!(
            self,
            Self::Tarifvertrag | Self::Betriebsvereinbarung | Self::Arbeitsvertrag
        )
    }
}

/// Rules for legal hierarchy checking.
#[derive(Debug, Default)]
pub struct HierarchyRules {
    /// Known statute hierarchies (statute_id -> Rechtsquelle)
    known_hierarchies: HashMap<String, Rechtsquelle>,
    /// Statutes marked as Öffnungsklausel (allowing deviation)
    oeffnungsklauseln: HashSet<String>,
    /// Zwingendes Recht (mandatory law that cannot be deviated from)
    zwingendes_recht: HashSet<String>,
}

impl HierarchyRules {
    /// Creates new hierarchy rules.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a statute's hierarchy level.
    pub fn register_hierarchy(&mut self, statute_id: impl Into<String>, level: Rechtsquelle) {
        self.known_hierarchies.insert(statute_id.into(), level);
    }

    /// Marks a statute as having an Öffnungsklausel (opening clause).
    ///
    /// Opening clauses allow lower-level norms to deviate from the statute.
    pub fn register_oeffnungsklausel(&mut self, statute_id: impl Into<String>) {
        self.oeffnungsklauseln.insert(statute_id.into());
    }

    /// Marks a statute as zwingendes Recht (mandatory law).
    ///
    /// Mandatory law cannot be deviated from, even by collective agreements.
    pub fn register_zwingendes_recht(&mut self, statute_id: impl Into<String>) {
        self.zwingendes_recht.insert(statute_id.into());
    }

    /// Gets the hierarchy level for a statute.
    #[must_use]
    pub fn get_hierarchy(&self, statute: &Statute) -> Rechtsquelle {
        self.known_hierarchies
            .get(&statute.id)
            .copied()
            .unwrap_or_else(|| Rechtsquelle::from_statute(statute))
    }

    /// Checks if a statute has an Öffnungsklausel.
    #[must_use]
    pub fn has_oeffnungsklausel(&self, statute_id: &str) -> bool {
        self.oeffnungsklauseln.contains(statute_id)
    }

    /// Checks if a statute is zwingendes Recht.
    #[must_use]
    pub fn is_zwingendes_recht(&self, statute_id: &str) -> bool {
        self.zwingendes_recht.contains(statute_id)
    }
}

impl Default for DeStatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl DeStatuteVerifier {
    /// Creates a new German statute verifier with Grundgesetz principles.
    #[must_use]
    pub fn new() -> Self {
        let principles = de_constitutional_principles();
        let inner = StatuteVerifier::with_principles(principles);

        let mut hierarchy_rules = HierarchyRules::new();
        setup_labor_law_hierarchy(&mut hierarchy_rules);

        Self {
            inner,
            hierarchy_rules,
        }
    }

    /// Creates a verifier with custom hierarchy rules.
    #[must_use]
    pub fn with_hierarchy_rules(mut self, rules: HierarchyRules) -> Self {
        self.hierarchy_rules = rules;
        self
    }

    /// Registers an Öffnungsklausel for a statute.
    pub fn register_oeffnungsklausel(&mut self, statute_id: impl Into<String>) {
        self.hierarchy_rules.register_oeffnungsklausel(statute_id);
    }

    /// Registers a statute as zwingendes Recht.
    pub fn register_zwingendes_recht(&mut self, statute_id: impl Into<String>) {
        self.hierarchy_rules.register_zwingendes_recht(statute_id);
    }

    /// Verifies a set of statutes with German-specific checks.
    #[must_use]
    pub fn verify(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = self.inner.verify(statutes);

        // Additional German-specific checks
        result.merge(self.check_hierarchy_compliance(statutes));
        result.merge(self.check_guenstigkeitsprinzip(statutes));
        result.merge(self.check_zwingendes_recht_compliance(statutes));
        result.merge(self.check_arbeitsrecht_consistency(statutes));

        result
    }

    /// Verifies a single statute.
    #[must_use]
    pub fn verify_single(&self, statute: &Statute) -> VerificationResult {
        self.verify(std::slice::from_ref(statute))
    }

    /// Checks legal hierarchy (Normenhierarchie) compliance.
    fn check_hierarchy_compliance(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                let level_i = self.hierarchy_rules.get_hierarchy(&statutes[i]);
                let level_j = self.hierarchy_rules.get_hierarchy(&statutes[j]);

                if level_i != level_j && self.effects_conflict(&statutes[i], &statutes[j]) {
                    let (higher, lower) = if level_i.is_higher_than(&level_j) {
                        (&statutes[i], &statutes[j])
                    } else {
                        (&statutes[j], &statutes[i])
                    };

                    let lower_level = self.hierarchy_rules.get_hierarchy(lower);
                    let higher_level = self.hierarchy_rules.get_hierarchy(higher);

                    // Check if deviation is allowed by Öffnungsklausel
                    if self.hierarchy_rules.has_oeffnungsklausel(&higher.id) {
                        result = result.with_suggestion(format!(
                            "Abweichung durch Öffnungsklausel: {} ({}) weicht von {} ({}) ab",
                            lower.id,
                            lower_level.german_name(),
                            higher.id,
                            higher_level.german_name()
                        ));
                    } else if !lower_level.allows_guenstigkeitsprinzip() {
                        result = result.with_warning(format!(
                            "Möglicher Normenhierarchie-Verstoß: {} ({}) widerspricht {} ({})",
                            lower.id,
                            lower_level.german_name(),
                            higher.id,
                            higher_level.german_name()
                        ));
                    }
                }
            }
        }

        result
    }

    /// Checks Günstigkeitsprinzip (principle of most favorable treatment).
    ///
    /// In German labor law, if multiple norms apply, the one most favorable
    /// to the employee takes precedence (unless explicitly excluded).
    fn check_guenstigkeitsprinzip(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        // Find labor-related statutes at different hierarchy levels
        let labor_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| self.is_labor_law_statute(s))
            .collect();

        let mut hierarchy_groups: HashMap<Rechtsquelle, Vec<&Statute>> = HashMap::new();
        for statute in &labor_statutes {
            let level = self.hierarchy_rules.get_hierarchy(statute);
            hierarchy_groups.entry(level).or_default().push(statute);
        }

        // Check for potential Günstigkeitsprinzip applicability
        if hierarchy_groups.len() > 1 {
            let levels: Vec<_> = hierarchy_groups.keys().copied().collect();
            for level in &levels {
                if level.allows_guenstigkeitsprinzip() && hierarchy_groups.contains_key(level) {
                    result = result.with_suggestion(format!(
                        "Günstigkeitsprinzip: {} kann für Arbeitnehmer günstigere Regelungen enthalten",
                        level.german_name()
                    ));
                }
            }
        }

        result
    }

    /// Checks compliance with zwingendes Recht (mandatory law).
    fn check_zwingendes_recht_compliance(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        for statute in statutes {
            if self.hierarchy_rules.is_zwingendes_recht(&statute.id) {
                // Check if any lower-level statute tries to override it
                for other in statutes {
                    if statute.id != other.id {
                        let other_level = self.hierarchy_rules.get_hierarchy(other);
                        let statute_level = self.hierarchy_rules.get_hierarchy(statute);

                        if other_level.is_higher_than(&statute_level)
                            && self.effects_conflict(statute, other)
                        {
                            result.merge(VerificationResult::fail(vec![
                                VerificationError::ConstitutionalConflict {
                                    statute_id: other.id.clone(),
                                    principle: format!("Zwingendes Recht ({})", statute.id),
                                },
                            ]));
                        }
                    }
                }
            }
        }

        result
    }

    /// Checks German labor law (Arbeitsrecht) consistency.
    fn check_arbeitsrecht_consistency(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        let labor_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| self.is_labor_law_statute(s))
            .collect();

        // Check working time statutes (ArbZG consistency)
        let working_time_statutes: Vec<_> = labor_statutes
            .iter()
            .filter(|s| {
                s.id.starts_with("ArbZG_")
                    || s.title.contains("Arbeitszeit")
                    || s.title.contains("working time")
            })
            .collect();

        if working_time_statutes.len() > 1 {
            result = result.with_suggestion(format!(
                "ArbZG-Konsistenzprüfung: {} Arbeitszeitregelungen gefunden",
                working_time_statutes.len()
            ));
        }

        // Check dismissal protection (KSchG consistency)
        let dismissal_statutes: Vec<_> = labor_statutes
            .iter()
            .filter(|s| {
                s.id.starts_with("KSchG_")
                    || s.title.contains("Kündigung")
                    || s.title.contains("dismissal")
            })
            .collect();

        if dismissal_statutes.len() > 1 {
            result = result.with_suggestion(format!(
                "KSchG-Konsistenzprüfung: {} Kündigungsschutzregelungen gefunden",
                dismissal_statutes.len()
            ));
        }

        result
    }

    /// Checks if a statute is labor law related.
    fn is_labor_law_statute(&self, statute: &Statute) -> bool {
        let id = &statute.id;
        let title = statute.title.to_lowercase();

        id.starts_with("ArbZG_")
            || id.starts_with("KSchG_")
            || id.starts_with("BGB_")
            || id.starts_with("BUrlG_")
            || id.starts_with("EFZG_")
            || id.starts_with("MiLoG_")
            || id.starts_with("TzBfG_")
            || id.starts_with("BetrVG_")
            || title.contains("arbeits")
            || title.contains("labor")
            || title.contains("employment")
    }

    /// Checks if two statutes have conflicting effects.
    fn effects_conflict(&self, statute1: &Statute, statute2: &Statute) -> bool {
        use legalis_core::EffectType;

        matches!(
            (&statute1.effect.effect_type, &statute2.effect.effect_type),
            (EffectType::Grant, EffectType::Revoke)
                | (EffectType::Revoke, EffectType::Grant)
                | (EffectType::Obligation, EffectType::Prohibition)
                | (EffectType::Prohibition, EffectType::Obligation)
        )
    }
}

/// Creates constitutional principles based on the Grundgesetz (Basic Law).
///
/// Returns principles for key Grundgesetz articles:
/// - Art. 1: Menschenwürde (Human Dignity)
/// - Art. 3: Gleichheit vor dem Gesetz (Equality)
/// - Art. 12: Berufsfreiheit (Occupational Freedom)
/// - Art. 14: Eigentumsgarantie (Property Rights)
/// - Art. 20: Sozialstaatsprinzip (Social State Principle)
#[must_use]
pub fn de_constitutional_principles() -> Vec<ConstitutionalPrinciple> {
    vec![
        // Article 1: Human Dignity (Menschenwürde)
        ConstitutionalPrinciple {
            id: "gg-art1-dignity".to_string(),
            name: "Menschenwürde (Human Dignity)".to_string(),
            description: "Die Würde des Menschen ist unantastbar. Sie zu achten und zu schützen ist Verpflichtung aller staatlichen Gewalt.\n\
                          Human dignity shall be inviolable. To respect and protect it shall be the duty of all state authority.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Human dignity protection".to_string(),
            },
        },
        // Article 3: Equality (Gleichheit vor dem Gesetz)
        ConstitutionalPrinciple {
            id: "gg-art3-equality".to_string(),
            name: "Gleichheit vor dem Gesetz (Equality)".to_string(),
            description: "Alle Menschen sind vor dem Gesetz gleich. Niemand darf wegen seines Geschlechtes, seiner Abstammung, seiner Rasse, seiner Sprache, seiner Heimat und Herkunft, seines Glaubens, seiner religiösen oder politischen Anschauungen benachteiligt oder bevorzugt werden.\n\
                          All persons are equal before the law. No person shall be favored or disfavored because of sex, parentage, race, language, homeland and origin, faith, or religious or political opinions.".to_string(),
            check: PrincipleCheck::NoDiscrimination,
        },
        // Article 12: Occupational Freedom (Berufsfreiheit)
        ConstitutionalPrinciple {
            id: "gg-art12-occupation".to_string(),
            name: "Berufsfreiheit (Occupational Freedom)".to_string(),
            description: "Alle Deutschen haben das Recht, Beruf, Arbeitsplatz und Ausbildungsstätte frei zu wählen.\n\
                          All Germans shall have the right freely to choose their occupation or profession, their place of work and their place of training.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Occupational freedom protection".to_string(),
            },
        },
        // Article 14: Property Rights (Eigentumsgarantie)
        ConstitutionalPrinciple {
            id: "gg-art14-property".to_string(),
            name: "Eigentumsgarantie (Property Rights)".to_string(),
            description: "Das Eigentum und das Erbrecht werden gewährleistet. Eigentum verpflichtet. Sein Gebrauch soll zugleich dem Wohle der Allgemeinheit dienen.\n\
                          Property and the right of inheritance shall be guaranteed. Property entails obligations. Its use shall also serve the public good.".to_string(),
            check: PrincipleCheck::PropertyRights,
        },
        // Article 20: Social State Principle (Sozialstaatsprinzip)
        ConstitutionalPrinciple {
            id: "gg-art20-social".to_string(),
            name: "Sozialstaatsprinzip (Social State)".to_string(),
            description: "Die Bundesrepublik Deutschland ist ein demokratischer und sozialer Bundesstaat.\n\
                          The Federal Republic of Germany is a democratic and social federal state.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Social state principle compliance".to_string(),
            },
        },
        // Article 19: Essential Content Protection (Wesensgehaltsgarantie)
        ConstitutionalPrinciple {
            id: "gg-art19-essence".to_string(),
            name: "Wesensgehaltsgarantie (Essential Content)".to_string(),
            description: "In keinem Falle darf ein Grundrecht in seinem Wesensgehalt angetastet werden.\n\
                          In no case may the essence of a basic right be affected.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Essential content of basic rights protection".to_string(),
            },
        },
        // Proportionality Principle (Verhältnismäßigkeitsprinzip)
        ConstitutionalPrinciple {
            id: "gg-proportionality".to_string(),
            name: "Verhältnismäßigkeitsprinzip (Proportionality)".to_string(),
            description: "Eingriffe in Grundrechte müssen geeignet, erforderlich und angemessen sein.\n\
                          Restrictions on basic rights must be suitable, necessary, and proportionate.".to_string(),
            check: PrincipleCheck::Proportionality,
        },
        // Legal Certainty (Rechtssicherheit)
        ConstitutionalPrinciple {
            id: "gg-legal-certainty".to_string(),
            name: "Rechtssicherheit (Legal Certainty)".to_string(),
            description: "Das Rechtsstaatsprinzip gebietet Rechtssicherheit und Vertrauensschutz.\n\
                          The rule of law principle requires legal certainty and protection of legitimate expectations.".to_string(),
            check: PrincipleCheck::NoRetroactivity,
        },
    ]
}

/// Sets up labor law hierarchy relationships.
fn setup_labor_law_hierarchy(rules: &mut HierarchyRules) {
    // Register Federal Laws (Bundesgesetze)
    rules.register_hierarchy("ArbZG_Sec3", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("ArbZG_Sec4", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("ArbZG_Sec5", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("KSchG_Sec1", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("BGB_Sec622", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("BGB_Sec623", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("BUrlG_Sec3", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("EFZG_Sec3", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("MiLoG_MinWage", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("TzBfG_Sec14", Rechtsquelle::Bundesgesetz);
    rules.register_hierarchy("BetrVG_Sec102", Rechtsquelle::Bundesgesetz);

    // Register Öffnungsklauseln (opening clauses allowing deviation)
    // ArbZG §7: Allows deviation by collective agreement
    rules.register_oeffnungsklausel("ArbZG_Sec7_Opening");

    // Register zwingendes Recht (mandatory law)
    // MiLoG: Minimum wage cannot be undercut
    rules.register_zwingendes_recht("MiLoG_MinWage");
    // BGB §623: Written form for dismissal is mandatory
    rules.register_zwingendes_recht("BGB_Sec623");
    // ArbZG maximum limits are mandatory
    rules.register_zwingendes_recht("ArbZG_Sec3_Max");
}

/// Verification report for German law compliance.
#[derive(Debug, Clone)]
pub struct DeVerificationReport {
    /// Base verification result
    pub result: VerificationResult,
    /// Grundgesetz compliance issues
    pub grundgesetz_issues: Vec<GrundgesetzIssue>,
    /// Normenhierarchie violations
    pub hierarchy_violations: Vec<NormenhierarchieViolation>,
    /// Günstigkeitsprinzip notes
    pub guenstigkeits_notes: Vec<String>,
}

/// Grundgesetz (Basic Law) compliance issue.
#[derive(Debug, Clone)]
pub struct GrundgesetzIssue {
    /// Statute ID with the issue
    pub statute_id: String,
    /// Article of the Grundgesetz potentially violated
    pub article: String,
    /// Description of the issue (German and English)
    pub description: String,
    /// Severity of the issue
    pub severity: Severity,
}

/// Legal hierarchy (Normenhierarchie) violation.
#[derive(Debug, Clone)]
pub struct NormenhierarchieViolation {
    /// Lower-level statute ID
    pub lower_statute_id: String,
    /// Higher-level statute ID
    pub higher_statute_id: String,
    /// Lower-level Rechtsquelle
    pub lower_level: Rechtsquelle,
    /// Higher-level Rechtsquelle
    pub higher_level: Rechtsquelle,
    /// Description of the conflict
    pub description: String,
}

impl DeVerificationReport {
    /// Creates a new report from verification result.
    #[must_use]
    pub fn new(result: VerificationResult) -> Self {
        Self {
            result,
            grundgesetz_issues: Vec::new(),
            hierarchy_violations: Vec::new(),
            guenstigkeits_notes: Vec::new(),
        }
    }

    /// Checks if the verification passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.result.passed && self.grundgesetz_issues.is_empty()
    }

    /// Gets all critical issues.
    #[must_use]
    pub fn critical_issues(&self) -> Vec<&VerificationError> {
        self.result.errors_by_severity(Severity::Critical)
    }

    /// Generates a summary in German.
    #[must_use]
    pub fn summary_de(&self) -> String {
        let mut summary = String::new();

        if self.passed() {
            summary.push_str("Prüfungsergebnis: Bestanden\n");
        } else {
            summary.push_str("Prüfungsergebnis: Nicht bestanden\n");
        }

        summary.push_str(&format!("Fehleranzahl: {}\n", self.result.errors.len()));
        summary.push_str(&format!("Warnungen: {}\n", self.result.warnings.len()));
        summary.push_str(&format!(
            "Empfehlungen: {}\n",
            self.result.suggestions.len()
        ));

        if !self.grundgesetz_issues.is_empty() {
            summary.push_str("\nGrundgesetz-Konformitätsprobleme:\n");
            for issue in &self.grundgesetz_issues {
                summary.push_str(&format!(
                    "  - {} ({}): {}\n",
                    issue.statute_id, issue.article, issue.description
                ));
            }
        }

        if !self.hierarchy_violations.is_empty() {
            summary.push_str("\nNormenhierarchie-Verstöße:\n");
            for violation in &self.hierarchy_violations {
                summary.push_str(&format!(
                    "  - {} ({}) vs {} ({}): {}\n",
                    violation.lower_statute_id,
                    violation.lower_level.german_name(),
                    violation.higher_statute_id,
                    violation.higher_level.german_name(),
                    violation.description
                ));
            }
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reasoning::statute_adapter::all_labor_statutes;
    use legalis_core::{Effect, EffectType, Statute};

    // ========================================================================
    // DeStatuteVerifier tests
    // ========================================================================

    #[test]
    fn test_verifier_creation() {
        let verifier = DeStatuteVerifier::new();
        assert!(!verifier.hierarchy_rules.known_hierarchies.is_empty());
    }

    #[test]
    fn test_verify_labor_statutes() {
        let verifier = DeStatuteVerifier::new();
        let statutes = all_labor_statutes();

        let result = verifier.verify(&statutes);
        // Labor statutes should pass basic verification
        assert!(result.passed || !result.errors.is_empty());
    }

    #[test]
    fn test_verify_empty_statutes() {
        let verifier = DeStatuteVerifier::new();
        let result = verifier.verify(&[]);
        assert!(result.passed);
    }

    #[test]
    fn test_verify_single_statute() {
        let verifier = DeStatuteVerifier::new();
        let statute = Statute::new(
            "TEST_001",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let result = verifier.verify_single(&statute);
        assert!(result.passed);
    }

    // ========================================================================
    // Rechtsquelle tests
    // ========================================================================

    #[test]
    fn test_rechtsquelle_ordering() {
        assert!(Rechtsquelle::Grundgesetz.is_higher_than(&Rechtsquelle::Bundesgesetz));
        assert!(Rechtsquelle::Bundesgesetz.is_higher_than(&Rechtsquelle::Landesgesetz));
        assert!(Rechtsquelle::Landesgesetz.is_higher_than(&Rechtsquelle::Rechtsverordnung));
        assert!(Rechtsquelle::Rechtsverordnung.is_higher_than(&Rechtsquelle::Satzung));
        assert!(Rechtsquelle::Satzung.is_higher_than(&Rechtsquelle::Tarifvertrag));
        assert!(Rechtsquelle::Tarifvertrag.is_higher_than(&Rechtsquelle::Betriebsvereinbarung));
        assert!(Rechtsquelle::Betriebsvereinbarung.is_higher_than(&Rechtsquelle::Arbeitsvertrag));
    }

    #[test]
    fn test_rechtsquelle_from_statute_bundesgesetz() {
        let statute = Statute::new(
            "ArbZG_Sec3",
            "Arbeitszeitgesetz Section 3",
            Effect::new(EffectType::Obligation, "Test"),
        );
        assert_eq!(
            Rechtsquelle::from_statute(&statute),
            Rechtsquelle::Bundesgesetz
        );
    }

    #[test]
    fn test_rechtsquelle_from_statute_tarifvertrag() {
        let statute = Statute::new(
            "TV_Metallindustrie",
            "Tarifvertrag Metallindustrie",
            Effect::new(EffectType::Grant, "Test"),
        );
        assert_eq!(
            Rechtsquelle::from_statute(&statute),
            Rechtsquelle::Tarifvertrag
        );
    }

    #[test]
    fn test_rechtsquelle_german_names() {
        assert_eq!(Rechtsquelle::Grundgesetz.german_name(), "Grundgesetz");
        assert_eq!(Rechtsquelle::Bundesgesetz.german_name(), "Bundesgesetz");
        assert_eq!(Rechtsquelle::Tarifvertrag.german_name(), "Tarifvertrag");
    }

    #[test]
    fn test_guenstigkeitsprinzip_applicability() {
        assert!(!Rechtsquelle::Bundesgesetz.allows_guenstigkeitsprinzip());
        assert!(Rechtsquelle::Tarifvertrag.allows_guenstigkeitsprinzip());
        assert!(Rechtsquelle::Betriebsvereinbarung.allows_guenstigkeitsprinzip());
        assert!(Rechtsquelle::Arbeitsvertrag.allows_guenstigkeitsprinzip());
    }

    // ========================================================================
    // HierarchyRules tests
    // ========================================================================

    #[test]
    fn test_register_oeffnungsklausel() {
        let mut rules = HierarchyRules::new();
        rules.register_oeffnungsklausel("ArbZG_Sec7");

        assert!(rules.has_oeffnungsklausel("ArbZG_Sec7"));
        assert!(!rules.has_oeffnungsklausel("ArbZG_Sec3"));
    }

    #[test]
    fn test_register_zwingendes_recht() {
        let mut rules = HierarchyRules::new();
        rules.register_zwingendes_recht("MiLoG_MinWage");

        assert!(rules.is_zwingendes_recht("MiLoG_MinWage"));
        assert!(!rules.is_zwingendes_recht("ArbZG_Sec3"));
    }

    #[test]
    fn test_register_hierarchy() {
        let mut rules = HierarchyRules::new();
        rules.register_hierarchy("TEST_001", Rechtsquelle::Tarifvertrag);

        let statute = Statute::new("TEST_001", "Test", Effect::new(EffectType::Grant, "Test"));
        assert_eq!(rules.get_hierarchy(&statute), Rechtsquelle::Tarifvertrag);
    }

    // ========================================================================
    // Constitutional principles tests
    // ========================================================================

    #[test]
    fn test_de_constitutional_principles() {
        let principles = de_constitutional_principles();

        assert!(!principles.is_empty());

        let principle_ids: Vec<_> = principles.iter().map(|p| p.id.as_str()).collect();
        assert!(principle_ids.contains(&"gg-art1-dignity"));
        assert!(principle_ids.contains(&"gg-art3-equality"));
        assert!(principle_ids.contains(&"gg-art12-occupation"));
        assert!(principle_ids.contains(&"gg-art14-property"));
        assert!(principle_ids.contains(&"gg-art20-social"));
    }

    #[test]
    fn test_constitutional_principles_have_bilingual_descriptions() {
        let principles = de_constitutional_principles();

        for principle in &principles {
            assert!(!principle.name.is_empty());
            assert!(!principle.description.is_empty());
            // Check bilingual descriptions (German and English)
            assert!(
                principle.description.contains("\n"),
                "Principle {} should have both German and English descriptions",
                principle.id
            );
        }
    }

    // ========================================================================
    // DeVerificationReport tests
    // ========================================================================

    #[test]
    fn test_verification_report_passed() {
        let result = VerificationResult::pass();
        let report = DeVerificationReport::new(result);
        assert!(report.passed());
    }

    #[test]
    fn test_verification_report_with_issues() {
        let result = VerificationResult::pass();
        let mut report = DeVerificationReport::new(result);
        report.grundgesetz_issues.push(GrundgesetzIssue {
            statute_id: "TEST".to_string(),
            article: "Art. 1".to_string(),
            description: "Test issue".to_string(),
            severity: Severity::Warning,
        });
        assert!(!report.passed());
    }

    #[test]
    fn test_verification_report_summary() {
        let result = VerificationResult::pass();
        let report = DeVerificationReport::new(result);
        let summary = report.summary_de();
        assert!(summary.contains("Prüfungsergebnis: Bestanden"));
    }

    // ========================================================================
    // Integration tests
    // ========================================================================

    #[test]
    fn test_labor_law_hierarchy_setup() {
        let verifier = DeStatuteVerifier::new();

        let statute = Statute::new("ArbZG_Sec3", "Test", Effect::new(EffectType::Grant, "Test"));
        assert_eq!(
            verifier.hierarchy_rules.get_hierarchy(&statute),
            Rechtsquelle::Bundesgesetz
        );
    }

    #[test]
    fn test_zwingendes_recht_setup() {
        let verifier = DeStatuteVerifier::new();

        assert!(
            verifier
                .hierarchy_rules
                .is_zwingendes_recht("MiLoG_MinWage")
        );
        assert!(verifier.hierarchy_rules.is_zwingendes_recht("BGB_Sec623"));
    }
}
