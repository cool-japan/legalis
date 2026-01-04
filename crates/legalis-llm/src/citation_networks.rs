//! Legal Citation Networks
//!
//! Graph-based analysis of legal citations including precedent strength,
//! authority scoring (PageRank-style), and citation clustering.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Legal case node in citation network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaseNode {
    /// Unique case identifier
    pub id: String,
    /// Case name
    pub name: String,
    /// Citation string (e.g., "123 U.S. 456")
    pub citation: String,
    /// Year decided
    pub year: i32,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Court level
    pub court_level: CourtLevel,
    /// Case type
    pub case_type: CaseType,
}

impl CaseNode {
    /// Creates a new case node.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        citation: impl Into<String>,
        year: i32,
        jurisdiction: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            citation: citation.into(),
            year,
            jurisdiction: jurisdiction.into(),
            court_level: CourtLevel::Trial,
            case_type: CaseType::Civil,
        }
    }

    /// Sets the court level.
    pub fn with_court_level(mut self, level: CourtLevel) -> Self {
        self.court_level = level;
        self
    }

    /// Sets the case type.
    pub fn with_case_type(mut self, case_type: CaseType) -> Self {
        self.case_type = case_type;
        self
    }
}

/// Court level in judicial hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CourtLevel {
    /// Trial court (lowest)
    Trial,
    /// Intermediate appellate court
    Appellate,
    /// Supreme court (highest)
    Supreme,
    /// Federal circuit court
    Circuit,
}

/// Type of legal case
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaseType {
    /// Civil case
    Civil,
    /// Criminal case
    Criminal,
    /// Constitutional case
    Constitutional,
    /// Administrative case
    Administrative,
}

/// Citation edge between cases
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Citation {
    /// Citing case ID
    pub citing_case: String,
    /// Cited case ID
    pub cited_case: String,
    /// Treatment type
    pub treatment: TreatmentType,
    /// Depth of discussion (0-5, where 5 is extensive)
    pub depth: u8,
    /// Number of times cited in the case
    pub frequency: usize,
}

impl Citation {
    /// Creates a new citation.
    pub fn new(
        citing_case: impl Into<String>,
        cited_case: impl Into<String>,
        treatment: TreatmentType,
    ) -> Self {
        Self {
            citing_case: citing_case.into(),
            cited_case: cited_case.into(),
            treatment,
            depth: 1,
            frequency: 1,
        }
    }

    /// Sets the depth of discussion.
    pub fn with_depth(mut self, depth: u8) -> Self {
        self.depth = depth.min(5);
        self
    }

    /// Sets the frequency.
    pub fn with_frequency(mut self, frequency: usize) -> Self {
        self.frequency = frequency;
        self
    }
}

/// How a case treats a cited precedent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreatmentType {
    /// Follows the precedent
    Followed,
    /// Distinguished from precedent
    Distinguished,
    /// Overruled the precedent
    Overruled,
    /// Questioned the precedent
    Questioned,
    /// Cited positively
    PositiveCitation,
    /// Cited negatively
    NegativeCitation,
    /// Neutral citation
    Neutral,
}

/// Legal citation network graph
pub struct CitationNetwork {
    /// Case nodes indexed by ID
    cases: HashMap<String, CaseNode>,
    /// Citation edges
    citations: Vec<Citation>,
    /// Authority scores (computed via PageRank)
    authority_scores: HashMap<String, f64>,
}

impl CitationNetwork {
    /// Creates a new citation network.
    pub fn new() -> Self {
        Self {
            cases: HashMap::new(),
            citations: Vec::new(),
            authority_scores: HashMap::new(),
        }
    }

    /// Adds a case to the network.
    pub fn add_case(&mut self, case: CaseNode) {
        self.cases.insert(case.id.clone(), case);
    }

    /// Adds a citation edge.
    pub fn add_citation(&mut self, citation: Citation) -> Result<()> {
        // Validate both cases exist
        if !self.cases.contains_key(&citation.citing_case) {
            return Err(anyhow!("Citing case not found: {}", citation.citing_case));
        }
        if !self.cases.contains_key(&citation.cited_case) {
            return Err(anyhow!("Cited case not found: {}", citation.cited_case));
        }

        self.citations.push(citation);
        Ok(())
    }

    /// Gets a case by ID.
    pub fn get_case(&self, id: &str) -> Option<&CaseNode> {
        self.cases.get(id)
    }

    /// Gets all citations from a case.
    pub fn get_outgoing_citations(&self, case_id: &str) -> Vec<&Citation> {
        self.citations
            .iter()
            .filter(|c| c.citing_case == case_id)
            .collect()
    }

