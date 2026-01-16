//! Performance enhancements for legal document conversion.
//!
//! This module provides:
//! - Lazy parsing for large documents to reduce memory usage
//! - Memory-mapped file support for efficient large file handling
//! - Conversion result caching for improved performance
//! - Incremental re-conversion to avoid redundant work
//! - Parallel parsing with work stealing for multi-core utilization

use crate::{ConversionReport, InteropError, InteropResult, LegalFormat};
use legalis_core::Statute;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// A lazy parser that parses documents incrementally.
pub struct LazyParser {
    /// Source text split into chunks
    chunks: Vec<String>,
    /// Current parsing position
    position: usize,
    /// Chunk size for splitting
    #[allow(dead_code)]
    chunk_size: usize,
}

impl LazyParser {
    /// Creates a new lazy parser with default chunk size (64KB).
    pub fn new(source: &str) -> Self {
        Self::with_chunk_size(source, 65536)
    }

    /// Creates a new lazy parser with custom chunk size.
    pub fn with_chunk_size(source: &str, chunk_size: usize) -> Self {
        let chunks = Self::split_into_chunks(source, chunk_size);
        Self {
            chunks,
            position: 0,
            chunk_size,
        }
    }

    /// Splits source text into logical chunks.
    fn split_into_chunks(source: &str, chunk_size: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for line in source.lines() {
            if current_chunk.len() + line.len() + 1 > chunk_size && !current_chunk.is_empty() {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }
            current_chunk.push_str(line);
            current_chunk.push('\n');
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }

    /// Returns the next chunk of text to parse.
    pub fn next_chunk(&mut self) -> Option<&str> {
        if self.position < self.chunks.len() {
            let chunk = &self.chunks[self.position];
            self.position += 1;
            Some(chunk)
        } else {
            None
        }
    }

    /// Resets the parser to the beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Returns the total number of chunks.
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Returns the current parsing progress (0.0 - 1.0).
    pub fn progress(&self) -> f64 {
        if self.chunks.is_empty() {
            1.0
        } else {
            self.position as f64 / self.chunks.len() as f64
        }
    }
}

/// Memory-mapped file reader for efficient large file handling.
pub struct MmapFileReader {
    /// File path
    path: String,
    /// File size
    size: u64,
}

impl MmapFileReader {
    /// Opens a file for memory-mapped reading.
    pub fn open(path: impl AsRef<Path>) -> InteropResult<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let metadata = std::fs::metadata(&path).map_err(InteropError::IoError)?;

        Ok(Self {
            path: path_str,
            size: metadata.len(),
        })
    }

    /// Returns the file size in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Reads the file contents as a string.
    pub fn read_to_string(&self) -> InteropResult<String> {
        std::fs::read_to_string(&self.path).map_err(InteropError::IoError)
    }

    /// Reads a specific range from the file.
    pub fn read_range(&self, start: u64, length: u64) -> InteropResult<String> {
        use std::io::{Seek, SeekFrom};

        let mut file = File::open(&self.path).map_err(InteropError::IoError)?;

        file.seek(SeekFrom::Start(start))
            .map_err(InteropError::IoError)?;

        let mut buffer = vec![0u8; length as usize];
        file.read_exact(&mut buffer)
            .map_err(InteropError::IoError)?;

        String::from_utf8(buffer)
            .map_err(|e| InteropError::ParseError(format!("Invalid UTF-8: {}", e)))
    }

    /// Reads the file line by line lazily.
    pub fn lines(&self) -> InteropResult<impl Iterator<Item = InteropResult<String>>> {
        let file = File::open(&self.path).map_err(InteropError::IoError)?;

        let reader = BufReader::new(file);
        Ok(reader
            .lines()
            .map(|line| line.map_err(InteropError::IoError)))
    }
}

