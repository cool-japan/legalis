//! IP Validation Logic
//!
//! This module provides validation functions for all four IP types:
//! - Patents: Novelty, inventive step, industrial application
//! - Trademarks: Distinctiveness, similarity, deceptiveness
//! - Copyright: Term calculation, fair dealing assessment
//! - Designs: Novelty, individual character, functionality

use super::error::{IpError, Result};
use super::types::*;
use chrono::{Datelike, NaiveDate};

// ========== PATENT VALIDATION ==========

/// Validate patent eligibility and term
pub fn validate_patent(patent: &Patent) -> Result<PatentValidationReport> {
    let mut report = PatentValidationReport {
        is_valid: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        years_remaining: 0,
    };

    // Check if expired
    let years_since_filing = patent.years_since_filing();
    if years_since_filing > 20 {
        report.is_valid = false;
        report.errors.push(IpError::PatentExpired {
            years_ago: years_since_filing as u32,
        });
    } else {
        report.years_remaining = 20 - years_since_filing as u32;
    }

    // Check for prior art (simplified - check if any prior art is highly relevant)
    for prior_art in &patent.prior_art {
        if prior_art.relevance == PriorArtRelevance::HighlyRelevant {
            report.is_valid = false;
            report.errors.push(IpError::LacksNovelty {
                description: prior_art.description.clone(),
            });
        }
    }

    // Warn if approaching expiry
    if report.years_remaining <= 2 && report.years_remaining > 0 {
        report.warnings.push(format!(
            "Patent expiring soon: {} years remaining",
            report.years_remaining
        ));
    }

    // Check claims exist
    if patent.claims.is_empty() {
        report
            .warnings
            .push("No claims defined - patent scope unclear".to_string());
    }

    Ok(report)
}

/// Patent validation report
#[derive(Debug, Clone)]
pub struct PatentValidationReport {
    pub is_valid: bool,
    pub errors: Vec<IpError>,
    pub warnings: Vec<String>,
    pub years_remaining: u32,
}

/// Check if an invention meets patentability criteria
///
/// Three requirements (Patents Act s. 13):
/// 1. Novelty (s. 14) - not part of prior art
/// 2. Inventive step (s. 15) - not obvious to person skilled in the art
/// 3. Industrial application (s. 16) - can be made/used in industry
pub fn assess_patentability(
    _invention_description: &str,
    has_prior_art: bool,
    is_obvious: bool,
    is_industrially_applicable: bool,
) -> Result<()> {
    if has_prior_art {
        return Err(IpError::LacksNovelty {
            description: "Prior art exists in the same field".to_string(),
        });
    }

    if is_obvious {
        return Err(IpError::LacksInventiveStep);
    }

    if !is_industrially_applicable {
        return Err(IpError::NotIndustriallyApplicable);
    }

    Ok(())
}

// ========== TRADEMARK VALIDATION ==========

