//! Trade Marks Act 1995 Implementation
//!
//! Australian trade mark law for registration and protection of marks.
//!
//! ## Registration Requirements
//!
//! A trade mark can be registered if it:
//! 1. **Can distinguish** goods/services (s.41)
//! 2. **Is not contrary to law** (s.42)
//! 3. **Is not deceptive** (s.43)
//! 4. **Does not conflict with earlier marks** (s.44)
//!
//! ## Key Cases
//!
//! - **Cantarella Bros v Modena (2014)**: Distinctiveness of foreign words
//! - **Self Care IP v Allergan (2023)**: Likelihood of confusion test

use super::error::{IpError, Result};
use super::types::{IpOwner, RegistrationStatus};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Trade mark under Australian law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeMark {
    /// Trade mark number
    pub mark_number: String,
    /// The mark itself (word, device, etc.)
    pub mark: String,
    /// Type of mark
    pub mark_type: TradeMarkType,
    /// Owner
    pub owner: IpOwner,
    /// Goods/services classes
    pub classes: Vec<TradeMarkClass>,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Registration date
    pub registration_date: Option<NaiveDate>,
    /// Renewal due date
    pub renewal_date: Option<NaiveDate>,
    /// Status
    pub status: RegistrationStatus,
    /// Priority claim
    pub priority_claim: Option<PriorityClaim>,
}

/// Type of trade mark
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TradeMarkType {
    /// Word mark
    Word,
    /// Logo/device mark
    Device,
    /// Combined word and device
    Combined,
    /// Shape mark (3D)
    Shape,
    /// Sound mark
    Sound,
    /// Colour mark
    Colour,
    /// Scent mark
    Scent,
    /// Certification mark (s.169)
    Certification,
    /// Collective mark (s.162)
    Collective,
    /// Defensive mark (s.185)
    Defensive,
}

/// Nice Classification class for goods/services
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradeMarkClass {
    /// Class number (1-45)
    pub class_number: u8,
    /// Description of goods/services
    pub description: String,
}

impl TradeMarkClass {
    /// Check if this is a goods class (1-34)
    pub fn is_goods(&self) -> bool {
        self.class_number <= 34
    }

    /// Check if this is a services class (35-45)
    pub fn is_services(&self) -> bool {
        self.class_number >= 35
    }
}

/// Priority claim under Paris Convention
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PriorityClaim {
    /// Country of first filing
    pub country: String,
    /// Application number
    pub application_number: String,
    /// Filing date
    pub filing_date: NaiveDate,
}

/// Trade mark application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeMarkApplication {
    /// The mark
    pub mark: String,
    /// Type of mark
    pub mark_type: TradeMarkType,
    /// Applicant
    pub applicant: IpOwner,
    /// Classes applied for
    pub classes: Vec<TradeMarkClass>,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Evidence of use (if claiming acquired distinctiveness)
    pub evidence_of_use: Option<EvidenceOfUse>,
    /// Priority claim
    pub priority_claim: Option<PriorityClaim>,
}

/// Evidence of use for acquired distinctiveness
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceOfUse {
    /// Period of use
    pub use_since: NaiveDate,
    /// Sales figures
    pub sales_figures: Option<f64>,
    /// Advertising spend
    pub advertising_spend: Option<f64>,
    /// Survey evidence
    pub survey_evidence: bool,
    /// Description
    pub description: String,
}

/// Absolute grounds for refusal (s.41, s.42)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbsoluteGrounds {
    /// Is mark capable of distinguishing? (s.41)
    pub capable_of_distinguishing: bool,
    /// Is mark inherently adapted to distinguish? (s.41(3))
    pub inherently_distinctive: bool,
    /// Has acquired distinctiveness through use? (s.41(5))
    pub acquired_distinctiveness: bool,
    /// Is contrary to law? (s.42(a))
    pub contrary_to_law: bool,
    /// Is scandalous? (s.42(a))
    pub scandalous: bool,
    /// Reasons
    pub analysis: Vec<String>,
}

