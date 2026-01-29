//! South African Insolvency Law
//!
//! Liquidation and insolvency of individuals and companies.
//!
//! ## Key Legislation
//!
//! - Insolvency Act 24 of 1936 (individual insolvency)
//! - Companies Act 71 of 2008 (corporate insolvency, business rescue)
//! - Close Corporations Act 69 of 1984 (CC insolvency)
//!
//! ## Types of Insolvency
//!
//! - Sequestration (individuals, partnerships, trusts)
//! - Liquidation (companies, close corporations)
//! - Business rescue (Chapter 6 Companies Act)

use crate::common::Zar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for insolvency operations
pub type InsolvencyResult<T> = Result<T, InsolvencyError>;

/// Sequestration (individual insolvency) - Insolvency Act
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequestration {
    /// Insolvent person
    pub debtor: String,
    /// Total liabilities
    pub total_liabilities: Zar,
    /// Total assets
    pub total_assets: Zar,
    /// Type of sequestration
    pub sequestration_type: SequestrationType,
    /// Trustee appointed
    pub trustee_appointed: bool,
}

impl Sequestration {
    /// Check if insolvent (liabilities exceed assets)
    pub fn is_insolvent(&self) -> bool {
        self.total_liabilities.cents() > self.total_assets.cents()
    }

    /// Calculate deficit
    pub fn deficit(&self) -> Zar {
        Zar::from_cents(self.total_liabilities.cents() - self.total_assets.cents())
    }

    /// Advantage to creditors test (s12 Insolvency Act)
    pub fn advantage_to_creditors(&self, sequestration_costs: Zar) -> bool {
        // Assets must exceed costs for there to be advantage
        self.total_assets.cents() > sequestration_costs.cents()
    }
}

/// Sequestration types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SequestrationType {
    /// Voluntary surrender (s3 - debtor applies)
    VoluntarySurrender,
    /// Compulsory sequestration (s9 - creditor applies)
    CompulsorySequestration,
}

impl SequestrationType {
    /// Requirements for sequestration
    pub fn requirements(&self) -> Vec<&'static str> {
        match self {
            Self::VoluntarySurrender => vec![
                "Debtor is insolvent",
                "Advantage to creditors",
                "Application by debtor",
            ],
            Self::CompulsorySequestration => vec![
                "Debtor committed act of insolvency (s8)",
                "Advantage to creditors",
                "Liquidated claim ≥ R100",
            ],
        }
    }
}

/// Acts of insolvency (s8 Insolvency Act)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActOfInsolvency {
    /// Published statement of inability to pay debts
    PublishedInability,
    /// Absconded from residence with intent to evade creditors
    Absconded,
    /// Made disposition of property to defraud creditors
    FraudulentDisposition,
    /// Made disposition of property with effect of prejudicing creditors
    PrejudicialDisposition,
    /// Removed or attempted to remove property to evade creditors
    RemovedProperty,
    /// In writing notified creditor of inability to pay debts
    NotifiedInability,
    /// Judgment debtor failed to satisfy judgment within 10 days
    JudgmentNotSatisfied,
}

/// Liquidation (corporate insolvency) - Companies Act
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Liquidation {
    /// Company name
    pub company: String,
    /// Liquidation type
    pub liquidation_type: LiquidationType,
    /// Liquidator appointed
    pub liquidator_appointed: bool,
    /// Total liabilities
    pub total_liabilities: Zar,
    /// Total assets
    pub total_assets: Zar,
}

impl Liquidation {
    /// Check if insolvent
    pub fn is_insolvent(&self) -> bool {
        self.total_liabilities.cents() > self.total_assets.cents()
    }

    /// Check if unable to pay debts as they fall due
    pub fn unable_to_pay_debts(&self) -> bool {
        // Commercial insolvency test (cash flow test)
        true // Simplified - would assess current liabilities vs current assets
    }
}

/// Liquidation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiquidationType {
    /// Voluntary liquidation (members' or creditors')
    Voluntary,
    /// Compulsory liquidation by court order
    Compulsory,
    /// Provisional liquidation
    Provisional,
}

/// Business rescue (Chapter 6 Companies Act)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessRescue {
    /// Company name
    pub company: String,
    /// Financially distressed
    pub financially_distressed: bool,
    /// Reasonable prospect of rescue
    pub reasonable_prospect: bool,
    /// Business rescue practitioner appointed
    pub practitioner_appointed: bool,
    /// Business rescue plan published
    pub plan_published: bool,
    /// Moratorium on legal proceedings
    pub moratorium_active: bool,
}

impl BusinessRescue {
    /// Check if qualifies for business rescue (s128-129)
    pub fn qualifies(&self) -> bool {
        self.financially_distressed && self.reasonable_prospect
    }

