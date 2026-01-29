//! Book VI: Succession (继承编)
//!
//! Articles 1119-1163 of the Civil Code
//!
//! Covers:
//! - Testamentary succession
//! - Intestate succession
//! - Bequests and agreements
//! - Partition of estate

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Succession type (继承方式)
///
/// Articles 1123-1163
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuccessionType {
    /// Testamentary succession (遗嘱继承) - Article 1133
    Testamentary,
    /// Intestate succession (法定继承) - Article 1127
    Intestate,
    /// Bequest (遗赠) - Article 1133
    Bequest,
}

impl SuccessionType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Testamentary => BilingualText::new("遗嘱继承", "Testamentary succession"),
            Self::Intestate => BilingualText::new("法定继承", "Intestate succession"),
            Self::Bequest => BilingualText::new("遗赠", "Bequest"),
        }
    }
}

/// Order of intestate succession (法定继承顺序)
///
/// Article 1127
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntestateSuccessionOrder {
    /// First order (第一顺序): Spouse, children, parents
    FirstOrder,
    /// Second order (第二顺序): Siblings, paternal grandparents, maternal grandparents
    SecondOrder,
}

impl IntestateSuccessionOrder {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::FirstOrder => BilingualText::new(
                "第一顺序继承人（配偶、子女、父母）",
                "First order (spouse, children, parents)",
            ),
            Self::SecondOrder => BilingualText::new(
                "第二顺序继承人（兄弟姐妹、祖父母、外祖父母）",
                "Second order (siblings, grandparents)",
            ),
        }
    }
}

/// Heir type (继承人类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeirType {
    /// Spouse (配偶)
    Spouse,
    /// Child (子女)
    Child,
    /// Parent (父母)
    Parent,
    /// Sibling (兄弟姐妹)
    Sibling,
    /// Grandparent (祖父母、外祖父母)
    Grandparent,
}

impl HeirType {
    /// Get succession order
    pub fn succession_order(&self) -> IntestateSuccessionOrder {
        match self {
            Self::Spouse | Self::Child | Self::Parent => IntestateSuccessionOrder::FirstOrder,
            Self::Sibling | Self::Grandparent => IntestateSuccessionOrder::SecondOrder,
        }
    }

    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Spouse => BilingualText::new("配偶", "Spouse"),
            Self::Child => BilingualText::new("子女", "Child"),
            Self::Parent => BilingualText::new("父母", "Parent"),
            Self::Sibling => BilingualText::new("兄弟姐妹", "Sibling"),
            Self::Grandparent => BilingualText::new("祖父母/外祖父母", "Grandparent"),
        }
    }
}

/// Heir (继承人)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heir {
    /// Name
    pub name: String,
    /// Heir type
    pub heir_type: HeirType,
    /// Share of estate (if specified)
    pub share: Option<f64>,
}

impl Heir {
    /// Get succession order
    pub fn succession_order(&self) -> IntestateSuccessionOrder {
        self.heir_type.succession_order()
    }
}

/// Will/Testament (遗嘱)
///
/// Articles 1133-1144
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Will {
    /// Testator (遗嘱人)
    pub testator: String,
    /// Date of execution
    pub execution_date: DateTime<Utc>,
    /// Will type
    pub will_type: WillType,
    /// Beneficiaries
    pub beneficiaries: Vec<String>,
    /// Is valid
    pub is_valid: bool,
    /// Revoked
    pub is_revoked: bool,
}

/// Type of will (遗嘱类型)
///
/// Articles 1134-1139
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WillType {
    /// Notarized will (公证遗嘱) - Article 1139
    Notarized,
    /// Holographic will (自书遗嘱) - Article 1134
    Holographic,
    /// Witnessed will (代书遗嘱) - Article 1135
    Witnessed,
    /// Recorded will (录音录像遗嘱) - Article 1136
    Recorded,
    /// Oral will (口头遗嘱) - Article 1138
    Oral,
}

impl WillType {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Notarized => BilingualText::new("公证遗嘱", "Notarized will"),
            Self::Holographic => BilingualText::new("自书遗嘱", "Holographic will"),
            Self::Witnessed => BilingualText::new("代书遗嘱", "Witnessed will"),
            Self::Recorded => BilingualText::new("录音录像遗嘱", "Recorded will"),
            Self::Oral => BilingualText::new("口头遗嘱", "Oral will"),
        }
    }

    /// Check if will type is valid only in emergencies
    ///
    /// Article 1138: Oral wills only valid in emergency situations
    pub fn requires_emergency(&self) -> bool {
        matches!(self, Self::Oral)
    }
}

