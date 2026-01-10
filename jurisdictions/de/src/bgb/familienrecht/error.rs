//! Error types for German Family Law validation
//!
//! Provides bilingual (German/English) error messages with BGB article references.

use thiserror::Error;

/// Result type for family law operations
pub type Result<T> = std::result::Result<T, FamilyLawError>;

/// Errors that can occur when validating family law structures
#[derive(Error, Debug, Clone, PartialEq)]
pub enum FamilyLawError {
    // Marriage Formation Errors (§§1303-1311 BGB)
    #[error(
        "Mindestalter nicht erreicht: Person ist {actual_age} Jahre alt, Mindestalter ist 18 Jahre (§1303 BGB)\n\
         Minimum age not met: Person is {actual_age} years old, minimum age is 18 years (§1303 BGB)"
    )]
    BelowMarriageAge { actual_age: u32 },

    #[error(
        "Bestehende Ehe: Eine Partei ist bereits verheiratet (§1306 BGB)\n\
         Existing marriage: One party is already married (§1306 BGB)"
    )]
    ExistingMarriage,

    #[error(
        "Verwandtschaft: Parteien sind in gerader Linie verwandt oder Geschwister (§1307 BGB)\n\
         Consanguinity: Parties are lineally related or siblings (§1307 BGB)"
    )]
    Consanguinity,

    #[error(
        "Fehlende Geschäftsfähigkeit: Eine Partei ist geschäftsunfähig (§1304 BGB)\n\
         Lack of capacity: One party lacks legal capacity (§1304 BGB)"
    )]
    LackOfCapacity,

    #[error(
        "Ehe ungültig: Eheverbote bestehen (§§1306-1311 BGB)\n\
         Marriage invalid: Marriage impediments exist (§§1306-1311 BGB)"
    )]
    MarriageImpedimentsExist,

    #[error(
        "Standesamt fehlt: Kein Standesamt angegeben\n\
         Registrar office missing: No registrar office specified"
    )]
    NoRegistrarOffice,

    // Matrimonial Property Agreement Errors (§§1408-1410 BGB)
    #[error(
        "Fehlende Beurkundung: Ehevertrag muss notariell beurkundet sein (§1410 BGB)\n\
         Missing notarization: Matrimonial property agreement must be notarized (§1410 BGB)"
    )]
    AgreementNotNotarized,

    #[error(
        "Ungültige Ehegatten: Ehevertrag muss zwischen Ehegatten geschlossen werden\n\
         Invalid spouses: Agreement must be between spouses"
    )]
    AgreementNotBetweenSpouses,

    // Divorce Errors (§§1564-1587 BGB)
    #[error(
        "Trennungszeit zu kurz: {actual_months} Monate Trennung, erforderlich: {required_months} Monate (§1566 BGB)\n\
         Separation period too short: {actual_months} months separation, required: {required_months} months (§1566 BGB)"
    )]
    InsufficientSeparationPeriod {
        actual_months: u32,
        required_months: u32,
    },

    #[error(
        "Ehe nicht gescheitert: Ehescheitern nicht nachgewiesen (§1565 BGB)\n\
         Marriage not broken down: Marriage breakdown not proven (§1565 BGB)"
    )]
    MarriageNotBrokenDown,

    #[error(
        "Fehlende Zustimmung: Scheidung ohne Zustimmung erfordert 3 Jahre Trennung (§1566 Abs. 2 BGB)\n\
         Missing consent: Divorce without consent requires 3 years separation (§1566 para. 2 BGB)"
    )]
    NoConsentAndInsufficientSeparation,

    #[error(
        "Ungültige Ehe: Nur gültige Ehen können geschieden werden\n\
         Invalid marriage: Only valid marriages can be divorced"
    )]
    CannotDivorceInvalidMarriage,

    // Maintenance Errors (§§1569-1586 BGB, §§1601-1615 BGB)
    #[error(
        "Unterhaltsbetrag fehlt oder ist null\n\
         Maintenance amount missing or zero"
    )]
    NoMaintenanceAmount,

    #[error(
        "Unterhaltsgrund fehlt: Kein gesetzlicher Unterhaltsgrund angegeben (§§1570-1576 BGB)\n\
         Maintenance ground missing: No statutory maintenance ground specified (§§1570-1576 BGB)"
    )]
    NoMaintenanceGround,

    #[error(
        "Unterhaltspflichtiger fehlt: Keine Person als Unterhaltspflichtiger angegeben\n\
         Obligor missing: No person specified as maintenance obligor"
    )]
    NoObligor,

    #[error(
        "Unterhaltsberechtigter fehlt: Keine Person als Unterhaltsberechtigter angegeben\n\
         Beneficiary missing: No person specified as maintenance beneficiary"
    )]
    NoBeneficiary,

    #[error(
        "Selbstunterhalt: Unterhaltspflichtiger und Berechtigter können nicht identisch sein\n\
         Self-maintenance: Obligor and beneficiary cannot be the same person"
    )]
    SelfMaintenance,

    // Accrued Gains Errors (§§1372-1390 BGB)
    #[error(
        "Zugewinnausgleich nicht anwendbar: Güterstand ist nicht Zugewinngemeinschaft (§1363 BGB)\n\
         Accrued gains not applicable: Property regime is not community of accrued gains (§1363 BGB)"
    )]
    AccruedGainsNotApplicable,

    #[error(
        "Vermögenswerte fehlen: Anfangsvermögen oder Endvermögen nicht angegeben (§§1374-1375 BGB)\n\
         Assets missing: Initial or final assets not specified (§§1374-1375 BGB)"
    )]
    AssetsMissing,

    #[error(
        "Negative Vermögenswerte: Vermögenswerte können nicht negativ sein\n\
         Negative assets: Asset values cannot be negative"
    )]
    NegativeAssets,

    // Pension Equalization Errors (§§1587-1587p BGB)
    #[error(
        "Versorgungsausgleich fehlt: Versorgungsausgleich muss durchgeführt werden (§1587 BGB)\n\
         Pension equalization missing: Pension equalization must be performed (§1587 BGB)"
    )]
    PensionEqualizationRequired,

    #[error(
        "Versorgungsanrechte fehlen: Versorgungsanrechte für beide Ehegatten erforderlich\n\
         Pension rights missing: Pension rights required for both spouses"
    )]
    PensionRightsMissing,

    // Parentage Errors (§§1591-1600 BGB)
    #[error(
        "Mutter ungültig: Mutter muss weiblich sein (§1591 BGB)\n\
         Mother invalid: Mother must be female (§1591 BGB)"
    )]
    InvalidMother,

    #[error(
        "Vater ungültig: Vater muss männlich sein (§1592 BGB)\n\
         Father invalid: Father must be male (§1592 BGB)"
    )]
    InvalidFather,

    #[error(
        "Kind älter als Elternteil: Geburtsdatum des Kindes liegt vor Geburtsdatum des Elternteils\n\
         Child older than parent: Child's birth date is before parent's birth date"
    )]
    ChildOlderThanParent,

    #[error(
        "Abstammungsstatus fehlt: Kein Abstammungsstatus angegeben (§§1591-1592 BGB)\n\
         Parentage status missing: No parentage status specified (§§1591-1592 BGB)"
    )]
    NoParentageStatus,

    // Parental Custody Errors (§§1626-1698 BGB)
    #[error(
        "Sorgeberechtigter fehlt: Mindestens ein Sorgeberechtigter erforderlich (§1626 BGB)\n\
         Custody holder missing: At least one custody holder required (§1626 BGB)"
    )]
    NoCustodyHolders,

    #[error(
        "Gemeinsame Sorge ungültig: Gemeinsame Sorge erfordert genau zwei Sorgeberechtigte (§1626 BGB)\n\
         Joint custody invalid: Joint custody requires exactly two custody holders (§1626 BGB)"
    )]
    InvalidJointCustody,

    #[error(
        "Kind volljährig: Sorgerecht endet mit Volljährigkeit (§1626 BGB)\n\
         Child adult: Custody ends upon reaching adulthood (§1626 BGB)"
    )]
    ChildAdult,

    #[error(
        "Sorgeberechtigter minderjährig: Sorgeberechtigter muss volljährig sein\n\
         Custody holder minor: Custody holder must be of legal age"
    )]
    CustodyHolderMinor,

    // General Validation Errors
    #[error(
        "Fehlende Person: {person_type} nicht angegeben\n\
         Missing person: {person_type} not specified"
    )]
    MissingPerson { person_type: String },

    #[error(
        "Ungültiges Datum: {date_type} ist ungültig\n\
         Invalid date: {date_type} is invalid"
    )]
    InvalidDate { date_type: String },

    #[error(
        "Name fehlt: Personenname ist leer\n\
         Name missing: Person name is empty"
    )]
    EmptyName,

    #[error(
        "Ungültige Ehegatteneigenschaft: Personen müssen unterschiedlich sein\n\
         Invalid spouse property: Persons must be different"
    )]
    SpousesIdentical,

    #[error(
        "Zukunftsdatum: Datum liegt in der Zukunft\n\
         Future date: Date is in the future"
    )]
    FutureDate,
}

