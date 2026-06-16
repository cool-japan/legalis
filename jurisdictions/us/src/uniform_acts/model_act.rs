//! Shared metadata for uniform / model acts.
//!
//! Every uniform act tracked by this module is a *model statute* drafted by a
//! national law-reform body and then offered to the individual states for
//! enactment. This module captures the metadata common to all of them
//! (drafting body, promulgation year, revisions) so that each act file
//! (`utc`, `upc`, `ullca`, `uaa`, ...) can expose a uniform `model_act()`
//! descriptor.

use serde::{Deserialize, Serialize};

/// The 50 states plus the District of Columbia, using two-letter postal codes.
///
/// This is the canonical jurisdiction set used across the uniform-act trackers
/// so that adoption percentages are computed against a consistent denominator
/// (51 jurisdictions).
pub const US_JURISDICTIONS: [&str; 51] = [
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY", "DC",
];

/// Body responsible for drafting a uniform or model act.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DraftingBody {
    /// Uniform Law Commission (formerly the National Conference of
    /// Commissioners on Uniform State Laws, NCCUSL).
    UniformLawCommission,

    /// The American Law Institute (ALI).
    AmericanLawInstitute,

    /// Jointly drafted by the Uniform Law Commission and the American Law
    /// Institute (for example, the Uniform Commercial Code).
    UlcAndAli,
}

impl DraftingBody {
    /// Full name of the drafting body.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::UniformLawCommission => "Uniform Law Commission",
            Self::AmericanLawInstitute => "American Law Institute",
            Self::UlcAndAli => "Uniform Law Commission and American Law Institute",
        }
    }

    /// Common abbreviation for the drafting body.
    #[must_use]
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::UniformLawCommission => "ULC",
            Self::AmericanLawInstitute => "ALI",
            Self::UlcAndAli => "ULC/ALI",
        }
    }
}

/// Metadata describing a uniform / model act.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelActMetadata {
    /// Short name / common abbreviation (for example, "UTC").
    pub short_name: String,

    /// Full official name (for example, "Uniform Trust Code").
    pub full_name: String,

    /// Body that drafted the act.
    pub drafting_body: DraftingBody,

    /// Year the act was first promulgated.
    pub promulgated_year: u16,

    /// Years in which the act received official revisions or amendments.
    pub revision_years: Vec<u16>,

    /// One-line description of the act's subject matter.
    pub summary: String,
}

impl ModelActMetadata {
    /// Create new model-act metadata.
    #[must_use]
    pub fn new(
        short_name: impl Into<String>,
        full_name: impl Into<String>,
        drafting_body: DraftingBody,
        promulgated_year: u16,
    ) -> Self {
        Self {
            short_name: short_name.into(),
            full_name: full_name.into(),
            drafting_body,
            promulgated_year,
            revision_years: vec![],
            summary: String::new(),
        }
    }

    /// Record the years in which the act was officially revised or amended.
    #[must_use]
    pub fn with_revisions(mut self, years: impl IntoIterator<Item = u16>) -> Self {
        self.revision_years = years.into_iter().collect();
        self
    }

    /// Set the one-line subject-matter summary.
    #[must_use]
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = summary.into();
        self
    }

    /// Year of the most recent official text (latest revision, or the
    /// promulgation year if the act has never been revised).
    #[must_use]
    pub fn latest_version_year(&self) -> u16 {
        self.revision_years
            .iter()
            .copied()
            .max()
            .unwrap_or(self.promulgated_year)
            .max(self.promulgated_year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_us_jurisdictions_count() {
        assert_eq!(US_JURISDICTIONS.len(), 51);
        assert!(US_JURISDICTIONS.contains(&"CA"));
        assert!(US_JURISDICTIONS.contains(&"DC"));
    }

    #[test]
    fn test_drafting_body_names() {
        assert_eq!(
            DraftingBody::UniformLawCommission.name(),
            "Uniform Law Commission"
        );
        assert_eq!(DraftingBody::UniformLawCommission.abbreviation(), "ULC");
        assert_eq!(DraftingBody::UlcAndAli.abbreviation(), "ULC/ALI");
    }

    #[test]
    fn test_metadata_builder_and_latest_version() {
        let meta = ModelActMetadata::new(
            "UTC",
            "Uniform Trust Code",
            DraftingBody::UniformLawCommission,
            2000,
        )
        .with_revisions([2001, 2003, 2004, 2005, 2010])
        .with_summary("National codification of trust law.");

        assert_eq!(meta.short_name, "UTC");
        assert_eq!(meta.promulgated_year, 2000);
        assert_eq!(meta.latest_version_year(), 2010);
        assert!(!meta.summary.is_empty());
    }

    #[test]
    fn test_latest_version_without_revisions() {
        let meta = ModelActMetadata::new(
            "UAA",
            "Uniform Arbitration Act",
            DraftingBody::UniformLawCommission,
            1955,
        );
        assert_eq!(meta.latest_version_year(), 1955);
    }
}
