//! Company Meetings and Resolutions
//!
//! This module implements company meetings and resolutions under CA 2006 Part 13.
//!
//! ## Types of Meeting
//!
//! ### Annual General Meeting (AGM)
//! - **Public companies**: Must hold AGM within 6 months of accounting reference date (s.336)
//! - **Private companies**: No requirement to hold AGM (s.336(1))
//!
//! ### General Meeting
//! - Directors may call at any time (s.302)
//! - Members with 5%+ voting rights can require (s.303)
//! - Members with 10%+ can call if directors fail (s.305)
//!
//! ## Notice Requirements (ss.307-313)
//!
//! ### Public Company
//! - AGM: 21 clear days' notice (s.307(2))
//! - Other GM: 14 clear days' notice (s.307(1))
//!
//! ### Private Company
//! - 14 clear days' notice (s.307(1))
//! - Can be shortened with consent of 90%+ by voting rights (s.307(5))
//!
//! ## Resolutions (ss.281-300)
//!
//! ### Ordinary Resolution (s.282)
//! - Simple majority (>50%)
//! - Used for most decisions
//!
//! ### Special Resolution (s.283)
//! - 75% majority
//! - Required for: changing articles, changing name, reducing capital,
//!   winding up voluntarily, disapplying pre-emption
//!
//! ### Written Resolutions (ss.288-300)
//! - Private companies only
//! - Same majorities as at meeting
//! - Cannot be used to remove director/auditor
//!
//! ## Quorum (s.318)
//!
//! - Default: 2 qualifying persons
//! - Single-member company: 1 qualifying person

use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

use super::types::CompanyType;

// ============================================================================
// Meeting Types
// ============================================================================

/// Type of company meeting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeetingType {
    /// Annual General Meeting (public companies must hold)
    AnnualGeneralMeeting,
    /// General Meeting (any meeting other than AGM)
    GeneralMeeting,
    /// Class meeting (meeting of specific share class)
    ClassMeeting,
    /// Board meeting (directors only)
    BoardMeeting,
}

impl MeetingType {
    /// Get required notice period in clear days
    pub fn required_notice_days(&self, company_type: CompanyType) -> u32 {
        match self {
            Self::AnnualGeneralMeeting => 21, // s.307(2)
            Self::GeneralMeeting | Self::ClassMeeting => {
                match company_type {
                    CompanyType::PublicLimitedCompany => 14, // s.307(1)
                    _ => 14,                                 // s.307(1)
                }
            }
            Self::BoardMeeting => 0, // As per articles
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::AnnualGeneralMeeting => {
                "Annual General Meeting - required for public companies within 6 months \
                 of accounting reference date (s.336)"
            }
            Self::GeneralMeeting => "General Meeting - may be called at any time by directors",
            Self::ClassMeeting => "Meeting of holders of specific class of shares",
            Self::BoardMeeting => "Board meeting of directors",
        }
    }
}

// ============================================================================
// Meeting Notice
// ============================================================================

/// Meeting notice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingNotice {
    /// Meeting type
    pub meeting_type: MeetingType,
    /// Company type
    pub company_type: CompanyType,
    /// Date notice sent
    pub notice_sent: NaiveDate,
    /// Meeting date
    pub meeting_date: NaiveDate,
    /// Meeting time
    pub meeting_time: String,
    /// Location (or virtual meeting details)
    pub location: MeetingLocation,
    /// Business to be conducted
    pub business: Vec<MeetingBusiness>,
    /// Special resolution(s) to be proposed
    pub special_resolutions: Vec<String>,
    /// Who sent notice
    pub sent_by: NoticeSender,
    /// Notice validity
    pub valid: bool,
    /// Analysis
    pub analysis: String,
}

/// Location of meeting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeetingLocation {
    /// Physical location
    Physical {
        /// Address
        address: String,
    },
    /// Virtual/online meeting
    Virtual {
        /// Platform
        platform: String,
        /// Access details
        access_details: String,
    },
    /// Hybrid (physical + virtual)
    Hybrid {
        /// Physical address
        address: String,
        /// Virtual platform
        platform: String,
    },
}

