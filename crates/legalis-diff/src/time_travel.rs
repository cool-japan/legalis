//! Time-travel diffing for point-in-time statute analysis.
//!
//! This module provides functionality for:
//! - Point-in-time statute reconstruction
//! - Temporal diff queries
//! - Effective date-aware comparisons
//! - Sunset clause tracking
//! - Amendment chain visualization

use crate::{DiffError, DiffResult, StatuteDiff, diff};
use chrono::{DateTime, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A statute with temporal validity information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalStatute {
    /// The statute content.
    pub statute: Statute,
    /// When this version became effective.
    pub effective_date: DateTime<Utc>,
    /// When this version was superseded (None if still current).
    pub superseded_date: Option<DateTime<Utc>>,
    /// Optional sunset clause (automatic expiration).
    pub sunset_date: Option<DateTime<Utc>>,
    /// Amendment identifier.
    pub amendment_id: String,
}

impl TemporalStatute {
    /// Creates a new temporal statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::time_travel::TemporalStatute;
    /// use chrono::Utc;
    ///
    /// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let temporal = TemporalStatute::new(statute, Utc::now(), "v1.0");
    ///
    /// assert!(temporal.is_active_at(Utc::now()));
    /// ```
    pub fn new(statute: Statute, effective_date: DateTime<Utc>, amendment_id: &str) -> Self {
        Self {
            statute,
            effective_date,
            superseded_date: None,
            sunset_date: None,
            amendment_id: amendment_id.to_string(),
        }
    }

    /// Sets the sunset date for automatic expiration.
    pub fn with_sunset(mut self, sunset_date: DateTime<Utc>) -> Self {
        self.sunset_date = Some(sunset_date);
        self
    }

    /// Marks this statute version as superseded.
    pub fn supersede(&mut self, superseded_date: DateTime<Utc>) {
        self.superseded_date = Some(superseded_date);
    }

    /// Checks if this statute version is active at a given time.
    pub fn is_active_at(&self, time: DateTime<Utc>) -> bool {
        // Must be after effective date
        if time < self.effective_date {
            return false;
        }

        // Must be before superseded date (if any)
        if let Some(superseded) = self.superseded_date
            && time >= superseded
        {
            return false;
        }

        // Must be before sunset date (if any)
        if let Some(sunset) = self.sunset_date
            && time >= sunset
        {
            return false;
        }

        true
    }

    /// Gets the end date of this version's validity.
    pub fn end_date(&self) -> Option<DateTime<Utc>> {
        match (self.superseded_date, self.sunset_date) {
            (Some(superseded), Some(sunset)) => Some(superseded.min(sunset)),
            (Some(superseded), None) => Some(superseded),
            (None, Some(sunset)) => Some(sunset),
            (None, None) => None,
        }
    }
}

/// A repository of temporal statutes for time-travel diffing.
#[derive(Debug, Clone)]
pub struct TemporalRepository {
    /// Statute versions indexed by statute ID.
    versions: HashMap<String, Vec<TemporalStatute>>,
}

impl TemporalRepository {
    /// Creates a new empty temporal repository.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::time_travel::TemporalRepository;
    ///
    /// let repo = TemporalRepository::new();
    /// assert_eq!(repo.statute_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    /// Adds a temporal statute to the repository.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::time_travel::{TemporalRepository, TemporalStatute};
    /// use chrono::Utc;
    ///
    /// let mut repo = TemporalRepository::new();
    /// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let temporal = TemporalStatute::new(statute, Utc::now(), "v1.0");
    ///
    /// repo.add(temporal);
    /// assert_eq!(repo.statute_count(), 1);
    /// ```
    pub fn add(&mut self, temporal: TemporalStatute) {
        let statute_id = temporal.statute.id.clone();
        let versions = self.versions.entry(statute_id).or_default();

        // Keep versions sorted by effective date
        versions.push(temporal);
        versions.sort_by_key(|v| v.effective_date);
    }

    /// Gets the statute version that was active at a specific point in time.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::time_travel::{TemporalRepository, TemporalStatute};
    /// use chrono::Utc;
    ///
    /// let mut repo = TemporalRepository::new();
    /// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let now = Utc::now();
    /// let temporal = TemporalStatute::new(statute, now, "v1.0");
    ///
    /// repo.add(temporal);
    /// let result = repo.at_time("law", now);
    /// assert!(result.is_some());
    /// ```
    pub fn at_time(&self, statute_id: &str, time: DateTime<Utc>) -> Option<&TemporalStatute> {
        self.versions.get(statute_id).and_then(|versions| {
            versions.iter().rfind(|v| v.is_active_at(time)) // Get the most recent version active at that time
        })
    }

    /// Gets all versions of a statute.
    pub fn all_versions(&self, statute_id: &str) -> Option<&[TemporalStatute]> {
        self.versions.get(statute_id).map(|v| v.as_slice())
    }

    /// Gets the current (most recent) version of a statute.
    pub fn current(&self, statute_id: &str) -> Option<&TemporalStatute> {
        self.versions
            .get(statute_id)
            .and_then(|versions| versions.last())
    }

    /// Gets the number of statutes in the repository.
    pub fn statute_count(&self) -> usize {
        self.versions.len()
    }

    /// Gets the total number of versions across all statutes.
    pub fn version_count(&self) -> usize {
        self.versions.values().map(|v| v.len()).sum()
    }
}

impl Default for TemporalRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// A diff between two points in time for a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDiff {
    /// The statute ID.
    pub statute_id: String,
    /// The time of the "old" version.
    pub from_time: DateTime<Utc>,
    /// The time of the "new" version.
    pub to_time: DateTime<Utc>,
    /// The amendment IDs for the versions.
    pub from_amendment: String,
    pub to_amendment: String,
    /// The actual diff.
    pub diff: StatuteDiff,
}

