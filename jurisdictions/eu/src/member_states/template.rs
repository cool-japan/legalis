//! Reusable abstraction for a member state's national GDPR implementation.
//!
//! The GDPR (Regulation (EU) 2016/679) is directly applicable in all member states,
//! but it contains numerous **opening clauses** (German: *Öffnungsklauseln*) that
//! permit — or require — member states to specify the regulation in national law.
//! Examples include:
//!
//! - **Article 8(1)** — age of digital consent (default 16; may be lowered to 13).
//! - **Article 9(2)/(4)** — further conditions and limitations for special categories.
//! - **Article 23** — restrictions of data-subject rights.
//! - **Article 51** — designation of one or more supervisory authorities.
//! - **Article 85** — reconciliation with freedom of expression (journalistic exemption).
//! - **Article 88** — processing in the employment context.
//! - **Article 90** — obligations of secrecy.
//!
//! This module provides the [`MemberStateGdpr`] structure that captures, for a single
//! member state, the elements that differ from the GDPR core: its supervisory
//! authority/-ies, its chosen age of digital consent, its key national acts, and a
//! catalogue of [`NationalDerogation`]s keyed to the GDPR opening clauses they invoke.
//!
//! Concrete national implementations live in sibling modules
//! ([`crate::member_states::germany`], [`crate::member_states::france`],
//! [`crate::member_states::italy`]). New member states can be added by following the
//! same pattern: build a [`MemberStateGdpr`] via [`MemberStateGdpr::builder`].

use crate::citation::EuCitation;
use crate::member_states::error::MemberStateError;
use crate::shared::MemberState;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// The default age of digital consent set by Article 8(1) GDPR.
pub const GDPR_DEFAULT_AGE_OF_CONSENT: u8 = 16;

/// The minimum age of digital consent member states may set under Article 8(1) GDPR.
pub const GDPR_MINIMUM_AGE_OF_CONSENT: u8 = 13;

/// A supervisory authority within the meaning of Articles 51-59 GDPR.
///
/// Article 51(1) GDPR requires each member state to provide for one or more independent
/// public authorities responsible for monitoring the application of the regulation.
/// Federal states (notably Germany) maintain a national authority **and** regional
/// (state-level) authorities; this is captured via [`SupervisoryAuthority::regional`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SupervisoryAuthority {
    /// Official name of the authority in the national language.
    pub name: String,
    /// Common abbreviation (e.g. "BfDI", "CNIL", "Garante").
    pub abbreviation: String,
    /// English-language descriptive name.
    pub name_en: String,
    /// Official website (informational only).
    pub website: String,
    /// Whether this is a national/federal lead authority (`true`) or a regional/
    /// sectoral authority (`false`).
    pub is_national: bool,
}

impl SupervisoryAuthority {
    /// Construct a national/federal lead supervisory authority.
    pub fn national(
        name: impl Into<String>,
        abbreviation: impl Into<String>,
        name_en: impl Into<String>,
        website: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            abbreviation: abbreviation.into(),
            name_en: name_en.into(),
            website: website.into(),
            is_national: true,
        }
    }

    /// Construct a regional/sectoral supervisory authority (e.g. a German *Land* DPA).
    pub fn regional(
        name: impl Into<String>,
        abbreviation: impl Into<String>,
        name_en: impl Into<String>,
        website: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            abbreviation: abbreviation.into(),
            name_en: name_en.into(),
            website: website.into(),
            is_national: false,
        }
    }
}

