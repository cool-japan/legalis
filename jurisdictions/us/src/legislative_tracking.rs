//! Legislative Tracking System
//!
//! This module provides comprehensive tracking of legislative bills, amendments, committees,
//! and sessions across all 50 states plus DC.
//!
//! ## Features
//!
//! - **Bill Tracking**: Track bills through the legislative process from introduction to enactment
//! - **Legislative Calendar**: Session tracking with deadlines and recess periods
//! - **Amendment Tracking**: Monitor amendments to bills through committee and floor stages
//! - **Committee System**: Track committee assignments, hearings, and reports
//! - **State Comparison**: Compare similar legislation across multiple states
//!
//! ## Example: Bill Tracking
//!
//! ```rust
//! use legalis_us::legislative_tracking::{Bill, BillStatus, Chamber};
//! use chrono::NaiveDate;
//!
//! let bill = Bill::builder()
//!     .state("CA")
//!     .chamber(Chamber::House)
//!     .number("AB 123")
//!     .title("Data Privacy Protection Act")
//!     .status(BillStatus::Introduced)
//!     .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
//!     .build();
//!
//! assert_eq!(bill.state_code, "CA");
//! assert_eq!(bill.number, "AB 123");
//! ```
//!
//! ## Example: Legislative Session
//!
//! ```rust
//! use legalis_us::legislative_tracking::{LegislativeSession, SessionType};
//! use chrono::NaiveDate;
//!
//! let session = LegislativeSession::builder()
//!     .state("NY")
//!     .session_type(SessionType::Regular)
//!     .year(2024)
//!     .start_date(NaiveDate::from_ymd_opt(2024, 1, 3).unwrap())
//!     .end_date(NaiveDate::from_ymd_opt(2024, 6, 20).unwrap())
//!     .build();
//!
//! assert_eq!(session.year, 2024);
//! ```

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== Bill Tracking Module ====================

/// Chamber where a bill originates or is being considered
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chamber {
    /// State Senate (upper chamber)
    Senate,
    /// State House/Assembly (lower chamber)
    House,
    /// Joint resolution or concurrent resolution
    Joint,
}

impl Chamber {
    /// Returns the typical prefix for bills in this chamber
    pub fn bill_prefix(&self, state: &str) -> &'static str {
        match (self, state) {
            (Chamber::Senate, _) => "SB",
            (Chamber::House, "CA" | "NY" | "WI") => "AB", // Assembly states
            (Chamber::House, _) => "HB",                  // House states
            (Chamber::Joint, _) => "JR",
        }
    }

    /// Returns the full name of the chamber for a given state
    pub fn full_name(&self, state: &str) -> &'static str {
        match (self, state) {
            (Chamber::Senate, _) => "Senate",
            (Chamber::House, "CA" | "NY" | "WI") => "Assembly",
            (Chamber::House, _) => "House of Representatives",
            (Chamber::Joint, _) => "Joint Chamber",
        }
    }
}

/// Status of a bill in the legislative process
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BillStatus {
    /// Bill has been introduced but not yet assigned to committee
    Introduced,
    /// Bill assigned to committee for review
    InCommittee { committee: String },
    /// Committee has reported the bill favorably
    ReportedFromCommittee { committee: String },
    /// Bill is on the chamber floor for debate and vote
    OnFloor { chamber: Chamber },
    /// Bill passed one chamber and sent to the other
    PassedFirstChamber { chamber: Chamber },
    /// Bill passed both chambers
    Passed,
    /// Bill sent to governor/executive for signature
    SentToGovernor { date: NaiveDate },
    /// Bill signed into law
    Enacted {
        date: NaiveDate,
        chapter: Option<String>,
    },
    /// Bill vetoed by governor
    Vetoed {
        date: NaiveDate,
        reason: Option<String>,
    },
    /// Veto was overridden by legislature
    VetoOverridden { date: NaiveDate },
    /// Bill failed to pass
    Failed { reason: String },
    /// Bill withdrawn by sponsor
    Withdrawn,
    /// Bill tabled (postponed indefinitely)
    Tabled,
}

impl BillStatus {
    /// Returns true if the bill is still active in the legislative process
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            BillStatus::Introduced
                | BillStatus::InCommittee { .. }
                | BillStatus::ReportedFromCommittee { .. }
                | BillStatus::OnFloor { .. }
                | BillStatus::PassedFirstChamber { .. }
                | BillStatus::Passed
                | BillStatus::SentToGovernor { .. }
        )
    }

    /// Returns true if the bill became law
    pub fn is_enacted(&self) -> bool {
        matches!(
            self,
            BillStatus::Enacted { .. } | BillStatus::VetoOverridden { .. }
        )
    }

    /// Returns true if the bill is dead (failed, vetoed without override, etc.)
    pub fn is_dead(&self) -> bool {
        matches!(
            self,
            BillStatus::Failed { .. }
                | BillStatus::Vetoed { .. }
                | BillStatus::Withdrawn
                | BillStatus::Tabled
        )
    }
}