/// Computes a diff between two points in time.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::time_travel::{TemporalRepository, TemporalStatute, diff_at_times};
/// use chrono::{Utc, Duration};
///
/// let mut repo = TemporalRepository::new();
///
/// let statute1 = Statute::new("law", "Old Title", Effect::new(EffectType::Grant, "Benefit"));
/// let time1 = Utc::now() - Duration::days(10);
/// let temporal1 = TemporalStatute::new(statute1, time1, "v1.0");
/// repo.add(temporal1);
///
/// let statute2 = Statute::new("law", "New Title", Effect::new(EffectType::Grant, "Benefit"));
/// let time2 = Utc::now();
/// let temporal2 = TemporalStatute::new(statute2, time2, "v2.0");
/// repo.add(temporal2);
///
/// let result = diff_at_times(&repo, "law", time1, time2);
/// assert!(result.is_ok());
/// ```
pub fn diff_at_times(
    repo: &TemporalRepository,
    statute_id: &str,
    from_time: DateTime<Utc>,
    to_time: DateTime<Utc>,
) -> DiffResult<TemporalDiff> {
    let from_version = repo.at_time(statute_id, from_time).ok_or_else(|| {
        DiffError::InvalidComparison(format!("No statute version at {:?}", from_time))
    })?;

    let to_version = repo.at_time(statute_id, to_time).ok_or_else(|| {
        DiffError::InvalidComparison(format!("No statute version at {:?}", to_time))
    })?;

    let diff_result = diff(&from_version.statute, &to_version.statute)?;

    Ok(TemporalDiff {
        statute_id: statute_id.to_string(),
        from_time,
        to_time,
        from_amendment: from_version.amendment_id.clone(),
        to_amendment: to_version.amendment_id.clone(),
        diff: diff_result,
    })
}

/// Amendment chain representing the evolution of a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendmentChain {
    /// The statute ID.
    pub statute_id: String,
    /// The chain of amendments in chronological order.
    pub amendments: Vec<AmendmentNode>,
}

/// A node in the amendment chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendmentNode {
    /// Amendment identifier.
    pub amendment_id: String,
    /// When it became effective.
    pub effective_date: DateTime<Utc>,
    /// When it was superseded (if applicable).
    pub superseded_date: Option<DateTime<Utc>>,
    /// Sunset date (if applicable).
    pub sunset_date: Option<DateTime<Utc>>,
    /// Changes made in this amendment.
    pub changes_summary: String,
}

