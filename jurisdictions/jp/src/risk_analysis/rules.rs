//! Risk Detection Rules (リスク検出ルール)
//!
//! This module provides detection rules for various types of contract risks.

use super::types::*;

/// Detection rule trait
pub trait DetectionRule {
    /// Rule identifier
    fn rule_id(&self) -> &str;

    /// Rule description
    fn description(&self) -> &str;

    /// Checks if the rule applies to the given contract type
    fn applies_to(&self, contract_type: ContractType) -> bool;

    /// Detects risks in the given text
    fn detect(&self, text: &str, location: &str) -> Vec<RiskFinding>;
}

/// Consumer protection violation detector (消費者保護法違反検出)
pub struct ConsumerProtectionRule;

impl DetectionRule for ConsumerProtectionRule {
    fn rule_id(&self) -> &str {
        "consumer-protection-001"
    }

    fn description(&self) -> &str {
        "Detects violations of Consumer Contract Act"
    }

    fn applies_to(&self, contract_type: ContractType) -> bool {
        matches!(
            contract_type,
            ContractType::Consumer | ContractType::General
        )
    }

    fn detect(&self, text: &str, location: &str) -> Vec<RiskFinding> {
        let mut findings = Vec::new();
        let text_lower = text.to_lowercase();

        // Full exemption clause detection (Article 8-1-1)
        let full_exemption_keywords = [
            "一切責任を負いません",
            "責任を負わない",
            "all liability is excluded",
            "no responsibility",
        ];

        for keyword in &full_exemption_keywords {
            if text_lower.contains(keyword) {
                findings.push(
                    RiskFinding::new(
                        format!("{}:full-exemption", self.rule_id()),
                        RiskSeverity::Critical,
                        RiskCategory::LiabilityExemption,
                        location,
                        text,
                        "全部免責条項は消費者契約法第8条1項1号により無効です。\n\
                         Full exemption clauses are invalid under Consumer Contract Act Article 8-1-1.",
                        "免責範囲を制限し、故意または重過失の場合は責任を負う旨を明記してください。",
                    )
                    .with_legal_reference("Consumer Contract Act Article 8-1-1")
                    .with_confidence(0.95),
                );
                break;
            }
        }

        // Excessive penalty detection (Article 9-1)
        let penalty_keywords = ["違約金", "損害賠償の予定", "penalty", "liquidated damages"];

        for keyword in &penalty_keywords {
            if text_lower.contains(keyword) {
                // Check for percentage indicators
                if text.contains("100%") || text.contains("全額") || text.contains("全部") {
                    findings.push(
                        RiskFinding::new(
                            format!("{}:excessive-penalty", self.rule_id()),
                            RiskSeverity::High,
                            RiskCategory::ExcessivePenalty,
                            location,
                            text,
                            "過大な違約金は消費者契約法第9条1項により無効となる可能性があります。\n\
                             Excessive penalties may be invalid under Consumer Contract Act Article 9-1.",
                            "違約金額を平均的損害額の範囲内に制限してください。",
                        )
                        .with_legal_reference("Consumer Contract Act Article 9-1")
                        .with_confidence(0.85),
                    );
                    break;
                }
            }
        }

        // Consumer disadvantage clause (Article 10)
        let disadvantage_keywords = [
            "一方的に変更",
            "裁量により変更",
            "unilaterally modify",
            "消費者の負担",
        ];

        for keyword in &disadvantage_keywords {
            if text_lower.contains(keyword) {
                findings.push(
                    RiskFinding::new(
                        format!("{}:consumer-disadvantage", self.rule_id()),
                        RiskSeverity::High,
                        RiskCategory::ConsumerProtection,
                        location,
                        text,
                        "消費者の利益を一方的に害する条項は無効となる可能性があります（第10条）。\n\
                         Clauses that unfairly disadvantage consumers may be invalid (Article 10).",
                        "双方の同意を得て変更する旨を明記してください。",
                    )
                    .with_legal_reference("Consumer Contract Act Article 10")
                    .with_confidence(0.80),
                );
                break;
            }
        }

        findings
    }
}

/// Labor law violation detector (労働法違反検出)
pub struct LaborLawRule;

impl DetectionRule for LaborLawRule {
    fn rule_id(&self) -> &str {
        "labor-law-001"
    }

    fn description(&self) -> &str {
        "Detects violations of Labor Standards Act and Labor Contract Act"
    }

    fn applies_to(&self, contract_type: ContractType) -> bool {
        matches!(
            contract_type,
            ContractType::Employment | ContractType::General
        )
    }

