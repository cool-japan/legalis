//! Patents Act 1990 Implementation
//!
//! Australian patent law based on the "manner of manufacture" test.
//!
//! ## Patentability Requirements (s.18)
//!
//! An invention is patentable if it:
//! 1. **Is a manner of manufacture** (s.18(1)(a)) - NRDC test
//! 2. **Is novel** (s.18(1)(b)(i)) - Not in prior art base
//! 3. **Involves an inventive step** (s.18(1)(b)(ii)) - Not obvious
//! 4. **Is useful** (s.18(1)(c)) - Has practical utility
//! 5. **Was not secretly used** (s.18(1A)) - No secret commercial use
//!
//! ## Key Cases
//!
//! - **NRDC v Commissioner of Patents (1959)**: Manner of manufacture test
//! - **Lockwood v Doric (2004)**: Inventive step/obviousness
//! - **D'Arcy v Myriad Genetics (2015)**: Isolated gene sequences not patentable
//! - **Aktiebolaget Hässle v Alphapharm (2002)**: Selection patents
//!
//! ## Excluded Subject Matter (s.18(2))
//!
//! - Human beings and biological processes for their generation
//! - Inventions contrary to law

use super::error::{IpError, Result};
use super::types::{IpOwner, PriorArt, RegistrationStatus};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Patent under Australian law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Patent {
    /// Patent number (e.g., AU2024100001)
    pub patent_number: String,
    /// Title of invention
    pub title: String,
    /// Abstract
    pub abstract_text: String,
    /// Patent claims
    pub claims: Vec<PatentClaim>,
    /// Inventor(s)
    pub inventors: Vec<String>,
    /// Owner/applicant
    pub owner: IpOwner,
    /// Filing date (complete application)
    pub filing_date: NaiveDate,
    /// Priority date (if claiming priority under Paris Convention)
    pub priority_date: Option<NaiveDate>,
    /// Grant date
    pub grant_date: Option<NaiveDate>,
    /// Expiry date (20 years from filing for standard patents)
    pub expiry_date: NaiveDate,
    /// Status
    pub status: RegistrationStatus,
    /// IPC classification
    pub ipc_classification: Vec<String>,
    /// Whether divisional application
    pub is_divisional: bool,
    /// Parent application (if divisional)
    pub parent_application: Option<String>,
}

/// Patent claim
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PatentClaim {
    /// Claim number
    pub number: u32,
    /// Claim text
    pub text: String,
    /// Whether this is an independent claim
    pub independent: bool,
    /// Dependent on claim number (if dependent claim)
    pub depends_on: Option<u32>,
}

/// Patent application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatentApplication {
    /// Title of invention
    pub title: String,
    /// Description/specification
    pub description: String,
    /// Claims
    pub claims: Vec<PatentClaim>,
    /// Inventor(s)
    pub inventors: Vec<String>,
    /// Applicant
    pub applicant: IpOwner,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Priority date (if claiming Paris Convention priority)
    pub priority_date: Option<NaiveDate>,
    /// Prior art known to applicant
    pub prior_art: Vec<PriorArt>,
    /// Technical field
    pub technical_field: String,
    /// Whether provisional application
    pub is_provisional: bool,
}

/// Patentability assessment result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Patentability {
    /// Is a manner of manufacture? (s.18(1)(a))
    pub manner_of_manufacture: MannerOfManufacture,
    /// Is the invention new? (s.18(1)(b)(i))
    pub novelty: Novelty,
    /// Does it involve an inventive step? (s.18(1)(b)(ii))
    pub inventive_step: InventiveStep,
    /// Is it useful? (s.18(1)(c))
    pub industrial_application: IndustrialApplicability,
    /// Is subject matter excluded? (s.18(2))
    pub excluded_subject_matter: Vec<String>,
    /// Overall patentable?
    pub is_patentable: bool,
    /// Reasons if not patentable
    pub refusal_reasons: Vec<String>,
}

/// Manner of manufacture assessment (NRDC test)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MannerOfManufacture {
    /// Is a manner of manufacture?
    pub is_manner_of_manufacture: bool,
    /// Does it have an artificially created state of affairs?
    pub artificially_created_state: bool,
    /// Is it economically useful?
    pub economic_utility: bool,
    /// Analysis
    pub analysis: String,
}