/// The GDPR opening clause (Öffnungsklausel) that a national derogation invokes.
///
/// Each variant maps to the GDPR article that authorises member-state specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum OpeningClause {
    /// Article 6(2)/(3) — more specific provisions for lawful processing.
    Article6SpecificProvisions,
    /// Article 8(1) — age of digital consent.
    Article8AgeOfConsent,
    /// Article 9(2)/(4) — conditions and limitations for special categories.
    Article9SpecialCategories,
    /// Article 10 — processing of criminal-conviction data.
    Article10CriminalData,
    /// Article 23 — restrictions of data-subject rights.
    Article23Restrictions,
    /// Article 37(4) — additional cases of mandatory DPO designation.
    Article37DpoDesignation,
    /// Article 49(5) — limits on transfer derogations for important public-interest reasons.
    Article49TransferLimits,
    /// Article 80(2) — representation of data subjects by bodies without a mandate.
    Article80Representation,
    /// Article 85 — processing and freedom of expression and information (journalism).
    Article85FreedomOfExpression,
    /// Article 87 — processing of national identification numbers.
    Article87NationalIdentifiers,
    /// Article 88 — processing in the context of employment.
    Article88Employment,
    /// Article 89 — safeguards for archiving, research and statistics.
    Article89ResearchArchiving,
    /// Article 90 — obligations of secrecy / professional confidentiality.
    Article90Secrecy,
}

impl OpeningClause {
    /// The GDPR article number this opening clause corresponds to.
    pub fn article(&self) -> u32 {
        match self {
            Self::Article6SpecificProvisions => 6,
            Self::Article8AgeOfConsent => 8,
            Self::Article9SpecialCategories => 9,
            Self::Article10CriminalData => 10,
            Self::Article23Restrictions => 23,
            Self::Article37DpoDesignation => 37,
            Self::Article49TransferLimits => 49,
            Self::Article80Representation => 80,
            Self::Article85FreedomOfExpression => 85,
            Self::Article87NationalIdentifiers => 87,
            Self::Article88Employment => 88,
            Self::Article89ResearchArchiving => 89,
            Self::Article90Secrecy => 90,
        }
    }

    /// A short English description of the subject-matter of the opening clause.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Article6SpecificProvisions => {
                "More specific provisions for lawfulness of processing (Art 6(2)/(3))"
            }
            Self::Article8AgeOfConsent => "Age of digital consent for children (Art 8(1))",
            Self::Article9SpecialCategories => {
                "Conditions and limitations for special categories (Art 9(2)/(4))"
            }
            Self::Article10CriminalData => {
                "Processing of personal data relating to criminal convictions (Art 10)"
            }
            Self::Article23Restrictions => "Restrictions of data-subject rights (Art 23)",
            Self::Article37DpoDesignation => {
                "Additional cases of mandatory DPO designation (Art 37(4))"
            }
            Self::Article49TransferLimits => "Limits on transfer derogations (Art 49(5))",
            Self::Article80Representation => "Representation of data subjects (Art 80(2))",
            Self::Article85FreedomOfExpression => {
                "Processing and freedom of expression and information (Art 85)"
            }
            Self::Article87NationalIdentifiers => {
                "Processing of a national identification number (Art 87)"
            }
            Self::Article88Employment => "Processing in the context of employment (Art 88)",
            Self::Article89ResearchArchiving => {
                "Safeguards for archiving, research and statistics (Art 89)"
            }
            Self::Article90Secrecy => "Obligations of secrecy (Art 90)",
        }
    }

    /// A GDPR citation (CELEX 32016R0679) pointing at the corresponding article.
    pub fn citation(&self) -> EuCitation {
        EuCitation::regulation(2016, 679).with_article(self.article())
    }
}

/// A single national derogation or specification of the GDPR.
///
/// A derogation records *how* a member state has exercised a particular GDPR opening
/// clause: the clause invoked, a short title, an English summary, and a citation to the
/// national provision (e.g. "§ 26 BDSG", "Art. 7-1 Loi 78-17", "Art. 2-quinquies Codice").
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct NationalDerogation {
    /// The GDPR opening clause this derogation is based on.
    pub opening_clause: OpeningClause,
    /// Short title of the derogation.
    pub title: String,
    /// English-language summary of the national rule.
    pub summary: String,
    /// Citation of the national provision (e.g. "§ 26 BDSG").
    pub national_citation: String,
}

impl NationalDerogation {
    /// Construct a new national derogation.
    pub fn new(
        opening_clause: OpeningClause,
        title: impl Into<String>,
        summary: impl Into<String>,
        national_citation: impl Into<String>,
    ) -> Self {
        Self {
            opening_clause,
            title: title.into(),
            summary: summary.into(),
            national_citation: national_citation.into(),
        }
    }