/// Business to be conducted at meeting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingBusiness {
    /// Item number
    pub item_number: u32,
    /// Description
    pub description: String,
    /// Resolution type required
    pub resolution_type: ResolutionType,
    /// Is this routine AGM business?
    pub routine_agm_business: bool,
}

/// Who sent the meeting notice
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoticeSender {
    /// Directors (s.302)
    Directors,
    /// Members with 5%+ (requisition under s.303)
    MemberRequisition,
    /// Members with 10%+ (where directors failed s.305)
    MembersDirect,
    /// Auditor (s.518 on ceasing to hold office)
    Auditor,
    /// Court (s.306)
    Court,
}

impl MeetingNotice {
    /// Calculate clear days between notice and meeting
    pub fn clear_days(&self) -> i64 {
        let days = (self.meeting_date - self.notice_sent).num_days();
        // Clear days excludes both day of sending and day of meeting
        days.saturating_sub(1)
    }

    /// Analyze notice validity
    pub fn analyze(
        meeting_type: MeetingType,
        company_type: CompanyType,
        notice_sent: NaiveDate,
        meeting_date: NaiveDate,
        short_notice_consent_percent: Option<f64>,
    ) -> NoticeValidity {
        let clear_days = (meeting_date - notice_sent).num_days() - 1;
        let required = meeting_type.required_notice_days(company_type) as i64;

        let short_notice_valid = short_notice_consent_percent
            .map(|p| p >= 90.0)
            .unwrap_or(false);

        let valid = clear_days >= required || short_notice_valid;

        let analysis = if valid {
            if clear_days >= required {
                format!(
                    "Notice VALID. {} clear days given (minimum {} required under s.307). \
                     Meeting type: {:?}.",
                    clear_days, required, meeting_type
                )
            } else {
                "Notice VALID by short notice consent. 90%+ by voting rights consented \
                 (s.307(5) private / s.307(6) public)."
                    .to_string()
            }
        } else {
            format!(
                "Notice INVALID. Only {} clear days given but {} required. \
                 Short notice consent not obtained or insufficient.",
                clear_days, required
            )
        };

        NoticeValidity {
            clear_days_given: clear_days,
            clear_days_required: required,
            short_notice_consent: short_notice_consent_percent,
            valid,
            analysis,
        }
    }
}

/// Notice validity result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoticeValidity {
    /// Clear days given
    pub clear_days_given: i64,
    /// Clear days required
    pub clear_days_required: i64,
    /// Short notice consent percentage (if any)
    pub short_notice_consent: Option<f64>,
    /// Is notice valid?
    pub valid: bool,
    /// Analysis
    pub analysis: String,
}

// ============================================================================
// Resolutions
// ============================================================================

/// Type of resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolutionType {
    /// Ordinary resolution - simple majority >50%
    Ordinary,
    /// Special resolution - 75% majority
    Special,
    /// Written resolution (private companies)
    WrittenOrdinary,
    /// Written special resolution (private companies)
    WrittenSpecial,
    /// Elective resolution (private companies - certain opt-outs)
    Elective,
}

impl ResolutionType {
    /// Get required majority percentage
    pub fn required_majority(&self) -> f64 {
        match self {
            Self::Ordinary | Self::WrittenOrdinary => 50.0,
            Self::Special | Self::WrittenSpecial | Self::Elective => 75.0,
        }
    }

