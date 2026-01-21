//! Land Law Types (ປະເພດກົດໝາຍທີ່ດິນ)
//!
//! This module defines all types related to Lao Land Law 2019 (Law No. 70/NA).
//!
//! ## Legal Foundation: Article 3 - State Ownership Principle
//!
//! **LAO (ພາສາລາວ):**
//! > ທີ່ດິນທັງໝົດເປັນຊັບສິນຂອງຊາດທີ່ລັດເປັນຜູ້ຄຸ້ມຄອງ ແລະ ໃຫ້ປະຊາຊົນ, ອົງການຈັດຕັ້ງນຳໃຊ້
//!
//! **ENGLISH:**
//! > All land is the property of the national community under state management,
//! > and is allocated to the people and organizations for use.
//!
//! This fundamental principle means that in Lao PDR:
//! - Citizens cannot "own" land in the Western sense
//! - Only "land use rights" (ສິດນຳໃຊ້ທີ່ດິນ) can be held
//! - The state retains ultimate ownership
//!
//! ## Land Use Rights (ສິດນຳໃຊ້ທີ່ດິນ)
//!
//! Under Lao law, there are two main types of land use rights:
//!
//! 1. **Perpetual Use Right** (ສິດນຳໃຊ້ຖາວອນ)
//!    - For Lao citizens only
//!    - Can be inherited and transferred
//!    - Subject to land use regulations
//!
//! 2. **Temporary Use Right** (ສິດນຳໃຊ້ຊົ່ວຄາວ)
//!    - Limited duration (typically 30-50 years)
//!    - May be renewable
//!    - Common for commercial purposes
//!
//! ## Foreign Ownership Restrictions
//!
//! Foreign nationals and entities face significant restrictions:
//! - Cannot hold perpetual land use rights
//! - May obtain long-term leases (up to 99 years in special economic zones)
//! - May hold condominium units (but not the land beneath)
//! - Require government approval for most transactions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// State Land Ownership (ທີ່ດິນເປັນຊັບສິນຂອງຊາດ)
///
/// Article 3: All land in Lao PDR is owned by the state and allocated to
/// the people and organizations for use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateLand {
    /// Land parcel ID
    pub parcel_id: String,
    /// Province name (ແຂວງ / Province)
    pub province: String,
    /// District name (ເມືອງ / District)
    pub district: String,
    /// Village name (ບ້ານ / Village)
    pub village: String,
    /// Land area in square meters
    pub area_sqm: u64,
    /// Land classification
    pub classification: LandClassification,
    /// Current land use right holder (if any)
    pub use_right_holder: Option<String>,
}

/// Land Classification (ການຈັດປະເພດທີ່ດິນ)
///
/// Different categories of land under Lao law with different regulations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandClassification {
    /// Agricultural land (ທີ່ດິນກະສິກຳ)
    Agricultural,
    /// Residential land (ທີ່ດິນຢູ່ອາໄສ)
    Residential,
    /// Commercial land (ທີ່ດິນການຄ້າ)
    Commercial,
    /// Industrial land (ທີ່ດິນອຸດສາຫະກຳ)
    Industrial,
    /// Forest land (ທີ່ດິນປ່າໄມ້)
    Forest,
    /// Water resources land (ທີ່ດິນແຫຼ່ງນ້ຳ)
    WaterResources,
    /// Public infrastructure land (ທີ່ດິນໂຄງລ່າງພື້ນຖານ)
    PublicInfrastructure,
}

/// Land Use Right Type (ປະເພດສິດນຳໃຊ້ທີ່ດິນ)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandUseRight {
    /// Perpetual use right (ສິດນຳໃຊ້ຖາວອນ)
    ///
    /// Only available to Lao citizens. Can be inherited and transferred.
    PerpetualUse {
        /// Holder name (must be Lao citizen)
        holder_name: String,
        /// Holder nationality (must be "LAO")
        holder_nationality: String,
        /// Granted date
        granted_at: DateTime<Utc>,
        /// Land parcel ID
        parcel_id: String,
        /// Area in square meters
        area_sqm: u64,
        /// Permitted land use
        permitted_use: LandUsePurpose,
    },

    /// Temporary use right (ສິດນຳໃຊ້ຊົ່ວຄາວ)
    ///
    /// Limited duration, may be renewable. Available to both citizens and organizations.
    TemporaryUse {
        /// Holder name
        holder_name: String,
        /// Holder nationality
        holder_nationality: String,
        /// Granted date
        granted_at: DateTime<Utc>,
        /// Expiry date
        expires_at: DateTime<Utc>,
        /// Land parcel ID
        parcel_id: String,
        /// Area in square meters
        area_sqm: u64,
        /// Permitted land use
        permitted_use: LandUsePurpose,
        /// Whether renewable
        renewable: bool,
    },
}