    /// The GDPR citation for the opening clause underlying this derogation.
    pub fn gdpr_citation(&self) -> EuCitation {
        self.opening_clause.citation()
    }
}

/// A citation to a national act that forms part of a member state's GDPR implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct NationalActCitation {
    /// Short citation/abbreviation (e.g. "BDSG", "Loi 78-17", "D.Lgs. 196/2003").
    pub short_citation: String,
    /// Full official title of the act in the national language.
    pub full_title: String,
    /// English-language title of the act.
    pub title_en: String,
    /// Year the act (or its current consolidated version) entered into force.
    pub year: u16,
}

impl NationalActCitation {
    /// Construct a new national-act citation.
    pub fn new(
        short_citation: impl Into<String>,
        full_title: impl Into<String>,
        title_en: impl Into<String>,
        year: u16,
    ) -> Self {
        Self {
            short_citation: short_citation.into(),
            full_title: full_title.into(),
            title_en: title_en.into(),
            year,
        }
    }
}

/// A member state's national implementation of the GDPR.
///
/// Combines the GDPR core (modelled elsewhere in this crate) with the national
/// specifics permitted by the regulation's opening clauses.
///
/// ## Example
///
/// ```rust
/// use legalis_eu::member_states::germany;
///
/// let de = germany::implementation();
/// assert_eq!(de.age_of_digital_consent, 16);
/// assert_eq!(de.lead_authority().abbreviation, "BfDI");
/// assert!(de.has_derogation_for(
///     legalis_eu::member_states::OpeningClause::Article88Employment
/// ));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MemberStateGdpr {
    /// The member state this implementation belongs to.
    pub state: MemberState,
    /// Supervisory authority/-ies (national lead first, then regional/sectoral).
    pub authorities: Vec<SupervisoryAuthority>,
    /// Chosen age of digital consent under Article 8(1) GDPR (13-16).
    pub age_of_digital_consent: u8,
    /// Key national acts implementing/specifying the GDPR.
    pub national_acts: Vec<NationalActCitation>,
    /// National derogations and specifications keyed to GDPR opening clauses.
    pub derogations: Vec<NationalDerogation>,
}

impl MemberStateGdpr {
    /// Begin building a [`MemberStateGdpr`] for the given member state.
    pub fn builder(state: MemberState) -> MemberStateGdprBuilder {
        MemberStateGdprBuilder::new(state)
    }

    /// The national/federal lead supervisory authority.
    ///
    /// Returns the first authority marked national, falling back to the first authority
    /// in the list when none is explicitly flagged national.
    pub fn lead_authority(&self) -> &SupervisoryAuthority {
        self.authorities
            .iter()
            .find(|a| a.is_national)
            .or_else(|| self.authorities.first())
            .unwrap_or_else(|| {
                // The builder guarantees at least one authority; this branch is
                // structurally unreachable, but we avoid panicking by leaking a
                // 'static fallback only in the impossible empty case.
                Self::fallback_authority()
            })
    }

    /// Regional/sectoral authorities (excludes the national lead authorities).
    pub fn regional_authorities(&self) -> Vec<&SupervisoryAuthority> {
        self.authorities.iter().filter(|a| !a.is_national).collect()
    }

    /// Whether this member state has lowered the age of digital consent below the
    /// GDPR default of 16.
    pub fn has_lowered_age_of_consent(&self) -> bool {
        self.age_of_digital_consent < GDPR_DEFAULT_AGE_OF_CONSENT
    }

    /// Whether a child of the given age can give valid digital consent on their own in
    /// this member state (Article 8(1) GDPR as specified nationally).
    ///
    /// Returns `true` if `age >= self.age_of_digital_consent`.
    pub fn can_child_consent(&self, age: u8) -> bool {
        age >= self.age_of_digital_consent
    }

