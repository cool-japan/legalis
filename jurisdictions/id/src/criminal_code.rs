//! Indonesian Criminal Code (KUHP) - Kitab Undang-Undang Hukum Pidana
//!
//! ## Overview
//!
//! The Indonesian Criminal Code (KUHP, Staatsblad 1915 No. 732) is based on the
//! Dutch Wetboek van Strafrecht. A new KUHP was enacted in 2022 (UU No. 1/2023)
//! and will take effect in 2026.
//!
//! ## Structure (Books)
//!
//! ### Book I - General Provisions (Ketentuan Umum)
//! - Pasal 1-85: Scope, criminal liability, attempt, complicity, recidivism
//!
//! ### Book II - Crimes (Kejahatan)
//! - Pasal 104-488: Serious offenses including treason, murder, theft, fraud
//!
//! ### Book III - Misdemeanors (Pelanggaran)
//! - Pasal 489-569: Minor offenses, violations of regulations
//!
//! ## Key Principles
//!
//! - **Nullum crimen sine lege** - No crime without law (Pasal 1(1))
//! - **Criminal liability age**: 12 years under old KUHP, revised in new KUHP
//! - **Mens rea (intent/guilt)** required for most crimes
//! - **Corporate criminal liability** introduced in new KUHP

use serde::{Deserialize, Serialize};

/// Criminal offense category under KUHP
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OffenseCategory {
    /// Kejahatan (Crime) - Book II
    Crime,
    /// Pelanggaran (Misdemeanor) - Book III
    Misdemeanor,
}

impl OffenseCategory {
    /// Get category name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Crime => "Kejahatan",
            Self::Misdemeanor => "Pelanggaran",
        }
    }

    /// Get category name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Crime => "Crime",
            Self::Misdemeanor => "Misdemeanor",
        }
    }
}

/// Types of crimes under KUHP Book II
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrimeType {
    /// Crimes against state security (Pasal 104-129)
    AgainstStateSecurity,
    /// Crimes against public order (Pasal 130-181)
    AgainstPublicOrder,
    /// Crimes against public authority (Pasal 207-241)
    AgainstPublicAuthority,
    /// Perjury and false testimony (Pasal 242-243)
    Perjury,
    /// Crimes against morality (Pasal 281-303)
    AgainstMorality,
    /// Crimes against personal liberty (Pasal 324-337)
    AgainstPersonalLiberty,
    /// Crimes against life (Pasal 338-350)
    AgainstLife,
    /// Assault and battery (Pasal 351-358)
    AssaultAndBattery,
    /// Causing death or injury by negligence (Pasal 359-367)
    CausingDeathByNegligence,
    /// Theft (Pasal 362-367)
    Theft,
    /// Extortion and threats (Pasal 368-371)
    ExtortionAndThreats,
    /// Embezzlement (Pasal 372-377)
    Embezzlement,
    /// Fraud (Pasal 378-395)
    Fraud,
    /// Destruction of property (Pasal 406-412)
    DestructionOfProperty,
    /// Defamation (Pasal 310-321)
    Defamation,
    /// Corruption (separate law UU 31/1999 as amended)
    Corruption,
    /// Cybercrime (UU ITE 19/2016)
    Cybercrime,
    /// Money laundering (UU 8/2010)
    MoneyLaundering,
    /// Other crimes
    Other(String),
}