/// Estate (遗产)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Estate {
    /// Deceased (被继承人)
    pub deceased: String,
    /// Date of death
    pub death_date: DateTime<Utc>,
    /// Total estate value
    pub total_value: f64,
    /// Currency
    pub currency: String,
    /// Debts to be paid from estate
    pub debts: f64,
    /// Has will
    pub has_will: bool,
}

impl Estate {
    /// Get net estate value (after debts)
    ///
    /// Article 1159: Debts paid from estate
    pub fn net_value(&self) -> f64 {
        (self.total_value - self.debts).max(0.0)
    }

    /// Check if estate is sufficient to cover debts
    pub fn is_solvent(&self) -> bool {
        self.total_value >= self.debts
    }
}

/// Disinheritance reason (取消继承权事由)
///
/// Article 1125
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisinheritanceReason {
    /// Intentionally killed the deceased (故意杀害被继承人)
    IntentionalKilling,
    /// Killed other heirs for inheritance (为争夺遗产杀害其他继承人)
    KillingOtherHeirs,
    /// Abandoned or abused deceased (遗弃、虐待被继承人)
    AbandonmentOrAbuse,
    /// Forged, tampered, concealed, or destroyed will (伪造、篡改、隐匿或销毁遗嘱)
    WillFraud,
    /// Seriously infringed lawful rights (以欺诈、胁迫手段迫使或妨碍被继承人设立、变更或撤回遗嘱)
    FraudOrCoercionRegardingWill,
}

impl DisinheritanceReason {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::IntentionalKilling => {
                BilingualText::new("故意杀害被继承人", "Intentionally killed deceased")
            }
            Self::KillingOtherHeirs => BilingualText::new(
                "为争夺遗产杀害其他继承人",
                "Killed other heirs for inheritance",
            ),
            Self::AbandonmentOrAbuse => {
                BilingualText::new("遗弃、虐待被继承人", "Abandoned or abused deceased")
            }
            Self::WillFraud => {
                BilingualText::new("伪造、篡改、隐匿或销毁遗嘱", "Forged, tampered with will")
            }
            Self::FraudOrCoercionRegardingWill => BilingualText::new(
                "以欺诈、胁迫手段迫使或妨碍遗嘱",
                "Fraud or coercion regarding will",
            ),
        }
    }
}

// ============================================================================
// Validators
// ============================================================================

/// Validate will
///
/// Articles 1133-1144
pub fn validate_will(will: &Will, is_emergency: bool) -> Result<(), SuccessionError> {
    // Check if will is revoked
    if will.is_revoked {
        return Err(SuccessionError::WillRevoked {
            testator: will.testator.clone(),
        });
    }

    // Article 1138: Oral wills only valid in emergencies
    if will.will_type.requires_emergency() && !is_emergency {
        return Err(SuccessionError::OralWillNotInEmergency {
            testator: will.testator.clone(),
        });
    }

    // Check validity
    if !will.is_valid {
        return Err(SuccessionError::InvalidWill {
            testator: will.testator.clone(),
            will_type: will.will_type.description(),
        });
    }

    Ok(())
}

/// Calculate intestate succession shares
///
/// Article 1130: First order heirs inherit in equal shares (generally)
pub fn calculate_intestate_shares(heirs: &[Heir], net_estate_value: f64) -> Vec<(String, f64)> {
    // Article 1127: First order heirs inherit before second order
    let first_order_heirs: Vec<&Heir> = heirs
        .iter()
        .filter(|h| h.succession_order() == IntestateSuccessionOrder::FirstOrder)
        .collect();

    let heirs_to_use = if first_order_heirs.is_empty() {
        // No first order heirs, use second order
        heirs
            .iter()
            .filter(|h| h.succession_order() == IntestateSuccessionOrder::SecondOrder)
            .collect()
    } else {
        first_order_heirs
    };

    if heirs_to_use.is_empty() {
        return Vec::new();
    }

    // Article 1130: Equal shares (generally)
    let share_per_heir = net_estate_value / heirs_to_use.len() as f64;

    heirs_to_use
        .iter()
        .map(|h| (h.name.clone(), share_per_heir))
        .collect()
}

