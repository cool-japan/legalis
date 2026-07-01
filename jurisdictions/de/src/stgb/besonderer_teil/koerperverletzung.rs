//! StGB §§ 223-231 - Bodily Harm Offences (Körperverletzungsdelikte)
//!
//! ## § 223 StGB - Körperverletzung (basic bodily harm)
//!
//! > (1) Wer eine andere Person körperlich misshandelt oder an der Gesundheit
//! > schädigt, wird mit Freiheitsstrafe bis zu fünf Jahren oder mit Geldstrafe
//! > bestraft.
//! > (2) Der Versuch ist strafbar.
//!
//! **English**: Whoever physically maltreats or damages the health of another
//! person is punished with imprisonment of **up to five years or a fine**; the
//! **attempt is punishable** (§ 223 Abs. 2 StGB).
//!
//! The objective elements (objektiver Tatbestand) are two **alternative** acts
//! (Tathandlungen); at least one must be fulfilled:
//! - **körperliche Misshandlung** - an ill, improper treatment that more than
//!   insignificantly impairs bodily well-being or bodily integrity (üble,
//!   unangemessene Behandlung, die das körperliche Wohlbefinden mehr als nur
//!   unerheblich beeinträchtigt);
//! - **Gesundheitsschädigung** - causing or worsening a pathological condition
//!   (Hervorrufen oder Steigern eines pathologischen Zustands).
//!
//! The subjective element is **Vorsatz** (intent, § 15 StGB).
//!
//! Per **§ 230 StGB**, §§ 223 and 229 are *Antragsdelikte*: prosecution requires a
//! criminal complaint (Strafantrag) unless the prosecuting authority affirms a
//! **special public interest** in prosecution (besonderes öffentliches Interesse).
//!
//! ## § 224 StGB - Gefährliche Körperverletzung (dangerous bodily harm)
//!
//! A § 223 bodily harm committed by a qualifying means (§ 224 Abs. 1 Nr. 1-5 StGB):
//! poison/noxious substances, a weapon or other dangerous tool, a treacherous
//! ambush, jointly with another participant, or by a treatment endangering life.
//! Range: imprisonment of **six months to ten years**; in a less serious case
//! (minder schwerer Fall) **three months to five years** (§ 224 Abs. 1 Hs. 2).
//! The attempt is punishable.
//!
//! ## § 225 StGB - Misshandlung von Schutzbefohlenen (mistreatment of dependants)
//!
//! Torment or brutal/malicious maltreatment of a person under 18 or otherwise
//! helpless who is entrusted to the offender's care or authority: imprisonment of
//! **six months to ten years** (§ 225 Abs. 1 StGB).
//!
//! ## § 226 StGB - Schwere Körperverletzung (serious bodily harm; erfolgsqualifiziert)
//!
//! A § 223 bodily harm that causes a serious lasting consequence (§ 226 Abs. 1
//! Nr. 1-3 StGB): loss of sight, hearing, the ability to speak or to procreate;
//! loss or permanent uselessness of an important limb; permanent serious
//! disfigurement; falling into infirmity, paralysis, mental illness or disability.
//! Per § 18 StGB the serious consequence requires **at least negligence**. Range:
//! imprisonment of **one to ten years** (§ 226 Abs. 1). Where the offender causes
//! the consequence intentionally or knowingly (absichtlich oder wissentlich),
//! **not less than three years** (§ 226 Abs. 2 StGB).
//!
//! ## § 226a StGB - Verstümmelung weiblicher Genitalien (FGM)
//!
//! Mutilation of the external female genitalia: imprisonment of **not less than one
//! year** (§ 226a Abs. 1 StGB).
//!
//! ## § 227 StGB - Körperverletzung mit Todesfolge (bodily harm causing death)
//!
//! Where a § 223 bodily harm causes the victim's death (the death resulting from
//! the bodily harm, with at least negligence as to the death per § 18 StGB):
//! imprisonment of **not less than three years**; in a less serious case **one to
//! ten years** (§ 227 Abs. 2 StGB).
//!
//! ## § 228 StGB - Einwilligung (consent)
//!
//! > Wer eine Körperverletzung mit Einwilligung der verletzten Person vornimmt,
//! > handelt nur dann rechtswidrig, wenn die Tat trotz der Einwilligung gegen die
//! > guten Sitten verstößt.
//!
//! **English**: A valid consent (wirksame Einwilligung) of the injured person
//! **justifies** the bodily harm and excludes unlawfulness, **unless** the act,
//! despite the consent, is **contrary to good morals** (sittenwidrig). A
//! sittenwidrige consent does **not** justify.
//!
//! ## § 229 StGB - Fahrlässige Körperverletzung (negligent bodily harm)
//!
//! > Wer durch Fahrlässigkeit die Körperverletzung einer anderen Person verursacht,
//! > wird mit Freiheitsstrafe bis zu drei Jahren oder mit Geldstrafe bestraft.
//!
//! Causing bodily harm by negligence: imprisonment of **up to three years or a
//! fine**. No intent is required. Like § 223 it is an Antragsdelikt (§ 230 StGB).
//!
//! ## § 231 StGB - Beteiligung an einer Schlägerei (participation in a brawl)
//!
//! Participation in a brawl (Schlägerei) or in an attack committed by several
//! persons: imprisonment of **up to three years or a fine** (§ 231 Abs. 1 StGB).
//! Liability requires that the brawl/attack caused a person's **death** or a
//! **serious bodily harm (§ 226 StGB)** - an *objektive Bedingung der Strafbarkeit*
//! (objective condition of criminal liability) that lies outside the Tatbestand.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};
use crate::stgb::strafe::Strafrahmen;

