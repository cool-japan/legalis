//! Islamic Commercial Transactions (المعاملات التجارية)
//!
//! Commercial transactions under Saudi Islamic law must comply with Sharia principles,
//! particularly prohibiting:
//! - Riba (Interest/Usury - الربا)
//! - Gharar (Excessive uncertainty - الغرر)
//! - Maysir (Gambling - الميسر)

use crate::common::Sar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for commercial transaction operations
pub type CommercialTransactionResult<T> = Result<T, CommercialTransactionError>;

/// Commercial transaction errors
#[derive(Debug, Error)]
pub enum CommercialTransactionError {
    /// Contains Riba (interest)
    #[error("المعاملة تحتوي على الربا (Interest prohibited): {description}")]
    ContainsRiba { description: String },

    /// Contains Gharar (excessive uncertainty)
    #[error("المعاملة تحتوي على الغرر (Excessive uncertainty): {description}")]
    ContainsGharar { description: String },

    /// Contains Maysir (gambling)
    #[error("المعاملة تحتوي على الميسر (Gambling prohibited): {description}")]
    ContainsMaysir { description: String },

    /// Invalid contract structure
    #[error("عقد غير صالح: {reason}")]
    InvalidContract { reason: String },
}

/// Types of Islamic finance contracts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IslamicFinanceContract {
    /// Murabaha (مرابحة) - Cost-plus financing
    Murabaha,
    /// Ijara (إجارة) - Leasing
    Ijara,
    /// Mudaraba (مضاربة) - Profit-sharing partnership
    Mudaraba,
    /// Musharaka (مشاركة) - Joint venture partnership
    Musharaka,
    /// Istisna'a (استصناع) - Manufacturing contract
    Istisna,
    /// Salam (سلم) - Forward sale
    Salam,
    /// Takaful (تكافل) - Islamic insurance
    Takaful,
    /// Sukuk (صكوك) - Islamic bonds
    Sukuk,
    /// Qard Hasan (قرض حسن) - Benevolent loan
    QardHasan,
}

impl IslamicFinanceContract {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Murabaha => "مرابحة",
            Self::Ijara => "إجارة",
            Self::Mudaraba => "مضاربة",
            Self::Musharaka => "مشاركة",
            Self::Istisna => "استصناع",
            Self::Salam => "سلم",
            Self::Takaful => "تكافل",
            Self::Sukuk => "صكوك",
            Self::QardHasan => "قرض حسن",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Murabaha => "Murabaha (Cost-Plus Sale)",
            Self::Ijara => "Ijara (Leasing)",
            Self::Mudaraba => "Mudaraba (Profit-Sharing Partnership)",
            Self::Musharaka => "Musharaka (Joint Venture)",
            Self::Istisna => "Istisna'a (Manufacturing Contract)",
            Self::Salam => "Salam (Forward Sale)",
            Self::Takaful => "Takaful (Islamic Insurance)",
            Self::Sukuk => "Sukuk (Islamic Bonds)",
            Self::QardHasan => "Qard Hasan (Benevolent Loan)",
        }
    }

    /// Get contract description
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::Murabaha => {
                "Bank purchases asset and sells to customer at cost plus disclosed profit margin"
            }
            Self::Ijara => {
                "Islamic leasing where lessor retains ownership and receives rental payments"
            }
            Self::Mudaraba => {
                "Partnership where one provides capital (Rabb al-Mal) and other provides labor (Mudarib)"
            }
            Self::Musharaka => {
                "Joint venture where all partners contribute capital and share profits/losses"
            }
            Self::Istisna => "Contract for manufacturing goods with deferred delivery",
            Self::Salam => "Forward sale with full payment upfront and deferred delivery",
            Self::Takaful => {
                "Cooperative insurance based on mutual assistance and shared responsibility"
            }
            Self::Sukuk => "Asset-backed Islamic bonds representing ownership in tangible assets",
            Self::QardHasan => "Interest-free benevolent loan",
        }
    }

    /// Check if contract is Sharia-compliant by design
    pub fn is_sharia_compliant(&self) -> bool {
        true // All listed contracts are Sharia-compliant structures
    }
}

/// Commercial transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialTransaction {
    /// Transaction type
    pub contract_type: IslamicFinanceContract,
    /// Transaction amount
    pub amount: Sar,
    /// Profit margin (for Murabaha, etc.)
    pub profit_margin: Option<f64>,
    /// Duration in months
    pub duration_months: Option<u32>,
    /// Description
    pub description: String,
}

