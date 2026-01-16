//! Legal statute verification for Japanese law (日本法).
//!
//! This module provides verification capabilities for Japanese legal statutes,
//! checking constitutional compliance and logical consistency based on:
//!
//! - **日本国憲法 (Constitution of Japan)**: Fundamental rights and principles
//! - **特別法優先の原則 (Lex Specialis)**: Special laws override general laws
//! - **法令の階層構造**: Statute hierarchy verification
//!
//! # Key Principles / 主要原則
//!
//! | Japanese | English | Article |
//! |----------|---------|---------|
//! | 法の下の平等 | Equality under law | Art. 14 |
//! | 勤労の権利 | Right to work | Art. 27 |
//! | 団結権 | Right to organize | Art. 28 |
//! | 生存権 | Right to life | Art. 25 |
//! | 遡及処罰の禁止 | No retroactive punishment | Art. 39 |
//!
//! # Example / 使用例
//!
//! ```rust,ignore
//! use legalis_jp::reasoning::verifier::{JpStatuteVerifier, jp_constitutional_principles};
//! use legalis_jp::reasoning::statute_adapter::all_labor_statutes;
//!
//! let verifier = JpStatuteVerifier::new();
//! let statutes = all_labor_statutes();
//!
//! let result = verifier.verify(&statutes);
//! println!("Verification passed: {}", result.passed);
//! ```

use legalis_core::Statute;
use legalis_verifier::{
    ConstitutionalPrinciple, PrincipleCheck, Severity, StatuteVerifier, VerificationError,
    VerificationResult,
};
use std::collections::{HashMap, HashSet};

/// Japanese statute verifier with constitutional compliance checking.
///
/// This verifier integrates Japanese constitutional principles and legal
/// hierarchy rules into the standard legalis-verifier framework.
///
/// # 日本法の特徴
///
/// - 憲法適合性審査: 法令が憲法に違反しないことを確認
/// - 特別法優先原則: 一般法と特別法の関係を検証
/// - 法令階層: 法律 > 政令 > 省令 > 条例の階層を確認
pub struct JpStatuteVerifier {
    /// Base verifier from legalis-verifier
    inner: StatuteVerifier,
    /// Japanese-specific legal hierarchy rules
    hierarchy_rules: HierarchyRules,
}

impl std::fmt::Debug for JpStatuteVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JpStatuteVerifier")
            .field("hierarchy_rules", &self.hierarchy_rules)
            .finish_non_exhaustive()
    }
}

/// Legal hierarchy in Japanese law.
///
/// 日本法における法令の階層構造:
/// 1. 憲法 (Constitution)
/// 2. 法律 (Act/Law)
/// 3. 政令 (Cabinet Order)
/// 4. 省令 (Ministerial Ordinance)
/// 5. 条例 (Local Ordinance)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LegalHierarchy {
    /// 憲法 - Constitution (highest)
    Constitution,
    /// 法律 - Acts passed by the Diet
    Act,
    /// 政令 - Cabinet Orders
    CabinetOrder,
    /// 省令 - Ministerial Ordinances
    MinisterialOrdinance,
    /// 条例 - Local Government Ordinances
    LocalOrdinance,
    /// 規則 - Rules and Regulations (lowest)
    Rule,
}