/// Priority level for a bill
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BillPriority {
    /// Emergency legislation
    Emergency,
    /// High priority
    High,
    /// Normal priority
    #[default]
    Normal,
    /// Low priority
    Low,
}

/// Legislator who sponsors or co-sponsors a bill
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Legislator {
    /// Legislator's name
    pub name: String,
    /// Party affiliation (D, R, I, etc.)
    pub party: Option<String>,
    /// Chamber (Senate or House)
    pub chamber: Chamber,
    /// District number
    pub district: Option<u32>,
}

impl Legislator {
    /// Creates a new legislator
    pub fn new(name: String, chamber: Chamber) -> Self {
        Self {
            name,
            party: None,
            chamber,
            district: None,
        }
    }

    /// Sets the party affiliation
    pub fn with_party(mut self, party: String) -> Self {
        self.party = Some(party);
        self
    }

    /// Sets the district number
    pub fn with_district(mut self, district: u32) -> Self {
        self.district = Some(district);
        self
    }
}

/// A legislative bill being tracked through the process
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bill {
    /// State code (e.g., "CA", "NY", "TX")
    pub state_code: String,
    /// Chamber where bill originated
    pub chamber: Chamber,
    /// Bill number (e.g., "AB 123", "SB 456")
    pub number: String,
    /// Short title of the bill
    pub title: String,
    /// Full text summary (optional)
    pub summary: Option<String>,
    /// Current status
    pub status: BillStatus,
    /// Date introduced
    pub introduced_date: NaiveDate,
    /// Primary sponsor
    pub sponsor: Option<Legislator>,
    /// Co-sponsors
    pub cosponsors: Vec<Legislator>,
    /// Legislative session identifier
    pub session: String,
    /// Priority level
    pub priority: BillPriority,
    /// Subject matter tags
    pub tags: Vec<String>,
    /// Related bill numbers (companion bills, etc.)
    pub related_bills: Vec<String>,
}

impl Bill {
    /// Creates a new bill builder
    pub fn builder() -> BillBuilder {
        BillBuilder::default()
    }

    /// Updates the status of the bill
    pub fn update_status(&mut self, new_status: BillStatus) {
        self.status = new_status;
    }

    /// Adds a co-sponsor to the bill
    pub fn add_cosponsor(&mut self, legislator: Legislator) {
        self.cosponsors.push(legislator);
    }

    /// Adds a tag to the bill
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Returns the full bill identifier (e.g., "CA AB 123")
    pub fn full_identifier(&self) -> String {
        format!("{} {}", self.state_code, self.number)
    }
}

/// Builder for constructing a Bill
#[derive(Default)]
pub struct BillBuilder {
    state_code: Option<String>,
    chamber: Option<Chamber>,
    number: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    status: Option<BillStatus>,
    introduced_date: Option<NaiveDate>,
    sponsor: Option<Legislator>,
    cosponsors: Vec<Legislator>,
    session: Option<String>,
    priority: BillPriority,
    tags: Vec<String>,
    related_bills: Vec<String>,
}

impl BillBuilder {
    /// Sets the state code
    pub fn state(mut self, state_code: &str) -> Self {
        self.state_code = Some(state_code.to_string());
        self
    }

    /// Sets the chamber
    pub fn chamber(mut self, chamber: Chamber) -> Self {
        self.chamber = Some(chamber);
        self
    }

    /// Sets the bill number
    pub fn number(mut self, number: &str) -> Self {
        self.number = Some(number.to_string());
        self
    }

    /// Sets the title
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the summary
    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_string());
        self
    }

    /// Sets the status
    pub fn status(mut self, status: BillStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the introduced date
    pub fn introduced_date(mut self, date: NaiveDate) -> Self {
        self.introduced_date = Some(date);
        self
    }

    /// Sets the sponsor
    pub fn sponsor(mut self, sponsor: Legislator) -> Self {
        self.sponsor = Some(sponsor);
        self
    }

    /// Adds a co-sponsor
    pub fn cosponsor(mut self, cosponsor: Legislator) -> Self {
        self.cosponsors.push(cosponsor);
        self
    }

    /// Sets the session
    pub fn session(mut self, session: &str) -> Self {
        self.session = Some(session.to_string());
        self
    }

    /// Sets the priority
    pub fn priority(mut self, priority: BillPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Adds a tag
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Adds a related bill
    pub fn related_bill(mut self, bill: &str) -> Self {
        self.related_bills.push(bill.to_string());
        self
    }

    /// Builds the Bill
    pub fn build(self) -> Bill {
        Bill {
            state_code: self.state_code.expect("state_code is required"),
            chamber: self.chamber.expect("chamber is required"),
            number: self.number.expect("number is required"),
            title: self.title.expect("title is required"),
            summary: self.summary,
            status: self.status.unwrap_or(BillStatus::Introduced),
            introduced_date: self.introduced_date.expect("introduced_date is required"),
            sponsor: self.sponsor,
            cosponsors: self.cosponsors,
            session: self
                .session
                .unwrap_or_else(|| format!("{}", Utc::now().format("%Y"))),
            priority: self.priority,
            tags: self.tags,
            related_bills: self.related_bills,
        }
    }
}

// ==================== Legislative Calendar ====================

/// Type of legislative session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionType {
    /// Regular annual session
    Regular,
    /// Special/extraordinary session called by governor
    Special,
    /// Extraordinary session called by legislature
    Extraordinary,
    /// Veto session (to consider veto overrides)
    Veto,
}

