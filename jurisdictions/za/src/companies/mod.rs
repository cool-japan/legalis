//! South African Companies Act 71 of 2008
//!
//! Governs company formation, registration, and corporate governance.
//!
//! ## Company Types
//!
//! - Profit companies: (Pty) Ltd, Ltd, Inc
//! - Non-profit companies: NPC
//!
//! ## Key Features
//!
//! - King IV corporate governance principles
//! - Business rescue proceedings (Chapter 6)
//! - Social and ethics committee requirements (s72)
//! - B-BBEE compliance integration

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for company operations
pub type CompanyResult<T> = Result<T, CompanyError>;

/// Company types under Companies Act 2008
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    /// Private company - (Pty) Ltd - s8(2)(b)
    PrivateCompany,
    /// Public company - Ltd - s8(2)(c)
    PublicCompany,
    /// Personal liability company - Inc - s8(2)(d)
    PersonalLiabilityCompany,
    /// State-owned company - SOC Ltd - s8(2)(a)
    StateOwnedCompany,
    /// Non-profit company - NPC - s8(1)(c)
    NonProfitCompany,
    /// External company (foreign registered)
    ExternalCompany,
}

impl CompanyType {
    /// Get the suffix for company name
    pub fn suffix(&self) -> &'static str {
        match self {
            Self::PrivateCompany => "(Pty) Ltd",
            Self::PublicCompany => "Ltd",
            Self::PersonalLiabilityCompany => "Inc",
            Self::StateOwnedCompany => "SOC Ltd",
            Self::NonProfitCompany => "NPC",
            Self::ExternalCompany => "",
        }
    }

    /// Check if company has limited liability
    pub fn has_limited_liability(&self) -> bool {
        !matches!(self, Self::PersonalLiabilityCompany)
    }

    /// Minimum number of directors
    pub fn minimum_directors(&self) -> u32 {
        match self {
            Self::PrivateCompany => 1,
            Self::PublicCompany | Self::StateOwnedCompany => 3,
            Self::PersonalLiabilityCompany => 1,
            Self::NonProfitCompany => 3,
            Self::ExternalCompany => 0, // Governed by home jurisdiction
        }
    }

    /// Can issue shares to public
    pub fn can_offer_shares_to_public(&self) -> bool {
        matches!(self, Self::PublicCompany | Self::StateOwnedCompany)
    }

    /// Maximum shareholders (private company restriction)
    pub fn maximum_shareholders(&self) -> Option<u32> {
        match self {
            Self::PrivateCompany => Some(50), // Restriction removed in 2011 amendment
            _ => None,
        }
    }
}

/// Company registration with CIPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyRegistration {
    /// Company name (without suffix)
    pub name: String,
    /// Company type
    pub company_type: CompanyType,
    /// Registered address
    pub registered_address: String,
    /// Number of directors
    pub director_count: u32,
    /// Number of shareholders
    pub shareholder_count: u32,
    /// Financial year end month (1-12)
    pub financial_year_end: u8,
    /// Has MOI (Memorandum of Incorporation)
    pub has_moi: bool,
}

impl CompanyRegistration {
    /// Validate registration requirements
    pub fn is_valid(&self) -> bool {
        // Check director count
        let min_directors = self.company_type.minimum_directors();
        if self.director_count < min_directors {
            return false;
        }

        // Check shareholder restrictions for private companies
        if let Some(max) = self.company_type.maximum_shareholders()
            && self.shareholder_count > max
        {
            return false;
        }

        // Check name not empty
        if self.name.is_empty() {
            return false;
        }

        // MOI required
        if !self.has_moi {
            return false;
        }

        true
    }
}

/// B-BBEE (Broad-Based Black Economic Empowerment) scorecard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbbeeScorecard {
    /// Ownership points (25 max)
    pub ownership_points: f64,
    /// Management control points (19 max)
    pub management_control_points: f64,
    /// Skills development points (20 max)
    pub skills_development_points: f64,
    /// Enterprise and supplier development (40 max)
    pub enterprise_supplier_development_points: f64,
    /// Socio-economic development (5 max)
    pub socio_economic_development_points: f64,
    /// Priority elements achieved
    pub priority_elements_achieved: bool,
}

