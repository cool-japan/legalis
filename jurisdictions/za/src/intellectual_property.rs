//! South African Intellectual Property Law
//!
//! Protection of intellectual creations through patents, trademarks, copyright, and designs.
//!
//! ## Key Legislation
//!
//! - Patents Act 57 of 1978
//! - Trademarks Act 194 of 1993
//! - Copyright Act 98 of 1978
//! - Designs Act 195 of 1993
//! - Performers' Protection Act 11 of 1967
//! - Counterfeit Goods Act 37 of 1997
//!
//! ## Administered by
//!
//! - Companies and Intellectual Property Commission (CIPC)
//! - Copyright Tribunal

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for IP operations
pub type IpResult<T> = Result<T, IpError>;

/// Intellectual property types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntellectualPropertyType {
    /// Patent (20 years from filing)
    Patent,
    /// Trademark (10 years, renewable indefinitely)
    Trademark,
    /// Copyright (life + 50 years)
    Copyright,
    /// Design (aesthetic/functional)
    Design,
    /// Performers' rights
    PerformersRights,
    /// Trade secret
    TradeSecret,
}

/// Patent registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patent {
    /// Patent number
    pub patent_number: Option<String>,
    /// Invention title
    pub title: String,
    /// Inventor(s)
    pub inventors: Vec<String>,
    /// Is novel
    pub is_novel: bool,
    /// Involves inventive step
    pub inventive_step: bool,
    /// Is industrially applicable
    pub industrially_applicable: bool,
    /// Filing date
    pub filing_date: String,
    /// Granted
    pub granted: bool,
}

impl Patent {
    /// Check if patentable (s25 Patents Act)
    pub fn is_patentable(&self) -> bool {
        self.is_novel && self.inventive_step && self.industrially_applicable
    }

    /// Patent term (20 years from filing - s46)
    pub fn term_years(&self) -> u8 {
        20
    }

    /// Annual renewal fees required
    pub fn requires_renewal_fees(&self) -> bool {
        true
    }
}

/// Non-patentable subject matter (s25(2))
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NonPatentableSubject {
    /// Discovery, scientific theory, mathematical method
    ScientificTheoryOrMathematicalMethod,
    /// Literary, dramatic, musical or artistic work
    ArtisticWork,
    /// Scheme, rule or method for performing mental act, playing game, doing business
    BusinessMethod,
    /// Program for computer
    ComputerProgram,
    /// Presentation of information
    PresentationOfInformation,
}

/// Trademark registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trademark {
    /// Trademark number
    pub trademark_number: Option<String>,
    /// Mark (word, logo, slogan, etc.)
    pub mark: String,
    /// Trademark class (1-45 Nice Classification)
    pub classes: Vec<u8>,
    /// Is distinctive
    pub is_distinctive: bool,
    /// Not descriptive
    pub not_descriptive: bool,
    /// Not deceptive
    pub not_deceptive: bool,
    /// Registered
    pub registered: bool,
}

impl Trademark {
    /// Check if registrable (s10 Trademarks Act)
    pub fn is_registrable(&self) -> bool {
        self.is_distinctive && self.not_descriptive && self.not_deceptive
    }

    /// Trademark term (10 years, renewable - s28)
    pub fn term_years(&self) -> u8 {
        10
    }

    /// Can be renewed indefinitely
    pub fn renewable(&self) -> bool {
        true
    }
}

/// Copyright protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Copyright {
    /// Work title
    pub title: String,
    /// Author(s)
    pub authors: Vec<String>,
    /// Work type
    pub work_type: CopyrightWorkType,
    /// Creation date
    pub creation_date: String,
    /// Is original
    pub is_original: bool,
    /// Is fixed in material form
    pub is_fixed: bool,
}

impl Copyright {
    /// Check if protected (s2 Copyright Act)
    pub fn is_protected(&self) -> bool {
        self.is_original && self.is_fixed
    }

    /// Copyright term (life + 50 years for literary works)
    pub fn term_description(&self) -> &'static str {
        match self.work_type {
            CopyrightWorkType::LiteraryWork
            | CopyrightWorkType::MusicalWork
            | CopyrightWorkType::ArtisticWork => "Life of author + 50 years",
            CopyrightWorkType::Cinematograph | CopyrightWorkType::SoundRecording => {
                "50 years from first publication"
            }
            CopyrightWorkType::Broadcast | CopyrightWorkType::ProgramCarryingSignal => {
                "50 years from broadcast"
            }
            CopyrightWorkType::PublishedEdition => "50 years from first publication",
        }
    }

    /// Registration not required (automatic protection)
    pub fn registration_required(&self) -> bool {
        false
    }
}

