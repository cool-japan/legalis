//! Book VII: Tort Liability (侵权责任编)
//!
//! Articles 1164-1258 of the Civil Code
//!
//! Covers:
//! - General provisions on tort liability
//! - Specific tort types
//! - Products liability
//! - Medical malpractice
//! - Environmental pollution
//! - Highly dangerous activities

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Tort liability principle (归责原则)
///
/// Articles 1165-1167
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiabilityPrinciple {
    /// Fault liability (过错责任) - Article 1165
    Fault,
    /// Presumed fault (过错推定)
    PresumedFault,
    /// No-fault/Strict liability (无过错责任) - Article 1166
    StrictLiability,
}

impl LiabilityPrinciple {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Fault => BilingualText::new("过错责任原则", "Fault liability"),
            Self::PresumedFault => BilingualText::new("过错推定原则", "Presumed fault"),
            Self::StrictLiability => BilingualText::new("无过错责任原则", "Strict liability"),
        }
    }
}

/// Tort type (侵权类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortType {
    /// General tort (一般侵权)
    General,
    /// Products liability (产品责任) - Articles 1202-1207
    ProductsLiability,
    /// Medical malpractice (医疗损害) - Articles 1218-1228
    MedicalMalpractice,
    /// Traffic accident (机动车交通事故) - Articles 1208-1217
    TrafficAccident,
    /// Environmental pollution (环境污染) - Articles 1229-1235
    EnvironmentalPollution,
    /// Highly dangerous activities (高度危险活动) - Articles 1236-1244
    HighlyDangerousActivity,
    /// Animal-caused harm (饲养动物损害) - Articles 1245-1251
    AnimalCausedHarm,
    /// Building/Structure damage (建筑物、构筑物损害) - Articles 1252-1258
    BuildingDamage,
}

impl TortType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::General => BilingualText::new("一般侵权", "General tort"),
            Self::ProductsLiability => BilingualText::new("产品责任", "Products liability"),
            Self::MedicalMalpractice => BilingualText::new("医疗损害", "Medical malpractice"),
            Self::TrafficAccident => BilingualText::new("交通事故责任", "Traffic accident"),
            Self::EnvironmentalPollution => {
                BilingualText::new("环境污染责任", "Environmental pollution")
            }
            Self::HighlyDangerousActivity => {
                BilingualText::new("高度危险活动责任", "Highly dangerous activity")
            }
            Self::AnimalCausedHarm => BilingualText::new("饲养动物损害责任", "Animal-caused harm"),
            Self::BuildingDamage => BilingualText::new("建筑物损害责任", "Building damage"),
        }
    }

    /// Get applicable liability principle
    pub fn liability_principle(&self) -> LiabilityPrinciple {
        match self {
            Self::General => LiabilityPrinciple::Fault,
            Self::ProductsLiability
            | Self::EnvironmentalPollution
            | Self::HighlyDangerousActivity => LiabilityPrinciple::StrictLiability,
            Self::MedicalMalpractice | Self::BuildingDamage => LiabilityPrinciple::PresumedFault,
            Self::TrafficAccident | Self::AnimalCausedHarm => LiabilityPrinciple::StrictLiability,
        }
    }
}

/// Tort (侵权行为)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tort {
    /// Tortfeasor (侵权人)
    pub tortfeasor: String,
    /// Victim (受害人)
    pub victim: String,
    /// Tort type
    pub tort_type: TortType,
    /// Date of tort
    pub tort_date: DateTime<Utc>,
    /// Description
    pub description: BilingualText,
    /// Damage amount
    pub damage_amount: f64,
    /// Currency
    pub currency: String,
    /// Fault established (if applicable)
    pub fault_established: Option<bool>,
}

impl Tort {
    /// Get liability principle for this tort
    pub fn liability_principle(&self) -> LiabilityPrinciple {
        self.tort_type.liability_principle()
    }

