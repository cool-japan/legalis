//! VwVfG §§ 35-49 - The Administrative Act (Verwaltungsakt) and its life-cycle
//!
//! ## § 35 S. 1 VwVfG - Definition (Begriff des Verwaltungsakts)
//!
//! > Verwaltungsakt ist jede Verfügung, Entscheidung oder andere hoheitliche
//! > Maßnahme, die eine Behörde zur Regelung eines Einzelfalls auf dem Gebiet des
//! > öffentlichen Rechts trifft und die auf unmittelbare Rechtswirkung nach außen
//! > gerichtet ist.
//!
//! **English**: An administrative act is any order, decision or other **sovereign
//! measure** taken by an **authority** to **regulate** an **individual case** in the
//! field of **public law** and intended to have **direct external legal effect**.
//!
//! The five elements (Tatbestandsmerkmale) are:
//! 1. hoheitliche Maßnahme einer Behörde (sovereign measure of an authority),
//! 2. auf dem Gebiet des öffentlichen Rechts (in the field of public law),
//! 3. zur Regelung (a regulation, i.e. directed at a legal consequence),
//! 4. eines Einzelfalls (of an individual case),
//! 5. mit unmittelbarer Rechtswirkung nach außen (direct external legal effect).
//!
//! ## § 35 S. 2 VwVfG - Allgemeinverfügung (general order)
//!
//! A general order is an administrative act directed at a group of people determined
//! or determinable by general criteria, or concerning the public-law character of a
//! thing or its use by the public.
//!
//! ## § 36 VwVfG - Nebenbestimmungen (ancillary provisions)
//! ## §§ 41, 43 VwVfG - Bekanntgabe / Wirksamkeit (notification / effectiveness)
//! ## § 44 VwVfG - Nichtigkeit (nullity, Evidenztheorie + Katalog)
//! ## §§ 48, 49 VwVfG - Rücknahme / Widerruf (retraction / revocation)

use serde::{Deserialize, Serialize};

use crate::verwaltungsrecht::error::{Result, VwVfGError};

use chrono::NaiveDate;

/// The kind of regulatory content of an administrative act.
///
/// Art der Regelungswirkung eines Verwaltungsakts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VAArt {
    /// Commanding act (Befehl): imposes a duty to act, tolerate or refrain.
    Befehl,
    /// Shaping act (Gestaltung): creates, alters or ends a legal relationship.
    Gestaltung,
    /// Declaratory act (Feststellung): bindingly establishes a legal status.
    Feststellung,
}

impl VAArt {
    /// A short bilingual label of the act type.
    ///
    /// Kurzbezeichnung der Verwaltungsaktsart.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            VAArt::Befehl => "befehlender VA / commanding act",
            VAArt::Gestaltung => "gestaltender VA / shaping act",
            VAArt::Feststellung => "feststellender VA / declaratory act",
        }
    }
}

/// The effect direction of an administrative act, relevant for §§ 48/49 VwVfG.
///
/// Wirkungsrichtung des Verwaltungsakts (relevant für §§ 48/49 VwVfG).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VAWirkung {
    /// Favourable act (begünstigender VA): establishes or confirms a right/benefit.
    Beguenstigend,
    /// Burdening act (belastender VA): imposes a burden.
    Belastend,
    /// Act with third-party effect (VA mit Drittwirkung): favours one, burdens another.
    MitDrittwirkung,
}

impl VAWirkung {
    /// Whether the act is (at least partly) favourable to the addressee.
    ///
    /// Ob der Verwaltungsakt (zumindest auch) begünstigend wirkt.
    #[must_use]
    pub fn ist_beguenstigend(&self) -> bool {
        matches!(self, VAWirkung::Beguenstigend | VAWirkung::MitDrittwirkung)
    }
}