/// Land Use Purpose (ຈຸດປະສົງການນຳໃຊ້ທີ່ດິນ)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandUsePurpose {
    /// Residential (ຢູ່ອາໄສ)
    Residential,
    /// Agricultural cultivation (ປູກຝັງ)
    Agricultural,
    /// Commercial business (ການຄ້າ)
    Commercial,
    /// Industrial manufacturing (ອຸດສາຫະກຳ)
    Industrial,
    /// Tourism development (ທ່ອງທ່ຽວ)
    Tourism,
    /// Mixed use (ນຳໃຊ້ປະສົມ)
    MixedUse {
        /// List of permitted uses
        purposes: Vec<String>,
    },
}

/// Land Concession Type (ປະເພດສິດສຳປະທານທີ່ດິນ)
///
/// Land concessions are granted by the government for specific economic activities.
/// Governed by Investment Promotion Law and other sector-specific regulations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandConcession {
    /// Agricultural concession (ສຳປະທານກະສິກຳ)
    Agricultural {
        /// Concession holder
        holder: String,
        /// Concession area in hectares
        area_hectares: u64,
        /// Granted date
        granted_at: DateTime<Utc>,
        /// Expiry date (typically 30-50 years)
        expires_at: DateTime<Utc>,
        /// Crop types or agricultural activities
        activities: Vec<String>,
        /// Investment amount in LAK
        investment_lak: u64,
    },

    /// Industrial concession (ສຳປະທານອຸດສາຫະກຳ)
    Industrial {
        /// Concession holder
        holder: String,
        /// Concession area in hectares
        area_hectares: u64,
        /// Granted date
        granted_at: DateTime<Utc>,
        /// Expiry date (typically 50-75 years)
        expires_at: DateTime<Utc>,
        /// Industrial activities
        activities: Vec<String>,
        /// Investment amount in LAK
        investment_lak: u64,
        /// Whether in special economic zone
        in_sez: bool,
    },

    /// Commercial concession (ສຳປະທານການຄ້າ)
    Commercial {
        /// Concession holder
        holder: String,
        /// Concession area in hectares
        area_hectares: u64,
        /// Granted date
        granted_at: DateTime<Utc>,
        /// Expiry date (typically 30-50 years)
        expires_at: DateTime<Utc>,
        /// Commercial activities
        activities: Vec<String>,
        /// Investment amount in LAK
        investment_lak: u64,
    },

    /// Mining concession (ສຳປະທານບໍ່ແຮ່)
    Mining {
        /// Concession holder
        holder: String,
        /// Concession area in hectares
        area_hectares: u64,
        /// Granted date
        granted_at: DateTime<Utc>,
        /// Expiry date (varies by mineral type)
        expires_at: DateTime<Utc>,
        /// Mineral types
        mineral_types: Vec<String>,
        /// Investment amount in LAK
        investment_lak: u64,
        /// Environmental impact assessment approval
        eia_approved: bool,
    },

    /// Tourism concession (ສຳປະທານທ່ອງທ່ຽວ)
    Tourism {
        /// Concession holder
        holder: String,
        /// Concession area in hectares
        area_hectares: u64,
        /// Granted date
        granted_at: DateTime<Utc>,
        /// Expiry date (typically 50-99 years)
        expires_at: DateTime<Utc>,
        /// Tourism activities
        activities: Vec<String>,
        /// Investment amount in LAK
        investment_lak: u64,
    },
}

