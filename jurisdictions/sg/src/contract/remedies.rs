//! Contract Law - Remedies for Breach
//!
//! Models the principal remedies for breach of contract under Singapore law:
//!
//! - **Damages** measured by the expectation interest — to put the innocent
//!   party in the position it would have occupied had the contract been
//!   performed (*Robinson v Harman* (1848) 1 Ex 850), subject to:
//!   - **Remoteness** — *Hadley v Baxendale* (1854) 9 Exch 341, as applied in
//!     Singapore in *Robertson Quay Investment v Steen Consultants* \[2008\]
//!     SGCA 8 and *MFM Restaurants v Fish & Co* \[2010\] SGCA 36.
//!   - **Mitigation** — the innocent party must take reasonable steps to
//!     minimise its loss (*British Westinghouse v Underground Electric
//!     Railways* \[1912\] AC 673).
//! - **Specific performance** — an equitable, discretionary remedy granted only
//!   where common-law damages are inadequate (e.g. sale of land, or unique
//!   goods).
//! - **Termination** — the innocent party's right to elect to bring future
//!   obligations to an end following a repudiatory breach (*RDC Concrete v Sato
//!   Kogyo* \[2007\] SGCA 1).
//!
//! Monetary values are in SGD cents (`i64`).

use serde::{Deserialize, Serialize};

/// The limb of *Hadley v Baxendale* under which a head of loss is said to be
/// recoverable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemotenessLimb {
    /// First limb: loss arising naturally, i.e. in the ordinary course of
    /// things, from the breach.
    Ordinary,
    /// Second limb: loss that, though not ordinary, was in the reasonable
    /// contemplation of both parties at the time of contracting because of
    /// special circumstances communicated to (and accepted by) the defendant.
    SpecialContemplation,
}

/// A single head of claimed loss.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadOfLoss {
    /// Description of the loss.
    pub description: String,
    /// Amount claimed, in SGD cents.
    pub amount_cents: i64,
    /// The remoteness limb relied on.
    pub limb: RemotenessLimb,
    /// For the second limb: whether the special circumstances were actually
    /// communicated to the defendant before contracting.
    pub special_circumstances_communicated: bool,
    /// Whether this head represents loss the claimant could reasonably have
    /// avoided (i.e. unmitigated loss that should be disallowed).
    pub avoidable_by_mitigation: bool,
}

impl HeadOfLoss {
    /// Creates an ordinary (first-limb) head of loss.
    pub fn ordinary(description: impl Into<String>, amount_cents: i64) -> Self {
        Self {
            description: description.into(),
            amount_cents,
            limb: RemotenessLimb::Ordinary,
            special_circumstances_communicated: false,
            avoidable_by_mitigation: false,
        }
    }

    /// Creates a special (second-limb) head of loss, recording whether the
    /// special circumstances were communicated.
    pub fn special(description: impl Into<String>, amount_cents: i64, communicated: bool) -> Self {
        Self {
            description: description.into(),
            amount_cents,
            limb: RemotenessLimb::SpecialContemplation,
            special_circumstances_communicated: communicated,
            avoidable_by_mitigation: false,
        }
    }

    /// Marks this head as avoidable loss that reasonable mitigation would have
    /// prevented (and which is therefore irrecoverable).
    pub fn avoidable(mut self) -> Self {
        self.avoidable_by_mitigation = true;
        self
    }

    /// Returns whether the head of loss is recoverable: it must not be too
    /// remote, and must not have been avoidable by reasonable mitigation.
    ///
    /// First-limb loss is never too remote. Second-limb loss is recoverable only
    /// where the special circumstances were communicated at formation.
    pub fn is_recoverable(&self) -> bool {
        if self.avoidable_by_mitigation {
            return false;
        }
        match self.limb {
            RemotenessLimb::Ordinary => true,
            RemotenessLimb::SpecialContemplation => self.special_circumstances_communicated,
        }
    }
}

/// The measure on which an award of damages is based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamagesMeasure {
    /// Expectation (loss of bargain) — the normal measure (*Robinson v Harman*
    /// (1848) 1 Ex 850).
    Expectation,
    /// Reliance — wasted expenditure incurred in reliance on the contract
    /// (*Anglia Television v Reed* \[1972\] 1 QB 60).
    Reliance,
    /// Restitution — recovery of a benefit conferred.
    Restitution,
}

