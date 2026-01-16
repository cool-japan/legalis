//! Canada Interoperability Module
//!
//! Cross-jurisdiction interoperability for Canadian law.

#![allow(missing_docs)]

use legalis_core::Statute;
use serde::{Deserialize, Serialize};

use crate::common::Province;

// ============================================================================
// Cross-Jurisdiction Interoperability
// ============================================================================

/// Canadian jurisdiction interoperability handler
pub struct CanadianInterop;

impl CanadianInterop {
    /// Check if statute applies in province
    pub fn applies_in_province(statute: &Statute, province: &Province) -> bool {
        if let Some(jurisdiction) = &statute.jurisdiction {
            // Federal statutes apply everywhere
            if jurisdiction.contains("FED") {
                return true;
            }

            // Check provincial match
            if jurisdiction.contains(province.abbreviation()) {
                return true;
            }

            // Check territorial match
            match province {
                Province::Yukon | Province::NorthwestTerritories | Province::Nunavut => {
                    // Territories may have federal law apply differently
                    jurisdiction.contains("FED")
                }
                _ => false,
            }
        } else {
            // No jurisdiction specified - assume universal
            true
        }
    }

    /// Determine applicable law for inter-provincial matter
    pub fn determine_applicable_law(facts: &InterProvincialFacts) -> ApplicableLawResult {
        // Default choice of law rules
        let governing_law = match &facts.legal_area {
            LegalAreaType::Contract => Self::contract_choice_of_law(facts),
            LegalAreaType::Tort => Self::tort_choice_of_law(facts),
            LegalAreaType::Property => Self::property_choice_of_law(facts),
            LegalAreaType::Family => Self::family_choice_of_law(facts),
            LegalAreaType::Corporate => Self::corporate_choice_of_law(facts),
            LegalAreaType::Criminal => {
                // Criminal law is federal
                GoverningLaw::Federal
            }
        };

        let reasoning = Self::build_reasoning(facts, &governing_law);

        ApplicableLawResult {
            governing_law,
            secondary_laws: Self::determine_secondary_laws(facts),
            reasoning,
        }
    }

    /// Contract choice of law
    fn contract_choice_of_law(facts: &InterProvincialFacts) -> GoverningLaw {
        // Check for express choice
        if let Some(chosen) = &facts.express_choice_of_law {
            return GoverningLaw::Provincial(*chosen);
        }

        // Closest and most real connection test
        if let Some(performance) = &facts.place_of_performance {
            return GoverningLaw::Provincial(*performance);
        }

        // Place of contracting as fallback
        if let Some(contracting) = &facts.place_of_contracting {
            return GoverningLaw::Provincial(*contracting);
        }

        // Default to first connected province
        facts
            .connected_provinces
            .first()
            .map(|p| GoverningLaw::Provincial(*p))
            .unwrap_or(GoverningLaw::Federal)
    }

    /// Tort choice of law
    fn tort_choice_of_law(facts: &InterProvincialFacts) -> GoverningLaw {
        // Lex loci delicti - law of place of wrong
        if let Some(tort_place) = &facts.place_of_tort {
            return GoverningLaw::Provincial(*tort_place);
        }

        // Where harm occurred
        if let Some(harm) = &facts.place_of_harm {
            return GoverningLaw::Provincial(*harm);
        }

        // Most significant relationship
        facts
            .connected_provinces
            .first()
            .map(|p| GoverningLaw::Provincial(*p))
            .unwrap_or(GoverningLaw::Federal)
    }

    /// Property choice of law
    fn property_choice_of_law(facts: &InterProvincialFacts) -> GoverningLaw {
        // Lex situs - law of location of property
        if let Some(property) = &facts.place_of_property {
            return GoverningLaw::Provincial(*property);
        }

        facts
            .connected_provinces
            .first()
            .map(|p| GoverningLaw::Provincial(*p))
            .unwrap_or(GoverningLaw::Federal)
    }

    /// Family law choice of law
    fn family_choice_of_law(facts: &InterProvincialFacts) -> GoverningLaw {
        // Divorce Act applies federally
        if facts.involves_divorce {
            return GoverningLaw::Federal;
        }

        // Property division - provincial law of habitual residence
        if let Some(residence) = &facts.habitual_residence {
            return GoverningLaw::Provincial(*residence);
        }

        facts
            .connected_provinces
            .first()
            .map(|p| GoverningLaw::Provincial(*p))
            .unwrap_or(GoverningLaw::Federal)
    }