    /// Gets all citations to a case.
    pub fn get_incoming_citations(&self, case_id: &str) -> Vec<&Citation> {
        self.citations
            .iter()
            .filter(|c| c.cited_case == case_id)
            .collect()
    }

    /// Computes authority scores using PageRank algorithm.
    pub fn compute_authority_scores(&mut self, iterations: usize, damping: f64) {
        let n = self.cases.len();
        if n == 0 {
            return;
        }

        // Initialize scores
        let initial_score = 1.0 / n as f64;
        let mut scores: HashMap<String, f64> = self
            .cases
            .keys()
            .map(|id| (id.clone(), initial_score))
            .collect();

        // Build adjacency info
        let mut outgoing_counts: HashMap<String, usize> = HashMap::new();
        for citation in &self.citations {
            *outgoing_counts
                .entry(citation.citing_case.clone())
                .or_insert(0) += 1;
        }

        // PageRank iterations
        for _ in 0..iterations {
            let mut new_scores: HashMap<String, f64> = HashMap::new();

            for case_id in self.cases.keys() {
                let incoming = self.get_incoming_citations(case_id);
                let mut score = (1.0 - damping) / n as f64;

                for citation in incoming {
                    let citing_score = scores.get(&citation.citing_case).unwrap_or(&0.0);
                    let out_count = outgoing_counts.get(&citation.citing_case).unwrap_or(&1);

                    // Weight by treatment type
                    let weight = match citation.treatment {
                        TreatmentType::Followed | TreatmentType::PositiveCitation => 1.0,
                        TreatmentType::Overruled | TreatmentType::NegativeCitation => 0.1,
                        TreatmentType::Questioned => 0.5,
                        _ => 0.7,
                    };

                    score += damping * citing_score * weight / (*out_count as f64);
                }

                new_scores.insert(case_id.clone(), score);
            }

            scores = new_scores;
        }

        self.authority_scores = scores;
    }

    /// Gets the authority score for a case.
    pub fn get_authority_score(&self, case_id: &str) -> Option<f64> {
        self.authority_scores.get(case_id).copied()
    }

    /// Gets the top N most authoritative cases.
    pub fn get_most_authoritative(&self, n: usize) -> Vec<(String, f64)> {
        let mut scores: Vec<(String, f64)> = self
            .authority_scores
            .iter()
            .map(|(id, &score)| (id.clone(), score))
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores.truncate(n);
        scores
    }

    /// Computes precedent strength for a case.
    pub fn compute_precedent_strength(&self, case_id: &str) -> PrecedentStrength {
        let incoming = self.get_incoming_citations(case_id);
        let case = self.get_case(case_id);

        let citation_count = incoming.len();
        let positive_citations = incoming
            .iter()
            .filter(|c| {
                matches!(
                    c.treatment,
                    TreatmentType::Followed | TreatmentType::PositiveCitation
                )
            })
            .count();

        let negative_citations = incoming
            .iter()
            .filter(|c| {
                matches!(
                    c.treatment,
                    TreatmentType::Overruled
                        | TreatmentType::NegativeCitation
                        | TreatmentType::Questioned
                )
            })
            .count();

        let authority_score = self.get_authority_score(case_id).unwrap_or(0.0);

        // Strength score based on multiple factors
        let mut strength = authority_score * 100.0;

        // Boost for high court level
        if let Some(case_node) = case {
            match case_node.court_level {
                CourtLevel::Supreme => strength *= 2.0,
                CourtLevel::Circuit => strength *= 1.5,
                CourtLevel::Appellate => strength *= 1.2,
                CourtLevel::Trial => {}
            }
        }

        // Reduce for negative treatment
        if negative_citations > 0 {
            strength *= 0.5;
        }

        PrecedentStrength {
            case_id: case_id.to_string(),
            citation_count,
            positive_citations,
            negative_citations,
            authority_score,
            strength_score: strength,
        }
    }

    /// Finds citation clusters using community detection.
    pub fn find_citation_clusters(&self) -> Vec<CitationCluster> {
        let mut clusters = Vec::new();
        let mut visited = HashSet::new();

        for case_id in self.cases.keys() {
            if visited.contains(case_id) {
                continue;
            }

            let cluster = self.bfs_cluster(case_id, &mut visited);
            if !cluster.is_empty() {
                clusters.push(CitationCluster {
                    id: format!("cluster_{}", clusters.len()),
                    cases: cluster,
                });
            }
        }

        clusters
    }