    /// Financially distressed (s128(1)(f))
    pub fn is_financially_distressed_definition() -> &'static str {
        "Appears reasonably unlikely to pay all debts as they fall due within 6 months, or appears reasonably likely to become insolvent within 6 months"
    }
}

/// Distribution of assets (order of preference)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreditorClass {
    /// Costs of sequestration/liquidation
    CostsOfProceedings,
    /// Preferent claims (employees, SARS)
    PreferentCreditors,
    /// Concurrent creditors (ordinary unsecured)
    ConcurrentCreditors,
    /// Subordinated creditors (shareholders)
    SubordinatedCreditors,
}

impl CreditorClass {
    /// Get distribution priority (lower number = higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            Self::CostsOfProceedings => 1,
            Self::PreferentCreditors => 2,
            Self::ConcurrentCreditors => 3,
            Self::SubordinatedCreditors => 4,
        }
    }
}

/// Preferent claims (s98A Insolvency Act, s98 Companies Act)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreferentClaim {
    /// Employee remuneration (3 months, capped)
    EmployeeRemuneration,
    /// Employee leave pay
    EmployeeLeavePay,
    /// Employee retrenchment pay
    EmployeeRetrenchmentPay,
    /// SARS (certain taxes)
    SarsTaxClaims,
}

impl PreferentClaim {
    /// Get maximum amount for employee claims (R40,000 per employee)
    pub fn employee_claim_cap() -> Zar {
        Zar::from_rands(40_000)
    }
}

/// Voidable dispositions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoidableDisposition {
    /// Undue preference (s29 Insolvency Act)
    UnduePreference,
    /// Collusive dealing (s31)
    CollusiveDealing,
    /// Impeachable disposition (s26)
    ImpeachableDisposition,
}

impl VoidableDisposition {
    /// Suspicious period (6 months for undue preference)
    pub fn suspicious_period_months(&self) -> u8 {
        match self {
            Self::UnduePreference => 6,
            Self::CollusiveDealing => 24, // 2 years
            Self::ImpeachableDisposition => 6,
        }
    }
}

/// Rehabilitation (discharge from sequestration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rehabilitation {
    /// Automatic rehabilitation period (10 years)
    pub automatic_years: u8,
    /// Application for rehabilitation (4 years minimum)
    pub application_minimum_years: u8,
    /// All creditors paid in full
    pub creditors_paid_in_full: bool,
}

impl Default for Rehabilitation {
    fn default() -> Self {
        Self {
            automatic_years: 10,
            application_minimum_years: 4,
            creditors_paid_in_full: false,
        }
    }
}

impl Rehabilitation {
    /// Can apply for rehabilitation
    pub fn can_apply(&self, years_since_sequestration: u8) -> bool {
        years_since_sequestration >= self.application_minimum_years
    }
}

/// Insolvency errors
#[derive(Debug, Error)]
pub enum InsolvencyError {
    /// Not insolvent
    #[error("Not insolvent (assets R{assets} exceed liabilities R{liabilities})")]
    NotInsolvent { assets: i64, liabilities: i64 },

    /// No advantage to creditors
    #[error("No advantage to creditors (s12 Insolvency Act)")]
    NoAdvantage,

    /// No reasonable prospect of rescue
    #[error("No reasonable prospect of business rescue success")]
    NoReasonableProspect,

    /// Voidable disposition
    #[error("Voidable disposition ({disposition_type} within {months} months)")]
    VoidableDisposition {
        disposition_type: String,
        months: u8,
    },

    /// Preferent claim exceeds cap
    #[error("Preferent claim exceeds cap (claim R{claim}, cap R{cap})")]
    PreferentClaimExceedsCap { claim: i64, cap: i64 },
}

/// Validate sequestration application
pub fn validate_sequestration(sequestration: &Sequestration, costs: Zar) -> InsolvencyResult<()> {
    if !sequestration.is_insolvent() {
        return Err(InsolvencyError::NotInsolvent {
            assets: sequestration.total_assets.rands(),
            liabilities: sequestration.total_liabilities.rands(),
        });
    }

    if !sequestration.advantage_to_creditors(costs) {
        return Err(InsolvencyError::NoAdvantage);
    }

    Ok(())
}

/// Validate business rescue application
pub fn validate_business_rescue(rescue: &BusinessRescue) -> InsolvencyResult<()> {
    if !rescue.financially_distressed {
        return Err(InsolvencyError::NotInsolvent {
            assets: 0,
            liabilities: 0,
        });
    }

    if !rescue.reasonable_prospect {
        return Err(InsolvencyError::NoReasonableProspect);
    }

    Ok(())
}