/// Foreign Ownership Status (ສະຖານະການຖືຄອງຂອງຊາວຕ່າງປະເທດ)
///
/// Tracks whether a person or entity is subject to foreign ownership restrictions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForeignOwnershipStatus {
    /// Lao citizen (ພົນລະເມືອງລາວ)
    LaoCitizen {
        /// Citizen ID number
        citizen_id: String,
    },

    /// Foreign national (ຊາວຕ່າງປະເທດ)
    ForeignNational {
        /// Passport number
        passport_number: String,
        /// Nationality
        nationality: String,
        /// Whether approved for land lease
        lease_approved: bool,
    },

    /// Lao legal entity (ນິຕິບຸກຄົນລາວ)
    LaoEntity {
        /// Business registration number
        registration_number: String,
        /// Entity name
        entity_name: String,
    },

    /// Foreign-invested entity (ນິຕິບຸກຄົນມີທຶນຕ່າງປະເທດ)
    ForeignInvestedEntity {
        /// Business registration number
        registration_number: String,
        /// Entity name
        entity_name: String,
        /// Foreign ownership percentage (0-100)
        foreign_ownership_pct: u8,
        /// Investment license number (if applicable)
        investment_license: Option<String>,
    },
}

/// Land Title Type (ປະເພດໃບຕາດິນ)
///
/// Different types of land title documents issued by the land authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandTitleType {
    /// Full title deed (ໃບຕາດິນເຕັມສິດ)
    ///
    /// Highest form of land title, provides strongest legal protection.
    FullTitle,

    /// Temporary title certificate (ໃບຢັ້ງຢືນສິດນຳໃຊ້ທີ່ດິນຊົ່ວຄາວ)
    ///
    /// Temporary certificate pending full survey and registration.
    TemporaryCertificate,

    /// Tax receipt (ໃບເສຍພາສີ)
    ///
    /// Land tax receipt serving as evidence of use right (older system).
    TaxReceipt,

    /// Lease agreement (ສັນຍາເຊົ່າ)
    ///
    /// For leased land, not a title per se.
    LeaseAgreement,
}

/// Land Title (ໃບຕາດິນ)
///
/// Official document certifying land use rights.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandTitle {
    /// Title number
    pub title_number: String,
    /// Title type
    pub title_type: LandTitleType,
    /// Land parcel ID
    pub parcel_id: String,
    /// Holder name
    pub holder_name: String,
    /// Holder ownership status
    pub holder_status: ForeignOwnershipStatus,
    /// Land area in square meters
    pub area_sqm: u64,
    /// Location description (ແຂວງ/ເມືອງ/ບ້ານ)
    pub location: String,
    /// Issued date
    pub issued_at: DateTime<Utc>,
    /// Issuing authority
    pub issuing_authority: String,
    /// Whether registered in central cadastre
    pub cadastre_registered: bool,
}

/// Land Certificate (ໃບຢັ້ງຢືນສິດນຳໃຊ້ທີ່ດິນ)
///
/// Certificate of land use right (alternative to full title in some areas).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandCertificate {
    /// Certificate number
    pub certificate_number: String,
    /// Land parcel ID
    pub parcel_id: String,
    /// Holder name
    pub holder_name: String,
    /// Land area in square meters
    pub area_sqm: u64,
    /// Location description
    pub location: String,
    /// Issued date
    pub issued_at: DateTime<Utc>,
    /// Valid until (if temporary)
    pub valid_until: Option<DateTime<Utc>>,
}

/// Cadastral Survey (ການສຳຫຼວດທີ່ດິນ)
///
/// Official survey of land parcels for registration purposes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CadastralSurvey {
    /// Survey ID
    pub survey_id: String,
    /// Land parcel ID
    pub parcel_id: String,
    /// Survey date
    pub survey_date: DateTime<Utc>,
    /// Surveyor name
    pub surveyor: String,
    /// Surveyor license number
    pub surveyor_license: String,
    /// Measured area in square meters
    pub measured_area_sqm: f64,
    /// Boundary coordinates (lat, lon pairs)
    pub boundary_coordinates: Vec<(f64, f64)>,
    /// Adjacent parcel IDs
    pub adjacent_parcels: Vec<String>,
    /// Survey method
    pub survey_method: SurveyMethod,
    /// Approved by authority
    pub approved: bool,
}

/// Survey Method (ວິທີການສຳຫຼວດ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurveyMethod {
    /// GPS survey
    GPS,
    /// Traditional measurement
    Traditional,
    /// Aerial survey
    Aerial,
    /// Satellite imagery
    Satellite,
}

