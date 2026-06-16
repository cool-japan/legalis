//! Tort Law - Nuisance, Occupiers' Liability and General Defences
//!
//! This file models:
//!
//! - **Private nuisance** — an unlawful (substantial and unreasonable)
//!   interference with a person's use or enjoyment of land, actionable by a
//!   person with a proprietary interest (*Hunter v Canary Wharf* \[1997\] AC
//!   655; *Sturges v Bridgman* (1879) 11 Ch D 852).
//! - **Public nuisance** — an act endangering the life, health, property or
//!   comfort of a class of the public; actionable in tort by a private claimant
//!   only on proof of special damage beyond that suffered by the public
//!   generally (*Tate & Lyle v GLC* \[1983\] 2 AC 509).
//! - **Occupiers' liability** — the duty of an occupier of premises to take
//!   reasonable care for the safety of those who come onto the premises, the
//!   content of the duty varying with the entrant's status (lawful visitor vs
//!   trespasser — *British Railways Board v Herrington* \[1972\] AC 877; and in
//!   Singapore *Industrial Commercial Bank v Tan Swa Eng* \[1995\]).
//! - **General defences** — contributory negligence (apportionment under the
//!   Contributory Negligence and Personal Injuries Act 1953), volenti non fit
//!   injuria, and illegality (ex turpi causa).

use serde::{Deserialize, Serialize};

// ===========================================================================
// Private nuisance
// ===========================================================================

/// The kind of interference complained of in a private-nuisance claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterferenceKind {
    /// Noise.
    Noise,
    /// Smell / odour.
    Smell,
    /// Smoke, fumes or dust.
    SmokeOrFumes,
    /// Vibration.
    Vibration,
    /// Encroachment (e.g. tree roots, overhanging branches).
    Encroachment,
    /// Physical damage to the land or buildings.
    PhysicalDamage,
    /// Interference with light, water or other natural rights.
    NaturalRights,
}

/// A private-nuisance claim.
///
/// The touchstone is reasonableness between neighbours; the court weighs the
/// locality, the duration and intensity of the interference, any malice, and
/// whether the defendant took reasonable steps. Physical damage to land is
/// almost always actionable; mere interference with amenity is judged by the
/// standards of the locality (*St Helen's Smelting v Tipping* (1865) 11 HL Cas
/// 642).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateNuisanceClaim {
    /// Identifier of the claim.
    pub id: String,
    /// The claimant (must have a proprietary interest in the affected land).
    pub claimant: String,
    /// The defendant.
    pub defendant: String,
    /// The kind of interference.
    pub interference: InterferenceKind,
    /// Whether the claimant has a proprietary interest in the land (a standing
    /// requirement — *Hunter v Canary Wharf*).
    pub has_proprietary_interest: bool,
    /// Whether the interference is substantial (more than trivial).
    pub substantial: bool,
    /// Whether the interference is unreasonable in the circumstances (locality,
    /// duration, intensity, malice).
    pub unreasonable: bool,
    /// Whether the claim is for physical damage to land (which is judged by a
    /// stricter standard than amenity interference).
    pub physical_damage: bool,
}

impl PrivateNuisanceClaim {
    /// Creates a private-nuisance claim with standing and the substantial/
    /// unreasonable elements satisfied, which the caller may then adjust.
    pub fn new(
        id: impl Into<String>,
        claimant: impl Into<String>,
        defendant: impl Into<String>,
        interference: InterferenceKind,
    ) -> Self {
        let physical_damage = matches!(interference, InterferenceKind::PhysicalDamage);
        Self {
            id: id.into(),
            claimant: claimant.into(),
            defendant: defendant.into(),
            interference,
            has_proprietary_interest: true,
            substantial: true,
            unreasonable: true,
            physical_damage,
        }
    }

    /// Sets whether the claimant has a proprietary interest (standing).
    pub fn with_proprietary_interest(mut self, value: bool) -> Self {
        self.has_proprietary_interest = value;
        self
    }

    /// Sets whether the interference is substantial.
    pub fn with_substantial(mut self, value: bool) -> Self {
        self.substantial = value;
        self
    }

    /// Sets whether the interference is unreasonable.
    pub fn with_unreasonable(mut self, value: bool) -> Self {
        self.unreasonable = value;
        self
    }

    /// Returns whether the nuisance is actionable: standing, plus a substantial
    /// and unreasonable interference (physical damage to land is treated as
    /// substantial and unreasonable as a matter of course).
    pub fn is_actionable(&self) -> bool {
        if !self.has_proprietary_interest {
            return false;
        }
        if self.physical_damage {
            return true;
        }
        self.substantial && self.unreasonable
    }
}

// ===========================================================================
// Public nuisance
// ===========================================================================