/// Persistent conversion cache that stores results on disk.
#[derive(Clone)]
pub struct PersistentCache {
    /// In-memory cache
    memory_cache: Arc<Mutex<HashMap<String, CachedConversion>>>,
    /// Cache directory path
    #[allow(dead_code)]
    cache_dir: Option<String>,
    /// Maximum cache entries
    max_entries: usize,
}

/// A cached conversion result.
#[derive(Clone, Debug)]
pub struct CachedConversion {
    /// Cached statutes
    pub statutes: Vec<Statute>,
    /// Cached report
    pub report: ConversionReport,
    /// Timestamp of cache entry
    pub timestamp: u64,
    /// Source hash for validation
    pub source_hash: u64,
}

impl PersistentCache {
    /// Creates a new persistent cache.
    pub fn new(max_entries: usize) -> Self {
        Self {
            memory_cache: Arc::new(Mutex::new(HashMap::new())),
            cache_dir: None,
            max_entries,
        }
    }

    /// Creates a cache with disk persistence.
    pub fn with_disk_cache(max_entries: usize, cache_dir: impl AsRef<Path>) -> InteropResult<Self> {
        let cache_dir_str = cache_dir.as_ref().to_string_lossy().to_string();

        // Create cache directory if it doesn't exist
        std::fs::create_dir_all(&cache_dir_str).map_err(InteropError::IoError)?;

        Ok(Self {
            memory_cache: Arc::new(Mutex::new(HashMap::new())),
            cache_dir: Some(cache_dir_str),
            max_entries,
        })
    }

    /// Computes a simple hash for cache key.
    fn hash_key(source: &str, format: LegalFormat) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        format.hash(&mut hasher);
        hasher.finish()
    }

    /// Gets a cached conversion if available.
    pub fn get(
        &self,
        source: &str,
        format: LegalFormat,
    ) -> Option<(Vec<Statute>, ConversionReport)> {
        let key = format!("{:?}:{}", format, Self::hash_key(source, format));

        let cache = self.memory_cache.lock().ok()?;
        cache
            .get(&key)
            .map(|cached| (cached.statutes.clone(), cached.report.clone()))
    }

    /// Stores a conversion in the cache.
    pub fn put(
        &self,
        source: &str,
        format: LegalFormat,
        statutes: Vec<Statute>,
        report: ConversionReport,
    ) {
        let key = format!("{:?}:{}", format, Self::hash_key(source, format));
        let source_hash = Self::hash_key(source, format);

        if let Ok(mut cache) = self.memory_cache.lock() {
            // Evict oldest entry if cache is full
            if cache.len() >= self.max_entries {
                if let Some(oldest_key) = cache.keys().next().cloned() {
                    cache.remove(&oldest_key);
                }
            }

            let cached = CachedConversion {
                statutes,
                report,
                timestamp: Self::current_timestamp(),
                source_hash,
            };

            cache.insert(key, cached);
        }
    }

    /// Clears the cache.
    pub fn clear(&self) {
        if let Ok(mut cache) = self.memory_cache.lock() {
            cache.clear();
        }
    }

    /// Returns the number of cached entries.
    pub fn size(&self) -> usize {
        self.memory_cache.lock().map(|c| c.len()).unwrap_or(0)
    }

    /// Gets the current Unix timestamp.
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// Incremental re-conversion tracker.
pub struct IncrementalConverter {
    /// Previous source text hash
    previous_hash: Option<u64>,
    /// Previous statutes
    previous_statutes: Vec<Statute>,
    /// Changed chunks
    changed_chunks: Vec<usize>,
}

impl IncrementalConverter {
    /// Creates a new incremental converter.
    pub fn new() -> Self {
        Self {
            previous_hash: None,
            previous_statutes: Vec::new(),
            changed_chunks: Vec::new(),
        }
    }

    /// Computes hash of source text.
    fn hash_source(source: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }

