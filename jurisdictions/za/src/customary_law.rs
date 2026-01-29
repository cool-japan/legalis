//! South African Customary Law
//!
//! Recognition and regulation of indigenous African legal systems.
//!
//! ## Key Legislation
//!
//! - Recognition of Customary Marriages Act 120 of 1998
//! - Traditional Leadership and Governance Framework Act 41 of 2003
//! - Reform of Customary Law of Succession and Regulation of Related Matters Act 11 of 2009
//! - Constitution s211-212 (recognition of customary law)
//!
//! ## Principles
//!
//! - Customary law has equal status with common law (s211(3))
//! - Must be constitutional (s39(2))
//! - Courts must apply if applicable (s39(3))
//! - Development through legislation and judicial precedent

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for customary law operations
pub type CustomaryResult<T> = Result<T, CustomaryError>;

/// Traditional communities in South Africa
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraditionalCommunity {
    /// Zulu
    Zulu,
    /// Xhosa
    Xhosa,
    /// Pedi (Northern Sotho)
    Pedi,
    /// Tswana
    Tswana,
    /// Sotho (Southern Sotho)
    Sotho,
    /// Tsonga
    Tsonga,
    /// Swazi
    Swazi,
    /// Venda
    Venda,
    /// Ndebele
    Ndebele,
}

impl TraditionalCommunity {
    /// Get primary language
    pub fn primary_language(&self) -> &'static str {
        match self {
            Self::Zulu => "isiZulu",
            Self::Xhosa => "isiXhosa",
            Self::Pedi => "Sepedi",
            Self::Tswana => "Setswana",
            Self::Sotho => "Sesotho",
            Self::Tsonga => "Xitsonga",
            Self::Swazi => "siSwati",
            Self::Venda => "Tshivenda",
            Self::Ndebele => "isiNdebele",
        }
    }
}

/// Customary marriage (Act 120 of 1998)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomaryMarriage {
    /// Is marriage registered
    pub registered: bool,
    /// Registration number (if registered)
    pub registration_number: Option<String>,
    /// Husband
    pub husband: String,
    /// Wife/wives
    pub wives: Vec<String>,
    /// Marriage negotiated and entered according to customary law
    pub customary_law_compliant: bool,
    /// Both parties 18+ at date of marriage (or ministerial consent)
    pub age_requirement_met: bool,
}

impl CustomaryMarriage {
    /// Check if marriage is valid
    pub fn is_valid(&self) -> bool {
        self.customary_law_compliant && self.age_requirement_met && !self.wives.is_empty()
    }

    /// Check if polygynous
    pub fn is_polygynous(&self) -> bool {
        self.wives.len() > 1
    }

    /// Marriage regime (s7 - in community of property or customary proprietary system)
    pub fn default_marriage_regime(&self) -> MarriageRegime {
        // Pre-2000 marriages: customary proprietary system
        // Post-2000 marriages: community of property (s7(2))
        MarriageRegime::CommunityOfProperty
    }
}

/// Marriage regime under customary law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarriageRegime {
    /// Community of property (default post-2000)
    CommunityOfProperty,
    /// Out of community of property (antenuptial contract)
    OutOfCommunity,
    /// Customary proprietary system (pre-2000)
    CustomaryProprietarySystem,
}

/// Lobolo (bride price)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lobolo {
    /// Cattle agreed (traditional)
    pub cattle_agreed: u32,
    /// Cash equivalent (modern practice)
    pub cash_equivalent_zar: Option<i64>,
    /// Paid in full
    pub paid_in_full: bool,
}

impl Lobolo {
    /// Check if lobolo payment affects marriage validity
    pub fn required_for_validity(&self) -> bool {
        // Lobolo NOT a requirement for validity (s3(1)(b) Act 120/1998)
        false
    }
}

/// Customary succession (Act 11 of 2009)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomarySuccession {
    /// Deceased name
    pub deceased: String,
    /// Intestate (no will)
    pub intestate: bool,
    /// Surviving spouse(s)
    pub surviving_spouses: Vec<String>,
    /// Children
    pub children: Vec<String>,
}

impl CustomarySuccession {
    /// Apply customary law of succession (Act 11 of 2009)
    /// - Surviving spouse inherits
    /// - Male primogeniture abolished (unconstitutional - Bhe v Magistrate)
    /// - Daughters and sons inherit equally
    pub fn distribution_order(&self) -> Vec<&'static str> {
        vec![
            "Surviving spouse(s) - intestate share",
            "Descendants (children) - equal shares",
            "Parents",
            "Siblings and descendants of siblings",
        ]
    }

    /// Male primogeniture prohibited (Bhe v Magistrate, Khumalo 2005)
    pub fn male_primogeniture_prohibited(&self) -> bool {
        true // Unconstitutional - violates equality (s9)
    }
}

/// Traditional courts (s166(e) Constitution, Act 41 of 2003)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionalCourt {
    /// Traditional leader presiding
    pub traditional_leader: String,
    /// Community
    pub community: TraditionalCommunity,
    /// Jurisdiction limited to customary law matters
    pub jurisdiction_customary_matters: bool,
}

impl TraditionalCourt {
    /// Check if matter is within jurisdiction
    pub fn has_jurisdiction(&self, matter: &str) -> bool {
        self.jurisdiction_customary_matters && !matter.is_empty()
    }

    /// Appeal to Magistrate's Court available (s20 Framework Act)
    pub fn appeal_available(&self) -> bool {
        true
    }
}

/// Traditional leadership roles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraditionalLeaderRole {
    /// King/Queen
    King,
    /// Senior traditional leader (inkosi)
    SeniorTraditionalLeader,
    /// Headman/Headwoman (induna)
    Headman,
}