/// Check if heir has lost succession rights
///
/// Article 1125
pub fn check_disinheritance(reason: Option<DisinheritanceReason>) -> Result<(), SuccessionError> {
    if let Some(r) = reason {
        Err(SuccessionError::Disinherited {
            reason: r.description(),
        })
    } else {
        Ok(())
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Succession
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum SuccessionError {
    /// Will revoked
    #[error("Will revoked by testator: {testator}")]
    WillRevoked {
        /// Testator
        testator: String,
    },

    /// Oral will not in emergency
    #[error("Oral will by {testator} is only valid in emergency situations")]
    OralWillNotInEmergency {
        /// Testator
        testator: String,
    },

    /// Invalid will
    #[error("Invalid will by {testator}: {will_type}")]
    InvalidWill {
        /// Testator
        testator: String,
        /// Will type
        will_type: BilingualText,
    },

    /// Heir disinherited
    #[error("Heir disinherited: {reason}")]
    Disinherited {
        /// Reason
        reason: BilingualText,
    },

    /// Estate insolvent
    #[error("Estate is insolvent: debts exceed assets")]
    EstateInsolvent,
}

/// Result type for Succession operations
pub type SuccessionResult<T> = Result<T, SuccessionError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_succession_order() {
        let spouse = Heir {
            name: "配偶".to_string(),
            heir_type: HeirType::Spouse,
            share: None,
        };

        let child = Heir {
            name: "子女".to_string(),
            heir_type: HeirType::Child,
            share: None,
        };

        let sibling = Heir {
            name: "兄弟".to_string(),
            heir_type: HeirType::Sibling,
            share: None,
        };

        assert_eq!(
            spouse.succession_order(),
            IntestateSuccessionOrder::FirstOrder
        );
        assert_eq!(
            child.succession_order(),
            IntestateSuccessionOrder::FirstOrder
        );
        assert_eq!(
            sibling.succession_order(),
            IntestateSuccessionOrder::SecondOrder
        );
    }

    #[test]
    fn test_calculate_intestate_shares() {
        let heirs = vec![
            Heir {
                name: "配偶".to_string(),
                heir_type: HeirType::Spouse,
                share: None,
            },
            Heir {
                name: "子女1".to_string(),
                heir_type: HeirType::Child,
                share: None,
            },
            Heir {
                name: "子女2".to_string(),
                heir_type: HeirType::Child,
                share: None,
            },
        ];

        let shares = calculate_intestate_shares(&heirs, 900000.0);
        assert_eq!(shares.len(), 3);
        assert_eq!(shares[0].1, 300000.0); // Each gets 1/3
    }

    #[test]
    fn test_estate_net_value() {
        let estate = Estate {
            deceased: "被继承人".to_string(),
            death_date: Utc::now(),
            total_value: 1_000_000.0,
            currency: "CNY".to_string(),
            debts: 100_000.0,
            has_will: false,
        };

        assert_eq!(estate.net_value(), 900_000.0);
        assert!(estate.is_solvent());
    }

    #[test]
    fn test_estate_insolvent() {
        let estate = Estate {
            deceased: "被继承人".to_string(),
            death_date: Utc::now(),
            total_value: 100_000.0,
            currency: "CNY".to_string(),
            debts: 200_000.0,
            has_will: false,
        };

        assert_eq!(estate.net_value(), 0.0);
        assert!(!estate.is_solvent());
    }

    #[test]
    fn test_will_validation() {
        let will = Will {
            testator: "遗嘱人".to_string(),
            execution_date: Utc::now(),
            will_type: WillType::Notarized,
            beneficiaries: vec!["受益人".to_string()],
            is_valid: true,
            is_revoked: false,
        };

        assert!(validate_will(&will, false).is_ok());
    }

    #[test]
    fn test_oral_will_requires_emergency() {
        let oral_will = Will {
            testator: "遗嘱人".to_string(),
            execution_date: Utc::now(),
            will_type: WillType::Oral,
            beneficiaries: vec!["受益人".to_string()],
            is_valid: true,
            is_revoked: false,
        };

        // Should fail without emergency
        assert!(validate_will(&oral_will, false).is_err());
        // Should succeed in emergency
        assert!(validate_will(&oral_will, true).is_ok());
    }

    #[test]
    fn test_disinheritance() {
        let result = check_disinheritance(Some(DisinheritanceReason::IntentionalKilling));
        assert!(result.is_err());

        let result_ok = check_disinheritance(None);
        assert!(result_ok.is_ok());
    }
}
