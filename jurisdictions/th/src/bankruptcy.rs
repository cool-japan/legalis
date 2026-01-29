//! Bankruptcy Act - พ.ร.บ. ล้มละลาย พ.ศ. 2483

use serde::{Deserialize, Serialize};

/// Bankruptcy types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankruptcyType {
    /// Voluntary bankruptcy (ยื่นล้มละลายโดยสมัครใจ)
    Voluntary,
    /// Involuntary bankruptcy (ถูกบังคับล้มละลาย)
    Involuntary,
    /// Business reorganization (ฟื้นฟูกิจการ)
    Reorganization,
}

impl BankruptcyType {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Voluntary => "ยื่นล้มละลายโดยสมัครใจ",
            Self::Involuntary => "ถูกบังคับล้มละลาย",
            Self::Reorganization => "ฟื้นฟูกิจการ",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Voluntary => "Voluntary Bankruptcy",
            Self::Involuntary => "Involuntary Bankruptcy",
            Self::Reorganization => "Business Reorganization",
        }
    }
}

/// Creditor priority classes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CreditorPriority {
    /// Secured creditors (เจ้าหนี้มีประกัน)
    Secured = 1,
    /// Preferential creditors (เจ้าหนี้พิเศษ)
    Preferential = 2,
    /// Unsecured creditors (เจ้าหนี้สามัญ)
    Unsecured = 3,
    /// Subordinated creditors (เจ้าหนี้ด้อยสิทธิ)
    Subordinated = 4,
}

impl CreditorPriority {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Secured => "เจ้าหนี้มีประกัน",
            Self::Preferential => "เจ้าหนี้พิเศษ",
            Self::Unsecured => "เจ้าหนี้สามัญ",
            Self::Subordinated => "เจ้าหนี้ด้อยสิทธิ",
        }
    }
}

/// Acts of bankruptcy (เหตุล้มละลาย)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActOfBankruptcy {
    /// Unable to pay debts
    UnableToPayDebts,
    /// Fraudulent transfer
    FraudulentTransfer,
    /// Preference to creditor
    UnfairPreference,
    /// Absconding
    Absconding,
}

impl ActOfBankruptcy {
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::UnableToPayDebts => "ไม่สามารถชำระหนี้",
            Self::FraudulentTransfer => "โอนทรัพย์สินเพื่อหลีกเลี่ยง",
            Self::UnfairPreference => "ให้สิทธิพิเศษแก่เจ้าหนี้บางราย",
            Self::Absconding => "หลบหนี",
        }
    }
}

/// Minimum debt threshold for bankruptcy filing
pub const MIN_DEBT_THRESHOLD: u64 = 1_000_000; // 1M THB

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bankruptcy_types() {
        assert_eq!(BankruptcyType::Voluntary.name_en(), "Voluntary Bankruptcy");
    }

    #[test]
    fn test_creditor_priority() {
        assert!(CreditorPriority::Secured < CreditorPriority::Unsecured);
    }
}