impl TraditionalLeaderRole {
    /// Recognized under Constitution s211-212
    pub fn is_constitutionally_recognized(&self) -> bool {
        true
    }
}

/// Customary law errors
#[derive(Debug, Error)]
pub enum CustomaryError {
    /// Marriage not compliant with customary law
    #[error("Marriage not compliant with customary law requirements")]
    MarriageNotCompliant,

    /// Age requirement not met
    #[error("Age requirement not met (both parties must be 18+ or have ministerial consent)")]
    AgeRequirementNotMet,

    /// Succession rule unconstitutional
    #[error("Succession rule unconstitutional: {rule}")]
    UnconstitutionalSuccessionRule { rule: String },

    /// Traditional court exceeded jurisdiction
    #[error("Traditional court exceeded jurisdiction: {description}")]
    JurisdictionExceeded { description: String },

    /// Customary law conflicts with Constitution
    #[error("Customary law provision conflicts with Constitution (s2): {provision}")]
    ConstitutionalConflict { provision: String },
}

/// Validate customary marriage
pub fn validate_customary_marriage(marriage: &CustomaryMarriage) -> CustomaryResult<()> {
    if !marriage.customary_law_compliant {
        return Err(CustomaryError::MarriageNotCompliant);
    }

    if !marriage.age_requirement_met {
        return Err(CustomaryError::AgeRequirementNotMet);
    }

    Ok(())
}

/// Validate succession rule against Constitution
pub fn validate_succession_rule(rule: &str) -> CustomaryResult<()> {
    if rule.contains("male primogeniture") || rule.contains("male heir only") {
        return Err(CustomaryError::UnconstitutionalSuccessionRule {
            rule: "Male primogeniture violates s9 (equality)".to_string(),
        });
    }
    Ok(())
}

/// Get customary law compliance checklist
pub fn get_customary_law_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Customary marriage registered", "s4 Act 120/1998"),
        ("Both parties 18+ or ministerial consent", "s3(1)(a)"),
        (
            "Marriage negotiated and entered per customary law",
            "s3(1)(b)",
        ),
        ("Polygynous marriage: existing spouse notified", "s7A"),
        ("Marriage regime determined", "s7"),
        ("Customary succession: equality applied", "Act 11/2009"),
        ("Male primogeniture not applied", "Bhe v Magistrate 2005"),
        ("Traditional court jurisdiction limited", "s20 Act 41/2003"),
        ("Appeal to Magistrate's Court available", "s20"),
        ("Customary law consistent with Constitution", "s211(3)"),
        ("Living customary law recognized", "s39(2)"),
        ("Gender equality respected", "s9 Constitution"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traditional_communities() {
        assert_eq!(TraditionalCommunity::Zulu.primary_language(), "isiZulu");
        assert_eq!(TraditionalCommunity::Xhosa.primary_language(), "isiXhosa");
    }

    #[test]
    fn test_customary_marriage_valid() {
        let marriage = CustomaryMarriage {
            registered: true,
            registration_number: Some("CM123/2024".to_string()),
            husband: "Sipho Dlamini".to_string(),
            wives: vec!["Nomsa Dlamini".to_string()],
            customary_law_compliant: true,
            age_requirement_met: true,
        };
        assert!(marriage.is_valid());
        assert!(!marriage.is_polygynous());
        assert!(validate_customary_marriage(&marriage).is_ok());
    }

    #[test]
    fn test_customary_marriage_polygynous() {
        let marriage = CustomaryMarriage {
            registered: true,
            registration_number: Some("CM456/2024".to_string()),
            husband: "Thabo Ngcobo".to_string(),
            wives: vec!["Zanele Ngcobo".to_string(), "Precious Ngcobo".to_string()],
            customary_law_compliant: true,
            age_requirement_met: true,
        };
        assert!(marriage.is_valid());
        assert!(marriage.is_polygynous());
    }

    #[test]
    fn test_customary_marriage_invalid_age() {
        let marriage = CustomaryMarriage {
            registered: false,
            registration_number: None,
            husband: "Test".to_string(),
            wives: vec!["Test Wife".to_string()],
            customary_law_compliant: true,
            age_requirement_met: false,
        };
        assert!(!marriage.is_valid());
        assert!(validate_customary_marriage(&marriage).is_err());
    }

    #[test]
    fn test_lobolo_not_required() {
        let lobolo = Lobolo {
            cattle_agreed: 10,
            cash_equivalent_zar: Some(100_000),
            paid_in_full: false,
        };
        assert!(!lobolo.required_for_validity());
    }

    #[test]
    fn test_customary_succession() {
        let succession = CustomarySuccession {
            deceased: "Test Deceased".to_string(),
            intestate: true,
            surviving_spouses: vec!["Spouse 1".to_string()],
            children: vec!["Child 1".to_string(), "Child 2".to_string()],
        };
        assert!(succession.male_primogeniture_prohibited());
        assert!(!succession.distribution_order().is_empty());
    }

    #[test]
    fn test_male_primogeniture_unconstitutional() {
        let result = validate_succession_rule("male primogeniture applies");
        assert!(result.is_err());
    }

    #[test]
    fn test_traditional_court() {
        let court = TraditionalCourt {
            traditional_leader: "Inkosi Mthembu".to_string(),
            community: TraditionalCommunity::Zulu,
            jurisdiction_customary_matters: true,
        };
        assert!(court.has_jurisdiction("customary marriage dispute"));
        assert!(court.appeal_available());
    }

    #[test]
    fn test_traditional_leader_recognition() {
        assert!(TraditionalLeaderRole::King.is_constitutionally_recognized());
        assert!(TraditionalLeaderRole::Headman.is_constitutionally_recognized());
    }

    #[test]
    fn test_customary_law_checklist() {
        let checklist = get_customary_law_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
