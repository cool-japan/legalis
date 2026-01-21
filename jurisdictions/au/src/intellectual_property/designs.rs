//! Designs Act 2003 Implementation
//!
//! Australian design law protecting the visual appearance of products.
//!
//! ## Registration Requirements (s.15-16)
//!
//! A design is registrable if it:
//! 1. **Is new** (s.16(1)) - Not identical to prior art
//! 2. **Is distinctive** (s.16(2)) - Not substantially similar to prior art
//!
//! ## Key Concepts
//!
//! - **Design**: Features of shape, configuration, pattern, ornamentation
//! - **Product**: Thing manufactured or hand-made
//! - **Prior art base**: Designs published/used in Australia or elsewhere

use super::error::{IpError, Result};
use super::types::{IpOwner, PriorArt, RegistrationStatus};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Design under Australian law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Design {
    /// Design number
    pub design_number: String,
    /// Name/title
    pub name: String,
    /// Design type
    pub design_type: DesignType,
    /// Owner
    pub owner: IpOwner,
    /// Product to which design applies
    pub product: String,
    /// Locarno classification
    pub locarno_class: String,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Registration date
    pub registration_date: Option<NaiveDate>,
    /// Certification date (if certified)
    pub certification_date: Option<NaiveDate>,
    /// Expiry date (10 years from filing, renewable to 15)
    pub expiry_date: NaiveDate,
    /// Status
    pub status: RegistrationStatus,
    /// Priority claim
    pub priority_claim: Option<DesignPriorityClaim>,
    /// Representations (images)
    pub representations: Vec<DesignRepresentation>,
    /// Statement of newness and distinctiveness
    pub statement_of_newness: Option<String>,
}

/// Type of design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesignType {
    /// Shape or configuration (3D)
    Shape,
    /// Pattern (2D repeating)
    Pattern,
    /// Ornamentation (2D non-repeating)
    Ornamentation,
    /// Combined
    Combined,
}

/// Priority claim for design
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DesignPriorityClaim {
    /// Country of first filing
    pub country: String,
    /// Application number
    pub application_number: String,
    /// Filing date
    pub filing_date: NaiveDate,
}

/// Design representation (image)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DesignRepresentation {
    /// View description (e.g., "front view", "perspective")
    pub view: String,
    /// Image reference/path
    pub image_reference: String,
    /// Whether this is the main representation
    pub is_primary: bool,
}

/// Design application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignApplication {
    /// Name/title
    pub name: String,
    /// Design type
    pub design_type: DesignType,
    /// Applicant
    pub applicant: IpOwner,
    /// Product
    pub product: String,
    /// Locarno classification
    pub locarno_class: String,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Representations
    pub representations: Vec<DesignRepresentation>,
    /// Statement of newness and distinctiveness
    pub statement_of_newness: Option<String>,
    /// Prior art known to applicant
    pub prior_art: Vec<PriorArt>,
    /// Priority claim
    pub priority_claim: Option<DesignPriorityClaim>,
}

/// Design examination result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignExamination {
    /// Is design new? (s.16(1))
    pub is_new: bool,
    /// Is design distinctive? (s.16(2))
    pub is_distinctive: bool,
    /// Prior designs identified
    pub prior_designs: Vec<PriorDesign>,
    /// Overall registrable?
    pub is_registrable: bool,
    /// Reasons if not registrable
    pub refusal_reasons: Vec<String>,
}

/// Prior design reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorDesign {
    /// Reference (design number or publication)
    pub reference: String,
    /// Product
    pub product: String,
    /// Similarity assessment
    pub similarity: DesignSimilarity,
}

/// Design similarity assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesignSimilarity {
    /// Identical (destroys newness - s.16(1))
    Identical,
    /// Substantially similar in overall impression (destroys distinctiveness - s.16(2))
    SubstantiallySimilar,
    /// Different overall impression
    Different,
}

/// Design infringement (s.71)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignInfringement {
    /// Registered design
    pub registered_design: String,
    /// Allegedly infringing product
    pub infringing_product: String,
    /// Is infringement?
    pub is_infringement: bool,
    /// Basis for infringement
    pub basis: InfringementBasis,
    /// Defenses available
    pub defenses: Vec<DesignDefense>,
}

