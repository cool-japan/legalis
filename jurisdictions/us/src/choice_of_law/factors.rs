//! US Choice of Law Connecting Factors
//!
//! This module defines connecting factors used in US choice of law analysis.
//! These factors help determine which state has the most significant relationship
//! to a dispute.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Connecting factors for choice of law analysis.
///
/// Based on Restatement (Second) of Conflict of Laws § 6 (general principles)
/// and § 145 (torts), § 188 (contracts).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContactingFactor {
    /// Place where injury occurred (torts)
    PlaceOfInjury(String),

    /// Place where conduct causing injury occurred (torts)
    PlaceOfConduct(String),

    /// Plaintiff's domicile at time of injury
    PlaintiffDomicile(String),

    /// Defendant's domicile at time of injury
    DefendantDomicile(String),

    /// Plaintiff's principal place of business
    PlaintiffBusinessLocation(String),

    /// Defendant's principal place of business
    DefendantBusinessLocation(String),

    /// Place where contract was negotiated
    PlaceOfNegotiation(String),

    /// Place where contract was executed
    PlaceOfExecution(String),

    /// Place of performance for contract
    PlaceOfPerformance(String),

    /// Location of subject matter (e.g., real property, goods)
    LocationOfSubjectMatter(String),

    /// Place where parties' relationship is centered
    CenterOfRelationship(String),

    /// Forum state (where lawsuit is filed)
    ForumState(String),

    /// Custom factor for special circumstances
    Custom { name: String, state: String },
}

impl ContactingFactor {
    /// Get the state code associated with this factor.
    #[must_use]
    pub fn state_code(&self) -> &str {
        match self {
            Self::PlaceOfInjury(s)
            | Self::PlaceOfConduct(s)
            | Self::PlaintiffDomicile(s)
            | Self::DefendantDomicile(s)
            | Self::PlaintiffBusinessLocation(s)
            | Self::DefendantBusinessLocation(s)
            | Self::PlaceOfNegotiation(s)
            | Self::PlaceOfExecution(s)
            | Self::PlaceOfPerformance(s)
            | Self::LocationOfSubjectMatter(s)
            | Self::CenterOfRelationship(s)
            | Self::ForumState(s) => s,
            Self::Custom { state, .. } => state,
        }
    }

    /// Get human-readable description of this factor.
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            Self::PlaceOfInjury(s) => format!("Place of injury: {s}"),
            Self::PlaceOfConduct(s) => format!("Place of conduct: {s}"),
            Self::PlaintiffDomicile(s) => format!("Plaintiff's domicile: {s}"),
            Self::DefendantDomicile(s) => format!("Defendant's domicile: {s}"),
            Self::PlaintiffBusinessLocation(s) => {
                format!("Plaintiff's business location: {s}")
            }
            Self::DefendantBusinessLocation(s) => {
                format!("Defendant's business location: {s}")
            }
            Self::PlaceOfNegotiation(s) => format!("Place of negotiation: {s}"),
            Self::PlaceOfExecution(s) => format!("Place of execution: {s}"),
            Self::PlaceOfPerformance(s) => format!("Place of performance: {s}"),
            Self::LocationOfSubjectMatter(s) => format!("Location of subject matter: {s}"),
            Self::CenterOfRelationship(s) => format!("Center of relationship: {s}"),
            Self::ForumState(s) => format!("Forum state: {s}"),
            Self::Custom { name, state } => format!("{name}: {state}"),
        }
    }
}

/// Collection of choice of law factors for a dispute.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct USChoiceOfLawFactors {
    /// All connecting factors present in the dispute
    factors: Vec<ContactingFactor>,

    /// Policy considerations by state
    state_interests: HashMap<String, Vec<String>>,

    /// Additional context
    notes: Option<String>,
}

impl USChoiceOfLawFactors {
    /// Create new empty factors collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a connecting factor.
    #[must_use]
    pub fn with_factor(mut self, factor: ContactingFactor) -> Self {
        self.factors.push(factor);
        self
    }