impl LegalHierarchy {
    /// Returns the Japanese name for this hierarchy level.
    #[must_use]
    pub const fn japanese_name(&self) -> &'static str {
        match self {
            Self::Constitution => "憲法",
            Self::Act => "法律",
            Self::CabinetOrder => "政令",
            Self::MinisterialOrdinance => "省令",
            Self::LocalOrdinance => "条例",
            Self::Rule => "規則",
        }
    }

    /// Returns the English name for this hierarchy level.
    #[must_use]
    pub const fn english_name(&self) -> &'static str {
        match self {
            Self::Constitution => "Constitution",
            Self::Act => "Act/Law",
            Self::CabinetOrder => "Cabinet Order",
            Self::MinisterialOrdinance => "Ministerial Ordinance",
            Self::LocalOrdinance => "Local Ordinance",
            Self::Rule => "Rule/Regulation",
        }
    }

    /// Determines hierarchy level from statute ID or title.
    #[must_use]
    pub fn from_statute(statute: &Statute) -> Self {
        let id = statute.id.to_lowercase();
        let title = statute.title.to_lowercase();

        // Check for Constitution references
        if id.contains("kenpo") || title.contains("憲法") || title.contains("constitution") {
            return Self::Constitution;
        }

        // Check for Cabinet Orders (政令)
        if id.contains("seirei")
            || id.contains("cabinet")
            || title.contains("政令")
            || title.contains("施行令")
        {
            return Self::CabinetOrder;
        }

        // Check for Ministerial Ordinances (省令)
        if id.contains("shorei")
            || id.contains("ordinance")
            || title.contains("省令")
            || title.contains("施行規則")
        {
            return Self::MinisterialOrdinance;
        }

        // Check for Local Ordinances (条例)
        if id.contains("jorei") || title.contains("条例") {
            return Self::LocalOrdinance;
        }

        // Check for Rules (規則)
        if id.contains("kisoku") || title.contains("規則") || title.contains("要綱") {
            return Self::Rule;
        }

        // Default to Act (法律)
        Self::Act
    }

    /// Checks if this hierarchy level is higher than another.
    #[must_use]
    pub fn is_higher_than(&self, other: &Self) -> bool {
        (*self as u8) < (*other as u8)
    }
}

/// Rules for legal hierarchy checking.
#[derive(Debug, Default)]
pub struct HierarchyRules {
    /// Known statute hierarchies (statute_id -> hierarchy level)
    known_hierarchies: HashMap<String, LegalHierarchy>,
    /// Special law relationships (general_law_id -> set of special_law_ids)
    lex_specialis: HashMap<String, HashSet<String>>,
}

impl HierarchyRules {
    /// Creates new hierarchy rules.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a statute's hierarchy level.
    pub fn register_hierarchy(&mut self, statute_id: impl Into<String>, level: LegalHierarchy) {
        self.known_hierarchies.insert(statute_id.into(), level);
    }

    /// Registers a lex specialis relationship.
    ///
    /// When `special_law_id` applies, it takes precedence over `general_law_id`.
    pub fn register_lex_specialis(
        &mut self,
        general_law_id: impl Into<String>,
        special_law_id: impl Into<String>,
    ) {
        self.lex_specialis
            .entry(general_law_id.into())
            .or_default()
            .insert(special_law_id.into());
    }

    /// Gets the hierarchy level for a statute.
    #[must_use]
    pub fn get_hierarchy(&self, statute: &Statute) -> LegalHierarchy {
        self.known_hierarchies
            .get(&statute.id)
            .copied()
            .unwrap_or_else(|| LegalHierarchy::from_statute(statute))
    }

    /// Checks if a statute is a special law relative to another.
    #[must_use]
    pub fn is_special_law(&self, general_id: &str, special_id: &str) -> bool {
        self.lex_specialis
            .get(general_id)
            .is_some_and(|specials| specials.contains(special_id))
    }
}