    /// Get description with section reference
    pub fn description(&self) -> &'static str {
        match self {
            Self::Ordinary => "Ordinary resolution - simple majority >50% (CA 2006 s.282)",
            Self::Special => "Special resolution - 75%+ majority (CA 2006 s.283)",
            Self::WrittenOrdinary => {
                "Written ordinary resolution - majority of eligible members (s.288)"
            }
            Self::WrittenSpecial => "Written special resolution - 75%+ of eligible members (s.288)",
            Self::Elective => "Elective resolution - 75%+ unanimous private company opt-out",
        }
    }

    /// Can this be passed as written resolution?
    pub fn can_be_written(&self) -> bool {
        matches!(
            self,
            Self::Ordinary | Self::Special | Self::WrittenOrdinary | Self::WrittenSpecial
        )
    }
}

/// Resolution being proposed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolution {
    /// Resolution text
    pub text: String,
    /// Resolution type
    pub resolution_type: ResolutionType,
    /// Resolution category
    pub category: ResolutionCategory,
    /// Proposed by
    pub proposed_by: String,
    /// Seconded by (if required)
    pub seconded_by: Option<String>,
}

/// Category of resolution business
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolutionCategory {
    /// Routine AGM business
    RoutineAgm,
    /// Director appointment/removal
    DirectorAppointment,
    /// Director removal (special procedure s.168)
    DirectorRemoval,
    /// Auditor appointment
    AuditorAppointment,
    /// Auditor removal (special procedure s.510)
    AuditorRemoval,
    /// Share capital matters
    ShareCapital,
    /// Change of articles
    ArticlesChange,
    /// Change of name
    NameChange,
    /// Winding up
    WindingUp,
    /// Other special business
    SpecialBusiness,
    /// Other ordinary business
    OrdinaryBusiness,
}

impl ResolutionCategory {
    /// Get required resolution type
    pub fn required_resolution_type(&self) -> ResolutionType {
        match self {
            Self::ArticlesChange | Self::NameChange | Self::WindingUp | Self::ShareCapital => {
                ResolutionType::Special
            }
            _ => ResolutionType::Ordinary,
        }
    }

    /// Does this require special notice?
    pub fn requires_special_notice(&self) -> bool {
        matches!(self, Self::DirectorRemoval | Self::AuditorRemoval)
    }
}

/// Voting record for resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VotingRecord {
    /// Resolution
    pub resolution: Resolution,
    /// Voting method
    pub method: VotingMethod,
    /// Votes for
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Abstentions
    pub abstentions: u64,
    /// Is resolution passed?
    pub passed: bool,
    /// Analysis
    pub analysis: String,
}

/// Method of voting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VotingMethod {
    /// Show of hands (one member one vote)
    ShowOfHands,
    /// Poll (votes according to shares held)
    Poll,
    /// Written resolution
    Written,
}

impl VotingRecord {
    /// Calculate and record voting result
    pub fn record(
        resolution: Resolution,
        method: VotingMethod,
        votes_for: u64,
        votes_against: u64,
        abstentions: u64,
    ) -> Self {
        let total_cast = votes_for + votes_against;
        let percentage_for = if total_cast > 0 {
            (votes_for as f64 / total_cast as f64) * 100.0
        } else {
            0.0
        };

        let required = resolution.resolution_type.required_majority();

        // For ordinary resolution need >50%, for special need >=75%
        let passed = match resolution.resolution_type {
            ResolutionType::Ordinary | ResolutionType::WrittenOrdinary => percentage_for > required,
            ResolutionType::Special | ResolutionType::WrittenSpecial | ResolutionType::Elective => {
                percentage_for >= required
            }
        };

        let analysis = format!(
            "{:?} resolution: {} votes for ({:.1}%), {} against, {} abstentions. \
             Required: {}{}%. Result: {}.",
            resolution.resolution_type,
            votes_for,
            percentage_for,
            votes_against,
            abstentions,
            if matches!(
                resolution.resolution_type,
                ResolutionType::Ordinary | ResolutionType::WrittenOrdinary
            ) {
                ">"
            } else {
                ">="
            },
            required,
            if passed { "PASSED" } else { "NOT PASSED" }
        );

        Self {
            resolution,
            method,
            votes_for,
            votes_against,
            abstentions,
            passed,
            analysis,
        }
    }
}