/// Relative grounds for refusal (s.44)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelativeGrounds {
    /// Conflicting earlier marks
    pub conflicting_marks: Vec<ConflictingMark>,
    /// Overall assessment
    pub has_conflict: bool,
    /// Analysis
    pub analysis: String,
}

/// Conflicting earlier mark
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictingMark {
    /// Earlier mark number
    pub mark_number: String,
    /// Earlier mark
    pub mark: String,
    /// Classes in conflict
    pub conflicting_classes: Vec<u8>,
    /// Type of conflict
    pub conflict_type: ConflictType,
}

/// Type of conflict with earlier mark
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictType {
    /// Identical marks, identical goods/services (s.44(1))
    IdenticalMarkIdenticalGoods,
    /// Identical marks, similar goods/services (s.44(2))
    IdenticalMarkSimilarGoods,
    /// Similar marks, identical goods/services (s.44(2))
    SimilarMarkIdenticalGoods,
    /// Similar marks, similar goods/services (s.44(2))
    SimilarMarkSimilarGoods,
}

/// Likelihood of confusion assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LikelihoodOfConfusion {
    /// Overall likelihood of confusion
    pub is_confusing: bool,
    /// Visual similarity (0.0 - 1.0)
    pub visual_similarity: f32,
    /// Aural similarity (0.0 - 1.0)
    pub aural_similarity: f32,
    /// Conceptual similarity (0.0 - 1.0)
    pub conceptual_similarity: f32,
    /// Goods/services relatedness
    pub goods_relatedness: f32,
    /// Analysis
    pub analysis: String,
}

/// Trade mark infringement (s.120)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeMarkInfringement {
    /// Registered mark
    pub registered_mark: String,
    /// Allegedly infringing sign
    pub infringing_sign: String,
    /// Type of infringement
    pub infringement_type: InfringementType,
    /// Use as a trade mark?
    pub use_as_trade_mark: bool,
    /// Defenses available
    pub defenses: Vec<InfringementDefense>,
}

/// Type of trade mark infringement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementType {
    /// Identical mark, identical goods (s.120(1))
    IdenticalIdentical,
    /// Substantially identical, identical or similar goods (s.120(2))
    SubstantiallyIdentical,
    /// Deceptively similar, identical or similar goods (s.120(2))
    DeceptivelySimilar,
    /// Well-known mark dilution (s.120(3))
    WellKnownDilution,
    /// No infringement
    None,
}

/// Defense to trade mark infringement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementDefense {
    /// Own name defense (s.122(1)(a))
    OwnName,
    /// Descriptive use (s.122(1)(b))
    DescriptiveUse,
    /// Comparative advertising (s.122(1)(d))
    ComparativeAdvertising,
    /// Prior continuous use (s.124)
    PriorContinuousUse {
        /// Date since
        since: NaiveDate,
    },
    /// Non-use (s.92 - revocation ground)
    NonUse,
}

/// Validates trade mark application
pub fn validate_trademark_application(application: &TradeMarkApplication) -> Result<()> {
    // Check mark is not empty
    if application.mark.trim().is_empty() {
        return Err(IpError::MissingInformation {
            field: "mark".to_string(),
        });
    }

    // Check at least one class
    if application.classes.is_empty() {
        return Err(IpError::MissingInformation {
            field: "goods/services classes".to_string(),
        });
    }

    // Validate class numbers
    for class in &application.classes {
        if class.class_number == 0 || class.class_number > 45 {
            return Err(IpError::ValidationError {
                message: format!(
                    "Invalid class number: {}. Must be 1-45.",
                    class.class_number
                ),
            });
        }
    }

    Ok(())
}

