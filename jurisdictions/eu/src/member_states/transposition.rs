//! Directive transposition tracking.
//!
//! Unlike a Regulation (directly applicable), an EU **Directive** is binding as to the
//! result to be achieved but leaves the choice of form and methods to the member states
//! (Article 288 TFEU). Each directive sets a **transposition deadline** by which member
//! states must bring into force the national laws, regulations and administrative
//! provisions necessary to comply with it.
//!
//! This module models, per member state, how a given directive has been transposed: the
//! national act that transposes it, the date of transposition, and a [`TranspositionStatus`]
//! capturing whether transposition is complete, partial, late, or absent.
//!
//! While the GDPR itself is a Regulation (and therefore not "transposed"), several
//! closely related instruments *are* directives — e.g. the ePrivacy Directive
//! (2002/58/EC), the Law Enforcement Directive (2016/680), the NIS2 Directive
//! (2022/2555) and the Consumer Rights Directive (2011/83/EU). National GDPR
//! implementing acts (such as the German BDSG or the French Loi 78-17) frequently also
//! transpose the Law Enforcement Directive in the same instrument, which this tracker
//! can represent.

use crate::citation::EuCitation;
use crate::member_states::error::MemberStateError;
use crate::shared::MemberState;
use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// Status of a directive's transposition into national law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum TranspositionStatus {
    /// Fully transposed on or before the deadline.
    Complete,
    /// Transposed, but only in part (some provisions outstanding).
    Partial,
    /// Transposed, but after the deadline (late transposition).
    Late,
    /// Not yet transposed.
    NotTransposed,
    /// The directive does not require transposition for this member state
    /// (e.g. EEA-specific or sectoral non-applicability).
    NotApplicable,
}

impl TranspositionStatus {
    /// Whether the directive is considered transposed in any form (complete, partial,
    /// or late).
    pub fn is_transposed(&self) -> bool {
        matches!(self, Self::Complete | Self::Partial | Self::Late)
    }

    /// Whether the transposition fully satisfies the directive (complete only).
    pub fn is_compliant(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// A reference to the EU directive being tracked.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DirectiveReference {
    /// Year of the directive.
    pub year: u16,
    /// Sequential number of the directive.
    pub number: u32,
    /// Short English title (e.g. "ePrivacy Directive").
    pub title: String,
    /// Transposition deadline set by the directive (the date by which member states must
    /// bring transposing measures into force).
    pub transposition_deadline: NaiveDate,
}

impl DirectiveReference {
    /// Construct a directive reference.
    pub fn new(
        year: u16,
        number: u32,
        title: impl Into<String>,
        transposition_deadline: NaiveDate,
    ) -> Self {
        Self {
            year,
            number,
            title: title.into(),
            transposition_deadline,
        }
    }

    /// The CELEX/EUR-Lex citation for this directive.
    pub fn citation(&self) -> EuCitation {
        EuCitation::directive(self.year, self.number)
    }

    /// The CELEX number string (e.g. "32002L0058").
    pub fn celex(&self) -> String {
        format!("3{}L{:04}", self.year, self.number)
    }
}

/// A record of how one member state transposed one directive.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TranspositionRecord {
    /// The member state.
    pub state: MemberState,
    /// The directive being transposed.
    pub directive: DirectiveReference,
    /// Citation of the national act that effects the transposition (e.g. "BDSG",
    /// "Loi 78-17", "D.Lgs. 196/2003").
    pub national_act: String,
    /// Date on which the national transposing measures entered into force, if known.
    pub transposed_on: Option<NaiveDate>,
    /// Transposition status.
    pub status: TranspositionStatus,
}

impl TranspositionRecord {
    /// Construct a new transposition record.
    pub fn new(
        state: MemberState,
        directive: DirectiveReference,
        national_act: impl Into<String>,
        transposed_on: Option<NaiveDate>,
        status: TranspositionStatus,
    ) -> Self {
        Self {
            state,
            directive,
            national_act: national_act.into(),
            transposed_on,
            status,
        }
    }

    /// Whether transposition (if dated) occurred after the directive's deadline.
    ///
    /// Returns `None` when no transposition date is recorded.
    pub fn is_overdue(&self) -> Option<bool> {
        self.transposed_on
            .map(|date| date > self.directive.transposition_deadline)
    }

