//! EU Member States and EEA registry
//!
//! This module provides an enumeration of all EU member states and EEA countries.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// EU27 member states + EEA countries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum MemberState {
    // EU27 Member States
    Austria,
    Belgium,
    Bulgaria,
    Croatia,
    Cyprus,
    CzechRepublic,
    Denmark,
    Estonia,
    Finland,
    France,
    Germany,
    Greece,
    Hungary,
    Ireland,
    Italy,
    Latvia,
    Lithuania,
    Luxembourg,
    Malta,
    Netherlands,
    Poland,
    Portugal,
    Romania,
    Slovakia,
    Slovenia,
    Spain,
    Sweden,

    // EEA (not EU)
    Iceland,
    Liechtenstein,
    Norway,
}

impl MemberState {
    /// Get ISO 3166-1 alpha-2 country code
    ///
    /// ## Example
    ///
    /// ```rust
    /// use legalis_eu::shared::MemberState;
    ///
    /// assert_eq!(MemberState::Germany.iso_code(), "DE");
    /// assert_eq!(MemberState::France.iso_code(), "FR");
    /// ```
    pub fn iso_code(&self) -> &'static str {
        match self {
            Self::Austria => "AT",
            Self::Belgium => "BE",
            Self::Bulgaria => "BG",
            Self::Croatia => "HR",
            Self::Cyprus => "CY",
            Self::CzechRepublic => "CZ",
            Self::Denmark => "DK",
            Self::Estonia => "EE",
            Self::Finland => "FI",
            Self::France => "FR",
            Self::Germany => "DE",
            Self::Greece => "GR",
            Self::Hungary => "HU",
            Self::Ireland => "IE",
            Self::Italy => "IT",
            Self::Latvia => "LV",
            Self::Lithuania => "LT",
            Self::Luxembourg => "LU",
            Self::Malta => "MT",
            Self::Netherlands => "NL",
            Self::Poland => "PL",
            Self::Portugal => "PT",
            Self::Romania => "RO",
            Self::Slovakia => "SK",
            Self::Slovenia => "SI",
            Self::Spain => "ES",
            Self::Sweden => "SE",
            Self::Iceland => "IS",
            Self::Liechtenstein => "LI",
            Self::Norway => "NO",
        }
    }

    /// Get primary official language code (ISO 639-1)
    ///
    /// Note: Some countries have multiple official languages.
    /// This returns the primary/most common one.
    pub fn primary_language(&self) -> &'static str {
        match self {
            Self::Austria => "de",
            Self::Belgium => "nl", // Also fr, de
            Self::Bulgaria => "bg",
            Self::Croatia => "hr",
            Self::Cyprus => "el", // Greek
            Self::CzechRepublic => "cs",
            Self::Denmark => "da",
            Self::Estonia => "et",
            Self::Finland => "fi",
            Self::France => "fr",
            Self::Germany => "de",
            Self::Greece => "el",
            Self::Hungary => "hu",
            Self::Ireland => "en", // Also ga (Irish)
            Self::Italy => "it",
            Self::Latvia => "lv",
            Self::Lithuania => "lt",
            Self::Luxembourg => "lb", // Also fr, de
            Self::Malta => "mt",      // Also en
            Self::Netherlands => "nl",
            Self::Poland => "pl",
            Self::Portugal => "pt",
            Self::Romania => "ro",
            Self::Slovakia => "sk",
            Self::Slovenia => "sl",
            Self::Spain => "es",
            Self::Sweden => "sv",
            Self::Iceland => "is",
            Self::Liechtenstein => "de",
            Self::Norway => "no",
        }
    }

    /// Check if this is an EU member state (vs EEA only)
    pub fn is_eu_member(&self) -> bool {
        !matches!(self, Self::Iceland | Self::Liechtenstein | Self::Norway)
    }

    /// Get all EU27 member states
    pub fn eu_members() -> Vec<Self> {
        vec![
            Self::Austria,
            Self::Belgium,
            Self::Bulgaria,
            Self::Croatia,
            Self::Cyprus,
            Self::CzechRepublic,
            Self::Denmark,
            Self::Estonia,
            Self::Finland,
            Self::France,
            Self::Germany,
            Self::Greece,
            Self::Hungary,
            Self::Ireland,
            Self::Italy,
            Self::Latvia,
            Self::Lithuania,
            Self::Luxembourg,
            Self::Malta,
            Self::Netherlands,
            Self::Poland,
            Self::Portugal,
            Self::Romania,
            Self::Slovakia,
            Self::Slovenia,
            Self::Spain,
            Self::Sweden,
        ]
    }

    /// Get all EEA countries (EU + Iceland, Liechtenstein, Norway)
    pub fn eea_members() -> Vec<Self> {
        let mut members = Self::eu_members();
        members.extend_from_slice(&[Self::Iceland, Self::Liechtenstein, Self::Norway]);
        members
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_codes() {
        assert_eq!(MemberState::Germany.iso_code(), "DE");
        assert_eq!(MemberState::France.iso_code(), "FR");
        assert_eq!(MemberState::Italy.iso_code(), "IT");
    }

    #[test]
    fn test_primary_languages() {
        assert_eq!(MemberState::Germany.primary_language(), "de");
        assert_eq!(MemberState::France.primary_language(), "fr");
        assert_eq!(MemberState::Ireland.primary_language(), "en");
    }

    #[test]
    fn test_eu_membership() {
        assert!(MemberState::Germany.is_eu_member());
        assert!(MemberState::France.is_eu_member());
        assert!(!MemberState::Norway.is_eu_member());
        assert!(!MemberState::Iceland.is_eu_member());
    }

    #[test]
    fn test_member_counts() {
        assert_eq!(MemberState::eu_members().len(), 27);
        assert_eq!(MemberState::eea_members().len(), 30);
    }
}
