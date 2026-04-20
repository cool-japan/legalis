use super::*;

/// Facet type for search aggregations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacetType {
    /// Status facet
    Status,
    /// Jurisdiction facet
    Jurisdiction,
    /// Tags facet
    Tags,
    /// Year (from effective date)
    Year,
    /// Month (from effective date)
    Month,
    /// Custom facet
    Custom(String),
}

/// Facet value with count.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetValue {
    /// Value of the facet
    pub value: String,
    /// Count of items with this value
    pub count: usize,
}

/// Facet result for a specific facet type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetResult {
    /// Facet type
    pub facet_type: FacetType,
    /// Values with their counts
    pub values: Vec<FacetValue>,
    /// Total number of unique values
    pub total_values: usize,
}

impl FacetResult {
    /// Gets top N values by count.
    pub fn top_values(&self, n: usize) -> Vec<&FacetValue> {
        let mut sorted: Vec<_> = self.values.iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.count));
        sorted.into_iter().take(n).collect()
    }

    /// Finds a specific value.
    pub fn find_value(&self, value: &str) -> Option<&FacetValue> {
        self.values.iter().find(|v| v.value == value)
    }
}

/// Faceted search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetedSearchResult {
    /// Matching statute IDs
    pub statute_ids: Vec<String>,
    /// Facet results
    pub facets: Vec<FacetResult>,
    /// Total matches
    pub total_matches: usize,
}

/// Search suggestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    /// Suggested text
    pub text: String,
    /// Suggestion type
    pub suggestion_type: SuggestionType,
    /// Relevance score
    pub score: f64,
}

/// Type of search suggestion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestionType {
    /// Statute ID
    StatuteId,
    /// Statute title
    Title,
    /// Tag
    Tag,
    /// Jurisdiction
    Jurisdiction,
    /// General term
    Term,
}

/// Autocomplete provider.
#[derive(Debug)]
pub struct AutocompleteProvider {
    /// Index of statute IDs
    statute_ids: Vec<String>,
    /// Index of titles
    titles: Vec<String>,
    /// Index of tags
    tags: Vec<String>,
    /// Index of jurisdictions
    jurisdictions: Vec<String>,
}

impl AutocompleteProvider {
    /// Creates a new autocomplete provider.
    pub fn new() -> Self {
        Self {
            statute_ids: Vec::new(),
            titles: Vec::new(),
            tags: Vec::new(),
            jurisdictions: Vec::new(),
        }
    }

    /// Indexes a statute for autocomplete.
    pub fn index_statute(&mut self, entry: &StatuteEntry) {
        // Index statute ID
        if !self.statute_ids.contains(&entry.statute.id) {
            self.statute_ids.push(entry.statute.id.clone());
        }

        // Index title
        let title = entry.statute.title.clone();
        if !self.titles.contains(&title) {
            self.titles.push(title);
        }

        // Index tags
        for tag in &entry.tags {
            if !self.tags.contains(tag) {
                self.tags.push(tag.clone());
            }
        }

        // Index jurisdiction
        if !self.jurisdictions.contains(&entry.jurisdiction) {
            self.jurisdictions.push(entry.jurisdiction.clone());
        }
    }

    /// Gets suggestions for a query.
    pub fn suggest(&self, query: &str, max_results: usize) -> Vec<SearchSuggestion> {
        let query_lower = query.to_lowercase();
        let mut suggestions = Vec::new();

        // Search statute IDs
        for id in &self.statute_ids {
            if id.to_lowercase().contains(&query_lower) {
                suggestions.push(SearchSuggestion {
                    text: id.clone(),
                    suggestion_type: SuggestionType::StatuteId,
                    score: Self::calculate_score(&query_lower, &id.to_lowercase()),
                });
            }
        }

        // Search titles
        for title in &self.titles {
            if title.to_lowercase().contains(&query_lower) {
                suggestions.push(SearchSuggestion {
                    text: title.clone(),
                    suggestion_type: SuggestionType::Title,
                    score: Self::calculate_score(&query_lower, &title.to_lowercase()),
                });
            }
        }

        // Search tags
        for tag in &self.tags {
            if tag.to_lowercase().contains(&query_lower) {
                suggestions.push(SearchSuggestion {
                    text: tag.clone(),
                    suggestion_type: SuggestionType::Tag,
                    score: Self::calculate_score(&query_lower, &tag.to_lowercase()),
                });
            }
        }

        // Search jurisdictions
        for jurisdiction in &self.jurisdictions {
            if jurisdiction.to_lowercase().contains(&query_lower) {
                suggestions.push(SearchSuggestion {
                    text: jurisdiction.clone(),
                    suggestion_type: SuggestionType::Jurisdiction,
                    score: Self::calculate_score(&query_lower, &jurisdiction.to_lowercase()),
                });
            }
        }

        // Sort by score (descending)
        suggestions.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(max_results);
        suggestions
    }

    /// Calculates relevance score.
    fn calculate_score(query: &str, text: &str) -> f64 {
        // Exact match gets highest score
        if query == text {
            return 1.0;
        }

        // Prefix match gets high score
        if text.starts_with(query) {
            return 0.9;
        }

        // Contains match gets medium score
        if text.contains(query) {
            return 0.7;
        }

        // Fuzzy match gets lower score
        0.5
    }
}