/// A legislative session with scheduling information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegislativeSession {
    /// State code
    pub state_code: String,
    /// Session identifier (e.g., "2024 Regular")
    pub session_id: String,
    /// Session type
    pub session_type: SessionType,
    /// Year of the session
    pub year: u32,
    /// Start date
    pub start_date: NaiveDate,
    /// End date (or scheduled end date)
    pub end_date: NaiveDate,
    /// Key deadlines (e.g., committee, floor vote, crossover)
    pub deadlines: HashMap<String, NaiveDate>,
    /// Recess periods
    pub recesses: Vec<(NaiveDate, NaiveDate)>,
}

impl LegislativeSession {
    /// Creates a new session builder
    pub fn builder() -> LegislativeSessionBuilder {
        LegislativeSessionBuilder::default()
    }

    /// Returns true if the session is currently active
    pub fn is_active(&self) -> bool {
        let today = Utc::now().date_naive();
        today >= self.start_date && today <= self.end_date
    }

    /// Returns true if a given date is during a recess
    pub fn is_in_recess(&self, date: NaiveDate) -> bool {
        self.recesses
            .iter()
            .any(|(start, end)| date >= *start && date <= *end)
    }

    /// Returns the deadline for a given milestone (e.g., "committee", "floor_vote")
    pub fn deadline(&self, milestone: &str) -> Option<NaiveDate> {
        self.deadlines.get(milestone).copied()
    }

    /// Adds or updates a deadline
    pub fn set_deadline(&mut self, milestone: String, date: NaiveDate) {
        self.deadlines.insert(milestone, date);
    }

    /// Adds a recess period
    pub fn add_recess(&mut self, start: NaiveDate, end: NaiveDate) {
        self.recesses.push((start, end));
    }
}

/// Builder for constructing a LegislativeSession
#[derive(Default)]
pub struct LegislativeSessionBuilder {
    state_code: Option<String>,
    session_id: Option<String>,
    session_type: Option<SessionType>,
    year: Option<u32>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    deadlines: HashMap<String, NaiveDate>,
    recesses: Vec<(NaiveDate, NaiveDate)>,
}

impl LegislativeSessionBuilder {
    /// Sets the state code
    pub fn state(mut self, state_code: &str) -> Self {
        self.state_code = Some(state_code.to_string());
        self
    }

    /// Sets the session ID
    pub fn session_id(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }

    /// Sets the session type
    pub fn session_type(mut self, session_type: SessionType) -> Self {
        self.session_type = Some(session_type);
        self
    }

    /// Sets the year
    pub fn year(mut self, year: u32) -> Self {
        self.year = Some(year);
        self
    }

    /// Sets the start date
    pub fn start_date(mut self, date: NaiveDate) -> Self {
        self.start_date = Some(date);
        self
    }

    /// Sets the end date
    pub fn end_date(mut self, date: NaiveDate) -> Self {
        self.end_date = Some(date);
        self
    }

    /// Adds a deadline
    pub fn deadline(mut self, milestone: &str, date: NaiveDate) -> Self {
        self.deadlines.insert(milestone.to_string(), date);
        self
    }

    /// Adds a recess period
    pub fn recess(mut self, start: NaiveDate, end: NaiveDate) -> Self {
        self.recesses.push((start, end));
        self
    }

    /// Builds the LegislativeSession
    pub fn build(self) -> LegislativeSession {
        let state_code = self.state_code.expect("state_code is required");
        let year = self.year.expect("year is required");
        let session_type = self.session_type.unwrap_or(SessionType::Regular);

        let session_id = self.session_id.unwrap_or_else(|| {
            format!(
                "{} {}",
                year,
                match session_type {
                    SessionType::Regular => "Regular",
                    SessionType::Special => "Special",
                    SessionType::Extraordinary => "Extraordinary",
                    SessionType::Veto => "Veto",
                }
            )
        });

        LegislativeSession {
            state_code,
            session_id,
            session_type,
            year,
            start_date: self.start_date.expect("start_date is required"),
            end_date: self.end_date.expect("end_date is required"),
            deadlines: self.deadlines,
            recesses: self.recesses,
        }
    }
}

// ==================== Amendment Tracking ====================

/// Type of amendment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmendmentType {
    /// Amendment proposed in committee
    Committee,
    /// Amendment proposed on the floor
    Floor,
    /// Conference committee amendment (reconciling House/Senate versions)
    Conference,
    /// Technical/non-substantive amendment
    Technical,
}

/// Status of an amendment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmendmentStatus {
    /// Amendment has been proposed
    Proposed,
    /// Amendment adopted
    Adopted,
    /// Amendment rejected
    Rejected,
    /// Amendment withdrawn
    Withdrawn,
}