impl CrimeType {
    /// Get crime type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::AgainstStateSecurity => "Kejahatan terhadap Keamanan Negara",
            Self::AgainstPublicOrder => "Kejahatan terhadap Ketertiban Umum",
            Self::AgainstPublicAuthority => "Kejahatan terhadap Penguasa Umum",
            Self::Perjury => "Sumpah Palsu",
            Self::AgainstMorality => "Kejahatan terhadap Kesusilaan",
            Self::AgainstPersonalLiberty => "Kejahatan terhadap Kemerdekaan Orang",
            Self::AgainstLife => "Kejahatan terhadap Nyawa",
            Self::AssaultAndBattery => "Penganiayaan",
            Self::CausingDeathByNegligence => "Menyebabkan Mati atau Luka karena Kealpaan",
            Self::Theft => "Pencurian",
            Self::ExtortionAndThreats => "Pemerasan dan Ancaman",
            Self::Embezzlement => "Penggelapan",
            Self::Fraud => "Penipuan",
            Self::DestructionOfProperty => "Penghancuran Barang",
            Self::Defamation => "Penghinaan",
            Self::Corruption => "Korupsi",
            Self::Cybercrime => "Kejahatan Siber",
            Self::MoneyLaundering => "Pencucian Uang",
            Self::Other(name) => name,
        }
    }

    /// Get crime type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::AgainstStateSecurity => "Crimes Against State Security",
            Self::AgainstPublicOrder => "Crimes Against Public Order",
            Self::AgainstPublicAuthority => "Crimes Against Public Authority",
            Self::Perjury => "Perjury and False Testimony",
            Self::AgainstMorality => "Crimes Against Morality",
            Self::AgainstPersonalLiberty => "Crimes Against Personal Liberty",
            Self::AgainstLife => "Crimes Against Life",
            Self::AssaultAndBattery => "Assault and Battery",
            Self::CausingDeathByNegligence => "Causing Death or Injury by Negligence",
            Self::Theft => "Theft",
            Self::ExtortionAndThreats => "Extortion and Threats",
            Self::Embezzlement => "Embezzlement",
            Self::Fraud => "Fraud",
            Self::DestructionOfProperty => "Destruction of Property",
            Self::Defamation => "Defamation",
            Self::Corruption => "Corruption",
            Self::Cybercrime => "Cybercrime",
            Self::MoneyLaundering => "Money Laundering",
            Self::Other(name) => name,
        }
    }
}

/// Criminal penalty types - Pasal 10
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PenaltyType {
    /// Principal penalties (pidana pokok)
    Principal(PrincipalPenalty),
    /// Additional penalties (pidana tambahan)
    Additional(AdditionalPenalty),
}

/// Principal penalties - Pasal 10(a)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrincipalPenalty {
    /// Death penalty (pidana mati)
    Death,
    /// Life imprisonment (pidana penjara seumur hidup)
    LifeImprisonment,
    /// Imprisonment for fixed term (pidana penjara sementara)
    Imprisonment {
        /// Minimum days
        min_days: u32,
        /// Maximum days
        max_days: u32,
    },
    /// Detention (pidana kurungan) - for misdemeanors
    Detention {
        /// Minimum days
        min_days: u32,
        /// Maximum days
        max_days: u32,
    },
    /// Fine (pidana denda)
    Fine {
        /// Minimum amount in Rupiah
        min_amount: i64,
        /// Maximum amount in Rupiah
        max_amount: i64,
    },
}

/// Additional penalties - Pasal 10(b)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdditionalPenalty {
    /// Deprivation of certain rights (pencabutan hak-hak tertentu)
    DeprivationOfRights,
    /// Confiscation of certain goods (perampasan barang-barang tertentu)
    Confiscation,
    /// Publication of judgment (pengumuman keputusan hakim)
    PublicationOfJudgment,
    /// Payment of compensation (pembayaran ganti kerja)
    Compensation { amount: i64 },
    /// Fulfillment of customary obligations (pemenuhan kewajiban adat)
    CustomaryObligation,
}

/// Criminal liability age threshold
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CriminalAge {
    /// Old KUHP: 12 years
    Old { age: u32 },
    /// New KUHP (UU 1/2023): Progressive responsibility system
    New { age: u32 },
}

impl CriminalAge {
    /// Check if person can be held criminally liable (old KUHP)
    pub fn is_criminally_liable_old(age: u32) -> bool {
        age >= 12
    }

    /// Check if person can be held criminally liable (new KUHP)
    pub fn is_criminally_liable_new(age: u32) -> bool {
        // New KUHP has progressive system:
        // < 12: no criminal liability
        // 12-14: special treatment
        // 14-18: juvenile justice
        // >= 18: full criminal liability
        age >= 12
    }

    /// Get appropriate juvenile justice measures for age
    pub fn juvenile_justice_measures(age: u32) -> Option<Vec<&'static str>> {
        if age < 12 {
            None // No criminal liability
        } else if age < 14 {
            Some(vec![
                "Pengembalian kepada orang tua/wali",
                "Penyerahan kepada pemerintah",
                "Rehabilitasi",
            ])
        } else if age < 18 {
            Some(vec![
                "Tindakan (measures)",
                "Pidana (penalties)",
                "Pelatihan kerja",
                "Pembinaan dalam lembaga",
            ])
        } else {
            None // Adult criminal liability
        }
    }
}