/// A computed award of damages for breach.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamagesAward {
    /// The measure applied.
    pub measure: DamagesMeasure,
    /// The heads of loss claimed.
    pub heads: Vec<HeadOfLoss>,
    /// Total claimed across all heads, in SGD cents.
    pub claimed_cents: i64,
    /// Total recoverable after applying remoteness and mitigation, in SGD cents.
    pub recoverable_cents: i64,
    /// The heads disallowed as too remote (descriptions).
    pub remote_heads: Vec<String>,
    /// The heads disallowed for failure to mitigate (descriptions).
    pub unmitigated_heads: Vec<String>,
}

impl DamagesAward {
    /// Returns the amount disallowed (claimed minus recoverable), in SGD cents.
    pub fn disallowed_cents(&self) -> i64 {
        self.claimed_cents - self.recoverable_cents
    }

    /// Returns the recoverable amount as SGD (whole dollars and cents).
    pub fn recoverable_sgd(&self) -> f64 {
        self.recoverable_cents as f64 / 100.0
    }
}

/// Factors bearing on whether specific performance should be ordered.
///
/// Specific performance is discretionary and is not granted where damages are
/// an adequate remedy. It is the ordinary remedy for contracts for the sale of
/// land (each parcel being unique) and may be granted for unique goods, but is
/// refused for contracts requiring constant supervision or for personal
/// service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecificPerformanceFactors {
    /// Whether common-law damages would be an adequate remedy.
    pub damages_adequate: bool,
    /// Whether the subject matter is unique (e.g. land, a specific chattel of
    /// special value).
    pub subject_matter_unique: bool,
    /// Whether the contract is one of personal service (for which specific
    /// performance is not granted).
    pub personal_service: bool,
    /// Whether performance would require constant judicial supervision (a
    /// discretionary bar — *Co-operative Insurance v Argyll Stores* \[1998\] AC
    /// 1).
    pub requires_constant_supervision: bool,
}

impl SpecificPerformanceFactors {
    /// Creates a set of factors with sensible defaults (damages adequate,
    /// subject matter not unique).
    pub fn new() -> Self {
        Self {
            damages_adequate: true,
            subject_matter_unique: false,
            personal_service: false,
            requires_constant_supervision: false,
        }
    }

    /// Marks the subject matter as unique (which tends to make damages
    /// inadequate).
    pub fn unique_subject_matter(mut self) -> Self {
        self.subject_matter_unique = true;
        self.damages_adequate = false;
        self
    }

    /// Marks the contract as one of personal service.
    pub fn personal_service(mut self) -> Self {
        self.personal_service = true;
        self
    }

    /// Marks the contract as requiring constant supervision.
    pub fn requires_supervision(mut self) -> Self {
        self.requires_constant_supervision = true;
        self
    }

    /// Returns whether specific performance is, in principle, available: damages
    /// must be inadequate, and none of the discretionary bars present.
    pub fn is_available(&self) -> bool {
        !self.damages_adequate && !self.personal_service && !self.requires_constant_supervision
    }
}

impl Default for SpecificPerformanceFactors {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_limb_loss_is_always_recoverable() {
        let head = HeadOfLoss::ordinary("cost of cure", 500_000);
        assert!(head.is_recoverable());
    }

    #[test]
    fn second_limb_loss_needs_communication() {
        let uncommunicated =
            HeadOfLoss::special("lost lucrative dyeing contract", 1_000_000, false);
        assert!(!uncommunicated.is_recoverable());

        let communicated = HeadOfLoss::special("lost lucrative dyeing contract", 1_000_000, true);
        assert!(communicated.is_recoverable());
    }

    #[test]
    fn avoidable_loss_is_disallowed_even_if_ordinary() {
        let head = HeadOfLoss::ordinary("rent on premises kept after breach", 300_000).avoidable();
        assert!(!head.is_recoverable());
    }

    #[test]
    fn specific_performance_for_unique_subject_matter() {
        let factors = SpecificPerformanceFactors::new().unique_subject_matter();
        assert!(factors.is_available());
    }

    #[test]
    fn specific_performance_refused_for_personal_service() {
        let factors = SpecificPerformanceFactors::new()
            .unique_subject_matter()
            .personal_service();
        assert!(!factors.is_available());
    }

    #[test]
    fn damages_award_disallowed_arithmetic() {
        let award = DamagesAward {
            measure: DamagesMeasure::Expectation,
            heads: Vec::new(),
            claimed_cents: 1_500_000,
            recoverable_cents: 500_000,
            remote_heads: vec!["lost contract".to_string()],
            unmitigated_heads: Vec::new(),
        };
        assert_eq!(award.disallowed_cents(), 1_000_000);
        assert!((award.recoverable_sgd() - 5_000.0).abs() < f64::EPSILON);
    }
}
