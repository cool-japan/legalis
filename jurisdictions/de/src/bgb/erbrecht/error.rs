//! Error types for German Succession Law validation
//!
//! Provides bilingual (German/English) error messages with BGB article references.

use thiserror::Error;

/// Result type for succession law operations
pub type Result<T> = std::result::Result<T, SuccessionLawError>;

/// Errors that can occur when validating succession law structures
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SuccessionLawError {
    // Will Formalities Errors (§§2064-2086, §§2231-2247 BGB)
    #[error(
        "Eigenhändiges Testament nicht handgeschrieben (§2247 Abs. 1 BGB)\n\
         Holographic will not handwritten (§2247 para. 1 BGB)"
    )]
    WillNotHandwritten,

    #[error(
        "Testament fehlt Unterschrift (§2247 Abs. 1 BGB)\n\
         Will missing signature (§2247 para. 1 BGB)"
    )]
    WillMissingSignature,

    #[error(
        "Testament widerrufen: Widerruf am {revoked_date}\n\
         Will revoked: Revoked on {revoked_date}"
    )]
    WillRevoked { revoked_date: String },

    #[error(
        "Testierfähigkeit fehlt: Person unter 16 Jahren (§2229 Abs. 1 BGB)\n\
         Testamentary capacity lacking: Person under 16 years (§2229 para. 1 BGB)"
    )]
    NoTestamentaryCapacity,

    #[error(
        "Gemeinschaftliches Testament: Besondere Formalien für 16-17 Jährige (§2229 Abs. 2 BGB)\n\
         Joint will: Special formalities for 16-17 year olds (§2229 para. 2 BGB)"
    )]
    LimitedTestamentaryCapacity,

    #[error(
        "Öffentliches Testament fehlt Beurkundung (§2232 BGB)\n\
         Public will missing notarization (§2232 BGB)"
    )]
    PublicWillNotNotarized,

    // Testamentary Succession Errors
    #[error(
        "Begünstigter fehlt: Mindestens ein Erbe erforderlich\n\
         Beneficiary missing: At least one heir required"
    )]
    NoBeneficiaries,

    #[error(
        "Erbteile ungültig: Summe beträgt {total:.2}, muss 1.0 sein\n\
         Invalid shares: Sum is {total:.2}, must be 1.0"
    )]
    InvalidSharesSum { total: f64 },

    #[error(
        "Erbteil ungültig: Nenner ist null\n\
         Invalid share: Denominator is zero"
    )]
    InvalidShareDenominatorZero,

    // Legal Succession Errors (§§1924-1936 BGB)
    #[error(
        "Keine gesetzlichen Erben: Keine Verwandten gefunden\n\
         No statutory heirs: No relatives found"
    )]
    NoStatutoryHeirs,

    #[error(
        "Ehegattenerbrecht ungültig: Kein Ehegatte vorhanden (§1931 BGB)\n\
         Spouse inheritance invalid: No spouse present (§1931 BGB)"
    )]
    NoSpouse,

    #[error(
        "Erbfolgeordnung fehlt: Ordnung muss angegeben werden (§§1924-1929 BGB)\n\
         Succession order missing: Order must be specified (§§1924-1929 BGB)"
    )]
    NoSuccessionOrder,

    // Compulsory Portion Errors (§§2303-2338 BGB)
    #[error(
        "Pflichtteilsberechtigung fehlt: {relationship} nicht pflichtteilsberechtigt (§2303 BGB)\n\
         No compulsory portion entitlement: {relationship} not entitled (§2303 BGB)"
    )]
    NotEntitledToCompulsoryPortion { relationship: String },

    #[error(
        "Nachlasswert fehlt oder ist null: Pflichtteil kann nicht berechnet werden\n\
         Estate value missing or zero: Compulsory portion cannot be calculated"
    )]
    NoEstateValue,

    #[error(
        "Pflichtteil ungültig: Betrag €{amount:.2} überschreitet Nachlasswert €{estate_value:.2}\n\
         Compulsory portion invalid: Amount €{amount:.2} exceeds estate value €{estate_value:.2}"
    )]
    CompulsoryPortionExceedsEstate { amount: f64, estate_value: f64 },

    // Inheritance Contract Errors (§§2274-2302 BGB)
    #[error(
        "Erbvertrag fehlt Beurkundung: Notarielle Beurkundung erforderlich (§2276 BGB)\n\
         Inheritance contract missing notarization: Notarization required (§2276 BGB)"
    )]
    InheritanceContractNotNotarized,

    #[error(
        "Erbvertrag widerrufen\n\
         Inheritance contract revoked"
    )]
    InheritanceContractRevoked,

    // Estate Errors
    #[error(
        "Nachlass ungültig: Verbindlichkeiten €{liabilities:.2} übersteigen Vermögen €{assets:.2}\n\
         Estate invalid: Liabilities €{liabilities:.2} exceed assets €{assets:.2}"
    )]
    InsolvantEstate { assets: f64, liabilities: f64 },

    #[error(
        "Nachlass leer: Keine Vermögenswerte vorhanden\n\
         Estate empty: No assets present"
    )]
    EmptyEstate,

    // Acceptance/Renunciation Errors (§§1942-2063 BGB)
    #[error(
        "Ausschlagungsfrist abgelaufen: Frist war {deadline}, Entscheidung am {decision_date} (§1944 BGB)\n\
         Renunciation deadline expired: Deadline was {deadline}, decision on {decision_date} (§1944 BGB)"
    )]
    RenunciationDeadlineExpired {
        deadline: String,
        decision_date: String,
    },

    #[error(
        "Erbschaftsannahme/-ausschlagung fehlt: Entscheidung erforderlich\n\
         Inheritance acceptance/renunciation missing: Decision required"
    )]
    NoInheritanceDecision,

    // General Validation Errors
    #[error(
        "Verstorbener fehlt: Erblasser nicht angegeben\n\
         Deceased missing: Testator not specified"
    )]
    NoDeceased,

    #[error(
        "Todesdatum ungültig: Todesdatum {death_date} liegt vor Geburtsdatum {birth_date}\n\
         Invalid death date: Death date {death_date} is before birth date {birth_date}"
    )]
    DeathBeforeBirth {
        death_date: String,
        birth_date: String,
    },

    #[error(
        "Todesdatum in der Zukunft: {death_date}\n\
         Death date in future: {death_date}"
    )]
    FutureDeathDate { death_date: String },

    #[error(
        "Erbe fehlt: Keine Erben angegeben\n\
         Heir missing: No heirs specified"
    )]
    NoHeirs,

    #[error(
        "Name fehlt: Name ist leer\n\
         Name missing: Name is empty"
    )]
    EmptyName,

    #[error(
        "Ungültiges Datum: {date_type}\n\
         Invalid date: {date_type}"
    )]
    InvalidDate { date_type: String },

    #[error(
        "Testament vor Geburt erstellt: Testament {will_date}, Geburt {birth_date}\n\
         Will created before birth: Will {will_date}, birth {birth_date}"
    )]
    WillBeforeBirth {
        will_date: String,
        birth_date: String,
    },

    #[error(
        "Testament nach Tod erstellt: Testament {will_date}, Tod {death_date}\n\
         Will created after death: Will {will_date}, death {death_date}"
    )]
    WillAfterDeath {
        will_date: String,
        death_date: String,
    },
}

