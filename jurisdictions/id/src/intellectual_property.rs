//! Indonesian Intellectual Property Law
//!
//! ## Overview
//!
//! IP protection in Indonesia is governed by several laws:
//! - **UU No. 28/2014**: Hak Cipta (Copyright)
//! - **UU No. 13/2016**: Paten (Patent)
//! - **UU No. 20/2016**: Merek dan Indikasi Geografis (Trademark and Geographical Indication)
//! - **UU No. 31/2000**: Desain Industri (Industrial Design)
//! - **UU No. 30/2000**: Rahasia Dagang (Trade Secret)
//! - **UU No. 32/2000**: Desain Tata Letak Sirkuit Terpadu (IC Layout Design)
//!
//! ## Regulatory Authority
//!
//! - **Direktorat Jenderal Kekayaan Intelektual (DJKI)**: Directorate General of Intellectual Property

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of intellectual property
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpType {
    /// Copyright (Hak Cipta)
    Copyright,
    /// Patent (Paten)
    Patent,
    /// Simple patent (Paten Sederhana)
    SimplePatent,
    /// Trademark (Merek)
    Trademark,
    /// Geographical indication (Indikasi Geografis)
    GeographicalIndication,
    /// Industrial design (Desain Industri)
    IndustrialDesign,
    /// Trade secret (Rahasia Dagang)
    TradeSecret,
    /// IC layout design (Desain Tata Letak Sirkuit Terpadu)
    IcLayoutDesign,
}

impl IpType {
    /// Get IP type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Copyright => "Hak Cipta",
            Self::Patent => "Paten",
            Self::SimplePatent => "Paten Sederhana",
            Self::Trademark => "Merek",
            Self::GeographicalIndication => "Indikasi Geografis",
            Self::IndustrialDesign => "Desain Industri",
            Self::TradeSecret => "Rahasia Dagang",
            Self::IcLayoutDesign => "Desain Tata Letak Sirkuit Terpadu",
        }
    }

    /// Get IP type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Copyright => "Copyright",
            Self::Patent => "Patent",
            Self::SimplePatent => "Simple Patent",
            Self::Trademark => "Trademark",
            Self::GeographicalIndication => "Geographical Indication",
            Self::IndustrialDesign => "Industrial Design",
            Self::TradeSecret => "Trade Secret",
            Self::IcLayoutDesign => "IC Layout Design",
        }
    }

    /// Get protection duration in years
    pub fn protection_duration_years(&self) -> Option<u32> {
        match self {
            Self::Copyright => Some(70), // Life + 70 years
            Self::Patent => Some(20),
            Self::SimplePatent => Some(10),
            Self::Trademark => Some(10), // Renewable indefinitely
            Self::GeographicalIndication => None, // No expiry
            Self::IndustrialDesign => Some(10),
            Self::TradeSecret => None, // As long as kept secret
            Self::IcLayoutDesign => Some(10),
        }
    }

    /// Check if registration is required for protection
    pub fn requires_registration(&self) -> bool {
        match self {
            Self::Copyright | Self::TradeSecret => false, // Automatic protection
            _ => true,                                    // Registration required
        }
    }
}

/// Copyright work type - Pasal 40 UU 28/2014
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CopyrightWorkType {
    /// Literary work (book, computer program, etc.)
    LiteraryWork,
    /// Musical work (song, musical composition)
    MusicalWork,
    /// Performing arts (dance, choreography)
    PerformingArts,
    /// Visual arts (painting, sculpture, batik)
    VisualArts,
    /// Cinematographic work (film, video)
    CinematographicWork,
    /// Photographic work
    PhotographicWork,
    /// Architectural work (building design)
    ArchitecturalWork,
    /// Map and technical drawing
    MapAndTechnicalDrawing,
    /// Translation, adaptation, compilation
    DerivativeWork,
    /// Database
    Database,
}

impl CopyrightWorkType {
    /// Get work type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::LiteraryWork => "Karya Tulis (buku, program komputer, dll)",
            Self::MusicalWork => "Karya Musik (lagu, komposisi musik)",
            Self::PerformingArts => "Karya Seni Pertunjukan (tari, koreografi)",
            Self::VisualArts => "Karya Seni Rupa (lukisan, patung, batik)",
            Self::CinematographicWork => "Karya Sinematografi (film, video)",
            Self::PhotographicWork => "Karya Fotografi",
            Self::ArchitecturalWork => "Karya Arsitektur",
            Self::MapAndTechnicalDrawing => "Peta dan Gambar Teknis",
            Self::DerivativeWork => "Terjemahan, Adaptasi, Kompilasi",
            Self::Database => "Basis Data",
        }
    }

    /// Get work type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::LiteraryWork => "Literary Work",
            Self::MusicalWork => "Musical Work",
            Self::PerformingArts => "Performing Arts",
            Self::VisualArts => "Visual Arts",
            Self::CinematographicWork => "Cinematographic Work",
            Self::PhotographicWork => "Photographic Work",
            Self::ArchitecturalWork => "Architectural Work",
            Self::MapAndTechnicalDrawing => "Map and Technical Drawing",
            Self::DerivativeWork => "Derivative Work",
            Self::Database => "Database",
        }
    }
}