/// A public-nuisance claim brought in tort by a private claimant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicNuisanceClaim {
    /// Identifier of the claim.
    pub id: String,
    /// The claimant.
    pub claimant: String,
    /// The defendant.
    pub defendant: String,
    /// Description of the nuisance.
    pub description: String,
    /// Whether the nuisance affects a class of the public (a requirement).
    pub affects_class_of_public: bool,
    /// Whether the claimant has suffered special damage over and above that
    /// suffered by the public generally (the standing requirement for a private
    /// action).
    pub special_damage: bool,
}

impl PublicNuisanceClaim {
    /// Creates a public-nuisance claim.
    pub fn new(
        id: impl Into<String>,
        claimant: impl Into<String>,
        defendant: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            claimant: claimant.into(),
            defendant: defendant.into(),
            description: description.into(),
            affects_class_of_public: true,
            special_damage: false,
        }
    }

    /// Records that the claimant has suffered special damage.
    pub fn with_special_damage(mut self) -> Self {
        self.special_damage = true;
        self
    }

    /// Sets whether a class of the public is affected.
    pub fn with_class_affected(mut self, value: bool) -> Self {
        self.affects_class_of_public = value;
        self
    }

    /// Returns whether a private claimant may sue: the nuisance must affect a
    /// class of the public and the claimant must prove special damage.
    pub fn private_action_available(&self) -> bool {
        self.affects_class_of_public && self.special_damage
    }
}

// ===========================================================================
// Occupiers' liability
// ===========================================================================

/// The status of an entrant onto premises, which fixes the content of the
/// occupier's duty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntrantStatus {
    /// A lawful visitor (invitee or licensee): owed the common duty of care to
    /// be reasonably safe for the purposes of the visit.
    LawfulVisitor,
    /// A contractual entrant (entering under a contract).
    ContractualEntrant,
    /// A trespasser: owed a more limited duty of common humanity / to take
    /// reasonable care in the circumstances (*British Railways Board v
    /// Herrington* \[1972\] AC 877).
    Trespasser,
    /// A child trespasser, to whom allurements may be relevant (*Glasgow Corp v
    /// Taylor* \[1922\] 1 AC 44).
    ChildTrespasser,
}

impl EntrantStatus {
    /// Returns a short label for the entrant status (used in error messages).
    pub fn label(&self) -> &'static str {
        match self {
            EntrantStatus::LawfulVisitor => "lawful visitor",
            EntrantStatus::ContractualEntrant => "contractual entrant",
            EntrantStatus::Trespasser => "trespasser",
            EntrantStatus::ChildTrespasser => "child trespasser",
        }
    }

    /// Returns whether the occupier owes the (higher) common duty of care, as
    /// opposed to the limited duty owed to a trespasser.
    pub fn owed_common_duty(&self) -> bool {
        matches!(
            self,
            EntrantStatus::LawfulVisitor | EntrantStatus::ContractualEntrant
        )
    }
}

/// An occupiers'-liability claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OccupiersLiabilityClaim {
    /// Identifier of the claim.
    pub id: String,
    /// The injured entrant.
    pub entrant: String,
    /// The occupier of the premises.
    pub occupier: String,
    /// The entrant's status.
    pub status: EntrantStatus,
    /// Description of the danger / state of the premises.
    pub danger: String,
    /// Whether the occupier failed to take the care required for that entrant.
    pub failed_to_take_reasonable_care: bool,
    /// Whether the occupier had given an adequate warning of the danger (which
    /// may discharge the duty if it enabled the visitor to be reasonably safe).
    pub adequate_warning_given: bool,
    /// Whether the danger arose from the faulty work of an independent
    /// contractor for whom the occupier is not liable if it acted reasonably in
    /// engaging and checking the contractor.
    pub independent_contractor_defence: bool,
}

impl OccupiersLiabilityClaim {
    /// Creates an occupiers'-liability claim.
    pub fn new(
        id: impl Into<String>,
        entrant: impl Into<String>,
        occupier: impl Into<String>,
        status: EntrantStatus,
        danger: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            entrant: entrant.into(),
            occupier: occupier.into(),
            status,
            danger: danger.into(),
            failed_to_take_reasonable_care: true,
            adequate_warning_given: false,
            independent_contractor_defence: false,
        }
    }

    /// Records that an adequate warning of the danger was given.
    pub fn with_adequate_warning(mut self) -> Self {
        self.adequate_warning_given = true;
        self
    }

    /// Records that the danger was due to an independent contractor and the
    /// occupier acted reasonably (engagement + checking).
    pub fn with_independent_contractor_defence(mut self) -> Self {
        self.independent_contractor_defence = true;
        self
    }

    /// Sets whether the occupier failed to take reasonable care.
    pub fn with_failure(mut self, value: bool) -> Self {
        self.failed_to_take_reasonable_care = value;
        self
    }

    /// Returns whether the occupier is liable: a failure to take the care
    /// required, not discharged by an adequate warning or the independent-
    /// contractor defence.
    pub fn is_liable(&self) -> bool {
        self.failed_to_take_reasonable_care
            && !self.adequate_warning_given
            && !self.independent_contractor_defence
    }
}

