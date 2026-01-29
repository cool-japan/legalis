//! Data subject rights under PDPL

use serde::{Deserialize, Serialize};

/// Data subject rights - Article 11
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSubjectRight {
    Access,
    Rectification,
    Erasure,
    Restriction,
    DataPortability,
    ObjectToProcessing,
    AutomatedDecisionMaking,
    Complaint,
    Compensation,
}

impl DataSubjectRight {
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Access => "Right to Access",
            Self::Rectification => "Right to Rectification",
            Self::Erasure => "Right to Erasure",
            Self::Restriction => "Right to Restriction",
            Self::DataPortability => "Right to Data Portability",
            Self::ObjectToProcessing => "Right to Object",
            Self::AutomatedDecisionMaking => "Automated Decision-Making Rights",
            Self::Complaint => "Right to Complaint",
            Self::Compensation => "Right to Compensation",
        }
    }
}
