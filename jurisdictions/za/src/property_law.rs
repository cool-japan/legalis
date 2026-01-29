//! South African Property Law
//!
//! Based on Roman-Dutch law principles and statutory modifications.
//!
//! ## Key Legislation
//!
//! - Deeds Registries Act 47 of 1937
//! - Sectional Titles Act 95 of 1986
//! - Restitution of Land Rights Act 22 of 1994
//! - Extension of Security of Tenure Act 62 of 1997 (ESTA)
//! - Land Reform (Labour Tenants) Act 3 of 1996
//!
//! ## Property Types
//!
//! - Ownership (dominium)
//! - Limited real rights (servitudes, mortgages, mineral rights)
//! - Personal rights (leases, licenses)
//!
//! ## Land Reform
//!
//! Constitutional imperative (s25) to redress colonial and apartheid dispossession

use crate::common::Zar;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for property operations
pub type PropertyResult<T> = Result<T, PropertyError>;

/// Property ownership types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OwnershipType {
    /// Full ownership (dominium)
    FullOwnership,
    /// Co-ownership (common ownership)
    CoOwnership {
        share_numerator: u32,
        share_denominator: u32,
    },
    /// Joint ownership
    JointOwnership,
}

impl OwnershipType {
    /// Calculate ownership percentage
    pub fn ownership_percentage(&self) -> f64 {
        match self {
            Self::FullOwnership => 100.0,
            Self::CoOwnership {
                share_numerator,
                share_denominator,
            } => (*share_numerator as f64 / *share_denominator as f64) * 100.0,
            Self::JointOwnership => 100.0, // Joint owners own whole, not shares
        }
    }
}

/// Real rights (rights in property)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RealRight {
    /// Ownership
    Ownership,
    /// Mortgage bond
    MortgageBond,
    /// Servitude (right of way, etc.)
    Servitude(ServitudeType),
    /// Mineral rights
    MineralRights,
    /// Usufruct
    Usufruct,
    /// Habitatio (right to occupy)
    Habitatio,
    /// Pledge
    Pledge,
}

/// Servitude types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServitudeType {
    /// Right of way (praedial)
    RightOfWay,
    /// Water servitude
    Water,
    /// Support servitude (buildings)
    Support,
    /// Personal servitude
    Personal,
}

/// Deeds registration requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeedsRegistration {
    /// Property description
    pub property_description: String,
    /// Title deed number
    pub title_deed_number: Option<String>,
    /// Registered owner
    pub registered_owner: String,
    /// Purchase price
    pub purchase_price: Zar,
    /// Transfer duty paid
    pub transfer_duty_paid: bool,
    /// Deeds Office location
    pub deeds_office: DeedsOffice,
}

impl DeedsRegistration {
    /// Validate registration requirements
    pub fn is_valid(&self) -> bool {
        !self.property_description.is_empty()
            && !self.registered_owner.is_empty()
            && self.transfer_duty_paid
    }
}

/// Deeds Offices in South Africa
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeedsOffice {
    /// Pretoria (Gauteng, Limpopo, Mpumalanga, North West)
    Pretoria,
    /// Cape Town (Western Cape, Northern Cape)
    CapeTown,
    /// Johannesburg (Gauteng)
    Johannesburg,
    /// Pietermaritzburg (KwaZulu-Natal)
    Pietermaritzburg,
    /// Bloemfontein (Free State)
    Bloemfontein,
    /// King William's Town (Eastern Cape)
    KingWilliamsTown,
    /// Vryburg (North West)
    Vryburg,
}

/// Sectional title ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionalTitle {
    /// Sectional title scheme name
    pub scheme_name: String,
    /// Section number
    pub section_number: u32,
    /// Participation quota (share in common property)
    pub participation_quota: f64,
    /// Levy contribution (monthly)
    pub monthly_levy: Zar,
    /// Body corporate registered
    pub body_corporate_registered: bool,
}

impl SectionalTitle {
    /// Calculate levy from participation quota
    pub fn calculate_levy(total_levy_budget: Zar, participation_quota: f64) -> Zar {
        Zar::from_cents((total_levy_budget.cents() as f64 * participation_quota) as i64)
    }
}

/// Land reform types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandReformType {
    /// Restitution (restore land to dispossessed)
    Restitution,
    /// Redistribution (land to landless)
    Redistribution,
    /// Tenure reform (secure tenure rights)
    TenureReform,
}