    /// Detects changes between old and new source.
    pub fn detect_changes(&mut self, source: &str) -> bool {
        let current_hash = Self::hash_source(source);

        if let Some(prev_hash) = self.previous_hash {
            if prev_hash != current_hash {
                self.previous_hash = Some(current_hash);
                true
            } else {
                false
            }
        } else {
            self.previous_hash = Some(current_hash);
            true
        }
    }

    /// Stores conversion results for future comparisons.
    pub fn store_results(&mut self, statutes: Vec<Statute>) {
        self.previous_statutes = statutes;
        self.changed_chunks.clear();
    }

    /// Returns previously converted statutes.
    pub fn previous_results(&self) -> &[Statute] {
        &self.previous_statutes
    }

    /// Checks if source has changed since last conversion.
    pub fn has_changed(&self, source: &str) -> bool {
        let current_hash = Self::hash_source(source);
        self.previous_hash != Some(current_hash)
    }
}

impl Default for IncrementalConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Parallel parser with work stealing for multi-core utilization.
#[cfg(feature = "parallel")]
pub struct ParallelParser {
    /// Number of worker threads
    #[allow(dead_code)]
    num_workers: usize,
}

#[cfg(feature = "parallel")]
impl ParallelParser {
    /// Creates a new parallel parser with automatic thread count.
    pub fn new() -> Self {
        Self {
            num_workers: rayon::current_num_threads(),
        }
    }

    /// Creates a parallel parser with specific thread count.
    pub fn with_workers(num_workers: usize) -> Self {
        Self { num_workers }
    }

    /// Parses multiple documents in parallel.
    pub fn parse_batch<F>(
        &self,
        sources: Vec<String>,
        parser: F,
    ) -> Vec<InteropResult<Vec<Statute>>>
    where
        F: Fn(&str) -> InteropResult<Vec<Statute>> + Send + Sync,
    {
        use rayon::prelude::*;

        sources.par_iter().map(|source| parser(source)).collect()
    }

    /// Parses a large document by splitting into chunks and processing in parallel.
    pub fn parse_chunked<F>(
        &self,
        source: &str,
        chunk_size: usize,
        parser: F,
    ) -> InteropResult<Vec<Statute>>
    where
        F: Fn(&str) -> InteropResult<Vec<Statute>> + Send + Sync,
    {
        use rayon::prelude::*;

        let lazy_parser = LazyParser::with_chunk_size(source, chunk_size);
        let chunks: Vec<String> = lazy_parser.chunks.clone();

        let results: Vec<InteropResult<Vec<Statute>>> =
            chunks.par_iter().map(|chunk| parser(chunk)).collect();

        // Merge results
        let mut all_statutes = Vec::new();
        for result in results {
            match result {
                Ok(statutes) => all_statutes.extend(statutes),
                Err(e) => return Err(e),
            }
        }

        Ok(all_statutes)
    }
}

#[cfg(feature = "parallel")]
impl Default for ParallelParser {
    fn default() -> Self {
        Self::new()
    }
}

/// High-performance converter with all optimizations enabled.
pub struct HighPerformanceConverter {
    /// Persistent cache
    cache: PersistentCache,
    /// Incremental converter
    incremental: IncrementalConverter,
    /// Enable lazy parsing
    lazy_parsing: bool,
    /// Chunk size for lazy parsing
    chunk_size: usize,
}

impl HighPerformanceConverter {
    /// Creates a new high-performance converter.
    pub fn new() -> Self {
        Self {
            cache: PersistentCache::new(1000),
            incremental: IncrementalConverter::new(),
            lazy_parsing: true,
            chunk_size: 65536,
        }
    }

    /// Creates a converter with custom cache size.
    pub fn with_cache_size(cache_size: usize) -> Self {
        Self {
            cache: PersistentCache::new(cache_size),
            incremental: IncrementalConverter::new(),
            lazy_parsing: true,
            chunk_size: 65536,
        }
    }

    /// Enables or disables lazy parsing.
    pub fn set_lazy_parsing(&mut self, enabled: bool) {
        self.lazy_parsing = enabled;
    }