/// An administrative act (Verwaltungsakt), § 35 VwVfG.
///
/// Models the five definitional elements of § 35 sent. 1 VwVfG plus § 35 sent. 2
/// (Allgemeinverfügung) and the favourable/burdening distinction relevant for
/// §§ 48/49 VwVfG.
///
/// Verwaltungsakt im Sinne des § 35 VwVfG mit seinen fünf Tatbestandsmerkmalen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verwaltungsakt {
    /// Issuing authority (erlassende Behörde). Must be named (§ 37 Abs. 3 VwVfG).
    pub behoerde: String,
    /// Whether the act is a sovereign measure of an authority
    /// (hoheitliche Maßnahme - element 1 of § 35 sent. 1).
    pub hoheitliche_massnahme: bool,
    /// Whether it is in the field of public law
    /// (öffentliches Recht - element 2).
    pub oeffentliches_recht: bool,
    /// Whether it is a regulation directed at a legal consequence
    /// (Regelung - element 3).
    pub regelung: bool,
    /// Whether it concerns an individual case
    /// (Einzelfall - element 4).
    pub einzelfall: bool,
    /// Whether it has direct external legal effect
    /// (unmittelbare Rechtswirkung nach außen - element 5).
    pub aussenwirkung: bool,
    /// Whether the act is a general order (Allgemeinverfügung, § 35 S. 2 VwVfG).
    /// A general order satisfies the "individual case" element by concerning a
    /// group determined by general criteria or a thing.
    pub ist_allgemeinverfuegung: bool,
    /// Whether the act is favourable (begünstigend) rather than burdening
    /// (belastend); relevant for §§ 48/49 VwVfG.
    pub beguenstigend: bool,
    /// The regulatory type (Art der Regelung).
    pub art: VAArt,
    /// The substantive content of the act (Regelungsinhalt / Tenor).
    pub inhalt: String,
}

impl Verwaltungsakt {
    /// Whether all five elements of § 35 sent. 1 VwVfG are satisfied.
    ///
    /// The "individual case" element (Einzelfall) is satisfied either by an
    /// ordinary concrete-individual regulation (`einzelfall == true`) or by a
    /// general order under § 35 sent. 2 VwVfG (`ist_allgemeinverfuegung == true`).
    ///
    /// Ob alle fünf Tatbestandsmerkmale des § 35 S. 1 VwVfG vorliegen.
    #[must_use]
    pub fn is_verwaltungsakt(&self) -> bool {
        self.hoheitliche_massnahme
            && self.oeffentliches_recht
            && self.regelung
            && (self.einzelfall || self.ist_allgemeinverfuegung)
            && self.aussenwirkung
    }
}

/// Validate that a measure is an administrative act under § 35 VwVfG.
///
/// Checks each element of § 35 sent. 1 VwVfG and returns the first missing one.
///
/// Prüft die Verwaltungsaktsqualität nach § 35 VwVfG.
///
/// # Errors
/// - [`VwVfGError::EmptyField`] if no issuing authority is named (§ 37 Abs. 3 VwVfG).
/// - [`VwVfGError::MissingHoheitlicheMassnahme`] if it is not a sovereign measure.
/// - [`VwVfGError::NoRegelung`] if it contains no regulation.
/// - [`VwVfGError::NoAussenwirkung`] if it lacks direct external legal effect.
/// - [`VwVfGError::NotEinzelfall`] if it neither concerns an individual case nor is
///   a general order (§ 35 S. 2 VwVfG).
/// - [`VwVfGError::NotAVerwaltungsakt`] if it is not in the field of public law.
pub fn validate_verwaltungsakt(va: &Verwaltungsakt) -> Result<()> {
    if va.behoerde.trim().is_empty() {
        return Err(VwVfGError::EmptyField {
            field: "behoerde".to_string(),
        });
    }
    if !va.hoheitliche_massnahme {
        return Err(VwVfGError::MissingHoheitlicheMassnahme);
    }
    if !va.oeffentliches_recht {
        // A measure on the field of private law is not a Verwaltungsakt at all.
        return Err(VwVfGError::NotAVerwaltungsakt);
    }
    if !va.regelung {
        return Err(VwVfGError::NoRegelung);
    }
    if !va.einzelfall && !va.ist_allgemeinverfuegung {
        return Err(VwVfGError::NotEinzelfall);
    }
    if !va.aussenwirkung {
        return Err(VwVfGError::NoAussenwirkung);
    }
    Ok(())
}

/// An ancillary provision to an administrative act, § 36 VwVfG.
///
/// Nebenbestimmung zum Verwaltungsakt (§ 36 VwVfG).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nebenbestimmung {
    /// Time limit (Befristung, § 36 Abs. 2 Nr. 1 VwVfG).
    Befristung,
    /// Condition (Bedingung, § 36 Abs. 2 Nr. 2 VwVfG).
    Bedingung,
    /// Obligation / charge (Auflage, § 36 Abs. 2 Nr. 4 VwVfG).
    Auflage,
    /// Reservation of revocation (Widerrufsvorbehalt, § 36 Abs. 2 Nr. 3 VwVfG).
    Widerrufsvorbehalt,
    /// Reservation of a subsequent obligation (Auflagenvorbehalt, § 36 Abs. 2 Nr. 5 VwVfG).
    Auflagenvorbehalt,
}

