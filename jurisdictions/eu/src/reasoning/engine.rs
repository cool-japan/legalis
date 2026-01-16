//! Legal Reasoning Engine for EU Law.
//!
//! Provides automated compliance analysis and violation detection.

use legalis_core::StatuteRegistry;

use super::context::EuEvaluationContext;
use super::error::ReasoningResult;
use super::statute_adapter::all_eu_statutes;
use super::types::{LegalAnalysis, Violation, ViolationSeverity};

use crate::competition::types::RelevantMarket;
use crate::gdpr::types::DataController;

/// Legal Reasoning Engine for EU Law
pub struct LegalReasoningEngine {
    registry: StatuteRegistry,
}

impl Default for LegalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalReasoningEngine {
    /// Create a new reasoning engine with all EU statutes
    #[must_use]
    pub fn new() -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in all_eu_statutes() {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Create an engine with custom statutes
    #[must_use]
    pub fn with_statutes(statutes: Vec<legalis_core::Statute>) -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in statutes {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Analyze a data controller for GDPR compliance
    pub fn analyze_gdpr_compliance(
        &self,
        controller: &DataController,
        has_lawful_basis: bool,
        has_consent_mechanism: bool,
        has_security_measures: bool,
    ) -> ReasoningResult<LegalAnalysis> {
        let _ctx = EuEvaluationContext::new(controller);
        let mut violations = Vec::new();

        // Check lawful basis (Article 6)
        if !has_lawful_basis {
            violations.push(
                Violation::new(
                    "GDPR_Art6",
                    "Lawful Basis",
                    "Processing lacks a valid lawful basis under Article 6(1)",
                    ViolationSeverity::Critical,
                )
                .with_legal_reference("GDPR Art. 6(1)")
                .with_remediation("Establish a valid lawful basis for processing"),
            );
        }

        // Check consent mechanism (Article 7)
        if !has_consent_mechanism {
            violations.push(
                Violation::new(
                    "GDPR_Art7",
                    "Consent Mechanism",
                    "Consent does not meet requirements (freely given, specific, informed, unambiguous)",
                    ViolationSeverity::Major,
                )
                .with_legal_reference("GDPR Art. 7")
                .with_remediation("Implement compliant consent mechanisms"),
            );
        }

        // Check security measures (Article 32)
        if !has_security_measures {
            violations.push(
                Violation::new(
                    "GDPR_Art32",
                    "Security Measures",
                    "Appropriate technical and organisational security measures not implemented",
                    ViolationSeverity::Major,
                )
                .with_legal_reference("GDPR Art. 32")
                .with_remediation("Implement appropriate security measures"),
            );
        }

        // Check DPO requirement (Article 37) - required for certain controllers
        if controller.established_in_eu && !controller.dpo_appointed {
            // Advisory - may or may not be required depending on activities
            violations.push(
                Violation::new(
                    "GDPR_Art37",
                    "Data Protection Officer",
                    "DPO not designated - may be required depending on processing activities",
                    ViolationSeverity::Advisory,
                )
                .with_legal_reference("GDPR Art. 37")
                .with_remediation("Assess DPO requirement and designate if necessary"),
            );
        }

        if violations.is_empty() {
            Ok(LegalAnalysis::compliant("GdprCompliance"))
        } else {
            Ok(LegalAnalysis::non_compliant("GdprCompliance", violations))
        }
    }

    /// Analyze an undertaking for competition law compliance (Article 102)
    pub fn analyze_dominance_abuse(
        &self,
        market: &RelevantMarket,
        predatory_pricing: bool,
        tying_products: bool,
        refusal_to_deal: bool,
    ) -> ReasoningResult<LegalAnalysis> {
        let _ctx = EuEvaluationContext::new(market);
        let mut violations = Vec::new();

        // Article 102 only applies to dominant undertakings
        if !market.indicates_dominance() {
            return Ok(LegalAnalysis::compliant("CompetitionLaw").with_confidence(0.9));
        }

        // Check for predatory pricing
        if predatory_pricing {
            violations.push(
                Violation::new(
                    "TFEU_Art102",
                    "Predatory Pricing",
                    format!(
                        "Dominant undertaking ({}% market share) engaged in predatory pricing",
                        (market.market_share * 100.0).round() as u32
                    ),
                    ViolationSeverity::Critical,
                )
                .with_legal_reference("TFEU Art. 102(a)")
                .with_remediation("Cease below-cost pricing practices"),
            );
        }

        // Check for tying/bundling
        if tying_products {
            violations.push(
                Violation::new(
                    "TFEU_Art102",
                    "Tying/Bundling",
                    "Dominant undertaking engaged in unlawful tying or bundling",
                    ViolationSeverity::Major,
                )
                .with_legal_reference("TFEU Art. 102(d)")
                .with_remediation("Unbundle products and offer separately"),
            );
        }

        // Check for refusal to deal
        if refusal_to_deal {
            violations.push(
                Violation::new(
                    "TFEU_Art102",
                    "Refusal to Deal",
                    "Dominant undertaking engaged in anticompetitive refusal to supply",
                    ViolationSeverity::Major,
                )
                .with_legal_reference("TFEU Art. 102(b)")
                .with_remediation("Provide access on fair, reasonable terms"),
            );
        }

        if violations.is_empty() {
            Ok(LegalAnalysis::compliant("CompetitionLaw"))
        } else {
            Ok(LegalAnalysis::non_compliant("CompetitionLaw", violations))
        }
    }

    /// Analyze consumer rights compliance for distance selling
    pub fn analyze_consumer_rights(
        &self,
        is_distance_contract: bool,
        withdrawal_right_provided: bool,
        information_requirements_met: bool,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut violations = Vec::new();

        if is_distance_contract {
            // Check withdrawal right (Article 9)
            if !withdrawal_right_provided {
                violations.push(
                    Violation::new(
                        "CRD_Art9",
                        "Withdrawal Right",
                        "14-day withdrawal right not provided for distance contract",
                        ViolationSeverity::Critical,
                    )
                    .with_legal_reference("Consumer Rights Directive Art. 9")
                    .with_remediation("Provide 14-day withdrawal period"),
                );
            }

            // Check information requirements (Article 6)
            if !information_requirements_met {
                violations.push(
                    Violation::new(
                        "CRD_Art6",
                        "Information Requirements",
                        "Pre-contractual information requirements not met",
                        ViolationSeverity::Major,
                    )
                    .with_legal_reference("Consumer Rights Directive Art. 6")
                    .with_remediation("Provide all required pre-contractual information"),
                );
            }
        }

        if violations.is_empty() {
            Ok(LegalAnalysis::compliant("ConsumerRights"))
        } else {
            Ok(LegalAnalysis::non_compliant("ConsumerRights", violations))
        }
    }

    /// Get a reference to the statute registry
    #[must_use]
    pub fn registry(&self) -> &StatuteRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::ComplianceStatus;
    use super::*;
    use crate::competition::types::GeographicMarket;

    #[test]
    fn test_engine_creation() {
        let engine = LegalReasoningEngine::new();
        assert!(!engine.registry().is_empty());
    }

    #[test]
    fn test_gdpr_compliant() {
        let engine = LegalReasoningEngine::new();
        let controller = DataController {
            id: "DC001".to_string(),
            name: "Compliant Corp".to_string(),
            established_in_eu: true,
            dpo_appointed: true,
        };

        let analysis = engine
            .analyze_gdpr_compliance(&controller, true, true, true)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_compliant());
    }

    #[test]
    fn test_gdpr_non_compliant() {
        let engine = LegalReasoningEngine::new();
        let controller = DataController {
            id: "DC002".to_string(),
            name: "Non-Compliant Corp".to_string(),
            established_in_eu: true,
            dpo_appointed: false,
        };

        let analysis = engine
            .analyze_gdpr_compliance(&controller, false, false, false)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_non_compliant());

        if let ComplianceStatus::NonCompliant { violations } = &analysis.compliance_status {
            assert!(violations.iter().any(|v| v.statute_id == "GDPR_Art6"));
            assert!(violations.iter().any(|v| v.statute_id == "GDPR_Art32"));
        }
    }

