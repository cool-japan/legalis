//! Insurance Law Types (ປະເພດກົດໝາຍປະກັນໄພ)
//!
//! Type definitions for Lao insurance law based on the
//! **Law on Insurance (Lao PDR), No. 06/NA, 2011**
//! (ກົດໝາຍວ່າດ້ວຍການປະກັນໄພ).
//!
//! # Legal References
//!
//! - Law on Insurance 2011 (No. 06/NA) - the primary statute, administered by the
//!   Ministry of Finance.
//! - The classes of insurance and the principles of insurable interest, utmost
//!   good faith, indemnity and subrogation track the internationally accepted
//!   foundations of insurance law adopted by the Lao regime.
//!
//! # Numeric thresholds
//!
//! Where the underlying statute fixes a quantifiable requirement (such as the
//! solvency principle that admitted assets must be at least equal to liabilities)
//! it is encoded as a named, documented constant. Monetary figures the statute
//! does not fix precisely (such as a specific minimum registered capital in LAK)
//! are not fabricated; instead the corresponding fields are validated for internal
//! consistency (for example, registered capital must be positive) rather than as
//! invented statutory figures.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Law on Insurance 2011 (No. 06/NA)
// ============================================================================

/// Whether motor vehicle third-party liability insurance is compulsory in Lao PDR.
/// Motor third-party liability cover is mandatory under the insurance regime.
/// ການປະກັນໄພຄວາມຮັບຜິດຊອບຕໍ່ບຸກຄົນທີສາມຂອງລົດເປັນການບັງຄັບ
pub const MOTOR_THIRD_PARTY_COMPULSORY: bool = true;

/// Minimum solvency ratio expressed as a percentage of liabilities.
///
/// The solvency principle requires an insurer's admitted assets to be at least
/// equal to its liabilities, i.e. admitted assets must be at least 100% of
/// liabilities.
/// ອັດຕາສ່ວນຄວາມສາມາດຊຳລະໜີ້ຂັ້ນຕ່ຳເປັນເປີເຊັນ
pub const MIN_SOLVENCY_RATIO_PERCENT: u32 = 100;

/// Number of insurance classes modelled by this module.
/// ຈຳນວນປະເພດການປະກັນໄພ
pub const INSURANCE_CLASS_COUNT: usize = 10;

// ============================================================================
// Insurance Classes - ປະເພດການປະກັນໄພ
// ============================================================================

/// Class of insurance - ປະເພດການປະກັນໄພ
///
/// The recognised classes of insurance business. Motor vehicle third-party
/// liability insurance is compulsory; the remaining classes are taken out
/// voluntarily.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InsuranceClass {
    /// Life insurance (ປະກັນຊີວິດ)
    Life,
    /// Health / medical insurance (ປະກັນສຸຂະພາບ)
    Health,
    /// Motor vehicle insurance, incl. compulsory third-party liability (ປະກັນລົດ)
    Motor,
    /// Property and fire insurance (ປະກັນຊັບສິນ ແລະ ອັກຄີໄພ)
    Property,
    /// Liability insurance (ປະກັນຄວາມຮັບຜິດຊອບ)
    Liability,
    /// Marine insurance (ປະກັນທາງທະເລ)
    Marine,
    /// Agricultural insurance (ປະກັນກະສິກຳ)
    Agricultural,
    /// Travel insurance (ປະກັນການເດີນທາງ)
    Travel,
    /// Reinsurance (ການປະກັນໄພຕໍ່)
    Reinsurance,
    /// Microinsurance (ປະກັນໄພຈຸລະພາກ)
    Microinsurance,
}

impl InsuranceClass {
    /// All insurance classes modelled by this module.
    pub fn all() -> [InsuranceClass; INSURANCE_CLASS_COUNT] {
        [
            InsuranceClass::Life,
            InsuranceClass::Health,
            InsuranceClass::Motor,
            InsuranceClass::Property,
            InsuranceClass::Liability,
            InsuranceClass::Marine,
            InsuranceClass::Agricultural,
            InsuranceClass::Travel,
            InsuranceClass::Reinsurance,
            InsuranceClass::Microinsurance,
        ]
    }

