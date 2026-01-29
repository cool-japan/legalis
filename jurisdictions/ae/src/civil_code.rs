//! UAE Civil Transactions Law - Federal Law No. 5/1985
//!
//! The Civil Transactions Law (المعاملات المدنية) is the UAE's civil code,
//! governing contracts, obligations, torts, and property rights.
//!
//! ## Structure
//!
//! The law is divided into books covering:
//! 1. General Provisions
//! 2. Obligations (Contracts and Torts)
//! 3. Specific Contracts (Sale, Lease, etc.)
//! 4. Security Interests
//! 5. Proof and Limitation Periods
//!
//! ## Key Principles
//!
//! - Based on Islamic Sharia and Egyptian Civil Code
//! - Freedom of contract (within legal and moral limits)
//! - Good faith and fair dealing
//! - Protection of weaker parties

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for civil code operations
pub type CivilCodeResult<T> = Result<T, CivilCodeError>;

/// Contract formation requirements - Article 129
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFormation {
    /// Offer (إيجاب)
    pub offer: bool,
    /// Acceptance (قبول)
    pub acceptance: bool,
    /// Capacity (أهلية)
    pub capacity: bool,
    /// Lawful object (محل مشروع)
    pub lawful_object: bool,
    /// Lawful cause (سبب مشروع)
    pub lawful_cause: bool,
}

impl ContractFormation {
    /// Check if contract formation is valid
    pub fn is_valid(&self) -> bool {
        self.offer && self.acceptance && self.capacity && self.lawful_object && self.lawful_cause
    }

    /// Get missing elements
    pub fn missing_elements(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if !self.offer {
            missing.push("Offer (إيجاب)");
        }
        if !self.acceptance {
            missing.push("Acceptance (قبول)");
        }
        if !self.capacity {
            missing.push("Capacity (أهلية)");
        }
        if !self.lawful_object {
            missing.push("Lawful Object (محل مشروع)");
        }
        if !self.lawful_cause {
            missing.push("Lawful Cause (سبب مشروع)");
        }
        missing
    }
}

/// Contract types under UAE Civil Code
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale (بيع) - Article 489+
    Sale,
    /// Lease (إيجار) - Article 654+
    Lease,
    /// Gift (هبة) - Article 611+
    Gift,
    /// Loan (قرض) - Article 680+
    Loan,
    /// Pledge (رهن) - Article 1309+
    Pledge,
    /// Agency (وكالة) - Article 918+
    Agency,
    /// Partnership (شركة) - Article 869+
    Partnership,
    /// Guarantee (كفالة) - Article 1106+
    Guarantee,
    /// Service contract (عقد خدمة)
    Service,
}

impl ContractType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Sale => "بيع",
            Self::Lease => "إيجار",
            Self::Gift => "هبة",
            Self::Loan => "قرض",
            Self::Pledge => "رهن",
            Self::Agency => "وكالة",
            Self::Partnership => "شركة",
            Self::Guarantee => "كفالة",
            Self::Service => "عقد خدمة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Sale => "Sale",
            Self::Lease => "Lease",
            Self::Gift => "Gift",
            Self::Loan => "Loan",
            Self::Pledge => "Pledge",
            Self::Agency => "Agency",
            Self::Partnership => "Partnership",
            Self::Guarantee => "Guarantee",
            Self::Service => "Service Contract",
        }
    }

    /// Check if contract requires written form
    pub fn requires_written_form(&self) -> bool {
        matches!(self, Self::Pledge | Self::Guarantee | Self::Partnership)
    }

    /// Starting article number in Civil Transactions Law
    pub fn article_reference(&self) -> u32 {
        match self {
            Self::Sale => 489,
            Self::Lease => 654,
            Self::Gift => 611,
            Self::Loan => 680,
            Self::Pledge => 1309,
            Self::Agency => 918,
            Self::Partnership => 869,
            Self::Guarantee => 1106,
            Self::Service => 872,
        }
    }
}

/// Grounds for contract invalidity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvalidityGround {
    /// Lack of capacity (انعدام الأهلية)
    LackOfCapacity,
    /// Absence of consent (انعدام الرضا)
    AbsenceOfConsent,
    /// Fraud (تدليس) - Article 185
    Fraud,
    /// Duress (إكراه) - Article 177
    Duress,
    /// Mistake (غلط) - Article 186
    Mistake,
    /// Unlawful object (محل غير مشروع)
    UnlawfulObject,
    /// Unlawful cause (سبب غير مشروع)
    UnlawfulCause,
    /// Violation of public order (مخالفة النظام العام)
    PublicOrderViolation,
}

