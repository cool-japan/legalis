//! Federal Constitution of Malaysia (1957)
//!
//! The supreme law of Malaysia, establishing the framework of government,
//! fundamental liberties, and the relationship between federal and state powers.
//!
//! # Key Features
//!
//! - **Part II**: Fundamental Liberties (Articles 5-13)
//! - **Part III**: Citizenship
//! - **Part IV**: Federation (13 states + 3 federal territories)
//! - **Part VI**: Relations between Federation and States
//! - **Part VIII**: Elections
//! - **Part IX**: The Judiciary
//! - **Schedule 9**: Legislative Lists (Federal, State, Concurrent)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Constitutional error types.
#[derive(Debug, Error)]
pub enum ConstitutionalError {
    /// Violation of fundamental liberties.
    #[error("Fundamental liberty violated: Article {article}")]
    FundamentalLibertyViolation { article: u8 },

    /// Ultra vires (beyond constitutional powers).
    #[error("Ultra vires: {description}")]
    UltraVires { description: String },

    /// Federal-state conflict.
    #[error("Federal-state conflict: {description}")]
    FederalStateConflict { description: String },

    /// Invalid constitutional amendment.
    #[error("Invalid constitutional amendment: {reason}")]
    InvalidAmendment { reason: String },
}

/// Result type for constitutional operations.
pub type Result<T> = std::result::Result<T, ConstitutionalError>;

/// Fundamental liberties under Part II of the Federal Constitution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FundamentalLiberty {
    /// Article 5: Liberty of the person.
    LibertyOfPerson,
    /// Article 6: Prohibition of slavery and forced labour.
    ProhibitionOfSlavery,
    /// Article 7: Protection against retrospective criminal laws.
    NoRetrospectiveLaws,
    /// Article 8: Equality before the law.
    Equality,
    /// Article 9: Prohibition of banishment and freedom of movement.
    FreedomOfMovement,
    /// Article 10: Freedom of speech, assembly and association.
    FreedomOfExpression,
    /// Article 11: Freedom of religion.
    FreedomOfReligion,
    /// Article 12: Rights in respect of education.
    RightToEducation,
    /// Article 13: Rights to property.
    RightToProperty,
}

impl FundamentalLiberty {
    /// Returns the article number.
    #[must_use]
    pub fn article(&self) -> u8 {
        match self {
            Self::LibertyOfPerson => 5,
            Self::ProhibitionOfSlavery => 6,
            Self::NoRetrospectiveLaws => 7,
            Self::Equality => 8,
            Self::FreedomOfMovement => 9,
            Self::FreedomOfExpression => 10,
            Self::FreedomOfReligion => 11,
            Self::RightToEducation => 12,
            Self::RightToProperty => 13,
        }
    }

    /// Returns a description of the liberty.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::LibertyOfPerson => {
                "No person shall be deprived of life or personal liberty save in accordance with law"
            }
            Self::ProhibitionOfSlavery => "No person shall be held in slavery",
            Self::NoRetrospectiveLaws => {
                "No person shall be punished for an act or omission which was not punishable by law when it was done"
            }
            Self::Equality => {
                "All persons are equal before the law and entitled to the equal protection of the law"
            }
            Self::FreedomOfMovement => {
                "Every citizen has the right to move freely throughout the Federation"
            }
            Self::FreedomOfExpression => {
                "Every citizen has the right to freedom of speech and expression"
            }
            Self::FreedomOfReligion => {
                "Every person has the right to profess and practice his religion"
            }
            Self::RightToEducation => {
                "There shall be no discrimination against any citizen on grounds of religion, race, descent or place of birth"
            }
            Self::RightToProperty => {
                "No person shall be deprived of property save in accordance with law"
            }
        }
    }
}

/// Legislative list under Schedule 9 (division of powers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegislativeList {
    /// Federal List (List I) - Exclusive federal jurisdiction.
    Federal,
    /// State List (List II) - Exclusive state jurisdiction.
    State,
    /// Concurrent List (List III) - Both federal and state jurisdiction.
    Concurrent,
}

/// Matters under each legislative list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegislativeMatter {
    /// The legislative list.
    pub list: LegislativeList,
    /// Matter description.
    pub matter: String,
    /// List item number.
    pub item_number: Option<u8>,
}

impl LegislativeMatter {
    /// Creates a new legislative matter.
    #[must_use]
    pub fn new(list: LegislativeList, matter: impl Into<String>) -> Self {
        Self {
            list,
            matter: matter.into(),
            item_number: None,
        }
    }

    /// Sets the item number.
    #[must_use]
    pub fn with_item_number(mut self, item_number: u8) -> Self {
        self.item_number = Some(item_number);
        self
    }
}

/// Federal Constitution validator.
#[derive(Debug, Clone)]
pub struct ConstitutionalValidator;