    #[test]
    fn test_competition_law_dominant() {
        let engine = LegalReasoningEngine::new();
        let market = RelevantMarket {
            product_market: "Cloud Infrastructure".to_string(),
            geographic_market: GeographicMarket::EuWide,
            market_share: 0.65,
        };

        let analysis = engine
            .analyze_dominance_abuse(&market, true, false, false)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_non_compliant());

        if let ComplianceStatus::NonCompliant { violations } = &analysis.compliance_status {
            assert!(
                violations
                    .iter()
                    .any(|v| v.statute_name == "Predatory Pricing")
            );
        }
    }

    #[test]
    fn test_competition_law_non_dominant() {
        let engine = LegalReasoningEngine::new();
        let market = RelevantMarket {
            product_market: "Software".to_string(),
            geographic_market: GeographicMarket::EuWide,
            market_share: 0.15,
        };

        // Non-dominant undertakings aren't subject to Article 102
        let analysis = engine
            .analyze_dominance_abuse(&market, true, true, true)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_compliant());
    }

    #[test]
    fn test_consumer_rights() {
        let engine = LegalReasoningEngine::new();

        // Compliant distance seller
        let analysis = engine
            .analyze_consumer_rights(true, true, true)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_compliant());

        // Non-compliant distance seller
        let analysis = engine
            .analyze_consumer_rights(true, false, false)
            .expect("Analysis should succeed");
        assert!(analysis.compliance_status.is_non_compliant());

        if let ComplianceStatus::NonCompliant { violations } = &analysis.compliance_status {
            assert!(violations.iter().any(|v| v.statute_id == "CRD_Art9"));
        }
    }
}