impl Nebenbestimmung {
    /// The § 36 Abs. 2 citation of the ancillary provision.
    ///
    /// Das §-Zitat (§ 36 Abs. 2 VwVfG) der Nebenbestimmung.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            Nebenbestimmung::Befristung => "§ 36 Abs. 2 Nr. 1 VwVfG",
            Nebenbestimmung::Bedingung => "§ 36 Abs. 2 Nr. 2 VwVfG",
            Nebenbestimmung::Widerrufsvorbehalt => "§ 36 Abs. 2 Nr. 3 VwVfG",
            Nebenbestimmung::Auflage => "§ 36 Abs. 2 Nr. 4 VwVfG",
            Nebenbestimmung::Auflagenvorbehalt => "§ 36 Abs. 2 Nr. 5 VwVfG",
        }
    }
}

/// Validate an ancillary provision under § 36 VwVfG.
///
/// A discretionary administrative act (Ermessens-VA) may be furnished with any
/// ancillary provision (§ 36 Abs. 2 VwVfG). A bound administrative act (gebundener
/// VA) may only carry an ancillary provision where the law so permits or where the
/// provision serves to ensure that the act's statutory requirements are met
/// (§ 36 Abs. 1 VwVfG). This function models the typical case: for a bound act, any
/// of the discretionary ancillary provisions is treated as inadmissible.
///
/// Prüft die Zulässigkeit einer Nebenbestimmung nach § 36 VwVfG.
///
/// # Errors
/// - [`VwVfGError::UnzulaessigeNebenbestimmung`] if a discretionary ancillary
///   provision is attached to a bound administrative act (§ 36 Abs. 1 VwVfG).
pub fn validate_nebenbestimmung(va_is_ermessen: bool, _nb: Nebenbestimmung) -> Result<()> {
    if va_is_ermessen {
        // § 36 Abs. 2 VwVfG: discretionary act may carry any ancillary provision.
        Ok(())
    } else {
        // § 36 Abs. 1 VwVfG: bound act only to ensure its statutory requirements
        // are met - the freely chosen ancillary provision is inadmissible here.
        Err(VwVfGError::UnzulaessigeNebenbestimmung)
    }
}

/// Notification of an administrative act, § 41 VwVfG.
///
/// Bekanntgabe eines Verwaltungsakts (§ 41 VwVfG).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bekanntgabe {
    /// Whether notification has actually been effected (Bekanntgabe erfolgt).
    pub erfolgt: bool,
    /// Date of notification (Datum der Bekanntgabe), if known.
    pub datum: Option<NaiveDate>,
}

impl Bekanntgabe {
    /// Create an effected notification on the given date.
    ///
    /// Erzeugt eine erfolgte Bekanntgabe zum angegebenen Datum.
    #[must_use]
    pub fn erfolgt_am(datum: NaiveDate) -> Self {
        Self {
            erfolgt: true,
            datum: Some(datum),
        }
    }

    /// Create a not-yet-effected notification.
    ///
    /// Erzeugt eine noch nicht erfolgte Bekanntgabe.
    #[must_use]
    pub fn nicht_erfolgt() -> Self {
        Self {
            erfolgt: false,
            datum: None,
        }
    }
}

/// Determine whether an administrative act becomes effective, § 43 Abs. 1 VwVfG.
///
/// An administrative act becomes effective (wirksam) vis-à-vis the addressee at the
/// moment it is notified to them (§ 43 Abs. 1 VwVfG). This requires that the measure
/// is in fact an administrative act (§ 35 VwVfG) and that notification has been
/// effected (§ 41 VwVfG). A void administrative act never becomes effective
/// (§ 43 Abs. 3 VwVfG); use [`pruefe_nichtigkeit`] to determine nullity.
///
/// Prüft das Wirksamwerden eines Verwaltungsakts nach § 43 Abs. 1 VwVfG.
///
/// # Errors
/// - [`VwVfGError::NotAVerwaltungsakt`] (or a more specific § 35 error) if the
///   measure is not an administrative act.
/// - [`VwVfGError::NotBekanntgegeben`] if notification has not been effected.
pub fn wird_wirksam(va: &Verwaltungsakt, bg: &Bekanntgabe) -> Result<()> {
    validate_verwaltungsakt(va)?;
    if !bg.erfolgt {
        return Err(VwVfGError::NotBekanntgegeben);
    }
    Ok(())
}

