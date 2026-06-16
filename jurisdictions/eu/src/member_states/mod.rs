//! Member-state GDPR implementations.
//!
//! The GDPR (Regulation (EU) 2016/679) is directly applicable across the Union, but it
//! deliberately leaves room for national specification through its **opening clauses**.
//! This module layers national specifics on top of the GDPR core modelled in
//! [`crate::gdpr`]:
//!
//! - [`template`] — the reusable [`MemberStateGdpr`] abstraction (supervisory authority,
//!   age of digital consent, national acts and [`NationalDerogation`]s keyed to GDPR
//!   [`OpeningClause`]s) plus its builder.
//! - [`germany`], [`france`], [`italy`] — concrete national implementations (BDSG,
//!   Loi Informatique et Libertés, Codice Privacy).
//! - [`transposition`] — directive-transposition tracking (directive → national act +
//!   date + [`TranspositionStatus`]).
//!
//! ## National-law integration
//!
//! Use [`for_state`] to obtain a member state's implementation, or
//! [`combined_consent_assessment`] / [`NationalGdprQuery`] to combine the GDPR core
//! age-of-consent rule (Article 8(1)) with the nationally-specified age.
//!
//! ```rust
//! use legalis_eu::member_states::{self, NationalGdprQuery};
//! use legalis_eu::shared::MemberState;
//!
//! // Germany keeps the GDPR default of 16; France lowered it to 15; Italy to 14.
//! let query = NationalGdprQuery::new(MemberState::France)
//!     .expect("France is implemented");
//! assert_eq!(query.age_of_digital_consent(), 15);
//! assert!(query.child_can_consent(15));
//! assert!(!query.child_can_consent(14));
//!
//! // Enumerate the member states with a national implementation.
//! assert_eq!(member_states::implemented_states().len(), 3);
//! ```

pub mod error;
pub mod france;
pub mod germany;
pub mod italy;
pub mod template;
pub mod transposition;

pub use error::MemberStateError;
pub use template::{
    GDPR_DEFAULT_AGE_OF_CONSENT, GDPR_MINIMUM_AGE_OF_CONSENT, MemberStateGdpr,
    MemberStateGdprBuilder, NationalActCitation, NationalDerogation, OpeningClause,
    SupervisoryAuthority,
};
pub use transposition::{
    DirectiveReference, TranspositionRecord, TranspositionStatus, TranspositionTracker,
};

use crate::shared::MemberState;

/// Return the national GDPR implementation for the given member state, if one is
/// modelled in this crate.
///
/// Currently Germany, France and Italy are implemented. Other member states return
/// [`None`]; callers can fall back to the GDPR core defaults (age 16, no national
/// derogations).
///
/// ## Example
///
/// ```rust
/// use legalis_eu::member_states::for_state;
/// use legalis_eu::shared::MemberState;
///
/// assert!(for_state(MemberState::Germany).is_some());
/// assert!(for_state(MemberState::Luxembourg).is_none());
/// ```
pub fn for_state(state: MemberState) -> Option<MemberStateGdpr> {
    match state {
        MemberState::Germany => Some(germany::implementation()),
        MemberState::France => Some(france::implementation()),
        MemberState::Italy => Some(italy::implementation()),
        _ => None,
    }
}

/// The list of member states for which a national implementation is modelled.
pub fn implemented_states() -> Vec<MemberState> {
    vec![
        MemberState::Germany,
        MemberState::France,
        MemberState::Italy,
    ]
}

/// All modelled national implementations.
pub fn all_implementations() -> Vec<MemberStateGdpr> {
    implemented_states()
        .into_iter()
        .filter_map(for_state)
        .collect()
}

/// Resolve the effective age of digital consent for a member state under Article 8(1)
/// GDPR.
///
/// If the member state has a national implementation, its specified age is returned;
/// otherwise the GDPR default of 16 ([`GDPR_DEFAULT_AGE_OF_CONSENT`]) applies.
///
/// ## Example
///
/// ```rust
/// use legalis_eu::member_states::effective_age_of_digital_consent;
/// use legalis_eu::shared::MemberState;
///
/// assert_eq!(effective_age_of_digital_consent(MemberState::France), 15);
/// assert_eq!(effective_age_of_digital_consent(MemberState::Italy), 14);
/// assert_eq!(effective_age_of_digital_consent(MemberState::Germany), 16);
/// // No national implementation -> GDPR default of 16.
/// assert_eq!(effective_age_of_digital_consent(MemberState::Spain), 16);
/// ```
pub fn effective_age_of_digital_consent(state: MemberState) -> u8 {
    for_state(state)
        .map(|impl_| impl_.age_of_digital_consent)
        .unwrap_or(GDPR_DEFAULT_AGE_OF_CONSENT)
}