// ===========================================================================
// General defences
// ===========================================================================

/// A general defence to a tort claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TortDefence {
    /// Contributory negligence: the claimant's own want of care contributed to
    /// the loss. Apportions, rather than defeats, the claim — damages are
    /// reduced to the extent the court thinks just (Contributory Negligence and
    /// Personal Injuries Act 1953, s. 3).
    ContributoryNegligence {
        /// Percentage reduction (0–100) attributable to the claimant.
        claimant_fault_percent: u8,
    },
    /// Volenti non fit injuria: the claimant voluntarily accepted the risk with
    /// full knowledge of its nature and extent. A complete defence (*ICI v
    /// Shatwell* \[1965\] AC 656).
    VolentiNonFitInjuria,
    /// Illegality (ex turpi causa): the claim arises out of the claimant's own
    /// illegal act (*Patel v Mirza* \[2016\] UKSC 42; *Ochroid Trading v
    /// Chua Siok Lui* \[2018\] SGCA 5). A complete defence where it applies.
    Illegality,
}

impl TortDefence {
    /// Returns the controlling authority / statutory provision for the defence.
    pub fn authority(&self) -> &'static str {
        match self {
            TortDefence::ContributoryNegligence { .. } => {
                "Contributory Negligence and Personal Injuries Act 1953, s. 3"
            }
            TortDefence::VolentiNonFitInjuria => "ICI v Shatwell [1965] AC 656",
            TortDefence::Illegality => "Ochroid Trading v Chua Siok Lui [2018] SGCA 5",
        }
    }

    /// Returns whether the defence is a complete defence (defeats the claim
    /// entirely), as opposed to apportioning damages.
    pub fn is_complete_defence(&self) -> bool {
        !matches!(self, TortDefence::ContributoryNegligence { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_nuisance_needs_standing() {
        let claim = PrivateNuisanceClaim::new("p1", "Owner", "Factory", InterferenceKind::Noise);
        assert!(claim.is_actionable());

        let no_standing = claim.with_proprietary_interest(false);
        assert!(!no_standing.is_actionable());
    }

    #[test]
    fn physical_damage_is_always_actionable() {
        let claim =
            PrivateNuisanceClaim::new("p2", "Owner", "Neighbour", InterferenceKind::PhysicalDamage)
                .with_substantial(false)
                .with_unreasonable(false);
        // Physical damage to land overrides amenity considerations.
        assert!(claim.is_actionable());
    }

    #[test]
    fn amenity_nuisance_needs_substantial_and_unreasonable() {
        let trivial = PrivateNuisanceClaim::new("p3", "Owner", "Cafe", InterferenceKind::Smell)
            .with_substantial(false);
        assert!(!trivial.is_actionable());
    }

    #[test]
    fn public_nuisance_private_action_needs_special_damage() {
        let claim = PublicNuisanceClaim::new("pn1", "Trader", "Obstructor", "blocked the river");
        assert!(!claim.private_action_available());
        assert!(claim.with_special_damage().private_action_available());
    }

    #[test]
    fn occupier_liable_to_visitor_without_warning() {
        let claim = OccupiersLiabilityClaim::new(
            "o1",
            "Visitor",
            "Shop",
            EntrantStatus::LawfulVisitor,
            "wet floor",
        );
        assert!(claim.is_liable());
    }

    #[test]
    fn adequate_warning_discharges_duty() {
        let claim = OccupiersLiabilityClaim::new(
            "o2",
            "Visitor",
            "Shop",
            EntrantStatus::LawfulVisitor,
            "wet floor",
        )
        .with_adequate_warning();
        assert!(!claim.is_liable());
    }

    #[test]
    fn entrant_status_controls_duty_level() {
        assert!(EntrantStatus::LawfulVisitor.owed_common_duty());
        assert!(!EntrantStatus::Trespasser.owed_common_duty());
        assert_eq!(EntrantStatus::ChildTrespasser.label(), "child trespasser");
    }

    #[test]
    fn contributory_negligence_apportions_not_defeats() {
        let defence = TortDefence::ContributoryNegligence {
            claimant_fault_percent: 25,
        };
        assert!(!defence.is_complete_defence());
        assert_eq!(
            defence.authority(),
            "Contributory Negligence and Personal Injuries Act 1953, s. 3"
        );
    }

    #[test]
    fn volenti_is_a_complete_defence() {
        assert!(TortDefence::VolentiNonFitInjuria.is_complete_defence());
    }

    #[test]
    fn serde_roundtrip_occupiers_claim() {
        let claim = OccupiersLiabilityClaim::new(
            "o3",
            "Child",
            "Quarry",
            EntrantStatus::ChildTrespasser,
            "unfenced water",
        );
        let json = serde_json::to_string(&claim).expect("serialize");
        let back: OccupiersLiabilityClaim = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(claim, back);
    }
}