/// Criminal intent (mens rea) - Pasal 59
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MensRea {
    /// Intentional (sengaja/dolus)
    Intentional,
    /// Negligence (kelalaian/culpa)
    Negligence,
    /// Strict liability (no intent required)
    StrictLiability,
}

impl MensRea {
    /// Get mens rea description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::Intentional => "Kesengajaan (dolus)",
            Self::Negligence => "Kelalaian (culpa)",
            Self::StrictLiability => "Pertanggungjawaban mutlak",
        }
    }

    /// Get mens rea description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::Intentional => "Intentional (dolus)",
            Self::Negligence => "Negligence (culpa)",
            Self::StrictLiability => "Strict liability",
        }
    }
}

/// Criminal offense record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriminalOffense {
    /// Offense identifier
    pub id: String,
    /// Article number (Pasal)
    pub article: u32,
    /// Paragraph (ayat) if applicable
    pub paragraph: Option<u32>,
    /// Offense category
    pub category: OffenseCategory,
    /// Crime type
    pub crime_type: CrimeType,
    /// Required mens rea
    pub mens_rea: MensRea,
    /// Penalty
    pub penalty: Vec<PenaltyType>,
    /// Description in Indonesian
    pub description_id: String,
    /// Description in English
    pub description_en: String,
    /// Whether this is from new KUHP (UU 1/2023)
    pub is_new_kuhp: bool,
}

impl CriminalOffense {
    /// Create new offense record
    pub fn new(
        id: impl Into<String>,
        article: u32,
        category: OffenseCategory,
        crime_type: CrimeType,
        mens_rea: MensRea,
    ) -> Self {
        Self {
            id: id.into(),
            article,
            paragraph: None,
            category,
            crime_type,
            mens_rea,
            penalty: Vec::new(),
            description_id: String::new(),
            description_en: String::new(),
            is_new_kuhp: false,
        }
    }

    /// Set paragraph
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// Add penalty
    pub fn with_penalty(mut self, penalty: PenaltyType) -> Self {
        self.penalty.push(penalty);
        self
    }

    /// Set Indonesian description
    pub fn with_description_id(mut self, description: impl Into<String>) -> Self {
        self.description_id = description.into();
        self
    }

    /// Set English description
    pub fn with_description_en(mut self, description: impl Into<String>) -> Self {
        self.description_en = description.into();
        self
    }

    /// Mark as new KUHP
    pub fn as_new_kuhp(mut self) -> Self {
        self.is_new_kuhp = true;
        self
    }
}

/// Common criminal offenses
pub mod common_offenses {
    use super::*;

    /// Murder (Pasal 338) - imprisonment 15 years
    pub fn murder() -> CriminalOffense {
        CriminalOffense::new(
            "murder",
            338,
            OffenseCategory::Crime,
            CrimeType::AgainstLife,
            MensRea::Intentional,
        )
        .with_penalty(PenaltyType::Principal(PrincipalPenalty::Imprisonment {
            min_days: 0,
            max_days: 15 * 365,
        }))
        .with_description_id("Barangsiapa dengan sengaja merampas nyawa orang lain")
        .with_description_en("Whoever intentionally takes another person's life")
    }

    /// Theft (Pasal 362) - imprisonment max 5 years
    pub fn theft() -> CriminalOffense {
        CriminalOffense::new(
            "theft",
            362,
            OffenseCategory::Crime,
            CrimeType::Theft,
            MensRea::Intentional,
        )
        .with_penalty(PenaltyType::Principal(PrincipalPenalty::Imprisonment {
            min_days: 0,
            max_days: 5 * 365,
        }))
        .with_description_id("Barangsiapa mengambil barang sesuatu yang seluruhnya atau sebagian kepunyaan orang lain")
        .with_description_en("Whoever takes property wholly or partially belonging to another")
    }

    /// Fraud (Pasal 378) - imprisonment max 4 years
    pub fn fraud() -> CriminalOffense {
        CriminalOffense::new(
            "fraud",
            378,
            OffenseCategory::Crime,
            CrimeType::Fraud,
            MensRea::Intentional,
        )
        .with_penalty(PenaltyType::Principal(PrincipalPenalty::Imprisonment {
            min_days: 0,
            max_days: 4 * 365,
        }))
        .with_description_id("Barangsiapa dengan maksud menguntungkan diri sendiri atau orang lain secara melawan hukum dengan memakai nama palsu atau martabat palsu, dengan tipu muslihat")
        .with_description_en("Whoever with intent to unlawfully benefit themselves or others using false name or false dignity, through deceit")
    }

