//! Access and correction requests — PDPA Part 5 (ss. 21-22).
//!
//! * **Access request (s. 21)**: on an individual's request, an organisation
//!   must provide the individual with their personal data and information about
//!   how it has been used or disclosed, "as soon as reasonably possible". Where
//!   the organisation cannot respond within **30 days**, it must, within those
//!   30 days, inform the individual in writing of the time by which it will
//!   respond (PDP Regulations 2021, reg. 5).
//! * **Correction request (s. 22)**: on request, an organisation must (unless it
//!   is satisfied on reasonable grounds that a correction should not be made)
//!   correct an error or omission in the individual's personal data as soon as
//!   practicable, and send the corrected data to other organisations to which it
//!   was disclosed within the preceding year.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Prescribed period (in days) within which an organisation must either respond
/// to an access request or notify the individual of the time by which it will
/// respond (PDPA s. 21 read with reg. 5 of the PDP Regulations 2021).
pub const ACCESS_REQUEST_RESPONSE_DAYS: i64 = 30;

/// Kind of data-subject request under Part 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataSubjectRequestKind {
    /// Request for access to personal data and use/disclosure information (s. 21).
    Access,
    /// Request to correct an error or omission in personal data (s. 22).
    Correction,
}

impl DataSubjectRequestKind {
    /// Returns the governing PDPA section reference.
    pub fn statute_section(&self) -> &'static str {
        match self {
            DataSubjectRequestKind::Access => "PDPA s. 21",
            DataSubjectRequestKind::Correction => "PDPA s. 22",
        }
    }
}

/// A data-subject access or correction request and its handling status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSubjectRequest {
    /// Stable identifier for this request.
    pub request_id: String,
    /// Identifier for the requesting individual.
    pub data_subject_id: String,
    /// Kind of request.
    pub kind: DataSubjectRequestKind,
    /// When the request was received.
    pub received_date: DateTime<Utc>,
    /// When the organisation responded (provided data / made the correction), if
    /// it has.
    pub responded_date: Option<DateTime<Utc>>,
    /// When the organisation notified the individual that it could not respond
    /// within 30 days, and of the time by which it would respond (s. 21 /
    /// reg. 5). Only relevant to access requests.
    pub extension_notice_date: Option<DateTime<Utc>>,
    /// For a correction request: whether the organisation declined to correct on
    /// reasonable grounds (s. 22(2)), in which case it must annotate the data
    /// (s. 22(5)) rather than respond with a correction.
    pub correction_refused_reasonable_grounds: bool,
}

impl DataSubjectRequest {
    /// Creates an access request received now.
    pub fn access(request_id: impl Into<String>, data_subject_id: impl Into<String>) -> Self {
        Self::new(request_id, data_subject_id, DataSubjectRequestKind::Access)
    }

    /// Creates a correction request received now.
    pub fn correction(request_id: impl Into<String>, data_subject_id: impl Into<String>) -> Self {
        Self::new(
            request_id,
            data_subject_id,
            DataSubjectRequestKind::Correction,
        )
    }

    fn new(
        request_id: impl Into<String>,
        data_subject_id: impl Into<String>,
        kind: DataSubjectRequestKind,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            data_subject_id: data_subject_id.into(),
            kind,
            received_date: Utc::now(),
            responded_date: None,
            extension_notice_date: None,
            correction_refused_reasonable_grounds: false,
        }
    }

    /// Overrides the received date (useful for testing the 30-day window).
    pub fn received_at(mut self, when: DateTime<Utc>) -> Self {
        self.received_date = when;
        self
    }

    /// Records that the organisation responded at the given time.
    pub fn respond(&mut self, when: DateTime<Utc>) -> &mut Self {
        self.responded_date = Some(when);
        self
    }

    /// Records that an extension notice was sent at the given time (s. 21 /
    /// reg. 5).
    pub fn send_extension_notice(&mut self, when: DateTime<Utc>) -> &mut Self {
        self.extension_notice_date = Some(when);
        self
    }

    /// Records that a correction was refused on reasonable grounds (s. 22(2)).
    pub fn refuse_correction(&mut self) -> &mut Self {
        self.correction_refused_reasonable_grounds = true;
        self
    }

    /// Returns the statutory deadline (30 days after receipt) for responding or
    /// sending an extension notice (s. 21 / reg. 5).
    pub fn response_deadline(&self) -> DateTime<Utc> {
        self.received_date + Duration::days(ACCESS_REQUEST_RESPONSE_DAYS)
    }

    /// Returns whether the request has been handled within the prescribed period.
    ///
    /// A request is compliant if, on the calendar date `now`, either:
    /// * the organisation has responded within 30 days; or
    /// * the organisation has sent a valid extension notice within 30 days (for
    ///   access requests under reg. 5); or
    /// * for a correction request, it was refused on reasonable grounds (which
    ///   triggers annotation under s. 22(5) instead of a correction).
    ///
    /// Otherwise, if 30 days have elapsed without any of the above, the request
    /// is overdue.
    pub fn is_within_deadline(&self, now: DateTime<Utc>) -> bool {
        let deadline = self.response_deadline();
        if self.correction_refused_reasonable_grounds {
            return true;
        }
        if let Some(responded) = self.responded_date
            && responded <= deadline
        {
            return true;
        }
        if let Some(notice) = self.extension_notice_date
            && notice <= deadline
            && self.kind == DataSubjectRequestKind::Access
        {
            return true;
        }
        // Not yet responded / notified: compliant only while still within window.
        now <= deadline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s)
            .expect("valid timestamp")
            .with_timezone(&Utc)
    }

    #[test]
    fn access_response_within_30_days_is_compliant() {
        let mut req =
            DataSubjectRequest::access("r1", "subj").received_at(at("2026-03-01T00:00:00Z"));
        req.respond(at("2026-03-20T00:00:00Z"));
        assert!(req.is_within_deadline(at("2026-04-15T00:00:00Z")));
    }

    #[test]
    fn access_overdue_without_response_or_notice() {
        let req = DataSubjectRequest::access("r2", "subj").received_at(at("2026-03-01T00:00:00Z"));
        // Day 31 with no response and no extension notice -> overdue.
        assert!(!req.is_within_deadline(at("2026-04-01T00:00:00Z")));
    }

    #[test]
    fn access_extension_notice_keeps_compliant() {
        let mut req =
            DataSubjectRequest::access("r3", "subj").received_at(at("2026-03-01T00:00:00Z"));
        req.send_extension_notice(at("2026-03-25T00:00:00Z"));
        // Even past day 30, a timely extension notice keeps it compliant.
        assert!(req.is_within_deadline(at("2026-04-10T00:00:00Z")));
    }

    #[test]
    fn correction_refused_on_reasonable_grounds_is_compliant() {
        let mut req =
            DataSubjectRequest::correction("r4", "subj").received_at(at("2026-03-01T00:00:00Z"));
        req.refuse_correction();
        assert!(req.is_within_deadline(at("2026-05-01T00:00:00Z")));
    }
}
