//! StGB §§ 22-30 - Attempt and Participation (Versuch und Teilnahme)
//!
//! # Attempt (Versuch), §§ 22-24 StGB
//!
//! ## § 22 StGB - Begriffsbestimmung (Definition)
//!
//! > Eine Straftat versucht, wer nach seiner Vorstellung von der Tat zur
//! > Verwirklichung des Tatbestandes unmittelbar ansetzt.
//!
//! **English**: A person attempts an offence if, according to his conception of
//! the act, he **takes an immediate step** towards the realisation of the offence
//! (Tatentschluss + unmittelbares Ansetzen).
//!
//! ## § 23 StGB - Strafbarkeit des Versuchs
//!
//! - The attempt of a **felony** (Verbrechen, § 12 Abs. 1 StGB: minimum penalty
//!   ≥ 1 year imprisonment) is always punishable.
//! - The attempt of a **misdemeanour** (Vergehen) is punishable only where the
//!   law **expressly** so provides (§ 23 Abs. 1 StGB).
//! - The attempt may be punished more leniently than the completed offence
//!   (§ 23 Abs. 2 StGB → § 49 Abs. 1 StGB).
//!
//! ## § 24 StGB - Rücktritt (Withdrawal)
//!
//! - **Unfinished attempt** (unbeendeter Versuch): the offender becomes exempt by
//!   **voluntarily abandoning** further execution (§ 24 Abs. 1 S. 1 Alt. 1).
//! - **Finished attempt** (beendeter Versuch): the offender must **voluntarily
//!   prevent** completion (§ 24 Abs. 1 S. 1 Alt. 2).
//! - If the offence is not completed without the offender's contribution, he is
//!   exempt if he **voluntarily and earnestly endeavours** to prevent completion
//!   (§ 24 Abs. 1 S. 2).
//!
//! Withdrawal must be **voluntary** (freiwillig): the offender desists although he
//! still could complete the offence ("Ich will nicht, obwohl ich könnte").
//!
//! # Participation (Täterschaft und Teilnahme), §§ 25-30 StGB
//!
//! - **§ 25** - Perpetration: direct perpetrator (unmittelbarer Täter), indirect
//!   perpetrator (mittelbarer Täter - "durch einen anderen"), co-perpetrators
//!   (Mittäter - "gemeinschaftlich").
//! - **§ 26** - Incitement (Anstiftung): intentionally **determining another** to
//!   commit an intentional unlawful act; punished like the perpetrator.
//! - **§ 27** - Aiding (Beihilfe): intentionally **assisting** another's
//!   intentional unlawful act; the sentence is mitigated (§ 27 Abs. 2 → § 49
//!   Abs. 1 StGB).
//! - **§§ 28-29** - special personal characteristics; independent punishment of
//!   participants.
//! - **§ 30** - attempted participation (versuchte Beteiligung) in a felony.
//!
//! Participation is **accessory** (limitierte Akzessorietät): it requires an
//! intentional and unlawful principal act (vorsätzliche rechtswidrige Haupttat),
//! but not a culpable one.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};

/// Classification of an offence as felony or misdemeanour (§ 12 StGB), decisive
/// for the punishability of an attempt (§ 23 Abs. 1 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Deliktstyp {
    /// Felony (Verbrechen): minimum penalty of at least one year (§ 12 Abs. 1).
    Verbrechen,
    /// Misdemeanour (Vergehen): lesser minimum penalty (§ 12 Abs. 2).
    Vergehen,
}

/// Stage of the attempt, relevant for the requirements of withdrawal (§ 24 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Versuchsstadium {
    /// Unfinished attempt (unbeendeter Versuch): not yet all acts done.
    Unbeendet,
    /// Finished attempt (beendeter Versuch): all acts done, result still pending.
    Beendet,
}

/// A withdrawal from the attempt (Rücktritt, § 24 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ruecktritt {
    /// Stage of the attempt at the moment of withdrawal.
    pub stadium: Versuchsstadium,
    /// Whether the withdrawal was voluntary (freiwillig).
    pub freiwillig: bool,
    /// Whether the offender abandoned further execution (Aufgabe der weiteren
    /// Tatausführung) - decisive for the unfinished attempt.
    pub tat_aufgegeben: bool,
    /// Whether the offender prevented completion (Verhinderung der Vollendung) -
    /// decisive for the finished attempt.
    pub vollendung_verhindert: bool,
    /// Whether the offence has already been completed (vollendet); a completed
    /// offence rules out withdrawal.
    pub bereits_vollendet: bool,
}