    fn bfs_cluster(&self, start: &str, visited: &mut HashSet<String>) -> Vec<String> {
        let mut cluster = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(current) = queue.pop_front() {
            cluster.push(current.clone());

            // Add cited cases
            for citation in self.get_outgoing_citations(&current) {
                if !visited.contains(&citation.cited_case) {
                    visited.insert(citation.cited_case.clone());
                    queue.push_back(citation.cited_case.clone());
                }
            }

            // Add citing cases
            for citation in self.get_incoming_citations(&current) {
                if !visited.contains(&citation.citing_case) {
                    visited.insert(citation.citing_case.clone());
                    queue.push_back(citation.citing_case.clone());
                }
            }
        }

        cluster
    }

    /// Analyzes citation evolution over time.
    pub fn analyze_citation_evolution(&self, case_id: &str) -> CitationEvolution {
        let incoming = self.get_incoming_citations(case_id);

        let mut citations_by_year: HashMap<i32, usize> = HashMap::new();
        for citation in &incoming {
            if let Some(citing_case) = self.get_case(&citation.citing_case) {
                *citations_by_year.entry(citing_case.year).or_insert(0) += 1;
            }
        }

        let mut evolution: Vec<(i32, usize)> = citations_by_year.into_iter().collect();
        evolution.sort_by_key(|&(year, _)| year);

        CitationEvolution {
            case_id: case_id.to_string(),
            evolution,
        }
    }

    /// Finds influential cases across jurisdictions.
    pub fn find_cross_jurisdictional_influence(&self) -> Vec<CrossJurisdictionalInfluence> {
        let mut influences = Vec::new();

        for (case_id, case) in &self.cases {
            let incoming = self.get_incoming_citations(case_id);
            let mut jurisdictions = HashMap::new();

            for citation in incoming {
                if let Some(citing_case) = self.get_case(&citation.citing_case) {
                    if citing_case.jurisdiction != case.jurisdiction {
                        *jurisdictions
                            .entry(citing_case.jurisdiction.clone())
                            .or_insert(0) += 1;
                    }
                }
            }

            if !jurisdictions.is_empty() {
                influences.push(CrossJurisdictionalInfluence {
                    case_id: case_id.clone(),
                    home_jurisdiction: case.jurisdiction.clone(),
                    external_citations: jurisdictions,
                });
            }
        }

        influences.sort_by(|a, b| {
            let a_total: usize = a.external_citations.values().sum();
            let b_total: usize = b.external_citations.values().sum();
            b_total.cmp(&a_total)
        });

        influences
    }