/// Validate trademark registration and detect conflicts
pub fn validate_trademark(
    trademark: &Trademark,
    existing_marks: &[Trademark],
) -> Result<TrademarkValidationReport> {
    let mut report = TrademarkValidationReport {
        is_registrable: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        conflicts: Vec::new(),
    };

    // Check Nice Classification validity (1-45)
    for &class in &trademark.classes {
        if !(1..=45).contains(&class) {
            report.is_registrable = false;
            report.errors.push(IpError::InvalidClass {
                class: class as u32,
            });
        }
    }

    // Check distinctiveness (simplified - check for generic words)
    let generic_terms = [
        "computer", "phone", "software", "clothing", "food", "service",
    ];
    let mark_lower = trademark.mark.to_lowercase();
    if generic_terms.iter().any(|term| mark_lower == *term) {
        report.is_registrable = false;
        report.errors.push(IpError::PurelyDescriptive {
            description: format!("'{}' is a generic term", trademark.mark),
        });
    }

    // Check for prohibited elements (simplified)
    let prohibited = ["singapore", "government", "official", "royal"];
    if prohibited.iter().any(|term| mark_lower.contains(*term)) {
        report
            .warnings
            .push("Contains potentially prohibited element - may require approval".to_string());
    }

    // Check for conflicts with existing marks
    for existing in existing_marks {
        // Check if in same class
        let has_common_class = trademark
            .classes
            .iter()
            .any(|c| existing.classes.contains(c));

        if has_common_class {
            let similarity = trademark.similarity_score(existing);
            if similarity >= 70 {
                // 70% threshold for similarity
                report.is_registrable = false;
                report.errors.push(IpError::TrademarkTooSimilar {
                    existing: existing.mark.clone(),
                    class: trademark.classes[0],
                    score: similarity,
                });
                report.conflicts.push(TrademarkConflict {
                    conflicting_mark: existing.mark.clone(),
                    conflicting_registration: existing.registration_number.clone(),
                    similarity_score: similarity,
                    common_classes: trademark
                        .classes
                        .iter()
                        .filter(|c| existing.classes.contains(c))
                        .copied()
                        .collect(),
                });
            } else if similarity >= 50 {
                report.warnings.push(format!(
                    "Moderate similarity to '{}': {}%",
                    existing.mark, similarity
                ));
            }
        }
    }

    // Check renewal status (if registered)
    if trademark.status == TrademarkStatus::Registered && trademark.needs_renewal() {
        report.warnings.push(format!(
            "Trademark needs renewal - {} years since registration",
            trademark.years_since_registration().unwrap_or(0)
        ));
    }

    Ok(report)
}

/// Trademark validation report
#[derive(Debug, Clone)]
pub struct TrademarkValidationReport {
    pub is_registrable: bool,
    pub errors: Vec<IpError>,
    pub warnings: Vec<String>,
    pub conflicts: Vec<TrademarkConflict>,
}

/// Trademark conflict information
#[derive(Debug, Clone)]
pub struct TrademarkConflict {
    pub conflicting_mark: String,
    pub conflicting_registration: String,
    pub similarity_score: u8,
    pub common_classes: Vec<u8>,
}

/// Assess trademark distinctiveness
///
/// Trade Marks Act s. 7(1)(b): Must be capable of distinguishing
pub fn assess_distinctiveness(mark: &str, goods_description: &str) -> Result<u8> {
    let mark_lower = mark.to_lowercase();
    let goods_lower = goods_description.to_lowercase();

    // Score 0-100
    let mut score = 70; // Start with moderate distinctiveness

    // Reduce significantly if mark is identical to goods description
    if mark_lower == goods_lower {
        score -= 70; // Nearly eliminates distinctiveness
    }
    // Reduce if mark contains goods description or vice versa
    else if mark_lower.contains(&goods_lower) || goods_lower.contains(&mark_lower) {
        score -= 40;
    }

    // Reduce if very short (1-2 characters)
    if mark.len() <= 2 {
        score -= 30;
    }

    // Increase if unusual/coined word
    if mark.len() > 8 && !mark_lower.contains(' ') {
        score += 20;
    }

    if score < 30 {
        return Err(IpError::NotDistinctive { class: 1 }); // Class 1 as placeholder
    }

    Ok(score.min(100))
}

// ========== COPYRIGHT VALIDATION ==========

