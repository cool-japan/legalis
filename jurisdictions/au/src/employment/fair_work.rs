//! Fair Work Act 2009 Analysis
//!
//! Implementation of Fair Work Act provisions including:
//! - National Employment Standards
//! - Modern awards
//! - Enterprise agreements
//! - General protections

use serde::{Deserialize, Serialize};

use super::types::{
    AdverseAction, BootResult, EmploymentType, NationalEmploymentStandard, ProtectedAttribute,
};

// ============================================================================
// NES Analyzer
// ============================================================================

/// Analyzer for National Employment Standards
pub struct NesAnalyzer;

impl NesAnalyzer {
    /// Check compliance with NES
    pub fn check_compliance(facts: &NesFacts) -> NesResult {
        let mut compliant_standards = Vec::new();
        let mut breached_standards = Vec::new();
        let mut issues = Vec::new();

        // Check each applicable standard
        for standard in &facts.applicable_standards {
            let compliance = Self::check_standard(standard, facts);
            if compliance.compliant {
                compliant_standards.push(standard.clone());
            } else {
                breached_standards.push(standard.clone());
                if let Some(issue) = compliance.issue {
                    issues.push(issue);
                }
            }
        }

        let reasoning = Self::build_reasoning(&compliant_standards, &breached_standards, &issues);

        NesResult {
            compliant: breached_standards.is_empty(),
            compliant_standards,
            breached_standards,
            issues,
            reasoning,
        }
    }

    /// Check a specific standard
    fn check_standard(
        standard: &NationalEmploymentStandard,
        facts: &NesFacts,
    ) -> StandardCompliance {
        match standard {
            NationalEmploymentStandard::MaximumWeeklyHours => Self::check_maximum_hours(facts),
            NationalEmploymentStandard::AnnualLeave => Self::check_annual_leave(facts),
            NationalEmploymentStandard::PersonalCarersLeave => Self::check_personal_leave(facts),
            NationalEmploymentStandard::NoticeOfTermination => Self::check_notice(facts),
            NationalEmploymentStandard::RedundancyPay => Self::check_redundancy(facts),
            NationalEmploymentStandard::PublicHolidays => Self::check_public_holidays(facts),
            _ => StandardCompliance {
                compliant: true,
                issue: None,
            },
        }
    }

    /// Check maximum weekly hours (s.62)
    fn check_maximum_hours(facts: &NesFacts) -> StandardCompliance {
        // 38 hours + reasonable additional hours
        if facts.weekly_hours <= 38.0 {
            return StandardCompliance {
                compliant: true,
                issue: None,
            };
        }

        // Additional hours must be reasonable
        if facts.additional_hours_reasonable {
            StandardCompliance {
                compliant: true,
                issue: None,
            }
        } else {
            StandardCompliance {
                compliant: false,
                issue: Some(format!(
                    "Additional hours ({:.1}) not reasonable per s.62(3) factors",
                    facts.weekly_hours - 38.0
                )),
            }
        }
    }

    /// Check annual leave (s.87)
    fn check_annual_leave(facts: &NesFacts) -> StandardCompliance {
        let required_weeks = match facts.employment_type {
            EmploymentType::Casual => {
                return StandardCompliance {
                    compliant: true,
                    issue: None,
                };
            }
            EmploymentType::FullTime | EmploymentType::PartTime => 4.0,
            _ => 4.0,
        };

        // Shift workers get 5 weeks
        let required = if facts.is_shift_worker {
            5.0
        } else {
            required_weeks
        };

        if facts.annual_leave_weeks >= required {
            StandardCompliance {
                compliant: true,
                issue: None,
            }
        } else {
            StandardCompliance {
                compliant: false,
                issue: Some(format!(
                    "Annual leave {:.1} weeks below minimum {} weeks (s.87)",
                    facts.annual_leave_weeks, required
                )),
            }
        }
    }

    /// Check personal/carer's leave (s.96)
    fn check_personal_leave(facts: &NesFacts) -> StandardCompliance {
        if matches!(facts.employment_type, EmploymentType::Casual) {
            return StandardCompliance {
                compliant: true,
                issue: None,
            };
        }

        if facts.personal_leave_days >= 10.0 * facts.service_years.min(1.0) {
            StandardCompliance {
                compliant: true,
                issue: None,
            }
        } else {
            StandardCompliance {
                compliant: false,
                issue: Some("Personal/carer's leave below 10 days per year (s.96)".to_string()),
            }
        }
    }