    /// Whether this class of insurance is compulsory in Lao PDR.
    ///
    /// Motor vehicle third-party liability insurance is the compulsory class
    /// (see [`MOTOR_THIRD_PARTY_COMPULSORY`]); all other classes are voluntary.
    pub fn is_compulsory(&self) -> bool {
        match self {
            InsuranceClass::Motor => MOTOR_THIRD_PARTY_COMPULSORY,
            InsuranceClass::Life
            | InsuranceClass::Health
            | InsuranceClass::Property
            | InsuranceClass::Liability
            | InsuranceClass::Marine
            | InsuranceClass::Agricultural
            | InsuranceClass::Travel
            | InsuranceClass::Reinsurance
            | InsuranceClass::Microinsurance => false,
        }
    }

    /// Whether this class is an indemnity (non-life) class to which the principle
    /// of indemnity applies.
    pub fn is_indemnity_class(&self) -> bool {
        !matches!(self, InsuranceClass::Life)
    }

    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            InsuranceClass::Life => "ປະກັນຊີວິດ",
            InsuranceClass::Health => "ປະກັນສຸຂະພາບ",
            InsuranceClass::Motor => "ປະກັນລົດ",
            InsuranceClass::Property => "ປະກັນຊັບສິນ ແລະ ອັກຄີໄພ",
            InsuranceClass::Liability => "ປະກັນຄວາມຮັບຜິດຊອບ",
            InsuranceClass::Marine => "ປະກັນທາງທະເລ",
            InsuranceClass::Agricultural => "ປະກັນກະສິກຳ",
            InsuranceClass::Travel => "ປະກັນການເດີນທາງ",
            InsuranceClass::Reinsurance => "ການປະກັນໄພຕໍ່",
            InsuranceClass::Microinsurance => "ປະກັນໄພຈຸລະພາກ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            InsuranceClass::Life => "life insurance",
            InsuranceClass::Health => "health insurance",
            InsuranceClass::Motor => "motor vehicle insurance",
            InsuranceClass::Property => "property and fire insurance",
            InsuranceClass::Liability => "liability insurance",
            InsuranceClass::Marine => "marine insurance",
            InsuranceClass::Agricultural => "agricultural insurance",
            InsuranceClass::Travel => "travel insurance",
            InsuranceClass::Reinsurance => "reinsurance",
            InsuranceClass::Microinsurance => "microinsurance",
        }
    }
}

/// Type of insurer (insurance undertaking) - ປະເພດບໍລິສັດປະກັນໄພ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InsurerType {
    /// Insurer writing life insurance business (ບໍລິສັດປະກັນຊີວິດ)
    LifeInsurer,
    /// Insurer writing non-life (general) insurance business (ບໍລິສັດປະກັນໄພທົ່ວໄປ)
    NonLifeInsurer,
    /// Composite insurer writing both life and non-life business (ບໍລິສັດປະກັນໄພປະສົມ)
    CompositeInsurer,
    /// Reinsurer accepting insurance risks from other insurers (ບໍລິສັດປະກັນໄພຕໍ່)
    Reinsurer,
    /// Microinsurer writing microinsurance business (ບໍລິສັດປະກັນໄພຈຸລະພາກ)
    Microinsurer,
}

impl InsurerType {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            InsurerType::LifeInsurer => "ບໍລິສັດປະກັນຊີວິດ",
            InsurerType::NonLifeInsurer => "ບໍລິສັດປະກັນໄພທົ່ວໄປ",
            InsurerType::CompositeInsurer => "ບໍລິສັດປະກັນໄພປະສົມ",
            InsurerType::Reinsurer => "ບໍລິສັດປະກັນໄພຕໍ່",
            InsurerType::Microinsurer => "ບໍລິສັດປະກັນໄພຈຸລະພາກ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            InsurerType::LifeInsurer => "life insurer",
            InsurerType::NonLifeInsurer => "non-life insurer",
            InsurerType::CompositeInsurer => "composite insurer",
            InsurerType::Reinsurer => "reinsurer",
            InsurerType::Microinsurer => "microinsurer",
        }
    }
}