    fn detect(&self, text: &str, location: &str) -> Vec<RiskFinding> {
        let mut findings = Vec::new();
        let text_lower = text.to_lowercase();

        // Illegal penalty deduction (Article 16)
        let penalty_keywords = ["違約金を定め", "損害賠償額を予定", "penalty for breach"];

        for keyword in &penalty_keywords {
            if text_lower.contains(keyword) {
                findings.push(
                    RiskFinding::new(
                        format!("{}:illegal-penalty", self.rule_id()),
                        RiskSeverity::Critical,
                        RiskCategory::LaborLaw,
                        location,
                        text,
                        "労働契約不履行について違約金や損害賠償額の予定を定めることは労働基準法第16条で禁止されています。\n\
                         Stipulating penalties for breach of labor contracts is prohibited under Labor Standards Act Article 16.",
                        "違約金条項を削除してください。実際の損害が発生した場合は別途請求可能です。",
                    )
                    .with_legal_reference("Labor Standards Act Article 16")
                    .with_confidence(0.95),
                );
                break;
            }
        }

        // Illegal non-compete (overly broad)
        let non_compete_keywords = ["競業避止", "競業禁止", "non-compete", "競合他社"];

        for keyword in &non_compete_keywords {
            if text_lower.contains(keyword) {
                // Check if overly broad
                if text_lower.contains("永久")
                    || text_lower.contains("indefinitely")
                    || text_lower.contains("全国")
                {
                    findings.push(
                        RiskFinding::new(
                            format!("{}:illegal-noncompete", self.rule_id()),
                            RiskSeverity::High,
                            RiskCategory::IllegalNonCompete,
                            location,
                            text,
                            "過度に広範な競業避止義務は、労働者の職業選択の自由を侵害し無効となる可能性があります。\n\
                             Overly broad non-compete obligations may violate freedom of occupation and be invalid.",
                            "期間・地域・業種を合理的な範囲に限定してください。",
                        )
                        .with_confidence(0.85),
                    );
                    break;
                }
            }
        }

        // Forced savings (Article 18)
        if text_lower.contains("強制貯蓄") || text_lower.contains("貯蓄金の管理") {
            findings.push(
                RiskFinding::new(
                    format!("{}:forced-savings", self.rule_id()),
                    RiskSeverity::Critical,
                    RiskCategory::LaborLaw,
                    location,
                    text,
                    "労働者の貯蓄金を管理する契約は労働基準法第18条で禁止されています。\n\
                     Contracts to manage workers' savings are prohibited under Labor Standards Act Article 18.",
                    "強制貯蓄条項を削除してください。",
                )
                .with_legal_reference("Labor Standards Act Article 18")
                .with_confidence(0.95),
            );
        }

        findings
    }
}

/// Ambiguous clause detector (曖昧条項検出)
pub struct AmbiguousClauseRule;

impl DetectionRule for AmbiguousClauseRule {
    fn rule_id(&self) -> &str {
        "ambiguous-clause-001"
    }

    fn description(&self) -> &str {
        "Detects ambiguous or unclear contract clauses"
    }

    fn applies_to(&self, _contract_type: ContractType) -> bool {
        true // Applies to all contract types
    }

    fn detect(&self, text: &str, location: &str) -> Vec<RiskFinding> {
        let mut findings = Vec::new();
        let text_lower = text.to_lowercase();

        // Vague terms
        let vague_keywords = [
            "適宜",
            "相当の",
            "合理的な",
            "as appropriate",
            "reasonable",
            "等",
            "その他",
        ];

        let mut vague_count = 0;
        for keyword in &vague_keywords {
            if text_lower.contains(keyword) {
                vague_count += 1;
            }
        }

        if vague_count >= 2 {
            findings.push(
                RiskFinding::new(
                    format!("{}:vague-terms", self.rule_id()),
                    RiskSeverity::Medium,
                    RiskCategory::AmbiguousClause,
                    location,
                    text,
                    "曖昧な用語が複数含まれており、解釈に争いが生じる可能性があります。\n\
                     Multiple vague terms may lead to interpretation disputes.",
                    "具体的な基準や数値を明記してください。",
                )
                .with_confidence(0.70),
            );
        }

        // Missing definition
        if (text.contains("本契約において") || text.contains("本規約において"))
            && !text.contains("とは")
            && !text.contains("means")
        {
            findings.push(
                RiskFinding::new(
                    format!("{}:missing-definition", self.rule_id()),
                    RiskSeverity::Low,
                    RiskCategory::AmbiguousClause,
                    location,
                    text,
                    "重要な用語の定義が不明確です。\n\
                     Important terms lack clear definition.",
                    "用語の定義を明確に記載してください。",
                )
                .with_confidence(0.60),
            );
        }

        findings
    }
}

/// Jurisdiction clause detector (管轄条項検出)
pub struct JurisdictionClauseRule;

impl DetectionRule for JurisdictionClauseRule {
    fn rule_id(&self) -> &str {
        "jurisdiction-001"
    }

    fn description(&self) -> &str {
        "Detects potentially unfair jurisdiction clauses"
    }

    fn applies_to(&self, _contract_type: ContractType) -> bool {
        true
    }