/// An amendment to a bill
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Amendment {
    /// Amendment identifier (e.g., "AM 001")
    pub id: String,
    /// Bill this amendment modifies
    pub bill_number: String,
    /// Type of amendment
    pub amendment_type: AmendmentType,
    /// Status
    pub status: AmendmentStatus,
    /// Date proposed
    pub proposed_date: NaiveDate,
    /// Sponsor of the amendment
    pub sponsor: Option<Legislator>,
    /// Summary of changes
    pub summary: String,
    /// Text comparison (optional - shows before/after)
    pub text_comparison: Option<String>,
}

impl Amendment {
    /// Creates a new amendment builder
    pub fn builder() -> AmendmentBuilder {
        AmendmentBuilder::default()
    }

    /// Returns true if the amendment was adopted
    pub fn is_adopted(&self) -> bool {
        self.status == AmendmentStatus::Adopted
    }
}

/// Builder for constructing an Amendment
#[derive(Default)]
pub struct AmendmentBuilder {
    id: Option<String>,
    bill_number: Option<String>,
    amendment_type: Option<AmendmentType>,
    status: Option<AmendmentStatus>,
    proposed_date: Option<NaiveDate>,
    sponsor: Option<Legislator>,
    summary: Option<String>,
    text_comparison: Option<String>,
}

impl AmendmentBuilder {
    /// Sets the amendment ID
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Sets the bill number
    pub fn bill_number(mut self, bill_number: &str) -> Self {
        self.bill_number = Some(bill_number.to_string());
        self
    }

    /// Sets the amendment type
    pub fn amendment_type(mut self, amendment_type: AmendmentType) -> Self {
        self.amendment_type = Some(amendment_type);
        self
    }

    /// Sets the status
    pub fn status(mut self, status: AmendmentStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the proposed date
    pub fn proposed_date(mut self, date: NaiveDate) -> Self {
        self.proposed_date = Some(date);
        self
    }

    /// Sets the sponsor
    pub fn sponsor(mut self, sponsor: Legislator) -> Self {
        self.sponsor = Some(sponsor);
        self
    }

    /// Sets the summary
    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_string());
        self
    }

    /// Sets the text comparison
    pub fn text_comparison(mut self, text_comparison: &str) -> Self {
        self.text_comparison = Some(text_comparison.to_string());
        self
    }

    /// Builds the Amendment
    pub fn build(self) -> Amendment {
        Amendment {
            id: self.id.expect("id is required"),
            bill_number: self.bill_number.expect("bill_number is required"),
            amendment_type: self.amendment_type.unwrap_or(AmendmentType::Floor),
            status: self.status.unwrap_or(AmendmentStatus::Proposed),
            proposed_date: self.proposed_date.expect("proposed_date is required"),
            sponsor: self.sponsor,
            summary: self.summary.expect("summary is required"),
            text_comparison: self.text_comparison,
        }
    }
}

// ==================== Committee System ====================

/// Type of legislative committee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommitteeType {
    /// Standing committee (permanent)
    Standing,
    /// Select/special committee (temporary)
    Select,
    /// Joint committee (House + Senate)
    Joint,
    /// Conference committee (reconciling versions)
    Conference,
}

/// A legislative committee
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Committee {
    /// Committee identifier
    pub id: String,
    /// Committee name
    pub name: String,
    /// State code
    pub state_code: String,
    /// Chamber (or Joint)
    pub chamber: Option<Chamber>,
    /// Committee type
    pub committee_type: CommitteeType,
    /// Committee chair
    pub chair: Option<Legislator>,
    /// Committee members
    pub members: Vec<Legislator>,
    /// Subject matter jurisdiction
    pub jurisdiction: Vec<String>,
}

impl Committee {
    /// Creates a new committee builder
    pub fn builder() -> CommitteeBuilder {
        CommitteeBuilder::default()
    }

    /// Returns true if this committee has jurisdiction over the given subject
    pub fn has_jurisdiction(&self, subject: &str) -> bool {
        self.jurisdiction
            .iter()
            .any(|j| j.to_lowercase().contains(&subject.to_lowercase()))
    }

    /// Adds a member to the committee
    pub fn add_member(&mut self, legislator: Legislator) {
        if !self.members.contains(&legislator) {
            self.members.push(legislator);
        }
    }
}

/// Builder for constructing a Committee
#[derive(Default)]
pub struct CommitteeBuilder {
    id: Option<String>,
    name: Option<String>,
    state_code: Option<String>,
    chamber: Option<Chamber>,
    committee_type: Option<CommitteeType>,
    chair: Option<Legislator>,
    members: Vec<Legislator>,
    jurisdiction: Vec<String>,
}

impl CommitteeBuilder {
    /// Sets the committee ID
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Sets the committee name
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the state code
    pub fn state(mut self, state_code: &str) -> Self {
        self.state_code = Some(state_code.to_string());
        self
    }

    /// Sets the chamber
    pub fn chamber(mut self, chamber: Chamber) -> Self {
        self.chamber = Some(chamber);
        self
    }

    /// Sets the committee type
    pub fn committee_type(mut self, committee_type: CommitteeType) -> Self {
        self.committee_type = Some(committee_type);
        self
    }