// ============================================================================
// Quorum
// ============================================================================

/// Quorum requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuorumRequirement {
    /// Meeting type
    pub meeting_type: MeetingType,
    /// Required number of qualifying persons
    pub required_persons: u32,
    /// Is company single-member?
    pub single_member: bool,
    /// Persons present
    pub persons_present: u32,
    /// Is quorum met?
    pub quorum_met: bool,
    /// Analysis
    pub analysis: String,
}

impl QuorumRequirement {
    /// Check if quorum is met
    pub fn check(meeting_type: MeetingType, single_member: bool, persons_present: u32) -> Self {
        let required = if single_member { 1 } else { 2 };
        let quorum_met = persons_present >= required;

        let analysis = if quorum_met {
            format!(
                "Quorum MET. {} qualifying person(s) present (minimum {} required under s.318). \
                 Meeting can proceed.",
                persons_present, required
            )
        } else {
            format!(
                "Quorum NOT MET. Only {} present, {} required (s.318). \
                 Meeting cannot proceed - must be adjourned.",
                persons_present, required
            )
        };

        Self {
            meeting_type,
            required_persons: required,
            single_member,
            persons_present,
            quorum_met,
            analysis,
        }
    }
}

// ============================================================================
// Proxy Voting
// ============================================================================

/// Proxy appointment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProxyAppointment {
    /// Member appointing proxy
    pub member: String,
    /// Proxy appointed
    pub proxy: String,
    /// Is proxy a member?
    pub proxy_is_member: bool,
    /// Specific instructions
    pub instructions: ProxyInstructions,
    /// Date of appointment
    pub appointment_date: NaiveDate,
    /// Valid until
    pub valid_until: Option<NaiveDate>,
    /// Is appointment valid?
    pub valid: bool,
    /// Analysis
    pub analysis: String,
}

/// Proxy voting instructions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProxyInstructions {
    /// Vote as proxy sees fit (discretionary)
    Discretionary,
    /// Vote for all resolutions
    VoteForAll,
    /// Vote against all resolutions
    VoteAgainstAll,
    /// Specific instructions per resolution
    Specific {
        /// Resolution number to instruction
        instructions: Vec<(u32, ProxyVote)>,
    },
}

/// Individual proxy vote instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProxyVote {
    /// Vote for
    For,
    /// Vote against
    Against,
    /// Abstain
    Abstain,
    /// Discretion
    Discretion,
}

impl ProxyAppointment {
    /// Analyze proxy appointment validity
    pub fn analyze(member: &str, proxy: &str, appointment_date: NaiveDate) -> Self {
        // Under s.324, every member has right to appoint proxy
        // Proxy need not be member (s.324(1))

        let analysis = format!(
            "Proxy appointment VALID. {} appointed {} as proxy. Under CA 2006 s.324, \
             every member has right to appoint proxy to attend, speak and vote. \
             Proxy need not be member. Form must be received 48 hours before meeting.",
            member, proxy
        );

        Self {
            member: member.to_string(),
            proxy: proxy.to_string(),
            proxy_is_member: false,
            instructions: ProxyInstructions::Discretionary,
            appointment_date,
            valid_until: None,
            valid: true,
            analysis,
        }
    }
}

// ============================================================================
// Written Resolutions (Private Companies)
// ============================================================================

/// Written resolution (CA 2006 ss.288-300)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WrittenResolution {
    /// Resolution text
    pub text: String,
    /// Resolution type (ordinary or special)
    pub resolution_type: ResolutionType,
    /// Date proposed
    pub proposed_date: NaiveDate,
    /// Circulation date
    pub circulation_date: NaiveDate,
    /// Lapse date (28 days from circulation)
    pub lapse_date: NaiveDate,
    /// Eligible members
    pub eligible_members: Vec<EligibleMember>,
    /// Members who agreed
    pub agreed: Vec<String>,
    /// Total eligible votes
    pub total_eligible_votes: u64,
    /// Votes in favor
    pub votes_in_favor: u64,
    /// Is resolution passed?
    pub passed: bool,
    /// Analysis
    pub analysis: String,
}

