//! UK Adequacy Decisions for International Transfers
//!
//! Post-Brexit, the UK maintains its own list of countries with adequacy decisions,
//! separate from the EU's adequacy landscape.
//!
//! Under UK GDPR Article 45, the Secretary of State can make adequacy regulations
//! recognizing that a third country provides an adequate level of data protection.

use serde::{Deserialize, Serialize};

/// UK adequacy decisions for international data transfers
///
/// As of 2024, the UK recognizes:
/// - All EEA countries (Norway, Iceland, Liechtenstein)
/// - EU member states
/// - Countries with UK-specific adequacy decisions
///
/// Note: The UK does NOT automatically recognize all EU adequacy decisions made
/// after Brexit. The UK must make its own determinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UkAdequacyDecision {
    /// EEA countries (Norway, Iceland, Liechtenstein)
    Eea,

    /// EU member states (27 countries)
    EuMemberState,

    /// Andorra (EU adequacy retained by UK)
    Andorra,

    /// Argentina (EU adequacy retained by UK)
    Argentina,

    /// Canada (commercial organizations under PIPEDA)
    Canada,

    /// Faroe Islands (EU adequacy retained by UK)
    FaroeIslands,

    /// Guernsey (EU adequacy retained by UK)
    Guernsey,

    /// Israel (EU adequacy retained by UK)
    Israel,

    /// Isle of Man (EU adequacy retained by UK)
    IsleOfMan,

    /// Japan (EU adequacy retained by UK)
    Japan,

    /// Jersey (EU adequacy retained by UK)
    Jersey,

    /// New Zealand (EU adequacy retained by UK)
    NewZealand,

    /// Republic of Korea (South Korea) - UK-specific adequacy decision (2021)
    SouthKorea,

    /// Switzerland (EU adequacy retained by UK)
    Switzerland,

    /// Uruguay (EU adequacy retained by UK)
    Uruguay,

    /// United States (under UK-US Data Bridge, 2023)
    /// Note: Separate from EU-US Data Privacy Framework
    UnitedStates,
}

impl UkAdequacyDecision {
    /// Get the date the UK adequacy decision was made
    ///
    /// Returns (year, month, day) tuple
    pub fn decision_date(&self) -> (u32, u8, u8) {
        match self {
            // Brexit transition adequacy decisions (retained EU decisions)
            Self::Eea
            | Self::EuMemberState
            | Self::Andorra
            | Self::Argentina
            | Self::Canada
            | Self::FaroeIslands
            | Self::Guernsey
            | Self::Israel
            | Self::IsleOfMan
            | Self::Japan
            | Self::Jersey
            | Self::NewZealand
            | Self::Switzerland
            | Self::Uruguay => (2021, 1, 1), // Effective from end of Brexit transition

            // UK-specific post-Brexit decisions
            Self::SouthKorea => (2021, 12, 21),
            Self::UnitedStates => (2023, 10, 12), // UK-US Data Bridge
        }
    }

    /// Check if this adequacy decision is still valid
    ///
    /// Adequacy decisions are reviewed periodically (typically every 4 years)
    pub fn is_valid(&self) -> bool {
        // All current UK adequacy decisions remain valid as of 2024
        // The ICO and Secretary of State conduct ongoing monitoring
        true
    }

    /// Get the legal basis for the adequacy decision
    pub fn legal_basis(&self) -> &'static str {
        match self {
            Self::Eea | Self::EuMemberState => {
                "UK GDPR Article 45; EEA and EU recognized as adequate by default"
            }
            Self::UnitedStates => "UK GDPR Article 45; UK-US Data Bridge (2023)",
            Self::SouthKorea => {
                "UK GDPR Article 45; UK adequacy regulations for Republic of Korea (2021)"
            }
            _ => "UK GDPR Article 45; Retained EU adequacy decision (transitioned at Brexit)",
        }
    }

    /// Get any conditions or restrictions on the adequacy decision
    pub fn conditions(&self) -> Option<&'static str> {
        match self {
            Self::Canada => Some(
                "Limited to commercial organizations subject to PIPEDA (Personal Information \
                 Protection and Electronic Documents Act)",
            ),
            Self::UnitedStates => Some(
                "Limited to organizations certified under the UK-US Data Bridge framework. \
                 Does not cover US government access.",
            ),
            _ => None,
        }
    }
}

/// Check if a country has a UK adequacy decision
///
/// This allows transfers to that country without additional safeguards
/// under UK GDPR Article 45.
///
/// # Arguments
/// * `country` - The country name (case-insensitive)
///
/// # Returns
/// * `true` if the country has a UK adequacy decision
/// * `false` if additional safeguards are required (IDTA, SCCs, etc.)
///
/// # Example
/// ```
/// use legalis_uk::data_protection::is_adequate_country_uk;
///
/// assert!(is_adequate_country_uk("France")); // EU member
/// assert!(is_adequate_country_uk("Japan")); // Retained EU adequacy
/// assert!(is_adequate_country_uk("South Korea")); // UK-specific adequacy
/// assert!(!is_adequate_country_uk("Brazil")); // No adequacy decision
/// assert!(!is_adequate_country_uk("India")); // No adequacy decision
/// ```
pub fn is_adequate_country_uk(country: &str) -> bool {
    let country_lower = country.to_lowercase();

    // EEA countries
    if matches!(
        country_lower.as_str(),
        "norway" | "iceland" | "liechtenstein"
    ) {
        return true;
    }

    // EU member states (27 as of 2024)
    let eu_members = [
        "austria",
        "belgium",
        "bulgaria",
        "croatia",
        "cyprus",
        "czech republic",
        "czechia",
        "denmark",
        "estonia",
        "finland",
        "france",
        "germany",
        "greece",
        "hungary",
        "ireland",
        "italy",
        "latvia",
        "lithuania",
        "luxembourg",
        "malta",
        "netherlands",
        "poland",
        "portugal",
        "romania",
        "slovakia",
        "slovenia",
        "spain",
        "sweden",
    ];

    if eu_members.contains(&country_lower.as_str()) {
        return true;
    }

    // Countries with specific adequacy decisions
    matches!(
        country_lower.as_str(),
        "andorra"
            | "argentina"
            | "canada"
            | "faroe islands"
            | "guernsey"
            | "israel"
            | "isle of man"
            | "japan"
            | "jersey"
            | "new zealand"
            | "south korea"
            | "korea"
            | "republic of korea"
            | "switzerland"
            | "uruguay"
            | "united states"
            | "usa"
            | "us"
    )
}