    /// Sets the chair
    pub fn chair(mut self, chair: Legislator) -> Self {
        self.chair = Some(chair);
        self
    }

    /// Adds a member
    pub fn member(mut self, member: Legislator) -> Self {
        self.members.push(member);
        self
    }

    /// Adds jurisdiction
    pub fn jurisdiction(mut self, subject: &str) -> Self {
        self.jurisdiction.push(subject.to_string());
        self
    }

    /// Builds the Committee
    pub fn build(self) -> Committee {
        Committee {
            id: self.id.expect("id is required"),
            name: self.name.expect("name is required"),
            state_code: self.state_code.expect("state_code is required"),
            chamber: self.chamber,
            committee_type: self.committee_type.unwrap_or(CommitteeType::Standing),
            chair: self.chair,
            members: self.members,
            jurisdiction: self.jurisdiction,
        }
    }
}

/// A committee hearing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitteeHearing {
    /// Committee holding the hearing
    pub committee_id: String,
    /// Bill(s) being considered
    pub bills: Vec<String>,
    /// Date and time of hearing
    pub date: NaiveDate,
    /// Location
    pub location: Option<String>,
    /// Witnesses scheduled to testify
    pub witnesses: Vec<String>,
    /// Outcome (if completed)
    pub outcome: Option<String>,
}

impl CommitteeHearing {
    /// Creates a new hearing
    pub fn new(committee_id: String, date: NaiveDate) -> Self {
        Self {
            committee_id,
            bills: Vec::new(),
            date,
            location: None,
            witnesses: Vec::new(),
            outcome: None,
        }
    }

    /// Adds a bill to be considered
    pub fn add_bill(&mut self, bill_number: String) {
        self.bills.push(bill_number);
    }

    /// Adds a witness
    pub fn add_witness(&mut self, witness: String) {
        self.witnesses.push(witness);
    }

    /// Sets the outcome
    pub fn set_outcome(&mut self, outcome: String) {
        self.outcome = Some(outcome);
    }
}

/// A committee report on a bill
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitteeReport {
    /// Committee issuing the report
    pub committee_id: String,
    /// Bill number
    pub bill_number: String,
    /// Date of report
    pub report_date: NaiveDate,
    /// Recommendation (e.g., "Do Pass", "Do Not Pass", "Do Pass as Amended")
    pub recommendation: String,
    /// Vote tally (yeas, nays, abstentions)
    pub vote_tally: Option<(u32, u32, u32)>,
    /// Report text/summary
    pub summary: Option<String>,
}

impl CommitteeReport {
    /// Creates a new committee report
    pub fn new(committee_id: String, bill_number: String, report_date: NaiveDate) -> Self {
        Self {
            committee_id,
            bill_number,
            report_date,
            recommendation: String::new(),
            vote_tally: None,
            summary: None,
        }
    }

    /// Sets the recommendation
    pub fn with_recommendation(mut self, recommendation: String) -> Self {
        self.recommendation = recommendation;
        self
    }

    /// Sets the vote tally (yeas, nays, abstentions)
    pub fn with_vote_tally(mut self, yeas: u32, nays: u32, abstentions: u32) -> Self {
        self.vote_tally = Some((yeas, nays, abstentions));
        self
    }

    /// Sets the summary
    pub fn with_summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Returns true if the committee recommended passage
    pub fn recommends_passage(&self) -> bool {
        self.recommendation.to_lowercase().contains("do pass")
    }
}

// ==================== State Comparison Dashboard ====================

/// Similarity score between two bills
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillSimilarity {
    /// First bill
    pub bill1: String,
    /// Second bill
    pub bill2: String,
    /// Similarity score (0.0 to 1.0)
    pub score: f64,
    /// Common elements
    pub common_elements: Vec<String>,
}

impl BillSimilarity {
    /// Creates a new similarity comparison
    pub fn new(bill1: String, bill2: String, score: f64) -> Self {
        Self {
            bill1,
            bill2,
            score,
            common_elements: Vec::new(),
        }
    }

    /// Adds a common element
    pub fn add_common_element(&mut self, element: String) {
        self.common_elements.push(element);
    }

    /// Returns true if bills are highly similar (>= 0.75)
    pub fn is_highly_similar(&self) -> bool {
        self.score >= 0.75
    }
}

/// Tracks adoption of uniform/model legislation across states
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniformLawAdoption {
    /// Name of the uniform law
    pub name: String,
    /// States that have adopted this uniform law
    pub adopted_states: Vec<String>,
    /// Year first proposed/published
    pub year_proposed: u32,
    /// Organization that published it (e.g., "ULC", "NCCUSL")
    pub publisher: String,
    /// State-specific variations
    pub variations: HashMap<String, Vec<String>>,
}

impl UniformLawAdoption {
    /// Creates a new uniform law adoption tracker
    pub fn new(name: String, year_proposed: u32, publisher: String) -> Self {
        Self {
            name,
            adopted_states: Vec::new(),
            year_proposed,
            publisher,
            variations: HashMap::new(),
        }
    }