/// Eligible member for written resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EligibleMember {
    /// Member name
    pub name: String,
    /// Voting rights
    pub votes: u64,
    /// Has agreed
    pub agreed: bool,
    /// Date of agreement
    pub agreement_date: Option<NaiveDate>,
}

impl WrittenResolution {
    /// Analyze written resolution
    pub fn analyze(
        text: &str,
        resolution_type: ResolutionType,
        proposed_date: NaiveDate,
        total_votes: u64,
        votes_in_favor: u64,
    ) -> Self {
        let circulation_date = proposed_date;
        let lapse_date = proposed_date + Duration::days(28);

        let percentage = if total_votes > 0 {
            (votes_in_favor as f64 / total_votes as f64) * 100.0
        } else {
            0.0
        };

        let required = resolution_type.required_majority();
        let passed = match resolution_type {
            ResolutionType::WrittenOrdinary | ResolutionType::Ordinary => percentage > required,
            _ => percentage >= required,
        };

        let analysis = if passed {
            format!(
                "Written resolution PASSED. {:.1}% voted in favor ({}{}% required). \
                 CA 2006 ss.288-300 requirements satisfied.",
                percentage,
                if matches!(
                    resolution_type,
                    ResolutionType::WrittenOrdinary | ResolutionType::Ordinary
                ) {
                    ">"
                } else {
                    ">="
                },
                required
            )
        } else {
            format!(
                "Written resolution NOT PASSED. Only {:.1}% voted in favor ({}{}% required). \
                 Lapses {} if threshold not reached.",
                percentage,
                if matches!(
                    resolution_type,
                    ResolutionType::WrittenOrdinary | ResolutionType::Ordinary
                ) {
                    ">"
                } else {
                    ">="
                },
                required,
                lapse_date
            )
        };

        Self {
            text: text.to_string(),
            resolution_type,
            proposed_date,
            circulation_date,
            lapse_date,
            eligible_members: vec![],
            agreed: vec![],
            total_eligible_votes: total_votes,
            votes_in_favor,
            passed,
            analysis,
        }
    }

    /// Cannot use written resolution for these matters (s.288(2))
    pub fn is_prohibited_matter(category: ResolutionCategory) -> bool {
        matches!(
            category,
            ResolutionCategory::DirectorRemoval | ResolutionCategory::AuditorRemoval
        )
    }
}

// ============================================================================
// Special Notice (s.312)
// ============================================================================

/// Special notice requirement (28 days)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialNotice {
    /// Resolution requiring special notice
    pub resolution: String,
    /// Category
    pub category: ResolutionCategory,
    /// Date special notice given to company
    pub notice_date: NaiveDate,
    /// Meeting date
    pub meeting_date: NaiveDate,
    /// Is 28 days notice satisfied?
    pub notice_satisfied: bool,
    /// Analysis
    pub analysis: String,
}