    fn detect(&self, text: &str, location: &str) -> Vec<RiskFinding> {
        let mut findings = Vec::new();
        let text_lower = text.to_lowercase();

        // Exclusive jurisdiction in distant location
        if (text_lower.contains("専属的合意管轄") || text_lower.contains("exclusive jurisdiction"))
            && !text_lower.contains("当事者の")
            && !text_lower.contains("双方")
        {
            findings.push(
                RiskFinding::new(
                    format!("{}:exclusive-jurisdiction", self.rule_id()),
                    RiskSeverity::Medium,
                    RiskCategory::UnreasonableJurisdiction,
                    location,
                    text,
                    "一方当事者に不利な専属的合意管轄条項は無効となる可能性があります。\n\
                     Exclusive jurisdiction clauses unfavorable to one party may be invalid.",
                    "双方の利便性を考慮した管轄裁判所を定めるか、非専属的合意としてください。",
                )
                .with_confidence(0.75),
            );
        }

        findings
    }
}

/// Data protection detector (個人情報保護検出)
pub struct DataProtectionRule;

impl DetectionRule for DataProtectionRule {
    fn rule_id(&self) -> &str {
        "data-protection-001"
    }

    fn description(&self) -> &str {
        "Detects data protection and privacy issues"
    }

    fn applies_to(&self, _contract_type: ContractType) -> bool {
        true
    }

    fn detect(&self, text: &str, location: &str) -> Vec<RiskFinding> {
        let mut findings = Vec::new();
        let text_lower = text.to_lowercase();

        // Personal information handling without consent
        let personal_data_keywords = ["個人情報", "personal information", "プライバシー"];

        for keyword in &personal_data_keywords {
            if text_lower.contains(keyword) {
                // Check if consent mechanism is missing
                if !text_lower.contains("同意") && !text_lower.contains("consent") {
                    findings.push(
                        RiskFinding::new(
                            format!("{}:missing-consent", self.rule_id()),
                            RiskSeverity::High,
                            RiskCategory::DataProtection,
                            location,
                            text,
                            "個人情報の取扱いについて本人の同意が明記されていません。\n\
                             Personal data handling lacks explicit consent provisions.",
                            "個人情報保護法に基づき、本人の同意取得手続きを明記してください。",
                        )
                        .with_legal_reference("Act on the Protection of Personal Information")
                        .with_confidence(0.80),
                    );
                    break;
                }
            }
        }

        findings
    }
}

/// Rule engine that applies multiple detection rules
pub struct RuleEngine {
    rules: Vec<Box<dyn DetectionRule>>,
}

impl RuleEngine {
    /// Creates a new rule engine with default rules
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(ConsumerProtectionRule),
                Box::new(LaborLawRule),
                Box::new(AmbiguousClauseRule),
                Box::new(JurisdictionClauseRule),
                Box::new(DataProtectionRule),
            ],
        }
    }

    /// Creates an empty rule engine
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Adds a custom rule
    pub fn add_rule(&mut self, rule: Box<dyn DetectionRule>) {
        self.rules.push(rule);
    }

    /// Applies all rules to the given contract text
    pub fn analyze_text(
        &self,
        text: &str,
        location: &str,
        contract_type: ContractType,
    ) -> Vec<RiskFinding> {
        let mut findings = Vec::new();

        for rule in &self.rules {
            if rule.applies_to(contract_type) {
                let rule_findings = rule.detect(text, location);
                findings.extend(rule_findings);
            }
        }

        findings
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_protection_full_exemption() {
        let rule = ConsumerProtectionRule;
        let findings = rule.detect("当社は一切責任を負いません。", "Article 5");

        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, RiskSeverity::Critical);
        assert_eq!(findings[0].category, RiskCategory::LiabilityExemption);
    }

    #[test]
    fn test_labor_law_penalty() {
        let rule = LaborLawRule;
        let findings = rule.detect("退職時には違約金を定める。", "Article 10");

        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, RiskSeverity::Critical);
        assert_eq!(findings[0].category, RiskCategory::LaborLaw);
    }

    #[test]
    fn test_ambiguous_clause() {
        let rule = AmbiguousClauseRule;
        let findings = rule.detect("適宜相当の措置を講じる等の対応を行う。", "Article 3");

        assert!(!findings.is_empty());
        assert_eq!(findings[0].category, RiskCategory::AmbiguousClause);
    }

    #[test]
    fn test_rule_engine() {
        let engine = RuleEngine::new();
        let findings = engine.analyze_text(
            "当社は一切責任を負いません。",
            "Article 1",
            ContractType::Consumer,
        );

        assert!(!findings.is_empty());
    }

    #[test]
    fn test_rule_applicability() {
        let consumer_rule = ConsumerProtectionRule;
        assert!(consumer_rule.applies_to(ContractType::Consumer));
        assert!(!consumer_rule.applies_to(ContractType::Employment));

        let labor_rule = LaborLawRule;
        assert!(labor_rule.applies_to(ContractType::Employment));
        assert!(!labor_rule.applies_to(ContractType::Consumer));

        let ambiguous_rule = AmbiguousClauseRule;
        assert!(ambiguous_rule.applies_to(ContractType::Consumer));
        assert!(ambiguous_rule.applies_to(ContractType::Employment));
    }
}