/// The outcome of combining the GDPR core consent rule with national specifics for a
/// child of a given age.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombinedConsentAssessment {
    /// The member state assessed.
    pub state: MemberState,
    /// The age of the child whose consent is in question.
    pub child_age: u8,
    /// The effective age of digital consent applied (national if implemented, else 16).
    pub applicable_age_of_consent: u8,
    /// Whether the child may give valid consent on their own.
    pub child_can_consent: bool,
    /// Whether consent of the holder of parental responsibility is required.
    pub parental_consent_required: bool,
    /// Whether a national implementation (rather than the GDPR default) was applied.
    pub national_implementation_applied: bool,
}

/// Combine the GDPR Article 8(1) consent rule with national specifics for a child of the
/// given age in the given member state.
///
/// This is the principal national-law-integration entry point for the child-consent
/// dimension: it resolves the applicable age (national override or GDPR default) and
/// computes whether the child can consent alone.
///
/// ## Example
///
/// ```rust
/// use legalis_eu::member_states::combined_consent_assessment;
/// use legalis_eu::shared::MemberState;
///
/// // A 14-year-old in Italy can consent (age 14); the same child in Germany cannot (16).
/// let it = combined_consent_assessment(MemberState::Italy, 14);
/// assert!(it.child_can_consent);
/// assert!(it.national_implementation_applied);
///
/// let de = combined_consent_assessment(MemberState::Germany, 14);
/// assert!(!de.child_can_consent);
/// assert!(de.parental_consent_required);
/// ```
pub fn combined_consent_assessment(state: MemberState, child_age: u8) -> CombinedConsentAssessment {
    let implementation = for_state(state);
    let national_implementation_applied = implementation.is_some();
    let applicable_age_of_consent = implementation
        .as_ref()
        .map(|impl_| impl_.age_of_digital_consent)
        .unwrap_or(GDPR_DEFAULT_AGE_OF_CONSENT);
    let child_can_consent = child_age >= applicable_age_of_consent;
    CombinedConsentAssessment {
        state,
        child_age,
        applicable_age_of_consent,
        child_can_consent,
        parental_consent_required: !child_can_consent,
        national_implementation_applied,
    }
}

/// A small query facade combining GDPR core with a member state's national specifics.
///
/// Construct with [`NationalGdprQuery::new`] (returns an error for member states without
/// a modelled implementation) and use the accessors to answer combined questions.
#[derive(Debug, Clone)]
pub struct NationalGdprQuery {
    implementation: MemberStateGdpr,
}

impl NationalGdprQuery {
    /// Build a query for a member state that has a national implementation.
    ///
    /// Returns [`MemberStateError::NoImplementation`] if the member state is not modelled.
    pub fn new(state: MemberState) -> Result<Self, MemberStateError> {
        for_state(state)
            .map(|implementation| Self { implementation })
            .ok_or_else(|| MemberStateError::no_implementation(format!("{:?}", state)))
    }

    /// The underlying national implementation.
    pub fn implementation(&self) -> &MemberStateGdpr {
        &self.implementation
    }

    /// The member state being queried.
    pub fn state(&self) -> MemberState {
        self.implementation.state
    }

    /// The nationally-specified age of digital consent (Article 8(1) GDPR).
    pub fn age_of_digital_consent(&self) -> u8 {
        self.implementation.age_of_digital_consent
    }

    /// Whether a child of the given age may give valid consent on their own.
    pub fn child_can_consent(&self, age: u8) -> bool {
        self.implementation.can_child_consent(age)
    }

    /// The lead (national/federal) supervisory authority.
    pub fn supervisory_authority(&self) -> &SupervisoryAuthority {
        self.implementation.lead_authority()
    }