/// Land Transaction Type (ປະເພດການເຮັດທຸລະກຳທີ່ດິນ)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandTransactionType {
    /// Sale of land use right (ການຂາຍສິດນຳໃຊ້ທີ່ດິນ)
    Sale {
        /// Seller name
        seller: String,
        /// Buyer name
        buyer: String,
        /// Sale price in LAK
        price_lak: u64,
        /// Transaction date
        transaction_date: DateTime<Utc>,
    },

    /// Lease of land use right (ການເຊົ່າສິດນຳໃຊ້ທີ່ດິນ)
    Lease {
        /// Lessor name
        lessor: String,
        /// Lessee name
        lessee: String,
        /// Lease duration in years
        duration_years: u32,
        /// Annual rent in LAK
        annual_rent_lak: u64,
        /// Lease start date
        start_date: DateTime<Utc>,
        /// Lease end date
        end_date: DateTime<Utc>,
    },

    /// Mortgage of land use right (ການຈຳນອງສິດນຳໃຊ້ທີ່ດິນ)
    Mortgage {
        /// Mortgagor name
        mortgagor: String,
        /// Mortgagee name (usually bank)
        mortgagee: String,
        /// Loan amount in LAK
        loan_amount_lak: u64,
        /// Mortgage date
        mortgage_date: DateTime<Utc>,
        /// Maturity date
        maturity_date: DateTime<Utc>,
    },

    /// Inheritance transfer (ການໂອນມໍລະດົກ)
    Inheritance {
        /// Deceased owner
        deceased: String,
        /// Heir name
        heir: String,
        /// Transfer date
        transfer_date: DateTime<Utc>,
    },

    /// Gift transfer (ການມອບໃຫ້)
    Gift {
        /// Donor name
        donor: String,
        /// Donee name
        donee: String,
        /// Transfer date
        transfer_date: DateTime<Utc>,
    },
}

/// Land Transaction (ການເຮັດທຸລະກຳທີ່ດິນ)
///
/// Records a transaction involving land use rights.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandTransaction {
    /// Transaction ID
    pub transaction_id: String,
    /// Land parcel ID
    pub parcel_id: String,
    /// Transaction type
    pub transaction_type: LandTransactionType,
    /// Area involved in square meters
    pub area_sqm: u64,
    /// Government approval obtained
    pub government_approval: bool,
    /// Approval authority
    pub approval_authority: Option<String>,
    /// Registered in land office
    pub registered: bool,
    /// Registration date
    pub registration_date: Option<DateTime<Utc>>,
}

/// Land Dispute Type (ປະເພດຂໍ້ຂັດແຍ່ງທີ່ດິນ)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandDisputeType {
    /// Boundary dispute (ຂໍ້ຂັດແຍ່ງກ່ຽວກັບເຂດແດນ)
    Boundary {
        /// First party
        party1: String,
        /// Second party
        party2: String,
        /// Disputed parcel IDs
        disputed_parcels: Vec<String>,
    },

    /// Ownership dispute (ຂໍ້ຂັດແຍ່ງກ່ຽວກັບກຳມະສິດ)
    Ownership {
        /// Claimant 1
        claimant1: String,
        /// Claimant 2
        claimant2: String,
        /// Disputed parcel ID
        disputed_parcel: String,
    },

    /// Illegal occupation (ການຍຶດຄອງທີ່ດິນໂດຍຜິດກົດໝາຍ)
    IllegalOccupation {
        /// Occupier name
        occupier: String,
        /// Rightful owner
        rightful_owner: String,
        /// Parcel ID
        parcel_id: String,
    },

    /// Transaction dispute (ຂໍ້ຂັດແຍ່ງກ່ຽວກັບການເຮັດທຸລະກຳ)
    TransactionDispute {
        /// Party 1
        party1: String,
        /// Party 2
        party2: String,
        /// Transaction ID
        transaction_id: String,
    },

    /// Compensation dispute (ຂໍ້ຂັດແຍ່ງກ່ຽວກັບການຊົດເຊີຍ)
    Compensation {
        /// Land owner
        land_owner: String,
        /// Government authority
        authority: String,
        /// Parcel ID (for public use acquisition)
        parcel_id: String,
        /// Disputed compensation amount in LAK
        disputed_amount_lak: u64,
    },
}