/// Patent patentability requirements - Pasal 2-9 UU 13/2016
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatentRequirements {
    /// Novel (baru) - not disclosed to public before filing
    pub is_novel: bool,
    /// Inventive step (mengandung langkah inventif) - non-obvious to person skilled in art
    pub has_inventive_step: bool,
    /// Industrially applicable (dapat diterapkan dalam industri)
    pub is_industrially_applicable: bool,
}

impl PatentRequirements {
    /// Check if invention meets patentability requirements
    pub fn is_patentable(&self) -> bool {
        self.is_novel && self.has_inventive_step && self.is_industrially_applicable
    }
}

/// Non-patentable subject matter - Pasal 9 UU 13/2016
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NonPatentableSubject {
    /// Scientific theories and mathematical methods
    ScientificTheories,
    /// Aesthetic creations
    AestheticCreations,
    /// Business methods, computer programs (as such)
    BusinessMethods,
    /// Presentation of information
    PresentationOfInformation,
    /// Plant or animal varieties (protected by separate law)
    PlantAnimalVarieties,
    /// Diagnostic, therapeutic, surgical methods for humans/animals
    MedicalMethods,
}

impl NonPatentableSubject {
    /// Get subject description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::ScientificTheories => "Teori dan metode ilmiah dan matematika",
            Self::AestheticCreations => "Kreasi estetika",
            Self::BusinessMethods => "Metode bisnis dan program komputer",
            Self::PresentationOfInformation => "Presentasi tentang informasi",
            Self::PlantAnimalVarieties => "Varietas tanaman atau hewan",
            Self::MedicalMethods => "Metode diagnostik, terapeutik, dan bedah untuk manusia/hewan",
        }
    }

    /// Get subject description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::ScientificTheories => "Scientific theories and mathematical methods",
            Self::AestheticCreations => "Aesthetic creations",
            Self::BusinessMethods => "Business methods and computer programs",
            Self::PresentationOfInformation => "Presentation of information",
            Self::PlantAnimalVarieties => "Plant or animal varieties",
            Self::MedicalMethods => "Diagnostic, therapeutic, and surgical methods",
        }
    }
}

/// Trademark type - Pasal 1 UU 20/2016
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrademarkType {
    /// Word mark (nama, kata)
    WordMark,
    /// Figurative mark (logo, gambar)
    FigurativeMark,
    /// Combined mark (word + logo)
    CombinedMark,
    /// Three-dimensional mark (bentuk 3D)
    ThreeDimensionalMark,
    /// Sound mark (suara)
    SoundMark,
    /// Hologram mark
    HologramMark,
}

impl TrademarkType {
    /// Get trademark type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::WordMark => "Merek Kata",
            Self::FigurativeMark => "Merek Lukisan",
            Self::CombinedMark => "Merek Kombinasi",
            Self::ThreeDimensionalMark => "Merek Tiga Dimensi",
            Self::SoundMark => "Merek Suara",
            Self::HologramMark => "Merek Hologram",
        }
    }

    /// Get trademark type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::WordMark => "Word Mark",
            Self::FigurativeMark => "Figurative Mark",
            Self::CombinedMark => "Combined Mark",
            Self::ThreeDimensionalMark => "Three-Dimensional Mark",
            Self::SoundMark => "Sound Mark",
            Self::HologramMark => "Hologram Mark",
        }
    }
}

/// Grounds for trademark refusal - Pasal 20-21 UU 20/2016
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrademarkRefusalGround {
    /// Identical or similar to registered mark for same class
    ConfusinglySimilar,
    /// Generic or common term
    Generic,
    /// Descriptive of goods/services
    Descriptive,
    /// Contrary to public order or morality
    ContraryToPublicOrder,
    /// Identical to well-known mark
    IdenticalToWellKnownMark,
    /// Identical to geographical indication
    IdenticalToGeographicalIndication,
    /// Similar to state emblem, flag, or official seal
    SimilarToStateEmblem,
    /// Bad faith registration
    BadFaith,
}

impl TrademarkRefusalGround {
    /// Get refusal ground description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::ConfusinglySimilar => {
                "Mempunyai persamaan pada pokoknya atau keseluruhannya dengan Merek terdaftar"
            }
            Self::Generic => "Merupakan nama generik atau umum",
            Self::Descriptive => "Merupakan keterangan atau berkaitan dengan barang/jasa",
            Self::ContraryToPublicOrder => {
                "Bertentangan dengan peraturan perundang-undangan, moralitas agama, kesusilaan, atau ketertiban umum"
            }
            Self::IdenticalToWellKnownMark => {
                "Merupakan atau menyerupai nama atau singkatan nama orang terkenal"
            }
            Self::IdenticalToGeographicalIndication => {
                "Merupakan atau menyerupai Indikasi Geografis yang sudah dikenal"
            }
            Self::SimilarToStateEmblem => {
                "Merupakan tiruan atau menyerupai nama atau singkatan nama, bendera, lambang atau simbol atau emblem negara"
            }
            Self::BadFaith => "Permohonan yang diajukan oleh Pemohon yang beriktikad tidak baik",
        }
    }

    /// Get refusal ground description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::ConfusinglySimilar => "Confusingly similar to registered trademark",
            Self::Generic => "Generic or common term",
            Self::Descriptive => "Descriptive of goods or services",
            Self::ContraryToPublicOrder => "Contrary to law, morality, religion, or public order",
            Self::IdenticalToWellKnownMark => "Identical or similar to well-known mark",
            Self::IdenticalToGeographicalIndication => {
                "Identical or similar to geographical indication"
            }
            Self::SimilarToStateEmblem => "Similar to state emblem, flag, or official seal",
            Self::BadFaith => "Application filed in bad faith",
        }
    }
}

