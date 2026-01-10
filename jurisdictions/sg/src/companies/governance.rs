//! Corporate Governance - AGM, Board Meetings, Resolutions
//!
//! This module provides types and utilities for Singapore corporate governance requirements.
//!
//! ## Key Requirements
//!
//! - **AGM (Annual General Meeting)**: s. 175
//!   - First AGM: Within 18 months of incorporation
//!   - Subsequent: Within 15 months of previous AGM, within 6 months of FYE
//! - **Board Meetings**: Director decision-making
//! - **Resolutions**: Ordinary and special resolutions
//! - **Minutes**: Record-keeping requirements
//!
//! ## Examples
//!
//! ```
//! use legalis_sg::companies::governance::*;
//! use chrono::Utc;
//!
//! // Calculate next AGM deadline
//! let incorporation_date = Utc::now();
//! let deadline = calculate_first_agm_deadline(incorporation_date);
//! println!("First AGM must be held by: {}", deadline);
//! ```

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Annual General Meeting (AGM) record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnualGeneralMeeting {
    /// Meeting date/time
    pub meeting_date: DateTime<Utc>,
    /// Meeting location
    pub location: String,
    /// Attendees (directors, shareholders)
    pub attendees: Vec<Attendee>,
    /// Agenda items
    pub agenda: Vec<AgendaItem>,
    /// Resolutions passed
    pub resolutions: Vec<Resolution>,
    /// Minutes recorded
    pub minutes: Option<String>,
}

/// Meeting attendee
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attendee {
    /// Name
    pub name: String,
    /// Role (Director, Shareholder, Secretary, etc.)
    pub role: AttendeeRole,
    /// Present or represented by proxy
    pub present: bool,
    /// Proxy holder (if represented by proxy)
    pub proxy_holder: Option<String>,
}

/// Attendee role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttendeeRole {
    /// Company director
    Director,
    /// Shareholder/member
    Shareholder,
    /// Company secretary
    CompanySecretary,
    /// Auditor
    Auditor,
    /// Legal counsel
    LegalCounsel,
    /// Observer
    Observer,
}

/// AGM agenda item
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgendaItem {
    /// Item number
    pub item_number: u32,
    /// Description
    pub description: String,
    /// Whether item requires resolution
    pub requires_resolution: bool,
}

/// Company resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolution {
    /// Resolution number
    pub resolution_number: String,
    /// Type of resolution
    pub resolution_type: ResolutionType,
    /// Resolution text
    pub text: String,
    /// Voting results
    pub voting_result: VotingResult,
    /// Date passed
    pub date_passed: DateTime<Utc>,
}

/// Type of resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionType {
    /// Ordinary resolution (>50% votes)
    ///
    /// Used for routine matters:
    /// - Approval of accounts
    /// - Declaration of dividends
    /// - Appointment of auditors
    /// - Appointment of directors
    Ordinary,

    /// Special resolution (≥75% votes)
    ///
    /// Required for major matters:
    /// - Amendment of constitution
    /// - Change of company name
    /// - Reduction of share capital
    /// - Winding up
    Special,

    /// Written resolution (all members agree)
    ///
    /// Can be passed without meeting if all members agree
    Written,

    /// Board resolution (directors only)
    ///
    /// Decisions made by board of directors
    Board,
}

impl ResolutionType {
    /// Returns required majority percentage
    pub fn required_majority(&self) -> f64 {
        match self {
            ResolutionType::Ordinary => 50.0,
            ResolutionType::Special => 75.0,
            ResolutionType::Written => 100.0,
            ResolutionType::Board => 50.0,
        }
    }
}

/// Voting result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VotingResult {
    /// Total votes cast
    pub total_votes: u64,
    /// Votes in favor
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Abstentions
    pub abstentions: u64,
    /// Whether resolution passed
    pub passed: bool,
}

impl VotingResult {
    /// Creates a new voting result
    pub fn new(votes_for: u64, votes_against: u64, abstentions: u64) -> Self {
        let total_votes = votes_for + votes_against;
        let passed = votes_for > votes_against; // Simple majority for ordinary

        Self {
            total_votes,
            votes_for,
            votes_against,
            abstentions,
            passed,
        }
    }

    /// Calculates percentage in favor
    pub fn percentage_for(&self) -> f64 {
        if self.total_votes == 0 {
            0.0
        } else {
            (self.votes_for as f64 / self.total_votes as f64) * 100.0
        }
    }

    /// Checks if resolution passed with required majority
    pub fn passed_with_majority(&self, required_percent: f64) -> bool {
        self.percentage_for() >= required_percent
    }
}

/// Calculates first AGM deadline (18 months from incorporation)
///
/// Section 175(1): First AGM must be held within 18 months of incorporation.
///
/// ## Examples
///
/// ```
/// use legalis_sg::companies::governance::*;
/// use chrono::Utc;
///
/// let incorporation = Utc::now();
/// let deadline = calculate_first_agm_deadline(incorporation);
/// // Deadline is 18 months (548 days) from incorporation
/// ```
pub fn calculate_first_agm_deadline(incorporation_date: DateTime<Utc>) -> DateTime<Utc> {
    // 18 months = approximately 548 days
    incorporation_date + Duration::days(548)
}

/// Calculates subsequent AGM deadline
///
/// Section 175(2): Subsequent AGMs must be held:
/// - Within 15 months of previous AGM
/// - Within 6 months after FYE
/// - At least once per calendar year
///
/// ## Examples
///
/// ```
/// use legalis_sg::companies::governance::*;
/// use chrono::Utc;
///
/// let last_agm = Utc::now();
/// let deadline = calculate_subsequent_agm_deadline(last_agm);
/// // Deadline is 15 months (456 days) from last AGM
/// ```
pub fn calculate_subsequent_agm_deadline(last_agm_date: DateTime<Utc>) -> DateTime<Utc> {
    // 15 months = approximately 456 days
    last_agm_date + Duration::days(456)
}