impl Ruecktritt {
    /// Whether this withdrawal exempts from punishment under § 24 Abs. 1 StGB.
    ///
    /// Requires a voluntary withdrawal at an attempt that has not yet been
    /// completed:
    /// - unfinished attempt → abandoning further execution suffices;
    /// - finished attempt → completion must be prevented.
    #[must_use]
    pub fn is_strafbefreiend(&self) -> bool {
        if self.bereits_vollendet || !self.freiwillig {
            return false;
        }
        match self.stadium {
            Versuchsstadium::Unbeendet => self.tat_aufgegeben,
            Versuchsstadium::Beendet => self.vollendung_verhindert,
        }
    }
}

/// An attempt (Versuch, §§ 22-24 StGB).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Versuch {
    /// Description of the intended offence.
    pub tat: String,
    /// Classification as felony or misdemeanour (§ 12 StGB).
    pub deliktstyp: Deliktstyp,
    /// Whether the offence's attempt is expressly punishable (only relevant for a
    /// misdemeanour; ignored for a felony, which is always punishable).
    pub versuch_ausdruecklich_strafbar: bool,
    /// Whether the offender had made the decision to commit the act (Tatentschluss).
    pub tatentschluss: bool,
    /// Whether the offender took an immediate step (unmittelbares Ansetzen, § 22).
    pub unmittelbares_ansetzen: bool,
    /// A withdrawal, if asserted (§ 24 StGB).
    pub ruecktritt: Option<Ruecktritt>,
}

/// Validate the punishability of an attempt under §§ 22-24 StGB.
///
/// # Errors
/// - [`StgbError::VersuchNichtStrafbar`] if the attempt of a misdemeanour is not
///   expressly declared punishable (§ 23 Abs. 1 StGB).
/// - [`StgbError::NoSchuldform`] if the offender lacked the decision to commit
///   the act (Tatentschluss).
/// - [`StgbError::NoUnmittelbaresAnsetzen`] if there was no immediate step
///   (§ 22 StGB) - i.e. mere preparation (straflose Vorbereitung).
/// - [`StgbError::StrafbefreienderRuecktritt`] if a valid withdrawal exempts the
///   offender from punishment (§ 24 StGB).
pub fn validate_versuch(v: &Versuch) -> Result<()> {
    // § 23 Abs. 1 StGB - punishability of the attempt.
    if matches!(v.deliktstyp, Deliktstyp::Vergehen) && !v.versuch_ausdruecklich_strafbar {
        return Err(StgbError::VersuchNichtStrafbar);
    }
    // § 22 StGB - Tatentschluss (subjective) and unmittelbares Ansetzen (objective).
    if !v.tatentschluss {
        return Err(StgbError::NoSchuldform);
    }
    if !v.unmittelbares_ansetzen {
        return Err(StgbError::NoUnmittelbaresAnsetzen);
    }
    // § 24 StGB - withdrawal exempts from punishment.
    if let Some(r) = &v.ruecktritt
        && r.is_strafbefreiend()
    {
        return Err(StgbError::StrafbefreienderRuecktritt);
    }
    Ok(())
}

/// Form of perpetration (Täterschaft, § 25 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Taeterschaft {
    /// Direct perpetrator (unmittelbarer Täter, § 25 Abs. 1 Alt. 1).
    UnmittelbarerTaeter,
    /// Indirect perpetrator (mittelbarer Täter "durch einen anderen", § 25 Abs. 1
    /// Alt. 2).
    MittelbarerTaeter,
    /// Co-perpetrator (Mittäter "gemeinschaftlich", § 25 Abs. 2).
    Mittaeter,
}

/// Form of participation (Teilnahme, §§ 26-27 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Teilnahme {
    /// Incitement (Anstiftung, § 26 StGB) - punished like the perpetrator.
    Anstiftung,
    /// Aiding (Beihilfe, § 27 StGB) - sentence mitigated (§ 27 Abs. 2 StGB).
    Beihilfe,
}