    /// Validate the record for internal consistency.
    ///
    /// In particular, a [`TranspositionStatus::Late`] record with a transposition date
    /// must actually be after the deadline, and a [`TranspositionStatus::Complete`]
    /// record with a date must be on or before the deadline.
    pub fn validate(&self) -> Result<(), MemberStateError> {
        if self.national_act.trim().is_empty() && self.status.is_transposed() {
            return Err(MemberStateError::invalid_transposition(
                "transposed status requires a national act citation",
            ));
        }
        if let Some(date) = self.transposed_on {
            let overdue = date > self.directive.transposition_deadline;
            match self.status {
                TranspositionStatus::Late if !overdue => {
                    return Err(MemberStateError::invalid_transposition(
                        "status Late but transposition date is not after the deadline",
                    ));
                }
                TranspositionStatus::Complete if overdue => {
                    return Err(MemberStateError::invalid_transposition(
                        "status Complete but transposition date is after the deadline (should be Late)",
                    ));
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// A tracker holding transposition records, queryable by member state and directive.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TranspositionTracker {
    /// All recorded transpositions.
    pub records: Vec<TranspositionRecord>,
}

impl TranspositionTracker {
    /// Create an empty tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a record after validating it.
    pub fn add(&mut self, record: TranspositionRecord) -> Result<(), MemberStateError> {
        record.validate()?;
        self.records.push(record);
        Ok(())
    }

    /// All records for a given member state.
    pub fn for_state(&self, state: MemberState) -> Vec<&TranspositionRecord> {
        self.records.iter().filter(|r| r.state == state).collect()
    }

    /// All records for a given directive (matched by year and number).
    pub fn for_directive(&self, year: u16, number: u32) -> Vec<&TranspositionRecord> {
        self.records
            .iter()
            .filter(|r| r.directive.year == year && r.directive.number == number)
            .collect()
    }

    /// The record for a specific member state and directive, if present.
    pub fn lookup(
        &self,
        state: MemberState,
        year: u16,
        number: u32,
    ) -> Option<&TranspositionRecord> {
        self.records
            .iter()
            .find(|r| r.state == state && r.directive.year == year && r.directive.number == number)
    }

    /// The number of member states that have fully transposed the given directive.
    pub fn complete_count(&self, year: u16, number: u32) -> usize {
        self.for_directive(year, number)
            .iter()
            .filter(|r| r.status.is_compliant())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eprivacy() -> DirectiveReference {
        // ePrivacy Directive 2002/58/EC, transposition deadline 31 October 2003.
        DirectiveReference::new(
            2002,
            58,
            "ePrivacy Directive",
            NaiveDate::from_ymd_opt(2003, 10, 31).expect("valid date"),
        )
    }

    #[test]
    fn test_directive_celex() {
        assert_eq!(eprivacy().celex(), "32002L0058");
        assert_eq!(eprivacy().citation().celex, "32002L0058");
    }

    #[test]
    fn test_status_helpers() {
        assert!(TranspositionStatus::Complete.is_transposed());
        assert!(TranspositionStatus::Late.is_transposed());
        assert!(TranspositionStatus::Partial.is_transposed());
        assert!(!TranspositionStatus::NotTransposed.is_transposed());
        assert!(TranspositionStatus::Complete.is_compliant());
        assert!(!TranspositionStatus::Late.is_compliant());
    }

    #[test]
    fn test_overdue_detection() {
        let late = TranspositionRecord::new(
            MemberState::Germany,
            eprivacy(),
            "TKG",
            Some(NaiveDate::from_ymd_opt(2004, 6, 1).expect("valid")),
            TranspositionStatus::Late,
        );
        assert_eq!(late.is_overdue(), Some(true));
        assert!(late.validate().is_ok());

        let on_time = TranspositionRecord::new(
            MemberState::France,
            eprivacy(),
            "LCEN",
            Some(NaiveDate::from_ymd_opt(2003, 10, 1).expect("valid")),
            TranspositionStatus::Complete,
        );
        assert_eq!(on_time.is_overdue(), Some(false));
        assert!(on_time.validate().is_ok());
    }

    #[test]
    fn test_validate_inconsistent_status() {
        // Marked Late but date is on time -> invalid.
        let bad = TranspositionRecord::new(
            MemberState::Italy,
            eprivacy(),
            "Codice",
            Some(NaiveDate::from_ymd_opt(2003, 1, 1).expect("valid")),
            TranspositionStatus::Late,
        );
        assert!(bad.validate().is_err());

        // Marked Complete but date is overdue -> invalid.
        let bad2 = TranspositionRecord::new(
            MemberState::Italy,
            eprivacy(),
            "Codice",
            Some(NaiveDate::from_ymd_opt(2005, 1, 1).expect("valid")),
            TranspositionStatus::Complete,
        );
        assert!(bad2.validate().is_err());
    }

    #[test]
    fn test_tracker_queries() {
        let mut tracker = TranspositionTracker::new();
        tracker
            .add(TranspositionRecord::new(
                MemberState::Germany,
                eprivacy(),
                "TKG/TTDSG",
                // Germany transposed the ePrivacy Directive after the 31 Oct 2003
                // deadline (TKG of June 2004), hence Late.
                Some(NaiveDate::from_ymd_opt(2004, 6, 1).expect("valid")),
                TranspositionStatus::Late,
            ))
            .expect("add ok");
        tracker
            .add(TranspositionRecord::new(
                MemberState::France,
                eprivacy(),
                "LCEN",
                Some(NaiveDate::from_ymd_opt(2003, 10, 1).expect("valid")),
                TranspositionStatus::Complete,
            ))
            .expect("add ok");

        assert_eq!(tracker.for_state(MemberState::Germany).len(), 1);
        assert_eq!(tracker.for_directive(2002, 58).len(), 2);
        assert!(tracker.lookup(MemberState::France, 2002, 58).is_some());
        assert!(tracker.lookup(MemberState::Italy, 2002, 58).is_none());
        // Only France is recorded as Complete; Germany's record is Late.
        assert_eq!(tracker.complete_count(2002, 58), 1);
    }
}