impl InvalidityGround {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::LackOfCapacity => "انعدام الأهلية",
            Self::AbsenceOfConsent => "انعدام الرضا",
            Self::Fraud => "تدليس",
            Self::Duress => "إكراه",
            Self::Mistake => "غلط",
            Self::UnlawfulObject => "محل غير مشروع",
            Self::UnlawfulCause => "سبب غير مشروع",
            Self::PublicOrderViolation => "مخالفة النظام العام",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::LackOfCapacity => "Lack of Capacity",
            Self::AbsenceOfConsent => "Absence of Consent",
            Self::Fraud => "Fraud",
            Self::Duress => "Duress",
            Self::Mistake => "Mistake",
            Self::UnlawfulObject => "Unlawful Object",
            Self::UnlawfulCause => "Unlawful Cause",
            Self::PublicOrderViolation => "Violation of Public Order",
        }
    }

    /// Check if contract is void (باطل) or voidable (قابل للإبطال)
    pub fn is_void(&self) -> bool {
        matches!(
            self,
            Self::UnlawfulObject | Self::UnlawfulCause | Self::PublicOrderViolation
        )
    }

    /// Check if contract is voidable
    pub fn is_voidable(&self) -> bool {
        !self.is_void()
    }
}

/// Tort liability types - Article 282+
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TortLiability {
    /// Direct harm (ضرر مباشر)
    DirectHarm,
    /// Indirect harm (ضرر غير مباشر)
    IndirectHarm,
    /// Joint liability (تضامن)
    JointLiability,
    /// Strict liability (مسؤولية مطلقة)
    StrictLiability,
}

impl TortLiability {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::DirectHarm => "ضرر مباشر",
            Self::IndirectHarm => "ضرر غير مباشر",
            Self::JointLiability => "تضامن",
            Self::StrictLiability => "مسؤولية مطلقة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::DirectHarm => "Direct Harm",
            Self::IndirectHarm => "Indirect Harm",
            Self::JointLiability => "Joint and Several Liability",
            Self::StrictLiability => "Strict Liability",
        }
    }
}

/// Limitation periods - Article 473+
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LimitationPeriod {
    /// General limitation (15 years)
    General,
    /// Contractual claims (15 years)
    Contractual,
    /// Tort claims (3 years from knowledge, 15 years absolute)
    Tort,
    /// Commercial transactions (10 years)
    Commercial,
    /// Periodic payments (5 years)
    PeriodicPayments,
}

impl LimitationPeriod {
    /// Get period in years
    pub fn years(&self) -> u32 {
        match self {
            Self::General | Self::Contractual => 15,
            Self::Tort => 3, // From knowledge, 15 absolute
            Self::Commercial => 10,
            Self::PeriodicPayments => 5,
        }
    }

    /// Get name in Arabic
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::General => "المدة العامة (15 سنة)",
            Self::Contractual => "الدعاوى العقدية (15 سنة)",
            Self::Tort => "الدعاوى التقصيرية (3 سنوات من العلم)",
            Self::Commercial => "المعاملات التجارية (10 سنوات)",
            Self::PeriodicPayments => "المدفوعات الدورية (5 سنوات)",
        }
    }

    /// Get name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::General => "General (15 years)",
            Self::Contractual => "Contractual Claims (15 years)",
            Self::Tort => "Tort Claims (3 years from knowledge)",
            Self::Commercial => "Commercial Transactions (10 years)",
            Self::PeriodicPayments => "Periodic Payments (5 years)",
        }
    }
}

/// Compensation for damages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compensation {
    /// Actual loss (الضرر الفعلي)
    pub actual_loss: Aed,
    /// Lost profit (الكسب الفائت)
    pub lost_profit: Aed,
    /// Moral damages (الضرر المعنوي)
    pub moral_damages: Option<Aed>,
    /// Total compensation
    pub total: Aed,
}

impl Compensation {
    /// Calculate total compensation
    pub fn calculate(actual_loss: Aed, lost_profit: Aed, moral_damages: Option<Aed>) -> Self {
        let total = actual_loss + lost_profit + moral_damages.unwrap_or_else(|| Aed::from_fils(0));

        Self {
            actual_loss,
            lost_profit,
            moral_damages,
            total,
        }
    }
}

