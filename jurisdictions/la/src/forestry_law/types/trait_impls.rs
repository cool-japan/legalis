//! # BenefitSharingArrangement - Trait Implementations
//!
//! This module contains trait implementations for `BenefitSharingArrangement`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{
    DISTRICT_BENEFIT_SHARE_PERCENT, HARVESTING_SEASON_START_MONTH, MIN_DIAMETER_HARDWOOD_CM,
    NATIONAL_BENEFIT_SHARE_PERCENT, VILLAGE_BENEFIT_SHARE_PERCENT,
};
use super::permit_types::{
    BenefitSharingArrangement, ConcessionStatus, ExportProductType, ForestClassification,
    ForestConcession, ForestProductExportPermit, LogEntry, NtfpPermit, SawmillLicense,
    TimberHarvestingPermit, TransportPermit, TreeSpecies, VillageForest,
};
use super::types_3::{ConcessionType, NtfpType, PermitStatus};

impl Default for BenefitSharingArrangement {
    fn default() -> Self {
        Self {
            village_share_percent: VILLAGE_BENEFIT_SHARE_PERCENT,
            district_share_percent: DISTRICT_BENEFIT_SHARE_PERCENT,
            national_share_percent: NATIONAL_BENEFIT_SHARE_PERCENT,
            agreement_date: String::new(),
            validity_years: 5,
        }
    }
}

impl Default for ForestConcession {
    fn default() -> Self {
        Self {
            concession_number: String::new(),
            holder_name: String::new(),
            holder_name_lao: None,
            concession_type: ConcessionType::Plantation,
            area_hectares: 0.0,
            term_years: 0,
            province: String::new(),
            districts: Vec::new(),
            start_date: String::new(),
            end_date: String::new(),
            performance_bond_lak: 0,
            project_value_lak: None,
            has_eia: false,
            has_management_plan: false,
            status: ConcessionStatus::ApplicationPending,
            primary_species: Vec::new(),
            annual_production_quota_m3: None,
            reforestation_commitment_hectares: None,
            community_agreements: Vec::new(),
        }
    }
}

impl Default for ForestProductExportPermit {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            exporter_name: String::new(),
            exporter_name_lao: None,
            product_type: ExportProductType::SawnTimber,
            species: None,
            quantity: 0.0,
            quantity_unit: "m³".to_string(),
            value_usd: None,
            destination_country: String::new(),
            issue_date: String::new(),
            expiry_date: String::new(),
            status: PermitStatus::Pending,
            cites_permit_number: None,
            phytosanitary_certificate: None,
            origin_certificate: None,
            source_permits: Vec::new(),
        }
    }
}

impl Default for LogEntry {
    fn default() -> Self {
        Self {
            log_id: String::new(),
            species: TreeSpecies::OtherHardwood,
            length_meters: 0.0,
            diameter_cm: 0,
            volume_cubic_meters: 0.0,
            harvest_permit_reference: String::new(),
            harvest_date: String::new(),
            harvest_province: String::new(),
            harvest_district: String::new(),
            current_location: String::new(),
            chain_of_custody: Vec::new(),
            is_cites_listed: false,
            quality_grade: None,
        }
    }
}

impl Default for NtfpPermit {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            holder_name: String::new(),
            holder_name_lao: None,
            ntfp_type: NtfpType::Other,
            province: String::new(),
            district: String::new(),
            village: None,
            quantity_allowed: 0.0,
            quantity_unit: "kg".to_string(),
            issue_date: String::new(),
            expiry_date: String::new(),
            status: PermitStatus::Pending,
            commercial_use: false,
            fee_paid_lak: None,
        }
    }
}

impl Default for SawmillLicense {
    fn default() -> Self {
        Self {
            license_number: String::new(),
            facility_name: String::new(),
            facility_name_lao: None,
            owner_name: String::new(),
            province: String::new(),
            district: String::new(),
            annual_capacity_cubic_meters: 0.0,
            issue_date: String::new(),
            expiry_date: String::new(),
            status: PermitStatus::Pending,
            environmental_compliance: false,
            has_log_tracking: false,
            permitted_species: Vec::new(),
        }
    }
}

impl Default for TimberHarvestingPermit {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            holder_name: String::new(),
            holder_name_lao: None,
            forest_type: ForestClassification::Production,
            province: String::new(),
            district: String::new(),
            village: None,
            species: TreeSpecies::OtherHardwood,
            volume_cubic_meters: 0.0,
            tree_count: None,
            harvesting_month: HARVESTING_SEASON_START_MONTH,
            harvesting_year: 2026,
            minimum_diameter_cm: MIN_DIAMETER_HARDWOOD_CM,
            issue_date: String::new(),
            expiry_date: String::new(),
            issuing_authority: String::new(),
            aac_allocation: None,
            quota_reference: None,
            status: PermitStatus::Pending,
            reforestation_required: true,
            reforestation_area_hectares: None,
        }
    }
}

impl Default for TransportPermit {
    fn default() -> Self {
        Self {
            permit_number: String::new(),
            holder_name: String::new(),
            origin_province: String::new(),
            origin_district: String::new(),
            destination_province: String::new(),
            destination_district: String::new(),
            destination_facility: None,
            species: TreeSpecies::OtherHardwood,
            volume_cubic_meters: 0.0,
            log_count: 0,
            vehicle_registration: String::new(),
            issue_date: String::new(),
            expiry_date: String::new(),
            specified_route: None,
            status: PermitStatus::Pending,
            harvest_permit_reference: String::new(),
        }
    }
}

impl Default for VillageForest {
    fn default() -> Self {
        Self {
            village_name: String::new(),
            village_name_lao: String::new(),
            district: String::new(),
            province: String::new(),
            area_hectares: 0.0,
            registration_date: String::new(),
            has_management_agreement: false,
            agreement_expiry: None,
            household_count: 0,
            key_species: Vec::new(),
            traditional_uses: Vec::new(),
            has_community_enterprise: false,
        }
    }
}