    /// Check if liability can be established
    pub fn can_establish_liability(&self) -> bool {
        match self.liability_principle() {
            LiabilityPrinciple::Fault => {
                // Requires proof of fault
                self.fault_established.unwrap_or(false)
            }
            LiabilityPrinciple::PresumedFault => {
                // Fault presumed unless tortfeasor proves no fault
                !matches!(self.fault_established, Some(false))
            }
            LiabilityPrinciple::StrictLiability => {
                // No fault required
                true
            }
        }
    }
}

/// Products liability (产品责任)
///
/// Articles 1202-1207
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductsLiability {
    /// Product manufacturer (生产者)
    pub manufacturer: String,
    /// Product seller (销售者)
    pub seller: String,
    /// Injured person
    pub injured_person: String,
    /// Product description
    pub product: BilingualText,
    /// Defect type
    pub defect_type: ProductDefectType,
    /// Injury date
    pub injury_date: DateTime<Utc>,
    /// Damage amount
    pub damage_amount: f64,
    /// Currency
    pub currency: String,
}

/// Product defect type (产品缺陷类型)
///
/// Article 1202
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductDefectType {
    /// Manufacturing defect (生产缺陷)
    Manufacturing,
    /// Design defect (设计缺陷)
    Design,
    /// Warning defect (警示缺陷)
    Warning,
}

impl ProductDefectType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Manufacturing => BilingualText::new("生产缺陷", "Manufacturing defect"),
            Self::Design => BilingualText::new("设计缺陷", "Design defect"),
            Self::Warning => BilingualText::new("警示缺陷", "Warning defect"),
        }
    }
}

/// Medical malpractice (医疗损害)
///
/// Articles 1218-1228
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalMalpractice {
    /// Medical institution (医疗机构)
    pub medical_institution: String,
    /// Medical staff involved
    pub medical_staff: Vec<String>,
    /// Patient
    pub patient: String,
    /// Incident date
    pub incident_date: DateTime<Utc>,
    /// Description
    pub description: BilingualText,
    /// Damage amount
    pub damage_amount: f64,
    /// Currency
    pub currency: String,
    /// Medical records available
    pub medical_records_available: bool,
}

/// Environmental pollution liability (环境污染责任)
///
/// Articles 1229-1235
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalPollution {
    /// Polluter (污染者)
    pub polluter: String,
    /// Victims
    pub victims: Vec<String>,
    /// Pollution date
    pub pollution_date: DateTime<Utc>,
    /// Pollution type
    pub pollution_type: BilingualText,
    /// Environmental damage description
    pub description: BilingualText,
    /// Damage amount
    pub damage_amount: f64,
    /// Currency
    pub currency: String,
}

/// Highly dangerous activity liability (高度危险活动责任)
///
/// Articles 1236-1244
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlyDangerousActivity {
    /// Operator (经营者)
    pub operator: String,
    /// Activity type
    pub activity_type: BilingualText,
    /// Victim
    pub victim: String,
    /// Incident date
    pub incident_date: DateTime<Utc>,
    /// Description
    pub description: BilingualText,
    /// Damage amount
    pub damage_amount: f64,
    /// Currency
    pub currency: String,
    /// Victim's intentional or gross negligence
    pub victim_intentional_or_gross_negligence: bool,
}

impl HighlyDangerousActivity {
    /// Check if liability can be reduced
    ///
    /// Article 1236: Liability cannot be reduced unless victim intentional or gross negligence
    pub fn can_reduce_liability(&self) -> bool {
        self.victim_intentional_or_gross_negligence
    }
}

/// Defense to tort liability (抗辩事由)
///
/// Articles 1171-1177
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortDefense {
    /// Victim's fault (受害人过错) - Article 1173
    VictimFault,
    /// Third party fault (第三人过错) - Article 1175
    ThirdPartyFault,
    /// Force majeure (不可抗力) - Article 1177
    ForceMajeure,
    /// Self-defense (正当防卫) - Article 1177
    SelfDefense,
    /// Emergency (紧急避险) - Article 1182
    Emergency,
}