/// Checks absolute grounds for refusal (s.41, s.42)
pub fn check_absolute_grounds(application: &TradeMarkApplication) -> Result<AbsoluteGrounds> {
    let mark_lower = application.mark.to_lowercase();
    let mut analysis = Vec::new();

    // Check if descriptive (s.41(2))
    let descriptive_terms = ["best", "quality", "fresh", "natural", "organic", "pure"];
    let is_descriptive = descriptive_terms.iter().any(|term| mark_lower == *term);

    if is_descriptive {
        analysis.push(format!(
            "Mark '{}' is descriptive under s.41(2)",
            application.mark
        ));
    }

    // Check if scandalous (s.42(a))
    let scandalous_terms = ["offensive term placeholder"]; // Real implementation would have proper list
    let is_scandalous = scandalous_terms
        .iter()
        .any(|term| mark_lower.contains(term));

    if is_scandalous {
        analysis.push("Mark is scandalous under s.42(a)".to_string());
    }

    // Check inherent distinctiveness
    let inherently_distinctive = !is_descriptive
        && application.mark.len() > 2
        && !mark_lower.chars().all(|c| c.is_numeric());

    // Check acquired distinctiveness
    let acquired_distinctiveness = application.evidence_of_use.is_some()
        && application
            .evidence_of_use
            .as_ref()
            .map(|e| e.use_since < application.filing_date)
            .unwrap_or(false);

    let capable_of_distinguishing = inherently_distinctive || acquired_distinctiveness;

    if !capable_of_distinguishing {
        analysis.push("Mark not capable of distinguishing under s.41".to_string());
    }

    Ok(AbsoluteGrounds {
        capable_of_distinguishing,
        inherently_distinctive,
        acquired_distinctiveness,
        contrary_to_law: false,
        scandalous: is_scandalous,
        analysis,
    })
}

/// Checks relative grounds for refusal (s.44)
pub fn check_relative_grounds(
    application: &TradeMarkApplication,
    earlier_marks: &[TradeMark],
) -> Result<RelativeGrounds> {
    let mut conflicting_marks = Vec::new();

    for earlier in earlier_marks {
        // Check if marks are identical or similar
        let marks_similar = application.mark.to_lowercase() == earlier.mark.to_lowercase()
            || levenshtein_similar(&application.mark, &earlier.mark);

        if marks_similar {
            // Check class overlap
            let conflicting_classes: Vec<u8> = application
                .classes
                .iter()
                .filter(|c| {
                    earlier
                        .classes
                        .iter()
                        .any(|ec| ec.class_number == c.class_number)
                })
                .map(|c| c.class_number)
                .collect();

            if !conflicting_classes.is_empty() {
                let identical_mark = application.mark.to_lowercase() == earlier.mark.to_lowercase();
                let identical_goods = conflicting_classes.len() == application.classes.len();

                let conflict_type = match (identical_mark, identical_goods) {
                    (true, true) => ConflictType::IdenticalMarkIdenticalGoods,
                    (true, false) => ConflictType::IdenticalMarkSimilarGoods,
                    (false, true) => ConflictType::SimilarMarkIdenticalGoods,
                    (false, false) => ConflictType::SimilarMarkSimilarGoods,
                };

                conflicting_marks.push(ConflictingMark {
                    mark_number: earlier.mark_number.clone(),
                    mark: earlier.mark.clone(),
                    conflicting_classes,
                    conflict_type,
                });
            }
        }
    }

    let has_conflict = !conflicting_marks.is_empty();
    let analysis = if has_conflict {
        format!(
            "Found {} conflicting earlier mark(s) under s.44",
            conflicting_marks.len()
        )
    } else {
        "No conflicting earlier marks found".to_string()
    };

    Ok(RelativeGrounds {
        conflicting_marks,
        has_conflict,
        analysis,
    })
}

/// Simple Levenshtein distance check for similarity
fn levenshtein_similar(a: &str, b: &str) -> bool {
    let a = a.to_lowercase();
    let b = b.to_lowercase();

    if a == b {
        return true;
    }

    let len_diff = (a.len() as i32 - b.len() as i32).unsigned_abs() as usize;
    if len_diff > 2 {
        return false;
    }

    // Simple character match check
    let common_chars = a.chars().filter(|c| b.contains(*c)).count();
    let similarity = common_chars as f32 / a.len().max(b.len()) as f32;

    similarity > 0.7
}