/// Validate copyright protection and term
pub fn validate_copyright(
    copyright: &Copyright,
    author_death_date: Option<NaiveDate>,
) -> Result<CopyrightValidationReport> {
    let mut report = CopyrightValidationReport {
        is_protected: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        years_remaining: None,
    };

    // Check if expired
    if copyright.is_expired(author_death_date) {
        report.is_protected = false;
        if let Some(death_date) = author_death_date {
            let years = copyright.years_since_author_death(death_date);
            report.errors.push(IpError::CopyrightExpired {
                years: years as u32,
                event: "author's death".to_string(),
            });
        } else if let Some(pub_date) = copyright.publication_date {
            let today = chrono::Utc::now().date_naive();
            let years = today.year() - pub_date.year();
            report.errors.push(IpError::CopyrightExpired {
                years: years as u32,
                event: "publication".to_string(),
            });
        }
    } else if let Some(expiry) = copyright.expiry_date(author_death_date) {
        let today = chrono::Utc::now().date_naive();
        let years_remaining = expiry.year() - today.year();
        report.years_remaining = Some(years_remaining as u32);

        if years_remaining <= 5 {
            report.warnings.push(format!(
                "Copyright expiring soon: {} years remaining",
                years_remaining
            ));
        }
    }

    // Check if work is copyrightable
    if copyright.title.is_empty() {
        report.warnings.push("Work has no title".to_string());
    }

    if copyright.authors.is_empty() {
        report.warnings.push("No authors specified".to_string());
    }

    Ok(report)
}

/// Copyright validation report
#[derive(Debug, Clone)]
pub struct CopyrightValidationReport {
    pub is_protected: bool,
    pub errors: Vec<IpError>,
    pub warnings: Vec<String>,
    pub years_remaining: Option<u32>,
}

/// Assess whether fair dealing applies
///
/// Copyright Act s. 35-42: Fair dealing exceptions
pub fn assess_fair_dealing(
    purpose: FairDealingPurpose,
    amount_used: f32, // Percentage of work used (0.0-1.0)
    is_commercial: bool,
    competes_with_original: bool,
) -> Result<bool> {
    // Fair dealing factors (simplified analysis):
    // 1. Purpose (research, criticism, news)
    // 2. Amount used (less is better)
    // 3. Commercial nature
    // 4. Effect on market

    let mut score = 0;

    // Purpose score
    match purpose {
        FairDealingPurpose::Research => score += 30,
        FairDealingPurpose::Criticism => score += 25,
        FairDealingPurpose::NewsReporting => score += 25,
        FairDealingPurpose::JudicialProceedings => score += 35,
        FairDealingPurpose::ProfessionalAdvice => score += 20,
    }

    // Amount used (less is better)
    if amount_used <= 0.1 {
        score += 30; // 10% or less
    } else if amount_used <= 0.25 {
        score += 20; // 25% or less
    } else if amount_used <= 0.5 {
        score += 10; // 50% or less
    }

    // Commercial nature (non-commercial favored)
    if !is_commercial {
        score += 20;
    }

    // Market effect (no competition favored)
    if !competes_with_original {
        score += 20;
    }

    // Threshold: 60+ for fair dealing
    if score >= 60 {
        Ok(true)
    } else {
        Err(IpError::FairDealingNotApplicable {
            reason: format!("Fair dealing score: {}/100 (threshold: 60)", score),
        })
    }
}

// ========== DESIGN VALIDATION ==========

/// Validate registered design eligibility
pub fn validate_design(
    design: &RegisteredDesign,
    existing_designs: &[RegisteredDesign],
) -> Result<DesignValidationReport> {
    let mut report = DesignValidationReport {
        is_registrable: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        similar_designs: Vec::new(),
    };

    // Check if expired
    if design.is_expired() {
        report.is_registrable = false;
        if let Some(years) = design.years_since_registration() {
            report.errors.push(IpError::DesignExpired {
                years: years as u32,
            });
        }
    }

    // Check for novelty (simplified - check if very similar designs exist)
    for existing in existing_designs {
        let similarity = assess_design_similarity(design, existing);
        if similarity >= 80 {
            report.is_registrable = false;
            report.errors.push(IpError::DesignLacksNovelty {
                description: format!(
                    "Very similar to design '{}': {}% similarity",
                    existing.title, similarity
                ),
            });
            report
                .similar_designs
                .push((existing.title.clone(), similarity));
        } else if similarity >= 60 {
            report.warnings.push(format!(
                "Moderately similar to '{}': {}%",
                existing.title, similarity
            ));
        }
    }

    // Check renewal status
    if let Some(renewal_date) = design.first_renewal_date() {
        let today = chrono::Utc::now().date_naive();
        if today > renewal_date {
            report
                .warnings
                .push("Design needs renewal (5-year term)".to_string());
        }
    }

    Ok(report)
}