impl Teilnahme {
    /// Whether the participant's sentence is mandatorily mitigated.
    ///
    /// Incitement (§ 26) is punished like perpetration; aiding (§ 27 Abs. 2) is
    /// mitigated under § 49 Abs. 1 StGB.
    #[must_use]
    pub fn sentence_mitigated(&self) -> bool {
        matches!(self, Teilnahme::Beihilfe)
    }
}

/// The principal act (Haupttat) to which participation attaches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Haupttat {
    /// Description of the principal offence.
    pub tat: String,
    /// Whether the principal acted intentionally (vorsätzlich).
    pub vorsaetzlich: bool,
    /// Whether the principal act is unlawful (rechtswidrig).
    pub rechtswidrig: bool,
}

impl Haupttat {
    /// Whether the principal act suffices for accessory participation under the
    /// limited-accessoriety doctrine (limitierte Akzessorietät): an intentional
    /// and unlawful - though not necessarily culpable - principal act.
    #[must_use]
    pub fn supports_participation(&self) -> bool {
        self.vorsaetzlich && self.rechtswidrig
    }
}

/// A case of participation (§§ 26-27 StGB).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeilnahmeCase {
    /// Form of participation.
    pub form: Teilnahme,
    /// The principal act.
    pub haupttat: Haupttat,
    /// Whether the participant acted with double intent (doppelter
    /// Teilnehmervorsatz): intent as to his own contribution **and** as to the
    /// principal act.
    pub doppelter_vorsatz: bool,
    /// For incitement: whether the participant determined the principal to the act
    /// (Bestimmen zur Tat, § 26 StGB).
    pub bestimmen: bool,
    /// For aiding: whether the participant rendered assistance (Hilfeleisten,
    /// § 27 StGB).
    pub hilfeleisten: bool,
}