/// IP registration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRegistration {
    /// Registration number
    pub registration_number: String,
    /// IP type
    pub ip_type: IpType,
    /// Title or name
    pub title: String,
    /// Owner name
    pub owner_name: String,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Registration date (if granted)
    pub registration_date: Option<NaiveDate>,
    /// Expiry date
    pub expiry_date: Option<NaiveDate>,
    /// Whether registration is active
    pub is_active: bool,
}

impl IpRegistration {
    /// Calculate expiry date from registration date
    pub fn calculate_expiry_date(&self) -> Option<NaiveDate> {
        if let Some(duration) = self.ip_type.protection_duration_years() {
            self.registration_date.and_then(|reg_date| {
                reg_date.checked_add_signed(chrono::Duration::days((duration * 365) as i64))
            })
        } else {
            None
        }
    }

    /// Check if registration is still valid
    pub fn is_valid(&self, current_date: NaiveDate) -> bool {
        if !self.is_active {
            return false;
        }

        match self.expiry_date {
            Some(expiry) => current_date <= expiry,
            None => true, // No expiry
        }
    }
}

/// IP infringement type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementType {
    /// Direct infringement (reproducing, distributing, performing)
    DirectInfringement,
    /// Contributory infringement (facilitating infringement)
    ContributoryInfringement,
    /// Inducing infringement
    InducingInfringement,
    /// Counterfeiting (trademark)
    Counterfeiting,
    /// Passing off
    PassingOff,
    /// Unauthorized use
    UnauthorizedUse,
}

impl InfringementType {
    /// Get infringement type description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::DirectInfringement => "Pelanggaran langsung",
            Self::ContributoryInfringement => "Pelanggaran kontributif",
            Self::InducingInfringement => "Menginduksi pelanggaran",
            Self::Counterfeiting => "Pemalsuan",
            Self::PassingOff => "Penyamaran",
            Self::UnauthorizedUse => "Penggunaan tanpa izin",
        }
    }

    /// Get infringement type description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::DirectInfringement => "Direct infringement",
            Self::ContributoryInfringement => "Contributory infringement",
            Self::InducingInfringement => "Inducing infringement",
            Self::Counterfeiting => "Counterfeiting",
            Self::PassingOff => "Passing off",
            Self::UnauthorizedUse => "Unauthorized use",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_type() {
        let copyright = IpType::Copyright;
        assert_eq!(copyright.name_id(), "Hak Cipta");
        assert_eq!(copyright.protection_duration_years(), Some(70));
        assert!(!copyright.requires_registration());

        let patent = IpType::Patent;
        assert_eq!(patent.protection_duration_years(), Some(20));
        assert!(patent.requires_registration());
    }

    #[test]
    fn test_patent_requirements() {
        let patentable = PatentRequirements {
            is_novel: true,
            has_inventive_step: true,
            is_industrially_applicable: true,
        };
        assert!(patentable.is_patentable());

        let not_patentable = PatentRequirements {
            is_novel: true,
            has_inventive_step: false,
            is_industrially_applicable: true,
        };
        assert!(!not_patentable.is_patentable());
    }

    #[test]
    fn test_copyright_work_type() {
        let literary = CopyrightWorkType::LiteraryWork;
        assert_eq!(
            literary.name_id(),
            "Karya Tulis (buku, program komputer, dll)"
        );
        assert_eq!(literary.name_en(), "Literary Work");
    }

    #[test]
    fn test_trademark_type() {
        let word_mark = TrademarkType::WordMark;
        assert_eq!(word_mark.name_id(), "Merek Kata");
        assert_eq!(word_mark.name_en(), "Word Mark");
    }

    #[test]
    fn test_ip_registration_validity() {
        let registration = IpRegistration {
            registration_number: "REG001".to_string(),
            ip_type: IpType::Patent,
            title: "Test Patent".to_string(),
            owner_name: "Test Owner".to_string(),
            filing_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date"),
            registration_date: Some(NaiveDate::from_ymd_opt(2021, 1, 1).expect("Valid date")),
            expiry_date: Some(NaiveDate::from_ymd_opt(2041, 1, 1).expect("Valid date")),
            is_active: true,
        };

        let current_date = NaiveDate::from_ymd_opt(2025, 1, 1).expect("Valid date");
        assert!(registration.is_valid(current_date));

        let future_date = NaiveDate::from_ymd_opt(2042, 1, 1).expect("Valid date");
        assert!(!registration.is_valid(future_date));
    }
}