    /// Adds a state that has adopted this law
    pub fn add_state(&mut self, state_code: String) {
        if !self.adopted_states.contains(&state_code) {
            self.adopted_states.push(state_code);
        }
    }

    /// Adds a variation for a specific state
    pub fn add_variation(&mut self, state_code: String, variation: String) {
        self.variations
            .entry(state_code)
            .or_default()
            .push(variation);
    }

    /// Returns the adoption percentage (0.0 to 1.0)
    pub fn adoption_percentage(&self) -> f64 {
        self.adopted_states.len() as f64 / 51.0 // 50 states + DC
    }

    /// Returns true if the state has adopted this uniform law
    pub fn is_adopted_by(&self, state_code: &str) -> bool {
        self.adopted_states.contains(&state_code.to_string())
    }
}

/// Compares legislation across multiple states
#[derive(Debug, Clone, Default)]
pub struct StateLegislativeComparator {
    /// Bills being compared
    bills: HashMap<String, Bill>,
}

impl StateLegislativeComparator {
    /// Creates a new comparator
    pub fn new() -> Self {
        Self {
            bills: HashMap::new(),
        }
    }

    /// Adds a bill to the comparison
    pub fn add_bill(&mut self, bill: Bill) {
        self.bills.insert(bill.full_identifier(), bill);
    }

    /// Finds bills with similar titles across states
    pub fn find_similar_bills(&self, min_similarity: f64) -> Vec<BillSimilarity> {
        let mut similarities = Vec::new();

        let bill_list: Vec<_> = self.bills.values().collect();

        for i in 0..bill_list.len() {
            for j in (i + 1)..bill_list.len() {
                let bill1 = bill_list[i];
                let bill2 = bill_list[j];

                if bill1.state_code == bill2.state_code {
                    continue; // Skip same-state comparisons
                }

                let score = Self::calculate_similarity(&bill1.title, &bill2.title);

                if score >= min_similarity {
                    let mut similarity = BillSimilarity::new(
                        bill1.full_identifier(),
                        bill2.full_identifier(),
                        score,
                    );

                    // Find common tags
                    for tag in &bill1.tags {
                        if bill2.tags.contains(tag) {
                            similarity.add_common_element(tag.clone());
                        }
                    }

                    similarities.push(similarity);
                }
            }
        }

        similarities.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        similarities
    }

    /// Calculates similarity between two strings (simple token-based approach)
    fn calculate_similarity(text1: &str, text2: &str) -> f64 {
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();
        let tokens1: Vec<&str> = text1_lower.split_whitespace().collect();
        let tokens2: Vec<&str> = text2_lower.split_whitespace().collect();

        if tokens1.is_empty() || tokens2.is_empty() {
            return 0.0;
        }

        let common_count = tokens1.iter().filter(|t| tokens2.contains(t)).count();

        let total_tokens = tokens1.len().max(tokens2.len());

        common_count as f64 / total_tokens as f64
    }

    /// Groups bills by subject matter
    pub fn group_by_subject(&self) -> HashMap<String, Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for bill in self.bills.values() {
            for tag in &bill.tags {
                groups
                    .entry(tag.clone())
                    .or_default()
                    .push(bill.full_identifier());
            }
        }