/// Status of an insurance policy - ສະຖານະຂອງສັນຍາປະກັນໄພ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PolicyStatus {
    /// Proposed but not yet in force (ສະເໜີ)
    Proposed,
    /// In force / active (ມີຜົນບັງຄັບໃຊ້)
    Active,
    /// Lapsed, e.g. for non-payment of premium (ໝົດຜົນ)
    Lapsed,
    /// Expired at the end of the policy term (ໝົດອາຍຸ)
    Expired,
    /// Cancelled before the end of the term (ຍົກເລີກ)
    Cancelled,
}

impl PolicyStatus {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            PolicyStatus::Proposed => "ສະເໜີ",
            PolicyStatus::Active => "ມີຜົນບັງຄັບໃຊ້",
            PolicyStatus::Lapsed => "ໝົດຜົນ",
            PolicyStatus::Expired => "ໝົດອາຍຸ",
            PolicyStatus::Cancelled => "ຍົກເລີກ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            PolicyStatus::Proposed => "proposed",
            PolicyStatus::Active => "active",
            PolicyStatus::Lapsed => "lapsed",
            PolicyStatus::Expired => "expired",
            PolicyStatus::Cancelled => "cancelled",
        }
    }
}

/// Status of an insurance claim - ສະຖານະຂອງການຮຽກຮ້ອງຄ່າສິນໄໝ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClaimStatus {
    /// Notified to the insurer (ແຈ້ງແລ້ວ)
    Notified,
    /// Under assessment by the insurer (ກຳລັງປະເມີນ)
    UnderAssessment,
    /// Approved for payment (ອະນຸມັດແລ້ວ)
    Approved,
    /// Paid to the claimant (ຈ່າຍແລ້ວ)
    Paid,
    /// Rejected by the insurer (ປະຕິເສດ)
    Rejected,
}

impl ClaimStatus {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            ClaimStatus::Notified => "ແຈ້ງແລ້ວ",
            ClaimStatus::UnderAssessment => "ກຳລັງປະເມີນ",
            ClaimStatus::Approved => "ອະນຸມັດແລ້ວ",
            ClaimStatus::Paid => "ຈ່າຍແລ້ວ",
            ClaimStatus::Rejected => "ປະຕິເສດ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            ClaimStatus::Notified => "notified",
            ClaimStatus::UnderAssessment => "under assessment",
            ClaimStatus::Approved => "approved",
            ClaimStatus::Paid => "paid",
            ClaimStatus::Rejected => "rejected",
        }
    }
}

/// Type of insurance intermediary - ປະເພດຕົວກາງປະກັນໄພ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IntermediaryType {
    /// Insurance agent acting for an insurer (ຕົວແທນປະກັນໄພ)
    Agent,
    /// Insurance broker acting for the insured (ນາຍໜ້າປະກັນໄພ)
    Broker,
}

impl IntermediaryType {
    /// Get the Lao name - ຮັບຊື່ເປັນພາສາລາວ
    pub fn lao_name(&self) -> &'static str {
        match self {
            IntermediaryType::Agent => "ຕົວແທນປະກັນໄພ",
            IntermediaryType::Broker => "ນາຍໜ້າປະກັນໄພ",
        }
    }

    /// Get the English name.
    pub fn english_name(&self) -> &'static str {
        match self {
            IntermediaryType::Agent => "insurance agent",
            IntermediaryType::Broker => "insurance broker",
        }
    }
}

// ============================================================================
// Insurers - ບໍລິສັດປະກັນໄພ
// ============================================================================

/// Insurer (insurance undertaking) - ບໍລິສັດປະກັນໄພ
///
/// Insurers must be licensed by the Ministry of Finance, hold positive registered
/// capital, and remain solvent (admitted assets at least equal to liabilities).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Insurer {
    /// Name of the insurer (ຊື່ບໍລິສັດປະກັນໄພ)
    pub name: String,
    /// Type of insurer (ປະເພດບໍລິສັດປະກັນໄພ)
    pub insurer_type: InsurerType,
    /// Registered capital in LAK (ທຶນຈົດທະບຽນເປັນກີບ)
    pub registered_capital_lak: u64,
    /// Whether the insurer is licensed by the Ministry of Finance (ໄດ້ຮັບໃບອະນຸຍາດ)
    pub licensed: bool,
    /// Admitted assets in LAK (ຊັບສິນທີ່ຮັບຮູ້ໄດ້ເປັນກີບ)
    pub admitted_assets_lak: u64,
    /// Liabilities in LAK (ໜີ້ສິນເປັນກີບ)
    pub liabilities_lak: u64,
}