    /// Embezzlement (Pasal 372) - imprisonment max 4 years
    pub fn embezzlement() -> CriminalOffense {
        CriminalOffense::new(
            "embezzlement",
            372,
            OffenseCategory::Crime,
            CrimeType::Embezzlement,
            MensRea::Intentional,
        )
        .with_penalty(PenaltyType::Principal(PrincipalPenalty::Imprisonment {
            min_days: 0,
            max_days: 4 * 365,
        }))
        .with_description_id("Barangsiapa dengan sengaja dan melawan hukum memiliki barang sesuatu yang seluruhnya atau sebagian adalah kepunyaan orang lain")
        .with_description_en("Whoever intentionally and unlawfully possesses property wholly or partially belonging to another")
    }

    /// Defamation (Pasal 310) - imprisonment max 9 months or fine
    pub fn defamation() -> CriminalOffense {
        CriminalOffense::new(
            "defamation",
            310,
            OffenseCategory::Crime,
            CrimeType::Defamation,
            MensRea::Intentional,
        )
        .with_penalty(PenaltyType::Principal(PrincipalPenalty::Imprisonment {
            min_days: 0,
            max_days: 270,
        }))
        .with_penalty(PenaltyType::Principal(PrincipalPenalty::Fine {
            min_amount: 0,
            max_amount: 4_500_000,
        }))
        .with_description_id("Barangsiapa sengaja menyerang kehormatan atau nama baik seseorang dengan menuduhkan sesuatu hal")
        .with_description_en("Whoever intentionally attacks someone's honor or good name by accusing them of something")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offense_category() {
        let crime = OffenseCategory::Crime;
        assert_eq!(crime.name_id(), "Kejahatan");
        assert_eq!(crime.name_en(), "Crime");

        let misdemeanor = OffenseCategory::Misdemeanor;
        assert_eq!(misdemeanor.name_id(), "Pelanggaran");
        assert_eq!(misdemeanor.name_en(), "Misdemeanor");
    }

    #[test]
    fn test_criminal_age_old() {
        assert!(!CriminalAge::is_criminally_liable_old(11));
        assert!(CriminalAge::is_criminally_liable_old(12));
        assert!(CriminalAge::is_criminally_liable_old(18));
    }

    #[test]
    fn test_criminal_age_new() {
        assert!(!CriminalAge::is_criminally_liable_new(11));
        assert!(CriminalAge::is_criminally_liable_new(12));
        assert!(CriminalAge::is_criminally_liable_new(18));
    }

    #[test]
    fn test_juvenile_justice_measures() {
        assert!(CriminalAge::juvenile_justice_measures(11).is_none());
        assert!(CriminalAge::juvenile_justice_measures(13).is_some());
        assert!(CriminalAge::juvenile_justice_measures(16).is_some());
        assert!(CriminalAge::juvenile_justice_measures(18).is_none());
    }

    #[test]
    fn test_mens_rea() {
        let intentional = MensRea::Intentional;
        assert_eq!(intentional.description_id(), "Kesengajaan (dolus)");
        assert_eq!(intentional.description_en(), "Intentional (dolus)");
    }

    #[test]
    fn test_common_offenses() {
        let murder = common_offenses::murder();
        assert_eq!(murder.article, 338);
        assert_eq!(murder.category, OffenseCategory::Crime);
        assert!(!murder.penalty.is_empty());

        let theft = common_offenses::theft();
        assert_eq!(theft.article, 362);

        let fraud = common_offenses::fraud();
        assert_eq!(fraud.article, 378);
    }

    #[test]
    fn test_criminal_offense_builder() {
        let offense = CriminalOffense::new(
            "test",
            100,
            OffenseCategory::Crime,
            CrimeType::Theft,
            MensRea::Intentional,
        )
        .with_paragraph(1)
        .with_penalty(PenaltyType::Principal(PrincipalPenalty::Imprisonment {
            min_days: 0,
            max_days: 365,
        }))
        .with_description_id("Test description ID")
        .with_description_en("Test description EN")
        .as_new_kuhp();

        assert_eq!(offense.article, 100);
        assert_eq!(offense.paragraph, Some(1));
        assert!(offense.is_new_kuhp);
        assert_eq!(offense.penalty.len(), 1);
    }
}