/// A qualifying dangerous means of § 224 Abs. 1 StGB (gefährliche Körperverletzung).
///
/// Each variant turns a § 223 bodily harm into the qualified offence of § 224.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GefaehrlichesMittel224 {
    /// Administering poison or other noxious substances (Beibringung von Gift oder
    /// anderen gesundheitsschädlichen Stoffen, § 224 Abs. 1 Nr. 1 StGB).
    Gift,
    /// By means of a weapon or another dangerous tool (mittels einer Waffe oder
    /// eines anderen gefährlichen Werkzeugs, § 224 Abs. 1 Nr. 2 StGB).
    Waffe,
    /// By means of a treacherous ambush (mittels eines hinterlistigen Überfalls,
    /// § 224 Abs. 1 Nr. 3 StGB).
    HinterlistigerUeberfall,
    /// Jointly with another participant (mit einem anderen Beteiligten
    /// gemeinschaftlich, § 224 Abs. 1 Nr. 4 StGB).
    Gemeinschaftlich,
    /// By means of a treatment endangering life (mittels einer das Leben
    /// gefährdenden Behandlung, § 224 Abs. 1 Nr. 5 StGB).
    LebensgefaehrdendeBehandlung,
}

impl GefaehrlichesMittel224 {
    /// The § citation of the qualifying means within § 224 Abs. 1 StGB.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            GefaehrlichesMittel224::Gift => "§ 224 Abs. 1 Nr. 1 StGB",
            GefaehrlichesMittel224::Waffe => "§ 224 Abs. 1 Nr. 2 StGB",
            GefaehrlichesMittel224::HinterlistigerUeberfall => "§ 224 Abs. 1 Nr. 3 StGB",
            GefaehrlichesMittel224::Gemeinschaftlich => "§ 224 Abs. 1 Nr. 4 StGB",
            GefaehrlichesMittel224::LebensgefaehrdendeBehandlung => "§ 224 Abs. 1 Nr. 5 StGB",
        }
    }
}

/// A serious lasting consequence of § 226 Abs. 1 StGB (schwere Körperverletzung).
///
/// Each variant is a qualifying *Erfolg* whose occurrence elevates a § 223 bodily
/// harm to the erfolgsqualifiziertes Delikt of § 226 StGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchwereFolge226 {
    /// Loss of the ability to see in one or both eyes (Verlust des Sehvermögens,
    /// § 226 Abs. 1 Nr. 1 StGB).
    VerlustSehvermoegen,
    /// Loss of hearing (Verlust des Gehörs, § 226 Abs. 1 Nr. 1 StGB).
    VerlustGehoer,
    /// Loss of the ability to speak (Verlust des Sprechvermögens, § 226 Abs. 1
    /// Nr. 1 StGB).
    VerlustSprechvermoegen,
    /// Loss of the ability to procreate (Verlust der Fortpflanzungsfähigkeit,
    /// § 226 Abs. 1 Nr. 1 StGB).
    VerlustFortpflanzung,
    /// Loss or permanent uselessness of an important limb (Verlust oder dauernde
    /// Unbrauchbarkeit eines wichtigen Körpergliedes, § 226 Abs. 1 Nr. 2 StGB).
    VerlustKoerperglied,
    /// Permanent serious disfigurement (dauernde erhebliche Entstellung, § 226
    /// Abs. 1 Nr. 3 StGB).
    DauerndeEntstellung,
    /// Falling into infirmity, paralysis, mental illness or disability (Siechtum,
    /// Lähmung, geistige Krankheit oder Behinderung, § 226 Abs. 1 Nr. 3 StGB).
    SiechtumLaehmung,
}