impl Default for AutocompleteProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Saved search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearch {
    /// Search ID
    pub search_id: Uuid,
    /// Search name
    pub name: String,
    /// Search query
    pub query: SearchQuery,
    /// Owner user ID
    pub owner: String,
    /// Alert enabled
    pub alert_enabled: bool,
    /// Alert frequency in seconds
    pub alert_frequency_seconds: Option<i64>,
    /// Last executed
    pub last_executed: Option<DateTime<Utc>>,
    /// Last result count
    pub last_result_count: Option<usize>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

impl SavedSearch {
    /// Creates a new saved search.
    pub fn new(name: impl Into<String>, query: SearchQuery, owner: impl Into<String>) -> Self {
        Self {
            search_id: Uuid::new_v4(),
            name: name.into(),
            query,
            owner: owner.into(),
            alert_enabled: false,
            alert_frequency_seconds: None,
            last_executed: None,
            last_result_count: None,
            created_at: Utc::now(),
        }
    }

    /// Enables alerts with frequency.
    pub fn with_alert(mut self, frequency_seconds: i64) -> Self {
        self.alert_enabled = true;
        self.alert_frequency_seconds = Some(frequency_seconds);
        self
    }

    /// Checks if alert should be triggered.
    pub fn should_trigger_alert(&self) -> bool {
        if !self.alert_enabled {
            return false;
        }

        if let Some(freq) = self.alert_frequency_seconds {
            if let Some(last_exec) = self.last_executed {
                let elapsed = Utc::now() - last_exec;
                return elapsed.num_seconds() >= freq;
            }
            // Never executed, should trigger
            return true;
        }

        false
    }

    /// Updates execution info.
    pub fn update_execution(&mut self, result_count: usize) {
        self.last_executed = Some(Utc::now());
        self.last_result_count = Some(result_count);
    }
}

/// Search analytics tracker.
#[derive(Debug)]
pub struct SearchAnalytics {
    /// Query frequency tracking
    query_counts: HashMap<String, usize>,
    /// Recent searches
    recent_searches: Vec<(String, DateTime<Utc>)>,
    /// Search result counts
    result_counts: Vec<usize>,
    /// Max recent searches to track
    max_recent: usize,
}

impl SearchAnalytics {
    /// Creates a new search analytics tracker.
    pub fn new() -> Self {
        Self {
            query_counts: HashMap::new(),
            recent_searches: Vec::new(),
            result_counts: Vec::new(),
            max_recent: 1000,
        }
    }

    /// Records a search.
    pub fn record_search(&mut self, query: &str, result_count: usize) {
        // Track query frequency
        *self.query_counts.entry(query.to_string()).or_insert(0) += 1;

        // Track recent searches
        self.recent_searches.push((query.to_string(), Utc::now()));
        if self.recent_searches.len() > self.max_recent {
            self.recent_searches
                .drain(0..self.recent_searches.len() - self.max_recent);
        }

        // Track result counts
        self.result_counts.push(result_count);
    }

    /// Gets most popular queries.
    pub fn top_queries(&self, n: usize) -> Vec<(String, usize)> {
        let mut queries: Vec<_> = self
            .query_counts
            .iter()
            .map(|(q, c)| (q.clone(), *c))
            .collect();
        queries.sort_by_key(|b| std::cmp::Reverse(b.1));
        queries.into_iter().take(n).collect()
    }

    /// Gets average result count.
    pub fn average_result_count(&self) -> f64 {
        if self.result_counts.is_empty() {
            return 0.0;
        }
        let sum: usize = self.result_counts.iter().sum();
        sum as f64 / self.result_counts.len() as f64
    }

    /// Gets zero-result queries.
    pub fn zero_result_queries(&self) -> Vec<String> {
        self.recent_searches
            .iter()
            .enumerate()
            .filter(|(i, _)| self.result_counts.get(*i).map(|&c| c == 0).unwrap_or(false))
            .map(|(_, (q, _))| q.clone())
            .collect()
    }

    /// Gets total searches.
    pub fn total_searches(&self) -> usize {
        self.recent_searches.len()
    }

    /// Gets searches in time range.
    pub fn searches_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> usize {
        self.recent_searches
            .iter()
            .filter(|(_, ts)| ts >= &start && ts <= &end)
            .count()
    }
}

impl Default for SearchAnalytics {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic search using embeddings (placeholder for future ML integration).
#[derive(Debug)]
pub struct SemanticSearch {
    /// Enabled flag
    enabled: bool,
    /// Embedding dimension
    dimension: usize,
}

impl SemanticSearch {
    /// Creates a new semantic search engine.
    pub fn new(dimension: usize) -> Self {
        Self {
            enabled: false,
            dimension,
        }
    }

    /// Enables semantic search.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Checks if enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Gets embedding dimension.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Placeholder for semantic search (would integrate with ML models).
    pub fn search(&self, _query: &str, _top_k: usize) -> Vec<(String, f64)> {
        // In a real implementation, this would:
        // 1. Generate embedding for query
        // 2. Search vector database for similar embeddings
        // 3. Return statute IDs with similarity scores
        Vec::new()
    }
}

impl Default for SemanticSearch {
    fn default() -> Self {
        Self::new(384) // Default BERT dimension
    }
}