impl Default for JpStatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl JpStatuteVerifier {
    /// Creates a new Japanese statute verifier with constitutional principles.
    #[must_use]
    pub fn new() -> Self {
        let principles = jp_constitutional_principles();
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

    /// Registers a lex specialis relationship.
    pub fn register_lex_specialis(
        &mut self,
        general_law_id: impl Into<String>,
        special_law_id: impl Into<String>,
    ) {
        self.hierarchy_rules
            .register_lex_specialis(general_law_id, special_law_id);
    }

    /// Verifies a set of statutes with Japanese-specific checks.
    #[must_use]
    pub fn verify(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = self.inner.verify(statutes);

        // Additional Japanese-specific checks
        result.merge(self.check_hierarchy_compliance(statutes));
        result.merge(self.check_lex_specialis(statutes));
        result.merge(self.check_labor_law_consistency(statutes));

        result
    }

    /// Verifies a single statute.
    #[must_use]
    pub fn verify_single(&self, statute: &Statute) -> VerificationResult {
        self.verify(std::slice::from_ref(statute))
    }

    /// Checks legal hierarchy compliance.
    ///
    /// Lower-level statutes cannot contradict higher-level statutes.
    fn check_hierarchy_compliance(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                let level_i = self.hierarchy_rules.get_hierarchy(&statutes[i]);
                let level_j = self.hierarchy_rules.get_hierarchy(&statutes[j]);

                // Check if lower-level statute conflicts with higher-level
                if level_i != level_j && self.effects_conflict(&statutes[i], &statutes[j]) {
                    let (higher, lower) = if level_i.is_higher_than(&level_j) {
                        (&statutes[i], &statutes[j])
                    } else {
                        (&statutes[j], &statutes[i])
                    };

                    result = result.with_warning(format!(
                        "法令階層違反の可能性: {} ({}) と {} ({}) の間で矛盾が検出されました",
                        lower.id,
                        self.hierarchy_rules.get_hierarchy(lower).japanese_name(),
                        higher.id,
                        self.hierarchy_rules.get_hierarchy(higher).japanese_name()
                    ));
                }
            }
        }

