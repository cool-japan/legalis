//! PDPA-regulated organisations and the Data Protection Officer (s. 11).
//!
//! Under the **Accountability Obligation** (s. 11), every organisation **must**
//! designate at least one individual (the "Data Protection Officer", DPO) to be
//! responsible for ensuring the organisation's compliance with the Act
//! (s. 11(3)), and **must make available to the public** the business contact
//! information of at least one such individual (s. 11(5)).
//!
//! The designation is therefore *mandatory*, not advisory. Separately, the PDPC
//! publishes guidance on the **scale of resourcing** a DPO function should have;
//! we model that as a non-binding [`DpoStaffingRecommendation`] keyed on the
//! organisation's data-handling profile.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Maximum financial penalty (in SGD) that may be imposed on an organisation
/// whose annual turnover in Singapore does **not** exceed
/// [`HIGH_TURNOVER_THRESHOLD_SGD`] (PDPA s. 48J(3)(b), in force 1 October 2022).
pub const MAX_PENALTY_SGD: u64 = 1_000_000;

/// Annual Singapore turnover (in SGD) above which the maximum penalty is instead
/// 10% of that turnover (PDPA s. 48J(3)(a)).
pub const HIGH_TURNOVER_THRESHOLD_SGD: u64 = 10_000_000;

/// Percentage of annual Singapore turnover used as the penalty cap for
/// high-turnover organisations (PDPA s. 48J(3)(a)).
pub const HIGH_TURNOVER_PENALTY_PERCENT: u64 = 10;

/// Computes the maximum financial penalty (in SGD) for an organisation given its
/// annual Singapore turnover (PDPA s. 48J(3)).
///
/// For an organisation whose annual turnover in Singapore exceeds
/// [`HIGH_TURNOVER_THRESHOLD_SGD`] (SGD 10 million), the cap is 10% of that
/// turnover; otherwise it is [`MAX_PENALTY_SGD`] (SGD 1 million). In effect the
/// cap is the higher of SGD 1 million and 10% of Singapore turnover.
pub fn max_financial_penalty_sgd(annual_sg_turnover_sgd: u64) -> u64 {
    if annual_sg_turnover_sgd > HIGH_TURNOVER_THRESHOLD_SGD {
        annual_sg_turnover_sgd / 100 * HIGH_TURNOVER_PENALTY_PERCENT
    } else {
        MAX_PENALTY_SGD
    }
}

/// Type of PDPA-regulated organisation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganisationType {
    /// Private-sector organisation (company, partnership, etc.).
    Private,
    /// Public agency. Note that public agencies are largely governed by the
    /// Public Sector (Governance) Act rather than the PDPA's Data Protection
    /// Provisions (s. 4(1)(c)), but the obligation to designate a DPO is still
    /// modelled here for completeness.
    PublicAgency,
    /// Non-profit organisation / charity.
    NonProfit,
}

/// Business contact information of the designated DPO, which must be made
/// available to the public (s. 11(5)).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DpoContact {
    /// Name or title of the DPO (a title such as "Data Protection Officer" is
    /// acceptable; an individual need not be personally named).
    pub name: String,
    /// Business e-mail address.
    pub email: String,
    /// Business telephone number.
    pub phone: String,
    /// Date of designation (s. 11(3)).
    pub designated_date: DateTime<Utc>,
    /// Whether the DPO's business contact information has actually been made
    /// available to the public (s. 11(5)).
    pub published_to_public: bool,
}

impl DpoContact {
    /// Creates DPO contact details, designated now and not yet published.
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        phone: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            phone: phone.into(),
            designated_date: Utc::now(),
            published_to_public: false,
        }
    }

    /// Marks the DPO's business contact information as published (s. 11(5)).
    pub fn publish(&mut self) -> &mut Self {
        self.published_to_public = true;
        self
    }
}

/// Non-binding recommendation on how a DPO function should be resourced, based on
/// the organisation's data-handling profile. The *designation* of a DPO is
/// always mandatory (s. 11(3)); this only advises on scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DpoStaffingRecommendation {
    /// A single designated individual, who may hold the role alongside other
    /// duties, is sufficient.
    SingleDesignatedIndividual,
    /// A dedicated DPO (and supporting processes) is recommended given the
    /// volume or sensitivity of personal data handled.
    DedicatedDpo,
    /// A full data-protection team / office is recommended given large-scale
    /// or high-sensitivity processing.
    DataProtectionOffice,
}

/// A PDPA-regulated organisation and its accountability posture (s. 11-12).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdpaOrganisation {
    /// Organisation name.
    pub name: String,
    /// Unique Entity Number, if any.
    pub uen: Option<String>,
    /// Organisation type.
    pub organisation_type: OrganisationType,
    /// Designated DPO contact (s. 11(3)/(5)). `None` means no DPO has been
    /// designated, which contravenes the mandatory s. 11(3) duty.
    pub dpo_contact: Option<DpoContact>,
    /// URL of the organisation's published data protection / privacy policy
    /// (s. 12 — develop and implement policies and practices).
    pub privacy_policy_url: Option<String>,
    /// Approximate number of individuals whose personal data the organisation
    /// handles (used only for the non-binding staffing recommendation).
    pub data_subjects_handled: u64,
    /// Whether the organisation routinely handles sensitive data (financial or
    /// health), which raises the recommended staffing level.
    pub handles_sensitive_data: bool,
}

