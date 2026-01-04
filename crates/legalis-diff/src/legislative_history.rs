//! Legislative history integration for statute tracking.
//!
//! This module tracks the complete legislative history of statutes,
//! including bill origins, amendments, debates, and committee reviews.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete legislative history for a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeHistory {
    /// Statute ID.
    pub statute_id: String,
    /// Original bill that created the statute.
    pub origin_bill: Bill,
    /// All amendments to the statute.
    pub amendments: Vec<Amendment>,
    /// Committee reviews.
    pub committee_reviews: Vec<CommitteeReview>,
    /// Floor debates.
    pub debates: Vec<Debate>,
    /// Votes on the statute and amendments.
    pub votes: Vec<Vote>,
    /// Sponsor information.
    pub sponsors: Vec<Sponsor>,
}

impl LegislativeHistory {
    /// Creates a new legislative history.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::legislative_history::{LegislativeHistory, Bill};
    /// use chrono::Utc;
    ///
    /// let bill = Bill::new("HB-123", "Tax Reform Act", Utc::now());
    /// let history = LegislativeHistory::new("statute-1", bill);
    ///
    /// assert_eq!(history.statute_id, "statute-1");
    /// ```
    pub fn new(statute_id: &str, origin_bill: Bill) -> Self {
        Self {
            statute_id: statute_id.to_string(),
            origin_bill,
            amendments: Vec::new(),
            committee_reviews: Vec::new(),
            debates: Vec::new(),
            votes: Vec::new(),
            sponsors: Vec::new(),
        }
    }

    /// Adds an amendment to the history.
    pub fn add_amendment(&mut self, amendment: Amendment) {
        self.amendments.push(amendment);
    }

    /// Adds a committee review.
    pub fn add_committee_review(&mut self, review: CommitteeReview) {
        self.committee_reviews.push(review);
    }

    /// Adds a debate record.
    pub fn add_debate(&mut self, debate: Debate) {
        self.debates.push(debate);
    }

    /// Adds a vote record.
    pub fn add_vote(&mut self, vote: Vote) {
        self.votes.push(vote);
    }

    /// Adds a sponsor.
    pub fn add_sponsor(&mut self, sponsor: Sponsor) {
        self.sponsors.push(sponsor);
    }

    /// Gets the complete timeline of events.
    pub fn timeline(&self) -> Vec<HistoryEvent> {
        let mut events = Vec::new();

        // Add bill introduction
        events.push(HistoryEvent {
            timestamp: self.origin_bill.introduced_date,
            event_type: EventType::BillIntroduced,
            description: format!("Bill {} introduced", self.origin_bill.bill_number),
        });

        // Add amendments
        for amendment in &self.amendments {
            events.push(HistoryEvent {
                timestamp: amendment.proposed_date,
                event_type: EventType::AmendmentProposed,
                description: format!("Amendment {} proposed", amendment.amendment_number),
            });
        }

        // Add committee reviews
        for review in &self.committee_reviews {
            events.push(HistoryEvent {
                timestamp: review.review_date,
                event_type: EventType::CommitteeReview,
                description: format!("Reviewed by {}", review.committee_name),
            });
        }

        // Add votes
        for vote in &self.votes {
            events.push(HistoryEvent {
                timestamp: vote.date,
                event_type: EventType::Vote,
                description: format!("Vote: {} yeas, {} nays", vote.yeas, vote.nays),
            });
        }

        // Sort by timestamp
        events.sort_by_key(|e| e.timestamp);
        events
    }
}

/// A legislative bill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bill {
    /// Bill number (e.g., "HB-123", "S-456").
    pub bill_number: String,
    /// Bill title.
    pub title: String,
    /// When the bill was introduced.
    pub introduced_date: DateTime<Utc>,
    /// Chamber where introduced (House, Senate, etc.).
    pub chamber: Chamber,
    /// Bill text.
    pub text: Option<String>,
}