/// Assesses likelihood of confusion (s.120)
pub fn assess_likelihood_of_confusion(
    mark1: &TradeMark,
    mark2: &TradeMark,
) -> LikelihoodOfConfusion {
    // Visual similarity
    let visual = if mark1.mark.to_lowercase() == mark2.mark.to_lowercase() {
        1.0
    } else {
        let common = mark1
            .mark
            .to_lowercase()
            .chars()
            .filter(|c| mark2.mark.to_lowercase().contains(*c))
            .count();
        common as f32 / mark1.mark.len().max(mark2.mark.len()) as f32
    };

    // Aural similarity (simplified)
    let aural = visual; // Simplified - real implementation would use phonetic comparison

    // Conceptual similarity (simplified)
    let conceptual = if mark1.mark == mark2.mark { 1.0 } else { 0.0 };

    // Goods relatedness
    let common_classes = mark1
        .classes
        .iter()
        .filter(|c| {
            mark2
                .classes
                .iter()
                .any(|c2| c2.class_number == c.class_number)
        })
        .count();
    let goods_relatedness =
        common_classes as f32 / mark1.classes.len().max(mark2.classes.len()) as f32;

    // Overall assessment
    let overall = (visual + aural + conceptual + goods_relatedness) / 4.0;
    let is_confusing = overall > 0.6;

    let analysis = format!(
        "Overall similarity: {:.0}%. Visual: {:.0}%, Aural: {:.0}%, \
         Conceptual: {:.0}%, Goods: {:.0}%",
        overall * 100.0,
        visual * 100.0,
        aural * 100.0,
        conceptual * 100.0,
        goods_relatedness * 100.0
    );

    LikelihoodOfConfusion {
        is_confusing,
        visual_similarity: visual,
        aural_similarity: aural,
        conceptual_similarity: conceptual,
        goods_relatedness,
        analysis,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intellectual_property::types::OwnerType;

    fn create_test_application() -> TradeMarkApplication {
        TradeMarkApplication {
            mark: "ACME".to_string(),
            mark_type: TradeMarkType::Word,
            applicant: IpOwner {
                name: "Acme Pty Ltd".to_string(),
                owner_type: OwnerType::Company,
                address: Some("Sydney, NSW".to_string()),
                country: "AU".to_string(),
                abn: Some("12345678901".to_string()),
                acn: Some("123456789".to_string()),
            },
            classes: vec![TradeMarkClass {
                class_number: 25,
                description: "Clothing".to_string(),
            }],
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            evidence_of_use: None,
            priority_claim: None,
        }
    }

    #[test]
    fn test_validate_application_valid() {
        let app = create_test_application();
        assert!(validate_trademark_application(&app).is_ok());
    }

    #[test]
    fn test_validate_application_empty_mark() {
        let mut app = create_test_application();
        app.mark = "".to_string();

        let result = validate_trademark_application(&app);
        assert!(result.is_err());
    }

    #[test]
    fn test_absolute_grounds_distinctive() {
        let app = create_test_application();
        let grounds = check_absolute_grounds(&app).unwrap();

        assert!(grounds.capable_of_distinguishing);
        assert!(grounds.inherently_distinctive);
        assert!(!grounds.scandalous);
    }

    #[test]
    fn test_absolute_grounds_descriptive() {
        let mut app = create_test_application();
        app.mark = "BEST".to_string();

        let grounds = check_absolute_grounds(&app).unwrap();
        assert!(!grounds.inherently_distinctive);
    }

    #[test]
    fn test_relative_grounds_no_conflict() {
        let app = create_test_application();
        let earlier_marks: Vec<TradeMark> = vec![];

        let grounds = check_relative_grounds(&app, &earlier_marks).unwrap();
        assert!(!grounds.has_conflict);
    }

    #[test]
    fn test_class_goods_services() {
        let goods_class = TradeMarkClass {
            class_number: 25,
            description: "Clothing".to_string(),
        };
        let services_class = TradeMarkClass {
            class_number: 35,
            description: "Advertising".to_string(),
        };

        assert!(goods_class.is_goods());
        assert!(!goods_class.is_services());
        assert!(!services_class.is_goods());
        assert!(services_class.is_services());
    }
}
