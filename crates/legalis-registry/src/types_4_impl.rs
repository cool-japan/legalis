//! Additional impl blocks for StatuteRegistry (split from types_4.rs).

use chrono::{DateTime, Utc};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
// serde_json used via fully-qualified paths below
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::functions::RegistryResult;
use super::types::{DuplicateCandidate, EnrichmentType, FieldProfile, HealthStatus, RegistryError};
#[cfg(feature = "yaml")]
use super::types_3::BackupMetadata;
use super::types_3::{
    BulkConfig, DataProfile, RegistryDifference, RegistryEvent, StatuteDifferenceDetail,
};
use super::types_4::StatuteRegistry;
#[cfg(feature = "yaml")]
use super::types_5::RegistryBackup;
use super::types_5::{
    ComplianceDashboard, DataRetentionConfig, DataRetentionRule, LineageEntry, PiiDetector,
    QualityAssessment, StatuteDiff,
};
use super::types_6::{AuditReportConfig, StatuteEntry, StatuteStatus};
use super::types_7::{
    BulkOperationResult, DataSovereigntyConfig, DuplicateDetectionResult, EnrichmentResult,
    EnrichmentSuggestion, GeographicRegion, HealthCheckResult, OperationMetrics,
    RetentionExecutionResult, SimilarityScore,
};
#[cfg(feature = "csv-export")]
use super::types_8::StatuteSummary;
use super::types_8::{AuditReport, ComponentHealth, EnrichmentConfig, PiiScanResult, QualityScore};

impl StatuteRegistry {
    /// Computes the difference between two versions of a statute.
    ///
    /// # Errors
    ///
    /// Returns an error if either version is not found.
    pub fn diff(
        &self,
        statute_id: &str,
        old_version: u32,
        new_version: u32,
    ) -> RegistryResult<StatuteDiff> {
        let old = self.get_version(statute_id, old_version)?;
        let new = self.get_version(statute_id, new_version)?;
        Ok(StatuteDiff::compute(old, new))
    }
    /// Computes the difference between a version and the latest version.
    pub fn diff_with_latest(
        &self,
        statute_id: &str,
        old_version: u32,
    ) -> RegistryResult<StatuteDiff> {
        let latest_version = self
            .latest_version(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        self.diff(statute_id, old_version, latest_version)
    }
}
impl StatuteRegistry {
    /// Returns the current operation metrics.
    ///
    /// Note: This requires the registry to track metrics internally.
    /// This is a placeholder that returns default metrics.
    pub fn metrics(&self) -> OperationMetrics {
        OperationMetrics::default()
    }
}
#[cfg(feature = "yaml")]
impl StatuteRegistry {
    /// Exports the registry to YAML format.
    ///
    /// # Errors
    ///
    /// Returns an error if YAML serialization fails.
    pub fn export_yaml(&self) -> Result<String, serde_yaml::Error> {
        let backup = RegistryBackup {
            statutes: self.statutes.values().cloned().collect(),
            versions: self.versions.clone(),
            events: self.event_store.all_events().into_iter().cloned().collect(),
            metadata: BackupMetadata {
                created_at: Utc::now(),
                format_version: "1.0".to_string(),
                statute_count: self.statutes.len(),
                event_count: self.event_store.count(),
                description: Some("YAML export".to_string()),
            },
        };
        serde_yaml::to_string(&backup)
    }
    /// Imports a registry from YAML format.
    ///
    /// # Errors
    ///
    /// Returns an error if YAML deserialization fails or the backup is invalid.
    pub fn import_yaml(&mut self, yaml: &str) -> Result<(), Box<dyn std::error::Error>> {
        let backup: RegistryBackup = serde_yaml::from_str(yaml)?;
        self.restore_from_backup(backup)?;
        Ok(())
    }
    /// Exports a single statute entry to YAML.
    pub fn export_statute_yaml(entry: &StatuteEntry) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(entry)
    }
    /// Imports a single statute entry from YAML.
    pub fn import_statute_yaml(yaml: &str) -> Result<StatuteEntry, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}