/// Validate participation under §§ 26-27 StGB.
///
/// # Errors
/// - [`StgbError::NoHaupttat`] if there is no intentional unlawful principal act.
/// - [`StgbError::NoSchuldform`] if the participant lacked double intent.
/// - [`StgbError::NoBestimmen`] if incitement is asserted without determining the
///   principal to the act.
/// - [`StgbError::NoHilfeleisten`] if aiding is asserted without rendering
///   assistance.
pub fn validate_teilnahme(c: &TeilnahmeCase) -> Result<()> {
    if !c.haupttat.supports_participation() {
        return Err(StgbError::NoHaupttat);
    }
    if !c.doppelter_vorsatz {
        return Err(StgbError::NoSchuldform);
    }
    match c.form {
        Teilnahme::Anstiftung => {
            if !c.bestimmen {
                return Err(StgbError::NoBestimmen);
            }
        }
        Teilnahme::Beihilfe => {
            if !c.hilfeleisten {
                return Err(StgbError::NoHilfeleisten);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn felony_attempt() -> Versuch {
        Versuch {
            tat: "Totschlag".to_string(),
            deliktstyp: Deliktstyp::Verbrechen,
            versuch_ausdruecklich_strafbar: false,
            tatentschluss: true,
            unmittelbares_ansetzen: true,
            ruecktritt: None,
        }
    }

    #[test]
    fn felony_attempt_is_punishable() {
        assert!(validate_versuch(&felony_attempt()).is_ok());
    }

    #[test]
    fn misdemeanour_attempt_needs_express_provision() {
        let v = Versuch {
            tat: "einfacher Hausfriedensbruch".to_string(),
            deliktstyp: Deliktstyp::Vergehen,
            versuch_ausdruecklich_strafbar: false,
            ..felony_attempt()
        };
        assert!(matches!(
            validate_versuch(&v),
            Err(StgbError::VersuchNichtStrafbar)
        ));
        let v_ok = Versuch {
            versuch_ausdruecklich_strafbar: true,
            ..v
        };
        assert!(validate_versuch(&v_ok).is_ok());
    }

    #[test]
    fn mere_preparation_is_not_attempt() {
        let v = Versuch {
            unmittelbares_ansetzen: false,
            ..felony_attempt()
        };
        assert!(matches!(
            validate_versuch(&v),
            Err(StgbError::NoUnmittelbaresAnsetzen)
        ));
    }

    #[test]
    fn voluntary_withdrawal_unfinished_exempts() {
        let r = Ruecktritt {
            stadium: Versuchsstadium::Unbeendet,
            freiwillig: true,
            tat_aufgegeben: true,
            vollendung_verhindert: false,
            bereits_vollendet: false,
        };
        assert!(r.is_strafbefreiend());
        let v = Versuch {
            ruecktritt: Some(r),
            ..felony_attempt()
        };
        assert!(matches!(
            validate_versuch(&v),
            Err(StgbError::StrafbefreienderRuecktritt)
        ));
    }

    #[test]
    fn finished_attempt_requires_preventing_completion() {
        // Merely abandoning is insufficient for a finished attempt.
        let r = Ruecktritt {
            stadium: Versuchsstadium::Beendet,
            freiwillig: true,
            tat_aufgegeben: true,
            vollendung_verhindert: false,
            bereits_vollendet: false,
        };
        assert!(!r.is_strafbefreiend());
        let r_ok = Ruecktritt {
            vollendung_verhindert: true,
            ..r
        };
        assert!(r_ok.is_strafbefreiend());
    }

    #[test]
    fn involuntary_withdrawal_does_not_exempt() {
        let r = Ruecktritt {
            stadium: Versuchsstadium::Unbeendet,
            freiwillig: false,
            tat_aufgegeben: true,
            vollendung_verhindert: false,
            bereits_vollendet: false,
        };
        assert!(!r.is_strafbefreiend());
    }

    #[test]
    fn completed_offence_rules_out_withdrawal() {
        let r = Ruecktritt {
            stadium: Versuchsstadium::Beendet,
            freiwillig: true,
            tat_aufgegeben: true,
            vollendung_verhindert: true,
            bereits_vollendet: true,
        };
        assert!(!r.is_strafbefreiend());
    }

    fn valid_principal() -> Haupttat {
        Haupttat {
            tat: "Diebstahl".to_string(),
            vorsaetzlich: true,
            rechtswidrig: true,
        }
    }

    #[test]
    fn incitement_requires_determining() {
        let c = TeilnahmeCase {
            form: Teilnahme::Anstiftung,
            haupttat: valid_principal(),
            doppelter_vorsatz: true,
            bestimmen: true,
            hilfeleisten: false,
        };
        assert!(validate_teilnahme(&c).is_ok());
        let c_no = TeilnahmeCase {
            bestimmen: false,
            ..c
        };
        assert!(matches!(
            validate_teilnahme(&c_no),
            Err(StgbError::NoBestimmen)
        ));
    }

    #[test]
    fn aiding_requires_assistance_and_is_mitigated() {
        assert!(Teilnahme::Beihilfe.sentence_mitigated());
        assert!(!Teilnahme::Anstiftung.sentence_mitigated());
        let c = TeilnahmeCase {
            form: Teilnahme::Beihilfe,
            haupttat: valid_principal(),
            doppelter_vorsatz: true,
            bestimmen: false,
            hilfeleisten: true,
        };
        assert!(validate_teilnahme(&c).is_ok());
        let c_no = TeilnahmeCase {
            hilfeleisten: false,
            ..c
        };
        assert!(matches!(
            validate_teilnahme(&c_no),
            Err(StgbError::NoHilfeleisten)
        ));
    }

    #[test]
    fn participation_requires_intentional_unlawful_principal() {
        let c = TeilnahmeCase {
            form: Teilnahme::Beihilfe,
            haupttat: Haupttat {
                tat: "fahrlässige Tat".to_string(),
                vorsaetzlich: false,
                rechtswidrig: true,
            },
            doppelter_vorsatz: true,
            bestimmen: false,
            hilfeleisten: true,
        };
        assert!(matches!(validate_teilnahme(&c), Err(StgbError::NoHaupttat)));
    }

    #[test]
    fn taeterschaft_variants_exist() {
        // Compile-time coverage of the perpetration enum.
        let _ = Taeterschaft::UnmittelbarerTaeter;
        let _ = Taeterschaft::MittelbarerTaeter;
        assert_ne!(Taeterschaft::Mittaeter, Taeterschaft::UnmittelbarerTaeter);
    }
}
