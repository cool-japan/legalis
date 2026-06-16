//! Legal precedent analysis.
//!
//! [`PrecedentAnalyzer`] determines whether an authority is *binding* or
//! *persuasive* in a given [`Forum`], using a court-hierarchy model and
//! jurisdiction matching, and scores how *on point* it is by comparing the
//! authority's text with the facts of the matter at hand.

use super::{
    Forum, LegalAuthority, court_authority_weight, court_rank, is_us_jurisdiction,
    text_cosine_similarity,
};
use crate::{CourtLevel, Jurisdiction, TreatmentType};
use serde::{Deserialize, Serialize};

/// Whether an authority binds the forum court.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BindingStatus {
    /// Mandatory authority - the forum must follow it.
    Binding,
    /// Persuasive authority - the forum may consider it.
    Persuasive,
    /// The authority does not apply to the forum at all.
    NotApplicable,
    /// The authority has been overruled and is no longer good law.
    NoLongerGoodLaw,
}

impl BindingStatus {
    /// Returns a multiplicative weight reflecting the precedential force.
    fn factor(&self) -> f64 {
        match self {
            BindingStatus::Binding => 1.0,
            BindingStatus::Persuasive => 0.6,
            BindingStatus::NoLongerGoodLaw => 0.1,
            BindingStatus::NotApplicable => 0.0,
        }
    }

    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            BindingStatus::Binding => "binding",
            BindingStatus::Persuasive => "persuasive",
            BindingStatus::NotApplicable => "not applicable",
            BindingStatus::NoLongerGoodLaw => "no longer good law",
        }
    }
}

/// The result of analysing a single authority against a forum and fact pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrecedentAssessment {
    /// Identifier of the analysed authority.
    pub authority_id: String,
    /// Binding status in the forum.
    pub binding: BindingStatus,
    /// Combined precedential weight in `[0, 1]`.
    pub weight: f64,
    /// Factual similarity to the matter (`[0, 1]`).
    pub similarity: f64,
    /// Whether the authority is factually on point.
    pub on_point: bool,
    /// Explanation of the assessment.
    pub rationale: String,
}

/// Analyses the precedential value of authorities.
#[derive(Debug, Clone)]
pub struct PrecedentAnalyzer {
    /// Minimum similarity for an authority to be considered on point.
    on_point_threshold: f64,
}

impl Default for PrecedentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl PrecedentAnalyzer {
    /// Creates an analyser with a default on-point threshold of `0.10`.
    pub fn new() -> Self {
        Self {
            on_point_threshold: 0.10,
        }
    }

    /// Sets the on-point similarity threshold.
    pub fn with_on_point_threshold(mut self, threshold: f64) -> Self {
        self.on_point_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Assesses one authority against a forum and fact pattern.
    pub fn analyze(
        &self,
        authority: &LegalAuthority,
        forum: &Forum,
        facts: &str,
    ) -> PrecedentAssessment {
        let binding = self.determine_binding(authority, forum);
        let similarity = text_cosine_similarity(&authority.indexable_text(), facts);
        let on_point = similarity >= self.on_point_threshold;
        let weight = self.compute_weight(authority, binding, similarity);
        let rationale = self.build_rationale(authority, forum, binding, similarity, on_point);

        PrecedentAssessment {
            authority_id: authority.id.clone(),
            binding,
            weight,
            similarity,
            on_point,
            rationale,
        }
    }

    /// Assesses and ranks many authorities by precedential weight.
    pub fn rank_precedents(
        &self,
        authorities: &[LegalAuthority],
        forum: &Forum,
        facts: &str,
        top_k: usize,
    ) -> Vec<PrecedentAssessment> {
        let mut assessments: Vec<PrecedentAssessment> = authorities
            .iter()
            .map(|a| self.analyze(a, forum, facts))
            .collect();
        assessments.sort_by(|a, b| {
            b.weight
                .partial_cmp(&a.weight)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.authority_id.cmp(&b.authority_id))
        });
        assessments.truncate(top_k);
        assessments
    }

    // --- internals ----------------------------------------------------------

    fn determine_binding(&self, authority: &LegalAuthority, forum: &Forum) -> BindingStatus {
        if !authority.is_good_law() {
            return BindingStatus::NoLongerGoodLaw;
        }
        if !authority.authority_type.is_primary() {
            // Secondary sources are never binding.
            return BindingStatus::Persuasive;
        }

        // The U.S. Supreme Court binds every U.S. forum on questions of federal law.
        if authority.jurisdiction == Jurisdiction::UsFederal
            && authority.court_level == Some(CourtLevel::Supreme)
            && is_us_jurisdiction(&forum.jurisdiction)
        {
            return BindingStatus::Binding;
        }

        if authority.jurisdiction != forum.jurisdiction {
            return BindingStatus::Persuasive;
        }

        match authority.court_level {
            // Statutes, regulations and constitutional provisions of the same
            // jurisdiction are mandatory authority.
            None => BindingStatus::Binding,
            // A higher court in the same hierarchy binds the forum; a court of
            // equal or lower rank is only persuasive (horizontal stare decisis).
            Some(level) => {
                if court_rank(level) > court_rank(forum.court_level) {
                    BindingStatus::Binding
                } else {
                    BindingStatus::Persuasive
                }
            }
        }
    }

    fn compute_weight(
        &self,
        authority: &LegalAuthority,
        binding: BindingStatus,
        similarity: f64,
    ) -> f64 {
        let base = self.base_authority_weight(authority);
        let treatment = treatment_factor(authority.treatment);
        let relevance = 0.4 + 0.6 * similarity;
        (base * relevance * treatment * binding.factor()).clamp(0.0, 1.0)
    }