/// Copyright work types (s2)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CopyrightWorkType {
    /// Literary work (books, articles, software)
    LiteraryWork,
    /// Musical work
    MusicalWork,
    /// Artistic work (paintings, sculptures, photographs)
    ArtisticWork,
    /// Cinematograph film
    Cinematograph,
    /// Sound recording
    SoundRecording,
    /// Broadcast
    Broadcast,
    /// Programme-carrying signal
    ProgramCarryingSignal,
    /// Published edition
    PublishedEdition,
}

/// Design registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Design {
    /// Design number
    pub design_number: Option<String>,
    /// Design type
    pub design_type: DesignType,
    /// Is new
    pub is_new: bool,
    /// Is original
    pub is_original: bool,
    /// Registered
    pub registered: bool,
}

impl Design {
    /// Check if registrable (s14 Designs Act)
    pub fn is_registrable(&self) -> bool {
        self.is_new && self.is_original
    }

    /// Design term
    pub fn term_years(&self) -> u8 {
        match self.design_type {
            DesignType::Aesthetic => 15, // Initial 10 years + 5 year renewal
            DesignType::Functional => 10,
        }
    }
}

/// Design types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DesignType {
    /// Aesthetic design (appearance)
    Aesthetic,
    /// Functional design (how it works)
    Functional,
}

/// Copyright exceptions (fair dealing - s12)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FairDealingPurpose {
    /// Research or private study
    ResearchOrPrivateStudy,
    /// Personal or private use
    PersonalOrPrivateUse,
    /// Criticism or review
    CriticismOrReview,
    /// Reporting current events
    ReportingCurrentEvents,
    /// Judicial proceedings
    JudicialProceedings,
}

impl FairDealingPurpose {
    /// Check if use is fair dealing
    pub fn is_permitted(&self) -> bool {
        true // All enumerated purposes are permitted under s12
    }
}

/// IP enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpEnforcement {
    /// IP type infringed
    pub ip_type: IntellectualPropertyType,
    /// Is counterfeit
    pub is_counterfeit: bool,
    /// Commercial scale infringement
    pub commercial_scale: bool,
}

impl IpEnforcement {
    /// Remedies available (s16 Copyright Act, s34 Trademarks Act)
    pub fn available_remedies(&self) -> Vec<&'static str> {
        vec![
            "Interdict (injunction)",
            "Delivery up of infringing goods",
            "Damages or reasonable royalty",
            "Anton Piller order (search and seizure)",
            "Criminal prosecution (if commercial)",
        ]
    }

    /// Criminal sanctions available (Counterfeit Goods Act)
    pub fn criminal_sanctions_available(&self) -> bool {
        self.is_counterfeit || self.commercial_scale
    }
}

/// IP errors
#[derive(Debug, Error)]
pub enum IpError {
    /// Not patentable
    #[error("Invention not patentable: {reason}")]
    NotPatentable { reason: String },

    /// Trademark not distinctive
    #[error("Trademark not distinctive or is descriptive")]
    TrademarkNotDistinctive,

    /// Copyright not original
    #[error("Work lacks originality or is not fixed in material form")]
    CopyrightNotOriginal,

    /// Design not new
    #[error("Design not new or not original")]
    DesignNotNew,

    /// Infringement
    #[error("IP infringement ({ip_type}): {description}")]
    Infringement {
        ip_type: String,
        description: String,
    },

    /// Counterfeit goods
    #[error("Counterfeit goods (Counterfeit Goods Act): {description}")]
    CounterfeitGoods { description: String },

    /// Renewal fee not paid
    #[error("Renewal fee not paid - {ip_type} may lapse")]
    RenewalFeeNotPaid { ip_type: String },
}

/// Validate patent application
pub fn validate_patent(patent: &Patent) -> IpResult<()> {
    if !patent.is_novel {
        return Err(IpError::NotPatentable {
            reason: "Invention lacks novelty".to_string(),
        });
    }

    if !patent.inventive_step {
        return Err(IpError::NotPatentable {
            reason: "Invention lacks inventive step".to_string(),
        });
    }

    if !patent.industrially_applicable {
        return Err(IpError::NotPatentable {
            reason: "Invention not industrially applicable".to_string(),
        });
    }

    Ok(())
}