/// Land restitution claim (s25(7) Constitution)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestitutionClaim {
    /// Dispossession date (must be after 19 June 1913)
    pub dispossession_date: String,
    /// Claimant name
    pub claimant: String,
    /// Property description
    pub property_description: String,
    /// Claim lodged by deadline (31 Dec 1998 / reopened 2014)
    pub lodged_by_deadline: bool,
}

impl RestitutionClaim {
    /// Validate if claim is eligible
    pub fn is_eligible(&self) -> bool {
        self.lodged_by_deadline && !self.property_description.is_empty()
    }
}

/// Extension of Security of Tenure (ESTA) - farm dwellers/labour tenants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstaProtection {
    /// Occupier name
    pub occupier: String,
    /// Period of occupation (years)
    pub years_of_occupation: u32,
    /// Is over 60 or disabled
    pub over_60_or_disabled: bool,
    /// Has consent to occupy
    pub has_consent: bool,
}

impl EstaProtection {
    /// Check if occupier has long-term security
    pub fn has_long_term_security(&self) -> bool {
        self.years_of_occupation >= 10 || self.over_60_or_disabled
    }

    /// Notice period for eviction (months)
    pub fn eviction_notice_period(&self) -> u8 {
        if self.has_long_term_security() {
            12 // 12 months for long-term occupiers
        } else if self.years_of_occupation >= 5 {
            6 // 6 months for 5-10 years
        } else {
            1 // 1 month for less than 5 years
        }
    }
}

/// Transfer duty rates (2024)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDuty {
    /// Property value
    pub property_value: Zar,
}

impl TransferDuty {
    /// Calculate transfer duty (2024 rates)
    pub fn calculate(&self) -> Zar {
        let value = self.property_value.cents();

        // Exemption threshold: R1,100,000
        if value <= 110_000_000 {
            return Zar::from_cents(0);
        }

        // R1,100,001 - R1,512,500: 3% on value above R1,100,000
        if value <= 151_250_000 {
            return Zar::from_cents(((value - 110_000_000) as f64 * 0.03) as i64);
        }

        // R1,512,501 - R2,117,500: R12,375 + 6% on value above R1,512,500
        if value <= 211_750_000 {
            let base = 1_237_500;
            let additional = ((value - 151_250_000) as f64 * 0.06) as i64;
            return Zar::from_cents(base + additional);
        }

        // R2,117,501 - R2,722,500: R48,675 + 8% on value above R2,117,500
        if value <= 272_250_000 {
            let base = 4_867_500;
            let additional = ((value - 211_750_000) as f64 * 0.08) as i64;
            return Zar::from_cents(base + additional);
        }

        // R2,722,501 - R12,100,000: R97,075 + 11% on value above R2,722,500
        if value <= 1_210_000_000 {
            let base = 9_707_500;
            let additional = ((value - 272_250_000) as f64 * 0.11) as i64;
            return Zar::from_cents(base + additional);
        }

        // Above R12,100,000: R1,128,600 + 13% on value above R12,100,000
        let base = 112_860_000;
        let additional = ((value - 1_210_000_000) as f64 * 0.13) as i64;
        Zar::from_cents(base + additional)
    }
}

/// Property errors
#[derive(Debug, Error)]
pub enum PropertyError {
    /// Unregistered property
    #[error("Property not registered in Deeds Office")]
    PropertyNotRegistered,

    /// Transfer duty not paid
    #[error("Transfer duty not paid (R{amount})")]
    TransferDutyNotPaid { amount: i64 },

    /// Invalid restitution claim
    #[error("Invalid restitution claim: {reason}")]
    InvalidRestitutionClaim { reason: String },

    /// ESTA eviction without proper notice
    #[error(
        "ESTA eviction without proper notice (required: {required} months, given: {given} months)"
    )]
    EstaEvictionInvalid { required: u8, given: u8 },

    /// Sectional title compliance
    #[error("Sectional title non-compliance: {description}")]
    SectionalTitleNonCompliance { description: String },

    /// Defective title
    #[error("Defective title: {defect}")]
    DefectiveTitle { defect: String },
}

/// Validate deeds registration
pub fn validate_deeds_registration(registration: &DeedsRegistration) -> PropertyResult<()> {
    if !registration.transfer_duty_paid {
        let duty = TransferDuty {
            property_value: registration.purchase_price,
        }
        .calculate();
        return Err(PropertyError::TransferDutyNotPaid {
            amount: duty.rands(),
        });
    }

    if registration.property_description.is_empty() {
        return Err(PropertyError::DefectiveTitle {
            defect: "No property description".to_string(),
        });
    }

    Ok(())
}