    /// Corporate choice of law
    fn corporate_choice_of_law(facts: &InterProvincialFacts) -> GoverningLaw {
        // Internal affairs governed by place of incorporation
        if let Some(incorporation) = &facts.place_of_incorporation {
            if *incorporation == Province::Federal {
                return GoverningLaw::Federal;
            }
            return GoverningLaw::Provincial(*incorporation);
        }

        // Default to federal for cross-provincial corporations
        if facts.connected_provinces.len() > 1 {
            return GoverningLaw::Federal;
        }

        facts
            .connected_provinces
            .first()
            .map(|p| GoverningLaw::Provincial(*p))
            .unwrap_or(GoverningLaw::Federal)
    }

    /// Determine secondary applicable laws
    fn determine_secondary_laws(facts: &InterProvincialFacts) -> Vec<GoverningLaw> {
        let mut secondary = Vec::new();

        // Add Quebec civil law considerations if Quebec involved
        if facts.connected_provinces.contains(&Province::Quebec) {
            secondary.push(GoverningLaw::CivilLaw);
        }

        // Add federal law for certain matters
        match &facts.legal_area {
            LegalAreaType::Family => {
                secondary.push(GoverningLaw::Federal); // Divorce Act
            }
            LegalAreaType::Contract => {
                // Bills of Exchange, Interest Act
                if facts.involves_negotiable_instruments || facts.involves_interest {
                    secondary.push(GoverningLaw::Federal);
                }
            }
            _ => {}
        }

        secondary
    }

    /// Build reasoning
    fn build_reasoning(facts: &InterProvincialFacts, law: &GoverningLaw) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Inter-provincial conflict of laws - {:?}",
            facts.legal_area
        ));
        parts.push(format!("Governing law: {:?}", law));

        match &facts.legal_area {
            LegalAreaType::Contract => {
                if facts.express_choice_of_law.is_some() {
                    parts.push("Express choice of law clause applies".to_string());
                } else {
                    parts.push(
                        "Closest and most real connection test (Vita Food Products)".to_string(),
                    );
                }
            }
            LegalAreaType::Tort => {
                parts.push("Lex loci delicti - law of place of wrong".to_string());
            }
            LegalAreaType::Property => {
                parts.push("Lex situs - law of location of property".to_string());
            }
            LegalAreaType::Family => {
                if facts.involves_divorce {
                    parts.push("Divorce Act (federal) governs divorce".to_string());
                }
            }
            LegalAreaType::Corporate => {
                parts.push("Internal affairs - law of place of incorporation".to_string());
            }
            LegalAreaType::Criminal => {
                parts.push("Criminal Code (federal) applies uniformly".to_string());
            }
        }

        parts.join(". ")
    }

    /// Check for Quebec civil law considerations
    pub fn involves_quebec_civil_law(facts: &InterProvincialFacts) -> bool {
        facts.connected_provinces.contains(&Province::Quebec)
            && !matches!(facts.legal_area, LegalAreaType::Criminal)
    }

    /// Map common law concept to Quebec civil law
    pub fn common_to_civil_law(concept: &CommonLawConcept) -> CivilLawConcept {
        match concept {
            CommonLawConcept::Consideration => CivilLawConcept::Cause,
            CommonLawConcept::EstoppelPromissory => CivilLawConcept::GoodFaithPrecontractual,
            CommonLawConcept::Negligence => CivilLawConcept::ExtracontractualFault,
            CommonLawConcept::Trust => CivilLawConcept::Fiducie,
            CommonLawConcept::Easement => CivilLawConcept::Servitude,
            CommonLawConcept::Mortgage => CivilLawConcept::Hypothec,
        }
    }
}

/// Inter-provincial facts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterProvincialFacts {
    /// Legal area
    pub legal_area: LegalAreaType,
    /// Connected provinces
    pub connected_provinces: Vec<Province>,
    /// Express choice of law
    pub express_choice_of_law: Option<Province>,
    /// Place of contracting
    pub place_of_contracting: Option<Province>,
    /// Place of performance
    pub place_of_performance: Option<Province>,
    /// Place of tort
    pub place_of_tort: Option<Province>,
    /// Place of harm
    pub place_of_harm: Option<Province>,
    /// Place of property
    pub place_of_property: Option<Province>,
    /// Habitual residence
    pub habitual_residence: Option<Province>,
    /// Place of incorporation
    pub place_of_incorporation: Option<Province>,
    /// Involves divorce
    pub involves_divorce: bool,
    /// Involves negotiable instruments
    pub involves_negotiable_instruments: bool,
    /// Involves interest
    pub involves_interest: bool,
}

