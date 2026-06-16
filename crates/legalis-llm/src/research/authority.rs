//! Authority strength ranking.
//!
//! [`AuthorityRanker`] scores legal authorities on four independent axes -
//! recency, court level, citation count and subsequent treatment - and
//! combines them into a single normalised strength score using configurable
//! weights. This drives "which authority is strongest" decisions independent
//! of any particular fact pattern (cf. [`super::PrecedentAnalyzer`], which is
//! fact-relative).

use super::{LegalAuthority, court_authority_weight};
use crate::TreatmentType;
use serde::{Deserialize, Serialize};

/// Default reference year used when none is supplied.
const DEFAULT_CURRENT_YEAR: i32 = 2025;
/// Default recency half-life in years.
const DEFAULT_HALF_LIFE: f64 = 25.0;
/// Citation-count saturation constant (count at which the score reaches 0.5).
const CITATION_SATURATION: f64 = 50.0;

/// Relative weights for the four authority-strength axes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AuthorityWeights {
    /// Weight for recency.
    pub recency: f64,
    /// Weight for court level.
    pub court_level: f64,
    /// Weight for citation count.
    pub citation_count: f64,
    /// Weight for subsequent treatment.
    pub treatment: f64,
}

impl Default for AuthorityWeights {
    fn default() -> Self {
        Self {
            recency: 0.25,
            court_level: 0.30,
            citation_count: 0.25,
            treatment: 0.20,
        }
    }
}

impl AuthorityWeights {
    fn sum(&self) -> f64 {
        self.recency + self.court_level + self.citation_count + self.treatment
    }
}

/// A decomposed authority-strength score; every field lies in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AuthorityScore {
    /// Combined weighted score.
    pub total: f64,
    /// Recency component.
    pub recency: f64,
    /// Court-level component.
    pub court: f64,
    /// Citation-count component.
    pub citations: f64,
    /// Treatment component.
    pub treatment: f64,
}

/// An authority paired with its strength score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankedAuthority {
    /// Identifier of the authority.
    pub authority_id: String,
    /// Its strength score.
    pub score: AuthorityScore,
}

/// Ranks authorities by overall strength.
#[derive(Debug, Clone)]
pub struct AuthorityRanker {
    weights: AuthorityWeights,
    current_year: i32,
    half_life: f64,
}

impl Default for AuthorityRanker {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthorityRanker {
    /// Creates a ranker with default weights and reference year.
    pub fn new() -> Self {
        Self {
            weights: AuthorityWeights::default(),
            current_year: DEFAULT_CURRENT_YEAR,
            half_life: DEFAULT_HALF_LIFE,
        }
    }

    /// Sets the axis weights.
    pub fn with_weights(mut self, weights: AuthorityWeights) -> Self {
        self.weights = weights;
        self
    }

    /// Sets the reference ("current") year for recency calculations.
    pub fn with_current_year(mut self, year: i32) -> Self {
        self.current_year = year;
        self
    }

    /// Sets the recency half-life in years.
    pub fn with_half_life(mut self, half_life: f64) -> Self {
        self.half_life = half_life.max(0.1);
        self
    }

    /// Scores a single authority.
    pub fn score(&self, authority: &LegalAuthority) -> AuthorityScore {
        let recency = self.recency_score(authority.year);
        let court = self.court_score(authority);
        let citations = citation_score(authority.citation_count);
        let treatment = treatment_score(authority.treatment);

        let denom = self.weights.sum();
        let total = if denom <= 0.0 {
            0.0
        } else {
            (self.weights.recency * recency
                + self.weights.court_level * court
                + self.weights.citation_count * citations
                + self.weights.treatment * treatment)
                / denom
        };

        AuthorityScore {
            total: total.clamp(0.0, 1.0),
            recency,
            court,
            citations,
            treatment,
        }
    }

    /// Ranks authorities by total strength, strongest first.
    pub fn rank(&self, authorities: &[LegalAuthority], top_k: usize) -> Vec<RankedAuthority> {
        let mut ranked: Vec<RankedAuthority> = authorities
            .iter()
            .map(|a| RankedAuthority {
                authority_id: a.id.clone(),
                score: self.score(a),
            })
            .collect();
        ranked.sort_by(|a, b| {
            b.score
                .total
                .partial_cmp(&a.score.total)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.authority_id.cmp(&b.authority_id))
        });
        ranked.truncate(top_k);
        ranked
    }