/// Novelty assessment (Patents Act 1990 s.7(1))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Novelty {
    /// Is novel?
    pub is_novel: bool,
    /// Anticipating prior art (if not novel)
    pub anticipating_art: Vec<PriorArt>,
    /// Novelty analysis
    pub analysis: String,
}

/// Inventive step assessment (Patents Act 1990 s.7(2))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventiveStep {
    /// Has inventive step?
    pub has_inventive_step: bool,
    /// Common general knowledge in relevant field
    pub common_general_knowledge: Vec<String>,
    /// Person skilled in the art
    pub skilled_person: String,
    /// Obviousness conclusion
    pub conclusion: String,
}

/// Industrial applicability/usefulness assessment (s.18(1)(c))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndustrialApplicability {
    /// Is useful?
    pub is_useful: bool,
    /// Utility description
    pub utility: Vec<String>,
    /// Reason if not useful
    pub reason: Option<String>,
}

/// Patent infringement analysis (s.117)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatentInfringement {
    /// Allegedly infringing product/process
    pub accused_product: String,
    /// Patent being asserted
    pub patent_number: String,
    /// Claims allegedly infringed
    pub infringed_claims: Vec<u32>,
    /// Direct infringement (s.117(1))?
    pub direct_infringement: bool,
    /// Indirect/contributory infringement (s.117(2))?
    pub indirect_infringement: bool,
    /// Defenses available
    pub defenses: Vec<InfringementDefense>,
}

/// Defenses to patent infringement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementDefense {
    /// Patent invalid (s.138)
    Invalidity(String),
    /// Prior use (s.119)
    PriorUse {
        /// Date of prior use
        date: NaiveDate,
    },
    /// Exhaustion
    Exhaustion,
    /// Experimental use (s.119C)
    ExperimentalUse,
    /// Regulatory use (Bolar exemption - s.119A)
    RegulatoryUse,
    /// Private and non-commercial use
    PrivateUse,
}

/// Validates patent claim format and content
pub fn validate_patent_claim(claim: &PatentClaim) -> Result<()> {
    if claim.text.trim().is_empty() {
        return Err(IpError::InvalidClaim {
            reason: "Claim text cannot be empty".to_string(),
        });
    }

    if claim.text.len() < 20 {
        return Err(IpError::InvalidClaim {
            reason: "Claim must be substantive (at least 20 characters)".to_string(),
        });
    }

    // Check dependency validity
    if !claim.independent && claim.depends_on.is_none() {
        return Err(IpError::InvalidClaim {
            reason: "Dependent claim must specify which claim it depends on".to_string(),
        });
    }

    if claim.independent && claim.depends_on.is_some() {
        return Err(IpError::InvalidClaim {
            reason: "Independent claim cannot depend on another claim".to_string(),
        });
    }

    Ok(())
}

/// Checks manner of manufacture (NRDC v Commissioner of Patents test)
///
/// The NRDC test asks:
/// 1. Does the claimed invention result in an "artificially created state of affairs"?
/// 2. Is the result of economic utility?
pub fn check_manner_of_manufacture(application: &PatentApplication) -> Result<MannerOfManufacture> {
    // Check for excluded subject matter first (s.18(2))
    let excluded_terms = [
        "human being",
        "biological process for generation of human",
        "mere discovery",
        "scientific principle",
        "abstract idea",
    ];

    for term in &excluded_terms {
        if application.description.to_lowercase().contains(term) {
            return Ok(MannerOfManufacture {
                is_manner_of_manufacture: false,
                artificially_created_state: false,
                economic_utility: false,
                analysis: format!(
                    "Excluded subject matter under s.18(2): {}. \
                     See D'Arcy v Myriad Genetics (2015)",
                    term
                ),
            });
        }
    }

    // Check for artificial state of affairs
    let has_artificial_state = application.description.contains("method")
        || application.description.contains("process")
        || application.description.contains("composition")
        || application.description.contains("device")
        || application.description.contains("apparatus")
        || application.description.contains("system");

    // Check for economic utility
    let has_economic_utility = !application.technical_field.is_empty()
        && !application.description.contains("purely theoretical")
        && !application.description.contains("aesthetic only");

    let is_mom = has_artificial_state && has_economic_utility;

    let analysis = if is_mom {
        "Invention produces an artificially created state of affairs with economic utility. \
         Satisfies NRDC test for manner of manufacture."
            .to_string()
    } else if !has_artificial_state {
        "Does not produce an artificially created state of affairs. \
         Mere discovery or natural phenomenon."
            .to_string()
    } else {
        "No economic or practical utility demonstrated.".to_string()
    };

    Ok(MannerOfManufacture {
        is_manner_of_manufacture: is_mom,
        artificially_created_state: has_artificial_state,
        economic_utility: has_economic_utility,
        analysis,
    })
}