impl FamilyLawError {
    /// Get the BGB article reference for this error
    pub fn article_reference(&self) -> &'static str {
        match self {
            Self::BelowMarriageAge { .. } => "§1303 BGB",
            Self::ExistingMarriage => "§1306 BGB",
            Self::Consanguinity => "§1307 BGB",
            Self::LackOfCapacity => "§1304 BGB",
            Self::MarriageImpedimentsExist => "§§1306-1311 BGB",
            Self::NoRegistrarOffice => "§1310 BGB",
            Self::AgreementNotNotarized => "§1410 BGB",
            Self::AgreementNotBetweenSpouses => "§1408 BGB",
            Self::InsufficientSeparationPeriod { .. } => "§1566 BGB",
            Self::MarriageNotBrokenDown => "§1565 BGB",
            Self::NoConsentAndInsufficientSeparation => "§1566 Abs. 2 BGB",
            Self::CannotDivorceInvalidMarriage => "§1564 BGB",
            Self::NoMaintenanceAmount => "§§1569-1586 BGB",
            Self::NoMaintenanceGround => "§§1570-1576 BGB",
            Self::NoObligor => "§1601 BGB",
            Self::NoBeneficiary => "§1601 BGB",
            Self::SelfMaintenance => "§1601 BGB",
            Self::AccruedGainsNotApplicable => "§1363 BGB",
            Self::AssetsMissing => "§§1374-1375 BGB",
            Self::NegativeAssets => "§1374 BGB",
            Self::PensionEqualizationRequired => "§1587 BGB",
            Self::PensionRightsMissing => "§1587 BGB",
            Self::InvalidMother => "§1591 BGB",
            Self::InvalidFather => "§1592 BGB",
            Self::ChildOlderThanParent => "§§1591-1592 BGB",
            Self::NoParentageStatus => "§§1591-1592 BGB",
            Self::NoCustodyHolders => "§1626 BGB",
            Self::InvalidJointCustody => "§1626 BGB",
            Self::ChildAdult => "§1626 BGB",
            Self::CustodyHolderMinor => "§1626 BGB",
            Self::MissingPerson { .. } => "BGB",
            Self::InvalidDate { .. } => "BGB",
            Self::EmptyName => "BGB",
            Self::SpousesIdentical => "§1310 BGB",
            Self::FutureDate => "BGB",
        }
    }

    /// Check if this is a marriage formation error
    pub fn is_marriage_formation_error(&self) -> bool {
        matches!(
            self,
            Self::BelowMarriageAge { .. }
                | Self::ExistingMarriage
                | Self::Consanguinity
                | Self::LackOfCapacity
                | Self::MarriageImpedimentsExist
                | Self::NoRegistrarOffice
                | Self::SpousesIdentical
        )
    }

    /// Check if this is a divorce error
    pub fn is_divorce_error(&self) -> bool {
        matches!(
            self,
            Self::InsufficientSeparationPeriod { .. }
                | Self::MarriageNotBrokenDown
                | Self::NoConsentAndInsufficientSeparation
                | Self::CannotDivorceInvalidMarriage
        )
    }

    /// Check if this is a maintenance error
    pub fn is_maintenance_error(&self) -> bool {
        matches!(
            self,
            Self::NoMaintenanceAmount
                | Self::NoMaintenanceGround
                | Self::NoObligor
                | Self::NoBeneficiary
                | Self::SelfMaintenance
        )
    }

    /// Check if this is a property regime error
    pub fn is_property_regime_error(&self) -> bool {
        matches!(
            self,
            Self::AccruedGainsNotApplicable
                | Self::AssetsMissing
                | Self::NegativeAssets
                | Self::AgreementNotNotarized
                | Self::AgreementNotBetweenSpouses
        )
    }

    /// Check if this is a parentage error
    pub fn is_parentage_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidMother
                | Self::InvalidFather
                | Self::ChildOlderThanParent
                | Self::NoParentageStatus
        )
    }

    /// Check if this is a custody error
    pub fn is_custody_error(&self) -> bool {
        matches!(
            self,
            Self::NoCustodyHolders
                | Self::InvalidJointCustody
                | Self::ChildAdult
                | Self::CustodyHolderMinor
        )
    }
}