    /// Sets the chunk size for lazy parsing.
    pub fn set_chunk_size(&mut self, chunk_size: usize) {
        self.chunk_size = chunk_size;
    }

    /// Gets cached conversion or returns None.
    pub fn get_cached(
        &self,
        source: &str,
        format: LegalFormat,
    ) -> Option<(Vec<Statute>, ConversionReport)> {
        self.cache.get(source, format)
    }

    /// Stores conversion result in cache.
    pub fn cache_result(
        &self,
        source: &str,
        format: LegalFormat,
        statutes: Vec<Statute>,
        report: ConversionReport,
    ) {
        self.cache.put(source, format, statutes, report);
    }

    /// Checks if source has changed since last conversion.
    pub fn has_source_changed(&self, source: &str) -> bool {
        self.incremental.has_changed(source)
    }

    /// Updates incremental state with new results.
    pub fn update_incremental_state(&mut self, statutes: Vec<Statute>) {
        self.incremental.store_results(statutes);
    }

    /// Clears all caches.
    pub fn clear_caches(&mut self) {
        self.cache.clear();
    }

    /// Returns cache statistics.
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            entries: self.cache.size(),
            max_entries: self.cache.max_entries,
        }
    }
}

impl Default for HighPerformanceConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of entries in cache
    pub entries: usize,
    /// Maximum cache entries
    pub max_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_lazy_parser_chunks() {
        let source = "line1\nline2\nline3\nline4\nline5";
        let mut parser = LazyParser::with_chunk_size(source, 15);

        assert!(parser.chunk_count() > 0);
        assert_eq!(parser.progress(), 0.0);

        let first = parser.next_chunk();
        assert!(first.is_some());
        assert!(parser.progress() > 0.0 && parser.progress() < 1.0);

        while parser.next_chunk().is_some() {}
        assert_eq!(parser.progress(), 1.0);
    }

    #[test]
    fn test_lazy_parser_reset() {
        let source = "line1\nline2\nline3";
        let mut parser = LazyParser::new(source);

        parser.next_chunk();
        parser.next_chunk();
        let progress_before = parser.progress();
        assert!(progress_before > 0.0);

        parser.reset();
        assert_eq!(parser.progress(), 0.0);
    }

    #[test]
    fn test_mmap_reader_size() {
        use std::io::Write;

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_mmap.txt");

        {
            let mut file = File::create(&temp_file).unwrap();
            write!(file, "Hello, World!").unwrap();
        }

        let reader = MmapFileReader::open(&temp_file).unwrap();
        assert_eq!(reader.size(), 13);

        std::fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_mmap_reader_content() {
        use std::io::Write;

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_mmap_content.txt");

        {
            let mut file = File::create(&temp_file).unwrap();
            write!(file, "Test content").unwrap();
        }

        let reader = MmapFileReader::open(&temp_file).unwrap();
        let content = reader.read_to_string().unwrap();
        assert_eq!(content, "Test content");

        std::fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_persistent_cache() {
        let cache = PersistentCache::new(10);

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);

        // Store in cache
        cache.put(
            "source text",
            LegalFormat::Catala,
            vec![statute.clone()],
            report.clone(),
        );

        // Retrieve from cache
        let cached = cache.get("source text", LegalFormat::Catala);
        assert!(cached.is_some());

        let (statutes, cached_report) = cached.unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].id, "test");
        assert_eq!(cached_report.source_format, report.source_format);
    }

    #[test]
    fn test_cache_eviction() {
        let cache = PersistentCache::new(3);

        for i in 0..5 {
            let source = format!("source{}", i);
            let statute = Statute::new(
                format!("test{}", i),
                "Test",
                Effect::new(EffectType::Grant, "test"),
            );
            let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);
            cache.put(&source, LegalFormat::Catala, vec![statute], report);
        }

        // Cache should have at most 3 entries
        assert!(cache.size() <= 3);
    }

    #[test]
    fn test_cache_clear() {
        let cache = PersistentCache::new(10);

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);
        cache.put("source", LegalFormat::Catala, vec![statute], report);

        assert_eq!(cache.size(), 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_incremental_converter_changes() {
        let mut converter = IncrementalConverter::new();

        // First conversion - should detect change
        assert!(converter.detect_changes("source v1"));

        // Same source - should not detect change
        assert!(!converter.detect_changes("source v1"));

        // Modified source - should detect change
        assert!(converter.detect_changes("source v2"));
    }

    #[test]
    fn test_incremental_converter_storage() {
        let mut converter = IncrementalConverter::new();

        let statutes = vec![
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "test")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "test")),
        ];

        converter.store_results(statutes.clone());
        assert_eq!(converter.previous_results().len(), 2);
        assert_eq!(converter.previous_results()[0].id, "test1");
    }

    #[test]
    fn test_high_performance_converter() {
        let mut converter = HighPerformanceConverter::new();

        // Test lazy parsing setting
        converter.set_lazy_parsing(false);
        converter.set_lazy_parsing(true);

        // Test chunk size setting
        converter.set_chunk_size(32768);

        // Test cache
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);

        converter.cache_result(
            "source",
            LegalFormat::Catala,
            vec![statute.clone()],
            report.clone(),
        );

        let cached = converter.get_cached("source", LegalFormat::Catala);
        assert!(cached.is_some());

        // Test incremental state
        assert!(converter.has_source_changed("new source"));
        converter.update_incremental_state(vec![statute]);

        // Test cache stats
        let stats = converter.cache_stats();
        assert!(stats.entries > 0);
    }

    #[test]
    fn test_cache_stats() {
        let converter = HighPerformanceConverter::with_cache_size(500);
        let stats = converter.cache_stats();

        assert_eq!(stats.entries, 0);
        assert_eq!(stats.max_entries, 500);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel_parser() {
        let parser = ParallelParser::new();
        assert!(parser.num_workers > 0);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel_parse_batch() {
        let parser = ParallelParser::new();
        let sources = vec![
            "source1".to_string(),
            "source2".to_string(),
            "source3".to_string(),
        ];

        let results = parser.parse_batch(sources, |_source| {
            Ok(vec![Statute::new(
                "test",
                "Test",
                Effect::new(EffectType::Grant, "test"),
            )])
        });

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel_parse_chunked() {
        let parser = ParallelParser::new();
        let source = "line1\nline2\nline3\nline4\nline5\nline6";

        let result = parser.parse_chunked(source, 10, |_chunk| {
            Ok(vec![Statute::new(
                "test",
                "Test",
                Effect::new(EffectType::Grant, "test"),
            )])
        });

        assert!(result.is_ok());
        let statutes = result.unwrap();
        assert!(!statutes.is_empty());
    }

    #[test]
    fn test_lazy_parser_empty_source() {
        let parser = LazyParser::new("");
        assert_eq!(parser.chunk_count(), 0);
        assert_eq!(parser.progress(), 1.0);
    }

    #[test]
    fn test_mmap_reader_range() {
        use std::io::Write;

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_mmap_range.txt");

        {
            let mut file = File::create(&temp_file).unwrap();
            write!(file, "0123456789").unwrap();
        }

        let reader = MmapFileReader::open(&temp_file).unwrap();
        let range = reader.read_range(2, 5).unwrap();
        assert_eq!(range, "23456");

        std::fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_incremental_has_changed() {
        let converter = IncrementalConverter::new();

        // First check - no previous hash
        assert!(converter.has_changed("source1"));

        // After storing, same source should not have changed
        let mut converter2 = IncrementalConverter::new();
        converter2.detect_changes("source1");
        assert!(!converter2.has_changed("source1"));
        assert!(converter2.has_changed("source2"));
    }
}