/// Checks novelty (Patents Act 1990 s.7(1))
///
/// An invention is novel if it is not part of the prior art base.
/// The prior art base includes all prior publications and uses.
pub fn check_novelty(application: &PatentApplication, prior_art: &[PriorArt]) -> Result<Novelty> {
    let mut anticipating_art = Vec::new();
    let priority_date = application.priority_date.unwrap_or(application.filing_date);

    // Check if any prior art anticipates the invention
    for art in prior_art {
        if let Some(pub_date) = art.publication_date
            && pub_date < priority_date
            && (art
                .relevance
                .to_lowercase()
                .contains("discloses all elements")
                || art.relevance.to_lowercase().contains("anticipates"))
        {
            anticipating_art.push(art.clone());
        }
    }

    let is_novel = anticipating_art.is_empty();
    let analysis = if is_novel {
        "No prior art document discloses all essential integers of the claims. \
         Invention is novel under s.7(1)."
            .to_string()
    } else {
        format!(
            "Anticipated by {} prior art reference(s). Not novel under s.7(1).",
            anticipating_art.len()
        )
    };

    Ok(Novelty {
        is_novel,
        anticipating_art,
        analysis,
    })
}

/// Checks industrial applicability/usefulness (s.18(1)(c))
///
/// An invention is useful if it can work as described and has practical utility.
pub fn check_industrial_application(
    application: &PatentApplication,
) -> Result<IndustrialApplicability> {
    // Check if the invention has practical utility
    let has_utility = !application.description.contains("purely theoretical")
        && !application.description.contains("perpetual motion")
        && !application.description.contains("impossible");

    if !has_utility {
        return Ok(IndustrialApplicability {
            is_useful: false,
            utility: vec![],
            reason: Some(
                "Invention lacks practical utility or is scientifically impossible".to_string(),
            ),
        });
    }

    Ok(IndustrialApplicability {
        is_useful: true,
        utility: vec![application.technical_field.clone()],
        reason: None,
    })
}

