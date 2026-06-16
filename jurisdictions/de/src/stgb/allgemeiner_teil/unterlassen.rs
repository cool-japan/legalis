//! StGB §§ 13-14 - Liability by Omission and Acting for Another
//!
//! ## § 13 StGB - Begehen durch Unterlassen (Commission by omission)
//!
//! > (1) Wer es unterlässt, einen Erfolg abzuwenden, der zum Tatbestand eines
//! > Strafgesetzes gehört, ist nach diesem Gesetz nur dann strafbar, wenn er
//! > rechtlich dafür einzustehen hat, dass der Erfolg nicht eintritt, und wenn
//! > das Unterlassen der Verwirklichung des gesetzlichen Tatbestandes durch ein
//! > Tun entspricht.
//! > (2) Die Strafe kann nach § 49 Abs. 1 gemildert werden.
//!
//! **English**: A person who omits to avert a result that is an element of a
//! criminal offence is liable only if he is **legally responsible** for ensuring
//! that the result does not occur (Garantenstellung), and if the omission
//! **corresponds** to the realisation of the statutory offence by a positive act
//! (Entsprechensklausel). The sentence may be mitigated (§ 49 Abs. 1 StGB).
//!
//! ## § 14 StGB - Handeln für einen anderen (Acting for another)
//!
//! Special personal characteristics (besondere persönliche Merkmale) that found,
//! aggravate or exclude liability are attributed to a representative (organ,
//! director, authorised agent) who acts for the represented person, even if those
//! characteristics are present only in the represented person.
//!
//! # Guarantor positions (Garantenstellungen) - doctrine
//!
//! German doctrine groups guarantor positions into two functions:
//! - **Beschützergarant** (protective guarantor): duty to protect a particular
//!   legal interest (e.g. parents for their children, close community).
//! - **Überwachergarant** (supervisory guarantor): duty to control a source of
//!   danger (e.g. Ingerenz - prior dangerous conduct; control over premises or
//!   things; supervision of third persons).

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};

/// A guarantor position (Garantenstellung) grounding a duty to avert a result
/// (§ 13 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Garantenstellung {
    /// Statutory duty (Gesetz), e.g. parental duty under § 1626 BGB.
    Gesetz,
    /// Voluntary assumption of protection (Übernahme/tatsächliche Übernahme),
    /// e.g. lifeguard, babysitter, physician.
    Uebernahme,
    /// Close personal community (enge Lebensgemeinschaft / Gefahrengemeinschaft).
    Lebensgemeinschaft,
    /// Prior dangerous conduct (Ingerenz / pflichtwidriges Vorverhalten).
    Ingerenz,
    /// Control over a source of danger or premises (Herrschaft über Gefahrenquelle).
    Gefahrenquelle,
    /// Responsibility for the conduct of third persons (Verantwortung für Dritte).
    VerantwortungFuerDritte,
    /// No guarantor position present.
    Keine,
}

impl Garantenstellung {
    /// Whether this constitutes a guarantor position founding a duty to act.
    #[must_use]
    pub fn is_guarantor(&self) -> bool {
        !matches!(self, Garantenstellung::Keine)
    }

    /// Whether this is a protective guarantor (Beschützergarant).
    #[must_use]
    pub fn is_protective(&self) -> bool {
        matches!(
            self,
            Garantenstellung::Gesetz
                | Garantenstellung::Uebernahme
                | Garantenstellung::Lebensgemeinschaft
        )
    }

    /// Whether this is a supervisory guarantor (Überwachergarant).
    #[must_use]
    pub fn is_supervisory(&self) -> bool {
        matches!(
            self,
            Garantenstellung::Ingerenz
                | Garantenstellung::Gefahrenquelle
                | Garantenstellung::VerantwortungFuerDritte
        )
    }
}

/// A case of alleged liability by omission under § 13 StGB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnterlassungsCase {
    /// The result that should have been averted (abzuwendender Erfolg).
    pub abzuwendender_erfolg: String,
    /// The guarantor position relied upon.
    pub garantenstellung: Garantenstellung,
    /// Whether averting the result was physically possible (Möglichkeit der
    /// Erfolgsabwendung).
    pub erfolgsabwendung_moeglich: bool,
    /// Whether averting the result was reasonable for the offender (Zumutbarkeit).
    pub zumutbar: bool,
    /// Whether the omission corresponds to commission by a positive act
    /// (Entsprechensklausel, § 13 Abs. 1 a.E.).
    pub entspricht_aktivem_tun: bool,
}