impl SchwereFolge226 {
    /// The § citation of the serious consequence within § 226 Abs. 1 StGB.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            SchwereFolge226::VerlustSehvermoegen
            | SchwereFolge226::VerlustGehoer
            | SchwereFolge226::VerlustSprechvermoegen
            | SchwereFolge226::VerlustFortpflanzung => "§ 226 Abs. 1 Nr. 1 StGB",
            SchwereFolge226::VerlustKoerperglied => "§ 226 Abs. 1 Nr. 2 StGB",
            SchwereFolge226::DauerndeEntstellung | SchwereFolge226::SiechtumLaehmung => {
                "§ 226 Abs. 1 Nr. 3 StGB"
            }
        }
    }
}

/// A declaration of consent to bodily harm (Einwilligung), § 228 StGB.
///
/// Consent justifies a bodily harm (it excludes unlawfulness) only where it is
/// **effective** (wirksam) *and* the act is not, despite the consent, **contrary to
/// good morals** (sittenwidrig). A consent that is ineffective or sittenwidrig does
/// not justify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Einwilligung {
    /// Whether the consent is effective (wirksam): given by an authorised person
    /// with insight and free of relevant defects of will (Einwilligungsfähigkeit,
    /// frei von Willensmängeln).
    pub wirksam: bool,
    /// Whether the act, despite the consent, is contrary to good morals
    /// (sittenwidrig, § 228 StGB) - which bars justification.
    pub sittenwidrig: bool,
}

impl Einwilligung {
    /// Whether this consent justifies the bodily harm under § 228 StGB, i.e. it is
    /// effective and the act is not sittenwidrig.
    #[must_use]
    pub fn rechtfertigt(&self) -> bool {
        self.wirksam && !self.sittenwidrig
    }
}

/// The specific bodily harm offence applicable to a case, §§ 223-231 StGB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodilyHarmOffence {
    /// Basic bodily harm (Körperverletzung, § 223 StGB).
    Koerperverletzung,
    /// Dangerous bodily harm (gefährliche Körperverletzung, § 224 StGB).
    GefaehrlicheKoerperverletzung {
        /// The qualifying dangerous means present (at least one required).
        mittel: Vec<GefaehrlichesMittel224>,
        /// Whether a less serious case applies (minder schwerer Fall, § 224 Abs. 1
        /// Hs. 2 StGB → three months to five years).
        minder_schwer: bool,
    },
    /// Mistreatment of a dependant (Misshandlung von Schutzbefohlenen, § 225 StGB).
    MisshandlungSchutzbefohlener,
    /// Serious bodily harm (schwere Körperverletzung, § 226 StGB).
    SchwereKoerperverletzung {
        /// The serious lasting consequences that occurred (at least one required).
        folgen: Vec<SchwereFolge226>,
        /// Whether the offender caused the consequence intentionally or knowingly
        /// (absichtlich oder wissentlich, § 226 Abs. 2 StGB → not less than three
        /// years).
        absichtlich: bool,
    },
    /// Mutilation of female genitalia (Verstümmelung weiblicher Genitalien,
    /// § 226a StGB).
    VerstuemmelungWeiblicherGenitalien,
    /// Bodily harm causing death (Körperverletzung mit Todesfolge, § 227 StGB).
    KoerperverletzungMitTodesfolge {
        /// Whether a less serious case applies (minder schwerer Fall, § 227 Abs. 2
        /// StGB → one to ten years).
        minder_schwer: bool,
    },
    /// Negligent bodily harm (fahrlässige Körperverletzung, § 229 StGB).
    FahrlaessigeKoerperverletzung,
    /// Participation in a brawl (Beteiligung an einer Schlägerei, § 231 StGB).
    BeteiligungSchlaegerei,
}