    /// Check notice of termination (s.117)
    fn check_notice(facts: &NesFacts) -> StandardCompliance {
        let required_weeks = Self::minimum_notice_weeks(facts.service_years);

        // Over 45 years old with 2+ years service gets extra week
        let required = if facts.employee_age >= 45.0 && facts.service_years >= 2.0 {
            required_weeks + 1.0
        } else {
            required_weeks
        };

        if facts.notice_weeks_given >= required {
            StandardCompliance {
                compliant: true,
                issue: None,
            }
        } else {
            StandardCompliance {
                compliant: false,
                issue: Some(format!(
                    "Notice {:.1} weeks below minimum {:.1} weeks (s.117)",
                    facts.notice_weeks_given, required
                )),
            }
        }
    }

    /// Minimum notice weeks based on service (s.117(3))
    fn minimum_notice_weeks(service_years: f64) -> f64 {
        if service_years < 1.0 {
            1.0
        } else if service_years < 3.0 {
            2.0
        } else if service_years < 5.0 {
            3.0
        } else {
            4.0
        }
    }

    /// Check redundancy pay (s.119)
    fn check_redundancy(facts: &NesFacts) -> StandardCompliance {
        // Small business (< 15 employees) exempt
        if facts.employer_size < 15 {
            return StandardCompliance {
                compliant: true,
                issue: None,
            };
        }

        let required_weeks = Self::redundancy_weeks(facts.service_years);

        if facts.redundancy_weeks_paid >= required_weeks {
            StandardCompliance {
                compliant: true,
                issue: None,
            }
        } else {
            StandardCompliance {
                compliant: false,
                issue: Some(format!(
                    "Redundancy pay {:.1} weeks below minimum {:.1} weeks (s.119)",
                    facts.redundancy_weeks_paid, required_weeks
                )),
            }
        }
    }

    /// Redundancy weeks based on service (s.119(2))
    fn redundancy_weeks(service_years: f64) -> f64 {
        if service_years < 1.0 {
            0.0
        } else if service_years < 2.0 {
            4.0
        } else if service_years < 3.0 {
            6.0
        } else if service_years < 4.0 {
            7.0
        } else if service_years < 5.0 {
            8.0
        } else if service_years < 6.0 {
            10.0
        } else if service_years < 7.0 {
            11.0
        } else if service_years < 8.0 {
            13.0
        } else if service_years < 9.0 {
            14.0
        } else if service_years < 10.0 {
            16.0
        } else {
            12.0 // Caps at 12 weeks after 10 years
        }
    }

    /// Check public holidays (s.114)
    fn check_public_holidays(facts: &NesFacts) -> StandardCompliance {
        if facts.public_holiday_entitlement {
            StandardCompliance {
                compliant: true,
                issue: None,
            }
        } else {
            StandardCompliance {
                compliant: false,
                issue: Some("Public holiday entitlement not provided (s.114)".to_string()),
            }
        }
    }

    /// Build reasoning
    fn build_reasoning(
        compliant: &[NationalEmploymentStandard],
        breached: &[NationalEmploymentStandard],
        issues: &[String],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("National Employment Standards analysis (Fair Work Act 2009)".to_string());

        if breached.is_empty() {
            parts.push("All applicable NES compliant".to_string());
        } else {
            parts.push(format!("NES breaches identified: {}", breached.len()));
            for issue in issues {
                parts.push(format!("- {}", issue));
            }
        }

        if !compliant.is_empty() {
            parts.push(format!("Compliant standards: {}", compliant.len()));
        }

        parts.join(". ")
    }
}

/// Compliance result for a standard
#[derive(Debug, Clone)]
struct StandardCompliance {
    compliant: bool,
    issue: Option<String>,
}

/// Facts for NES analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NesFacts {
    /// Employment type
    pub employment_type: EmploymentType,
    /// Applicable standards
    pub applicable_standards: Vec<NationalEmploymentStandard>,
    /// Weekly hours worked
    pub weekly_hours: f64,
    /// Additional hours reasonable
    pub additional_hours_reasonable: bool,
    /// Annual leave weeks accrued
    pub annual_leave_weeks: f64,
    /// Is shift worker
    pub is_shift_worker: bool,
    /// Personal leave days accrued
    pub personal_leave_days: f64,
    /// Years of service
    pub service_years: f64,
    /// Employee age
    pub employee_age: f64,
    /// Notice weeks given
    pub notice_weeks_given: f64,
    /// Redundancy weeks paid
    pub redundancy_weeks_paid: f64,
    /// Employer size
    pub employer_size: usize,
    /// Public holiday entitlement provided
    pub public_holiday_entitlement: bool,
}