#[cfg(feature = "csv-export")]
impl StatuteRegistry {
    /// Exports statute summaries to CSV format.
    ///
    /// # Errors
    ///
    /// Returns an error if CSV serialization fails.
    pub fn export_summaries_csv(&self) -> Result<String, csv::Error> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "statute_id",
            "title",
            "version",
            "status",
            "jurisdiction",
            "tags",
            "created_at",
            "modified_at",
            "is_active",
        ])?;
        for entry in self.statutes.values() {
            let summary = StatuteSummary::from(entry);
            wtr.write_record([
                &summary.statute_id,
                &summary.title,
                &summary.version.to_string(),
                &format!("{:?}", summary.status),
                &summary.jurisdiction,
                &summary.tags.join(";"),
                &summary.created_at.to_rfc3339(),
                &summary.modified_at.to_rfc3339(),
                &summary.is_active.to_string(),
            ])?;
        }
        let data = wtr
            .into_inner()
            .map_err(|e| csv::Error::from(std::io::Error::other(e)))?;
        Ok(String::from_utf8(data).unwrap_or_default())
    }
    /// Exports filtered statute summaries to CSV format.
    ///
    /// # Errors
    ///
    /// Returns an error if CSV serialization fails.
    pub fn export_filtered_csv(
        &self,
        filter: impl Fn(&StatuteEntry) -> bool,
    ) -> Result<String, csv::Error> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "statute_id",
            "title",
            "version",
            "status",
            "jurisdiction",
            "tags",
            "created_at",
            "modified_at",
            "is_active",
        ])?;
        for entry in self.statutes.values().filter(|e| filter(e)) {
            let summary = StatuteSummary::from(entry);
            wtr.write_record([
                &summary.statute_id,
                &summary.title,
                &summary.version.to_string(),
                &format!("{:?}", summary.status),
                &summary.jurisdiction,
                &summary.tags.join(";"),
                &summary.created_at.to_rfc3339(),
                &summary.modified_at.to_rfc3339(),
                &summary.is_active.to_string(),
            ])?;
        }
        let data = wtr
            .into_inner()
            .map_err(|e| csv::Error::from(std::io::Error::other(e)))?;
        Ok(String::from_utf8(data).unwrap_or_default())
    }
}
#[cfg(feature = "compression")]
impl StatuteRegistry {
    /// Exports a compressed backup.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or compression fails.
    pub fn export_compressed_backup(
        &self,
        description: Option<String>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use oxiarc_deflate::gzip::gzip_compress;
        let json = self.export_backup(description)?;
        let compressed = gzip_compress(json.as_bytes(), 6)?;
        Ok(compressed)
    }
    /// Imports a compressed backup.
    ///
    /// # Errors
    ///
    /// Returns an error if decompression or deserialization fails.
    pub fn import_compressed_backup(
        &mut self,
        compressed: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        use oxiarc_deflate::gzip::gzip_decompress;
        let bytes = gzip_decompress(compressed)?;
        let json = String::from_utf8(bytes)?;
        self.import_backup(&json)?;
        Ok(())
    }
    /// Returns the compression ratio of a backup (original_size / compressed_size).
    pub fn compression_ratio(
        &self,
        description: Option<String>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let original = self.export_backup(description)?;
        let compressed = self.export_compressed_backup(None)?;
        Ok(original.len() as f64 / compressed.len() as f64)
    }
}
impl StatuteRegistry {
    /// Performs a comprehensive health check on the registry.
    pub fn health_check(&self) -> HealthCheckResult {
        let start = std::time::Instant::now();
        let mut component_checks = HashMap::new();
        let cache_health = ComponentHealth::healthy("cache".to_string())
            .with_metric("capacity".to_string(), self.cache.cap().get() as f64)
            .with_metric("current_size".to_string(), self.cache.len() as f64);
        component_checks.insert("cache".to_string(), cache_health);
        let statute_count = self.statutes.len();
        let version_count: usize = self.versions.values().map(|v| v.len()).sum();
        let storage_health = ComponentHealth::healthy("storage".to_string())
            .with_metric("statutes".to_string(), statute_count as f64)
            .with_metric("versions".to_string(), version_count as f64);
        component_checks.insert("storage".to_string(), storage_health);
        let tag_count = self.tag_index.len();
        let jurisdiction_count = self.jurisdiction_index.len();
        let index_health = ComponentHealth::healthy("indexes".to_string())
            .with_metric("tags".to_string(), tag_count as f64)
            .with_metric("jurisdictions".to_string(), jurisdiction_count as f64);
        component_checks.insert("indexes".to_string(), index_health);
        let event_count = self.event_store.events.len();
        let event_health = ComponentHealth::healthy("event_store".to_string())
            .with_metric("events".to_string(), event_count as f64);
        component_checks.insert("event_store".to_string(), event_health);
        let mut issues = Vec::new();
        let errors = Vec::new();
        if statute_count == 0 {
            issues.push("Registry is empty".to_string());
        }
        if statute_count > 100000 {
            issues.push("Registry has very large number of statutes (>100k)".to_string());
        }
        if event_count > 1000000 {
            issues.push("Event store has very large number of events (>1M)".to_string());
        }
        let status = if !errors.is_empty() {
            HealthStatus::Unhealthy { errors }
        } else if !issues.is_empty() {
            HealthStatus::Degraded { issues }
        } else {
            HealthStatus::Healthy
        };
        let memory_estimate = statute_count * 1024 + version_count * 1024 + event_count * 512;
        let duration_ms = start.elapsed().as_millis() as u64;
        HealthCheckResult {
            status,
            timestamp: Utc::now(),
            statute_count,
            version_count,
            event_count,
            cache_hit_rate: 0.0,
            archived_count: self.archive.count(),
            memory_estimate_bytes: memory_estimate,
            check_duration_ms: duration_ms,
            component_checks,
        }
    }
    /// Compares this registry with another registry.
    pub fn compare_with(&self, other: &StatuteRegistry) -> RegistryDifference {
        let mut diff = RegistryDifference::new();
        let left_ids: HashSet<_> = self.statutes.keys().cloned().collect();
        let right_ids: HashSet<_> = other.statutes.keys().cloned().collect();
        diff.only_in_left = left_ids.difference(&right_ids).cloned().collect();
        diff.only_in_left.sort();
        diff.only_in_right = right_ids.difference(&left_ids).cloned().collect();
        diff.only_in_right.sort();
        for statute_id in left_ids.intersection(&right_ids) {
            let left_entry = &self.statutes[statute_id];
            let right_entry = &other.statutes[statute_id];
            if self.are_entries_identical(left_entry, right_entry) {
                diff.identical_statutes.push(statute_id.clone());
            } else {
                let differing_fields = self.find_differing_fields(left_entry, right_entry);
                diff.different_statutes.push(StatuteDifferenceDetail {
                    statute_id: statute_id.clone(),
                    differing_fields,
                    left_version: left_entry.version,
                    right_version: right_entry.version,
                });
            }
        }
        diff.identical_statutes.sort();
        diff
    }
    /// Checks if two statute entries are identical.
    fn are_entries_identical(&self, left: &StatuteEntry, right: &StatuteEntry) -> bool {
        left.statute.id == right.statute.id
            && left.statute.title == right.statute.title
            && left.version == right.version
            && left.status == right.status
            && left.jurisdiction == right.jurisdiction
            && left.tags == right.tags
    }
    /// Finds fields that differ between two entries.
    fn find_differing_fields(&self, left: &StatuteEntry, right: &StatuteEntry) -> Vec<String> {
        let mut fields = Vec::new();
        if left.statute.title != right.statute.title {
            fields.push("title".to_string());
        }
        if left.version != right.version {
            fields.push("version".to_string());
        }
        if left.status != right.status {
            fields.push("status".to_string());
        }
        if left.jurisdiction != right.jurisdiction {
            fields.push("jurisdiction".to_string());
        }
        if left.tags != right.tags {
            fields.push("tags".to_string());
        }
        if left.effective_date != right.effective_date {
            fields.push("effective_date".to_string());
        }
        if left.expiry_date != right.expiry_date {
            fields.push("expiry_date".to_string());
        }
        fields
    }
    /// Performs bulk registration with configuration.
    pub fn bulk_register(
        &mut self,
        entries: Vec<StatuteEntry>,
        config: BulkConfig,
    ) -> BulkOperationResult {
        let start = std::time::Instant::now();
        let mut result = BulkOperationResult::new();
        for chunk in entries.chunks(config.batch_size) {
            for entry in chunk {
                result.total_processed += 1;
                match self.register(entry.clone()) {
                    Ok(_) => result.successful += 1,
                    Err(e) => {
                        result.failed += 1;
                        result
                            .errors
                            .insert(entry.statute.id.clone(), e.to_string());
                        if !config.continue_on_error {
                            result.duration_ms = start.elapsed().as_millis() as u64;
                            return result;
                        }
                    }
                }
            }
        }
        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }
    /// Performs bulk deletion with configuration.
    pub fn bulk_delete_with_config(
        &mut self,
        statute_ids: Vec<String>,
        config: BulkConfig,
    ) -> BulkOperationResult {
        let start = std::time::Instant::now();
        let mut result = BulkOperationResult::new();
        for chunk in statute_ids.chunks(config.batch_size) {
            for statute_id in chunk {
                result.total_processed += 1;
                match self.delete(statute_id) {
                    Ok(_) => result.successful += 1,
                    Err(e) => {
                        result.failed += 1;
                        result.errors.insert(statute_id.clone(), e.to_string());
                        if !config.continue_on_error {
                            result.duration_ms = start.elapsed().as_millis() as u64;
                            return result;
                        }
                    }
                }
            }
        }
        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }
    /// Streams statute IDs matching a predicate.
    pub fn stream_ids<F>(&self, predicate: F) -> Vec<String>
    where
        F: Fn(&StatuteEntry) -> bool,
    {
        self.statutes
            .iter()
            .filter(|(_, entry)| predicate(entry))
            .map(|(id, _)| id.clone())
            .collect()
    }
    /// Streams entries matching a predicate with batching.
    pub fn stream_entries<F>(&self, predicate: F, batch_size: usize) -> Vec<Vec<StatuteEntry>>
    where
        F: Fn(&StatuteEntry) -> bool,
    {
        let entries: Vec<StatuteEntry> = self
            .statutes
            .values()
            .filter(|entry| predicate(entry))
            .cloned()
            .collect();
        entries
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}
impl StatuteRegistry {
    /// Calculates quality score for a statute entry.
    pub fn calculate_quality_score(&self, entry: &StatuteEntry) -> QualityScore {
        let mut completeness_score = 0.0;
        completeness_score += 30.0;
        if entry.expiry_date.is_some() {
            completeness_score += 10.0;
        }
        if !entry.tags.is_empty() {
            completeness_score += 15.0;
        }
        if !entry.metadata.is_empty() {
            completeness_score += 15.0;
        }
        if entry.amends.is_some() {
            completeness_score += 10.0;
        }
        if !entry.supersedes.is_empty() {
            completeness_score += 10.0;
        }
        if !entry.references.is_empty() {
            completeness_score += 10.0;
        }
        let mut consistency_score = 100.0;
        if let (Some(expiry), Some(effective)) = (entry.expiry_date, entry.effective_date)
            && expiry <= effective
        {
            consistency_score -= 30.0;
        }
        if entry.status == StatuteStatus::Repealed {
            if let Some(expiry) = entry.expiry_date {
                if expiry > Utc::now() {
                    consistency_score -= 20.0;
                }
            } else {
                consistency_score -= 20.0;
            }
        }
        let metadata_richness = if entry.metadata.is_empty() {
            0.0
        } else {
            ((entry.metadata.len() as f64).min(10.0) / 10.0) * 100.0
        };
        let doc_quality = {
            let title_len = entry.statute.title.len();
            let has_description = entry
                .metadata
                .contains_key("description")
                .then_some(())
                .is_some();
            let has_tags = !entry.tags.is_empty();
            let mut score = 0.0;
            if title_len > 10 {
                score += 40.0;
            } else if title_len > 5 {
                score += 20.0;
            }
            if has_description {
                score += 40.0;
            }
            if has_tags {
                score += 20.0;
            }
            score
        };
        QualityScore::new(
            completeness_score,
            consistency_score,
            metadata_richness,
            doc_quality,
        )
    }
    /// Performs quality assessment for a statute.
    pub fn assess_quality(&self, statute_id: &str) -> RegistryResult<QualityAssessment> {
        let entry = self
            .statutes
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        let score = self.calculate_quality_score(entry);
        let mut assessment = QualityAssessment::new(statute_id.to_string(), score);
        if entry.tags.is_empty() {
            assessment = assessment
                .with_issue("No tags assigned".to_string())
                .with_suggestion("Add relevant tags for better categorization".to_string());
        }
        if entry.metadata.is_empty() {
            assessment = assessment
                .with_issue("No metadata provided".to_string())
                .with_suggestion("Add metadata fields like description, author, etc.".to_string());
        }
        if let (Some(expiry), Some(effective)) = (entry.expiry_date, entry.effective_date)
            && expiry <= effective
        {
            assessment = assessment
                .with_issue("Expiry date is before or equal to effective date".to_string());
        }
        if entry.status == StatuteStatus::Repealed && entry.expiry_date.is_none() {
            assessment =
                assessment.with_issue("Status is Repealed but no expiry date is set".to_string());
        }
        if entry.statute.title.len() < 10 {
            assessment = assessment
                .with_issue("Title is too short".to_string())
                .with_suggestion("Use a more descriptive title".to_string());
        }
        Ok(assessment)
    }
    /// Assesses quality for all statutes in the registry.
    pub fn assess_all_quality(&self) -> Vec<QualityAssessment> {
        self.statutes
            .keys()
            .filter_map(|id| self.assess_quality(id).ok())
            .collect()
    }
    /// Calculates similarity between two statute entries.
    pub fn calculate_similarity(
        &self,
        entry1: &StatuteEntry,
        entry2: &StatuteEntry,
    ) -> SimilarityScore {
        let matcher = SkimMatcherV2::default();
        let title_sim = matcher
            .fuzzy_match(&entry1.statute.title, &entry2.statute.title)
            .map(|score| (score as f64 / 100.0).min(1.0))
            .unwrap_or(0.0);
        let content_sim = {
            let refs1: HashSet<_> = entry1.references.iter().collect();
            let refs2: HashSet<_> = entry2.references.iter().collect();
            let common = refs1.intersection(&refs2).count();
            let total = refs1.union(&refs2).count();
            if total > 0 {
                common as f64 / total as f64
            } else {
                if entry1.statute.effect.effect_type == entry2.statute.effect.effect_type {
                    0.5
                } else {
                    0.0
                }
            }
        };
        let tags1: HashSet<_> = entry1.tags.iter().collect();
        let tags2: HashSet<_> = entry2.tags.iter().collect();
        let common_tags = tags1.intersection(&tags2).count();
        let total_tags = tags1.union(&tags2).count();
        let metadata_sim = if total_tags > 0 {
            common_tags as f64 / total_tags as f64
        } else {
            0.0
        };
        SimilarityScore::new(title_sim, content_sim, metadata_sim)
    }
    /// Detects potential duplicate statutes.
    pub fn detect_duplicates(&self, threshold: f64) -> DuplicateDetectionResult {
        let statute_ids: Vec<_> = self.statutes.keys().cloned().collect();
        let mut result = DuplicateDetectionResult::new(threshold, statute_ids.len());
        for i in 0..statute_ids.len() {
            for j in (i + 1)..statute_ids.len() {
                let id1 = &statute_ids[i];
                let id2 = &statute_ids[j];
                if let (Some(entry1), Some(entry2)) =
                    (self.statutes.get(id1), self.statutes.get(id2))
                {
                    let similarity = self.calculate_similarity(entry1, entry2);
                    if similarity.overall >= threshold * 0.7 {
                        let reason = if similarity.overall >= threshold {
                            "High similarity detected".to_string()
                        } else {
                            "Moderate similarity detected".to_string()
                        };
                        result.add_candidate(DuplicateCandidate::new(
                            id1.clone(),
                            id2.clone(),
                            similarity,
                            reason,
                        ));
                    }
                }
            }
        }
        result
    }
    /// Profiles the data in the registry.
    pub fn profile_data(&mut self) -> DataProfile {
        let total = self.statutes.len();
        let mut profile = DataProfile::new(total);
        let mut total_quality = 0.0;
        let mut quality_counts: HashMap<char, usize> = HashMap::new();
        for entry in self.statutes.values() {
            let score = self.calculate_quality_score(entry);
            total_quality += score.overall;
            let grade = score.grade();
            *quality_counts.entry(grade).or_insert(0) += 1;
            *profile.status_distribution.entry(entry.status).or_insert(0) += 1;
            *profile
                .jurisdiction_distribution
                .entry(entry.jurisdiction.clone())
                .or_insert(0) += 1;
            for tag in &entry.tags {
                *profile.tag_patterns.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        if total > 0 {
            profile.average_quality = total_quality / total as f64;
        }
        profile.quality_distribution = quality_counts;
        let mut title_profile = FieldProfile::new("title".to_string(), total);
        let mut jurisdiction_profile = FieldProfile::new("jurisdiction".to_string(), total);
        let mut tags_profile = FieldProfile::new("tags".to_string(), total);
        let mut title_counts: HashMap<String, usize> = HashMap::new();
        let mut jurisdiction_counts: HashMap<String, usize> = HashMap::new();
        for entry in self.statutes.values() {
            *title_counts.entry(entry.statute.title.clone()).or_insert(0) += 1;
            *jurisdiction_counts
                .entry(entry.jurisdiction.clone())
                .or_insert(0) += 1;
            if entry.tags.is_empty() {
                tags_profile.null_count += 1;
            }
        }
        title_profile.unique_count = title_counts.len();
        title_profile.calculate_completeness();
        let mut title_vec: Vec<_> = title_counts.into_iter().collect();
        title_vec.sort_by_key(|b| std::cmp::Reverse(b.1));
        title_profile.most_common = title_vec.into_iter().take(10).collect();
        jurisdiction_profile.unique_count = jurisdiction_counts.len();
        jurisdiction_profile.calculate_completeness();
        let mut jurisdiction_vec: Vec<_> = jurisdiction_counts.into_iter().collect();
        jurisdiction_vec.sort_by_key(|b| std::cmp::Reverse(b.1));
        jurisdiction_profile.most_common = jurisdiction_vec.into_iter().take(10).collect();
        tags_profile.unique_count = profile.tag_patterns.len();
        tags_profile.calculate_completeness();
        profile.add_field_profile(title_profile);
        profile.add_field_profile(jurisdiction_profile);
        profile.add_field_profile(tags_profile);
        profile
    }
    /// Finds statutes with quality scores below a threshold.
    pub fn find_low_quality_statutes(&self, threshold: f64) -> Vec<(String, QualityScore)> {
        self.statutes
            .iter()
            .map(|(id, entry)| (id.clone(), self.calculate_quality_score(entry)))
            .filter(|(_, score)| score.overall < threshold)
            .collect()
    }
    /// Exports quality assessments to JSON.
    pub fn export_quality_assessments_json(&self) -> Result<String, serde_json::Error> {
        let assessments = self.assess_all_quality();
        serde_json::to_string_pretty(&assessments)
    }
    /// Exports duplicate detection results to JSON.
    pub fn export_duplicates_json(&self, threshold: f64) -> Result<String, serde_json::Error> {
        let duplicates = self.detect_duplicates(threshold);
        serde_json::to_string_pretty(&duplicates)
    }
}
impl StatuteRegistry {
    /// Analyzes a statute for enrichment opportunities.
    pub fn analyze_enrichment(
        &self,
        statute_id: &str,
        config: &EnrichmentConfig,
    ) -> RegistryResult<EnrichmentResult> {
        let entry = self
            .statutes
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        let mut result = EnrichmentResult::new(statute_id.to_string());
        if config.enable_auto_tagging {
            self.suggest_auto_tags(entry, &mut result);
        }
        if config.enable_metadata_inference {
            self.suggest_metadata(entry, &mut result);
        }
        if config.enable_jurisdiction_inference {
            self.suggest_jurisdiction_metadata(entry, &mut result);
        }
        Ok(result)
    }
    /// Suggests automatic tags based on content.
    fn suggest_auto_tags(&self, entry: &StatuteEntry, result: &mut EnrichmentResult) {
        let title_lower = entry.statute.title.to_lowercase();
        let tag_patterns = [
            ("civil", vec!["civil", "contract", "property", "tort"]),
            ("criminal", vec!["criminal", "penal", "offense", "crime"]),
            (
                "administrative",
                vec!["administrative", "regulation", "agency"],
            ),
            ("tax", vec!["tax", "revenue", "fiscal"]),
            ("employment", vec!["employment", "labor", "worker"]),
            ("corporate", vec!["corporate", "company", "business"]),
            (
                "intellectual-property",
                vec!["patent", "trademark", "copyright", "ip"],
            ),
            (
                "environmental",
                vec!["environmental", "pollution", "conservation"],
            ),
            ("healthcare", vec!["health", "medical", "patient"]),
            ("education", vec!["education", "school", "university"]),
        ];
        for (tag, keywords) in &tag_patterns {
            if !entry.tags.contains(&tag.to_string()) {
                let matches = keywords
                    .iter()
                    .filter(|kw| title_lower.contains(*kw))
                    .count();
                if matches > 0 {
                    let confidence = (matches as f64 / keywords.len() as f64).min(0.95);
                    result.add_suggestion(EnrichmentSuggestion::new(
                        EnrichmentType::AutoTag,
                        tag.to_string(),
                        confidence,
                        format!("Title contains keywords: {}", keywords.join(", ")),
                    ));
                }
            }
        }
    }
    /// Suggests metadata based on analysis.
    fn suggest_metadata(&self, entry: &StatuteEntry, result: &mut EnrichmentResult) {
        if !entry.metadata.contains_key("description") {
            result.add_suggestion(EnrichmentSuggestion::new(
                EnrichmentType::MetadataInference,
                "description".to_string(),
                0.6,
                "Missing description metadata - consider adding statute summary".to_string(),
            ));
        }
        if !entry.metadata.contains_key("category") && !entry.tags.is_empty() {
            let category = entry
                .tags
                .first()
                .expect("invariant: tags.is_empty() checked above");
            result.add_suggestion(EnrichmentSuggestion::new(
                EnrichmentType::CategoryClassification,
                category.clone(),
                0.75,
                format!("Category inferred from primary tag: {}", category),
            ));
        }
        if entry.effective_date.is_none() && !entry.metadata.contains_key("effective_date_note") {
            result.add_suggestion(EnrichmentSuggestion::new(
                EnrichmentType::MetadataInference,
                "effective_date_note".to_string(),
                0.5,
                "Consider adding effective date information".to_string(),
            ));
        }
    }
    /// Suggests jurisdiction-related metadata.
    fn suggest_jurisdiction_metadata(&self, entry: &StatuteEntry, result: &mut EnrichmentResult) {
        let jurisdiction_count = self
            .statutes
            .values()
            .filter(|e| e.jurisdiction == entry.jurisdiction)
            .count();
        if jurisdiction_count > 10 && !entry.metadata.contains_key("jurisdiction_family") {
            result.add_suggestion(EnrichmentSuggestion::new(
                EnrichmentType::JurisdictionInference,
                "jurisdiction_family".to_string(),
                0.8,
                format!(
                    "Part of {} statute family in jurisdiction {}",
                    jurisdiction_count, entry.jurisdiction
                ),
            ));
        }
    }
    /// Applies enrichment suggestions to a statute.
    pub fn apply_enrichment(
        &mut self,
        statute_id: &str,
        suggestions: &[EnrichmentSuggestion],
        min_confidence: f64,
    ) -> RegistryResult<usize> {
        let entry = self
            .statutes
            .get_mut(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        let mut applied_count = 0;
        for suggestion in suggestions {
            if !suggestion.meets_threshold(min_confidence) {
                continue;
            }
            match suggestion.enrichment_type {
                EnrichmentType::AutoTag => {
                    if !entry.tags.contains(&suggestion.suggestion) {
                        entry.tags.push(suggestion.suggestion.clone());
                        applied_count += 1;
                    }
                }
                EnrichmentType::MetadataInference
                | EnrichmentType::CategoryClassification
                | EnrichmentType::JurisdictionInference => {
                    let key = suggestion.suggestion.clone();
                    if let std::collections::hash_map::Entry::Vacant(e) = entry.metadata.entry(key)
                    {
                        e.insert(format!("Auto-enriched: {}", suggestion.reason));
                        applied_count += 1;
                    }
                }
                EnrichmentType::RelatedStatute => {
                    if !entry.references.contains(&suggestion.suggestion) {
                        entry.references.push(suggestion.suggestion.clone());
                        applied_count += 1;
                    }
                }
            }
        }
        entry.etag = Uuid::new_v4().to_string();
        Ok(applied_count)
    }
    /// Auto-enriches all statutes in the registry.
    pub fn auto_enrich_all(&mut self, config: &EnrichmentConfig) -> Vec<(String, usize)> {
        let statute_ids: Vec<_> = self.statutes.keys().cloned().collect();
        let mut results = Vec::new();
        for statute_id in statute_ids {
            if let Ok(enrichment) = self.analyze_enrichment(&statute_id, config) {
                let high_confidence = enrichment.high_confidence_suggestions(config.min_confidence);
                if !high_confidence.is_empty() {
                    let suggestions: Vec<_> = high_confidence.into_iter().cloned().collect();
                    if let Ok(count) =
                        self.apply_enrichment(&statute_id, &suggestions, config.min_confidence)
                        && count > 0
                    {
                        results.push((statute_id, count));
                    }
                }
            }
        }
        results
    }
}
impl StatuteRegistry {
    /// Records a lineage entry for a statute.
    #[allow(dead_code)]
    pub fn record_lineage(&mut self, _entry: LineageEntry) {}
}
impl StatuteRegistry {
    /// Scans a statute for PII using the detector.
    pub fn scan_for_pii(
        &mut self,
        statute_id: &str,
        detector: &PiiDetector,
    ) -> RegistryResult<PiiScanResult> {
        let entry = self
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        let content = format!(
            "{} {}",
            entry.statute.title,
            entry
                .metadata
                .values()
                .cloned()
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(detector.scan(statute_id, &content))
    }
    /// Applies data retention rules and returns affected statutes.
    pub fn apply_retention_rules(
        &mut self,
        config: &DataRetentionConfig,
    ) -> RetentionExecutionResult {
        let mut to_delete = Vec::new();
        let mut to_archive = Vec::new();
        let now = Utc::now();
        for (statute_id, entry) in self.statutes.iter() {
            for rule in config.rules() {
                match rule {
                    DataRetentionRule::RetainForDays(days) => {
                        let age = now.signed_duration_since(entry.created_at).num_days();
                        if age > *days as i64 {
                            to_delete.push(statute_id.clone());
                        }
                    }
                    DataRetentionRule::RetainUntil(until) => {
                        if now > *until {
                            to_delete.push(statute_id.clone());
                        }
                    }
                    DataRetentionRule::DeleteInactiveAfterDays(days) => {
                        if !entry.is_active() {
                            let age = now.signed_duration_since(entry.modified_at).num_days();
                            if age > *days as i64 {
                                to_delete.push(statute_id.clone());
                            }
                        }
                    }
                    DataRetentionRule::ArchiveAfterDays(days) => {
                        let age = now.signed_duration_since(entry.created_at).num_days();
                        if age > *days as i64 {
                            to_archive.push(statute_id.clone());
                        }
                    }
                    DataRetentionRule::RetainIndefinitely => {}
                }
            }
        }
        to_delete.sort();
        to_delete.dedup();
        to_archive.sort();
        to_archive.dedup();
        if !config.is_dry_run() {
            for statute_id in &to_delete {
                let _ = self.delete(statute_id);
            }
            for statute_id in &to_archive {
                let _ = self.archive_statute(statute_id, "Automatic retention policy".to_string());
            }
        }
        RetentionExecutionResult::new(to_delete, to_archive, config.is_dry_run())
    }
    /// Generates an audit report based on configuration.
    pub fn generate_audit_report(&self, config: &AuditReportConfig) -> AuditReport {
        let mut content_parts = Vec::new();
        content_parts.push(format!("Audit Report: {}", config.title));
        content_parts.push(format!("Generated: {}", Utc::now()));
        if let (Some(start), Some(end)) = (config.start_date, config.end_date) {
            content_parts.push(format!("Period: {} to {}", start, end));
        }
        content_parts.push(String::new());
        content_parts.push("=== Statistics ===".to_string());
        content_parts.push(format!("Total Statutes: {}", self.statutes.len()));
        content_parts.push(format!("Total Events: {}", self.event_store.events.len()));
        content_parts.push(String::new());
        if config.include_events {
            content_parts.push("=== Events ===".to_string());
            let mut event_count = 0;
            for event in &self.event_store.events {
                let event_timestamp = match event {
                    RegistryEvent::StatuteRegistered { timestamp, .. } => *timestamp,
                    RegistryEvent::StatuteUpdated { timestamp, .. } => *timestamp,
                    RegistryEvent::StatusChanged { timestamp, .. } => *timestamp,
                    RegistryEvent::TagAdded { timestamp, .. } => *timestamp,
                    RegistryEvent::TagRemoved { timestamp, .. } => *timestamp,
                    RegistryEvent::ReferenceAdded { timestamp, .. } => *timestamp,
                    RegistryEvent::ReferenceRemoved { timestamp, .. } => *timestamp,
                    RegistryEvent::MetadataUpdated { timestamp, .. } => *timestamp,
                    RegistryEvent::StatuteDeleted { timestamp, .. } => *timestamp,
                    RegistryEvent::StatuteArchived { timestamp, .. } => *timestamp,
                };
                let include = if let (Some(start), Some(end)) = (config.start_date, config.end_date)
                {
                    event_timestamp >= start && event_timestamp <= end
                } else {
                    true
                };
                if include {
                    content_parts.push(format!("- {:?} at {}", event, event_timestamp));
                    event_count += 1;
                }
            }
            content_parts.push(format!("Total events in period: {}", event_count));
            content_parts.push(String::new());
        }
        let content = content_parts.join("\n");
        AuditReport::new(
            config.title.clone(),
            (config.start_date, config.end_date),
            self.statutes.len(),
            self.event_store.events.len(),
            0,
            0,
            0.0,
            content,
            config.format,
        )
    }
    /// Generates a compliance dashboard with current metrics.
    pub fn generate_compliance_dashboard(&mut self, quality_threshold: f64) -> ComplianceDashboard {
        let total_statutes = self.statutes.len();
        let total_audit_events = self.event_store.events.len();
        let assessments = self.assess_all_quality();
        let low_quality = assessments
            .iter()
            .filter(|a| !a.score.meets_threshold(quality_threshold))
            .count();
        let avg_quality = if !assessments.is_empty() {
            assessments.iter().map(|a| a.score.overall).sum::<f64>() / assessments.len() as f64
        } else {
            0.0
        };
        ComplianceDashboard::new(
            total_statutes,
            0,
            0,
            avg_quality,
            low_quality,
            total_audit_events,
            0,
            0,
        )
    }
    /// Checks if a statute can be accessed from a specific region.
    pub fn check_sovereignty_access(
        &self,
        _statute_id: &str,
        _requesting_region: &GeographicRegion,
        config: &DataSovereigntyConfig,
    ) -> bool {
        config.is_region_allowed(_requesting_region)
    }
}
/// Selective export by criteria.
impl StatuteRegistry {
    /// Exports statutes matching a filter predicate.
    pub fn export_filtered_statutes<F>(&self, filter: F) -> Result<String, RegistryError>
    where
        F: Fn(&StatuteEntry) -> bool,
    {
        let filtered: Vec<_> = self
            .statutes
            .values()
            .filter(|entry| filter(entry))
            .collect();
        serde_json::to_string_pretty(&filtered)
            .map_err(|e| RegistryError::InvalidOperation(format!("Export failed: {}", e)))
    }
    /// Exports statutes by status.
    pub fn export_by_status(&self, status: StatuteStatus) -> Result<String, RegistryError> {
        self.export_filtered_statutes(|entry| entry.status == status)
    }
    /// Exports statutes by jurisdiction.
    pub fn export_by_jurisdiction(&self, jurisdiction: &str) -> Result<String, RegistryError> {
        self.export_filtered_statutes(|entry| entry.jurisdiction == jurisdiction)
    }
    /// Exports statutes by tag.
    pub fn export_by_tag(&self, tag: &str) -> Result<String, RegistryError> {
        self.export_filtered_statutes(|entry| entry.tags.iter().any(|t| t == tag))
    }
    /// Exports statutes modified within a date range.
    pub fn export_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<String, RegistryError> {
        self.export_filtered_statutes(|entry| {
            entry.modified_at >= start && entry.modified_at <= end
        })
    }
}