        groups
    }

    /// Returns bills from a specific state
    pub fn bills_by_state(&self, state_code: &str) -> Vec<&Bill> {
        self.bills
            .values()
            .filter(|b| b.state_code == state_code)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bill_creation() {
        let bill = Bill::builder()
            .state("CA")
            .chamber(Chamber::House)
            .number("AB 123")
            .title("Data Privacy Protection Act")
            .status(BillStatus::Introduced)
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
            .build();

        assert_eq!(bill.state_code, "CA");
        assert_eq!(bill.chamber, Chamber::House);
        assert_eq!(bill.number, "AB 123");
        assert_eq!(bill.title, "Data Privacy Protection Act");
        assert!(bill.status.is_active());
    }

    #[test]
    fn test_bill_status_transitions() {
        let mut bill = Bill::builder()
            .state("NY")
            .chamber(Chamber::Senate)
            .number("SB 456")
            .title("Environmental Protection Act")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap())
            .build();

        assert!(bill.status.is_active());
        assert!(!bill.status.is_enacted());
        assert!(!bill.status.is_dead());

        bill.update_status(BillStatus::InCommittee {
            committee: "Environmental Conservation".to_string(),
        });
        assert!(bill.status.is_active());

        bill.update_status(BillStatus::Enacted {
            date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            chapter: Some("Ch. 123".to_string()),
        });
        assert!(bill.status.is_enacted());
        assert!(!bill.status.is_active());
    }

    #[test]
    fn test_chamber_prefixes() {
        assert_eq!(Chamber::Senate.bill_prefix("CA"), "SB");
        assert_eq!(Chamber::House.bill_prefix("CA"), "AB"); // Assembly state
        assert_eq!(Chamber::House.bill_prefix("TX"), "HB"); // House state
        assert_eq!(Chamber::Joint.bill_prefix("NY"), "JR");
    }

    #[test]
    fn test_legislator_creation() {
        let legislator = Legislator::new("John Doe".to_string(), Chamber::Senate)
            .with_party("D".to_string())
            .with_district(15);

        assert_eq!(legislator.name, "John Doe");
        assert_eq!(legislator.party, Some("D".to_string()));
        assert_eq!(legislator.chamber, Chamber::Senate);
        assert_eq!(legislator.district, Some(15));
    }

    #[test]
    fn test_legislative_session_creation() {
        let session = LegislativeSession::builder()
            .state("CA")
            .session_type(SessionType::Regular)
            .year(2024)
            .start_date(NaiveDate::from_ymd_opt(2024, 1, 3).unwrap())
            .end_date(NaiveDate::from_ymd_opt(2024, 8, 31).unwrap())
            .deadline("committee", NaiveDate::from_ymd_opt(2024, 4, 30).unwrap())
            .deadline("floor_vote", NaiveDate::from_ymd_opt(2024, 5, 31).unwrap())
            .build();

        assert_eq!(session.state_code, "CA");
        assert_eq!(session.year, 2024);
        assert_eq!(session.session_type, SessionType::Regular);
        assert_eq!(
            session.deadline("committee"),
            Some(NaiveDate::from_ymd_opt(2024, 4, 30).unwrap())
        );
    }

    #[test]
    fn test_session_recess_detection() {
        let mut session = LegislativeSession::builder()
            .state("NY")
            .year(2024)
            .start_date(NaiveDate::from_ymd_opt(2024, 1, 3).unwrap())
            .end_date(NaiveDate::from_ymd_opt(2024, 6, 20).unwrap())
            .build();

        session.add_recess(
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 4, 14).unwrap(),
        );

        assert!(session.is_in_recess(NaiveDate::from_ymd_opt(2024, 4, 7).unwrap()));
        assert!(!session.is_in_recess(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()));
    }

    #[test]
    fn test_amendment_creation() {
        let sponsor = Legislator::new("Jane Smith".to_string(), Chamber::House);

        let amendment = Amendment::builder()
            .id("AM 001")
            .bill_number("HB 789")
            .amendment_type(AmendmentType::Floor)
            .status(AmendmentStatus::Adopted)
            .proposed_date(NaiveDate::from_ymd_opt(2024, 3, 15).unwrap())
            .sponsor(sponsor)
            .summary("Increase funding by $1M")
            .build();

        assert_eq!(amendment.id, "AM 001");
        assert_eq!(amendment.bill_number, "HB 789");
        assert!(amendment.is_adopted());
    }

    #[test]
    fn test_amendment_status() {
        let amendment = Amendment::builder()
            .id("AM 002")
            .bill_number("SB 100")
            .status(AmendmentStatus::Proposed)
            .proposed_date(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap())
            .summary("Technical correction")
            .build();

        assert!(!amendment.is_adopted());
    }

    #[test]
    fn test_committee_creation() {
        let chair = Legislator::new("Alice Johnson".to_string(), Chamber::Senate);

        let committee = Committee::builder()
            .id("SENATE-EDU")
            .name("Education Committee")
            .state("TX")
            .chamber(Chamber::Senate)
            .committee_type(CommitteeType::Standing)
            .chair(chair)
            .jurisdiction("Education")
            .jurisdiction("Schools")
            .build();

        assert_eq!(committee.id, "SENATE-EDU");
        assert_eq!(committee.name, "Education Committee");
        assert!(committee.has_jurisdiction("education"));
        assert!(committee.has_jurisdiction("Schools"));
        assert!(!committee.has_jurisdiction("healthcare"));
    }

    #[test]
    fn test_committee_hearing() {
        let mut hearing = CommitteeHearing::new(
            "SENATE-EDU".to_string(),
            NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        );

        hearing.add_bill("SB 100".to_string());
        hearing.add_bill("SB 101".to_string());
        hearing.add_witness("Dr. John Smith".to_string());
        hearing.set_outcome("Recommend Do Pass".to_string());

        assert_eq!(hearing.bills.len(), 2);
        assert_eq!(hearing.witnesses.len(), 1);
        assert_eq!(hearing.outcome, Some("Recommend Do Pass".to_string()));
    }

    #[test]
    fn test_committee_report() {
        let report = CommitteeReport::new(
            "SENATE-EDU".to_string(),
            "SB 100".to_string(),
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        )
        .with_recommendation("Do Pass as Amended".to_string())
        .with_vote_tally(8, 3, 1);

        assert!(report.recommends_passage());
        assert_eq!(report.vote_tally, Some((8, 3, 1)));
    }

    #[test]
    fn test_uniform_law_adoption() {
        let mut uniform_law = UniformLawAdoption::new(
            "Uniform Electronic Transactions Act".to_string(),
            1999,
            "ULC".to_string(),
        );

        uniform_law.add_state("CA".to_string());
        uniform_law.add_state("NY".to_string());
        uniform_law.add_state("TX".to_string());
        uniform_law.add_variation("CA".to_string(), "Excludes wills and trusts".to_string());

        assert!(uniform_law.is_adopted_by("CA"));
        assert!(!uniform_law.is_adopted_by("FL"));
        assert_eq!(uniform_law.adopted_states.len(), 3);
        assert!(uniform_law.adoption_percentage() > 0.05);
    }

    #[test]
    fn test_bill_similarity_calculation() {
        let title1 = "Consumer Data Privacy Protection Act";
        let title2 = "Consumer Data Privacy and Security Act";

        let score = StateLegislativeComparator::calculate_similarity(title1, title2);

        assert!(score > 0.5); // Should be fairly similar
    }

    #[test]
    fn test_state_legislative_comparator() {
        let mut comparator = StateLegislativeComparator::new();

        let bill1 = Bill::builder()
            .state("CA")
            .chamber(Chamber::House)
            .number("AB 100")
            .title("Privacy Protection Act")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap())
            .tag("privacy")
            .tag("consumer")
            .build();

        let bill2 = Bill::builder()
            .state("NY")
            .chamber(Chamber::Senate)
            .number("SB 200")
            .title("Privacy Protection Act")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
            .tag("privacy")
            .tag("data")
            .build();

        comparator.add_bill(bill1);
        comparator.add_bill(bill2);

        let similar = comparator.find_similar_bills(0.5);
        assert!(!similar.is_empty());
        assert!(similar[0].score > 0.5);
    }

    #[test]
    fn test_comparator_group_by_subject() {
        let mut comparator = StateLegislativeComparator::new();

        let bill1 = Bill::builder()
            .state("CA")
            .chamber(Chamber::House)
            .number("AB 100")
            .title("Privacy Act")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap())
            .tag("privacy")
            .build();

        let bill2 = Bill::builder()
            .state("NY")
            .chamber(Chamber::Senate)
            .number("SB 200")
            .title("Privacy Law")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
            .tag("privacy")
            .build();

        let bill3 = Bill::builder()
            .state("TX")
            .chamber(Chamber::House)
            .number("HB 300")
            .title("Education Reform")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 20).unwrap())
            .tag("education")
            .build();

        comparator.add_bill(bill1);
        comparator.add_bill(bill2);
        comparator.add_bill(bill3);

        let groups = comparator.group_by_subject();

        assert_eq!(groups.get("privacy").unwrap().len(), 2);
        assert_eq!(groups.get("education").unwrap().len(), 1);
    }

    #[test]
    fn test_comparator_bills_by_state() {
        let mut comparator = StateLegislativeComparator::new();

        let bill1 = Bill::builder()
            .state("CA")
            .chamber(Chamber::House)
            .number("AB 100")
            .title("Bill 1")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap())
            .build();

        let bill2 = Bill::builder()
            .state("CA")
            .chamber(Chamber::Senate)
            .number("SB 200")
            .title("Bill 2")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
            .build();

        let bill3 = Bill::builder()
            .state("NY")
            .chamber(Chamber::Senate)
            .number("SB 300")
            .title("Bill 3")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 20).unwrap())
            .build();

        comparator.add_bill(bill1);
        comparator.add_bill(bill2);
        comparator.add_bill(bill3);

        let ca_bills = comparator.bills_by_state("CA");
        let ny_bills = comparator.bills_by_state("NY");

        assert_eq!(ca_bills.len(), 2);
        assert_eq!(ny_bills.len(), 1);
    }

    #[test]
    fn test_bill_full_identifier() {
        let bill = Bill::builder()
            .state("TX")
            .chamber(Chamber::House)
            .number("HB 500")
            .title("Test Bill")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .build();

        assert_eq!(bill.full_identifier(), "TX HB 500");
    }

    #[test]
    fn test_bill_add_cosponsor() {
        let mut bill = Bill::builder()
            .state("FL")
            .chamber(Chamber::House)
            .number("HB 600")
            .title("Test Bill")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .build();

        assert_eq!(bill.cosponsors.len(), 0);

        bill.add_cosponsor(Legislator::new("Rep. Smith".to_string(), Chamber::House));
        bill.add_cosponsor(Legislator::new("Rep. Jones".to_string(), Chamber::House));

        assert_eq!(bill.cosponsors.len(), 2);
    }

    #[test]
    fn test_bill_add_tag() {
        let mut bill = Bill::builder()
            .state("IL")
            .chamber(Chamber::Senate)
            .number("SB 700")
            .title("Test Bill")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .build();

        bill.add_tag("healthcare".to_string());
        bill.add_tag("insurance".to_string());
        bill.add_tag("healthcare".to_string()); // Duplicate should not be added

        assert_eq!(bill.tags.len(), 2);
    }

    #[test]
    fn test_bill_priority() {
        let emergency_bill = Bill::builder()
            .state("CA")
            .chamber(Chamber::House)
            .number("AB 1")
            .title("Emergency Response Bill")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .priority(BillPriority::Emergency)
            .build();

        assert_eq!(emergency_bill.priority, BillPriority::Emergency);

        let normal_bill = Bill::builder()
            .state("NY")
            .chamber(Chamber::Senate)
            .number("SB 2")
            .title("Regular Bill")
            .introduced_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .build();

        // Default priority should be Normal
        assert_eq!(normal_bill.priority, BillPriority::Normal);
    }
}