    /// Add a state interest (policy consideration).
    #[must_use]
    pub fn with_state_interest(
        mut self,
        state: impl Into<String>,
        interest: impl Into<String>,
    ) -> Self {
        self.state_interests
            .entry(state.into())
            .or_default()
            .push(interest.into());
        self
    }

    /// Add contextual notes.
    #[must_use]
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Get all factors.
    #[must_use]
    pub fn factors(&self) -> &[ContactingFactor] {
        &self.factors
    }

    /// Get all states with contacts.
    #[must_use]
    pub fn connected_states(&self) -> Vec<&str> {
        let mut states: Vec<&str> = self
            .factors
            .iter()
            .map(ContactingFactor::state_code)
            .collect();
        states.sort_unstable();
        states.dedup();
        states
    }

    /// Count contacts for a specific state.
    #[must_use]
    pub fn contact_count(&self, state: &str) -> usize {
        self.factors
            .iter()
            .filter(|f| f.state_code() == state)
            .count()
    }

    /// Get state interests (policy considerations).
    #[must_use]
    pub fn state_interests(&self, state: &str) -> Vec<&str> {
        self.state_interests
            .get(state)
            .map(|interests| interests.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// Get all state interests as an iterator.
    pub fn all_state_interests(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.state_interests.iter()
    }

    /// Check if this is a true conflict (multiple states with legitimate interests).
    #[must_use]
    pub fn is_true_conflict(&self) -> bool {
        self.state_interests.len() > 1
    }

    /// Check if this is a false conflict (only one state has legitimate interest).
    #[must_use]
    pub fn is_false_conflict(&self) -> bool {
        self.state_interests.len() == 1
    }

    /// Get notes.
    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contacting_factor_state_code() {
        let factor = ContactingFactor::PlaceOfInjury("CA".to_string());
        assert_eq!(factor.state_code(), "CA");

        let factor = ContactingFactor::DefendantDomicile("NY".to_string());
        assert_eq!(factor.state_code(), "NY");
    }

    #[test]
    fn test_contacting_factor_description() {
        let factor = ContactingFactor::PlaceOfInjury("CA".to_string());
        assert!(factor.description().contains("Place of injury"));
        assert!(factor.description().contains("CA"));
    }

    #[test]
    fn test_factors_collection() {
        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("CA".to_string()))
            .with_factor(ContactingFactor::DefendantDomicile("NY".to_string()))
            .with_factor(ContactingFactor::PlaintiffDomicile("CA".to_string()));

        assert_eq!(factors.factors().len(), 3);
        assert_eq!(factors.connected_states(), vec!["CA", "NY"]);
        assert_eq!(factors.contact_count("CA"), 2);
        assert_eq!(factors.contact_count("NY"), 1);
    }

    #[test]
    fn test_state_interests() {
        let factors = USChoiceOfLawFactors::new()
            .with_state_interest("CA", "Protecting California residents")
            .with_state_interest("CA", "Regulating conduct within California")
            .with_state_interest("NY", "Regulating NY corporations");

        let ca_interests = factors.state_interests("CA");
        assert_eq!(ca_interests.len(), 2);

        let ny_interests = factors.state_interests("NY");
        assert_eq!(ny_interests.len(), 1);
    }

    #[test]
    fn test_conflict_detection() {
        let true_conflict = USChoiceOfLawFactors::new()
            .with_state_interest("CA", "Interest 1")
            .with_state_interest("NY", "Interest 2");
        assert!(true_conflict.is_true_conflict());
        assert!(!true_conflict.is_false_conflict());

        let false_conflict =
            USChoiceOfLawFactors::new().with_state_interest("CA", "Only CA has interest");
        assert!(!false_conflict.is_true_conflict());
        assert!(false_conflict.is_false_conflict());
    }

    #[test]
    fn test_builder_pattern() {
        let factors = USChoiceOfLawFactors::new()
            .with_factor(ContactingFactor::PlaceOfInjury("TX".to_string()))
            .with_state_interest("TX", "Protecting Texas residents")
            .with_notes("Auto accident case");

        assert_eq!(factors.factors().len(), 1);
        assert!(factors.notes().is_some());
        assert!(factors.notes().unwrap().contains("Auto accident"));
    }
}