    fn base_authority_weight(&self, authority: &LegalAuthority) -> f64 {
        if !authority.authority_type.is_primary() {
            return 0.4;
        }
        match authority.court_level {
            Some(level) => court_authority_weight(level),
            // Primary, non-case authority (statute / regulation / constitution).
            None => 0.9,
        }
    }

    fn build_rationale(
        &self,
        authority: &LegalAuthority,
        forum: &Forum,
        binding: BindingStatus,
        similarity: f64,
        on_point: bool,
    ) -> String {
        let court = authority
            .court_level
            .map(|c| format!("{c:?} court"))
            .unwrap_or_else(|| authority.authority_type.label().to_string());
        let point = if on_point {
            "factually on point"
        } else {
            "factually distinguishable"
        };
        format!(
            "{} ({}) from {} is {} authority in a {:?} forum of {}; \
             {} (similarity {:.2}).",
            authority.title,
            court,
            authority.jurisdiction.description(),
            binding.label(),
            forum.court_level,
            forum.jurisdiction.description(),
            point,
            similarity,
        )
    }
}

/// Returns a multiplicative factor reflecting subsequent treatment.
fn treatment_factor(treatment: Option<TreatmentType>) -> f64 {
    match treatment {
        None | Some(TreatmentType::Neutral) => 1.0,
        Some(TreatmentType::Followed) | Some(TreatmentType::PositiveCitation) => 1.1,
        Some(TreatmentType::Distinguished) => 0.8,
        Some(TreatmentType::Questioned) | Some(TreatmentType::NegativeCitation) => 0.6,
        Some(TreatmentType::Overruled) => 0.1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthorityType, Jurisdiction};

    fn case(id: &str, juris: Jurisdiction, court: CourtLevel, text: &str) -> LegalAuthority {
        LegalAuthority::new(id, id, "1 X. 1", text, AuthorityType::Case, juris)
            .with_court_level(court)
            .with_year(2000)
    }

    #[test]
    fn test_supreme_court_binds_all_us_forums() {
        let analyzer = PrecedentAnalyzer::new();
        let scotus = case(
            "scotus",
            Jurisdiction::UsFederal,
            CourtLevel::Supreme,
            "duty of care negligence",
        );
        let state_forum = Forum::new(Jurisdiction::UsState("Texas".into()), CourtLevel::Trial);
        let assessment = analyzer.analyze(&scotus, &state_forum, "negligence and duty of care");
        assert_eq!(assessment.binding, BindingStatus::Binding);
        assert!(assessment.on_point);
        assert!(assessment.weight > 0.0);
    }

    #[test]
    fn test_higher_court_binds_lower_same_jurisdiction() {
        let analyzer = PrecedentAnalyzer::new();
        let appellate = case(
            "app",
            Jurisdiction::UsState("California".into()),
            CourtLevel::Appellate,
            "contract formation",
        );
        let trial_forum = Forum::new(
            Jurisdiction::UsState("California".into()),
            CourtLevel::Trial,
        );
        assert_eq!(
            analyzer
                .analyze(&appellate, &trial_forum, "contract")
                .binding,
            BindingStatus::Binding
        );
    }

    #[test]
    fn test_out_of_jurisdiction_is_persuasive() {
        let analyzer = PrecedentAnalyzer::new();
        let ny = case(
            "ny",
            Jurisdiction::UsState("New York".into()),
            CourtLevel::Supreme,
            "negligence",
        );
        let forum = Forum::new(Jurisdiction::UsState("Florida".into()), CourtLevel::Trial);
        assert_eq!(
            analyzer.analyze(&ny, &forum, "negligence").binding,
            BindingStatus::Persuasive
        );
    }

    #[test]
    fn test_overruled_is_not_good_law() {
        let analyzer = PrecedentAnalyzer::new();
        let overruled = case(
            "old",
            Jurisdiction::UsFederal,
            CourtLevel::Supreme,
            "separate but equal",
        )
        .with_treatment(TreatmentType::Overruled);
        let forum = Forum::new(Jurisdiction::UsFederal, CourtLevel::Trial);
        let assessment = analyzer.analyze(&overruled, &forum, "separate but equal doctrine");
        assert_eq!(assessment.binding, BindingStatus::NoLongerGoodLaw);
        assert!(assessment.weight < 0.2);
    }

    #[test]
    fn test_secondary_source_persuasive_only() {
        let analyzer = PrecedentAnalyzer::new();
        let treatise = LegalAuthority::new(
            "t",
            "Prosser on Torts",
            "Treatise",
            "negligence duty breach causation damages",
            AuthorityType::SecondarySource,
            Jurisdiction::UsFederal,
        );
        let forum = Forum::new(Jurisdiction::UsFederal, CourtLevel::Supreme);
        assert_eq!(
            analyzer.analyze(&treatise, &forum, "negligence").binding,
            BindingStatus::Persuasive
        );
    }

    #[test]
    fn test_ranking_orders_by_weight() {
        let analyzer = PrecedentAnalyzer::new();
        let forum = Forum::new(Jurisdiction::UsFederal, CourtLevel::Trial);
        let authorities = vec![
            case(
                "binding_onpoint",
                Jurisdiction::UsFederal,
                CourtLevel::Supreme,
                "negligence duty of care foreseeable plaintiff",
            ),
            case(
                "persuasive_offpoint",
                Jurisdiction::Uk,
                CourtLevel::Trial,
                "maritime salvage law",
            ),
        ];
        let ranked = analyzer.rank_precedents(
            &authorities,
            &forum,
            "negligence and the duty of care to a foreseeable plaintiff",
            10,
        );
        assert_eq!(ranked[0].authority_id, "binding_onpoint");
        assert!(ranked[0].weight > ranked[1].weight);
    }
}