/// Design validation report
#[derive(Debug, Clone)]
pub struct DesignValidationReport {
    pub is_registrable: bool,
    pub errors: Vec<IpError>,
    pub warnings: Vec<String>,
    pub similar_designs: Vec<(String, u8)>, // (title, similarity%)
}

/// Assess similarity between two designs (simplified)
///
/// Returns similarity score 0-100%
fn assess_design_similarity(design1: &RegisteredDesign, design2: &RegisteredDesign) -> u8 {
    let mut score = 0;

    // Check if same product category
    let common_products = design1
        .products
        .iter()
        .filter(|p| design2.products.contains(p))
        .count();

    if common_products > 0 {
        score += 40;
    }

    // Check Locarno classification overlap
    let common_classes = design1
        .locarno_classes
        .iter()
        .filter(|c| design2.locarno_classes.contains(c))
        .count();

    if common_classes > 0 {
        score += 30;
    }

    // Title similarity (basic string comparison)
    let title1 = design1.title.to_lowercase();
    let title2 = design2.title.to_lowercase();

    if title1 == title2 {
        score += 30;
    } else if title1.contains(&title2) || title2.contains(&title1) {
        score += 15;
    }

    score.min(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_validation_expired() {
        let mut patent = Patent::new(
            "SG001",
            "Old Invention",
            "Inventor",
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        );
        patent.status = PatentStatus::Granted;

        let report = validate_patent(&patent).unwrap();
        assert!(!report.is_valid);
        assert_eq!(report.years_remaining, 0);
    }

    #[test]
    fn test_trademark_similarity_detection() {
        let tm1 = Trademark {
            classes: vec![9],
            ..Trademark::new(
                "TM001",
                "ACME",
                "Acme Corp",
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            )
        };

        let tm2 = Trademark {
            classes: vec![9],
            ..Trademark::new(
                "TM002",
                "ACMEE",
                "Other Corp",
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            )
        };

        let report = validate_trademark(&tm1, &[tm2]).unwrap();
        assert!(!report.is_registrable);
        assert!(!report.errors.is_empty());
    }

    #[test]
    fn test_copyright_term_calculation() {
        let copyright = Copyright::new(
            "Novel",
            "Author",
            WorkType::Literary,
            NaiveDate::from_ymd_opt(1950, 1, 1).unwrap(),
        );

        let death_date = NaiveDate::from_ymd_opt(1980, 1, 1).unwrap();
        let expiry = copyright.expiry_date(Some(death_date));

        assert_eq!(expiry.unwrap().year(), 2050); // 1980 + 70
    }

    #[test]
    fn test_fair_dealing_research() {
        let result = assess_fair_dealing(
            FairDealingPurpose::Research,
            0.15,  // 15% used
            false, // Non-commercial
            false, // Doesn't compete
        );

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_fair_dealing_commercial_large_use() {
        let result = assess_fair_dealing(
            FairDealingPurpose::Research,
            0.80, // 80% used
            true, // Commercial
            true, // Competes with original
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_design_validation() {
        let design = RegisteredDesign {
            registration_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            status: DesignStatus::Registered,
            ..RegisteredDesign::new(
                "D001",
                "Chair Design",
                "Designer",
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            )
        };

        let report = validate_design(&design, &[]).unwrap();
        assert!(report.is_registrable);
    }

    #[test]
    fn test_distinctiveness_assessment() {
        // Distinctive mark
        let score1 = assess_distinctiveness("XEROX", "photocopiers");
        assert!(score1.is_ok());
        assert!(score1.unwrap() >= 50);

        // Descriptive mark
        let score2 = assess_distinctiveness("COMPUTER", "computer");
        assert!(score2.is_err());
    }
}
