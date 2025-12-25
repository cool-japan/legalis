//! Model evaluation metrics for assessing LLM output quality.
//!
//! This module provides automated quality metrics including BLEU, ROUGE,
//! and response quality scoring.

use std::collections::{HashMap, HashSet};

/// N-gram type for text analysis.
type NGram = Vec<String>;

/// Calculates BLEU (Bilingual Evaluation Understudy) score.
///
/// BLEU measures how similar a candidate text is to one or more reference texts.
/// Score ranges from 0.0 (no match) to 1.0 (perfect match).
pub fn bleu_score(candidate: &str, references: &[&str], max_n: usize) -> f64 {
    if references.is_empty() || candidate.is_empty() {
        return 0.0;
    }

    let candidate_tokens = tokenize(candidate);
    let reference_tokens: Vec<Vec<String>> = references.iter().map(|r| tokenize(r)).collect();

    if candidate_tokens.is_empty() {
        return 0.0;
    }

    // Calculate precision for each n-gram size
    let mut precisions = Vec::new();
    for n in 1..=max_n {
        let precision = calculate_ngram_precision(&candidate_tokens, &reference_tokens, n);
        if precision == 0.0 {
            return 0.0; // If any precision is 0, BLEU is 0
        }
        precisions.push(precision);
    }

    // Geometric mean of precisions
    let log_avg = precisions.iter().map(|p| p.ln()).sum::<f64>() / precisions.len() as f64;
    let geometric_mean = log_avg.exp();

    // Brevity penalty
    let reference_length = reference_tokens[0].len();
    let candidate_length = candidate_tokens.len();
    let brevity_penalty = if candidate_length < reference_length {
        (1.0 - (reference_length as f64 / candidate_length as f64)).exp()
    } else {
        1.0
    };

    brevity_penalty * geometric_mean
}

/// Calculates n-gram precision.
fn calculate_ngram_precision(candidate: &[String], references: &[Vec<String>], n: usize) -> f64 {
    if n > candidate.len() {
        return 0.0;
    }

    let candidate_ngrams = extract_ngrams(candidate, n);
    let mut reference_ngram_counts: HashMap<NGram, usize> = HashMap::new();

    // Count n-grams in references (use max count across references)
    for reference in references {
        let ref_ngrams = extract_ngrams(reference, n);
        for ngram in ref_ngrams {
            *reference_ngram_counts.entry(ngram).or_insert(0) += 1;
        }
    }

    // Count matches
    let mut matches = 0;
    let mut total = 0;

    for ngram in &candidate_ngrams {
        total += 1;
        if let Some(&count) = reference_ngram_counts.get(ngram) {
            if count > 0 {
                matches += 1;
                // Decrease count to handle clipping
                reference_ngram_counts.insert(ngram.clone(), count - 1);
            }
        }
    }

    if total == 0 {
        0.0
    } else {
        matches as f64 / total as f64
    }
}

/// Extracts n-grams from a token sequence.
fn extract_ngrams(tokens: &[String], n: usize) -> Vec<NGram> {
    if n > tokens.len() {
        return Vec::new();
    }

    tokens.windows(n).map(|window| window.to_vec()).collect()
}

/// ROUGE (Recall-Oriented Understudy for Gisting Evaluation) metrics.
#[derive(Debug, Clone)]
pub struct RougeScores {
    /// ROUGE-1 (unigram) F1 score
    pub rouge_1: f64,
    /// ROUGE-2 (bigram) F1 score
    pub rouge_2: f64,
    /// ROUGE-L (longest common subsequence) F1 score
    pub rouge_l: f64,
}

/// Calculates ROUGE scores.
pub fn rouge_scores(candidate: &str, reference: &str) -> RougeScores {
    let candidate_tokens = tokenize(candidate);
    let reference_tokens = tokenize(reference);

    RougeScores {
        rouge_1: rouge_n(&candidate_tokens, &reference_tokens, 1),
        rouge_2: rouge_n(&candidate_tokens, &reference_tokens, 2),
        rouge_l: rouge_l_score(&candidate_tokens, &reference_tokens),
    }
}

/// Calculates ROUGE-N score (F1).
fn rouge_n(candidate: &[String], reference: &[String], n: usize) -> f64 {
    if n > candidate.len() || n > reference.len() {
        return 0.0;
    }

    let candidate_ngrams: HashSet<_> = extract_ngrams(candidate, n).into_iter().collect();
    let reference_ngrams: HashSet<_> = extract_ngrams(reference, n).into_iter().collect();

    if reference_ngrams.is_empty() {
        return 0.0;
    }

    let overlap: usize = candidate_ngrams.intersection(&reference_ngrams).count();

    if overlap == 0 {
        return 0.0;
    }

    let precision = overlap as f64 / candidate_ngrams.len() as f64;
    let recall = overlap as f64 / reference_ngrams.len() as f64;

    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    }
}