/// Generates an amendment chain for a statute.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::time_travel::{TemporalRepository, TemporalStatute, generate_amendment_chain};
/// use chrono::{Utc, Duration};
///
/// let mut repo = TemporalRepository::new();
///
/// let statute1 = Statute::new("law", "V1", Effect::new(EffectType::Grant, "Benefit"));
/// let time1 = Utc::now() - Duration::days(20);
/// repo.add(TemporalStatute::new(statute1, time1, "v1.0"));
///
/// let statute2 = Statute::new("law", "V2", Effect::new(EffectType::Grant, "Benefit"));
/// let time2 = Utc::now() - Duration::days(10);
/// repo.add(TemporalStatute::new(statute2, time2, "v2.0"));
///
/// let chain = generate_amendment_chain(&repo, "law").unwrap();
/// assert_eq!(chain.amendments.len(), 2);
/// ```
pub fn generate_amendment_chain(
    repo: &TemporalRepository,
    statute_id: &str,
) -> DiffResult<AmendmentChain> {
    let versions = repo.all_versions(statute_id).ok_or_else(|| {
        DiffError::InvalidComparison(format!("No statute with ID: {}", statute_id))
    })?;

    let mut amendments = Vec::new();

    for (i, version) in versions.iter().enumerate() {
        let changes_summary = if i == 0 {
            "Initial version".to_string()
        } else {
            let prev = &versions[i - 1];
            let diff_result = diff(&prev.statute, &version.statute)?;
            format!("{} changes", diff_result.changes.len())
        };

        amendments.push(AmendmentNode {
            amendment_id: version.amendment_id.clone(),
            effective_date: version.effective_date,
            superseded_date: version.superseded_date,
            sunset_date: version.sunset_date,
            changes_summary,
        });
    }

    Ok(AmendmentChain {
        statute_id: statute_id.to_string(),
        amendments,
    })
}

/// Sunset clause tracking information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunsetTracking {
    /// Statutes with active sunset clauses.
    pub active_sunsets: Vec<SunsetInfo>,
    /// Statutes that have already sunset.
    pub expired_sunsets: Vec<SunsetInfo>,
}

/// Information about a sunset clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunsetInfo {
    pub statute_id: String,
    pub amendment_id: String,
    pub effective_date: DateTime<Utc>,
    pub sunset_date: DateTime<Utc>,
    pub days_until_sunset: Option<i64>,
}

/// Tracks all sunset clauses in a repository.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::time_travel::{TemporalRepository, TemporalStatute, track_sunsets};
/// use chrono::{Utc, Duration};
///
/// let mut repo = TemporalRepository::new();
///
/// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let now = Utc::now();
/// let sunset = now + Duration::days(30);
/// let temporal = TemporalStatute::new(statute, now, "v1.0").with_sunset(sunset);
///
/// repo.add(temporal);
/// let tracking = track_sunsets(&repo, now);
///
/// assert_eq!(tracking.active_sunsets.len(), 1);
/// ```
pub fn track_sunsets(repo: &TemporalRepository, current_time: DateTime<Utc>) -> SunsetTracking {
    let mut active_sunsets = Vec::new();
    let mut expired_sunsets = Vec::new();

    for versions in repo.versions.values() {
        for version in versions {
            if let Some(sunset_date) = version.sunset_date {
                let days_until = (sunset_date - current_time).num_days();

                let info = SunsetInfo {
                    statute_id: version.statute.id.clone(),
                    amendment_id: version.amendment_id.clone(),
                    effective_date: version.effective_date,
                    sunset_date,
                    days_until_sunset: if sunset_date > current_time {
                        Some(days_until)
                    } else {
                        None
                    },
                };

                if sunset_date > current_time {
                    active_sunsets.push(info);
                } else {
                    expired_sunsets.push(info);
                }
            }
        }
    }

    // Sort by sunset date
    active_sunsets.sort_by_key(|s| s.sunset_date);
    expired_sunsets.sort_by_key(|s| s.sunset_date);

    SunsetTracking {
        active_sunsets,
        expired_sunsets,
    }
}