/// Get property law compliance checklist
pub fn get_property_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Title deed search", "Deeds Registries Act"),
        ("Transfer duty calculated and paid", "Transfer Duty Act"),
        ("Deeds Office registration", "s3 Deeds Registries Act"),
        ("Conveyancer appointed", "s15 Deeds Registries Act"),
        (
            "Bond registration (if financed)",
            "s50 Deeds Registries Act",
        ),
        ("Rates clearance certificate", "Municipal"),
        (
            "Levy clearance (sectional title)",
            "s15B Sectional Titles Act",
        ),
        ("ESTA compliance (farm/rural)", "ESTA s8-9"),
        ("Restitution check", "Restitution Act"),
        ("Mineral rights separated (if applicable)", "MPRDA"),
        ("Servitudes noted", "Deeds Registries Act"),
        ("Building plans approved", "Municipal"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_percentage() {
        assert_eq!(OwnershipType::FullOwnership.ownership_percentage(), 100.0);
        assert_eq!(
            OwnershipType::CoOwnership {
                share_numerator: 1,
                share_denominator: 2
            }
            .ownership_percentage(),
            50.0
        );
    }

    #[test]
    fn test_transfer_duty_exempt() {
        let duty = TransferDuty {
            property_value: Zar::from_rands(1_000_000),
        };
        assert_eq!(duty.calculate().rands(), 0);
    }

    #[test]
    fn test_transfer_duty_calculation() {
        let duty = TransferDuty {
            property_value: Zar::from_rands(2_000_000),
        };
        let calculated = duty.calculate();
        assert!(calculated.rands() > 0);
    }

    #[test]
    fn test_sectional_title_levy() {
        let levy = SectionalTitle::calculate_levy(Zar::from_rands(100_000), 0.05);
        assert_eq!(levy.rands(), 5_000);
    }

    #[test]
    fn test_esta_long_term_security() {
        let protection = EstaProtection {
            occupier: "Test Occupier".to_string(),
            years_of_occupation: 12,
            over_60_or_disabled: false,
            has_consent: true,
        };
        assert!(protection.has_long_term_security());
        assert_eq!(protection.eviction_notice_period(), 12);
    }

    #[test]
    fn test_esta_short_term() {
        let protection = EstaProtection {
            occupier: "Test Occupier".to_string(),
            years_of_occupation: 3,
            over_60_or_disabled: false,
            has_consent: true,
        };
        assert!(!protection.has_long_term_security());
        assert_eq!(protection.eviction_notice_period(), 1);
    }

    #[test]
    fn test_esta_elderly_protection() {
        let protection = EstaProtection {
            occupier: "Elderly Occupier".to_string(),
            years_of_occupation: 5,
            over_60_or_disabled: true,
            has_consent: true,
        };
        assert!(protection.has_long_term_security());
        assert_eq!(protection.eviction_notice_period(), 12);
    }

    #[test]
    fn test_restitution_claim_eligible() {
        let claim = RestitutionClaim {
            dispossession_date: "1960-01-01".to_string(),
            claimant: "Test Claimant".to_string(),
            property_description: "Erf 123 Cape Town".to_string(),
            lodged_by_deadline: true,
        };
        assert!(claim.is_eligible());
    }

    #[test]
    fn test_deeds_registration_valid() {
        let registration = DeedsRegistration {
            property_description: "Erf 456 Pretoria".to_string(),
            title_deed_number: Some("T12345/2024".to_string()),
            registered_owner: "John Doe".to_string(),
            purchase_price: Zar::from_rands(1_500_000),
            transfer_duty_paid: true,
            deeds_office: DeedsOffice::Pretoria,
        };
        assert!(registration.is_valid());
        assert!(validate_deeds_registration(&registration).is_ok());
    }

    #[test]
    fn test_deeds_registration_no_duty() {
        let registration = DeedsRegistration {
            property_description: "Erf 789 Johannesburg".to_string(),
            title_deed_number: None,
            registered_owner: "Jane Smith".to_string(),
            purchase_price: Zar::from_rands(2_000_000),
            transfer_duty_paid: false,
            deeds_office: DeedsOffice::Johannesburg,
        };
        assert!(validate_deeds_registration(&registration).is_err());
    }

    #[test]
    fn test_property_checklist() {
        let checklist = get_property_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