impl Bill {
    /// Creates a new bill.
    pub fn new(bill_number: &str, title: &str, introduced_date: DateTime<Utc>) -> Self {
        Self {
            bill_number: bill_number.to_string(),
            title: title.to_string(),
            introduced_date,
            chamber: Chamber::House,
            text: None,
        }
    }
}

/// Legislative chamber.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chamber {
    House,
    Senate,
    Joint,
}

/// An amendment to legislation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amendment {
    /// Amendment number.
    pub amendment_number: String,
    /// Who proposed it.
    pub proposer: String,
    /// When it was proposed.
    pub proposed_date: DateTime<Utc>,
    /// Amendment text.
    pub text: String,
    /// Whether it was adopted.
    pub adopted: bool,
    /// Related vote ID.
    pub vote_id: Option<String>,
}

/// Committee review of legislation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeReview {
    /// Committee name.
    pub committee_name: String,
    /// Review date.
    pub review_date: DateTime<Utc>,
    /// Committee recommendation.
    pub recommendation: Recommendation,
    /// Committee report.
    pub report: Option<String>,
}

/// Committee recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recommendation {
    /// Recommend passage.
    Pass,
    /// Recommend passage with amendments.
    PassWithAmendments,
    /// Do not recommend passage.
    DoNotPass,
    /// No recommendation.
    NoRecommendation,
}

/// Floor debate record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debate {
    /// Date of debate.
    pub date: DateTime<Utc>,
    /// Chamber where debate occurred.
    pub chamber: Chamber,
    /// Speakers and their statements.
    pub statements: Vec<Statement>,
}

/// A statement during debate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    /// Who spoke.
    pub speaker: String,
    /// What was said.
    pub content: String,
    /// Position (for/against/neutral).
    pub position: Position,
}

/// Position on legislation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    For,
    Against,
    Neutral,
}

/// Vote on legislation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Vote ID.
    pub id: String,
    /// Vote date.
    pub date: DateTime<Utc>,
    /// Chamber where vote occurred.
    pub chamber: Chamber,
    /// Number of yea votes.
    pub yeas: u32,
    /// Number of nay votes.
    pub nays: u32,
    /// Number of abstentions.
    pub abstentions: u32,
    /// Whether the vote passed.
    pub passed: bool,
    /// Individual votes by member.
    pub individual_votes: HashMap<String, VoteChoice>,
}

/// An individual's vote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteChoice {
    Yea,
    Nay,
    Abstain,
}

/// Sponsor of legislation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sponsor {
    /// Sponsor name.
    pub name: String,
    /// Sponsor type.
    pub sponsor_type: SponsorType,
    /// Chamber.
    pub chamber: Chamber,
}

/// Type of sponsor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SponsorType {
    /// Primary sponsor.
    Primary,
    /// Co-sponsor.
    CoSponsor,
}

/// Historical event in legislation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEvent {
    /// When the event occurred.
    pub timestamp: DateTime<Utc>,
    /// Type of event.
    pub event_type: EventType,
    /// Description.
    pub description: String,
}

/// Type of historical event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    BillIntroduced,
    AmendmentProposed,
    CommitteeReview,
    Debate,
    Vote,
    Enacted,
    Vetoed,
}

/// Analyzes the legislative intent from history.
///
/// # Examples
///
/// ```
/// use legalis_diff::legislative_history::{LegislativeHistory, Bill, analyze_intent};
/// use chrono::Utc;
///
/// let bill = Bill::new("HB-123", "Tax Reform", Utc::now());
/// let history = LegislativeHistory::new("statute-1", bill);
///
/// let intent = analyze_intent(&history);
/// assert!(!intent.is_empty());
/// ```
pub fn analyze_intent(history: &LegislativeHistory) -> String {
    let mut intent_analysis = String::new();

    // Analyze bill title for intent
    intent_analysis.push_str(&format!(
        "Original purpose: {}\n",
        history.origin_bill.title
    ));

    // Analyze amendments for changes in intent
    if !history.amendments.is_empty() {
        intent_analysis.push_str(&format!(
            "\nAmended {} times, indicating evolving legislative intent.\n",
            history.amendments.len()
        ));
    }

    // Analyze debates for stated purposes
    let total_statements: usize = history.debates.iter().map(|d| d.statements.len()).sum();
    if total_statements > 0 {
        intent_analysis.push_str(&format!(
            "\n{} debate statements provide insight into legislative intent.\n",
            total_statements
        ));
    }

    intent_analysis
}

