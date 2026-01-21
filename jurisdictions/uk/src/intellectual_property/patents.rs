//! Patents Act 1977 Implementation
//!
//! UK patent law implementing the European Patent Convention (EPC).
//!
//! ## Patentability Requirements (s.1)
//!
//! An invention is patentable if it:
//! 1. **Is new** (s.2) - Not part of the state of the art
//! 2. **Involves an inventive step** (s.3) - Not obvious to skilled person
//! 3. **Is capable of industrial application** (s.4) - Can be made or used
//! 4. **Is not excluded** (s.1(2)) - Not a discovery, scientific theory, mathematical method, etc.
//!
//! ## Key Cases
//!
//! - **Windsurfing v Tabur Marine \[1985\]**: Four-step obviousness test
//! - **Pozzoli v BDMO \[2007\]**: Restated Windsurfing test
//! - **Actavis v Eli Lilly \[2017\]**: Purposive construction of claims

use super::error::{IpError, IpResult};
use super::types::{IpOwner, PriorArt, RegistrationStatus};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Patent under UK law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Patent {
    /// Patent number (e.g., GB2123456)
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
    /// Filing date
    pub filing_date: NaiveDate,
    /// Priority date (if claiming priority)
    pub priority_date: Option<NaiveDate>,
    /// Grant date
    pub grant_date: Option<NaiveDate>,
    /// Expiry date (20 years from filing)
    pub expiry_date: NaiveDate,
    /// Status
    pub status: RegistrationStatus,
    /// IPC classification
    pub ipc_classification: Vec<String>,
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
    /// Description of invention
    pub description: String,
    /// Claims
    pub claims: Vec<PatentClaim>,
    /// Inventor(s)
    pub inventors: Vec<String>,
    /// Applicant
    pub applicant: IpOwner,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Priority date (if claiming priority under Paris Convention)
    pub priority_date: Option<NaiveDate>,
    /// Prior art known to applicant
    pub prior_art: Vec<PriorArt>,
    /// Technical field
    pub technical_field: String,
}

/// Patentability assessment result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Patentability {
    /// Is the invention new? (s.2)
    pub novelty: Novelty,
    /// Does it involve an inventive step? (s.3)
    pub inventive_step: InventiveStep,
    /// Is it capable of industrial application? (s.4)
    pub industrial_application: IndustrialApplicability,
    /// Is subject matter excluded? (s.1(2))
    pub excluded_subject_matter: Vec<String>,
    /// Overall patentable?
    pub is_patentable: bool,
    /// Reasons if not patentable
    pub refusal_reasons: Vec<String>,
}

/// Novelty assessment (Patents Act 1977 s.2)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Novelty {
    /// Is novel?
    pub is_novel: bool,
    /// Anticipating prior art (if not novel)
    pub anticipating_art: Vec<PriorArt>,
    /// Novelty analysis
    pub analysis: String,
}

/// Inventive step assessment (Patents Act 1977 s.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventiveStep {
    /// Has inventive step?
    pub has_inventive_step: bool,
    /// Obviousness analysis (Pozzoli test)
    pub pozzoli_analysis: PozzoliTest,
    /// Conclusion
    pub conclusion: String,
}

/// Pozzoli test for obviousness (Pozzoli v BDMO \[2007\])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PozzoliTest {
    /// Step 1: Identify the notional skilled person
    pub skilled_person: String,
    /// Step 2: Identify the common general knowledge
    pub common_general_knowledge: Vec<String>,
    /// Step 3: Identify the inventive concept
    pub inventive_concept: String,
    /// Step 4: Assess obviousness without hindsight
    pub obvious_without_hindsight: bool,
}

/// Industrial applicability assessment (Patents Act 1977 s.4)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndustrialApplicability {
    /// Capable of industrial application?
    pub is_industrially_applicable: bool,
    /// Industry/field of application
    pub industry: Vec<String>,
    /// Reason if not applicable
    pub reason: Option<String>,
}