impl ConstitutionalValidator {
    /// Validates if a matter falls under federal jurisdiction.
    ///
    /// # Examples of Federal List matters:
    /// - External affairs, defense, internal security
    /// - Civil and criminal law and procedure
    /// - Finance, trade, commerce and industry
    /// - Shipping, navigation, and fisheries
    /// - Communications and transport
    /// - Federal works and power
    /// - Education, medicine, and health
    #[must_use]
    pub fn is_federal_matter(matter: &str) -> bool {
        let federal_keywords = [
            "defense",
            "external affairs",
            "immigration",
            "trade",
            "commerce",
            "banking",
            "company",
            "copyright",
            "patent",
            "civil law",
            "criminal law",
            "criminal procedure",
            "procedure",
            "telecommunications",
        ];

        federal_keywords
            .iter()
            .any(|keyword| matter.to_lowercase().contains(keyword))
    }

    /// Validates if a matter falls under state jurisdiction.
    ///
    /// # Examples of State List matters:
    /// - Islamic law and personal and family law of Muslims
    /// - Land
    /// - Local government
    /// - Services of local authorities
    /// - State holidays
    /// - Malay customs
    #[must_use]
    pub fn is_state_matter(matter: &str) -> bool {
        let state_keywords = [
            "land",
            "islamic law",
            "syariah",
            "local government",
            "state holidays",
            "malay customs",
            "agriculture",
        ];

        state_keywords
            .iter()
            .any(|keyword| matter.to_lowercase().contains(keyword))
    }

    /// Checks for potential federal-state conflict.
    pub fn check_federal_state_conflict(matter: &str) -> Result<()> {
        if Self::is_federal_matter(matter) && Self::is_state_matter(matter) {
            return Err(ConstitutionalError::FederalStateConflict {
                description: format!(
                    "Matter '{}' may fall under both federal and state jurisdiction",
                    matter
                ),
            });
        }
        Ok(())
    }
}

/// Malaysian states and federal territories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    // States
    Johor,
    Kedah,
    Kelantan,
    Malacca,
    NegeriSembilan,
    Pahang,
    Penang,
    Perak,
    Perlis,
    Sabah,
    Sarawak,
    Selangor,
    Terengganu,

    // Federal Territories
    KualaLumpur,
    Labuan,
    Putrajaya,
}

impl State {
    /// Returns the Malay name of the state.
    #[must_use]
    pub fn malay_name(self) -> &'static str {
        match self {
            Self::Johor => "Johor",
            Self::Kedah => "Kedah",
            Self::Kelantan => "Kelantan",
            Self::Malacca => "Melaka",
            Self::NegeriSembilan => "Negeri Sembilan",
            Self::Pahang => "Pahang",
            Self::Penang => "Pulau Pinang",
            Self::Perak => "Perak",
            Self::Perlis => "Perlis",
            Self::Sabah => "Sabah",
            Self::Sarawak => "Sarawak",
            Self::Selangor => "Selangor",
            Self::Terengganu => "Terengganu",
            Self::KualaLumpur => "Kuala Lumpur",
            Self::Labuan => "Labuan",
            Self::Putrajaya => "Putrajaya",
        }
    }

    /// Checks if this is a federal territory.
    #[must_use]
    pub fn is_federal_territory(self) -> bool {
        matches!(self, Self::KualaLumpur | Self::Labuan | Self::Putrajaya)
    }

    /// Checks if Islamic law applies (all states except federal territories apply Islamic law for Muslims).
    #[must_use]
    pub fn has_syariah_jurisdiction(self) -> bool {
        !self.is_federal_territory()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fundamental_liberty_article() {
        assert_eq!(FundamentalLiberty::LibertyOfPerson.article(), 5);
        assert_eq!(FundamentalLiberty::Equality.article(), 8);
        assert_eq!(FundamentalLiberty::FreedomOfExpression.article(), 10);
    }

    #[test]
    fn test_is_federal_matter() {
        assert!(ConstitutionalValidator::is_federal_matter("company law"));
        assert!(ConstitutionalValidator::is_federal_matter(
            "criminal procedure"
        ));
        assert!(!ConstitutionalValidator::is_federal_matter("land matters"));
    }

    #[test]
    fn test_is_state_matter() {
        assert!(ConstitutionalValidator::is_state_matter("islamic law"));
        assert!(ConstitutionalValidator::is_state_matter("land"));
        assert!(!ConstitutionalValidator::is_state_matter("company law"));
    }

    #[test]
    fn test_state_methods() {
        assert!(State::KualaLumpur.is_federal_territory());
        assert!(!State::Selangor.is_federal_territory());
        assert!(State::Selangor.has_syariah_jurisdiction());
        assert!(!State::KualaLumpur.has_syariah_jurisdiction());
    }
}