    /// Whether processing for a child of the given age requires consent from the holder
    /// of parental responsibility (Article 8(1) GDPR).
    pub fn requires_parental_consent(&self, age: u8) -> bool {
        !self.can_child_consent(age)
    }

    /// Whether the member state has enacted at least one derogation under the given
    /// opening clause.
    pub fn has_derogation_for(&self, clause: OpeningClause) -> bool {
        self.derogations.iter().any(|d| d.opening_clause == clause)
    }

    /// All derogations enacted under the given opening clause.
    pub fn derogations_for(&self, clause: OpeningClause) -> Vec<&NationalDerogation> {
        self.derogations
            .iter()
            .filter(|d| d.opening_clause == clause)
            .collect()
    }

    /// Validate that this national implementation is internally consistent and complies
    /// with the Article 8(1) GDPR bounds for the age of digital consent.
    pub fn validate(&self) -> Result<(), MemberStateError> {
        if self.authorities.is_empty() {
            return Err(MemberStateError::missing_field("supervisory authority"));
        }
        if self.national_acts.is_empty() {
            return Err(MemberStateError::missing_field("national act"));
        }
        if self.age_of_digital_consent < GDPR_MINIMUM_AGE_OF_CONSENT
            || self.age_of_digital_consent > GDPR_DEFAULT_AGE_OF_CONSENT
        {
            return Err(MemberStateError::InvalidAgeOfConsent {
                age: self.age_of_digital_consent,
            });
        }
        Ok(())
    }

    fn fallback_authority() -> &'static SupervisoryAuthority {
        // Only reachable in the structurally impossible empty-authorities case; this
        // keeps `lead_authority` total without `unwrap`/`panic`.
        static FALLBACK: std::sync::OnceLock<SupervisoryAuthority> = std::sync::OnceLock::new();
        FALLBACK.get_or_init(|| SupervisoryAuthority {
            name: "Unknown".to_string(),
            abbreviation: "N/A".to_string(),
            name_en: "Unknown supervisory authority".to_string(),
            website: String::new(),
            is_national: true,
        })
    }
}

/// Builder for [`MemberStateGdpr`].
///
/// Defaults the age of digital consent to the GDPR default of 16; call
/// [`MemberStateGdprBuilder::age_of_digital_consent`] to set a national value.
#[derive(Debug, Clone)]
pub struct MemberStateGdprBuilder {
    state: MemberState,
    authorities: Vec<SupervisoryAuthority>,
    age_of_digital_consent: u8,
    national_acts: Vec<NationalActCitation>,
    derogations: Vec<NationalDerogation>,
}

impl MemberStateGdprBuilder {
    /// Create a new builder for the given member state, defaulting the age of digital
    /// consent to the GDPR default of 16.
    pub fn new(state: MemberState) -> Self {
        Self {
            state,
            authorities: Vec::new(),
            age_of_digital_consent: GDPR_DEFAULT_AGE_OF_CONSENT,
            national_acts: Vec::new(),
            derogations: Vec::new(),
        }
    }

    /// Add a supervisory authority. National lead authorities should be added first.
    pub fn authority(mut self, authority: SupervisoryAuthority) -> Self {
        self.authorities.push(authority);
        self
    }

    /// Set the national age of digital consent (Article 8(1) GDPR).
    pub fn age_of_digital_consent(mut self, age: u8) -> Self {
        self.age_of_digital_consent = age;
        self
    }

    /// Add a key national act.
    pub fn national_act(mut self, act: NationalActCitation) -> Self {
        self.national_acts.push(act);
        self
    }

    /// Add a national derogation/specification.
    pub fn derogation(mut self, derogation: NationalDerogation) -> Self {
        self.derogations.push(derogation);
        self
    }