/// Inputs for the nullity check under § 44 VwVfG.
///
/// Eingaben für die Nichtigkeitsprüfung nach § 44 VwVfG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NichtigkeitsCheck {
    /// Whether the act suffers a particularly serious defect
    /// (besonders schwerwiegender Fehler, § 44 Abs. 1 VwVfG).
    pub schwerwiegender_fehler: bool,
    /// Whether that defect is obvious to a sensible observer
    /// (Offensichtlichkeit, § 44 Abs. 1 VwVfG - Evidenztheorie).
    pub offensichtlich: bool,
    /// Whether one of the absolute catalogue cases applies
    /// (Katalogfall, § 44 Abs. 2 VwVfG), e.g. a written act not showing the
    /// issuing authority, an act impossible to perform, or one demanding an
    /// unlawful act.
    pub abs2_katalogfall: bool,
}

/// Check the nullity of an administrative act under § 44 VwVfG.
///
/// An administrative act is void (nichtig) if it suffers a particularly serious and
/// at the same time obvious defect (§ 44 Abs. 1 VwVfG, Evidenztheorie), and is void
/// in any event in the absolute catalogue cases of § 44 Abs. 2 VwVfG.
///
/// Prüft die Nichtigkeit eines Verwaltungsakts nach § 44 VwVfG.
///
/// # Errors
/// - [`VwVfGError::Nichtig`] if a catalogue case applies (§ 44 Abs. 2 VwVfG) or the
///   defect is both serious and obvious (§ 44 Abs. 1 VwVfG).
pub fn pruefe_nichtigkeit(c: &NichtigkeitsCheck) -> Result<()> {
    if c.abs2_katalogfall {
        return Err(VwVfGError::Nichtig {
            grund: "Katalogfall des § 44 Abs. 2 VwVfG / catalogue case of § 44 para. 2 VwVfG"
                .to_string(),
        });
    }
    if c.schwerwiegender_fehler && c.offensichtlich {
        return Err(VwVfGError::Nichtig {
            grund: "besonders schwerwiegender und offensichtlicher Fehler (§ 44 Abs. 1 VwVfG) / \
                    particularly serious and obvious defect (§ 44 para. 1 VwVfG)"
                .to_string(),
        });
    }
    Ok(())
}

/// Check the retraction of an unlawful administrative act under § 48 VwVfG.
///
/// § 48 VwVfG governs the retraction (Rücknahme) of an **unlawful** administrative
/// act. An unlawful burdening act may be retracted freely (§ 48 Abs. 1 S. 1 VwVfG).
/// An unlawful **favourable** act may be retracted only within the limits protecting
/// legitimate expectations (Vertrauensschutz, § 48 Abs. 2/3 VwVfG): where the
/// addressee's reliance is worthy of protection, retraction with effect for the past
/// is restricted. If the act is in fact lawful, retraction under § 48 is not the
/// correct instrument - revocation under § 49 VwVfG applies instead.
///
/// Prüft die Rücknahme eines rechtswidrigen Verwaltungsakts nach § 48 VwVfG.
///
/// # Errors
/// - [`VwVfGError::WiderrufUnzulaessig`] if the act is lawful (then § 49 territory).
/// - [`VwVfGError::RuecknahmeUnzulaessig`] if a favourable act is to be retracted but
///   the addressee's reliance is worthy of protection (§ 48 Abs. 2/3 VwVfG).
pub fn pruefe_ruecknahme(
    va_rechtswidrig: bool,
    beguenstigend: bool,
    vertrauen_schutzwuerdig: bool,
) -> Result<()> {
    if !va_rechtswidrig {
        // § 48 VwVfG presupposes an unlawful act; a lawful act is § 49 territory.
        return Err(VwVfGError::WiderrufUnzulaessig);
    }
    if beguenstigend && vertrauen_schutzwuerdig {
        return Err(VwVfGError::RuecknahmeUnzulaessig);
    }
    Ok(())
}