/// Tracks how a statute evolved through amendments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTracker {
    /// Statute ID.
    pub statute_id: String,
    /// Evolution steps.
    pub steps: Vec<EvolutionStep>,
}

/// A step in the evolution of a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStep {
    /// Step number.
    pub step: usize,
    /// When it occurred.
    pub date: DateTime<Utc>,
    /// What changed.
    pub description: String,
    /// Related amendment/bill.
    pub source: String,
}

impl EvolutionTracker {
    /// Creates a new evolution tracker.
    pub fn new(statute_id: &str) -> Self {
        Self {
            statute_id: statute_id.to_string(),
            steps: Vec::new(),
        }
    }

    /// Adds an evolution step.
    pub fn add_step(&mut self, date: DateTime<Utc>, description: &str, source: &str) {
        self.steps.push(EvolutionStep {
            step: self.steps.len() + 1,
            date,
            description: description.to_string(),
            source: source.to_string(),
        });
    }

    /// Gets the evolution timeline.
    pub fn timeline_summary(&self) -> String {
        let mut summary = format!("Evolution of Statute {}\n", self.statute_id);
        summary.push_str("=========================\n\n");

        for step in &self.steps {
            summary.push_str(&format!(
                "Step {}: {} - {} ({})\n",
                step.step,
                step.date.format("%Y-%m-%d"),
                step.description,
                step.source
            ));
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legislative_history_creation() {
        let bill = Bill::new("HB-123", "Test Bill", Utc::now());
        let history = LegislativeHistory::new("statute-1", bill);

        assert_eq!(history.statute_id, "statute-1");
        assert_eq!(history.origin_bill.bill_number, "HB-123");
    }

    #[test]
    fn test_add_amendment() {
        let bill = Bill::new("HB-123", "Test Bill", Utc::now());
        let mut history = LegislativeHistory::new("statute-1", bill);

        let amendment = Amendment {
            amendment_number: "A-1".to_string(),
            proposer: "Smith".to_string(),
            proposed_date: Utc::now(),
            text: "Amendment text".to_string(),
            adopted: true,
            vote_id: None,
        };

        history.add_amendment(amendment);
        assert_eq!(history.amendments.len(), 1);
    }

    #[test]
    fn test_timeline() {
        let bill = Bill::new("HB-123", "Test Bill", Utc::now());
        let mut history = LegislativeHistory::new("statute-1", bill);

        let vote = Vote {
            id: "vote-1".to_string(),
            date: Utc::now(),
            chamber: Chamber::House,
            yeas: 100,
            nays: 50,
            abstentions: 5,
            passed: true,
            individual_votes: HashMap::new(),
        };

        history.add_vote(vote);

        let timeline = history.timeline();
        assert!(!timeline.is_empty());
    }

    #[test]
    fn test_intent_analysis() {
        let bill = Bill::new("HB-123", "Tax Reform Act", Utc::now());
        let history = LegislativeHistory::new("statute-1", bill);

        let intent = analyze_intent(&history);
        assert!(intent.contains("Tax Reform Act"));
    }

    #[test]
    fn test_evolution_tracker() {
        let mut tracker = EvolutionTracker::new("statute-1");
        tracker.add_step(Utc::now(), "Initial version", "HB-123");
        tracker.add_step(Utc::now(), "Amended threshold", "HB-123-A1");

        assert_eq!(tracker.steps.len(), 2);

        let summary = tracker.timeline_summary();
        assert!(summary.contains("Evolution of Statute statute-1"));
    }
}