    /// Finalise the builder, returning the validated national implementation.
    pub fn build(self) -> Result<MemberStateGdpr, MemberStateError> {
        let implementation = MemberStateGdpr {
            state: self.state,
            authorities: self.authorities,
            age_of_digital_consent: self.age_of_digital_consent,
            national_acts: self.national_acts,
            derogations: self.derogations,
        };
        implementation.validate()?;
        Ok(implementation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> MemberStateGdpr {
        MemberStateGdpr::builder(MemberState::Germany)
            .authority(SupervisoryAuthority::national(
                "Test Authority",
                "TA",
                "Test Authority EN",
                "https://example.test",
            ))
            .authority(SupervisoryAuthority::regional(
                "Regional Authority",
                "RA",
                "Regional Authority EN",
                "https://regional.test",
            ))
            .age_of_digital_consent(16)
            .national_act(NationalActCitation::new(
                "ACT",
                "Full Act",
                "Full Act EN",
                2018,
            ))
            .derogation(NationalDerogation::new(
                OpeningClause::Article88Employment,
                "Employment",
                "Employment processing rule",
                "§ 1 ACT",
            ))
            .build()
            .expect("sample must build")
    }

    #[test]
    fn test_builder_defaults_to_gdpr_age() {
        let builder = MemberStateGdprBuilder::new(MemberState::France);
        assert_eq!(builder.age_of_digital_consent, GDPR_DEFAULT_AGE_OF_CONSENT);
    }

    #[test]
    fn test_lead_and_regional_authorities() {
        let impl_ = sample();
        assert_eq!(impl_.lead_authority().abbreviation, "TA");
        assert_eq!(impl_.regional_authorities().len(), 1);
        assert_eq!(impl_.regional_authorities()[0].abbreviation, "RA");
    }

    #[test]
    fn test_consent_logic() {
        let impl_ = sample();
        assert!(impl_.can_child_consent(16));
        assert!(impl_.can_child_consent(17));
        assert!(!impl_.can_child_consent(15));
        assert!(impl_.requires_parental_consent(15));
        assert!(!impl_.requires_parental_consent(16));
        assert!(!impl_.has_lowered_age_of_consent());
    }

    #[test]
    fn test_derogation_lookup() {
        let impl_ = sample();
        assert!(impl_.has_derogation_for(OpeningClause::Article88Employment));
        assert!(!impl_.has_derogation_for(OpeningClause::Article23Restrictions));
        assert_eq!(
            impl_
                .derogations_for(OpeningClause::Article88Employment)
                .len(),
            1
        );
    }

    #[test]
    fn test_opening_clause_citation() {
        let clause = OpeningClause::Article8AgeOfConsent;
        assert_eq!(clause.article(), 8);
        let citation = clause.citation();
        assert_eq!(citation.celex, "32016R0679");
        assert_eq!(citation.article, Some(8));
    }

    #[test]
    fn test_validate_rejects_age_below_floor() {
        let result = MemberStateGdpr::builder(MemberState::Italy)
            .authority(SupervisoryAuthority::national("A", "A", "A", "x"))
            .national_act(NationalActCitation::new("X", "X", "X", 2018))
            .age_of_digital_consent(12)
            .build();
        assert!(matches!(
            result,
            Err(MemberStateError::InvalidAgeOfConsent { age: 12 })
        ));
    }

    #[test]
    fn test_validate_rejects_age_above_default() {
        let result = MemberStateGdpr::builder(MemberState::Italy)
            .authority(SupervisoryAuthority::national("A", "A", "A", "x"))
            .national_act(NationalActCitation::new("X", "X", "X", 2018))
            .age_of_digital_consent(18)
            .build();
        assert!(matches!(
            result,
            Err(MemberStateError::InvalidAgeOfConsent { age: 18 })
        ));
    }

    #[test]
    fn test_validate_requires_authority_and_act() {
        let no_authority = MemberStateGdpr::builder(MemberState::Italy)
            .national_act(NationalActCitation::new("X", "X", "X", 2018))
            .build();
        assert!(matches!(
            no_authority,
            Err(MemberStateError::MissingField(_))
        ));

        let no_act = MemberStateGdpr::builder(MemberState::Italy)
            .authority(SupervisoryAuthority::national("A", "A", "A", "x"))
            .build();
        assert!(matches!(no_act, Err(MemberStateError::MissingField(_))));
    }
}