impl BbbeeScorecard {
    /// Calculate total score
    pub fn total_score(&self) -> f64 {
        self.ownership_points
            + self.management_control_points
            + self.skills_development_points
            + self.enterprise_supplier_development_points
            + self.socio_economic_development_points
    }

    /// Get BEE level (1-8, or Non-Compliant)
    pub fn level(&self) -> BbbeeLevel {
        let total = self.total_score();

        // Check discounting for priority elements
        if !self.priority_elements_achieved {
            return BbbeeLevel::Discounted;
        }

        match total as u32 {
            100.. => BbbeeLevel::Level1,
            95..=99 => BbbeeLevel::Level2,
            90..=94 => BbbeeLevel::Level3,
            80..=89 => BbbeeLevel::Level4,
            75..=79 => BbbeeLevel::Level5,
            70..=74 => BbbeeLevel::Level6,
            55..=69 => BbbeeLevel::Level7,
            40..=54 => BbbeeLevel::Level8,
            _ => BbbeeLevel::NonCompliant,
        }
    }

    /// Get procurement recognition level
    pub fn procurement_recognition_percent(&self) -> u32 {
        match self.level() {
            BbbeeLevel::Level1 => 135,
            BbbeeLevel::Level2 => 125,
            BbbeeLevel::Level3 => 110,
            BbbeeLevel::Level4 => 100,
            BbbeeLevel::Level5 => 80,
            BbbeeLevel::Level6 => 60,
            BbbeeLevel::Level7 => 50,
            BbbeeLevel::Level8 => 10,
            BbbeeLevel::NonCompliant | BbbeeLevel::Discounted => 0,
        }
    }
}

/// B-BBEE Level
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BbbeeLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    NonCompliant,
    /// Discounted due to priority element non-compliance
    Discounted,
}

impl BbbeeLevel {
    /// Get level number (0 for non-compliant)
    pub fn level_number(&self) -> u8 {
        match self {
            Self::Level1 => 1,
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Level4 => 4,
            Self::Level5 => 5,
            Self::Level6 => 6,
            Self::Level7 => 7,
            Self::Level8 => 8,
            Self::NonCompliant | Self::Discounted => 0,
        }
    }
}

/// King IV governance principles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KingIvPrinciple {
    /// Ethical culture and responsible leadership
    EthicalLeadership,
    /// Governing body composition and performance
    BoardComposition,
    /// Risk and opportunity governance
    RiskGovernance,
    /// Technology and information governance
    TechnologyGovernance,
    /// Compliance with laws
    LegalCompliance,
    /// Remuneration governance
    RemunerationGovernance,
    /// Assurance and audit
    AssuranceGovernance,
    /// Stakeholder relationships
    StakeholderRelationships,
}

/// Business rescue status (Chapter 6)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusinessRescueStatus {
    /// Not in business rescue
    NotInRescue,
    /// Business rescue commenced
    Commenced,
    /// Business rescue plan published
    PlanPublished,
    /// Business rescue plan adopted
    PlanAdopted,
    /// Business rescue terminated
    Terminated,
    /// Liquidation (if rescue fails)
    Liquidation,
}

/// Company errors
#[derive(Debug, Error)]
pub enum CompanyError {
    /// Insufficient directors
    #[error("Insufficient directors for {company_type}: {actual} (minimum {required})")]
    InsufficientDirectors {
        company_type: String,
        actual: u32,
        required: u32,
    },

    /// Invalid company name
    #[error("Invalid company name: {reason}")]
    InvalidName { reason: String },

    /// MOI required
    #[error("Memorandum of Incorporation (MOI) is required")]
    MoiRequired,

    /// B-BBEE non-compliance
    #[error("B-BBEE non-compliance: {description}")]
    BbbeeNonCompliance { description: String },

    /// Corporate governance violation
    #[error("Corporate governance violation (King IV): {principle}")]
    GovernanceViolation { principle: String },
}