/// Civil code errors
#[derive(Debug, Error)]
pub enum CivilCodeError {
    /// Invalid contract formation
    #[error("تكوين العقد غير صحيح: {missing_elements:?}")]
    InvalidContractFormation { missing_elements: Vec<String> },

    /// Contract invalidity
    #[error("العقد باطل أو قابل للإبطال: {ground}")]
    ContractInvalidity { ground: String },

    /// Limitation period expired
    #[error("انقضاء مدة التقادم: {period} سنوات")]
    LimitationExpired { period: u32 },

    /// Unlawful contract
    #[error("العقد غير مشروع: {reason}")]
    UnlawfulContract { reason: String },
}

/// Validate contract formation
pub fn validate_contract_formation(formation: &ContractFormation) -> CivilCodeResult<()> {
    if !formation.is_valid() {
        let missing = formation
            .missing_elements()
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Err(CivilCodeError::InvalidContractFormation {
            missing_elements: missing,
        });
    }
    Ok(())
}

/// Get civil code compliance checklist
pub fn get_civil_code_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("الإيجاب والقبول", "Offer and Acceptance", "Article 129"),
        ("الأهلية", "Capacity", "Article 163+"),
        ("المحل المشروع", "Lawful Object", "Article 198+"),
        ("السبب المشروع", "Lawful Cause", "Article 202+"),
        ("الرضا الصحيح", "Valid Consent", "Article 177+"),
        (
            "الكتابة (عند الحاجة)",
            "Writing (if required)",
            "Article 9+",
        ),
        ("حسن النية", "Good Faith", "Article 246"),
        ("الإثبات", "Proof", "Article 1+"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_formation_valid() {
        let formation = ContractFormation {
            offer: true,
            acceptance: true,
            capacity: true,
            lawful_object: true,
            lawful_cause: true,
        };

        assert!(formation.is_valid());
        assert!(validate_contract_formation(&formation).is_ok());
    }

    #[test]
    fn test_contract_formation_missing_elements() {
        let formation = ContractFormation {
            offer: true,
            acceptance: false,
            capacity: true,
            lawful_object: false,
            lawful_cause: true,
        };

        assert!(!formation.is_valid());
        let missing = formation.missing_elements();
        assert_eq!(missing.len(), 2);
        assert!(validate_contract_formation(&formation).is_err());
    }

    #[test]
    fn test_contract_types() {
        let sale = ContractType::Sale;
        assert_eq!(sale.name_ar(), "بيع");
        assert_eq!(sale.name_en(), "Sale");
        assert_eq!(sale.article_reference(), 489);
        assert!(!sale.requires_written_form());

        let pledge = ContractType::Pledge;
        assert!(pledge.requires_written_form());
    }

    #[test]
    fn test_invalidity_grounds() {
        let fraud = InvalidityGround::Fraud;
        assert_eq!(fraud.name_ar(), "تدليس");
        assert!(!fraud.is_void());
        assert!(fraud.is_voidable());

        let unlawful = InvalidityGround::UnlawfulObject;
        assert!(unlawful.is_void());
        assert!(!unlawful.is_voidable());
    }

    #[test]
    fn test_limitation_periods() {
        assert_eq!(LimitationPeriod::General.years(), 15);
        assert_eq!(LimitationPeriod::Tort.years(), 3);
        assert_eq!(LimitationPeriod::Commercial.years(), 10);
    }

    #[test]
    fn test_compensation_calculation() {
        let comp = Compensation::calculate(
            Aed::from_dirhams(10_000),
            Aed::from_dirhams(5_000),
            Some(Aed::from_dirhams(3_000)),
        );

        assert_eq!(comp.total.dirhams(), 18_000);
        assert_eq!(comp.actual_loss.dirhams(), 10_000);
    }

    #[test]
    fn test_compensation_no_moral_damages() {
        let comp =
            Compensation::calculate(Aed::from_dirhams(20_000), Aed::from_dirhams(10_000), None);

        assert_eq!(comp.total.dirhams(), 30_000);
        assert!(comp.moral_damages.is_none());
    }

    #[test]
    fn test_tort_liability() {
        let direct = TortLiability::DirectHarm;
        assert_eq!(direct.name_ar(), "ضرر مباشر");
        assert_eq!(direct.name_en(), "Direct Harm");
    }

    #[test]
    fn test_civil_code_checklist() {
        let checklist = get_civil_code_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 8);
    }
}