impl Insurer {
    /// Whether the insurer satisfies the solvency principle: admitted assets must
    /// be at least [`MIN_SOLVENCY_RATIO_PERCENT`]% of liabilities.
    pub fn is_solvent(&self) -> bool {
        let assets = u128::from(self.admitted_assets_lak);
        let liabilities = u128::from(self.liabilities_lak);
        let ratio = u128::from(MIN_SOLVENCY_RATIO_PERCENT);
        assets * 100 >= liabilities * ratio
    }
}

// ============================================================================
// Insurance Policies - ສັນຍາປະກັນໄພ
// ============================================================================

/// Insurance policy (contract) - ສັນຍາປະກັນໄພ
///
/// Models the essential elements of an insurance contract: insurable interest,
/// premium, sum insured and policy duration. Indemnity (non-life) policies are
/// subject to the principle of indemnity at claim time.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InsurancePolicy {
    /// Name of the policyholder / insured (ຊື່ຜູ້ເອົາປະກັນໄພ)
    pub policyholder: String,
    /// Class of insurance (ປະເພດການປະກັນໄພ)
    pub insurance_class: InsuranceClass,
    /// Whether the policyholder has an insurable interest (ມີຜົນປະໂຫຍດທີ່ສາມາດເອົາປະກັນໄພໄດ້)
    pub insurable_interest: bool,
    /// Sum insured in LAK (ຈຳນວນເງິນເອົາປະກັນໄພເປັນກີບ)
    pub sum_insured_lak: u64,
    /// Premium in LAK (ເບ້ຍປະກັນໄພເປັນກີບ)
    pub premium_lak: u64,
    /// Whether this is an indemnity (non-life) policy (ເປັນສັນຍາຊົດໃຊ້ຄ່າເສຍຫາຍ)
    pub is_indemnity: bool,
    /// Start date in YYYY-MM-DD form (ວັນທີເລີ່ມຕົ້ນ)
    pub start_date: String,
    /// End date in YYYY-MM-DD form (ວັນທີສິ້ນສຸດ)
    pub end_date: String,
    /// Current status of the policy (ສະຖານະຂອງສັນຍາ)
    pub status: PolicyStatus,
}

// ============================================================================
// Insurance Claims - ການຮຽກຮ້ອງຄ່າສິນໄໝ
// ============================================================================

/// Insurance claim - ການຮຽກຮ້ອງຄ່າສິນໄໝ
///
/// Models a claim under an insurance policy. For indemnity insurance the payout
/// is capped by both the actual loss and the sum insured (principle of indemnity).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InsuranceClaim {
    /// Class of insurance the claim arises under (ປະເພດການປະກັນໄພ)
    pub insurance_class: InsuranceClass,
    /// Sum insured under the policy in LAK (ຈຳນວນເງິນເອົາປະກັນໄພເປັນກີບ)
    pub sum_insured_lak: u64,
    /// Actual loss sustained in LAK (ຄວາມເສຍຫາຍຕົວຈິງເປັນກີບ)
    pub actual_loss_lak: u64,
    /// Amount claimed / to be paid in LAK (ຈຳນວນເງິນທີ່ຮຽກຮ້ອງເປັນກີບ)
    pub claim_amount_lak: u64,
    /// Whether the underlying policy is an indemnity policy (ເປັນສັນຍາຊົດໃຊ້ຄ່າເສຍຫາຍ)
    pub is_indemnity: bool,
    /// Whether the claim has been notified to the insurer (ໄດ້ແຈ້ງຕໍ່ບໍລິສັດປະກັນໄພ)
    pub notified: bool,
    /// Whether the claim has been found to be fraudulent (ເປັນການຮຽກຮ້ອງສໍ້ໂກງ)
    pub fraudulent: bool,
    /// Current status of the claim (ສະຖານະຂອງການຮຽກຮ້ອງ)
    pub status: ClaimStatus,
}

