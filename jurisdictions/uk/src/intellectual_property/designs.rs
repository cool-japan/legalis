//! UK Design Protection
//!
//! Two forms of design protection in UK:
//! 1. **Registered Designs** (Registered Designs Act 1949)
//! 2. **Unregistered Design Right** (CDPA 1988 Part III)
//!
//! ## Registered Designs (RDA 1949)
//!
//! Requirements (s.1B):
//! - **Novelty**: No identical design disclosed before filing
//! - **Individual character**: Informed user's overall impression differs from earlier designs
//!
//! Exclusions (s.1C):
//! - Features dictated solely by technical function ("must-fit")
//! - Interconnections ("must-match")
//!
//! Duration: 25 years (5-year renewable periods)
//!
//! ## Unregistered Design Right (CDPA Part III)
//!
//! Automatic protection for:
//! - Original designs (not commonplace)
//! - Shape or configuration of article
//!
//! Duration: 15 years from creation (or 10 years from first marketing if earlier)

use super::error::{IpError, IpResult};
use super::types::{IpOwner, RegistrationStatus};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Design (visual appearance of product)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Design {
    /// Design name/title
    pub title: String,
    /// Type of design
    pub design_type: DesignType,
    /// Visual features
    pub features: Vec<String>,
    /// Product to which design is applied
    pub product: String,
    /// Designer/creator
    pub designer: String,
    /// Owner
    pub owner: IpOwner,
    /// Creation date
    pub creation_date: NaiveDate,
    /// First disclosure date
    pub first_disclosure: Option<NaiveDate>,
}

/// Type of design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesignType {
    /// Shape/configuration
    Shape,
    /// Surface decoration/pattern
    Pattern,
    /// Ornamentation
    Ornamentation,
    /// Combination of shape and decoration
    Combined,
}

/// Design registration application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignRegistration {
    /// Design
    pub design: Design,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Registration number (if granted)
    pub registration_number: Option<String>,
    /// Registration date
    pub registration_date: Option<NaiveDate>,
    /// Status
    pub status: RegistrationStatus,
    /// Earlier similar designs
    pub earlier_designs: Vec<String>,
}

/// Design right type (registered vs unregistered)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesignRightType {
    /// Registered design (RDA 1949)
    Registered,
    /// Unregistered design right (CDPA 1988)
    Unregistered,
    /// Community design (retained EU right)
    Community,
}

/// Validates registered design application
pub fn validate_design_registration(registration: &DesignRegistration) -> IpResult<()> {
    // Check novelty - no identical design disclosed before filing
    if !registration.earlier_designs.is_empty() {
        // Simplified: if any earlier design is identical, lacks novelty
        for earlier in &registration.earlier_designs {
            if earlier.contains("identical") {
                return Err(IpError::DesignLacksNovelty {
                    prior_design: earlier.clone(),
                });
            }
        }
    }

    // Check for must-fit exclusions (s.1C(1))
    let has_must_fit = registration
        .design
        .features
        .iter()
        .any(|f| f.contains("solely dictated by function") || f.contains("must-fit"));

    if has_must_fit {
        return Err(IpError::MustFit);
    }

    // Check individual character
    if registration.earlier_designs.len() > 3
        && registration
            .design
            .features
            .iter()
            .all(|f| f.contains("commonplace"))
    {
        return Err(IpError::LacksIndividualCharacter);
    }

    Ok(())
}

/// Validates unregistered design right
pub fn validate_design_right(design: &Design) -> IpResult<()> {
    // Must be original (not commonplace)
    let is_commonplace = design.features.iter().any(|f| f.contains("commonplace"));

    if is_commonplace {
        return Err(IpError::LacksOriginality);
    }

    // Must be shape/configuration (not surface decoration alone)
    if matches!(design.design_type, DesignType::Pattern) {
        // Surface decoration alone not protected by unregistered design right
        return Err(IpError::NotCopyrightWork {
            work_type: "surface decoration alone".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_design() -> Design {
        Design {
            title: "Innovative Chair Design".to_string(),
            design_type: DesignType::Shape,
            features: vec!["Curved backrest".to_string(), "Ergonomic seat".to_string()],
            product: "Office Chair".to_string(),
            designer: "John Designer".to_string(),
            owner: IpOwner {
                name: "Furniture Co".to_string(),
                owner_type: super::super::types::OwnerType::Company,
                address: None,
                country: "GB".to_string(),
            },
            creation_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            first_disclosure: None,
        }
    }

    #[test]
    fn test_validate_design_registration_valid() {
        let registration = DesignRegistration {
            design: create_test_design(),
            filing_date: NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            registration_number: None,
            registration_date: None,
            status: RegistrationStatus::Pending,
            earlier_designs: vec![],
        };

        assert!(validate_design_registration(&registration).is_ok());
    }

    #[test]
    fn test_validate_design_lacks_novelty() {
        let registration = DesignRegistration {
            design: create_test_design(),
            filing_date: NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            registration_number: None,
            registration_date: None,
            status: RegistrationStatus::Pending,
            earlier_designs: vec!["Earlier identical chair design from 2023".to_string()],
        };

        let result = validate_design_registration(&registration);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IpError::DesignLacksNovelty { .. }
        ));
    }

    #[test]
    fn test_validate_design_must_fit() {
        let mut design = create_test_design();
        design.features = vec!["Feature solely dictated by function".to_string()];

        let registration = DesignRegistration {
            design,
            filing_date: NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            registration_number: None,
            registration_date: None,
            status: RegistrationStatus::Pending,
            earlier_designs: vec![],
        };

        let result = validate_design_registration(&registration);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IpError::MustFit));
    }

    #[test]
    fn test_validate_unregistered_design_right_valid() {
        let design = create_test_design();
        assert!(validate_design_right(&design).is_ok());
    }

    #[test]
    fn test_validate_unregistered_design_surface_decoration() {
        let mut design = create_test_design();
        design.design_type = DesignType::Pattern;

        let result = validate_design_right(&design);
        assert!(result.is_err());
    }
}