impl BodilyHarmOffence {
    /// The § citation of the offence.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            BodilyHarmOffence::Koerperverletzung => "§ 223 StGB",
            BodilyHarmOffence::GefaehrlicheKoerperverletzung { .. } => "§ 224 StGB",
            BodilyHarmOffence::MisshandlungSchutzbefohlener => "§ 225 StGB",
            BodilyHarmOffence::SchwereKoerperverletzung { .. } => "§ 226 StGB",
            BodilyHarmOffence::VerstuemmelungWeiblicherGenitalien => "§ 226a StGB",
            BodilyHarmOffence::KoerperverletzungMitTodesfolge { .. } => "§ 227 StGB",
            BodilyHarmOffence::FahrlaessigeKoerperverletzung => "§ 229 StGB",
            BodilyHarmOffence::BeteiligungSchlaegerei => "§ 231 StGB",
        }
    }

    /// Whether the offence builds on a § 223 bodily harm and therefore requires one
    /// of the two § 223 acts (körperliche Misshandlung or Gesundheitsschädigung).
    ///
    /// This is true for §§ 223, 224, 226, 226a, 227 and 229. It is **false** for
    /// § 225 (its own conduct of Quälen/rohes Misshandeln) and § 231 (mere
    /// participation in a brawl, with death/serious harm as an objective condition).
    #[must_use]
    pub fn requires_koerperverletzungserfolg(&self) -> bool {
        matches!(
            self,
            BodilyHarmOffence::Koerperverletzung
                | BodilyHarmOffence::GefaehrlicheKoerperverletzung { .. }
                | BodilyHarmOffence::SchwereKoerperverletzung { .. }
                | BodilyHarmOffence::VerstuemmelungWeiblicherGenitalien
                | BodilyHarmOffence::KoerperverletzungMitTodesfolge { .. }
                | BodilyHarmOffence::FahrlaessigeKoerperverletzung
        )
    }

    /// Whether the offence is intentional (vorsätzlich) and thus requires Vorsatz.
    ///
    /// Only § 229 (fahrlässige Körperverletzung) dispenses with intent.
    #[must_use]
    pub fn requires_vorsatz(&self) -> bool {
        !matches!(self, BodilyHarmOffence::FahrlaessigeKoerperverletzung)
    }

    /// Whether the offence is an *Antragsdelikt* under § 230 StGB (requiring a
    /// criminal complaint unless a special public interest is affirmed).
    ///
    /// This applies to §§ 223 and 229 StGB.
    #[must_use]
    pub fn ist_antragsdelikt(&self) -> bool {
        matches!(
            self,
            BodilyHarmOffence::Koerperverletzung | BodilyHarmOffence::FahrlaessigeKoerperverletzung
        )
    }

    /// The statutory sentencing range (Strafrahmen) of the offence.
    #[must_use]
    pub fn strafrahmen(&self) -> Strafrahmen {
        match self {
            // § 223 StGB - up to five years or a fine.
            BodilyHarmOffence::Koerperverletzung => Strafrahmen::up_to_months_or_fine(60),
            // § 224 StGB - six months to ten years; minder schwerer Fall three
            // months to five years.
            BodilyHarmOffence::GefaehrlicheKoerperverletzung { minder_schwer, .. } => {
                if *minder_schwer {
                    Strafrahmen::imprisonment(3, 60)
                } else {
                    Strafrahmen::imprisonment(6, 120)
                }
            }
            // § 225 StGB - six months to ten years.
            BodilyHarmOffence::MisshandlungSchutzbefohlener => Strafrahmen::imprisonment(6, 120),
            // § 226 StGB - one to ten years; absichtlich/wissentlich not less than
            // three years (§ 226 Abs. 2 StGB).
            BodilyHarmOffence::SchwereKoerperverletzung { absichtlich, .. } => {
                if *absichtlich {
                    Strafrahmen::at_least_months(36)
                } else {
                    Strafrahmen::imprisonment(12, 120)
                }
            }
            // § 226a StGB - not less than one year.
            BodilyHarmOffence::VerstuemmelungWeiblicherGenitalien => {
                Strafrahmen::at_least_months(12)
            }
            // § 227 StGB - not less than three years; minder schwerer Fall one to
            // ten years (§ 227 Abs. 2 StGB).
            BodilyHarmOffence::KoerperverletzungMitTodesfolge { minder_schwer } => {
                if *minder_schwer {
                    Strafrahmen::imprisonment(12, 120)
                } else {
                    Strafrahmen::at_least_months(36)
                }
            }
            // § 229 StGB - up to three years or a fine.
            BodilyHarmOffence::FahrlaessigeKoerperverletzung => {
                Strafrahmen::up_to_months_or_fine(36)
            }
            // § 231 StGB - up to three years or a fine.
            BodilyHarmOffence::BeteiligungSchlaegerei => Strafrahmen::up_to_months_or_fine(36),
        }
    }
}