/// Result of NES analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NesResult {
    /// Overall compliance
    pub compliant: bool,
    /// Compliant standards
    pub compliant_standards: Vec<NationalEmploymentStandard>,
    /// Breached standards
    pub breached_standards: Vec<NationalEmploymentStandard>,
    /// Issues identified
    pub issues: Vec<String>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// General Protections Analyzer
// ============================================================================

/// Analyzer for general protections (Part 3-1)
pub struct GeneralProtectionsAnalyzer;

impl GeneralProtectionsAnalyzer {
    /// Analyze general protections claim
    pub fn analyze(facts: &GeneralProtectionsFacts) -> GeneralProtectionsResult {
        let action_established = Self::check_adverse_action(facts);
        let protected_reason = Self::check_protected_reason(facts);
        let reverse_onus = facts.employee_establishes_prima_facie;

        let liable = if reverse_onus {
            // Employer must prove not because of protected reason
            action_established && protected_reason && !facts.employer_proves_other_reason
        } else {
            action_established && protected_reason
        };

        let remedies = if liable {
            Self::determine_remedies(facts)
        } else {
            Vec::new()
        };

        let reasoning = Self::build_reasoning(
            action_established,
            protected_reason,
            reverse_onus,
            liable,
            facts,
        );

        GeneralProtectionsResult {
            adverse_action: action_established,
            protected_reason,
            reverse_onus_applies: reverse_onus,
            contravention: liable,
            available_remedies: remedies,
            reasoning,
        }
    }

    /// Check adverse action (s.342)
    fn check_adverse_action(facts: &GeneralProtectionsFacts) -> bool {
        facts.adverse_action.is_some()
    }

    /// Check protected reason
    fn check_protected_reason(facts: &GeneralProtectionsFacts) -> bool {
        !facts.protected_attributes.is_empty()
    }

    /// Determine remedies
    fn determine_remedies(facts: &GeneralProtectionsFacts) -> Vec<GeneralProtectionsRemedy> {
        let mut remedies = Vec::new();

        // Compensation
        remedies.push(GeneralProtectionsRemedy::Compensation);

        // Reinstatement if dismissed
        if matches!(facts.adverse_action, Some(AdverseAction::Dismissal)) {
            remedies.push(GeneralProtectionsRemedy::Reinstatement);
        }

        // Injunction
        if facts.ongoing_conduct {
            remedies.push(GeneralProtectionsRemedy::Injunction);
        }

        // Pecuniary penalty
        remedies.push(GeneralProtectionsRemedy::PecuniaryPenalty);

        remedies
    }