/// Get insolvency compliance checklist
pub fn get_insolvency_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Insolvency test applied", "s3/s8 Insolvency Act"),
        ("Advantage to creditors assessed", "s12"),
        ("Trustee/liquidator appointed", "s18/s370"),
        ("First meeting of creditors", "s40/s401"),
        ("Proof of claims submitted", "s44"),
        ("Voidable dispositions challenged", "s26-31"),
        ("Preferent claims identified", "s98A/s98"),
        ("Distribution account prepared", "s109/s416"),
        ("Rehabilitation considered", "s124-127"),
        ("Business rescue as alternative (if company)", "s128-154"),
        ("Moratorium on proceedings (if business rescue)", "s133"),
        ("Business rescue plan drafted", "s150"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequestration_insolvent() {
        let sequestration = Sequestration {
            debtor: "John Doe".to_string(),
            total_liabilities: Zar::from_rands(500_000),
            total_assets: Zar::from_rands(200_000),
            sequestration_type: SequestrationType::VoluntarySurrender,
            trustee_appointed: false,
        };
        assert!(sequestration.is_insolvent());
        assert_eq!(sequestration.deficit().rands(), 300_000);
    }

    #[test]
    fn test_sequestration_advantage() {
        let sequestration = Sequestration {
            debtor: "Jane Doe".to_string(),
            total_liabilities: Zar::from_rands(100_000),
            total_assets: Zar::from_rands(50_000),
            sequestration_type: SequestrationType::VoluntarySurrender,
            trustee_appointed: false,
        };
        let costs = Zar::from_rands(30_000);
        assert!(sequestration.advantage_to_creditors(costs));
    }

    #[test]
    fn test_validate_sequestration() {
        let sequestration = Sequestration {
            debtor: "Test Debtor".to_string(),
            total_liabilities: Zar::from_rands(200_000),
            total_assets: Zar::from_rands(100_000),
            sequestration_type: SequestrationType::VoluntarySurrender,
            trustee_appointed: true,
        };
        let costs = Zar::from_rands(50_000);
        assert!(validate_sequestration(&sequestration, costs).is_ok());
    }

    #[test]
    fn test_validate_sequestration_not_insolvent() {
        let sequestration = Sequestration {
            debtor: "Solvent Person".to_string(),
            total_liabilities: Zar::from_rands(50_000),
            total_assets: Zar::from_rands(100_000),
            sequestration_type: SequestrationType::VoluntarySurrender,
            trustee_appointed: false,
        };
        let costs = Zar::from_rands(10_000);
        assert!(validate_sequestration(&sequestration, costs).is_err());
    }

    #[test]
    fn test_liquidation_insolvent() {
        let liquidation = Liquidation {
            company: "Insolvent Ltd".to_string(),
            liquidation_type: LiquidationType::Compulsory,
            liquidator_appointed: true,
            total_liabilities: Zar::from_rands(1_000_000),
            total_assets: Zar::from_rands(500_000),
        };
        assert!(liquidation.is_insolvent());
    }

    #[test]
    fn test_business_rescue_qualifies() {
        let rescue = BusinessRescue {
            company: "Distressed Company (Pty) Ltd".to_string(),
            financially_distressed: true,
            reasonable_prospect: true,
            practitioner_appointed: true,
            plan_published: false,
            moratorium_active: true,
        };
        assert!(rescue.qualifies());
        assert!(validate_business_rescue(&rescue).is_ok());
    }

    #[test]
    fn test_business_rescue_no_prospect() {
        let rescue = BusinessRescue {
            company: "Failing Company".to_string(),
            financially_distressed: true,
            reasonable_prospect: false,
            practitioner_appointed: false,
            plan_published: false,
            moratorium_active: false,
        };
        assert!(!rescue.qualifies());
        assert!(validate_business_rescue(&rescue).is_err());
    }

    #[test]
    fn test_creditor_class_priority() {
        assert_eq!(CreditorClass::CostsOfProceedings.priority(), 1);
        assert_eq!(CreditorClass::PreferentCreditors.priority(), 2);
        assert_eq!(CreditorClass::ConcurrentCreditors.priority(), 3);
        assert_eq!(CreditorClass::SubordinatedCreditors.priority(), 4);
    }

    #[test]
    fn test_preferent_claim_cap() {
        assert_eq!(PreferentClaim::employee_claim_cap().rands(), 40_000);
    }

    #[test]
    fn test_voidable_disposition_periods() {
        assert_eq!(
            VoidableDisposition::UnduePreference.suspicious_period_months(),
            6
        );
        assert_eq!(
            VoidableDisposition::CollusiveDealing.suspicious_period_months(),
            24
        );
    }

    #[test]
    fn test_rehabilitation() {
        let rehab = Rehabilitation::default();
        assert_eq!(rehab.automatic_years, 10);
        assert_eq!(rehab.application_minimum_years, 4);
        assert!(rehab.can_apply(5));
        assert!(!rehab.can_apply(3));
    }

    #[test]
    fn test_insolvency_checklist() {
        let checklist = get_insolvency_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