impl PdpaOrganisation {
    /// Creates an organisation with no DPO designated yet.
    pub fn new(name: impl Into<String>, organisation_type: OrganisationType) -> Self {
        Self {
            name: name.into(),
            uen: None,
            organisation_type,
            dpo_contact: None,
            privacy_policy_url: None,
            data_subjects_handled: 0,
            handles_sensitive_data: false,
        }
    }

    /// Designates a DPO (s. 11(3)).
    pub fn with_dpo(mut self, dpo: DpoContact) -> Self {
        self.dpo_contact = Some(dpo);
        self
    }

    /// Sets the UEN.
    pub fn with_uen(mut self, uen: impl Into<String>) -> Self {
        self.uen = Some(uen.into());
        self
    }

    /// Sets the published privacy policy URL.
    pub fn with_privacy_policy(mut self, url: impl Into<String>) -> Self {
        self.privacy_policy_url = Some(url.into());
        self
    }

    /// Sets the data-handling profile used for the staffing recommendation.
    pub fn with_data_profile(mut self, data_subjects: u64, handles_sensitive: bool) -> Self {
        self.data_subjects_handled = data_subjects;
        self.handles_sensitive_data = handles_sensitive;
        self
    }

    /// Returns `true` if the organisation has designated at least one DPO, as
    /// required by the mandatory duty in s. 11(3).
    pub fn has_designated_dpo(&self) -> bool {
        self.dpo_contact.is_some()
    }

    /// Returns the non-binding [`DpoStaffingRecommendation`] for this
    /// organisation based on its data-handling profile.
    ///
    /// Thresholds (advisory only): an organisation handling sensitive data or
    /// the personal data of 50,000+ individuals should have a full data
    /// protection office; one handling 5,000+ individuals (or any sensitive
    /// data) should have a dedicated DPO; otherwise a single designated
    /// individual suffices.
    pub fn dpo_staffing_recommendation(&self) -> DpoStaffingRecommendation {
        const OFFICE_THRESHOLD: u64 = 50_000;
        const DEDICATED_THRESHOLD: u64 = 5_000;
        if self.handles_sensitive_data && self.data_subjects_handled >= OFFICE_THRESHOLD {
            DpoStaffingRecommendation::DataProtectionOffice
        } else if self.handles_sensitive_data || self.data_subjects_handled >= DEDICATED_THRESHOLD {
            DpoStaffingRecommendation::DedicatedDpo
        } else {
            DpoStaffingRecommendation::SingleDesignatedIndividual
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn penalty_cap_low_turnover_is_one_million() {
        assert_eq!(max_financial_penalty_sgd(0), 1_000_000);
        assert_eq!(max_financial_penalty_sgd(10_000_000), 1_000_000);
    }

    #[test]
    fn penalty_cap_high_turnover_is_ten_percent() {
        // Turnover SGD 50m -> 10% = SGD 5m (exceeds the SGD 1m floor).
        assert_eq!(max_financial_penalty_sgd(50_000_000), 5_000_000);
        // Just over the threshold: SGD 10,000,001 -> 10% = SGD 1,000,000.
        assert_eq!(max_financial_penalty_sgd(10_000_001), 1_000_000);
    }

    #[test]
    fn dpo_designation_is_tracked() {
        let org = PdpaOrganisation::new("Acme Pte Ltd", OrganisationType::Private);
        assert!(!org.has_designated_dpo());
        let org = org.with_dpo(DpoContact::new("DPO", "dpo@acme.sg", "+6561234567"));
        assert!(org.has_designated_dpo());
    }

    #[test]
    fn staffing_recommendation_scales() {
        let small =
            PdpaOrganisation::new("Small", OrganisationType::Private).with_data_profile(100, false);
        assert_eq!(
            small.dpo_staffing_recommendation(),
            DpoStaffingRecommendation::SingleDesignatedIndividual
        );

        let medium = PdpaOrganisation::new("Medium", OrganisationType::Private)
            .with_data_profile(10_000, false);
        assert_eq!(
            medium.dpo_staffing_recommendation(),
            DpoStaffingRecommendation::DedicatedDpo
        );

        let large = PdpaOrganisation::new("Large", OrganisationType::Private)
            .with_data_profile(100_000, true);
        assert_eq!(
            large.dpo_staffing_recommendation(),
            DpoStaffingRecommendation::DataProtectionOffice
        );

        // Sensitive data alone bumps a small org to a dedicated DPO.
        let sensitive =
            PdpaOrganisation::new("Clinic", OrganisationType::Private).with_data_profile(200, true);
        assert_eq!(
            sensitive.dpo_staffing_recommendation(),
            DpoStaffingRecommendation::DedicatedDpo
        );
    }
}