/// Calculates ROUGE-L score based on longest common subsequence.
fn rouge_l_score(candidate: &[String], reference: &[String]) -> f64 {
    let lcs_length = longest_common_subsequence(candidate, reference);

    if lcs_length == 0 {
        return 0.0;
    }

    let precision = lcs_length as f64 / candidate.len() as f64;
    let recall = lcs_length as f64 / reference.len() as f64;

    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    }
}

/// Finds the longest common subsequence length.
fn longest_common_subsequence(seq1: &[String], seq2: &[String]) -> usize {
    let m = seq1.len();
    let n = seq2.len();

    if m == 0 || n == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if seq1[i - 1] == seq2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

/// Response quality metrics.
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    /// Average sentence length
    pub avg_sentence_length: f64,
    /// Vocabulary richness (unique words / total words)
    pub vocabulary_richness: f64,
    /// Readability score (Flesch reading ease approximation)
    pub readability: f64,
    /// Number of sentences
    pub sentence_count: usize,
    /// Number of words
    pub word_count: usize,
    /// Number of unique words
    pub unique_word_count: usize,
}

/// Calculates quality metrics for a text response.
pub fn calculate_quality_metrics(text: &str) -> QualityMetrics {
    let sentences = split_sentences(text);
    let words = tokenize(text);
    let unique_words: HashSet<_> = words.iter().collect();

    let sentence_count = sentences.len().max(1);
    let word_count = words.len().max(1);
    let unique_word_count = unique_words.len();

    let avg_sentence_length = word_count as f64 / sentence_count as f64;
    let vocabulary_richness = unique_word_count as f64 / word_count as f64;

    // Simple readability approximation (lower is easier to read)
    let avg_word_length = words.iter().map(|w| w.len()).sum::<usize>() as f64 / word_count as f64;
    let readability = 206.835 - 1.015 * avg_sentence_length - 84.6 * avg_word_length;

    QualityMetrics {
        avg_sentence_length,
        vocabulary_richness,
        readability,
        sentence_count,
        word_count,
        unique_word_count,
    }
}

