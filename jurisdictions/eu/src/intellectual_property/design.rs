//! Community Design Regulation (EC) No 6/2002

use super::error::IpError;
use super::types::{DesignAppearance, DesignProtectionPeriod, DesignType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Community Design (registered or unregistered)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommunityDesign {
    pub design_type: Option<DesignType>,
    pub appearance: Option<DesignAppearance>,
    pub creator: Option<String>,
    pub owner: Option<String>,
    pub is_novel: bool,
    pub has_individual_character: bool,
    pub protection_period: Option<DesignProtectionPeriod>,
}

impl CommunityDesign {
    pub fn new() -> Self {
        Self {
            design_type: None,
            appearance: None,
            creator: None,
            owner: None,
            is_novel: false,
            has_individual_character: false,
            protection_period: None,
        }
    }

    pub fn with_design_type(mut self, design_type: DesignType) -> Self {
        self.design_type = Some(design_type);
        self
    }

    pub fn with_appearance(mut self, appearance: DesignAppearance) -> Self {
        self.appearance = Some(appearance);
        self
    }

    pub fn with_creator(mut self, creator: impl Into<String>) -> Self {
        self.creator = Some(creator.into());
        self
    }

    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    pub fn with_novelty(mut self, is_novel: bool) -> Self {
        self.is_novel = is_novel;
        self
    }

    pub fn with_individual_character(mut self, has_individual_character: bool) -> Self {
        self.has_individual_character = has_individual_character;
        self
    }

    pub fn with_protection_period(mut self, period: DesignProtectionPeriod) -> Self {
        self.protection_period = Some(period);
        self
    }

    /// Validate Community Design under Regulation (EC) No 6/2002
    ///
    /// Checks:
    /// - Article 5: Novelty requirement
    /// - Article 6: Individual character requirement
    /// - Article 8: Protection scope
    pub fn validate(&self) -> Result<DesignValidation, IpError> {
        // Check required fields
        if self.design_type.is_none() {
            return Err(IpError::missing_field("design_type"));
        }

        if self.appearance.is_none() {
            return Err(IpError::missing_field("appearance"));
        }

        // Article 5: Novelty requirement
        if !self.is_novel {
            return Err(IpError::invalid_design(
                "Design must be novel - no identical design made available to public before filing (Art. 5)",
            ));
        }

        // Article 6: Individual character requirement
        if !self.has_individual_character {
            return Err(IpError::invalid_design(
                "Design must have individual character - overall impression differs from prior designs (Art. 6)",
            ));
        }

        // Calculate protection status
        let is_protected = if let Some(ref period) = self.protection_period {
            period.is_active()
        } else {
            // Unregistered designs have 3 years from disclosure
            matches!(self.design_type, Some(DesignType::Unregistered))
        };

        let max_protection_years = match self.design_type {
            Some(DesignType::Registered) => 25, // 5 renewal periods of 5 years each
            Some(DesignType::Unregistered) => 3, // 3 years from first disclosure
            None => 0,
        };

        Ok(DesignValidation {
            is_protectable: true,
            novelty_established: self.is_novel,
            individual_character_established: self.has_individual_character,
            is_protected,
            max_protection_years,
            recommendations: Vec::new(),
        })
    }
}

impl Default for CommunityDesign {
    fn default() -> Self {
        Self::new()
    }
}

/// Design validation result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DesignValidation {
    /// Whether design is protectable
    pub is_protectable: bool,

    /// Whether novelty is established
    pub novelty_established: bool,

    /// Whether individual character is established
    pub individual_character_established: bool,

    /// Whether currently protected
    pub is_protected: bool,

    /// Maximum protection period in years
    pub max_protection_years: u8,

    /// Recommendations
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_registered_design() {
        let appearance = DesignAppearance {
            features: vec!["Curved edges".to_string(), "Matte finish".to_string()],
            product_indication: "Smartphone case".to_string(),
        };

        let design = CommunityDesign::new()
            .with_design_type(DesignType::Registered)
            .with_appearance(appearance)
            .with_creator("Jane Designer")
            .with_owner("Design Co")
            .with_novelty(true)
            .with_individual_character(true);

        let result = design.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.is_protectable);
        assert_eq!(validation.max_protection_years, 25);
    }

    #[test]
    fn test_design_lacking_novelty() {
        let appearance = DesignAppearance {
            features: vec!["Standard shape".to_string()],
            product_indication: "Chair".to_string(),
        };

        let design = CommunityDesign::new()
            .with_design_type(DesignType::Registered)
            .with_appearance(appearance)
            .with_novelty(false)
            .with_individual_character(true);

        let result = design.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_unregistered_design_protection() {
        let appearance = DesignAppearance {
            features: vec!["Unique pattern".to_string()],
            product_indication: "Textile".to_string(),
        };

        let design = CommunityDesign::new()
            .with_design_type(DesignType::Unregistered)
            .with_appearance(appearance)
            .with_novelty(true)
            .with_individual_character(true);

        let result = design.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert_eq!(validation.max_protection_years, 3);
    }
}