impl CommercialTransaction {
    /// Create a new commercial transaction
    pub fn new(
        contract_type: IslamicFinanceContract,
        amount: Sar,
        description: impl Into<String>,
    ) -> Self {
        Self {
            contract_type,
            amount,
            profit_margin: None,
            duration_months: None,
            description: description.into(),
        }
    }

    /// Set profit margin (for applicable contracts)
    pub fn with_profit_margin(mut self, margin: f64) -> Self {
        self.profit_margin = Some(margin);
        self
    }

    /// Set duration
    pub fn with_duration_months(mut self, months: u32) -> Self {
        self.duration_months = Some(months);
        self
    }

    /// Validate Sharia compliance
    pub fn validate_sharia_compliance(&self) -> CommercialTransactionResult<()> {
        // Check for Riba (interest/usury)
        if let Some(margin) = self.profit_margin {
            match self.contract_type {
                IslamicFinanceContract::QardHasan => {
                    if margin > 0.0 {
                        return Err(CommercialTransactionError::ContainsRiba {
                            description: "Qard Hasan cannot have profit margin".to_string(),
                        });
                    }
                }
                IslamicFinanceContract::Murabaha => {
                    // Murabaha profit must be disclosed and fixed at contract time
                    if margin <= 0.0 {
                        return Err(CommercialTransactionError::InvalidContract {
                            reason: "Murabaha must have positive disclosed profit".to_string(),
                        });
                    }
                }
                _ => {}
            }
        }

        // Check for excessive Gharar (uncertainty)
        if self.description.is_empty() {
            return Err(CommercialTransactionError::ContainsGharar {
                description: "Contract terms must be clearly specified".to_string(),
            });
        }

        Ok(())
    }
}

/// Check if a transaction is Sharia-compliant
pub fn check_sharia_compliance(
    has_interest: bool,
    has_excessive_uncertainty: bool,
    has_gambling_element: bool,
) -> CommercialTransactionResult<()> {
    if has_interest {
        return Err(CommercialTransactionError::ContainsRiba {
            description: "Interest-based transactions are prohibited".to_string(),
        });
    }

    if has_excessive_uncertainty {
        return Err(CommercialTransactionError::ContainsGharar {
            description: "Excessive uncertainty violates Sharia".to_string(),
        });
    }

    if has_gambling_element {
        return Err(CommercialTransactionError::ContainsMaysir {
            description: "Gambling elements are prohibited".to_string(),
        });
    }

    Ok(())
}

/// Get Sharia compliance checklist for commercial transactions
pub fn get_sharia_compliance_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("عدم وجود الربا", "No interest (Riba)"),
        ("عدم الغرر المفرط", "No excessive uncertainty (Gharar)"),
        ("عدم الميسر", "No gambling (Maysir)"),
        ("ملكية أصول حقيقية", "Real asset ownership"),
        ("وضوح الشروط", "Clear contract terms"),
        ("نشاط حلال", "Halal business activity"),
        (
            "مشاركة الأرباح والخسائر",
            "Profit and loss sharing (if applicable)",
        ),
        ("شفافية", "Transparency in all dealings"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_islamic_finance_contracts() {
        let murabaha = IslamicFinanceContract::Murabaha;
        assert_eq!(murabaha.name_ar(), "مرابحة");
        assert!(murabaha.name_en().contains("Murabaha"));
        assert!(murabaha.is_sharia_compliant());
    }

    #[test]
    fn test_murabaha_transaction() {
        let transaction = CommercialTransaction::new(
            IslamicFinanceContract::Murabaha,
            Sar::from_riyals(100_000),
            "Purchase of equipment",
        )
        .with_profit_margin(5.0)
        .with_duration_months(24);

        assert!(transaction.validate_sharia_compliance().is_ok());
    }

    #[test]
    fn test_qard_hasan_with_profit_fails() {
        let transaction = CommercialTransaction::new(
            IslamicFinanceContract::QardHasan,
            Sar::from_riyals(50_000),
            "Benevolent loan",
        )
        .with_profit_margin(2.0);

        assert!(transaction.validate_sharia_compliance().is_err());
    }

    #[test]
    fn test_check_riba() {
        let result = check_sharia_compliance(true, false, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CommercialTransactionError::ContainsRiba { .. }
        ));
    }

    #[test]
    fn test_check_gharar() {
        let result = check_sharia_compliance(false, true, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CommercialTransactionError::ContainsGharar { .. }
        ));
    }

    #[test]
    fn test_check_maysir() {
        let result = check_sharia_compliance(false, false, true);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CommercialTransactionError::ContainsMaysir { .. }
        ));
    }

    #[test]
    fn test_sharia_compliant_transaction() {
        let result = check_sharia_compliance(false, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_checklist() {
        let checklist = get_sharia_compliance_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 8);
    }
}
