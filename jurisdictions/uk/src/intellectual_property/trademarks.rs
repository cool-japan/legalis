//! Trade Marks Act 1994 Implementation
//!
//! UK trade mark law implementing EU Trade Mark Directive.
//!
//! ## Registrability (s.1-3)
//!
//! A trade mark must be:
//! 1. A **sign** capable of being represented graphically
//! 2. **Distinctive** of goods/services (s.3(1)(b))
//! 3. **Not descriptive** (s.3(1)(c))
//! 4. **Not customary** (s.3(1)(d))
//! 5. **Not contrary to public policy** (s.3(3))
//!
//! ## Infringement (s.10)
//!
//! Infringement occurs when unauthorized person uses:
//! - **s.10(1)**: Identical mark on identical goods/services
//! - **s.10(2)**: Identical/similar mark on identical/similar goods/services (likelihood of confusion)
//! - **s.10(3)**: Identical/similar mark on dissimilar goods/services (reputation, unfair advantage/detriment)

use super::error::{IpError, IpResult};
use super::types::{IpOwner, RegistrationStatus};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Trade mark
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradeMark {
    /// The sign (word, logo, shape, sound, etc.)
    pub sign: String,
    /// Type of mark
    pub mark_type: TradeMarkType,
    /// Goods and/or services (Nice Classification)
    pub goods_services: Vec<String>,
    /// Nice classes
    pub nice_classes: Vec<u8>,
    /// Owner
    pub owner: Option<IpOwner>,
    /// Registration number (if registered)
    pub registration_number: Option<String>,
    /// Registration date
    pub registration_date: Option<NaiveDate>,
    /// Renewal date
    pub renewal_date: Option<NaiveDate>,
    /// Status
    pub status: RegistrationStatus,
}

/// Type of trade mark
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TradeMarkType {
    /// Word mark
    Word,
    /// Logo/device mark
    Logo,
    /// Combined word and logo
    Combined,
    /// Shape mark (3D)
    Shape,
    /// Sound mark
    Sound,
    /// Colour mark
    Colour,
    /// Pattern mark
    Pattern,
    /// Position mark
    Position,
}

/// Trade mark application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeMarkApplication {
    /// The sign
    pub sign: String,
    /// Type
    pub mark_type: TradeMarkType,
    /// Goods/services
    pub goods_services: Vec<String>,
    /// Nice classes
    pub nice_classes: Vec<u8>,
    /// Applicant
    pub applicant: IpOwner,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Claim of use date (if claiming earlier use)
    pub use_date: Option<NaiveDate>,
    /// Earlier marks cited
    pub earlier_marks: Vec<TradeMark>,
}

/// Absolute grounds for refusal (s.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbsoluteGroundsRefusal {
    /// Lacks distinctive character (s.3(1)(b))
    pub lacks_distinctiveness: bool,
    /// Consists of signs/indications describing goods/services (s.3(1)(c))
    pub descriptive: bool,
    /// Customary in trade (s.3(1)(d))
    pub customary: bool,
    /// Consists of shape (s.3(2))
    pub shape_exclusions: Vec<String>,
    /// Contrary to public policy or morality (s.3(3))
    pub contrary_to_policy: bool,
    /// Deceptive (s.3(3)(b))
    pub deceptive: bool,
    /// Prohibited by law (s.3(4))
    pub prohibited: bool,
    /// Bad faith (s.3(6))
    pub bad_faith: bool,
}

/// Relative grounds for refusal (s.5)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelativeGroundsRefusal {
    /// Earlier identical mark on identical goods/services (s.5(1))
    pub identical_mark_identical_goods: Option<String>,
    /// Likelihood of confusion (s.5(2))
    pub likelihood_of_confusion: Option<LikelihoodOfConfusion>,
    /// Earlier mark with reputation (s.5(3))
    pub reputation_mark: Option<String>,
}

/// Likelihood of confusion assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LikelihoodOfConfusion {
    /// Earlier mark
    pub earlier_mark: String,
    /// Similarity of signs (0.0 - 1.0)
    pub sign_similarity: f64,
    /// Similarity of goods/services (0.0 - 1.0)
    pub goods_similarity: f64,
    /// Overall likelihood of confusion
    pub likelihood: ConfusionLikelihood,
    /// Factors considered
    pub factors: Vec<String>,
}

/// Likelihood of confusion level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfusionLikelihood {
    /// No likelihood of confusion
    None,
    /// Low likelihood
    Low,
    /// Medium likelihood
    Medium,
    /// High likelihood (likely refusal)
    High,
    /// Certain confusion (definite refusal)
    Certain,
}