/// Validate liability by omission under § 13 StGB.
///
/// Requirements:
/// 1. A guarantor position (Garantenstellung).
/// 2. Possibility and reasonableness of averting the result.
/// 3. Equivalence of the omission to commission by a positive act.
///
/// # Errors
/// - [`StgbError::NoGarantenstellung`] if no guarantor position exists.
/// - [`StgbError::ErfolgsabwendungUnmoeglich`] if averting the result was
///   impossible or unreasonable.
/// - [`StgbError::TatbestandNotFulfilled`] if the omission does not correspond to
///   commission by a positive act.
pub fn validate_unterlassung(case: &UnterlassungsCase) -> Result<()> {
    if !case.garantenstellung.is_guarantor() {
        return Err(StgbError::NoGarantenstellung);
    }
    if !case.erfolgsabwendung_moeglich || !case.zumutbar {
        return Err(StgbError::ErfolgsabwendungUnmoeglich);
    }
    if !case.entspricht_aktivem_tun {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Entsprechensklausel (§ 13 Abs. 1 StGB)".to_string(),
        });
    }
    Ok(())
}

/// The basis on which a representative acts for another under § 14 StGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VertretungsBasis {
    /// Organ of a legal person (vertretungsberechtigtes Organ), § 14 Abs. 1 Nr. 1.
    OrganJuristischePerson,
    /// Partner authorised to represent a partnership, § 14 Abs. 1 Nr. 2.
    VertretungsberechtigterGesellschafter,
    /// Statutory representative (gesetzlicher Vertreter), § 14 Abs. 1 Nr. 3.
    GesetzlicherVertreter,
    /// Person charged with managing a business/establishment, § 14 Abs. 2 Nr. 1.
    Betriebsbeauftragter,
}

/// A case of acting for another under § 14 StGB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VertretungsCase {
    /// Basis of the representation.
    pub basis: VertretungsBasis,
    /// The special personal characteristic (besonderes persönliches Merkmal)
    /// that founds liability, present in the represented person.
    pub besonderes_merkmal: String,
    /// Whether the representative acted within the scope of the representation
    /// (in dieser Eigenschaft gehandelt).
    pub handeln_in_dieser_eigenschaft: bool,
}

/// Validate attribution of a special personal characteristic to a representative
/// under § 14 StGB.
///
/// # Errors
/// - [`StgbError::InvalidField`] if the special characteristic is empty.
/// - [`StgbError::TatbestandNotFulfilled`] if the representative did not act in
///   the relevant capacity.
pub fn validate_vertretung(case: &VertretungsCase) -> Result<()> {
    if case.besonderes_merkmal.trim().is_empty() {
        return Err(StgbError::InvalidField {
            field: "besonderes persönliches Merkmal (§ 14 StGB)".to_string(),
        });
    }
    if !case.handeln_in_dieser_eigenschaft {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Handeln in der Vertretereigenschaft (§ 14 StGB)".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_case() -> UnterlassungsCase {
        UnterlassungsCase {
            abzuwendender_erfolg: "Tod des Kindes".to_string(),
            garantenstellung: Garantenstellung::Gesetz,
            erfolgsabwendung_moeglich: true,
            zumutbar: true,
            entspricht_aktivem_tun: true,
        }
    }

    #[test]
    fn guarantor_classification() {
        assert!(Garantenstellung::Gesetz.is_protective());
        assert!(Garantenstellung::Ingerenz.is_supervisory());
        assert!(!Garantenstellung::Keine.is_guarantor());
    }

    #[test]
    fn omission_with_guarantor_is_valid() {
        assert!(validate_unterlassung(&valid_case()).is_ok());
    }

    #[test]
    fn omission_without_guarantor_fails() {
        let c = UnterlassungsCase {
            garantenstellung: Garantenstellung::Keine,
            ..valid_case()
        };
        assert!(matches!(
            validate_unterlassung(&c),
            Err(StgbError::NoGarantenstellung)
        ));
    }

    #[test]
    fn omission_impossible_fails() {
        let c = UnterlassungsCase {
            erfolgsabwendung_moeglich: false,
            ..valid_case()
        };
        assert!(matches!(
            validate_unterlassung(&c),
            Err(StgbError::ErfolgsabwendungUnmoeglich)
        ));
    }

    #[test]
    fn omission_not_corresponding_fails() {
        let c = UnterlassungsCase {
            entspricht_aktivem_tun: false,
            ..valid_case()
        };
        assert!(matches!(
            validate_unterlassung(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn vertretung_attributes_characteristic() {
        let c = VertretungsCase {
            basis: VertretungsBasis::OrganJuristischePerson,
            besonderes_merkmal: "Arbeitgebereigenschaft".to_string(),
            handeln_in_dieser_eigenschaft: true,
        };
        assert!(validate_vertretung(&c).is_ok());
    }

    #[test]
    fn vertretung_outside_capacity_fails() {
        let c = VertretungsCase {
            basis: VertretungsBasis::GesetzlicherVertreter,
            besonderes_merkmal: "Vermögensbetreuungspflicht".to_string(),
            handeln_in_dieser_eigenschaft: false,
        };
        assert!(matches!(
            validate_vertretung(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }
}