/// Validate trademark application
pub fn validate_trademark(trademark: &Trademark) -> IpResult<()> {
    if !trademark.is_distinctive || !trademark.not_descriptive {
        return Err(IpError::TrademarkNotDistinctive);
    }

    Ok(())
}

/// Get IP compliance checklist
pub fn get_ip_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Patent search conducted", "CIPC database"),
        ("Patent application filed", "s30 Patents Act"),
        ("Patent annual fees paid", "s46"),
        ("Trademark search conducted", "CIPC database"),
        ("Trademark registered", "s16 Trademarks Act"),
        ("Trademark renewed every 10 years", "s28"),
        ("Copyright notice on works", "Best practice"),
        ("Design registered (if applicable)", "s14 Designs Act"),
        ("IP assignment agreements documented", "s22 Patents Act"),
        ("Employee IP rights clarified", "s4 Copyright Act"),
        ("Licensing agreements in writing", "Best practice"),
        ("Anti-counterfeiting measures", "Counterfeit Goods Act"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_patentable() {
        let patent = Patent {
            patent_number: None,
            title: "Novel Widget".to_string(),
            inventors: vec!["John Doe".to_string()],
            is_novel: true,
            inventive_step: true,
            industrially_applicable: true,
            filing_date: "2024-01-01".to_string(),
            granted: false,
        };
        assert!(patent.is_patentable());
        assert_eq!(patent.term_years(), 20);
        assert!(validate_patent(&patent).is_ok());
    }

    #[test]
    fn test_patent_not_novel() {
        let patent = Patent {
            patent_number: None,
            title: "Existing Widget".to_string(),
            inventors: vec!["Jane Doe".to_string()],
            is_novel: false,
            inventive_step: true,
            industrially_applicable: true,
            filing_date: "2024-01-01".to_string(),
            granted: false,
        };
        assert!(!patent.is_patentable());
        assert!(validate_patent(&patent).is_err());
    }

    #[test]
    fn test_trademark_registrable() {
        let trademark = Trademark {
            trademark_number: None,
            mark: "ACME".to_string(),
            classes: vec![1, 5],
            is_distinctive: true,
            not_descriptive: true,
            not_deceptive: true,
            registered: false,
        };
        assert!(trademark.is_registrable());
        assert_eq!(trademark.term_years(), 10);
        assert!(trademark.renewable());
        assert!(validate_trademark(&trademark).is_ok());
    }

    #[test]
    fn test_trademark_not_distinctive() {
        let trademark = Trademark {
            trademark_number: None,
            mark: "FAST DELIVERY".to_string(),
            classes: vec![39],
            is_distinctive: false,
            not_descriptive: false,
            not_deceptive: true,
            registered: false,
        };
        assert!(!trademark.is_registrable());
        assert!(validate_trademark(&trademark).is_err());
    }

    #[test]
    fn test_copyright_protected() {
        let copyright = Copyright {
            title: "My Novel".to_string(),
            authors: vec!["Author Name".to_string()],
            work_type: CopyrightWorkType::LiteraryWork,
            creation_date: "2024-01-01".to_string(),
            is_original: true,
            is_fixed: true,
        };
        assert!(copyright.is_protected());
        assert!(!copyright.registration_required());
        assert_eq!(copyright.term_description(), "Life of author + 50 years");
    }

    #[test]
    fn test_copyright_not_original() {
        let copyright = Copyright {
            title: "Copy of Novel".to_string(),
            authors: vec!["Copyist".to_string()],
            work_type: CopyrightWorkType::LiteraryWork,
            creation_date: "2024-01-01".to_string(),
            is_original: false,
            is_fixed: true,
        };
        assert!(!copyright.is_protected());
    }

    #[test]
    fn test_design_registrable() {
        let design = Design {
            design_number: None,
            design_type: DesignType::Aesthetic,
            is_new: true,
            is_original: true,
            registered: false,
        };
        assert!(design.is_registrable());
        assert_eq!(design.term_years(), 15);
    }

    #[test]
    fn test_fair_dealing() {
        assert!(FairDealingPurpose::ResearchOrPrivateStudy.is_permitted());
        assert!(FairDealingPurpose::CriticismOrReview.is_permitted());
    }

    #[test]
    fn test_ip_enforcement() {
        let enforcement = IpEnforcement {
            ip_type: IntellectualPropertyType::Trademark,
            is_counterfeit: true,
            commercial_scale: true,
        };
        assert!(enforcement.criminal_sanctions_available());
        assert!(!enforcement.available_remedies().is_empty());
    }

    #[test]
    fn test_ip_checklist() {
        let checklist = get_ip_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
