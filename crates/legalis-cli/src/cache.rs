//! Command result caching for improved performance.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Cache entry metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Cache key (hash of input parameters)
    pub key: String,

    /// Cached result (serialized)
    pub value: String,

    /// Timestamp when cached (Unix epoch seconds)
    pub timestamp: u64,

    /// Source file paths that were used to generate this result
    pub source_files: Vec<String>,

    /// Command that generated this cache entry
    pub command: String,
}

/// Cache manager for command results.
pub struct CacheManager {
    cache_dir: PathBuf,
    max_age_seconds: u64,
}

impl CacheManager {
    /// Create a new cache manager.
    pub fn new() -> Result<Self> {
        let cache_dir = Self::cache_directory()?;
        fs::create_dir_all(&cache_dir).with_context(|| {
            format!("Failed to create cache directory: {}", cache_dir.display())
        })?;

        Ok(Self {
            cache_dir,
            max_age_seconds: 3600, // 1 hour default
        })
    }

    /// Create a cache manager with custom max age.
    pub fn with_max_age(max_age_seconds: u64) -> Result<Self> {
        let mut manager = Self::new()?;
        manager.max_age_seconds = max_age_seconds;
        Ok(manager)
    }

    /// Get the cache directory path.
    pub fn cache_directory() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .context("Failed to determine cache directory")?
            .join("legalis");
        Ok(cache_dir)
    }

    /// Generate a cache key from command name and parameters.
    pub fn generate_key(command: &str, params: &[&str]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        command.hash(&mut hasher);
        for param in params {
            param.hash(&mut hasher);
        }
        format!("{:x}", hasher.finish())
    }

    /// Get cached result if valid.
    pub fn get(&self, key: &str) -> Result<Option<CacheEntry>> {
        let cache_file = self.cache_dir.join(format!("{}.json", key));

        if !cache_file.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&cache_file)
            .with_context(|| format!("Failed to read cache file: {}", cache_file.display()))?;

        let entry: CacheEntry = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse cache file: {}", cache_file.display()))?;

        // Check if cache is expired
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        if now - entry.timestamp > self.max_age_seconds {
            // Cache expired, remove it
            let _ = fs::remove_file(&cache_file);
            return Ok(None);
        }

        // Check if source files have been modified
        for source_file in &entry.source_files {
            if let Ok(metadata) = fs::metadata(source_file) {
                if let Ok(modified) = metadata.modified() {
                    let modified_secs = modified.duration_since(UNIX_EPOCH)?.as_secs();
                    if modified_secs > entry.timestamp {
                        // Source file modified after cache, invalidate
                        let _ = fs::remove_file(&cache_file);
                        return Ok(None);
                    }
                }
            } else {
                // Source file doesn't exist anymore, invalidate cache
                let _ = fs::remove_file(&cache_file);
                return Ok(None);
            }
        }

        Ok(Some(entry))
    }

    /// Store a result in the cache.
    pub fn set(&self, entry: CacheEntry) -> Result<()> {
        let cache_file = self.cache_dir.join(format!("{}.json", entry.key));

        let content =
            serde_json::to_string_pretty(&entry).context("Failed to serialize cache entry")?;

        fs::write(&cache_file, content)
            .with_context(|| format!("Failed to write cache file: {}", cache_file.display()))?;

        Ok(())
    }

    /// Clear all cache entries.
    pub fn clear_all(&self) -> Result<usize> {
        let mut count = 0;

        if !self.cache_dir.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                fs::remove_file(path)?;
                count += 1;
            }
        }

        Ok(count)
    }

    /// Clear expired cache entries.
    pub fn clear_expired(&self) -> Result<usize> {
        let mut count = 0;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        if !self.cache_dir.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) {
                        if now - cache_entry.timestamp > self.max_age_seconds {
                            fs::remove_file(path)?;
                            count += 1;
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    /// Get cache statistics.
    pub fn stats(&self) -> Result<CacheStats> {
        let mut stats = CacheStats::default();

        if !self.cache_dir.exists() {
            return Ok(stats);
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                stats.total_entries += 1;

                if let Ok(metadata) = fs::metadata(&path) {
                    stats.total_size += metadata.len();
                }

                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) {
                        if now - cache_entry.timestamp > self.max_age_seconds {
                            stats.expired_entries += 1;
                        }

                        *stats.commands.entry(cache_entry.command).or_insert(0) += 1;
                    }
                }
            }
        }

        Ok(stats)
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new().expect("Failed to create cache manager")
    }
}

/// Cache statistics.
#[derive(Debug, Default)]
pub struct CacheStats {
    /// Total number of cache entries
    pub total_entries: usize,

    /// Number of expired entries
    pub expired_entries: usize,

    /// Total cache size in bytes
    pub total_size: u64,

    /// Number of entries per command
    pub commands: HashMap<String, usize>,
}

impl CacheStats {
    /// Format statistics for display.
    pub fn format(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("Total entries: {}\n", self.total_entries));
        output.push_str(&format!("Expired entries: {}\n", self.expired_entries));
        output.push_str(&format!(
            "Valid entries: {}\n",
            self.total_entries - self.expired_entries
        ));
        output.push_str(&format!(
            "Total size: {} bytes ({:.2} MB)\n",
            self.total_size,
            self.total_size as f64 / 1_048_576.0
        ));

        if !self.commands.is_empty() {
            output.push_str("\nEntries by command:\n");
            let mut commands: Vec<_> = self.commands.iter().collect();
            commands.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

            for (command, count) in commands {
                output.push_str(&format!("  {}: {}\n", command, count));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key1 = CacheManager::generate_key("verify", &["file1.ldsl", "file2.ldsl"]);
        let key2 = CacheManager::generate_key("verify", &["file1.ldsl", "file2.ldsl"]);
        let key3 = CacheManager::generate_key("verify", &["file1.ldsl"]);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_cache_manager_creation() {
        let manager = CacheManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_cache_stats_format() {
        let mut stats = CacheStats {
            total_entries: 10,
            expired_entries: 2,
            total_size: 1024,
            ..Default::default()
        };
        stats.commands.insert("verify".to_string(), 5);
        stats.commands.insert("parse".to_string(), 5);

        let formatted = stats.format();
        assert!(formatted.contains("Total entries: 10"));
        assert!(formatted.contains("Expired entries: 2"));
        assert!(formatted.contains("verify: 5"));
    }
}