/// Reconstructs a statute at a specific point in time.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::time_travel::{TemporalRepository, TemporalStatute, reconstruct_at_time};
/// use chrono::Utc;
///
/// let mut repo = TemporalRepository::new();
/// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let now = Utc::now();
/// let temporal = TemporalStatute::new(statute, now, "v1.0");
///
/// repo.add(temporal);
/// let reconstructed = reconstruct_at_time(&repo, "law", now);
/// assert!(reconstructed.is_some());
/// ```
pub fn reconstruct_at_time(
    repo: &TemporalRepository,
    statute_id: &str,
    time: DateTime<Utc>,
) -> Option<Statute> {
    repo.at_time(statute_id, time)
        .map(|temporal| temporal.statute.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use legalis_core::{Effect, EffectType};

    fn test_statute(title: &str) -> Statute {
        Statute::new("test", title, Effect::new(EffectType::Grant, "Benefit"))
    }

    #[test]
    fn test_temporal_statute_active() {
        let statute = test_statute("Test");
        let now = Utc::now();
        let temporal = TemporalStatute::new(statute, now, "v1.0");

        assert!(temporal.is_active_at(now));
        assert!(temporal.is_active_at(now + Duration::days(1)));
        assert!(!temporal.is_active_at(now - Duration::days(1)));
    }

    #[test]
    fn test_temporal_statute_sunset() {
        let statute = test_statute("Test");
        let now = Utc::now();
        let sunset = now + Duration::days(30);
        let temporal = TemporalStatute::new(statute, now, "v1.0").with_sunset(sunset);

        assert!(temporal.is_active_at(now + Duration::days(15)));
        assert!(!temporal.is_active_at(sunset + Duration::days(1)));
    }

    #[test]
    fn test_repository_add_and_retrieve() {
        let mut repo = TemporalRepository::new();
        let statute = test_statute("V1");
        let now = Utc::now();
        let temporal = TemporalStatute::new(statute, now, "v1.0");

        repo.add(temporal);

        assert_eq!(repo.statute_count(), 1);
        assert_eq!(repo.version_count(), 1);

        let retrieved = repo.at_time("test", now);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_repository_multiple_versions() {
        let mut repo = TemporalRepository::new();
        let now = Utc::now();

        let statute1 = test_statute("V1");
        let time1 = now - Duration::days(20);
        repo.add(TemporalStatute::new(statute1, time1, "v1.0"));

        let statute2 = test_statute("V2");
        let time2 = now - Duration::days(10);
        repo.add(TemporalStatute::new(statute2, time2, "v2.0"));

        assert_eq!(repo.statute_count(), 1);
        assert_eq!(repo.version_count(), 2);

        let old_version = repo.at_time("test", time1);
        assert!(old_version.is_some());
        assert_eq!(old_version.unwrap().statute.title, "V1");

        let new_version = repo.at_time("test", now);
        assert!(new_version.is_some());
        assert_eq!(new_version.unwrap().statute.title, "V2");
    }

    #[test]
    fn test_diff_at_times() {
        let mut repo = TemporalRepository::new();
        let now = Utc::now();

        let statute1 = test_statute("Old");
        let time1 = now - Duration::days(10);
        repo.add(TemporalStatute::new(statute1, time1, "v1.0"));

        let statute2 = test_statute("New");
        let time2 = now;
        repo.add(TemporalStatute::new(statute2, time2, "v2.0"));

        let result = diff_at_times(&repo, "test", time1, time2);
        assert!(result.is_ok());

        let temporal_diff = result.unwrap();
        assert_eq!(temporal_diff.from_amendment, "v1.0");
        assert_eq!(temporal_diff.to_amendment, "v2.0");
        assert!(!temporal_diff.diff.changes.is_empty());
    }

    #[test]
    fn test_amendment_chain() {
        let mut repo = TemporalRepository::new();
        let now = Utc::now();

        repo.add(TemporalStatute::new(
            test_statute("V1"),
            now - Duration::days(20),
            "v1.0",
        ));
        repo.add(TemporalStatute::new(
            test_statute("V2"),
            now - Duration::days(10),
            "v2.0",
        ));
        repo.add(TemporalStatute::new(test_statute("V3"), now, "v3.0"));

        let chain = generate_amendment_chain(&repo, "test").unwrap();
        assert_eq!(chain.amendments.len(), 3);
        assert_eq!(chain.amendments[0].amendment_id, "v1.0");
        assert_eq!(chain.amendments[1].amendment_id, "v2.0");
        assert_eq!(chain.amendments[2].amendment_id, "v3.0");
    }

    #[test]
    fn test_sunset_tracking() {
        let mut repo = TemporalRepository::new();
        let now = Utc::now();

        let future_sunset = now + Duration::days(30);
        let past_sunset = now - Duration::days(30);

        let statute1 = test_statute("Active");
        repo.add(
            TemporalStatute::new(statute1, now - Duration::days(60), "v1.0")
                .with_sunset(future_sunset),
        );

        let statute2 = test_statute("Expired");
        repo.add(
            TemporalStatute::new(statute2, now - Duration::days(90), "v2.0")
                .with_sunset(past_sunset),
        );

        let tracking = track_sunsets(&repo, now);
        assert_eq!(tracking.active_sunsets.len(), 1);
        assert_eq!(tracking.expired_sunsets.len(), 1);
    }

    #[test]
    fn test_reconstruct_at_time() {
        let mut repo = TemporalRepository::new();
        let now = Utc::now();

        let statute = test_statute("Version 1");
        let time = now - Duration::days(10);
        repo.add(TemporalStatute::new(statute, time, "v1.0"));

        let reconstructed = reconstruct_at_time(&repo, "test", time);
        assert!(reconstructed.is_some());
        assert_eq!(reconstructed.unwrap().title, "Version 1");
    }
}