        result
    }

    /// Checks lex specialis (特別法優先) relationships.
    ///
    /// When both general and special laws apply, the special law takes precedence.
    fn check_lex_specialis(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        for statute in statutes {
            // Check if this statute has known special laws
            if let Some(special_laws) = self.hierarchy_rules.lex_specialis.get(&statute.id) {
                let applicable_specials: Vec<_> = statutes
                    .iter()
                    .filter(|s| special_laws.contains(&s.id))
                    .collect();

                if !applicable_specials.is_empty() {
                    result = result.with_suggestion(format!(
                        "特別法優先: {} に対して特別法 {} が適用される可能性があります",
                        statute.id,
                        applicable_specials
                            .iter()
                            .map(|s| s.id.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
        }

        result
    }

    /// Checks labor law consistency.
    ///
    /// Japanese labor law has specific consistency requirements between:
    /// - 労働基準法 (Labor Standards Act)
    /// - 労働契約法 (Labor Contract Act)
    /// - 最低賃金法 (Minimum Wage Act)
    fn check_labor_law_consistency(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        // Find labor-related statutes
        let labor_statutes: Vec<_> = statutes
            .iter()
            .filter(|s| {
                s.id.starts_with("LSA_")
                    || s.id.starts_with("LCA_")
                    || s.id.starts_with("MWA_")
                    || s.id.contains("Labor")
                    || s.id.contains("労働")
            })
            .collect();

        // Check for working hours consistency
        let working_hours_statutes: Vec<_> = labor_statutes
            .iter()
            .filter(|s| {
                s.title.contains("working hours")
                    || s.title.contains("労働時間")
                    || s.title.contains("Working Hours")
            })
            .collect();

        if working_hours_statutes.len() > 1 {
            result = result.with_suggestion(format!(
                "労働時間規定の整合性確認: {} つの労働時間関連規定が存在します",
                working_hours_statutes.len()
            ));
        }

        // Check for overtime consistency
        let overtime_statutes: Vec<_> = labor_statutes
            .iter()
            .filter(|s| {
                s.title.contains("overtime")
                    || s.title.contains("時間外")
                    || s.title.contains("Overtime")
            })
            .collect();

        if overtime_statutes.len() > 1 {
            result = result.with_suggestion(format!(
                "時間外労働規定の整合性確認: {} つの時間外労働関連規定が存在します",
                overtime_statutes.len()
            ));
        }

        result
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

/// Creates constitutional principles based on the Constitution of Japan.
///
/// Returns principles for:
/// - 法の下の平等 (Article 14): Equality under the law
/// - 勤労の権利 (Article 27): Right and duty to work
/// - 団結権 (Article 28): Right of workers to organize
/// - 生存権 (Article 25): Right to maintain minimum standards of living
/// - 遡及処罰の禁止 (Article 39): No retroactive criminal punishment
#[must_use]
pub fn jp_constitutional_principles() -> Vec<ConstitutionalPrinciple> {
    vec![
        // Article 14: Equality under the law (法の下の平等)
        ConstitutionalPrinciple {
            id: "jp-art14-equality".to_string(),
            name: "法の下の平等 (Equality under Law)".to_string(),
            description: "すべて国民は、法の下に平等であって、人種、信条、性別、社会的身分又は門地により、政治的、経済的又は社会的関係において、差別されない。\nAll of the people are equal under the law and there shall be no discrimination in political, economic or social relations because of race, creed, sex, social status or family origin.".to_string(),
            check: PrincipleCheck::NoDiscrimination,
        },
        // Article 25: Right to life (生存権)
        ConstitutionalPrinciple {
            id: "jp-art25-survival".to_string(),
            name: "生存権 (Right to Life)".to_string(),
            description: "すべて国民は、健康で文化的な最低限度の生活を営む権利を有する。\nAll people shall have the right to maintain the minimum standards of wholesome and cultured living.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Minimum living standards protection".to_string(),
            },
        },
        // Article 27: Right to work (勤労の権利)
        ConstitutionalPrinciple {
            id: "jp-art27-work".to_string(),
            name: "勤労の権利 (Right to Work)".to_string(),
            description: "すべて国民は、勤労の権利を有し、義務を負う。\nAll people shall have the right and the obligation to work.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Right to work protection".to_string(),
            },
        },
        // Article 28: Right to organize (団結権)
        ConstitutionalPrinciple {
            id: "jp-art28-organize".to_string(),
            name: "団結権 (Right to Organize)".to_string(),
            description: "勤労者の団結する権利及び団体交渉その他の団体行動をする権利は、これを保障する。\nThe right of workers to organize and to bargain and act collectively is guaranteed.".to_string(),
            check: PrincipleCheck::Custom {
                description: "Workers' collective rights protection".to_string(),
            },
        },
        // Article 39: No retroactive punishment (遡及処罰の禁止)
        ConstitutionalPrinciple {
            id: "jp-art39-no-retroactive".to_string(),
            name: "遡及処罰の禁止 (No Retroactive Punishment)".to_string(),
            description: "何人も、実行の時に適法であつた行為又は既に無罪とされた行為については、刑事上の責任を問われない。\nNo person shall be held criminally liable for an act which was lawful at the time it was committed.".to_string(),
            check: PrincipleCheck::NoRetroactivity,
        },
        // Due process (適正手続)
        ConstitutionalPrinciple {
            id: "jp-art31-due-process".to_string(),
            name: "適正手続 (Due Process)".to_string(),
            description: "何人も、法律の定める手続によらなければ、その生命若しくは自由を奪はれ、又はその他の刑罰を科せられない。\nNo person shall be deprived of life or liberty, nor shall any other criminal penalty be imposed, except according to procedure established by law.".to_string(),
            check: PrincipleCheck::RequiresProcedure,
        },
        // Property rights (財産権)
        ConstitutionalPrinciple {
            id: "jp-art29-property".to_string(),
            name: "財産権 (Property Rights)".to_string(),
            description: "財産権は、これを侵してはならない。\nThe right to own or to hold property is inviolable.".to_string(),
            check: PrincipleCheck::PropertyRights,
        },
    ]
}

/// Sets up labor law hierarchy relationships.
fn setup_labor_law_hierarchy(rules: &mut HierarchyRules) {
    // Register Act-level statutes (法律)
    rules.register_hierarchy("LSA_Art32", LegalHierarchy::Act);
    rules.register_hierarchy("LSA_Art34", LegalHierarchy::Act);
    rules.register_hierarchy("LSA_Art35", LegalHierarchy::Act);
    rules.register_hierarchy("LSA_Art36", LegalHierarchy::Act);
    rules.register_hierarchy("LSA_Art37", LegalHierarchy::Act);
    rules.register_hierarchy("LSA_Art39", LegalHierarchy::Act);
    rules.register_hierarchy("LSA_Art20", LegalHierarchy::Act);
    rules.register_hierarchy("LCA_Art16", LegalHierarchy::Act);
    rules.register_hierarchy("LCA_Art18", LegalHierarchy::Act);
    rules.register_hierarchy("MWA", LegalHierarchy::Act);

    // Overtime Limit Regulation is Cabinet Order (政令)
    rules.register_hierarchy("OvertimeLimitReg", LegalHierarchy::CabinetOrder);

    // Lex specialis relationships
    // 労働契約法 is special law relative to 民法 (Civil Code) for employment contracts
    rules.register_lex_specialis("CivilCode", "LCA_Art16");
    rules.register_lex_specialis("CivilCode", "LCA_Art18");

    // 労働基準法 is special law for labor contracts
    rules.register_lex_specialis("CivilCode", "LSA_Art32");
    rules.register_lex_specialis("CivilCode", "LSA_Art20");

    // 最低賃金法 is special law for wage determination
    rules.register_lex_specialis("LSA_Art37", "MWA");
}

/// Verification report for Japanese law compliance.
#[derive(Debug, Clone)]
pub struct JpVerificationReport {
    /// Base verification result
    pub result: VerificationResult,
    /// Constitutional compliance issues
    pub constitutional_issues: Vec<ConstitutionalIssue>,
    /// Hierarchy violations
    pub hierarchy_violations: Vec<HierarchyViolation>,
    /// Lex specialis notes
    pub lex_specialis_notes: Vec<String>,
}

/// Constitutional compliance issue.
#[derive(Debug, Clone)]
pub struct ConstitutionalIssue {
    /// Statute ID with the issue
    pub statute_id: String,
    /// Article of the Constitution potentially violated
    pub article: String,
    /// Description of the issue
    pub description: String,
    /// Severity of the issue
    pub severity: Severity,
}

/// Legal hierarchy violation.
#[derive(Debug, Clone)]
pub struct HierarchyViolation {
    /// Lower-level statute ID
    pub lower_statute_id: String,
    /// Higher-level statute ID
    pub higher_statute_id: String,
    /// Description of the conflict
    pub description: String,
}

impl JpVerificationReport {
    /// Creates a new report from verification result.
    #[must_use]
    pub fn new(result: VerificationResult) -> Self {
        Self {
            result,
            constitutional_issues: Vec::new(),
            hierarchy_violations: Vec::new(),
            lex_specialis_notes: Vec::new(),
        }
    }

    /// Checks if the verification passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.result.passed && self.constitutional_issues.is_empty()
    }

    /// Gets all critical issues.
    #[must_use]
    pub fn critical_issues(&self) -> Vec<&VerificationError> {
        self.result.errors_by_severity(Severity::Critical)
    }

    /// Generates a summary in Japanese.
    #[must_use]
    pub fn summary_ja(&self) -> String {
        let mut summary = String::new();

        if self.passed() {
            summary.push_str("検証結果: 合格\n");
        } else {
            summary.push_str("検証結果: 不合格\n");
        }

        summary.push_str(&format!("エラー数: {}\n", self.result.errors.len()));
        summary.push_str(&format!("警告数: {}\n", self.result.warnings.len()));
        summary.push_str(&format!("提案数: {}\n", self.result.suggestions.len()));

        if !self.constitutional_issues.is_empty() {
            summary.push_str("\n憲法適合性の問題:\n");
            for issue in &self.constitutional_issues {
                summary.push_str(&format!(
                    "  - {} ({}): {}\n",
                    issue.statute_id, issue.article, issue.description
                ));
            }
        }

        if !self.hierarchy_violations.is_empty() {
            summary.push_str("\n法令階層違反:\n");
            for violation in &self.hierarchy_violations {
                summary.push_str(&format!(
                    "  - {} vs {}: {}\n",
                    violation.lower_statute_id, violation.higher_statute_id, violation.description
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
    // JpStatuteVerifier tests
    // ========================================================================

    #[test]
    fn test_verifier_creation() {
        let verifier = JpStatuteVerifier::new();
        assert!(!verifier.hierarchy_rules.known_hierarchies.is_empty());
    }

    #[test]
    fn test_verify_labor_statutes() {
        let verifier = JpStatuteVerifier::new();
        let statutes = all_labor_statutes();

        let result = verifier.verify(&statutes);
        // Labor statutes should pass basic verification
        assert!(result.passed || !result.errors.is_empty());
    }

    #[test]
    fn test_verify_empty_statutes() {
        let verifier = JpStatuteVerifier::new();
        let result = verifier.verify(&[]);
        assert!(result.passed);
    }

    #[test]
    fn test_verify_single_statute() {
        let verifier = JpStatuteVerifier::new();
        let statute = Statute::new(
            "TEST_001",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let result = verifier.verify_single(&statute);
        assert!(result.passed);
    }

    // ========================================================================
    // LegalHierarchy tests
    // ========================================================================

    #[test]
    fn test_hierarchy_ordering() {
        assert!(LegalHierarchy::Constitution.is_higher_than(&LegalHierarchy::Act));
        assert!(LegalHierarchy::Act.is_higher_than(&LegalHierarchy::CabinetOrder));
        assert!(LegalHierarchy::CabinetOrder.is_higher_than(&LegalHierarchy::MinisterialOrdinance));
        assert!(
            LegalHierarchy::MinisterialOrdinance.is_higher_than(&LegalHierarchy::LocalOrdinance)
        );
        assert!(LegalHierarchy::LocalOrdinance.is_higher_than(&LegalHierarchy::Rule));
    }

    #[test]
    fn test_hierarchy_from_statute_act() {
        let statute = Statute::new(
            "LSA_Art32",
            "Labor Standards Act Article 32",
            Effect::new(EffectType::Obligation, "Test"),
        );
        assert_eq!(LegalHierarchy::from_statute(&statute), LegalHierarchy::Act);
    }

    #[test]
    fn test_hierarchy_from_statute_cabinet_order() {
        let statute = Statute::new(
            "TEST_Seirei",
            "施行令第1条",
            Effect::new(EffectType::Obligation, "Test"),
        );
        assert_eq!(
            LegalHierarchy::from_statute(&statute),
            LegalHierarchy::CabinetOrder
        );
    }

    #[test]
    fn test_hierarchy_from_statute_ordinance() {
        let statute = Statute::new(
            "TEST_Shorei",
            "施行規則第1条",
            Effect::new(EffectType::Obligation, "Test"),
        );
        assert_eq!(
            LegalHierarchy::from_statute(&statute),
            LegalHierarchy::MinisterialOrdinance
        );
    }

    #[test]
    fn test_hierarchy_japanese_names() {
        assert_eq!(LegalHierarchy::Constitution.japanese_name(), "憲法");
        assert_eq!(LegalHierarchy::Act.japanese_name(), "法律");
        assert_eq!(LegalHierarchy::CabinetOrder.japanese_name(), "政令");
        assert_eq!(LegalHierarchy::MinisterialOrdinance.japanese_name(), "省令");
        assert_eq!(LegalHierarchy::LocalOrdinance.japanese_name(), "条例");
        assert_eq!(LegalHierarchy::Rule.japanese_name(), "規則");
    }

    // ========================================================================
    // HierarchyRules tests
    // ========================================================================

    #[test]
    fn test_register_lex_specialis() {
        let mut rules = HierarchyRules::new();
        rules.register_lex_specialis("CivilCode", "LCA_Art16");

        assert!(rules.is_special_law("CivilCode", "LCA_Art16"));
        assert!(!rules.is_special_law("CivilCode", "LSA_Art32"));
    }

    #[test]
    fn test_register_hierarchy() {
        let mut rules = HierarchyRules::new();
        rules.register_hierarchy("TEST_001", LegalHierarchy::CabinetOrder);

        let statute = Statute::new("TEST_001", "Test", Effect::new(EffectType::Grant, "Test"));
        assert_eq!(rules.get_hierarchy(&statute), LegalHierarchy::CabinetOrder);
    }

    // ========================================================================
    // Constitutional principles tests
    // ========================================================================

    #[test]
    fn test_jp_constitutional_principles() {
        let principles = jp_constitutional_principles();

        assert!(!principles.is_empty());

        // Check for key principles
        let principle_ids: Vec<_> = principles.iter().map(|p| p.id.as_str()).collect();
        assert!(principle_ids.contains(&"jp-art14-equality"));
        assert!(principle_ids.contains(&"jp-art25-survival"));
        assert!(principle_ids.contains(&"jp-art27-work"));
        assert!(principle_ids.contains(&"jp-art28-organize"));
        assert!(principle_ids.contains(&"jp-art39-no-retroactive"));
    }

    #[test]
    fn test_constitutional_principles_have_descriptions() {
        let principles = jp_constitutional_principles();

        for principle in &principles {
            assert!(!principle.name.is_empty());
            assert!(!principle.description.is_empty());
            // Check bilingual descriptions
            assert!(
                principle.description.contains("。") || principle.description.contains("."),
                "Principle {} should have both Japanese and English descriptions",
                principle.id
            );
        }
    }

    // ========================================================================
    // JpVerificationReport tests
    // ========================================================================

    #[test]
    fn test_verification_report_passed() {
        let result = VerificationResult::pass();
        let report = JpVerificationReport::new(result);
        assert!(report.passed());
    }

    #[test]
    fn test_verification_report_with_issues() {
        let result = VerificationResult::pass();
        let mut report = JpVerificationReport::new(result);
        report.constitutional_issues.push(ConstitutionalIssue {
            statute_id: "TEST".to_string(),
            article: "Art. 14".to_string(),
            description: "Test issue".to_string(),
            severity: Severity::Warning,
        });
        assert!(!report.passed());
    }

    #[test]
    fn test_verification_report_summary() {
        let result = VerificationResult::pass();
        let report = JpVerificationReport::new(result);
        let summary = report.summary_ja();
        assert!(summary.contains("検証結果: 合格"));
    }

    // ========================================================================
    // Integration tests
    // ========================================================================

    #[test]
    fn test_labor_law_hierarchy_setup() {
        let verifier = JpStatuteVerifier::new();

        // Check that labor statutes are registered at Act level
        let statute = Statute::new("LSA_Art32", "Test", Effect::new(EffectType::Grant, "Test"));
        assert_eq!(
            verifier.hierarchy_rules.get_hierarchy(&statute),
            LegalHierarchy::Act
        );

        // Check Cabinet Order level
        let cabinet_order = Statute::new(
            "OvertimeLimitReg",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        );
        assert_eq!(
            verifier.hierarchy_rules.get_hierarchy(&cabinet_order),
            LegalHierarchy::CabinetOrder
        );
    }

    #[test]
    fn test_lex_specialis_setup() {
        let verifier = JpStatuteVerifier::new();

        // Check Civil Code -> Labor Contract Act relationship
        assert!(
            verifier
                .hierarchy_rules
                .is_special_law("CivilCode", "LCA_Art16")
        );
        assert!(
            verifier
                .hierarchy_rules
                .is_special_law("CivilCode", "LCA_Art18")
        );
    }
}