/// Trade mark infringement (s.10)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeMarkInfringement {
    /// Registered trade mark
    pub registered_mark: String,
    /// Allegedly infringing sign
    pub accused_sign: String,
    /// Goods/services of accused use
    pub accused_goods_services: Vec<String>,
    /// Type of infringement claimed
    pub infringement_type: InfringementSection,
    /// Is there infringement?
    pub is_infringement: bool,
    /// Defenses available
    pub defenses: Vec<TradeMarkDefense>,
}

/// Section of s.10 being claimed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementSection {
    /// s.10(1): Identical mark, identical goods/services
    Section10_1,
    /// s.10(2): Identical/similar mark, identical/similar goods (likelihood of confusion)
    Section10_2,
    /// s.10(3): Reputation mark, dissimilar goods (taking unfair advantage/detriment)
    Section10_3,
}

/// Defenses to trade mark infringement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TradeMarkDefense {
    /// Own name defense (s.11(2)(a))
    OwnName,
    /// Descriptive use (s.11(2)(b))
    Descriptive,
    /// Indication of intended purpose (s.11(2)(c))
    IntendedPurpose,
    /// Honest concurrent use
    HonestConcurrentUse,
    /// Exhaustion of rights (first sale in EEA)
    Exhaustion,
    /// Mark not used (s.46 - revocation for non-use)
    NonUse,
}

/// Checks absolute grounds for refusal (s.3)
pub fn check_absolute_grounds(
    application: &TradeMarkApplication,
) -> IpResult<AbsoluteGroundsRefusal> {
    let sign_lower = application.sign.to_lowercase();

    // Check if purely descriptive (s.3(1)(c))
    let descriptive_terms = [
        "quality",
        "quantity",
        "intended purpose",
        "value",
        "geographical origin",
    ];
    let is_descriptive = application.goods_services.iter().any(|goods| {
        descriptive_terms
            .iter()
            .any(|term| goods.to_lowercase().contains(term) && sign_lower.contains(term))
    });

    // Check if customary (s.3(1)(d))
    let customary_terms = ["super", "best", "premium", "deluxe", "professional"];
    let is_customary = customary_terms.iter().any(|term| sign_lower.contains(term));

    // Check distinctiveness (s.3(1)(b))
    // Simplified: single letters/numbers lack distinctiveness
    let lacks_distinctiveness =
        application.sign.len() == 1 || application.sign.chars().all(|c| c.is_numeric());

    // Check shape exclusions (s.3(2))
    let mut shape_exclusions = Vec::new();
    if application.mark_type == TradeMarkType::Shape {
        if application.sign.contains("functional") {
            shape_exclusions
                .push("Shape necessary to obtain technical result (s.3(2)(b))".to_string());
        }
        if application.sign.contains("value") {
            shape_exclusions.push("Shape gives substantial value to goods (s.3(2)(c))".to_string());
        }
    }

    // Check public policy (s.3(3))
    let offensive_terms = ["offensive", "immoral", "scandalous"];
    let contrary_to_policy = offensive_terms.iter().any(|term| sign_lower.contains(term));

    Ok(AbsoluteGroundsRefusal {
        lacks_distinctiveness,
        descriptive: is_descriptive,
        customary: is_customary,
        shape_exclusions,
        contrary_to_policy,
        deceptive: sign_lower.contains("deceptive"),
        prohibited: false,
        bad_faith: false,
    })
}

/// Checks relative grounds for refusal (s.5)
pub fn check_relative_grounds(
    application: &TradeMarkApplication,
) -> IpResult<RelativeGroundsRefusal> {
    let mut identical_mark = None;
    let mut confusion = None;

    for earlier in &application.earlier_marks {
        // Check s.5(1): Identical mark on identical goods
        if earlier.sign.to_lowercase() == application.sign.to_lowercase() {
            let identical_goods = application.goods_services.iter().any(|goods| {
                earlier
                    .goods_services
                    .iter()
                    .any(|eg| eg.to_lowercase() == goods.to_lowercase())
            });

            if identical_goods {
                identical_mark = Some(earlier.sign.clone());
                break;
            }
        }

        // Check s.5(2): Likelihood of confusion
        let likelihood = assess_likelihood_of_confusion(application, earlier)?;
        if matches!(
            likelihood.likelihood,
            ConfusionLikelihood::High | ConfusionLikelihood::Certain
        ) {
            confusion = Some(likelihood);
            break;
        }
    }

    Ok(RelativeGroundsRefusal {
        identical_mark_identical_goods: identical_mark,
        likelihood_of_confusion: confusion,
        reputation_mark: None, // Simplified
    })
}