/// Basis for design infringement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementBasis {
    /// Identical to registered design (s.71(1)(a))
    Identical,
    /// Substantially similar in overall impression (s.71(1)(b))
    SubstantiallySimilar,
    /// No infringement
    None,
}

/// Defense to design infringement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesignDefense {
    /// Design not new (invalidity)
    NotNew(String),
    /// Design not distinctive (invalidity)
    NotDistinctive(String),
    /// Prior use
    PriorUse {
        /// Date of prior use
        date: NaiveDate,
    },
    /// Repair/spare parts (s.72)
    RepairSpares,
    /// Functional features (s.7(1))
    FunctionalFeatures,
}

/// Validates design application
pub fn validate_design_application(application: &DesignApplication) -> Result<()> {
    // Check name/title
    if application.name.trim().is_empty() {
        return Err(IpError::MissingInformation {
            field: "design name".to_string(),
        });
    }

    // Check product specified
    if application.product.trim().is_empty() {
        return Err(IpError::MissingInformation {
            field: "product".to_string(),
        });
    }

    // Check at least one representation
    if application.representations.is_empty() {
        return Err(IpError::MissingInformation {
            field: "design representations".to_string(),
        });
    }

    // Check Locarno classification
    if application.locarno_class.trim().is_empty() {
        return Err(IpError::MissingInformation {
            field: "Locarno classification".to_string(),
        });
    }

    Ok(())
}

/// Check design validity (newness and distinctiveness)
pub fn check_design_validity(
    application: &DesignApplication,
    prior_art: &[PriorArt],
) -> Result<DesignExamination> {
    let priority_date = application
        .priority_claim
        .as_ref()
        .map(|p| p.filing_date)
        .unwrap_or(application.filing_date);

    let mut prior_designs = Vec::new();
    let mut is_new = true;
    let mut is_distinctive = true;
    let mut refusal_reasons = Vec::new();

    // Check against prior art
    for art in prior_art {
        if let Some(pub_date) = art.publication_date
            && pub_date < priority_date
        {
            let similarity = assess_design_similarity(&art.relevance);

            let prior_design = PriorDesign {
                reference: art.reference.clone(),
                product: application.product.clone(),
                similarity,
            };

            match similarity {
                DesignSimilarity::Identical => {
                    is_new = false;
                    refusal_reasons.push(format!(
                        "Not new under s.16(1) - identical to prior design: {}",
                        art.reference
                    ));
                }
                DesignSimilarity::SubstantiallySimilar => {
                    is_distinctive = false;
                    refusal_reasons.push(format!(
                        "Not distinctive under s.16(2) - substantially similar to: {}",
                        art.reference
                    ));
                }
                DesignSimilarity::Different => {}
            }

            prior_designs.push(prior_design);
        }
    }

    Ok(DesignExamination {
        is_new,
        is_distinctive,
        prior_designs,
        is_registrable: is_new && is_distinctive,
        refusal_reasons,
    })
}

/// Assess design similarity from relevance description
fn assess_design_similarity(relevance: &str) -> DesignSimilarity {
    let relevance_lower = relevance.to_lowercase();

    if relevance_lower.contains("identical") {
        DesignSimilarity::Identical
    } else if relevance_lower.contains("substantially similar")
        || relevance_lower.contains("same overall impression")
    {
        DesignSimilarity::SubstantiallySimilar
    } else {
        DesignSimilarity::Different
    }
}