/// Alternative transfer mechanisms when adequacy is not available
///
/// UK GDPR Articles 46-49 provide alternative mechanisms for international
/// transfers when the destination country lacks an adequacy decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferMechanism {
    /// UK adequacy decision (Article 45)
    AdequacyDecision(UkAdequacyDecision),

    /// UK International Data Transfer Agreement (IDTA)
    /// Post-Brexit UK replacement for EU Standard Contractual Clauses
    UkIdta,

    /// EU Standard Contractual Clauses with UK Addendum
    /// Allows use of EU SCCs for UK transfers
    EuSccsWithAddendum,

    /// Binding Corporate Rules (Article 47)
    BindingCorporateRules { approval_reference: String },

    /// Approved Code of Conduct (Article 40)
    CodeOfConduct { code_reference: String },

    /// Approved Certification Mechanism (Article 42)
    CertificationMechanism { certificate_reference: String },

    /// Derogation for specific situations (Article 49)
    /// Should only be used for occasional, non-repetitive transfers
    Derogation(Article49Derogation),
}

/// Article 49 derogations for specific situations
///
/// These should ONLY be used for occasional, non-repetitive transfers.
/// They are NOT suitable for regular, systematic transfers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Article49Derogation {
    /// Explicit consent of the data subject
    /// (must be informed of risks of transfer to non-adequate country)
    ExplicitConsent,

    /// Necessary for performance of a contract
    ContractPerformance,

    /// Necessary for conclusion of a contract in the interest of data subject
    ContractConclusion,

    /// Necessary for important reasons of public interest
    PublicInterest,

    /// Necessary for establishment, exercise or defense of legal claims
    LegalClaims,

    /// Necessary to protect vital interests of data subject
    VitalInterests,

    /// Transfer from public register
    PublicRegister,

    /// Compelling legitimate interests (very limited use)
    /// Only for non-repetitive transfers, limited number of data subjects
    CompellingLegitimateInterests,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adequacy_eu_members() {
        assert!(is_adequate_country_uk("France"));
        assert!(is_adequate_country_uk("Germany"));
        assert!(is_adequate_country_uk("Spain"));
        assert!(is_adequate_country_uk("Poland"));
    }

    #[test]
    fn test_adequacy_eea() {
        assert!(is_adequate_country_uk("Norway"));
        assert!(is_adequate_country_uk("Iceland"));
        assert!(is_adequate_country_uk("Liechtenstein"));
    }

    #[test]
    fn test_adequacy_retained_eu_decisions() {
        assert!(is_adequate_country_uk("Japan"));
        assert!(is_adequate_country_uk("New Zealand"));
        assert!(is_adequate_country_uk("Switzerland"));
        assert!(is_adequate_country_uk("Canada"));
        assert!(is_adequate_country_uk("Israel"));
    }

    #[test]
    fn test_adequacy_uk_specific() {
        assert!(is_adequate_country_uk("South Korea"));
        assert!(is_adequate_country_uk("United States"));
    }

    #[test]
    fn test_adequacy_no_decision() {
        assert!(!is_adequate_country_uk("Brazil"));
        assert!(!is_adequate_country_uk("India"));
        assert!(!is_adequate_country_uk("China"));
        assert!(!is_adequate_country_uk("Russia"));
        assert!(!is_adequate_country_uk("Australia")); // No UK adequacy yet
    }

    #[test]
    fn test_adequacy_case_insensitive() {
        assert!(is_adequate_country_uk("FRANCE"));
        assert!(is_adequate_country_uk("france"));
        assert!(is_adequate_country_uk("FrAnCe"));
    }

    #[test]
    fn test_uk_adequacy_decision_properties() {
        let south_korea = UkAdequacyDecision::SouthKorea;
        assert_eq!(south_korea.decision_date(), (2021, 12, 21));
        assert!(south_korea.is_valid());
        assert!(
            south_korea
                .legal_basis()
                .contains("Republic of Korea (2021)")
        );
        assert!(south_korea.conditions().is_none());

        let us = UkAdequacyDecision::UnitedStates;
        assert_eq!(us.decision_date(), (2023, 10, 12));
        assert!(us.is_valid());
        assert!(us.legal_basis().contains("UK-US Data Bridge"));
        assert!(us.conditions().is_some());
        assert!(us.conditions().unwrap().contains("certified under"));
    }
}