// ============================================================================
// Intermediaries - ຕົວກາງປະກັນໄພ
// ============================================================================

/// Insurance intermediary (agent or broker) - ຕົວກາງປະກັນໄພ
///
/// Insurance agents and brokers must be licensed to carry on intermediation.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Intermediary {
    /// Name of the intermediary (ຊື່ຕົວກາງ)
    pub name: String,
    /// Type of intermediary (ປະເພດຕົວກາງ)
    pub intermediary_type: IntermediaryType,
    /// Whether the intermediary is licensed (ໄດ້ຮັບໃບອະນຸຍາດ)
    pub licensed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motor_is_compulsory() {
        assert_eq!(
            InsuranceClass::Motor.is_compulsory(),
            MOTOR_THIRD_PARTY_COMPULSORY
        );
        assert!(InsuranceClass::Motor.is_compulsory());
        assert!(!InsuranceClass::Life.is_compulsory());
        assert!(!InsuranceClass::Property.is_compulsory());
    }

    #[test]
    fn test_all_classes_count() {
        assert_eq!(InsuranceClass::all().len(), INSURANCE_CLASS_COUNT);
        assert_eq!(INSURANCE_CLASS_COUNT, 10);
    }

    #[test]
    fn test_bilingual_names_present() {
        for class in InsuranceClass::all() {
            assert!(!class.lao_name().is_empty());
            assert!(!class.english_name().is_empty());
        }
        for insurer_type in [
            InsurerType::LifeInsurer,
            InsurerType::NonLifeInsurer,
            InsurerType::CompositeInsurer,
            InsurerType::Reinsurer,
            InsurerType::Microinsurer,
        ] {
            assert!(!insurer_type.lao_name().is_empty());
            assert!(!insurer_type.english_name().is_empty());
        }
        for status in [
            PolicyStatus::Proposed,
            PolicyStatus::Active,
            PolicyStatus::Lapsed,
            PolicyStatus::Expired,
            PolicyStatus::Cancelled,
        ] {
            assert!(!status.lao_name().is_empty());
            assert!(!status.english_name().is_empty());
        }
        for status in [
            ClaimStatus::Notified,
            ClaimStatus::UnderAssessment,
            ClaimStatus::Approved,
            ClaimStatus::Paid,
            ClaimStatus::Rejected,
        ] {
            assert!(!status.lao_name().is_empty());
            assert!(!status.english_name().is_empty());
        }
        for kind in [IntermediaryType::Agent, IntermediaryType::Broker] {
            assert!(!kind.lao_name().is_empty());
            assert!(!kind.english_name().is_empty());
        }
    }

    #[test]
    fn test_indemnity_class_classification() {
        assert!(InsuranceClass::Motor.is_indemnity_class());
        assert!(InsuranceClass::Property.is_indemnity_class());
        assert!(!InsuranceClass::Life.is_indemnity_class());
    }

    #[test]
    fn test_insurer_solvency_helper() {
        let solvent = Insurer {
            name: "Lao Insurance Co".to_string(),
            insurer_type: InsurerType::NonLifeInsurer,
            registered_capital_lak: 50_000_000_000,
            licensed: true,
            admitted_assets_lak: 100_000_000_000,
            liabilities_lak: 60_000_000_000,
        };
        assert!(solvent.is_solvent());

        let insolvent = Insurer {
            liabilities_lak: 120_000_000_000,
            ..solvent.clone()
        };
        assert!(!insolvent.is_solvent());

        let exactly_solvent = Insurer {
            admitted_assets_lak: 60_000_000_000,
            liabilities_lak: 60_000_000_000,
            ..solvent.clone()
        };
        assert!(exactly_solvent.is_solvent());
    }

    #[test]
    fn test_min_solvency_ratio_is_100() {
        assert_eq!(MIN_SOLVENCY_RATIO_PERCENT, 100);
    }
}
