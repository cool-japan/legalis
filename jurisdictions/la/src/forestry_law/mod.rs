//! Forestry Law Module (ກົດໝາຍປ່າໄມ້)
//!
//! This module provides comprehensive support for Lao forestry law based on
//! **Forestry Law 2019** (Law No. 64/NA, dated June 13, 2019).
//!
//! # Legal Framework
//!
//! The Forestry Law 2019 is the primary legislation governing forest management,
//! conservation, and utilization in the Lao People's Democratic Republic. It establishes:
//!
//! - Forest classification system
//! - Forest use rights and permits
//! - Timber harvesting regulations
//! - Forest concession requirements
//! - Protected species management
//! - Community forestry framework
//! - Penalties and enforcement mechanisms
//!
//! # Key Provisions
//!
//! ## Forest Classifications (ການຈັດປະເພດປ່າໄມ້) - Articles 10-25
//!
//! **Article 10-14**: Forest classification categories:
//!
//! - **Protection Forests (ປ່າປ້ອງກັນ)** - Article 11
//!   - Watershed protection
//!   - Erosion control
//!   - National security zones
//!   - Strictly regulated, no commercial harvesting
//!
//! - **Conservation Forests (ປ່າສະຫງວນ)** - Article 12
//!   - Biodiversity conservation
//!   - Wildlife habitat preservation
//!   - National protected areas
//!   - Research and education purposes
//!
//! - **Production Forests (ປ່າຜະລິດ)** - Article 13
//!   - Sustainable timber harvesting
//!   - Annual Allowable Cut (AAC) system
//!   - Commercial utilization permitted
//!   - Reforestation requirements
//!
//! - **Rehabilitation Forests (ປ່າຟື້ນຟູ)** - Article 14
//!   - Degraded forest restoration
//!   - Reforestation projects
//!   - Restricted harvesting until recovery
//!
//! - **Village Forests (ປ່າບ້ານ)** - Article 15
//!   - Community management
//!   - Traditional use rights
//!   - Local benefit sharing
//!
//! ## Forest Use Rights (ສິດນຳໃຊ້ປ່າໄມ້) - Articles 30-45
//!
//! **Article 30-35**: Use right categories:
//!
//! - **Customary Use Rights** - Article 31
//!   - Traditional forest uses
//!   - Subsistence harvesting
//!   - Cultural practices
//!
//! - **Timber Harvesting Permits** - Article 32
//!   - Commercial logging permits
//!   - AAC allocation
//!   - Species-specific quotas
//!
//! - **Non-Timber Forest Products (NTFP)** - Article 33
//!   - Collection permits
//!   - Sustainable harvesting
//!   - Community allocation
//!
//! - **Forest Land Allocation** - Article 34
//!   - Household allocation
//!   - Community allocation
//!   - Agricultural use rights
//!
//! ## Timber Harvesting Regulations (ລະບຽບການຕັດໄມ້) - Articles 46-60
//!
//! **Article 46-50**: Harvesting requirements:
//!
//! - **Annual Allowable Cut (AAC)** - Article 47
//!   - National AAC determined annually
//!   - Provincial/district allocations
//!   - Species-specific quotas
//!
//! - **Harvesting Season** - Article 48
//!   - Dry season only: November to April
//!   - Wet season harvesting prohibited
//!   - Exception for salvage operations
//!
//! - **Minimum Diameter Limits** - Article 49
//!   - Species-specific cutting limits
//!   - Teak: minimum 40 cm DBH
//!   - Rosewood: minimum 30 cm DBH
//!   - Other hardwoods: minimum 35 cm DBH
//!
//! - **Prohibited Species** - Article 50
//!   - Category I: Strictly protected (no harvest)
//!   - Rosewood (Dalbergia spp.) restrictions
//!   - Agarwood (Aquilaria spp.) protections
//!
//! - **Log Tracking System** - Article 51
//!   - Mandatory log marking
//!   - Transport documentation
//!   - Chain of custody requirements
//!
//! ## Forest Concessions (ສຳປະທານປ່າໄມ້) - Articles 61-75
//!
//! **Article 61-65**: Concession types and limits:
//!
//! - **Forest Management Concession** - Article 62
//!   - Maximum term: 40 years
//!   - Maximum area: 10,000 hectares
//!   - Sustainable management plan required
//!   - Performance bond: 5% of project value
//!
//! - **Forest Plantation Concession** - Article 63
//!   - Maximum term: 50 years
//!   - Maximum area: 15,000 hectares
//!   - Environmental impact assessment required
//!   - Performance bond: 3% of project value
//!
//! ## Protected Species (ຊະນິດພັນທີ່ຖືກປົກປ້ອງ) - Articles 76-90
//!
//! **Article 76-80**: Species classification:
//!
//! - **Category I: Strictly Protected** - Article 77
//!   - No harvesting permitted
//!   - Critical endangered status
//!   - Maximum legal protection
//!
//! - **Category II: Managed Species** - Article 78
//!   - Quota-based harvesting
//!   - Permit required
//!   - Population monitoring
//!
//! - **Category III: Common Species** - Article 79
//!   - General harvesting permitted
//!   - Standard permit requirements
//!   - Sustainable use principles
//!
//! - **CITES Compliance** - Article 80
//!   - International trade restrictions
//!   - Export permit requirements
//!   - Documentation standards
//!
//! ## Community Forestry (ປ່າໄມ້ຊຸມຊົນ) - Articles 91-105
//!
//! **Article 91-95**: Community forest management:
//!
//! - **Village Forest Management Agreement** - Article 92
//!   - Formal agreement with authorities
//!   - Management plan requirements
//!   - Benefit sharing arrangements
//!
//! - **Community Forest Enterprises** - Article 93
//!   - Commercial activities permitted
//!   - Registration requirements
//!   - Revenue sharing rules
//!
//! - **Benefit Sharing Mechanisms** - Article 94
//!   - Village development fund: 50%
//!   - District forest fund: 30%
//!   - National forest fund: 20%
//!
//! - **Traditional Use Protections** - Article 95
//!   - Customary harvesting rights
//!   - Cultural site protections
//!   - Indigenous knowledge respect
//!
//! ## Penalties and Enforcement (ໂທດ ແລະ ການບັງຄັບໃຊ້) - Articles 106-120
//!
//! **Article 106-115**: Violation penalties:
//!
//! - **Illegal Logging** - Article 107
//!   - Fine: 2-10x timber value
//!   - Equipment confiscation
//!   - Criminal prosecution for repeat offenders
//!   - Reforestation obligation
//!
//! - **Wildlife Trafficking** - Article 108
//!   - Fine: 5-20x species value
//!   - Imprisonment: 3 months to 5 years
//!   - Vehicle/equipment confiscation
//!
//! - **Forest Fire Prevention** - Article 109
//!   - Fire prevention obligations
//!   - Compensation for damages
//!   - Criminal liability for negligence
//!
//! - **Reforestation Requirements** - Article 110
//!   - Mandatory replanting ratios
//!   - 5-year maintenance obligation
//!   - Performance bond requirements
//!
//! ## Permits and Fees (ໃບອະນຸຍາດ ແລະ ຄ່າທຳນຽມ) - Articles 121-135
//!
//! **Article 121-130**: Permit types:
//!
//! - **Timber Transport Permit** - Article 122
//!   - Required for all timber movement
//!   - 30-day validity
//!   - Route specification required
//!
//! - **Sawmill License** - Article 123
//!   - Annual license required
//!   - Capacity restrictions
//!   - Log intake records
//!
//! - **Export Permit** - Article 124
//!   - Required for forest product exports
//!   - Species verification
//!   - CITES documentation for listed species
//!
//! - **Processing Facility License** - Article 125
//!   - Annual registration
//!   - Environmental compliance
//!   - Raw material tracking
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for forestry compliance
//! - **Comprehensive Coverage**: All major aspects of Forestry Law 2019
//! - **Builder Patterns**: Easy construction of complex permit and concession structures
//! - **Species Protection**: Built-in species classification and CITES compliance
//!
//! # Examples
//!
//! ## Validating Timber Harvesting
//!
//! ```rust
//! use legalis_la::forestry_law::*;
//!
//! // Validate harvesting permit for teak
//! let permit = TimberHarvestingPermitBuilder::new()
//!     .permit_number("THP-2026-001")
//!     .holder_name("Logging Company Ltd")
//!     .holder_name_lao("ບໍລິສັດຕັດໄມ້ ຈຳກັດ")
//!     .forest_type(ForestClassification::Production)
//!     .province("Savannakhet")
//!     .district("Sepon")
//!     .species(TreeSpecies::Teak)
//!     .volume_cubic_meters(500.0)
//!     .harvesting_month(12) // December - dry season
//!     .minimum_diameter_cm(45)
//!     .issue_date("2026-01-01")
//!     .expiry_date("2026-04-30")
//!     .build();
//!
//! match validate_timber_harvesting_permit(&permit) {
//!     Ok(()) => println!("Permit is valid"),
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```
//!
//! ## Checking Protected Species
//!
//! ```rust
//! use legalis_la::forestry_law::*;
//!
//! let species = TreeSpecies::Rosewood;
//! let category = species.protection_category();
//!
//! match category {
//!     ProtectionCategory::CategoryI => println!("Strictly protected - no harvest"),
//!     ProtectionCategory::CategoryII => println!("Managed - quota required"),
//!     ProtectionCategory::CategoryIII => println!("Common - standard permit"),
//! }
//! ```
//!
//! ## Creating Forest Concession
//!
//! ```rust
//! use legalis_la::forestry_law::*;
//!
//! let concession = ForestConcessionBuilder::new()
//!     .concession_number("FC-2026-001")
//!     .holder_name("Plantation Company Ltd")
//!     .concession_type(ConcessionType::Plantation)
//!     .area_hectares(5000.0)
//!     .term_years(40)
//!     .province("Attapeu")
//!     .performance_bond_lak(500_000_000)
//!     .has_eia(true)
//!     .has_management_plan(true)
//!     .build();
//!
//! match validate_forest_concession(&concession) {
//!     Ok(()) => println!("Concession is valid"),
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::forestry_law::*;
//!
//! let error = ForestryLawError::IllegalHarvesting {
//!     species: "Rosewood".to_string(),
//!     reason: "Category I species - no harvest permitted".to_string(),
//! };
//!
//! println!("English: {}", error.english_message());
//! println!("Lao: {}", error.lao_message());
//! ```
//!
//! # Compliance Notes
//!
//! When implementing forestry compliance in Laos:
//!
//! 1. **Harvesting Season**: Only harvest during dry season (November-April)
//! 2. **Species Restrictions**: Check protection category before any harvesting
//! 3. **Diameter Limits**: Respect minimum cutting diameters for all species
//! 4. **Log Tracking**: Maintain complete chain of custody documentation
//! 5. **Permits**: Ensure all required permits are obtained before operations
//! 6. **Community Rights**: Respect village forest boundaries and traditional uses
//! 7. **Reforestation**: Meet all replanting obligations for harvested areas
//! 8. **CITES**: Obtain proper documentation for any CITES-listed species
//!
//! # Related Laws
//!
//! - **Environmental Protection Law 2012** - Environmental impact requirements
//! - **Land Law 2019** - Forest land classification and rights
//! - **Wildlife and Aquatic Law 2007** - Protected wildlife in forests
//! - **Investment Promotion Law 2016** - Foreign investment in forestry

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{ForestryLawError, Result};
pub use types::{
    BenefitSharingArrangement,
    // Chain of Custody
    ChainOfCustodyEntry,
    // Community Forestry
    CommunityForestEnterprise,
    ConcessionStatus,
    ConcessionType,
    DISTRICT_BENEFIT_SHARE_PERCENT,
    ExportProductType,
    // Forest Classifications
    ForestClassification,
    // Forest Concessions
    ForestConcession,
    ForestConcessionBuilder,
    // Export
    ForestProductExportPermit,
    ForestProductExportPermitBuilder,
    // Penalties
    ForestryViolation,
    HARVESTING_SEASON_END_MONTH,
    // Constants
    HARVESTING_SEASON_START_MONTH,
    // Log Tracking
    LogEntry,
    LogEntryBuilder,
    MANAGEMENT_CONCESSION_BOND_PERCENT,
    MAX_MANAGEMENT_CONCESSION_HECTARES,
    MAX_MANAGEMENT_CONCESSION_YEARS,
    MAX_PLANTATION_CONCESSION_HECTARES,
    MAX_PLANTATION_CONCESSION_YEARS,
    MIN_DIAMETER_HARDWOOD_CM,
    MIN_DIAMETER_ROSEWOOD_CM,
    MIN_DIAMETER_TEAK_CM,
    NATIONAL_BENEFIT_SHARE_PERCENT,
    // NTFP
    NtfpPermit,
    NtfpPermitBuilder,
    NtfpType,
    PLANTATION_CONCESSION_BOND_PERCENT,
    PenaltyAssessment,
    // Permit Status
    PermitStatus,
    ProcessingFacilityLicense,
    ProtectionCategory,
    REFORESTATION_MAINTENANCE_YEARS,
    // Sawmill and Processing
    SawmillLicense,
    SawmillLicenseBuilder,
    TRANSPORT_PERMIT_VALIDITY_DAYS,
    // Timber Harvesting
    TimberHarvestingPermit,
    TimberHarvestingPermitBuilder,
    TransportPermit,
    TransportPermitBuilder,
    // Tree Species
    TreeSpecies,
    VILLAGE_BENEFIT_SHARE_PERCENT,
    // Village Forests
    VillageForest,
    VillageForestAgreement,
    VillageForestBuilder,
    // Violation types
    ViolationStatus,
    ViolationType,
};
pub use validator::{
    calculate_penalty,
    validate_benefit_sharing,
    validate_chain_of_custody,
    validate_cites_compliance,
    validate_concession_area,
    validate_concession_term,
    validate_export_permit,
    // Concession Validators
    validate_forest_concession,
    // Comprehensive Validators
    validate_forestry_compliance,
    // Penalty Validators
    validate_forestry_violation,
    validate_harvesting_season,
    // Log Tracking Validators
    validate_log_entry,
    validate_minimum_diameter,
    // NTFP Validators
    validate_ntfp_permit,
    validate_ntfp_sustainable_harvest,
    validate_performance_bond,
    validate_processing_facility_license,
    // License Validators
    validate_sawmill_license,
    validate_species_protection,
    // Timber Harvesting Validators
    validate_timber_harvesting_permit,
    validate_transport_permit,
    // Village Forest Validators
    validate_village_forest,
    validate_village_forest_agreement,
};