    /// Build reasoning
    fn build_reasoning(
        action: bool,
        protected: bool,
        reverse: bool,
        liable: bool,
        facts: &GeneralProtectionsFacts,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("General protections analysis (Fair Work Act Part 3-1)".to_string());

        if action {
            if let Some(ref action_type) = facts.adverse_action {
                parts.push(format!(
                    "Adverse action established: {:?} (s.342)",
                    action_type
                ));
            }
        } else {
            parts.push("No adverse action established".to_string());
            return parts.join(". ");
        }

        if protected {
            for attr in &facts.protected_attributes {
                match attr {
                    ProtectedAttribute::WorkplaceRight => {
                        parts.push("Protected: workplace right (s.340)".to_string());
                    }
                    ProtectedAttribute::IndustrialActivity => {
                        parts.push("Protected: industrial activity (s.346)".to_string());
                    }
                    ProtectedAttribute::Discrimination(ground) => {
                        parts.push(format!("Protected: discrimination - {:?} (s.351)", ground));
                    }
                    ProtectedAttribute::TemporaryAbsence => {
                        parts.push("Protected: temporary absence (s.352)".to_string());
                    }
                }
            }
        }

        if reverse {
            parts.push("Reverse onus: employer must prove other reason (s.361)".to_string());
        }

        if liable {
            parts.push("Contravention established".to_string());
        } else if action && protected && !liable {
            parts.push("Employer proved action not because of protected reason".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for general protections analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeneralProtectionsFacts {
    /// Type of adverse action
    pub adverse_action: Option<AdverseAction>,
    /// Protected attributes involved
    pub protected_attributes: Vec<ProtectedAttribute>,
    /// Employee establishes prima facie case
    pub employee_establishes_prima_facie: bool,
    /// Employer proves other reason
    pub employer_proves_other_reason: bool,
    /// Ongoing conduct
    pub ongoing_conduct: bool,
}

/// General protections remedy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneralProtectionsRemedy {
    /// Compensation
    Compensation,
    /// Reinstatement
    Reinstatement,
    /// Injunction
    Injunction,
    /// Pecuniary penalty
    PecuniaryPenalty,
}

/// Result of general protections analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralProtectionsResult {
    /// Adverse action established
    pub adverse_action: bool,
    /// Protected reason involved
    pub protected_reason: bool,
    /// Reverse onus applies
    pub reverse_onus_applies: bool,
    /// Contravention found
    pub contravention: bool,
    /// Available remedies
    pub available_remedies: Vec<GeneralProtectionsRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Enterprise Agreement Analyzer
// ============================================================================

/// Analyzer for enterprise agreements
pub struct EnterpriseAgreementAnalyzer;

impl EnterpriseAgreementAnalyzer {
    /// Analyze enterprise agreement approval
    pub fn check_approval(facts: &AgreementFacts) -> AgreementApprovalResult {
        let mut issues = Vec::new();

        // Check BOOT
        let boot_passed = Self::check_boot(facts);
        if !boot_passed {
            issues.push("BOOT not satisfied (s.193)".to_string());
        }

        // Check genuine agreement
        let genuine = Self::check_genuine_agreement(facts);
        if !genuine {
            issues.push("Not genuinely agreed (s.188)".to_string());
        }

        // Check mandatory terms
        let terms_included = Self::check_mandatory_terms(facts);
        if !terms_included {
            issues.push("Missing mandatory terms (s.186)".to_string());
        }

        // Check consultation term
        let consultation = facts.consultation_term_included;
        if !consultation {
            issues.push("Missing consultation term (s.205)".to_string());
        }

        let approved = boot_passed && genuine && terms_included && consultation;
        let reasoning =
            Self::build_reasoning(boot_passed, genuine, terms_included, consultation, &issues);

        AgreementApprovalResult {
            approved,
            boot_satisfied: boot_passed,
            genuinely_agreed: genuine,
            mandatory_terms_included: terms_included,
            issues,
            reasoning,
        }
    }

    /// Check Better Off Overall Test
    fn check_boot(facts: &AgreementFacts) -> bool {
        matches!(facts.boot_result, BootResult::BetterOff)
            || (matches!(facts.boot_result, BootResult::Marginal) && facts.undertakings_given)
    }

    /// Check genuine agreement
    fn check_genuine_agreement(facts: &AgreementFacts) -> bool {
        facts.employees_properly_notified
            && facts.access_to_agreement
            && facts.access_to_award
            && facts.seven_days_notice
            && facts.majority_vote
    }

    /// Check mandatory terms included
    fn check_mandatory_terms(facts: &AgreementFacts) -> bool {
        facts.nominal_expiry_date && facts.dispute_resolution_term && facts.flexibility_term
    }

    /// Build reasoning
    fn build_reasoning(
        boot: bool,
        genuine: bool,
        terms: bool,
        consultation: bool,
        issues: &[String],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Enterprise agreement approval analysis".to_string());

        if boot {
            parts.push("BOOT satisfied: employees better off overall (s.193)".to_string());
        }

        if genuine {
            parts.push(
                "Genuinely agreed: proper notification and majority vote (s.188)".to_string(),
            );
        }

        if terms {
            parts.push("Mandatory terms included (s.186)".to_string());
        }

        if consultation {
            parts.push("Consultation term included (s.205)".to_string());
        }

        for issue in issues {
            parts.push(format!("Issue: {}", issue));
        }

        parts.join(". ")
    }
}

/// Facts for agreement analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgreementFacts {
    /// BOOT result
    pub boot_result: BootResult,
    /// Undertakings given
    pub undertakings_given: bool,
    /// Employees properly notified
    pub employees_properly_notified: bool,
    /// Access to proposed agreement
    pub access_to_agreement: bool,
    /// Access to relevant award
    pub access_to_award: bool,
    /// Seven days notice of vote
    pub seven_days_notice: bool,
    /// Majority vote achieved
    pub majority_vote: bool,
    /// Nominal expiry date included
    pub nominal_expiry_date: bool,
    /// Dispute resolution term
    pub dispute_resolution_term: bool,
    /// Flexibility term
    pub flexibility_term: bool,
    /// Consultation term included
    pub consultation_term_included: bool,
}

/// Result of agreement approval analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementApprovalResult {
    /// Agreement approved
    pub approved: bool,
    /// BOOT satisfied
    pub boot_satisfied: bool,
    /// Genuinely agreed
    pub genuinely_agreed: bool,
    /// Mandatory terms included
    pub mandatory_terms_included: bool,
    /// Issues identified
    pub issues: Vec<String>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nes_maximum_hours_compliant() {
        let facts = NesFacts {
            applicable_standards: vec![NationalEmploymentStandard::MaximumWeeklyHours],
            weekly_hours: 38.0,
            ..Default::default()
        };

        let result = NesAnalyzer::check_compliance(&facts);
        assert!(result.compliant);
    }

    #[test]
    fn test_nes_maximum_hours_unreasonable() {
        let facts = NesFacts {
            applicable_standards: vec![NationalEmploymentStandard::MaximumWeeklyHours],
            weekly_hours: 55.0,
            additional_hours_reasonable: false,
            ..Default::default()
        };

        let result = NesAnalyzer::check_compliance(&facts);
        assert!(!result.compliant);
        assert!(result.issues.iter().any(|i| i.contains("not reasonable")));
    }

    #[test]
    fn test_nes_annual_leave() {
        let facts = NesFacts {
            employment_type: EmploymentType::FullTime,
            applicable_standards: vec![NationalEmploymentStandard::AnnualLeave],
            annual_leave_weeks: 4.0,
            ..Default::default()
        };

        let result = NesAnalyzer::check_compliance(&facts);
        assert!(result.compliant);
    }

    #[test]
    fn test_nes_annual_leave_shift_worker() {
        let facts = NesFacts {
            employment_type: EmploymentType::FullTime,
            applicable_standards: vec![NationalEmploymentStandard::AnnualLeave],
            annual_leave_weeks: 4.0,
            is_shift_worker: true,
            ..Default::default()
        };

        let result = NesAnalyzer::check_compliance(&facts);
        assert!(!result.compliant); // Shift workers need 5 weeks
    }

    #[test]
    fn test_minimum_notice_weeks() {
        assert_eq!(NesAnalyzer::minimum_notice_weeks(0.5), 1.0);
        assert_eq!(NesAnalyzer::minimum_notice_weeks(1.5), 2.0);
        assert_eq!(NesAnalyzer::minimum_notice_weeks(4.0), 3.0);
        assert_eq!(NesAnalyzer::minimum_notice_weeks(6.0), 4.0);
    }

    #[test]
    fn test_redundancy_weeks() {
        assert_eq!(NesAnalyzer::redundancy_weeks(0.5), 0.0);
        assert_eq!(NesAnalyzer::redundancy_weeks(1.5), 4.0);
        assert_eq!(NesAnalyzer::redundancy_weeks(3.5), 7.0);
        assert_eq!(NesAnalyzer::redundancy_weeks(9.5), 16.0);
    }

    #[test]
    fn test_general_protections_dismissal() {
        let facts = GeneralProtectionsFacts {
            adverse_action: Some(AdverseAction::Dismissal),
            protected_attributes: vec![ProtectedAttribute::WorkplaceRight],
            employee_establishes_prima_facie: true,
            employer_proves_other_reason: false,
            ..Default::default()
        };

        let result = GeneralProtectionsAnalyzer::analyze(&facts);
        assert!(result.contravention);
        assert!(
            result
                .available_remedies
                .contains(&GeneralProtectionsRemedy::Reinstatement)
        );
    }

    #[test]
    fn test_general_protections_employer_proves() {
        let facts = GeneralProtectionsFacts {
            adverse_action: Some(AdverseAction::Dismissal),
            protected_attributes: vec![ProtectedAttribute::IndustrialActivity],
            employee_establishes_prima_facie: true,
            employer_proves_other_reason: true,
            ..Default::default()
        };

        let result = GeneralProtectionsAnalyzer::analyze(&facts);
        assert!(!result.contravention);
    }

    #[test]
    fn test_enterprise_agreement_approval() {
        let facts = AgreementFacts {
            boot_result: BootResult::BetterOff,
            employees_properly_notified: true,
            access_to_agreement: true,
            access_to_award: true,
            seven_days_notice: true,
            majority_vote: true,
            nominal_expiry_date: true,
            dispute_resolution_term: true,
            flexibility_term: true,
            consultation_term_included: true,
            ..Default::default()
        };

        let result = EnterpriseAgreementAnalyzer::check_approval(&facts);
        assert!(result.approved);
        assert!(result.boot_satisfied);
        assert!(result.genuinely_agreed);
    }

    #[test]
    fn test_enterprise_agreement_boot_fail() {
        let facts = AgreementFacts {
            boot_result: BootResult::NotBetterOff,
            ..Default::default()
        };

        let result = EnterpriseAgreementAnalyzer::check_approval(&facts);
        assert!(!result.approved);
        assert!(result.issues.iter().any(|i| i.contains("BOOT")));
    }
}