/// A bodily harm case (Körperverletzungsdelikt), §§ 223-231 StGB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodilyHarmCase {
    /// Description of the injured person (verletzte Person / Tatopfer).
    pub opfer: String,
    /// Whether a *körperliche Misshandlung* occurred (first § 223 alternative).
    pub koerperliche_misshandlung: bool,
    /// Whether a *Gesundheitsschädigung* occurred (second § 223 alternative).
    pub gesundheitsschaedigung: bool,
    /// Whether the offender acted intentionally (Vorsatz, § 15 StGB). Required for
    /// all intentional offences; irrelevant for § 229 (negligence).
    pub vorsatz: bool,
    /// Whether causation is established (Kausalität, conditio sine qua non).
    pub kausalitaet: bool,
    /// An optional declaration of consent (Einwilligung, § 228 StGB).
    pub einwilligung: Option<Einwilligung>,
    /// Whether a serious lasting consequence occurred (relevant for § 226 StGB).
    pub schwere_folge_eingetreten: bool,
    /// Whether the serious consequence was caused at least negligently (§ 18 StGB).
    pub schwere_folge_wenigstens_fahrlaessig: bool,
    /// Whether the victim's death occurred (relevant for §§ 227, 231 StGB).
    pub tod_eingetreten: bool,
    /// Whether a criminal complaint was filed (Strafantrag gestellt, § 230 StGB).
    pub strafantrag_gestellt: bool,
    /// Whether a special public interest in prosecution was affirmed (besonderes
    /// öffentliches Interesse, § 230 Abs. 1 S. 1 Hs. 2 StGB).
    pub oeffentliches_interesse: bool,
    /// The offence the case is charged under.
    pub offence: BodilyHarmOffence,
}