    /// All national derogations enacted under the given GDPR opening clause.
    pub fn derogations_for(&self, clause: OpeningClause) -> Vec<&NationalDerogation> {
        self.implementation.derogations_for(clause)
    }

    /// Produce a combined consent assessment for a child of the given age.
    pub fn assess_child_consent(&self, age: u8) -> CombinedConsentAssessment {
        combined_consent_assessment(self.state(), age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_state_resolution() {
        assert!(for_state(MemberState::Germany).is_some());
        assert!(for_state(MemberState::France).is_some());
        assert!(for_state(MemberState::Italy).is_some());
        assert!(for_state(MemberState::Spain).is_none());
        assert!(for_state(MemberState::Norway).is_none());
    }

    #[test]
    fn test_implemented_states_and_all() {
        assert_eq!(implemented_states().len(), 3);
        let all = all_implementations();
        assert_eq!(all.len(), 3);
        for impl_ in &all {
            assert!(impl_.validate().is_ok());
        }
    }

    #[test]
    fn test_effective_ages() {
        assert_eq!(effective_age_of_digital_consent(MemberState::Germany), 16);
        assert_eq!(effective_age_of_digital_consent(MemberState::France), 15);
        assert_eq!(effective_age_of_digital_consent(MemberState::Italy), 14);
        // Unmodelled member state -> GDPR default of 16.
        assert_eq!(effective_age_of_digital_consent(MemberState::Poland), 16);
    }

    #[test]
    fn test_combined_consent_assessment_national_vs_default() {
        // 14-year-old: can consent in Italy, not in Germany/France/Spain.
        let it = combined_consent_assessment(MemberState::Italy, 14);
        assert!(it.child_can_consent);
        assert!(it.national_implementation_applied);
        assert_eq!(it.applicable_age_of_consent, 14);

        let fr = combined_consent_assessment(MemberState::France, 14);
        assert!(!fr.child_can_consent);
        assert!(fr.parental_consent_required);

        let de = combined_consent_assessment(MemberState::Germany, 14);
        assert!(!de.child_can_consent);

        // Spain has no national implementation -> GDPR default 16 applies.
        let es = combined_consent_assessment(MemberState::Spain, 15);
        assert!(!es.child_can_consent);
        assert!(!es.national_implementation_applied);
        assert_eq!(es.applicable_age_of_consent, 16);
    }

    #[test]
    fn test_national_query_facade() {
        let query = NationalGdprQuery::new(MemberState::Germany).expect("germany implemented");
        assert_eq!(query.state(), MemberState::Germany);
        assert_eq!(query.age_of_digital_consent(), 16);
        assert_eq!(query.supervisory_authority().abbreviation, "BfDI");
        assert!(query.child_can_consent(16));
        assert!(!query.child_can_consent(15));
        assert!(
            query
                .derogations_for(OpeningClause::Article88Employment)
                .iter()
                .any(|d| d.national_citation == "§ 26 BDSG")
        );
    }

    #[test]
    fn test_national_query_error_for_unmodelled_state() {
        let result = NationalGdprQuery::new(MemberState::Cyprus);
        assert!(matches!(result, Err(MemberStateError::NoImplementation(_))));
    }

    #[test]
    fn test_query_assess_child_consent() {
        let query = NationalGdprQuery::new(MemberState::Italy).expect("italy implemented");
        let assessment = query.assess_child_consent(14);
        assert!(assessment.child_can_consent);
        assert_eq!(assessment.applicable_age_of_consent, 14);
    }

    #[test]
    fn test_three_states_have_distinct_ages() {
        // Confirms the headline facts: DE=16, FR=15, IT=14.
        let de = NationalGdprQuery::new(MemberState::Germany).expect("de");
        let fr = NationalGdprQuery::new(MemberState::France).expect("fr");
        let it = NationalGdprQuery::new(MemberState::Italy).expect("it");
        assert_eq!(de.age_of_digital_consent(), 16);
        assert_eq!(fr.age_of_digital_consent(), 15);
        assert_eq!(it.age_of_digital_consent(), 14);
        assert_ne!(de.age_of_digital_consent(), fr.age_of_digital_consent());
        assert_ne!(fr.age_of_digital_consent(), it.age_of_digital_consent());
    }
}