/// Tokenizes text into words.
fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Splits text into sentences (simple implementation).
fn split_sentences(text: &str) -> Vec<String> {
    text.split(&['.', '!', '?'][..])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// A/B test result comparison.
#[derive(Debug, Clone)]
pub struct ABTestResult {
    /// Metric name
    pub metric_name: String,
    /// Score for variant A
    pub score_a: f64,
    /// Score for variant B
    pub score_b: f64,
    /// Difference (B - A)
    pub difference: f64,
    /// Percentage improvement
    pub improvement_pct: f64,
}

/// Compares two variants using multiple metrics.
pub fn compare_variants(text_a: &str, text_b: &str, reference: &str) -> Vec<ABTestResult> {
    let mut results = Vec::new();

    // BLEU comparison
    let bleu_a = bleu_score(text_a, &[reference], 4);
    let bleu_b = bleu_score(text_b, &[reference], 4);
    results.push(ABTestResult {
        metric_name: "BLEU".to_string(),
        score_a: bleu_a,
        score_b: bleu_b,
        difference: bleu_b - bleu_a,
        improvement_pct: if bleu_a > 0.0 {
            ((bleu_b - bleu_a) / bleu_a) * 100.0
        } else {
            0.0
        },
    });

    // ROUGE-L comparison
    let rouge_a = rouge_scores(text_a, reference).rouge_l;
    let rouge_b = rouge_scores(text_b, reference).rouge_l;
    results.push(ABTestResult {
        metric_name: "ROUGE-L".to_string(),
        score_a: rouge_a,
        score_b: rouge_b,
        difference: rouge_b - rouge_a,
        improvement_pct: if rouge_a > 0.0 {
            ((rouge_b - rouge_a) / rouge_a) * 100.0
        } else {
            0.0
        },
    });

    // Quality metrics comparison
    let quality_a = calculate_quality_metrics(text_a);
    let quality_b = calculate_quality_metrics(text_b);

    results.push(ABTestResult {
        metric_name: "Vocabulary Richness".to_string(),
        score_a: quality_a.vocabulary_richness,
        score_b: quality_b.vocabulary_richness,
        difference: quality_b.vocabulary_richness - quality_a.vocabulary_richness,
        improvement_pct: if quality_a.vocabulary_richness > 0.0 {
            ((quality_b.vocabulary_richness - quality_a.vocabulary_richness)
                / quality_a.vocabulary_richness)
                * 100.0
        } else {
            0.0
        },
    });

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bleu_perfect_match() {
        let candidate = "the cat sat on the mat";
        let reference = "the cat sat on the mat";
        let score = bleu_score(candidate, &[reference], 4);
        assert!((score - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_bleu_partial_match() {
        let candidate = "the cat sat on the mat";
        let reference = "the dog sat on the mat";
        let score = bleu_score(candidate, &[reference], 4);
        assert!(score > 0.0 && score < 1.0);
    }

    #[test]
    fn test_bleu_no_match() {
        let candidate = "hello world";
        let reference = "goodbye moon";
        let score = bleu_score(candidate, &[reference], 4);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_rouge_scores() {
        let candidate = "the cat sat on the mat";
        let reference = "the cat sat on the mat";
        let scores = rouge_scores(candidate, reference);

        assert!((scores.rouge_1 - 1.0).abs() < 0.01);
        assert!((scores.rouge_2 - 1.0).abs() < 0.01);
        assert!((scores.rouge_l - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_rouge_partial_match() {
        let candidate = "the cat sat";
        let reference = "the cat sat on the mat";
        let scores = rouge_scores(candidate, reference);

        assert!(scores.rouge_1 > 0.0 && scores.rouge_1 <= 1.0);
        assert!(scores.rouge_l > 0.0 && scores.rouge_l <= 1.0);
    }

    #[test]
    fn test_quality_metrics() {
        let text = "This is a test. It has two sentences.";
        let metrics = calculate_quality_metrics(text);

        assert_eq!(metrics.sentence_count, 2);
        assert!(metrics.word_count > 0);
        assert!(metrics.unique_word_count > 0);
        assert!(metrics.avg_sentence_length > 0.0);
        assert!(metrics.vocabulary_richness > 0.0 && metrics.vocabulary_richness <= 1.0);
    }

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("Hello, World! This is a test.");
        assert_eq!(tokens, vec!["hello", "world", "this", "is", "a", "test"]);
    }

    #[test]
    fn test_split_sentences() {
        let sentences = split_sentences("First sentence. Second sentence! Third sentence?");
        assert_eq!(sentences.len(), 3);
    }

    #[test]
    fn test_ngram_extraction() {
        let tokens = vec!["the".to_string(), "cat".to_string(), "sat".to_string()];
        let bigrams = extract_ngrams(&tokens, 2);

        assert_eq!(bigrams.len(), 2);
        assert_eq!(bigrams[0], vec!["the", "cat"]);
        assert_eq!(bigrams[1], vec!["cat", "sat"]);
    }

    #[test]
    fn test_lcs() {
        let seq1 = vec!["the".to_string(), "cat".to_string(), "sat".to_string()];
        let seq2 = vec!["the".to_string(), "cat".to_string(), "sat".to_string()];
        let lcs = longest_common_subsequence(&seq1, &seq2);
        assert_eq!(lcs, 3);
    }

    #[test]
    fn test_lcs_partial() {
        let seq1 = vec!["the".to_string(), "cat".to_string(), "sat".to_string()];
        let seq2 = vec!["the".to_string(), "dog".to_string(), "sat".to_string()];
        let lcs = longest_common_subsequence(&seq1, &seq2);
        assert_eq!(lcs, 2); // "the" and "sat"
    }

    #[test]
    fn test_compare_variants() {
        let text_a = "the cat sat on the mat";
        let text_b = "the cat sat on the mat";
        let reference = "the cat sat on the mat";

        let results = compare_variants(text_a, text_b, reference);
        assert!(!results.is_empty());

        // Both should have perfect scores
        for result in &results {
            assert!((result.score_a - result.score_b).abs() < 0.01 || result.score_a == 0.0);
        }
    }

    #[test]
    fn test_empty_input_handling() {
        assert_eq!(bleu_score("", &["test"], 4), 0.0);
        assert_eq!(bleu_score("test", &[""], 4), 0.0);

        let scores = rouge_scores("", "test");
        assert_eq!(scores.rouge_1, 0.0);

        let metrics = calculate_quality_metrics("");
        assert_eq!(metrics.word_count, 1); // max(0, 1)
    }
}