impl TortDefense {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::VictimFault => BilingualText::new("受害人过错", "Victim's fault"),
            Self::ThirdPartyFault => BilingualText::new("第三人过错", "Third party fault"),
            Self::ForceMajeure => BilingualText::new("不可抗力", "Force majeure"),
            Self::SelfDefense => BilingualText::new("正当防卫", "Self-defense"),
            Self::Emergency => BilingualText::new("紧急避险", "Emergency"),
        }
    }
}

// ============================================================================
// Validators
// ============================================================================

/// Validate tort liability can be established
///
/// Articles 1165-1167
pub fn validate_tort_liability(tort: &Tort) -> Result<(), TortLiabilityError> {
    if !tort.can_establish_liability() {
        return Err(TortLiabilityError::CannotEstablishLiability {
            tortfeasor: tort.tortfeasor.clone(),
            victim: tort.victim.clone(),
            principle: tort.liability_principle().description(),
        });
    }

    // Check damage amount is positive
    if tort.damage_amount <= 0.0 {
        return Err(TortLiabilityError::InvalidDamageAmount {
            amount: tort.damage_amount,
        });
    }

    Ok(())
}

/// Validate products liability claim
///
/// Articles 1202-1207
pub fn validate_products_liability(
    liability: &ProductsLiability,
) -> Result<(), TortLiabilityError> {
    // Article 1202: Strict liability - no fault required

    if liability.damage_amount <= 0.0 {
        return Err(TortLiabilityError::InvalidDamageAmount {
            amount: liability.damage_amount,
        });
    }

    Ok(())
}

/// Validate medical malpractice claim
///
/// Articles 1218-1228
pub fn validate_medical_malpractice(
    malpractice: &MedicalMalpractice,
) -> Result<(), TortLiabilityError> {
    // Article 1222: Medical institution must provide medical records
    if !malpractice.medical_records_available {
        return Err(TortLiabilityError::MedicalRecordsNotAvailable {
            institution: malpractice.medical_institution.clone(),
        });
    }

    Ok(())
}