impl SuccessionLawError {
    /// Get the BGB article reference for this error
    pub fn article_reference(&self) -> &'static str {
        match self {
            Self::WillNotHandwritten => "§2247 Abs. 1 BGB",
            Self::WillMissingSignature => "§2247 Abs. 1 BGB",
            Self::WillRevoked { .. } => "§§2253-2258 BGB",
            Self::NoTestamentaryCapacity => "§2229 Abs. 1 BGB",
            Self::LimitedTestamentaryCapacity => "§2229 Abs. 2 BGB",
            Self::PublicWillNotNotarized => "§2232 BGB",
            Self::NoBeneficiaries => "§2064 BGB",
            Self::InvalidSharesSum { .. } => "BGB",
            Self::InvalidShareDenominatorZero => "BGB",
            Self::NoStatutoryHeirs => "§§1924-1936 BGB",
            Self::NoSpouse => "§1931 BGB",
            Self::NoSuccessionOrder => "§§1924-1929 BGB",
            Self::NotEntitledToCompulsoryPortion { .. } => "§2303 BGB",
            Self::NoEstateValue => "§§2303-2338 BGB",
            Self::CompulsoryPortionExceedsEstate { .. } => "§2303 BGB",
            Self::InheritanceContractNotNotarized => "§2276 BGB",
            Self::InheritanceContractRevoked => "§§2290-2301 BGB",
            Self::InsolvantEstate { .. } => "§§1958-1966 BGB",
            Self::EmptyEstate => "BGB",
            Self::RenunciationDeadlineExpired { .. } => "§1944 BGB",
            Self::NoInheritanceDecision => "§§1942-1953 BGB",
            Self::NoDeceased => "BGB",
            Self::DeathBeforeBirth { .. } => "BGB",
            Self::FutureDeathDate { .. } => "BGB",
            Self::NoHeirs => "BGB",
            Self::EmptyName => "BGB",
            Self::InvalidDate { .. } => "BGB",
            Self::WillBeforeBirth { .. } => "§2229 BGB",
            Self::WillAfterDeath { .. } => "BGB",
        }
    }

    /// Check if this is a will formalities error
    pub fn is_will_formalities_error(&self) -> bool {
        matches!(
            self,
            Self::WillNotHandwritten
                | Self::WillMissingSignature
                | Self::WillRevoked { .. }
                | Self::NoTestamentaryCapacity
                | Self::LimitedTestamentaryCapacity
                | Self::PublicWillNotNotarized
                | Self::WillBeforeBirth { .. }
                | Self::WillAfterDeath { .. }
        )
    }

    /// Check if this is a legal succession error
    pub fn is_legal_succession_error(&self) -> bool {
        matches!(
            self,
            Self::NoStatutoryHeirs | Self::NoSpouse | Self::NoSuccessionOrder
        )
    }

    /// Check if this is a compulsory portion error
    pub fn is_compulsory_portion_error(&self) -> bool {
        matches!(
            self,
            Self::NotEntitledToCompulsoryPortion { .. }
                | Self::NoEstateValue
                | Self::CompulsoryPortionExceedsEstate { .. }
        )
    }

    /// Check if this is an inheritance contract error
    pub fn is_inheritance_contract_error(&self) -> bool {
        matches!(
            self,
            Self::InheritanceContractNotNotarized | Self::InheritanceContractRevoked
        )
    }

    /// Check if this is an estate error
    pub fn is_estate_error(&self) -> bool {
        matches!(self, Self::InsolvantEstate { .. } | Self::EmptyEstate)
    }
}