/// Validate a bodily harm case under §§ 223-231 StGB.
///
/// The validation mirrors the doctrinal structure (Tatbestand → Rechtswidrigkeit):
/// it checks the objective and subjective elements of the charged offence, then -
/// for offences building on § 223 - applies the justifying effect of a valid
/// consent (§ 228 StGB), and finally the procedural requirement of a criminal
/// complaint for the Antragsdelikte §§ 223 and 229 (§ 230 StGB).
///
/// A **valid consent** (wirksam && !sittenwidrig) makes the § 223/§ 224 act
/// lawful; this is signalled by returning
/// [`StgbError::TatbestandNotFulfilled`] with the message
/// "§ 228 StGB: Einwilligung schließt Rechtswidrigkeit aus (gerechtfertigt)" -
/// i.e. **no liability**. A *sittenwidrige* consent does not justify.
///
/// # Errors
/// - [`StgbError::InvalidField`] if the victim description is empty, or - for an
///   Antragsdelikt - neither a complaint nor a special public interest is present.
/// - [`StgbError::TatbestandNotFulfilled`] if neither § 223 act is present (for a
///   § 223-based offence), or to signal justification by a valid consent.
/// - [`StgbError::NoKausalitaet`] if causation is not established.
/// - [`StgbError::FahrlaessigkeitNichtStrafbar`] if an intentional offence is
///   charged but intent is missing.
/// - [`StgbError::AbsichtMissing`] if § 224 is charged without any qualifying
///   means.
/// - [`StgbError::NoSchuldform`] if § 226 lacks an occurred serious consequence or
///   the at-least-negligence requirement of § 18 StGB.
/// - [`StgbError::TatbestandNotFulfilled`] if § 227 is charged without the death,
///   or § 231 without the objective condition of death/serious harm.
pub fn validate_bodily_harm(case: &BodilyHarmCase) -> Result<()> {
    if case.opfer.trim().is_empty() {
        return Err(StgbError::InvalidField {
            field: "opfer (verletzte Person, §§ 223 ff. StGB)".to_string(),
        });
    }

    // Objective base act: §§ 223, 224, 226, 226a, 227, 229 build on a § 223 bodily
    // harm and require at least one of the two alternative Tathandlungen.
    if case.offence.requires_koerperverletzungserfolg()
        && !(case.koerperliche_misshandlung || case.gesundheitsschaedigung)
    {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "körperliche Misshandlung oder Gesundheitsschädigung (§ 223 StGB)".to_string(),
        });
    }

    // Causation between conduct and result (conditio sine qua non).
    if !case.kausalitaet {
        return Err(StgbError::NoKausalitaet);
    }

    // Subjective element: intentional offences require Vorsatz (§ 15 StGB); § 229
    // is exempt as negligence is its very Schuldform.
    if case.offence.requires_vorsatz() && !case.vorsatz {
        return Err(StgbError::FahrlaessigkeitNichtStrafbar);
    }

    // Offence-specific objective requirements.
    match &case.offence {
        BodilyHarmOffence::GefaehrlicheKoerperverletzung { mittel, .. }
            // § 224 requires at least one qualifying dangerous means.
            if mittel.is_empty() =>
        {
            return Err(StgbError::AbsichtMissing {
                detail: "§ 224 StGB setzt mindestens ein gefährliches Mittel \
                         (Nr. 1-5) voraus"
                    .to_string(),
            });
        }
        BodilyHarmOffence::SchwereKoerperverletzung { .. }
            // § 226: the serious consequence must have occurred and, per § 18 StGB,
            // be attributable at least by negligence.
            if !case.schwere_folge_eingetreten || !case.schwere_folge_wenigstens_fahrlaessig =>
        {
            return Err(StgbError::NoSchuldform);
        }
        // § 227: the death must have resulted from the bodily harm.
        BodilyHarmOffence::KoerperverletzungMitTodesfolge { .. } if !case.tod_eingetreten => {
            return Err(StgbError::TatbestandNotFulfilled {
                element: "Tod des Opfers als Folge der Körperverletzung (§ 227 StGB)".to_string(),
            });
        }
        // § 231: objektive Bedingung der Strafbarkeit - the brawl/attack must have
        // caused a death or a serious bodily harm (§ 226 StGB).
        BodilyHarmOffence::BeteiligungSchlaegerei
            if !(case.tod_eingetreten || case.schwere_folge_eingetreten) =>
        {
            return Err(StgbError::TatbestandNotFulfilled {
                element: "schwere Folge (Tod oder schwere Körperverletzung) als objektive \
                          Bedingung der Strafbarkeit (§ 231 StGB)"
                    .to_string(),
            });
        }
        _ => {}
    }

    // Rechtswidrigkeit: a valid consent (§ 228 StGB) justifies a § 223-based bodily
    // harm. A sittenwidrige (or ineffective) consent does not justify.
    if case.offence.requires_koerperverletzungserfolg()
        && let Some(einwilligung) = &case.einwilligung
        && einwilligung.rechtfertigt()
    {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "§ 228 StGB: Einwilligung schließt Rechtswidrigkeit aus (gerechtfertigt)"
                .to_string(),
        });
    }

    // Procedural requirement: §§ 223 and 229 are Antragsdelikte (§ 230 StGB).
    if case.offence.ist_antragsdelikt()
        && !(case.strafantrag_gestellt || case.oeffentliches_interesse)
    {
        return Err(StgbError::InvalidField {
            field: "Strafantrag oder besonderes öffentliches Interesse (§ 230 StGB)".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_case(offence: BodilyHarmOffence) -> BodilyHarmCase {
        BodilyHarmCase {
            opfer: "Opfer".to_string(),
            koerperliche_misshandlung: true,
            gesundheitsschaedigung: false,
            vorsatz: true,
            kausalitaet: true,
            einwilligung: None,
            schwere_folge_eingetreten: false,
            schwere_folge_wenigstens_fahrlaessig: false,
            tod_eingetreten: false,
            strafantrag_gestellt: true,
            oeffentliches_interesse: false,
            offence,
        }
    }

    #[test]
    fn koerperverletzung_basic_is_valid_with_strafantrag() {
        let o = BodilyHarmOffence::Koerperverletzung;
        assert_eq!(o.paragraph(), "§ 223 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        assert!(!r.allows_life());
        assert!(validate_bodily_harm(&base_case(o)).is_ok());
    }

    #[test]
    fn koerperverletzung_requires_a_tathandlung() {
        let mut c = base_case(BodilyHarmOffence::Koerperverletzung);
        c.koerperliche_misshandlung = false;
        c.gesundheitsschaedigung = false;
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
        // Either alternative suffices.
        c.gesundheitsschaedigung = true;
        assert!(validate_bodily_harm(&c).is_ok());
    }

    #[test]
    fn koerperverletzung_requires_intent() {
        let mut c = base_case(BodilyHarmOffence::Koerperverletzung);
        c.vorsatz = false;
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
    }

    #[test]
    fn koerperverletzung_requires_strafantrag_or_public_interest() {
        let mut c = base_case(BodilyHarmOffence::Koerperverletzung);
        c.strafantrag_gestellt = false;
        c.oeffentliches_interesse = false;
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::InvalidField { .. })
        ));
        // Special public interest replaces the complaint (§ 230 StGB).
        c.oeffentliches_interesse = true;
        assert!(validate_bodily_harm(&c).is_ok());
    }

    #[test]
    fn wirksame_einwilligung_justifies_223() {
        let mut c = base_case(BodilyHarmOffence::Koerperverletzung);
        c.einwilligung = Some(Einwilligung {
            wirksam: true,
            sittenwidrig: false,
        });
        // Valid consent → justification (signalled as no liability).
        let res = validate_bodily_harm(&c);
        match res {
            Err(StgbError::TatbestandNotFulfilled { element }) => {
                assert!(element.contains("§ 228 StGB"));
                assert!(element.contains("gerechtfertigt"));
            }
            other => panic!("expected justification by consent, got {other:?}"),
        }
    }

    #[test]
    fn sittenwidrige_einwilligung_does_not_justify() {
        let mut c = base_case(BodilyHarmOffence::Koerperverletzung);
        c.einwilligung = Some(Einwilligung {
            wirksam: true,
            sittenwidrig: true,
        });
        // Despite consent, sittenwidrig → no justification, offence stands.
        assert!(validate_bodily_harm(&c).is_ok());

        // An ineffective consent likewise does not justify.
        c.einwilligung = Some(Einwilligung {
            wirksam: false,
            sittenwidrig: false,
        });
        assert!(validate_bodily_harm(&c).is_ok());
    }

    #[test]
    fn gefaehrliche_koerperverletzung_ranges_and_requires_mittel() {
        let o = BodilyHarmOffence::GefaehrlicheKoerperverletzung {
            mittel: vec![GefaehrlichesMittel224::Waffe],
            minder_schwer: false,
        };
        assert_eq!(o.paragraph(), "§ 224 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 6);
        assert_eq!(r.max_months, Some(120));
        assert!(!r.fine_alternative);
        assert!(validate_bodily_harm(&base_case(o)).is_ok());

        // Minder schwerer Fall: three months to five years.
        let o_ms = BodilyHarmOffence::GefaehrlicheKoerperverletzung {
            mittel: vec![GefaehrlichesMittel224::Gemeinschaftlich],
            minder_schwer: true,
        };
        let r_ms = o_ms.strafrahmen();
        assert_eq!(r_ms.effective_min_months(), 3);
        assert_eq!(r_ms.max_months, Some(60));

        // Missing qualifying means → AbsichtMissing.
        let o_empty = BodilyHarmOffence::GefaehrlicheKoerperverletzung {
            mittel: vec![],
            minder_schwer: false,
        };
        assert!(matches!(
            validate_bodily_harm(&base_case(o_empty)),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn schwere_koerperverletzung_ranges_and_requires_folge() {
        let o = BodilyHarmOffence::SchwereKoerperverletzung {
            folgen: vec![SchwereFolge226::VerlustSehvermoegen],
            absichtlich: false,
        };
        assert_eq!(o.paragraph(), "§ 226 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert_eq!(r.max_months, Some(120));

        let mut c = base_case(o);
        // Without an occurred consequence → NoSchuldform.
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::NoSchuldform)
        ));
        c.schwere_folge_eingetreten = true;
        // Still missing the § 18 at-least-negligence link.
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::NoSchuldform)
        ));
        c.schwere_folge_wenigstens_fahrlaessig = true;
        assert!(validate_bodily_harm(&c).is_ok());
    }

    #[test]
    fn schwere_koerperverletzung_absichtlich_range() {
        // § 226 Abs. 2 StGB: not less than three years.
        let o = BodilyHarmOffence::SchwereKoerperverletzung {
            folgen: vec![SchwereFolge226::VerlustKoerperglied],
            absichtlich: true,
        };
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 36);
        assert!(!r.allows_life());
    }

    #[test]
    fn koerperverletzung_mit_todesfolge_range_and_needs_death() {
        let o = BodilyHarmOffence::KoerperverletzungMitTodesfolge {
            minder_schwer: false,
        };
        assert_eq!(o.paragraph(), "§ 227 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 36);

        let mut c = base_case(o);
        // No death → Tatbestand not fulfilled.
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
        c.tod_eingetreten = true;
        assert!(validate_bodily_harm(&c).is_ok());

        // Minder schwerer Fall: one to ten years.
        let r_ms = BodilyHarmOffence::KoerperverletzungMitTodesfolge {
            minder_schwer: true,
        }
        .strafrahmen();
        assert_eq!(r_ms.effective_min_months(), 12);
        assert_eq!(r_ms.max_months, Some(120));
    }

    #[test]
    fn fahrlaessige_koerperverletzung_needs_no_intent_and_has_fine() {
        let o = BodilyHarmOffence::FahrlaessigeKoerperverletzung;
        assert_eq!(o.paragraph(), "§ 229 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(36));
        assert!(r.fine_alternative);

        let mut c = base_case(o);
        c.vorsatz = false;
        // Negligence suffices; § 229 is also an Antragsdelikt (complaint present).
        assert!(validate_bodily_harm(&c).is_ok());

        // Still an Antragsdelikt: without complaint and public interest it fails.
        c.strafantrag_gestellt = false;
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::InvalidField { .. })
        ));
    }

    #[test]
    fn misshandlung_schutzbefohlener_range() {
        let o = BodilyHarmOffence::MisshandlungSchutzbefohlener;
        assert_eq!(o.paragraph(), "§ 225 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 6);
        assert_eq!(r.max_months, Some(120));
        assert!(!r.fine_alternative);
        // Not a § 223-based offence and not an Antragsdelikt; validates directly.
        assert!(validate_bodily_harm(&base_case(o)).is_ok());
    }

    #[test]
    fn verstuemmelung_weiblicher_genitalien_range() {
        let o = BodilyHarmOffence::VerstuemmelungWeiblicherGenitalien;
        assert_eq!(o.paragraph(), "§ 226a StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert_eq!(r.max_months, Some(180));
        assert!(!r.allows_life());
        assert!(validate_bodily_harm(&base_case(o)).is_ok());
    }

    #[test]
    fn beteiligung_schlaegerei_range_and_objektive_bedingung() {
        let o = BodilyHarmOffence::BeteiligungSchlaegerei;
        assert_eq!(o.paragraph(), "§ 231 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(36));
        assert!(r.fine_alternative);

        // Not a § 223-based offence: needs no Tathandlung, but needs the objective
        // condition of death or serious harm.
        let mut c = base_case(o);
        c.koerperliche_misshandlung = false;
        c.gesundheitsschaedigung = false;
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
        c.schwere_folge_eingetreten = true;
        assert!(validate_bodily_harm(&c).is_ok());
    }

    #[test]
    fn causation_required_for_all() {
        let mut c = base_case(BodilyHarmOffence::Koerperverletzung);
        c.kausalitaet = false;
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::NoKausalitaet)
        ));
    }

    #[test]
    fn empty_opfer_is_invalid() {
        let mut c = base_case(BodilyHarmOffence::Koerperverletzung);
        c.opfer = "   ".to_string();
        assert!(matches!(
            validate_bodily_harm(&c),
            Err(StgbError::InvalidField { .. })
        ));
    }

    #[test]
    fn mittel_and_folge_paragraph_citations() {
        assert_eq!(
            GefaehrlichesMittel224::Gift.paragraph(),
            "§ 224 Abs. 1 Nr. 1 StGB"
        );
        assert_eq!(
            GefaehrlichesMittel224::LebensgefaehrdendeBehandlung.paragraph(),
            "§ 224 Abs. 1 Nr. 5 StGB"
        );
        assert_eq!(
            SchwereFolge226::VerlustGehoer.paragraph(),
            "§ 226 Abs. 1 Nr. 1 StGB"
        );
        assert_eq!(
            SchwereFolge226::VerlustKoerperglied.paragraph(),
            "§ 226 Abs. 1 Nr. 2 StGB"
        );
        assert_eq!(
            SchwereFolge226::DauerndeEntstellung.paragraph(),
            "§ 226 Abs. 1 Nr. 3 StGB"
        );
    }
}