/// Assesses likelihood of confusion (s.5(2))
pub fn assess_likelihood_of_confusion(
    application: &TradeMarkApplication,
    earlier_mark: &TradeMark,
) -> IpResult<LikelihoodOfConfusion> {
    // Visual/phonetic similarity
    let sign_similarity = calculate_sign_similarity(&application.sign, &earlier_mark.sign);

    // Goods/services similarity
    let goods_similarity =
        calculate_goods_similarity(&application.goods_services, &earlier_mark.goods_services);

    // Global appreciation test (considering all factors)
    let likelihood = if sign_similarity > 0.9 && goods_similarity > 0.9 {
        ConfusionLikelihood::Certain
    } else if sign_similarity > 0.7 && goods_similarity > 0.7 {
        ConfusionLikelihood::High
    } else if sign_similarity > 0.5 && goods_similarity > 0.5 {
        ConfusionLikelihood::Medium
    } else if sign_similarity > 0.3 || goods_similarity > 0.3 {
        ConfusionLikelihood::Low
    } else {
        ConfusionLikelihood::None
    };

    let factors = vec![
        format!("Sign similarity: {:.2}", sign_similarity),
        format!("Goods similarity: {:.2}", goods_similarity),
        format!(
            "Earlier mark reputation: {}",
            earlier_mark.registration_date.is_some()
        ),
    ];

    Ok(LikelihoodOfConfusion {
        earlier_mark: earlier_mark.sign.clone(),
        sign_similarity,
        goods_similarity,
        likelihood,
        factors,
    })
}

/// Validates trademark application
pub fn validate_trademark(application: &TradeMarkApplication) -> IpResult<()> {
    if application.sign.trim().is_empty() {
        return Err(IpError::MissingInformation {
            field: "sign".to_string(),
        });
    }

    if application.goods_services.is_empty() {
        return Err(IpError::MissingInformation {
            field: "goods_services".to_string(),
        });
    }

    // Check absolute grounds
    let absolute = check_absolute_grounds(application)?;

    if absolute.lacks_distinctiveness {
        return Err(IpError::LacksDistinctiveness);
    }

    if absolute.descriptive {
        return Err(IpError::Descriptive {
            description: application.goods_services.join(", "),
        });
    }

    if absolute.customary {
        return Err(IpError::Customary);
    }

    if absolute.contrary_to_policy || absolute.deceptive {
        return Err(IpError::Deceptive);
    }

    // Check relative grounds
    let relative = check_relative_grounds(application)?;

    if let Some(earlier) = relative.identical_mark_identical_goods {
        return Err(IpError::ConflictingTradeMark {
            earlier_mark: earlier,
        });
    }

    if let Some(confusion) = relative.likelihood_of_confusion {
        if matches!(
            confusion.likelihood,
            ConfusionLikelihood::High | ConfusionLikelihood::Certain
        ) {
            return Err(IpError::LikelihoodOfConfusion {
                earlier_mark: confusion.earlier_mark,
            });
        }
    }

    Ok(())
}

/// Calculate visual/phonetic similarity (simplified Levenshtein-based)
fn calculate_sign_similarity(sign1: &str, sign2: &str) -> f64 {
    let s1_lower = sign1.to_lowercase();
    let s2_lower = sign2.to_lowercase();

    if s1_lower == s2_lower {
        return 1.0;
    }

    // Simplified: check for containment or prefix matching
    if s1_lower.contains(&s2_lower) || s2_lower.contains(&s1_lower) {
        return 0.8;
    }

    // Check first 3 characters (phonetic similarity approximation)
    if s1_lower.len() >= 3 && s2_lower.len() >= 3 {
        let prefix_match = s1_lower[..3] == s2_lower[..3];
        if prefix_match {
            return 0.6;
        }
    }

    // Calculate Levenshtein-based similarity
    let max_len = s1_lower.len().max(s2_lower.len());
    if max_len == 0 {
        return 1.0;
    }

    let distance = levenshtein_distance(&s1_lower, &s2_lower);
    1.0 - (distance as f64 / max_len as f64)
}

/// Simple Levenshtein distance calculation
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
        row[0] = i;
    }
    for (j, val) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
        *val = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for (i, c1) in s1_chars.iter().enumerate() {
        for (j, c2) in s2_chars.iter().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
}

