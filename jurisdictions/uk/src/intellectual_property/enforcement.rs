//! IP Enforcement and Remedies
//!
//! UK IP enforcement mechanisms:
//! - UK IPO proceedings (oppositions, revocations)
//! - High Court IP litigation (Chancery Division)
//! - IP Enterprise Court (IPEC) for smaller claims
//! - Remedies: Injunctions, damages, account of profits, delivery up

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// UK IPO proceeding type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UkIpoProceeding {
    /// Opposition to patent/trade mark application
    Opposition {
        /// Application number being opposed
        application_number: String,
        /// Name of opponent
        opponent: String,
        /// Grounds for opposition
        grounds: Vec<String>,
    },
    /// Revocation of patent (s.72)
    PatentRevocation {
        /// Patent number to revoke
        patent_number: String,
        /// Grounds for revocation
        grounds: Vec<String>,
    },
    /// Revocation of trade mark for non-use (s.46)
    TradeMarkRevocation {
        /// Registration number to revoke
        registration_number: String,
        /// Number of years of non-use
        years_non_use: u32,
    },
    /// Invalidation for bad faith (s.47)
    Invalidation {
        /// Registration number to invalidate
        registration_number: String,
        /// Grounds for invalidation
        grounds: String,
    },
}

/// IP tribunal (specialized courts)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpTribunal {
    /// UK Intellectual Property Office
    UkIpo,
    /// High Court (Chancery Division, Patents Court)
    HighCourtChancery,
    /// IP Enterprise Court (IPEC) - for claims <£500k
    Ipec,
    /// Court of Appeal
    CourtOfAppeal,
    /// Supreme Court
    SupremeCourt,
}

/// IP enforcement action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpEnforcementAction {
    /// Type of IP right being enforced
    pub right_type: super::types::IpRightType,
    /// Right identifier (patent number, registration number, etc.)
    pub right_id: String,
    /// Alleged infringer
    pub alleged_infringer: String,
    /// Tribunal/court
    pub tribunal: IpTribunal,
    /// Date action commenced
    pub commencement_date: NaiveDate,
    /// Remedies sought
    pub remedies_sought: Vec<IpRemedyType>,
}

/// Type of IP remedy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpRemedyType {
    /// Injunction (prevent further infringement)
    Injunction,
    /// Damages (compensatory)
    Damages,
    /// Account of profits (defendant's gains)
    AccountOfProfits,
    /// Delivery up or destruction of infringing articles
    DeliveryUp,
    /// Declaration of invalidity
    DeclarationInvalidity,
}

/// IP remedy awarded
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpRemedy {
    /// Type of remedy
    pub remedy_type: IpRemedyType,
    /// Monetary value (if applicable)
    pub amount_gbp: Option<f64>,
    /// Description
    pub description: String,
    /// Date awarded
    pub award_date: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_enforcement_action() {
        let action = IpEnforcementAction {
            right_type: super::super::types::IpRightType::Patent,
            right_id: "GB2123456".to_string(),
            alleged_infringer: "Infringer Co Ltd".to_string(),
            tribunal: IpTribunal::HighCourtChancery,
            commencement_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            remedies_sought: vec![
                IpRemedyType::Injunction,
                IpRemedyType::Damages,
                IpRemedyType::DeliveryUp,
            ],
        };

        assert_eq!(action.remedies_sought.len(), 3);
    }

    #[test]
    fn test_ip_remedy_damages() {
        let remedy = IpRemedy {
            remedy_type: IpRemedyType::Damages,
            amount_gbp: Some(100_000.0),
            description: "Compensatory damages for patent infringement".to_string(),
            award_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
        };

        assert_eq!(remedy.amount_gbp, Some(100_000.0));
    }
}