/// Validate company registration
pub fn validate_registration(reg: &CompanyRegistration) -> CompanyResult<()> {
    let min_directors = reg.company_type.minimum_directors();
    if reg.director_count < min_directors {
        return Err(CompanyError::InsufficientDirectors {
            company_type: format!("{:?}", reg.company_type),
            actual: reg.director_count,
            required: min_directors,
        });
    }

    if reg.name.is_empty() || reg.name.len() < 3 {
        return Err(CompanyError::InvalidName {
            reason: "Name too short".to_string(),
        });
    }

    if !reg.has_moi {
        return Err(CompanyError::MoiRequired);
    }

    Ok(())
}

/// Get company law compliance checklist
pub fn get_company_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CIPC registration", "s14"),
        ("Memorandum of Incorporation (MOI)", "s15"),
        ("Minimum directors appointed", "s66"),
        ("Registered office address", "s23"),
        ("Share register maintained", "s50"),
        ("Annual return filed", "s33"),
        ("Financial statements prepared", "s30"),
        ("Audit (if required)", "s30(2)"),
        ("Social and ethics committee (if applicable)", "s72"),
        ("B-BBEE certificate (if applicable)", "B-BBEE Act"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_types() {
        let private = CompanyType::PrivateCompany;
        assert_eq!(private.suffix(), "(Pty) Ltd");
        assert!(private.has_limited_liability());
        assert_eq!(private.minimum_directors(), 1);
    }

    #[test]
    fn test_public_company() {
        let public = CompanyType::PublicCompany;
        assert!(public.can_offer_shares_to_public());
        assert_eq!(public.minimum_directors(), 3);
    }

    #[test]
    fn test_registration_valid() {
        let reg = CompanyRegistration {
            name: "Test Company".to_string(),
            company_type: CompanyType::PrivateCompany,
            registered_address: "123 Main St, Johannesburg".to_string(),
            director_count: 1,
            shareholder_count: 2,
            financial_year_end: 2,
            has_moi: true,
        };

        assert!(reg.is_valid());
        assert!(validate_registration(&reg).is_ok());
    }

    #[test]
    fn test_registration_invalid_directors() {
        let reg = CompanyRegistration {
            name: "Test Company".to_string(),
            company_type: CompanyType::PublicCompany,
            registered_address: "123 Main St".to_string(),
            director_count: 1, // Needs 3 for public
            shareholder_count: 100,
            financial_year_end: 2,
            has_moi: true,
        };

        assert!(!reg.is_valid());
        assert!(validate_registration(&reg).is_err());
    }

    #[test]
    fn test_bbbee_scorecard() {
        let scorecard = BbbeeScorecard {
            ownership_points: 20.0,
            management_control_points: 15.0,
            skills_development_points: 18.0,
            enterprise_supplier_development_points: 35.0,
            socio_economic_development_points: 4.0,
            priority_elements_achieved: true,
        };

        assert_eq!(scorecard.total_score(), 92.0);
        assert_eq!(scorecard.level(), BbbeeLevel::Level3);
        assert_eq!(scorecard.procurement_recognition_percent(), 110);
    }

    #[test]
    fn test_bbbee_discounted() {
        let scorecard = BbbeeScorecard {
            ownership_points: 25.0,
            management_control_points: 19.0,
            skills_development_points: 20.0,
            enterprise_supplier_development_points: 40.0,
            socio_economic_development_points: 5.0,
            priority_elements_achieved: false, // Missing priority
        };

        assert_eq!(scorecard.level(), BbbeeLevel::Discounted);
        assert_eq!(scorecard.procurement_recognition_percent(), 0);
    }

    #[test]
    fn test_bbbee_level_1() {
        let scorecard = BbbeeScorecard {
            ownership_points: 25.0,
            management_control_points: 19.0,
            skills_development_points: 20.0,
            enterprise_supplier_development_points: 40.0,
            socio_economic_development_points: 5.0,
            priority_elements_achieved: true,
        };

        assert_eq!(scorecard.total_score(), 109.0);
        assert_eq!(scorecard.level(), BbbeeLevel::Level1);
        assert_eq!(scorecard.procurement_recognition_percent(), 135);
    }

    #[test]
    fn test_company_checklist() {
        let checklist = get_company_checklist();
        assert!(!checklist.is_empty());
    }
}