/// Patent infringement analysis (s.60)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatentInfringement {
    /// Allegedly infringing product/process
    pub accused_product: String,
    /// Patent being asserted
    pub patent_number: String,
    /// Claims allegedly infringed
    pub infringed_claims: Vec<u32>,
    /// Type of infringement
    pub infringement_type: InfringementType,
    /// Direct infringement (s.60(1))?
    pub direct_infringement: bool,
    /// Indirect infringement (s.60(2))?
    pub indirect_infringement: bool,
    /// Defenses available
    pub defenses: Vec<InfringementDefense>,
}

/// Type of patent infringement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementType {
    /// Literal infringement (all claim elements present)
    Literal,
    /// Doctrine of equivalents
    Equivalents,
    /// Contributory infringement (supplying means)
    Contributory,
    /// No infringement
    None,
}

/// Defenses to patent infringement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementDefense {
    /// Patent invalid (s.72)
    Invalidity(String),
    /// Prior use (s.64)
    PriorUse {
        /// Date of prior use
        date: NaiveDate,
    },
    /// Exhaustion (first sale doctrine)
    Exhaustion,
    /// Experimental use
    ExperimentalUse,
    /// Private non-commercial use
    PrivateUse,
}

/// Validates patent claim format and content
pub fn validate_patent_claim(claim: &PatentClaim) -> IpResult<()> {
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

/// Checks novelty (Patents Act 1977 s.2)
///
/// An invention is new if it does not form part of the state of the art.
/// State of the art = everything made available to the public before priority date.
pub fn check_novelty(application: &PatentApplication, prior_art: &[PriorArt]) -> IpResult<Novelty> {
    let mut anticipating_art = Vec::new();
    let priority_date = application.priority_date.unwrap_or(application.filing_date);

    // Check if any prior art anticipates the invention
    for art in prior_art {
        if let Some(pub_date) = art.publication_date
            && pub_date < priority_date
        {
            // Prior art is earlier - check if it discloses all claim elements
            // Simplified: in reality, this requires detailed comparison
            if art.relevance.contains("discloses all elements") {
                anticipating_art.push(art.clone());
            }
        }
    }

    let is_novel = anticipating_art.is_empty();
    let analysis = if is_novel {
        "No prior art discloses all claim elements. Invention is novel.".to_string()
    } else {
        format!(
            "Anticipated by {} prior art reference(s)",
            anticipating_art.len()
        )
    };

    Ok(Novelty {
        is_novel,
        anticipating_art,
        analysis,
    })
}

/// Assesses inventive step using Pozzoli test (Patents Act 1977 s.3)
///
/// Pozzoli v BDMO \[2007\] restated the Windsurfing test:
/// 1. Identify the notional skilled person and their common general knowledge
/// 2. Identify the inventive concept of the claim
/// 3. Identify differences between prior art and inventive concept
/// 4. Assess whether those differences would be obvious to the skilled person
pub fn assess_inventive_step(
    application: &PatentApplication,
    prior_art: &[PriorArt],
) -> IpResult<InventiveStep> {
    // Step 1: Identify skilled person (domain expert in technical field)
    let skilled_person = format!(
        "Person skilled in the art of {}",
        application.technical_field
    );

    // Step 2: Common general knowledge in the field
    let common_general_knowledge = vec![
        format!("Standard practices in {}", application.technical_field),
        "Knowledge from textbooks and technical literature".to_string(),
    ];

    // Step 3: Inventive concept (from claims)
    let inventive_concept = if !application.claims.is_empty() {
        application.claims[0].text.clone()
    } else {
        application.description.clone()
    };

    // Step 4: Would it be obvious?
    // Simplified: check if combination of prior art makes it obvious
    let obvious = prior_art.len() >= 2
        && prior_art
            .iter()
            .all(|art| art.relevance.contains("closely related"));

    let pozzoli = PozzoliTest {
        skilled_person,
        common_general_knowledge,
        inventive_concept,
        obvious_without_hindsight: obvious,
    };

    let conclusion = if obvious {
        "Obvious to skilled person in light of prior art combination".to_string()
    } else {
        "Not obvious - involves inventive step".to_string()
    };

    Ok(InventiveStep {
        has_inventive_step: !obvious,
        pozzoli_analysis: pozzoli,
        conclusion,
    })
}

/// Checks industrial applicability (Patents Act 1977 s.4)
///
/// An invention is capable of industrial application if it can be made
/// or used in any kind of industry (including agriculture).
pub fn check_industrial_application(
    application: &PatentApplication,
) -> IpResult<IndustrialApplicability> {
    // Methods of treatment of human/animal body are excluded (s.4(2))
    let is_medical_treatment = application.technical_field.contains("medical treatment")
        || application.technical_field.contains("surgery")
        || application.technical_field.contains("therapy");

    if is_medical_treatment && !application.description.contains("apparatus") {
        return Ok(IndustrialApplicability {
            is_industrially_applicable: false,
            industry: vec![],
            reason: Some(
                "Methods of treatment of human/animal body excluded by s.4(2)".to_string(),
            ),
        });
    }

    // Check if it can be made or used
    let has_practical_utility = !application.description.contains("purely theoretical")
        && !application.description.contains("abstract concept");

    if !has_practical_utility {
        return Ok(IndustrialApplicability {
            is_industrially_applicable: false,
            industry: vec![],
            reason: Some("No practical utility demonstrated".to_string()),
        });
    }

    Ok(IndustrialApplicability {
        is_industrially_applicable: true,
        industry: vec![application.technical_field.clone()],
        reason: None,
    })
}

/// Comprehensive patentability check (s.1-4)
pub fn check_patentability(
    application: &PatentApplication,
    prior_art: &[PriorArt],
) -> IpResult<Patentability> {
    // Check excluded subject matter (s.1(2))
    let mut excluded = Vec::new();
    let excluded_terms = [
        "discovery",
        "scientific theory",
        "mathematical method",
        "aesthetic creation",
        "scheme, rule or method for performing a mental act",
        "playing a game",
        "doing business",
        "program for a computer",
        "presentation of information",
    ];

    for term in &excluded_terms {
        if application.description.to_lowercase().contains(term)
            || application.title.to_lowercase().contains(term)
        {
            excluded.push(term.to_string());
        }
    }

    // If excluded subject matter "as such", not patentable
    let has_technical_contribution = !excluded.is_empty()
        && (application.description.contains("technical effect")
            || application.description.contains("technical contribution"));

    if !excluded.is_empty() && !has_technical_contribution {
        return Ok(Patentability {
            novelty: Novelty {
                is_novel: false,
                anticipating_art: vec![],
                analysis: "Not assessed - excluded subject matter".to_string(),
            },
            inventive_step: InventiveStep {
                has_inventive_step: false,
                pozzoli_analysis: PozzoliTest {
                    skilled_person: String::new(),
                    common_general_knowledge: vec![],
                    inventive_concept: String::new(),
                    obvious_without_hindsight: true,
                },
                conclusion: "Not assessed - excluded subject matter".to_string(),
            },
            industrial_application: IndustrialApplicability {
                is_industrially_applicable: false,
                industry: vec![],
                reason: Some("Excluded subject matter".to_string()),
            },
            excluded_subject_matter: excluded.clone(),
            is_patentable: false,
            refusal_reasons: vec![format!("Excluded subject matter: {}", excluded.join(", "))],
        });
    }

    // Check novelty
    let novelty = check_novelty(application, prior_art)?;
    if !novelty.is_novel {
        return Err(IpError::LacksNovelty {
            prior_art: novelty.anticipating_art[0].reference.clone(),
        });
    }

    // Check inventive step
    let inventive_step = assess_inventive_step(application, prior_art)?;
    if !inventive_step.has_inventive_step {
        return Err(IpError::LacksInventiveStep {
            prior_art: format!("{} prior art references", prior_art.len()),
        });
    }

    // Check industrial applicability
    let industrial = check_industrial_application(application)?;
    if !industrial.is_industrially_applicable {
        return Err(IpError::NotIndustriallyApplicable {
            reason: industrial.reason.unwrap_or_default(),
        });
    }

    Ok(Patentability {
        novelty,
        inventive_step,
        industrial_application: industrial,
        excluded_subject_matter: excluded,
        is_patentable: true,
        refusal_reasons: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_application() -> PatentApplication {
        PatentApplication {
            title: "Novel pharmaceutical compound".to_string(),
            description: "A new compound with technical effect of treating disease X".to_string(),
            claims: vec![PatentClaim {
                number: 1,
                text: "A compound of formula C6H12O6 for treating disease X".to_string(),
                independent: true,
                depends_on: None,
            }],
            inventors: vec!["Dr. Smith".to_string()],
            applicant: IpOwner {
                name: "Pharma Co Ltd".to_string(),
                owner_type: super::super::types::OwnerType::Company,
                address: Some("London, UK".to_string()),
                country: "GB".to_string(),
            },
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            priority_date: None,
            prior_art: vec![],
            technical_field: "pharmaceuticals".to_string(),
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
    fn test_validate_claim_too_short() {
        let claim = PatentClaim {
            number: 1,
            text: "Short".to_string(),
            independent: true,
            depends_on: None,
        };

        assert!(validate_patent_claim(&claim).is_err());
    }

    #[test]
    fn test_novelty_with_no_prior_art() {
        let app = create_test_application();
        let novelty = check_novelty(&app, &[]).unwrap();

        assert!(novelty.is_novel);
        assert!(novelty.anticipating_art.is_empty());
    }

    #[test]
    fn test_novelty_with_anticipating_prior_art() {
        let app = create_test_application();
        let prior_art = vec![PriorArt {
            art_type: super::super::types::PriorArtType::Publication,
            reference: "Journal Article 2023".to_string(),
            publication_date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            relevance: "discloses all elements of the invention".to_string(),
        }];

        let novelty = check_novelty(&app, &prior_art).unwrap();
        assert!(!novelty.is_novel);
        assert_eq!(novelty.anticipating_art.len(), 1);
    }

    #[test]
    fn test_inventive_step_obvious() {
        let app = create_test_application();
        let prior_art = vec![
            PriorArt {
                art_type: super::super::types::PriorArtType::Patent,
                reference: "GB1234567".to_string(),
                publication_date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
                relevance: "closely related pharmaceutical compound".to_string(),
            },
            PriorArt {
                art_type: super::super::types::PriorArtType::Publication,
                reference: "Chemistry Journal 2023".to_string(),
                publication_date: Some(NaiveDate::from_ymd_opt(2023, 6, 1).unwrap()),
                relevance: "closely related synthesis method".to_string(),
            },
        ];

        let step = assess_inventive_step(&app, &prior_art).unwrap();
        assert!(!step.has_inventive_step);
    }

    #[test]
    fn test_industrial_application_valid() {
        let app = create_test_application();
        let industrial = check_industrial_application(&app).unwrap();

        assert!(industrial.is_industrially_applicable);
        assert!(industrial.industry.contains(&"pharmaceuticals".to_string()));
    }

    #[test]
    fn test_industrial_application_medical_treatment() {
        let mut app = create_test_application();
        app.technical_field = "medical treatment".to_string();
        app.description = "A method of treating cancer by surgery".to_string();

        let industrial = check_industrial_application(&app).unwrap();
        assert!(!industrial.is_industrially_applicable);
    }

    #[test]
    fn test_patentability_excluded_subject_matter() {
        let mut app = create_test_application();
        app.description = "A mathematical method for calculating prime numbers".to_string();

        let result = check_patentability(&app, &[]).unwrap();
        assert!(!result.is_patentable);
        assert!(!result.excluded_subject_matter.is_empty());
    }

    #[test]
    fn test_patentability_valid() {
        let app = create_test_application();
        let result = check_patentability(&app, &[]).unwrap();

        assert!(result.is_patentable);
        assert!(result.novelty.is_novel);
        assert!(result.inventive_step.has_inventive_step);
        assert!(result.industrial_application.is_industrially_applicable);
    }

    #[test]
    fn test_patentability_lacks_novelty() {
        let app = create_test_application();
        let prior_art = vec![PriorArt {
            art_type: super::super::types::PriorArtType::Publication,
            reference: "Prior Publication 2023".to_string(),
            publication_date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            relevance: "discloses all elements of claim 1".to_string(),
        }];

        let result = check_patentability(&app, &prior_art);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IpError::LacksNovelty { .. }));
    }
}