/// Check design infringement (s.71)
pub fn check_design_infringement(
    design: &Design,
    infringing_product: &str,
    similarity: DesignSimilarity,
) -> DesignInfringement {
    // Design must be certified for infringement action
    let is_certified = design.status == RegistrationStatus::Certified;

    let (is_infringement, basis) = if !is_certified {
        (false, InfringementBasis::None)
    } else {
        match similarity {
            DesignSimilarity::Identical => (true, InfringementBasis::Identical),
            DesignSimilarity::SubstantiallySimilar => {
                (true, InfringementBasis::SubstantiallySimilar)
            }
            DesignSimilarity::Different => (false, InfringementBasis::None),
        }
    };

    DesignInfringement {
        registered_design: design.design_number.clone(),
        infringing_product: infringing_product.to_string(),
        is_infringement,
        basis,
        defenses: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intellectual_property::types::{OwnerType, PriorArtType};

    fn create_test_application() -> DesignApplication {
        DesignApplication {
            name: "Phone Case Design".to_string(),
            design_type: DesignType::Shape,
            applicant: IpOwner {
                name: "Design Co Pty Ltd".to_string(),
                owner_type: OwnerType::Company,
                address: Some("Melbourne, VIC".to_string()),
                country: "AU".to_string(),
                abn: Some("12345678901".to_string()),
                acn: Some("123456789".to_string()),
            },
            product: "Phone case".to_string(),
            locarno_class: "14-03".to_string(),
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            representations: vec![DesignRepresentation {
                view: "Perspective view".to_string(),
                image_reference: "image1.png".to_string(),
                is_primary: true,
            }],
            statement_of_newness: Some("New shape of phone case with curved edges".to_string()),
            prior_art: vec![],
            priority_claim: None,
        }
    }

    fn create_test_design() -> Design {
        Design {
            design_number: "AU202400001".to_string(),
            name: "Phone Case Design".to_string(),
            design_type: DesignType::Shape,
            owner: IpOwner {
                name: "Design Co Pty Ltd".to_string(),
                owner_type: OwnerType::Company,
                address: Some("Melbourne, VIC".to_string()),
                country: "AU".to_string(),
                abn: Some("12345678901".to_string()),
                acn: Some("123456789".to_string()),
            },
            product: "Phone case".to_string(),
            locarno_class: "14-03".to_string(),
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            registration_date: Some(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()),
            certification_date: Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
            expiry_date: NaiveDate::from_ymd_opt(2034, 1, 1).unwrap(),
            status: RegistrationStatus::Certified,
            priority_claim: None,
            representations: vec![DesignRepresentation {
                view: "Perspective view".to_string(),
                image_reference: "image1.png".to_string(),
                is_primary: true,
            }],
            statement_of_newness: Some("New shape with curved edges".to_string()),
        }
    }

    #[test]
    fn test_validate_application_valid() {
        let app = create_test_application();
        assert!(validate_design_application(&app).is_ok());
    }

    #[test]
    fn test_validate_application_missing_name() {
        let mut app = create_test_application();
        app.name = "".to_string();

        let result = validate_design_application(&app);
        assert!(result.is_err());
    }

    #[test]
    fn test_design_validity_new() {
        let app = create_test_application();
        let result = check_design_validity(&app, &[]).unwrap();

        assert!(result.is_new);
        assert!(result.is_distinctive);
        assert!(result.is_registrable);
    }

    #[test]
    fn test_design_validity_not_new() {
        let app = create_test_application();
        let prior_art = vec![PriorArt {
            art_type: PriorArtType::PriorDesign,
            reference: "AU202300001".to_string(),
            publication_date: Some(NaiveDate::from_ymd_opt(2023, 6, 1).unwrap()),
            relevance: "Identical phone case design".to_string(),
            country: Some("AU".to_string()),
        }];

        let result = check_design_validity(&app, &prior_art).unwrap();
        assert!(!result.is_new);
        assert!(!result.is_registrable);
    }

    #[test]
    fn test_design_infringement_certified() {
        let design = create_test_design();
        let infringement = check_design_infringement(
            &design,
            "Similar case",
            DesignSimilarity::SubstantiallySimilar,
        );

        assert!(infringement.is_infringement);
        assert_eq!(infringement.basis, InfringementBasis::SubstantiallySimilar);
    }

    #[test]
    fn test_design_infringement_different() {
        let design = create_test_design();
        let infringement =
            check_design_infringement(&design, "Different case", DesignSimilarity::Different);

        assert!(!infringement.is_infringement);
        assert_eq!(infringement.basis, InfringementBasis::None);
    }

    #[test]
    fn test_design_similarity_assessment() {
        assert_eq!(
            assess_design_similarity("Identical design"),
            DesignSimilarity::Identical
        );
        assert_eq!(
            assess_design_similarity("Substantially similar overall impression"),
            DesignSimilarity::SubstantiallySimilar
        );
        assert_eq!(
            assess_design_similarity("Different design"),
            DesignSimilarity::Different
        );
    }
}