impl SpecialNotice {
    /// Check if special notice requirement satisfied
    pub fn check(
        resolution: &str,
        category: ResolutionCategory,
        notice_date: NaiveDate,
        meeting_date: NaiveDate,
    ) -> Self {
        let days_notice = (meeting_date - notice_date).num_days();
        let notice_satisfied = days_notice >= 28;

        let analysis = if !category.requires_special_notice() {
            format!("Special notice not required for {:?}.", category)
        } else if notice_satisfied {
            format!(
                "Special notice requirement SATISFIED. {} days notice given (28 required). \
                 CA 2006 s.312.",
                days_notice
            )
        } else {
            format!(
                "Special notice requirement NOT SATISFIED. Only {} days notice (28 required). \
                 Resolution cannot be moved.",
                days_notice
            )
        };

        Self {
            resolution: resolution.to_string(),
            category,
            notice_date,
            meeting_date,
            notice_satisfied,
            analysis,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notice_validity_agm() {
        let validity = MeetingNotice::analyze(
            MeetingType::AnnualGeneralMeeting,
            CompanyType::PublicLimitedCompany,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 23).unwrap(),
            None,
        );
        // 22 days total, 21 clear days - valid
        assert!(validity.valid);
        assert_eq!(validity.clear_days_given, 21);
    }

    #[test]
    fn test_notice_validity_insufficient() {
        let validity = MeetingNotice::analyze(
            MeetingType::AnnualGeneralMeeting,
            CompanyType::PublicLimitedCompany,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            None,
        );
        assert!(!validity.valid);
        assert!(validity.analysis.contains("INVALID"));
    }

    #[test]
    fn test_notice_short_notice_consent() {
        let validity = MeetingNotice::analyze(
            MeetingType::GeneralMeeting,
            CompanyType::PrivateLimitedByShares,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            Some(95.0),
        );
        assert!(validity.valid);
        assert!(validity.analysis.contains("short notice consent"));
    }

    #[test]
    fn test_ordinary_resolution_passes() {
        let resolution = Resolution {
            text: "Approve accounts".to_string(),
            resolution_type: ResolutionType::Ordinary,
            category: ResolutionCategory::RoutineAgm,
            proposed_by: "Chair".to_string(),
            seconded_by: None,
        };
        let record = VotingRecord::record(resolution, VotingMethod::Poll, 51, 49, 10);
        assert!(record.passed);
    }

    #[test]
    fn test_ordinary_resolution_fails_at_50() {
        let resolution = Resolution {
            text: "Approve accounts".to_string(),
            resolution_type: ResolutionType::Ordinary,
            category: ResolutionCategory::RoutineAgm,
            proposed_by: "Chair".to_string(),
            seconded_by: None,
        };
        let record = VotingRecord::record(resolution, VotingMethod::Poll, 50, 50, 0);
        assert!(!record.passed);
    }

    #[test]
    fn test_special_resolution_passes_at_75() {
        let resolution = Resolution {
            text: "Change articles".to_string(),
            resolution_type: ResolutionType::Special,
            category: ResolutionCategory::ArticlesChange,
            proposed_by: "Director".to_string(),
            seconded_by: Some("Member".to_string()),
        };
        let record = VotingRecord::record(resolution, VotingMethod::Poll, 75, 25, 0);
        assert!(record.passed);
    }

    #[test]
    fn test_special_resolution_fails_at_74() {
        let resolution = Resolution {
            text: "Change name".to_string(),
            resolution_type: ResolutionType::Special,
            category: ResolutionCategory::NameChange,
            proposed_by: "Director".to_string(),
            seconded_by: None,
        };
        let record = VotingRecord::record(resolution, VotingMethod::Poll, 74, 26, 0);
        assert!(!record.passed);
    }

    #[test]
    fn test_quorum_met() {
        let quorum = QuorumRequirement::check(MeetingType::GeneralMeeting, false, 3);
        assert!(quorum.quorum_met);
        assert_eq!(quorum.required_persons, 2);
    }

    #[test]
    fn test_quorum_single_member() {
        let quorum = QuorumRequirement::check(MeetingType::GeneralMeeting, true, 1);
        assert!(quorum.quorum_met);
        assert_eq!(quorum.required_persons, 1);
    }

    #[test]
    fn test_written_resolution_passed() {
        let wr = WrittenResolution::analyze(
            "Approve bonus issue",
            ResolutionType::WrittenOrdinary,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            100,
            60,
        );
        assert!(wr.passed);
    }

    #[test]
    fn test_special_notice_satisfied() {
        let notice = SpecialNotice::check(
            "Remove director X",
            ResolutionCategory::DirectorRemoval,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 30).unwrap(),
        );
        assert!(notice.notice_satisfied);
    }
}