/// Calculate goods/services similarity
fn calculate_goods_similarity(goods1: &[String], goods2: &[String]) -> f64 {
    if goods1.is_empty() || goods2.is_empty() {
        return 0.0;
    }

    let mut matches = 0;
    for g1 in goods1 {
        for g2 in goods2 {
            if g1.to_lowercase() == g2.to_lowercase()
                || g1.to_lowercase().contains(&g2.to_lowercase())
                || g2.to_lowercase().contains(&g1.to_lowercase())
            {
                matches += 1;
            }
        }
    }

    let total = goods1.len().max(goods2.len());
    matches as f64 / total as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_similarity_identical() {
        let sim = calculate_sign_similarity("APPLE", "APPLE");
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_sign_similarity_case_insensitive() {
        let sim = calculate_sign_similarity("Apple", "APPLE");
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_sign_similarity_phonetically_similar() {
        let sim = calculate_sign_similarity("APLE", "APPLE");
        assert!(sim > 0.5);
    }

    #[test]
    fn test_goods_similarity_identical() {
        let goods1 = vec!["Computers".to_string()];
        let goods2 = vec!["Computers".to_string()];
        let sim = calculate_goods_similarity(&goods1, &goods2);
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_validate_trademark_valid() {
        let app = TradeMarkApplication {
            sign: "UNIQUEBRAND".to_string(),
            mark_type: TradeMarkType::Word,
            goods_services: vec!["Software".to_string()],
            nice_classes: vec![9],
            applicant: IpOwner {
                name: "Tech Co".to_string(),
                owner_type: super::super::types::OwnerType::Company,
                address: None,
                country: "GB".to_string(),
            },
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            use_date: None,
            earlier_marks: vec![],
        };

        assert!(validate_trademark(&app).is_ok());
    }

    #[test]
    fn test_validate_trademark_lacks_distinctiveness() {
        let app = TradeMarkApplication {
            sign: "A".to_string(), // Single letter lacks distinctiveness
            mark_type: TradeMarkType::Word,
            goods_services: vec!["Software".to_string()],
            nice_classes: vec![9],
            applicant: IpOwner {
                name: "Tech Co".to_string(),
                owner_type: super::super::types::OwnerType::Company,
                address: None,
                country: "GB".to_string(),
            },
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            use_date: None,
            earlier_marks: vec![],
        };

        let result = validate_trademark(&app);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IpError::LacksDistinctiveness));
    }

    #[test]
    fn test_validate_trademark_conflicting() {
        let earlier = TradeMark {
            sign: "TECHBRAND".to_string(),
            mark_type: TradeMarkType::Word,
            goods_services: vec!["Software".to_string()],
            nice_classes: vec![9],
            owner: None,
            registration_number: Some("UK12345".to_string()),
            registration_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            renewal_date: None,
            status: RegistrationStatus::Granted,
        };

        let app = TradeMarkApplication {
            sign: "TECHBRAND".to_string(), // Identical
            mark_type: TradeMarkType::Word,
            goods_services: vec!["Software".to_string()], // Identical
            nice_classes: vec![9],
            applicant: IpOwner {
                name: "New Co".to_string(),
                owner_type: super::super::types::OwnerType::Company,
                address: None,
                country: "GB".to_string(),
            },
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            use_date: None,
            earlier_marks: vec![earlier],
        };

        let result = validate_trademark(&app);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IpError::ConflictingTradeMark { .. }
        ));
    }

    #[test]
    fn test_likelihood_of_confusion_high() {
        let earlier = TradeMark {
            sign: "APPLE".to_string(),
            mark_type: TradeMarkType::Word,
            goods_services: vec!["Computers".to_string()],
            nice_classes: vec![9],
            owner: None,
            registration_number: Some("UK00000".to_string()),
            registration_date: Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
            renewal_date: None,
            status: RegistrationStatus::Granted,
        };

        let app = TradeMarkApplication {
            sign: "APLE".to_string(), // Similar
            mark_type: TradeMarkType::Word,
            goods_services: vec!["Computers".to_string()], // Identical
            nice_classes: vec![9],
            applicant: IpOwner {
                name: "Competitor".to_string(),
                owner_type: super::super::types::OwnerType::Company,
                address: None,
                country: "GB".to_string(),
            },
            filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            use_date: None,
            earlier_marks: vec![],
        };

        let confusion = assess_likelihood_of_confusion(&app, &earlier).unwrap();
        assert!(matches!(
            confusion.likelihood,
            ConfusionLikelihood::High | ConfusionLikelihood::Medium
        ));
    }
}