/// Land Dispute (ຂໍ້ຂັດແຍ່ງທີ່ດິນ)
///
/// Records a dispute over land use rights or boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandDispute {
    /// Dispute ID
    pub dispute_id: String,
    /// Dispute type
    pub dispute_type: LandDisputeType,
    /// Filed date
    pub filed_at: DateTime<Utc>,
    /// Current status
    pub status: DisputeStatus,
    /// Resolution method
    pub resolution_method: Option<ResolutionMethod>,
    /// Resolution date (if resolved)
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Dispute Status (ສະຖານະຂໍ້ຂັດແຍ່ງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisputeStatus {
    /// Pending (ລໍຖ້າການແກ້ໄຂ)
    Pending,
    /// Under mediation (ກຳລັງໄກ່ເກ່ຍ)
    UnderMediation,
    /// Under litigation (ກຳລັງດຳເນີນຄະດີ)
    UnderLitigation,
    /// Resolved (ແກ້ໄຂແລ້ວ)
    Resolved,
    /// Appealed (ອຸທອນແລ້ວ)
    Appealed,
}

/// Resolution Method (ວິທີການແກ້ໄຂຂໍ້ຂັດແຍ່ງ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionMethod {
    /// Village mediation (ການໄກ່ເກ່ຍໃນບ້ານ)
    VillageMediation,
    /// District mediation (ການໄກ່ເກ່ຍໃນເມືອງ)
    DistrictMediation,
    /// Court judgment (ຄຳຕັດສິນຂອງສານ)
    CourtJudgment,
    /// Administrative decision (ຄຳຕັດສິນທາງບໍລິຫານ)
    AdministrativeDecision,
}

/// Land Registration Status (ສະຖານະການລົງທະບຽນທີ່ດິນ)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandRegistrationStatus {
    /// Parcel ID
    pub parcel_id: String,
    /// Whether cadastral survey completed
    pub survey_completed: bool,
    /// Whether title issued
    pub title_issued: bool,
    /// Whether registered in central database
    pub centrally_registered: bool,
    /// Registration office
    pub registration_office: String,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_land_creation() {
        let land = StateLand {
            parcel_id: "VTE-001-2023".to_string(),
            province: "Vientiane Capital".to_string(),
            district: "Chanthabouly".to_string(),
            village: "Ban Mixay".to_string(),
            area_sqm: 500,
            classification: LandClassification::Residential,
            use_right_holder: Some("Boupha Somchanh".to_string()),
        };
        assert_eq!(land.area_sqm, 500);
        assert_eq!(land.classification, LandClassification::Residential);
    }

    #[test]
    fn test_perpetual_use_right() {
        let use_right = LandUseRight::PerpetualUse {
            holder_name: "Khamla Sisavath".to_string(),
            holder_nationality: "LAO".to_string(),
            granted_at: Utc::now(),
            parcel_id: "VTE-002-2023".to_string(),
            area_sqm: 1000,
            permitted_use: LandUsePurpose::Residential,
        };

        match use_right {
            LandUseRight::PerpetualUse {
                holder_nationality, ..
            } => {
                assert_eq!(holder_nationality, "LAO");
            }
            _ => panic!("Expected PerpetualUse"),
        }
    }

    #[test]
    fn test_foreign_national_status() {
        let status = ForeignOwnershipStatus::ForeignNational {
            passport_number: "P1234567".to_string(),
            nationality: "Thailand".to_string(),
            lease_approved: true,
        };

        match status {
            ForeignOwnershipStatus::ForeignNational { nationality, .. } => {
                assert_eq!(nationality, "Thailand");
            }
            _ => panic!("Expected ForeignNational"),
        }
    }

    #[test]
    fn test_land_transaction_sale() {
        let transaction = LandTransaction {
            transaction_id: "TX-001-2023".to_string(),
            parcel_id: "VTE-003-2023".to_string(),
            transaction_type: LandTransactionType::Sale {
                seller: "Seller A".to_string(),
                buyer: "Buyer B".to_string(),
                price_lak: 500_000_000,
                transaction_date: Utc::now(),
            },
            area_sqm: 800,
            government_approval: true,
            approval_authority: Some("Department of Land Management".to_string()),
            registered: true,
            registration_date: Some(Utc::now()),
        };

        assert_eq!(transaction.transaction_id, "TX-001-2023");
        assert!(transaction.government_approval);
    }

    #[test]
    fn test_land_dispute_boundary() {
        let dispute = LandDispute {
            dispute_id: "DISP-001-2023".to_string(),
            dispute_type: LandDisputeType::Boundary {
                party1: "Party A".to_string(),
                party2: "Party B".to_string(),
                disputed_parcels: vec!["VTE-004-2023".to_string(), "VTE-005-2023".to_string()],
            },
            filed_at: Utc::now(),
            status: DisputeStatus::UnderMediation,
            resolution_method: None,
            resolved_at: None,
        };

        assert_eq!(dispute.status, DisputeStatus::UnderMediation);
    }
}