/// Calculate damages with defense adjustment
///
/// Articles 1173-1177
pub fn calculate_damages_with_defense(
    base_damages: f64,
    defense: Option<TortDefense>,
    victim_fault_percentage: Option<f64>,
) -> f64 {
    match defense {
        Some(TortDefense::VictimFault) => {
            // Article 1173: Reduce or eliminate liability based on victim's fault
            let reduction = victim_fault_percentage.unwrap_or(0.5); // Default 50%
            base_damages * (1.0 - reduction)
        }
        Some(TortDefense::ThirdPartyFault) => {
            // Article 1175: Third party may bear liability
            base_damages // Tortfeasor still liable, can seek contribution from third party
        }
        Some(TortDefense::ForceMajeure | TortDefense::SelfDefense) => {
            // Article 1177: No liability for force majeure or self-defense
            0.0
        }
        Some(TortDefense::Emergency) => {
            // Article 1182: Emergency - person who benefits bears liability
            0.0
        }
        None => base_damages,
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Tort Liability
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum TortLiabilityError {
    /// Cannot establish liability
    #[error("Cannot establish liability for {tortfeasor} against {victim} under {principle}")]
    CannotEstablishLiability {
        /// Tortfeasor
        tortfeasor: String,
        /// Victim
        victim: String,
        /// Liability principle
        principle: BilingualText,
    },

    /// Invalid damage amount
    #[error("Invalid damage amount: {amount}")]
    InvalidDamageAmount {
        /// Amount
        amount: f64,
    },

    /// Medical records not available
    #[error("Medical records not available from institution: {institution}")]
    MedicalRecordsNotAvailable {
        /// Medical institution
        institution: String,
    },

    /// Defense not applicable
    #[error("Defense not applicable: {defense}")]
    DefenseNotApplicable {
        /// Defense
        defense: BilingualText,
    },
}

/// Result type for Tort Liability operations
pub type TortLiabilityResult<T> = Result<T, TortLiabilityError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tort_liability_principles() {
        assert_eq!(
            TortType::General.liability_principle(),
            LiabilityPrinciple::Fault
        );
        assert_eq!(
            TortType::ProductsLiability.liability_principle(),
            LiabilityPrinciple::StrictLiability
        );
        assert_eq!(
            TortType::MedicalMalpractice.liability_principle(),
            LiabilityPrinciple::PresumedFault
        );
    }

    #[test]
    fn test_fault_liability() {
        let tort = Tort {
            tortfeasor: "侵权人".to_string(),
            victim: "受害人".to_string(),
            tort_type: TortType::General,
            tort_date: Utc::now(),
            description: BilingualText::new("一般侵权", "General tort"),
            damage_amount: 10000.0,
            currency: "CNY".to_string(),
            fault_established: Some(true),
        };

        assert!(tort.can_establish_liability());
        assert!(validate_tort_liability(&tort).is_ok());
    }

    #[test]
    fn test_fault_not_established() {
        let tort = Tort {
            tortfeasor: "侵权人".to_string(),
            victim: "受害人".to_string(),
            tort_type: TortType::General,
            tort_date: Utc::now(),
            description: BilingualText::new("一般侵权", "General tort"),
            damage_amount: 10000.0,
            currency: "CNY".to_string(),
            fault_established: Some(false),
        };

        assert!(!tort.can_establish_liability());
        assert!(validate_tort_liability(&tort).is_err());
    }

    #[test]
    fn test_strict_liability() {
        let tort = Tort {
            tortfeasor: "污染企业".to_string(),
            victim: "受害人".to_string(),
            tort_type: TortType::EnvironmentalPollution,
            tort_date: Utc::now(),
            description: BilingualText::new("环境污染", "Environmental pollution"),
            damage_amount: 100000.0,
            currency: "CNY".to_string(),
            fault_established: None, // Fault not relevant for strict liability
        };

        assert!(tort.can_establish_liability());
        assert!(validate_tort_liability(&tort).is_ok());
    }

    #[test]
    fn test_products_liability() {
        let liability = ProductsLiability {
            manufacturer: "生产商".to_string(),
            seller: "销售商".to_string(),
            injured_person: "受害人".to_string(),
            product: BilingualText::new("缺陷产品", "Defective product"),
            defect_type: ProductDefectType::Manufacturing,
            injury_date: Utc::now(),
            damage_amount: 50000.0,
            currency: "CNY".to_string(),
        };

        assert!(validate_products_liability(&liability).is_ok());
    }

    #[test]
    fn test_calculate_damages_with_victim_fault() {
        let damages = calculate_damages_with_defense(
            100000.0,
            Some(TortDefense::VictimFault),
            Some(0.3), // Victim 30% at fault
        );
        assert_eq!(damages, 70000.0);
    }

    #[test]
    fn test_calculate_damages_force_majeure() {
        let damages =
            calculate_damages_with_defense(100000.0, Some(TortDefense::ForceMajeure), None);
        assert_eq!(damages, 0.0);
    }

    #[test]
    fn test_highly_dangerous_activity() {
        let activity = HighlyDangerousActivity {
            operator: "经营者".to_string(),
            activity_type: BilingualText::new("高压作业", "High-voltage work"),
            victim: "受害人".to_string(),
            incident_date: Utc::now(),
            description: BilingualText::new("触电事故", "Electrocution accident"),
            damage_amount: 200000.0,
            currency: "CNY".to_string(),
            victim_intentional_or_gross_negligence: false,
        };

        assert!(!activity.can_reduce_liability());
    }
}