    // --- internals ----------------------------------------------------------

    fn recency_score(&self, year: Option<i32>) -> f64 {
        match year {
            None => 0.5,
            Some(y) => {
                let age = (self.current_year - y) as f64;
                if age <= 0.0 {
                    1.0
                } else {
                    0.5_f64.powf(age / self.half_life).clamp(0.0, 1.0)
                }
            }
        }
    }

    fn court_score(&self, authority: &LegalAuthority) -> f64 {
        if !authority.authority_type.is_primary() {
            return 0.3;
        }
        match authority.court_level {
            Some(level) => court_authority_weight(level),
            None => 0.9,
        }
    }
}

/// Maps a citation count to a saturating `[0, 1]` score.
fn citation_score(count: u32) -> f64 {
    let c = count as f64;
    c / (c + CITATION_SATURATION)
}

/// Maps subsequent treatment to a `[0, 1]` score.
fn treatment_score(treatment: Option<TreatmentType>) -> f64 {
    match treatment {
        None | Some(TreatmentType::Neutral) => 0.7,
        Some(TreatmentType::Followed) | Some(TreatmentType::PositiveCitation) => 1.0,
        Some(TreatmentType::Distinguished) => 0.5,
        Some(TreatmentType::Questioned) | Some(TreatmentType::NegativeCitation) => 0.3,
        Some(TreatmentType::Overruled) => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthorityType, CourtLevel, Jurisdiction};

    fn auth(id: &str) -> LegalAuthority {
        LegalAuthority::new(
            id,
            id,
            "1 X. 1",
            "some legal text",
            AuthorityType::Case,
            Jurisdiction::UsFederal,
        )
    }

    #[test]
    fn test_recency_decay() {
        let ranker = AuthorityRanker::new()
            .with_current_year(2025)
            .with_half_life(25.0);
        let recent = ranker.recency_score(Some(2025));
        let old = ranker.recency_score(Some(1925));
        let missing = ranker.recency_score(None);
        assert!((recent - 1.0).abs() < 1e-9);
        assert!(old < recent);
        assert!(old < 0.1);
        assert!((missing - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_citation_and_treatment_scores() {
        assert!((citation_score(0)).abs() < 1e-9);
        assert!((citation_score(50) - 0.5).abs() < 1e-9);
        assert!(citation_score(5000) > 0.98);
        assert!((treatment_score(Some(TreatmentType::Followed)) - 1.0).abs() < 1e-9);
        assert!((treatment_score(Some(TreatmentType::Overruled))).abs() < 1e-9);
    }

    #[test]
    fn test_court_level_ordering() {
        let ranker = AuthorityRanker::new();
        let supreme = auth("s").with_court_level(CourtLevel::Supreme);
        let trial = auth("t").with_court_level(CourtLevel::Trial);
        assert!(ranker.court_score(&supreme) > ranker.court_score(&trial));
    }

    #[test]
    fn test_full_score_and_ranking() {
        let ranker = AuthorityRanker::new().with_current_year(2025);
        let strong = auth("strong")
            .with_court_level(CourtLevel::Supreme)
            .with_year(2020)
            .with_citation_count(5000)
            .with_treatment(TreatmentType::Followed);
        let weak = auth("weak")
            .with_court_level(CourtLevel::Trial)
            .with_year(1950)
            .with_citation_count(2)
            .with_treatment(TreatmentType::Questioned);

        let ranked = ranker.rank(&[weak.clone(), strong.clone()], 10);
        assert_eq!(ranked[0].authority_id, "strong");
        assert!(ranked[0].score.total > ranked[1].score.total);
        assert!(ranked[0].score.total >= 0.0 && ranked[0].score.total <= 1.0);
    }

    #[test]
    fn test_zero_weights_safe() {
        let weights = AuthorityWeights {
            recency: 0.0,
            court_level: 0.0,
            citation_count: 0.0,
            treatment: 0.0,
        };
        let ranker = AuthorityRanker::new().with_weights(weights);
        let score = ranker.score(&auth("x").with_court_level(CourtLevel::Supreme));
        assert!((score.total).abs() < 1e-9);
    }
}
