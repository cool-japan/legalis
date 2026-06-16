//! StGB Error Types (Strafgesetzbuch)
//!
//! Bilingual error types (German primary, English secondary) for the German
//! Criminal Code (StGB), with specific § citations. Used across the General Part
//! (Allgemeiner Teil) and the Special Part (Besonderer Teil).

use thiserror::Error;

/// Result type for StGB operations.
pub type Result<T> = std::result::Result<T, StgbError>;

/// Validation and computation errors for the German Criminal Code (StGB).
#[derive(Error, Debug, Clone, PartialEq)]
pub enum StgbError {
    // === Penalties / Sentencing (Strafen, §§ 38-43 StGB) ===
    #[error(
        "Unzulässige Freiheitsstrafe: {months} Monate (§ 38 Abs. 2 StGB: 1 Monat bis 15 Jahre)\n\
         Inadmissible custodial sentence: {months} months (§ 38 para. 2 StGB: 1 month to 15 years)"
    )]
    InvalidFreiheitsstrafe { months: u32 },

    #[error(
        "Unzulässige Anzahl von Tagessätzen: {count} (§ 40 Abs. 1 StGB: 5 bis 360)\n\
         Inadmissible number of daily units: {count} (§ 40 para. 1 StGB: 5 to 360)"
    )]
    InvalidTagessaetze { count: u32 },

    #[error(
        "Unzulässige Tagessatzhöhe: {cents} Cent (§ 40 Abs. 2 S. 3 StGB: 1 bis 30.000 EUR)\n\
         Inadmissible daily-unit amount: {cents} cents (§ 40 para. 2 sent. 3 StGB: 1 to 30,000 EUR)"
    )]
    InvalidTagessatzHoehe { cents: u64 },

    #[error(
        "Strafe außerhalb des Strafrahmens: {months} Monate (zulässig {min}-{max} Monate)\n\
         Sentence outside statutory range: {months} months (permitted {min}-{max} months)"
    )]
    SentenceOutsideRange { months: u32, min: u32, max: u32 },

    #[error(
        "Geldstrafe für diesen Tatbestand nicht vorgesehen\n\
         Fine not available for this offence"
    )]
    FineNotAvailable,

    #[error(
        "Lebenslange Freiheitsstrafe für diesen Tatbestand nicht vorgesehen\n\
         Life imprisonment not available for this offence"
    )]
    LifeNotAvailable,

    // === General Part: Objective/Subjective offence elements (Tatbestand) ===
    #[error(
        "Tatbestand nicht erfüllt: {element} fehlt\n\
         Offence element not fulfilled: {element} missing"
    )]
    TatbestandNotFulfilled { element: String },

    #[error(
        "Kein tatbestandsmäßiges Handeln (Tathandlung fehlt)\n\
         No conduct fulfilling the offence definition (Tathandlung missing)"
    )]
    NoTathandlung,

    #[error(
        "Kausalität zwischen Handlung und Erfolg nicht gegeben (Conditio sine qua non)\n\
         Causation between conduct and result not established (but-for test)"
    )]
    NoKausalitaet,

    #[error(
        "Objektive Zurechnung des Erfolgs zu verneinen\n\
         Objective attribution of the result to be denied"
    )]
    NoObjektiveZurechnung,

    // === § 13 StGB - Unterlassen / Garantenstellung ===
    #[error(
        "Strafbarkeit durch Unterlassen scheitert: keine Garantenstellung (§ 13 StGB)\n\
         Liability by omission fails: no guarantor position (§ 13 StGB)"
    )]
    NoGarantenstellung,

    #[error(
        "Strafbarkeit durch Unterlassen scheitert: Erfolgsabwendung nicht möglich/zumutbar (§ 13 StGB)\n\
         Liability by omission fails: averting the result not possible/reasonable (§ 13 StGB)"
    )]
    ErfolgsabwendungUnmoeglich,

    // === § 15 StGB - Vorsatz / Fahrlässigkeit ===
    #[error(
        "Nur vorsätzliches Handeln ist strafbar; Fahrlässigkeit nicht mit Strafe bedroht (§ 15 StGB)\n\
         Only intentional conduct is punishable; negligence is not penalised (§ 15 StGB)"
    )]
    FahrlaessigkeitNichtStrafbar,

    #[error(
        "Subjektiver Tatbestand nicht erfüllt: weder Vorsatz noch (strafbare) Fahrlässigkeit\n\
         Subjective offence element not fulfilled: neither intent nor (punishable) negligence"
    )]
    NoSchuldform,

    // === §§ 16-17 StGB - Irrtum (Mistake) ===
    #[error(
        "Tatbestandsirrtum schließt den Vorsatz aus (§ 16 Abs. 1 StGB)\n\
         Mistake of fact excludes intent (§ 16 para. 1 StGB)"
    )]
    Tatbestandsirrtum,

    #[error(
        "Unvermeidbarer Verbotsirrtum: Schuld entfällt (§ 17 S. 1 StGB)\n\
         Unavoidable mistake of law: culpability is excluded (§ 17 sent. 1 StGB)"
    )]
    UnvermeidbarerVerbotsirrtum,

    // === §§ 19-21 StGB - Schuldfähigkeit (Capacity) ===
    #[error(
        "Schuldunfähigkeit des Kindes: Täter unter 14 Jahre (§ 19 StGB)\n\
         Incapacity of a child: offender under 14 years (§ 19 StGB)"
    )]
    Schuldunfaehig19Kind,

    #[error(
        "Schuldunfähigkeit wegen seelischer Störung (§ 20 StGB): Schuld entfällt\n\
         Lack of culpability due to a mental disorder (§ 20 StGB): culpability is excluded"
    )]
    Schuldunfaehig20,

    // === §§ 22-24 StGB - Versuch (Attempt) ===
    #[error(
        "Kein unmittelbares Ansetzen zur Tatbestandsverwirklichung (§ 22 StGB)\n\
         No immediate commencement of the offence (§ 22 StGB)"
    )]
    NoUnmittelbaresAnsetzen,

    #[error(
        "Versuch nicht strafbar: weder Verbrechen noch ausdrückliche Anordnung (§ 23 Abs. 1 StGB)\n\
         Attempt not punishable: neither a felony nor expressly ordered (§ 23 para. 1 StGB)"
    )]
    VersuchNichtStrafbar,

    #[error(
        "Strafbefreiender Rücktritt vom Versuch (§ 24 StGB): Straflosigkeit\n\
         Withdrawal from the attempt exempting from punishment (§ 24 StGB): impunity"
    )]
    StrafbefreienderRuecktritt,

    // === §§ 25-30 StGB - Täterschaft und Teilnahme (Perpetration & participation) ===
    #[error(
        "Keine Haupttat: Teilnahme setzt vorsätzliche rechtswidrige Haupttat voraus (§§ 26, 27 StGB)\n\
         No principal offence: participation requires an intentional unlawful principal act (§§ 26, 27 StGB)"
    )]
    NoHaupttat,

    #[error(
        "Anstiftung scheitert: kein Bestimmen zur Tat (§ 26 StGB)\n\
         Incitement fails: no determining of the principal to the act (§ 26 StGB)"
    )]
    NoBestimmen,

    #[error(
        "Beihilfe scheitert: kein Hilfeleisten zur Haupttat (§ 27 StGB)\n\
         Aiding fails: no assistance rendered to the principal act (§ 27 StGB)"
    )]
    NoHilfeleisten,

    // === §§ 32-35 StGB - Rechtfertigung / Entschuldigung ===
    #[error(
        "Tat durch Notwehr gerechtfertigt (§ 32 StGB): keine Rechtswidrigkeit\n\
         Act justified by self-defence (§ 32 StGB): no unlawfulness"
    )]
    GerechtfertigtNotwehr,

    #[error(
        "Notwehr scheitert: kein gegenwärtiger rechtswidriger Angriff (§ 32 Abs. 2 StGB)\n\
         Self-defence fails: no present unlawful attack (§ 32 para. 2 StGB)"
    )]
    NotwehrlageFehlt,

    #[error(
        "Notwehr scheitert: Verteidigung nicht erforderlich (§ 32 Abs. 2 StGB)\n\
         Self-defence fails: defence not necessary (§ 32 para. 2 StGB)"
    )]
    VerteidigungNichtErforderlich,

    #[error(
        "Tat durch rechtfertigenden Notstand gerechtfertigt (§ 34 StGB)\n\
         Act justified by necessity (§ 34 StGB)"
    )]
    GerechtfertigtNotstand,

    #[error(
        "Rechtfertigender Notstand scheitert: geschütztes Interesse überwiegt nicht wesentlich (§ 34 StGB)\n\
         Justifying necessity fails: protected interest does not substantially outweigh (§ 34 StGB)"
    )]
    InteresseUeberwiegtNicht,

    #[error(
        "Tat durch entschuldigenden Notstand entschuldigt (§ 35 StGB): Schuld entfällt\n\
         Act excused by excusing necessity (§ 35 StGB): culpability is excluded"
    )]
    EntschuldigtNotstand,

    // === Special Part / general structural errors ===
    #[error(
        "Tatobjekt fehlt oder ungeeignet: {detail}\n\
         Object of the offence missing or unsuitable: {detail}"
    )]
    InvalidTatobjekt { detail: String },

    #[error(
        "Erforderliche Absicht fehlt: {detail}\n\
         Required intent (Absicht) missing: {detail}"
    )]
    AbsichtMissing { detail: String },

    #[error(
        "Leerer oder ungültiger Wert: {field}\n\
         Empty or invalid value: {field}"
    )]
    InvalidField { field: String },

    #[error(
        "Ungültiger Geldbetrag: {detail}\n\
         Invalid monetary amount: {detail}"
    )]
    InvalidAmount { detail: String },
}