/// Checks if AGM is overdue
pub fn is_agm_overdue(last_agm_date: DateTime<Utc>, is_first_agm: bool) -> bool {
    let deadline = if is_first_agm {
        calculate_first_agm_deadline(last_agm_date)
    } else {
        calculate_subsequent_agm_deadline(last_agm_date)
    };

    Utc::now() > deadline
}

/// Calculates days until AGM due
pub fn days_until_agm_due(last_agm_date: DateTime<Utc>, is_first_agm: bool) -> i64 {
    let deadline = if is_first_agm {
        calculate_first_agm_deadline(last_agm_date)
    } else {
        calculate_subsequent_agm_deadline(last_agm_date)
    };

    (deadline - Utc::now()).num_days()
}

/// Board meeting record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoardMeeting {
    /// Meeting date/time
    pub meeting_date: DateTime<Utc>,
    /// Directors present
    pub directors_present: Vec<String>,
    /// Quorum met (typically majority of directors)
    pub quorum_met: bool,
    /// Resolutions passed
    pub resolutions: Vec<Resolution>,
    /// Minutes
    pub minutes: Option<String>,
}

impl BoardMeeting {
    /// Checks if quorum is met
    pub fn check_quorum(&self, total_directors: usize) -> bool {
        let present = self.directors_present.len();
        let required = (total_directors / 2) + 1; // Simple majority
        present >= required
    }
}

/// Notice period for meetings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoticeRequirement {
    /// AGM: 14 days notice (s. 177)
    AgmNotice,
    /// Extraordinary General Meeting: 14 days notice
    EgmNotice,
    /// Board meeting: Reasonable notice (typically 3-7 days)
    BoardMeetingNotice,
    /// Short notice (if all members agree)
    ShortNotice,
}

impl NoticeRequirement {
    /// Returns minimum notice period in days
    pub fn minimum_days(&self) -> u32 {
        match self {
            NoticeRequirement::AgmNotice => 14,
            NoticeRequirement::EgmNotice => 14,
            NoticeRequirement::BoardMeetingNotice => 3,
            NoticeRequirement::ShortNotice => 0,
        }
    }
}

/// Checks if sufficient notice was given
pub fn is_sufficient_notice(
    notice_date: DateTime<Utc>,
    meeting_date: DateTime<Utc>,
    requirement: NoticeRequirement,
) -> bool {
    let days_notice = (meeting_date - notice_date).num_days();
    days_notice >= requirement.minimum_days() as i64
}

/// Calculates annual return filing deadline (7 months after FYE)
///
/// Section 197: Annual return must be filed within 7 months of FYE.
pub fn calculate_annual_return_deadline(financial_year_end: DateTime<Utc>) -> DateTime<Utc> {
    // 7 months = approximately 213 days
    financial_year_end + Duration::days(213)
}

/// Checks if annual return is overdue
pub fn is_annual_return_overdue(financial_year_end: DateTime<Utc>) -> bool {
    let deadline = calculate_annual_return_deadline(financial_year_end);
    Utc::now() > deadline
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_first_agm_deadline() {
        let incorporation = Utc::now();
        let deadline = calculate_first_agm_deadline(incorporation);
        let days = (deadline - incorporation).num_days();
        assert_eq!(days, 548); // 18 months
    }

    #[test]
    fn test_calculate_subsequent_agm_deadline() {
        let last_agm = Utc::now();
        let deadline = calculate_subsequent_agm_deadline(last_agm);
        let days = (deadline - last_agm).num_days();
        assert_eq!(days, 456); // 15 months
    }

    #[test]
    fn test_voting_result() {
        let result = VotingResult::new(75, 25, 10);
        assert_eq!(result.percentage_for(), 75.0);
        assert!(result.passed_with_majority(50.0)); // Ordinary resolution
        assert!(result.passed_with_majority(75.0)); // Special resolution
    }

    #[test]
    fn test_resolution_type_majority() {
        assert_eq!(ResolutionType::Ordinary.required_majority(), 50.0);
        assert_eq!(ResolutionType::Special.required_majority(), 75.0);
        assert_eq!(ResolutionType::Written.required_majority(), 100.0);
    }

    #[test]
    fn test_board_meeting_quorum() {
        let meeting = BoardMeeting {
            meeting_date: Utc::now(),
            directors_present: vec!["Director 1".to_string(), "Director 2".to_string()],
            quorum_met: true,
            resolutions: vec![],
            minutes: None,
        };

        // 2 out of 3 directors = quorum met
        assert!(meeting.check_quorum(3));
        // 2 out of 5 directors = quorum not met
        assert!(!meeting.check_quorum(5));
    }

    #[test]
    fn test_notice_requirement_days() {
        assert_eq!(NoticeRequirement::AgmNotice.minimum_days(), 14);
        assert_eq!(NoticeRequirement::BoardMeetingNotice.minimum_days(), 3);
    }

    #[test]
    fn test_is_sufficient_notice() {
        let notice = Utc::now();
        let meeting_15_days = notice + Duration::days(15);
        let meeting_10_days = notice + Duration::days(10);

        assert!(is_sufficient_notice(
            notice,
            meeting_15_days,
            NoticeRequirement::AgmNotice
        ));
        assert!(!is_sufficient_notice(
            notice,
            meeting_10_days,
            NoticeRequirement::AgmNotice
        ));
    }

    #[test]
    fn test_calculate_annual_return_deadline() {
        let fye = Utc::now();
        let deadline = calculate_annual_return_deadline(fye);
        let days = (deadline - fye).num_days();
        assert_eq!(days, 213); // 7 months
    }
}