impl Default for InterProvincialFacts {
    fn default() -> Self {
        Self {
            legal_area: LegalAreaType::Contract,
            connected_provinces: Vec::new(),
            express_choice_of_law: None,
            place_of_contracting: None,
            place_of_performance: None,
            place_of_tort: None,
            place_of_harm: None,
            place_of_property: None,
            habitual_residence: None,
            place_of_incorporation: None,
            involves_divorce: false,
            involves_negotiable_instruments: false,
            involves_interest: false,
        }
    }
}

/// Legal area type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalAreaType {
    /// Contract
    Contract,
    /// Tort
    Tort,
    /// Property
    Property,
    /// Family
    Family,
    /// Corporate
    Corporate,
    /// Criminal
    Criminal,
}

/// Governing law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoverningLaw {
    /// Federal law
    Federal,
    /// Provincial law
    Provincial(Province),
    /// Quebec civil law
    CivilLaw,
}

/// Applicable law result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicableLawResult {
    /// Primary governing law
    pub governing_law: GoverningLaw,
    /// Secondary applicable laws
    pub secondary_laws: Vec<GoverningLaw>,
    /// Reasoning
    pub reasoning: String,
}

/// Common law concept
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommonLawConcept {
    /// Consideration (contract formation)
    Consideration,
    /// Promissory estoppel
    EstoppelPromissory,
    /// Negligence (tort)
    Negligence,
    /// Trust
    Trust,
    /// Easement
    Easement,
    /// Mortgage
    Mortgage,
}

/// Civil law concept
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CivilLawConcept {
    /// Cause (Art. 1410 CCQ)
    Cause,
    /// Pre-contractual good faith (Art. 1375 CCQ)
    GoodFaithPrecontractual,
    /// Extracontractual fault (Art. 1457 CCQ)
    ExtracontractualFault,
    /// Fiducie (Art. 1260 CCQ)
    Fiducie,
    /// Servitude (Art. 1177 CCQ)
    Servitude,
    /// Hypothec (Art. 2660 CCQ)
    Hypothec,
}

// ============================================================================
// Province Extension for Federal
// ============================================================================

impl Province {
    /// Federal pseudo-province for CBCA corporations
    #[allow(non_upper_case_globals)]
    pub const Federal: Province = Province::Ontario; // Placeholder - TODO: proper federal type
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_applies_in_province_federal() {
        let statute = Statute::new(
            "TEST",
            "Federal Act",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_jurisdiction("CA-FED");

        assert!(CanadianInterop::applies_in_province(
            &statute,
            &Province::Ontario
        ));
        assert!(CanadianInterop::applies_in_province(
            &statute,
            &Province::Quebec
        ));
    }

    #[test]
    fn test_applies_in_province_provincial() {
        let statute = Statute::new(
            "TEST",
            "Ontario Act",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_jurisdiction("ON");

        assert!(CanadianInterop::applies_in_province(
            &statute,
            &Province::Ontario
        ));
        assert!(!CanadianInterop::applies_in_province(
            &statute,
            &Province::Quebec
        ));
    }

    #[test]
    fn test_contract_express_choice() {
        let facts = InterProvincialFacts {
            legal_area: LegalAreaType::Contract,
            connected_provinces: vec![Province::Ontario, Province::BritishColumbia],
            express_choice_of_law: Some(Province::Alberta),
            ..Default::default()
        };

        let result = CanadianInterop::determine_applicable_law(&facts);
        assert_eq!(
            result.governing_law,
            GoverningLaw::Provincial(Province::Alberta)
        );
    }

    #[test]
    fn test_tort_lex_loci() {
        let facts = InterProvincialFacts {
            legal_area: LegalAreaType::Tort,
            connected_provinces: vec![Province::Ontario, Province::Quebec],
            place_of_tort: Some(Province::Ontario),
            ..Default::default()
        };

        let result = CanadianInterop::determine_applicable_law(&facts);
        assert_eq!(
            result.governing_law,
            GoverningLaw::Provincial(Province::Ontario)
        );
    }

    #[test]
    fn test_criminal_federal() {
        let facts = InterProvincialFacts {
            legal_area: LegalAreaType::Criminal,
            connected_provinces: vec![Province::Ontario],
            ..Default::default()
        };

        let result = CanadianInterop::determine_applicable_law(&facts);
        assert_eq!(result.governing_law, GoverningLaw::Federal);
    }

    #[test]
    fn test_quebec_civil_law() {
        let facts = InterProvincialFacts {
            legal_area: LegalAreaType::Contract,
            connected_provinces: vec![Province::Quebec],
            ..Default::default()
        };

        assert!(CanadianInterop::involves_quebec_civil_law(&facts));
    }

    #[test]
    fn test_common_to_civil() {
        let civil = CanadianInterop::common_to_civil_law(&CommonLawConcept::Consideration);
        assert_eq!(civil, CivilLawConcept::Cause);
    }
}