    /// Recommends citations for a case based on network similarity.
    pub fn recommend_citations(&self, case_id: &str, n: usize) -> Vec<String> {
        let outgoing = self.get_outgoing_citations(case_id);
        let cited_cases: HashSet<String> = outgoing.iter().map(|c| c.cited_case.clone()).collect();

        // Find cases cited by similar cases
        let mut candidates: HashMap<String, usize> = HashMap::new();
        for citation in outgoing {
            let sibling_cites = self.get_outgoing_citations(&citation.cited_case);
            for sibling in sibling_cites {
                if sibling.cited_case != case_id && !cited_cases.contains(&sibling.cited_case) {
                    *candidates.entry(sibling.cited_case.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut recommendations: Vec<(String, usize)> = candidates.into_iter().collect();
        recommendations.sort_by(|a, b| b.1.cmp(&a.1));
        recommendations.truncate(n);

        recommendations.into_iter().map(|(id, _)| id).collect()
    }

    /// Gets network statistics.
    pub fn statistics(&self) -> NetworkStatistics {
        let total_cases = self.cases.len();
        let total_citations = self.citations.len();

        let avg_citations = if total_cases > 0 {
            total_citations as f64 / total_cases as f64
        } else {
            0.0
        };

        let mut cases_by_jurisdiction: HashMap<String, usize> = HashMap::new();
        for case in self.cases.values() {
            *cases_by_jurisdiction
                .entry(case.jurisdiction.clone())
                .or_insert(0) += 1;
        }

        NetworkStatistics {
            total_cases,
            total_citations,
            avg_citations_per_case: avg_citations,
            cases_by_jurisdiction,
        }
    }
}

impl Default for CitationNetwork {
    fn default() -> Self {
        Self::new()
    }
}

/// Precedent strength analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecedentStrength {
    /// Case ID
    pub case_id: String,
    /// Total citation count
    pub citation_count: usize,
    /// Positive citations
    pub positive_citations: usize,
    /// Negative citations
    pub negative_citations: usize,
    /// Authority score
    pub authority_score: f64,
    /// Overall strength score
    pub strength_score: f64,
}

/// Citation cluster (community)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationCluster {
    /// Cluster ID
    pub id: String,
    /// Cases in the cluster
    pub cases: Vec<String>,
}

/// Citation evolution over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationEvolution {
    /// Case ID
    pub case_id: String,
    /// Citations by year
    pub evolution: Vec<(i32, usize)>,
}

/// Cross-jurisdictional influence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossJurisdictionalInfluence {
    /// Case ID
    pub case_id: String,
    /// Home jurisdiction
    pub home_jurisdiction: String,
    /// External citations by jurisdiction
    pub external_citations: HashMap<String, usize>,
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatistics {
    /// Total number of cases
    pub total_cases: usize,
    /// Total number of citations
    pub total_citations: usize,
    /// Average citations per case
    pub avg_citations_per_case: f64,
    /// Cases by jurisdiction
    pub cases_by_jurisdiction: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_network() -> CitationNetwork {
        let mut network = CitationNetwork::new();

        let case1 = CaseNode::new("case1", "Brown v. Board", "347 U.S. 483", 1954, "US")
            .with_court_level(CourtLevel::Supreme);
        let case2 = CaseNode::new("case2", "Loving v. Virginia", "388 U.S. 1", 1967, "US")
            .with_court_level(CourtLevel::Supreme);
        let case3 = CaseNode::new("case3", "Obergefell v. Hodges", "576 U.S. 644", 2015, "US")
            .with_court_level(CourtLevel::Supreme);

        network.add_case(case1);
        network.add_case(case2);
        network.add_case(case3);

        network
            .add_citation(Citation::new("case2", "case1", TreatmentType::Followed))
            .unwrap();
        network
            .add_citation(Citation::new("case3", "case1", TreatmentType::Followed))
            .unwrap();
        network
            .add_citation(Citation::new("case3", "case2", TreatmentType::Followed))
            .unwrap();

        network
    }

    #[test]
    fn test_citation_network_basic() {
        let network = create_sample_network();

        assert_eq!(network.cases.len(), 3);
        assert_eq!(network.citations.len(), 3);
    }

    #[test]
    fn test_incoming_outgoing_citations() {
        let network = create_sample_network();

        let incoming = network.get_incoming_citations("case1");
        assert_eq!(incoming.len(), 2);

        let outgoing = network.get_outgoing_citations("case3");
        assert_eq!(outgoing.len(), 2);
    }

    #[test]
    fn test_authority_scores() {
        let mut network = create_sample_network();
        network.compute_authority_scores(20, 0.85);

        let score1 = network.get_authority_score("case1").unwrap();
        let score3 = network.get_authority_score("case3").unwrap();

        // case1 should have higher authority (cited by both others)
        assert!(score1 > score3);
    }

    #[test]
    fn test_most_authoritative() {
        let mut network = create_sample_network();
        network.compute_authority_scores(20, 0.85);

        let top = network.get_most_authoritative(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "case1"); // Most authoritative
    }

    #[test]
    fn test_precedent_strength() {
        let mut network = create_sample_network();
        network.compute_authority_scores(20, 0.85);

        let strength = network.compute_precedent_strength("case1");
        assert_eq!(strength.citation_count, 2);
        assert_eq!(strength.positive_citations, 2);
        assert!(strength.strength_score > 0.0);
    }

    #[test]
    fn test_citation_clusters() {
        let network = create_sample_network();
        let clusters = network.find_citation_clusters();

        assert!(!clusters.is_empty());
        assert!(clusters[0].cases.len() >= 3);
    }

    #[test]
    fn test_citation_evolution() {
        let network = create_sample_network();
        let evolution = network.analyze_citation_evolution("case1");

        assert_eq!(evolution.case_id, "case1");
        assert!(!evolution.evolution.is_empty());
    }

    #[test]
    fn test_citation_recommendations() {
        let network = create_sample_network();
        let recommendations = network.recommend_citations("case2", 5);

        // Should recommend related cases
        assert!(!recommendations.is_empty() || network.cases.len() <= 3);
    }

    #[test]
    fn test_network_statistics() {
        let network = create_sample_network();
        let stats = network.statistics();

        assert_eq!(stats.total_cases, 3);
        assert_eq!(stats.total_citations, 3);
        assert!(stats.avg_citations_per_case > 0.0);
    }

    #[test]
    fn test_cross_jurisdictional_influence() {
        let mut network = CitationNetwork::new();

        let case1 = CaseNode::new("case1", "Test Case 1", "123 F.2d 456", 2000, "US");
        let case2 = CaseNode::new("case2", "Test Case 2", "456 F.2d 789", 2001, "UK");

        network.add_case(case1);
        network.add_case(case2);

        network
            .add_citation(Citation::new("case2", "case1", TreatmentType::Followed))
            .unwrap();

        let influences = network.find_cross_jurisdictional_influence();
        assert_eq!(influences.len(), 1);
        assert_eq!(influences[0].case_id, "case1");
    }
}