/// Check the revocation of a lawful administrative act under § 49 VwVfG.
///
/// § 49 VwVfG governs the revocation (Widerruf) of a **lawful** administrative act. A
/// lawful **burdening** act may, as a rule, be revoked with effect for the future
/// (§ 49 Abs. 1 VwVfG). A lawful **favourable** act may be revoked only on one of the
/// grounds listed in § 49 Abs. 2/3 VwVfG. If the act is in fact unlawful, revocation
/// under § 49 is not the correct instrument - retraction under § 48 VwVfG applies.
///
/// Prüft den Widerruf eines rechtmäßigen Verwaltungsakts nach § 49 VwVfG.
///
/// # Errors
/// - [`VwVfGError::RuecknahmeUnzulaessig`] if the act is unlawful (then § 48 territory).
/// - [`VwVfGError::WiderrufUnzulaessig`] if a favourable lawful act is to be revoked
///   without a ground under § 49 Abs. 2/3 VwVfG.
pub fn pruefe_widerruf(
    va_rechtmaessig: bool,
    beguenstigend: bool,
    widerrufsgrund_vorliegt: bool,
) -> Result<()> {
    if !va_rechtmaessig {
        // § 49 VwVfG presupposes a lawful act; an unlawful act is § 48 territory.
        return Err(VwVfGError::RuecknahmeUnzulaessig);
    }
    if beguenstigend && !widerrufsgrund_vorliegt {
        return Err(VwVfGError::WiderrufUnzulaessig);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_va() -> Verwaltungsakt {
        Verwaltungsakt {
            behoerde: "Ordnungsamt".to_string(),
            hoheitliche_massnahme: true,
            oeffentliches_recht: true,
            regelung: true,
            einzelfall: true,
            aussenwirkung: true,
            ist_allgemeinverfuegung: false,
            beguenstigend: false,
            art: VAArt::Befehl,
            inhalt: "Untersagung des Betriebs".to_string(),
        }
    }

    #[test]
    fn full_definition_is_verwaltungsakt() {
        let va = base_va();
        assert!(va.is_verwaltungsakt());
        assert!(validate_verwaltungsakt(&va).is_ok());
    }

    #[test]
    fn missing_behoerde_is_empty_field() {
        let mut va = base_va();
        va.behoerde = "   ".to_string();
        assert!(matches!(
            validate_verwaltungsakt(&va),
            Err(VwVfGError::EmptyField { .. })
        ));
    }

    #[test]
    fn missing_hoheitliche_massnahme_fails() {
        let mut va = base_va();
        va.hoheitliche_massnahme = false;
        assert!(!va.is_verwaltungsakt());
        assert!(matches!(
            validate_verwaltungsakt(&va),
            Err(VwVfGError::MissingHoheitlicheMassnahme)
        ));
    }

    #[test]
    fn missing_oeffentliches_recht_fails() {
        let mut va = base_va();
        va.oeffentliches_recht = false;
        assert!(matches!(
            validate_verwaltungsakt(&va),
            Err(VwVfGError::NotAVerwaltungsakt)
        ));
    }

    #[test]
    fn missing_regelung_fails() {
        let mut va = base_va();
        va.regelung = false;
        assert!(!va.is_verwaltungsakt());
        assert!(matches!(
            validate_verwaltungsakt(&va),
            Err(VwVfGError::NoRegelung)
        ));
    }

    #[test]
    fn missing_einzelfall_fails() {
        let mut va = base_va();
        va.einzelfall = false;
        assert!(!va.is_verwaltungsakt());
        assert!(matches!(
            validate_verwaltungsakt(&va),
            Err(VwVfGError::NotEinzelfall)
        ));
    }

    #[test]
    fn missing_aussenwirkung_fails() {
        let mut va = base_va();
        va.aussenwirkung = false;
        assert!(!va.is_verwaltungsakt());
        assert!(matches!(
            validate_verwaltungsakt(&va),
            Err(VwVfGError::NoAussenwirkung)
        ));
    }

    #[test]
    fn allgemeinverfuegung_satisfies_einzelfall() {
        let mut va = base_va();
        va.einzelfall = false;
        va.ist_allgemeinverfuegung = true;
        va.inhalt = "Versammlungsauflösung / Verkehrszeichen".to_string();
        assert!(va.is_verwaltungsakt());
        assert!(validate_verwaltungsakt(&va).is_ok());
    }

    #[test]
    fn nebenbestimmung_ok_for_ermessens_va() {
        assert!(validate_nebenbestimmung(true, Nebenbestimmung::Auflage).is_ok());
        assert!(validate_nebenbestimmung(true, Nebenbestimmung::Befristung).is_ok());
        assert_eq!(
            Nebenbestimmung::Widerrufsvorbehalt.paragraph(),
            "§ 36 Abs. 2 Nr. 3 VwVfG"
        );
    }

    #[test]
    fn nebenbestimmung_unzulaessig_for_gebundener_va() {
        assert!(matches!(
            validate_nebenbestimmung(false, Nebenbestimmung::Auflage),
            Err(VwVfGError::UnzulaessigeNebenbestimmung)
        ));
    }

    #[test]
    fn wird_wirksam_requires_bekanntgabe() {
        let va = base_va();
        let datum = NaiveDate::from_ymd_opt(2026, 1, 15).expect("valid date");
        let bg = Bekanntgabe::erfolgt_am(datum);
        assert!(wird_wirksam(&va, &bg).is_ok());

        let bg_none = Bekanntgabe::nicht_erfolgt();
        assert!(matches!(
            wird_wirksam(&va, &bg_none),
            Err(VwVfGError::NotBekanntgegeben)
        ));
    }

    #[test]
    fn wird_wirksam_requires_verwaltungsakt() {
        let mut va = base_va();
        va.regelung = false;
        let bg = Bekanntgabe::erfolgt_am(NaiveDate::from_ymd_opt(2026, 1, 15).expect("valid date"));
        assert!(matches!(
            wird_wirksam(&va, &bg),
            Err(VwVfGError::NoRegelung)
        ));
    }

    #[test]
    fn nichtigkeit_evidenztheorie() {
        let c = NichtigkeitsCheck {
            schwerwiegender_fehler: true,
            offensichtlich: true,
            abs2_katalogfall: false,
        };
        assert!(matches!(
            pruefe_nichtigkeit(&c),
            Err(VwVfGError::Nichtig { .. })
        ));

        // Serious but not obvious -> not void (merely voidable).
        let c2 = NichtigkeitsCheck {
            schwerwiegender_fehler: true,
            offensichtlich: false,
            abs2_katalogfall: false,
        };
        assert!(pruefe_nichtigkeit(&c2).is_ok());
    }

    #[test]
    fn nichtigkeit_katalogfall_abs2() {
        let c = NichtigkeitsCheck {
            schwerwiegender_fehler: false,
            offensichtlich: false,
            abs2_katalogfall: true,
        };
        assert!(matches!(
            pruefe_nichtigkeit(&c),
            Err(VwVfGError::Nichtig { .. })
        ));
    }

    #[test]
    fn ruecknahme_unlawful_burdening_ok() {
        // § 48: unlawful burdening act may be retracted freely.
        assert!(pruefe_ruecknahme(true, false, false).is_ok());
    }

    #[test]
    fn ruecknahme_favourable_vertrauensschutz_restricted() {
        // § 48 Abs. 2/3: favourable act + worthy reliance -> restricted.
        assert!(matches!(
            pruefe_ruecknahme(true, true, true),
            Err(VwVfGError::RuecknahmeUnzulaessig)
        ));
        // Favourable act but reliance not worthy of protection -> permissible.
        assert!(pruefe_ruecknahme(true, true, false).is_ok());
    }

    #[test]
    fn ruecknahme_of_lawful_act_is_widerruf_territory() {
        // A lawful act is not subject to § 48 retraction.
        assert!(matches!(
            pruefe_ruecknahme(false, false, false),
            Err(VwVfGError::WiderrufUnzulaessig)
        ));
    }

    #[test]
    fn widerruf_lawful_burdening_ok() {
        // § 49 Abs. 1: lawful burdening act may generally be revoked.
        assert!(pruefe_widerruf(true, false, false).is_ok());
    }

    #[test]
    fn widerruf_favourable_requires_ground() {
        // § 49 Abs. 2/3: favourable lawful act only on a statutory ground.
        assert!(matches!(
            pruefe_widerruf(true, true, false),
            Err(VwVfGError::WiderrufUnzulaessig)
        ));
        assert!(pruefe_widerruf(true, true, true).is_ok());
    }

    #[test]
    fn widerruf_of_unlawful_act_is_ruecknahme_territory() {
        assert!(matches!(
            pruefe_widerruf(false, false, false),
            Err(VwVfGError::RuecknahmeUnzulaessig)
        ));
    }

    #[test]
    fn va_helpers() {
        assert!(VAWirkung::Beguenstigend.ist_beguenstigend());
        assert!(VAWirkung::MitDrittwirkung.ist_beguenstigend());
        assert!(!VAWirkung::Belastend.ist_beguenstigend());
        assert_eq!(
            VAArt::Feststellung.label(),
            "feststellender VA / declaratory act"
        );
    }
}