/// Comprehensive patentability check (s.18)
pub fn check_patentability(
    application: &PatentApplication,
    prior_art: &[PriorArt],
) -> Result<Patentability> {
    // Check manner of manufacture
    let mom = check_manner_of_manufacture(application)?;
    if !mom.is_manner_of_manufacture {
        return Ok(Patentability {
            manner_of_manufacture: mom.clone(),
            novelty: Novelty {
                is_novel: false,
                anticipating_art: vec![],
                analysis: "Not assessed - not a manner of manufacture".to_string(),
            },
            inventive_step: InventiveStep {
                has_inventive_step: false,
                common_general_knowledge: vec![],
                skilled_person: String::new(),
                conclusion: "Not assessed - not a manner of manufacture".to_string(),
            },
            industrial_application: IndustrialApplicability {
                is_useful: false,
                utility: vec![],
                reason: Some("Not assessed - not a manner of manufacture".to_string()),
            },
            excluded_subject_matter: vec![mom.analysis.clone()],
            is_patentable: false,
            refusal_reasons: vec![format!("Not a manner of manufacture: {}", mom.analysis)],
        });
    }

    // Check novelty
    let novelty = check_novelty(application, prior_art)?;
    if !novelty.is_novel {
        return Err(IpError::LacksNovelty {
            prior_art: novelty.anticipating_art[0].reference.clone(),
        });
    }

    // Check inventive step (simplified - in reality requires Lockwood analysis)
    let obvious = prior_art.len() >= 2
        && prior_art
            .iter()
            .all(|art| art.relevance.to_lowercase().contains("closely related"));

    let inventive_step = InventiveStep {
        has_inventive_step: !obvious,
        common_general_knowledge: vec![format!(
            "Standard practices in {}",
            application.technical_field
        )],
        skilled_person: format!(
            "Person skilled in the art of {}",
            application.technical_field
        ),
        conclusion: if obvious {
            "Obvious combination of prior art. See Lockwood v Doric (2004)".to_string()
        } else {
            "Not obvious to person skilled in the art".to_string()
        },
    };

    if !inventive_step.has_inventive_step {
        return Err(IpError::LacksInventiveStep {
            prior_art: format!("{} prior art references", prior_art.len()),
        });
    }

    // Check usefulness
    let industrial = check_industrial_application(application)?;
    if !industrial.is_useful {
        return Err(IpError::NotUseful {
            reason: industrial.reason.unwrap_or_default(),
        });
    }

    Ok(Patentability {
        manner_of_manufacture: mom,
        novelty,
        inventive_step,
        industrial_application: industrial,
        excluded_subject_matter: vec![],
        is_patentable: true,
        refusal_reasons: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intellectual_property::types::{OwnerType, PriorArtType};

    fn create_test_application() -> PatentApplication {
        PatentApplication {
            title: "Novel mining extraction method".to_string(),
            description: "A method for extracting minerals using improved process".to_string(),
            claims: vec![PatentClaim {
                number: 1,
                text: "A method for extracting minerals comprising steps A, B, C".to_string(),
                independent: true,
                depends_on: None,
            }],
            inventors: vec!["Dr. Smith".to_string()],
            applicant: IpOwner {
                name: "Mining Tech Pty Ltd".to_string(),
                owner_type: OwnerType::Company,
                address: Some("Perth, WA".to_string()),
                country: "AU".to_string(),
                abn: Some("12345678901".to_string()),
                acn: Some("123456789".to_string()),
            },
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            priority_date: None,
            prior_art: vec![],
            technical_field: "mining".to_string(),
            is_provisional: false,
        }
    }

    #[test]
    fn test_validate_claim_valid() {
        let claim = PatentClaim {
            number: 1,
            text: "A device comprising a processor and memory".to_string(),
            independent: true,
            depends_on: None,
        };

        assert!(validate_patent_claim(&claim).is_ok());
    }

    #[test]
    fn test_validate_claim_empty() {
        let claim = PatentClaim {
            number: 1,
            text: "".to_string(),
            independent: true,
            depends_on: None,
        };

        assert!(validate_patent_claim(&claim).is_err());
    }

    #[test]
    fn test_manner_of_manufacture_valid() {
        let app = create_test_application();
        let mom = check_manner_of_manufacture(&app).unwrap();

        assert!(mom.is_manner_of_manufacture);
        assert!(mom.artificially_created_state);
        assert!(mom.economic_utility);
    }

    #[test]
    fn test_manner_of_manufacture_human_being() {
        let mut app = create_test_application();
        app.description = "A method for human being generation".to_string();

        let mom = check_manner_of_manufacture(&app).unwrap();
        assert!(!mom.is_manner_of_manufacture);
    }

    #[test]
    fn test_novelty_no_prior_art() {
        let app = create_test_application();
        let novelty = check_novelty(&app, &[]).unwrap();

        assert!(novelty.is_novel);
        assert!(novelty.anticipating_art.is_empty());
    }

    #[test]
    fn test_novelty_with_anticipating_art() {
        let app = create_test_application();
        let prior_art = vec![PriorArt {
            art_type: PriorArtType::AustralianPatent,
            reference: "AU2023100001".to_string(),
            publication_date: Some(NaiveDate::from_ymd_opt(2023, 6, 1).unwrap()),
            relevance: "Discloses all elements of the claimed invention".to_string(),
            country: Some("AU".to_string()),
        }];

        let novelty = check_novelty(&app, &prior_art).unwrap();
        assert!(!novelty.is_novel);
        assert_eq!(novelty.anticipating_art.len(), 1);
    }

    #[test]
    fn test_patentability_valid() {
        let app = create_test_application();
        let result = check_patentability(&app, &[]).unwrap();

        assert!(result.is_patentable);
        assert!(result.manner_of_manufacture.is_manner_of_manufacture);
        assert!(result.novelty.is_novel);
        assert!(result.inventive_step.has_inventive_step);
        assert!(result.industrial_application.is_useful);
    }

    #[test]
    fn test_patentability_lacks_novelty() {
        let app = create_test_application();
        let prior_art = vec![PriorArt {
            art_type: PriorArtType::Publication,
            reference: "Journal Article 2023".to_string(),
            publication_date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            relevance: "Discloses all elements - anticipates claim 1".to_string(),
            country: None,
        }];

        let result = check_patentability(&app, &prior_art);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IpError::LacksNovelty { .. }));
    }
}
