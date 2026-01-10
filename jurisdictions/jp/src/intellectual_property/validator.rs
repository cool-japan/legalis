//! Intellectual Property Validators (知的財産法バリデータ)
//!
//! Validation functions for Japanese intellectual property law compliance.

use super::error::{IntellectualPropertyError, Result};
use super::types::*;
use chrono::Utc;

// ============================================================================
// Patent Validation (特許法検証)
// ============================================================================

/// Validate patent application (特許出願の検証)
///
/// Validates compliance with:
/// - Article 36: Application requirements
/// - Article 29: Patentability requirements
///
/// # Arguments
/// * `application` - Patent application to validate
///
/// # Returns
/// * `Ok(())` if application is valid
/// * `Err(IntellectualPropertyError)` if application violates patent law
pub fn validate_patent_application(application: &PatentApplication) -> Result<()> {
    // Validate required fields (Article 36)
    if application.application_number.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "application_number".to_string(),
        });
    }

    if application.title.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "title".to_string(),
        });
    }

    if application.inventors.is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "inventors".to_string(),
        });
    }

    if application.applicants.is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "applicants".to_string(),
        });
    }

    if application.claims.is_empty() {
        return Err(IntellectualPropertyError::ClaimsNotSupported);
    }

    // Validate claims content
    for claim in &application.claims {
        if claim.trim().is_empty() || claim.len() < 10 {
            return Err(IntellectualPropertyError::InsufficientDisclosure);
        }
    }

    // Validate abstract
    if application.abstract_text.trim().is_empty() || application.abstract_text.len() < 20 {
        return Err(IntellectualPropertyError::InsufficientDisclosure);
    }

    // Validate priority claim if present
    if let Some(priority_date) = application.priority_date {
        if priority_date > application.filing_date {
            return Err(IntellectualPropertyError::InvalidPriorityClaim {
                reason: "Priority date cannot be after filing date".to_string(),
            });
        }

        let days_diff = (application.filing_date - priority_date).num_days();
        if days_diff > 365 {
            return Err(IntellectualPropertyError::InvalidPriorityClaim {
                reason: "Priority claim must be filed within 12 months".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate patent grant (特許査定の検証)
///
/// Validates patent grant validity including protection period and fees.
///
/// # Arguments
/// * `grant` - Patent grant to validate
///
/// # Returns
/// * `Ok(())` if grant is currently valid
/// * `Err(IntellectualPropertyError)` if grant is invalid or expired
pub fn validate_patent_grant(grant: &PatentGrant) -> Result<()> {
    // Validate application first
    validate_patent_application(&grant.application)?;

    // Check if patent has expired
    if grant.application.is_protection_expired(grant.grant_date) {
        let expiry_date = grant.application.filing_date
            + chrono::Duration::days((PATENT_PROTECTION_YEARS * 365) as i64);
        return Err(IntellectualPropertyError::PatentExpired {
            expiry_date: format!("{}", expiry_date),
        });
    }

    // Check if annual fees are current
    if !grant.are_annual_fees_current() {
        let years_since_grant = (Utc::now() - grant.grant_date).num_days() as f64 / 365.0;
        return Err(IntellectualPropertyError::AnnualFeesNotPaid {
            year: years_since_grant.ceil() as u32,
        });
    }

    Ok(())
}

/// Check for potential patent infringement (特許権侵害チェック)
///
/// Performs preliminary check for patent infringement.
/// Full infringement analysis requires detailed technical and legal review.
///
/// # Arguments
/// * `patent` - Valid patent grant
/// * `product_description` - Description of potentially infringing product
///
/// # Returns
/// * `Ok(())` if no obvious infringement
/// * `Err(IntellectualPropertyError)` if potential infringement detected
pub fn check_patent_infringement(patent: &PatentGrant, product_description: &str) -> Result<()> {
    // Ensure patent is valid first
    validate_patent_grant(patent)?;

    // Simple keyword matching for preliminary check
    // In practice, this requires detailed claim-by-claim analysis
    let description_lower = product_description.to_lowercase();

    for claim in &patent.application.claims {
        let claim_lower = claim.to_lowercase();
        let claim_words: Vec<&str> = claim_lower.split_whitespace().collect();

        if claim_words.len() >= 3 {
            let mut matches = 0;
            for word in &claim_words {
                if word.len() > 3 && description_lower.contains(word) {
                    matches += 1;
                }
            }

            // If significant overlap in keywords, flag potential infringement
            if matches as f64 / claim_words.len() as f64 > 0.5 {
                return Err(IntellectualPropertyError::PatentInfringement {
                    description: format!(
                        "Product may infringe claim: {}",
                        &claim[..claim.len().min(50)]
                    ),
                });
            }
        }
    }

    Ok(())
}

// ============================================================================
// Copyright Validation (著作権法検証)
// ============================================================================

/// Validate copyrighted work (著作物の検証)
///
/// Validates compliance with:
/// - Article 2-1-1: Work definition
/// - Article 10: Work categories
///
/// # Arguments
/// * `work` - Copyrighted work to validate
///
/// # Returns
/// * `Ok(())` if work is valid
/// * `Err(IntellectualPropertyError)` if work is not copyrightable
pub fn validate_copyrighted_work(work: &CopyrightedWork) -> Result<()> {
    // Validate required fields
    if work.title.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "title".to_string(),
        });
    }

    if work.authors.is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "authors".to_string(),
        });
    }

    if work.copyright_holder.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "copyright_holder".to_string(),
        });
    }

    // Check for originality - very basic check
    if work.title.len() < 3 {
        return Err(IntellectualPropertyError::LacksOriginality);
    }

    // Validate dates
    if let Some(pub_date) = work.first_publication_date {
        if pub_date < work.creation_date {
            return Err(IntellectualPropertyError::InvalidDate {
                reason: "Publication date cannot be before creation date".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate fair use claim (権利制限の妥当性検証)
///
/// Validates whether a fair use claim is valid under Articles 30-47-8.
///
/// # Arguments
/// * `fair_use_type` - Type of fair use claimed
/// * `use_description` - Description of how work is used
///
/// # Returns
/// * `Ok(())` if fair use appears valid
/// * `Err(IntellectualPropertyError)` if fair use is not applicable
pub fn validate_fair_use(fair_use_type: FairUseType, use_description: &str) -> Result<()> {
    if use_description.trim().len() < 20 {
        return Err(IntellectualPropertyError::FairUseNotApplicable {
            reason: "Insufficient description of use".to_string(),
        });
    }

    match fair_use_type {
        FairUseType::Quotation => {
            // Article 32: Quotation must be for criticism, research, etc.
            // and must clearly distinguish quoted portion
            if !use_description.to_lowercase().contains("quot")
                && !use_description.to_lowercase().contains("引用")
            {
                return Err(IntellectualPropertyError::InvalidQuotation {
                    reason: "Use does not appear to be proper quotation".to_string(),
                });
            }
        }
        FairUseType::PrivateUse => {
            // Article 30: Must be for personal use only
            if use_description.to_lowercase().contains("commercial")
                || use_description.to_lowercase().contains("public")
                || use_description.to_lowercase().contains("営利")
            {
                return Err(IntellectualPropertyError::FairUseNotApplicable {
                    reason: "Private use exception does not apply to commercial or public use"
                        .to_string(),
                });
            }
        }
        FairUseType::EducationalUse => {
            // Article 35: Must be for educational purposes
            if !use_description.to_lowercase().contains("education")
                && !use_description.to_lowercase().contains("teaching")
                && !use_description.to_lowercase().contains("教育")
            {
                return Err(IntellectualPropertyError::FairUseNotApplicable {
                    reason: "Educational use exception requires genuine educational context"
                        .to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

/// Check for copyright infringement (著作権侵害チェック)
///
/// Performs preliminary analysis of copyright infringement claim.
///
/// # Arguments
/// * `infringement` - Copyright infringement claim
///
/// # Returns
/// * `Ok(())` if claim appears valid
/// * `Err(IntellectualPropertyError)` if claim is invalid
pub fn validate_copyright_infringement(infringement: &CopyrightInfringement) -> Result<()> {
    // Validate the original work
    validate_copyrighted_work(&infringement.original_work)?;

    // Check if rights claimed are valid
    if infringement.rights_infringed.is_empty() {
        return Err(IntellectualPropertyError::ValidationError {
            message: "No rights specified as infringed".to_string(),
        });
    }

    // If fair use is claimed, validate it
    if let Some(fair_use) = infringement.fair_use_claim {
        // Fair use claim means no infringement
        validate_fair_use(fair_use, &infringement.alleged_infringing_work)?;
        return Err(IntellectualPropertyError::FairUseNotApplicable {
            reason: "Fair use claim validated - no infringement".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Trademark Validation (商標法検証)
// ============================================================================

/// Validate trademark application (商標出願の検証)
///
/// Validates compliance with:
/// - Article 3: Trademark distinctiveness
/// - Article 5: Application requirements
///
/// # Arguments
/// * `application` - Trademark application to validate
///
/// # Returns
/// * `Ok(())` if application is valid
/// * `Err(IntellectualPropertyError)` if application is invalid
pub fn validate_trademark_application(application: &TrademarkApplication) -> Result<()> {
    // Validate required fields
    if application.application_number.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "application_number".to_string(),
        });
    }

    if application.trademark_representation.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "trademark_representation".to_string(),
        });
    }

    if application.applicant.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "applicant".to_string(),
        });
    }

    if application.designated_classes.is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "designated_classes".to_string(),
        });
    }

    if application.designated_goods_services.is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "designated_goods_services".to_string(),
        });
    }

    // Validate Nice Classes (1-45)
    for &class in &application.designated_classes {
        if !(1..=45).contains(&class) {
            return Err(IntellectualPropertyError::InvalidClassDesignation { class });
        }
    }

    // Check for distinctiveness (Article 3)
    let mark_lower = application.trademark_representation.to_lowercase();

    // Check if it's merely descriptive
    let descriptive_words = [
        "best",
        "quality",
        "super",
        "premium",
        "standard",
        "excellent",
        "good",
        "new",
    ];
    if descriptive_words.iter().any(|&word| mark_lower == word) {
        return Err(IntellectualPropertyError::MerelyDescriptive {
            description: "Trademark consists only of descriptive terms".to_string(),
        });
    }

    // Check minimum length for word marks
    if application.trademark_type == TrademarkType::Word && mark_lower.len() < 2 {
        return Err(IntellectualPropertyError::LacksDistinctiveness);
    }

    Ok(())
}

/// Validate trademark registration (商標登録の検証)
///
/// Validates trademark registration validity.
///
/// # Arguments
/// * `registration` - Trademark registration to validate
///
/// # Returns
/// * `Ok(())` if registration is currently valid
/// * `Err(IntellectualPropertyError)` if registration is expired
pub fn validate_trademark_registration(registration: &TrademarkRegistration) -> Result<()> {
    // Validate application first
    validate_trademark_application(&registration.application)?;

    // Check if registration is still valid
    if !registration.is_valid() {
        let last_renewal = registration
            .last_renewal_date
            .unwrap_or(registration.registration_date);
        let expiry_date =
            last_renewal + chrono::Duration::days((TRADEMARK_RENEWAL_YEARS * 365) as i64);
        return Err(IntellectualPropertyError::TrademarkExpired {
            expiry_date: format!("{}", expiry_date),
        });
    }

    Ok(())
}

/// Assess trademark similarity (商標類似性評価)
///
/// Performs basic similarity assessment between two trademarks.
/// Full similarity assessment requires expert analysis.
///
/// # Arguments
/// * `mark1` - First trademark
/// * `mark2` - Second trademark
///
/// # Returns
/// * Similarity level between the marks
pub fn assess_trademark_similarity(mark1: &str, mark2: &str) -> SimilarityLevel {
    let m1 = mark1.to_lowercase();
    let m2 = mark2.to_lowercase();

    // Exact match
    if m1 == m2 {
        return SimilarityLevel::Identical;
    }

    // Calculate simple similarity metrics
    let max_len = m1.len().max(m2.len()) as f64;

    // Levenshtein-like simple check
    let mut matches = 0;
    let shorter = if m1.len() < m2.len() { &m1 } else { &m2 };
    let longer = if m1.len() >= m2.len() { &m1 } else { &m2 };

    for (i, c) in shorter.chars().enumerate() {
        if i < longer.len() && longer.chars().nth(i) == Some(c) {
            matches += 1;
        }
    }

    let similarity_ratio = matches as f64 / max_len;

    if similarity_ratio > 0.8 {
        SimilarityLevel::HighlySimilar
    } else if similarity_ratio > 0.6 {
        SimilarityLevel::Similar
    } else if similarity_ratio > 0.4 {
        SimilarityLevel::SomewhatSimilar
    } else {
        SimilarityLevel::NotSimilar
    }
}

// ============================================================================
// Design Validation (意匠法検証)
// ============================================================================

/// Validate design application (意匠出願の検証)
///
/// Validates compliance with:
/// - Article 3: Design requirements
/// - Article 6: Application requirements
///
/// # Arguments
/// * `application` - Design application to validate
///
/// # Returns
/// * `Ok(())` if application is valid
/// * `Err(IntellectualPropertyError)` if application is invalid
pub fn validate_design_application(application: &DesignApplication) -> Result<()> {
    // Validate required fields
    if application.application_number.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "application_number".to_string(),
        });
    }

    if application.article_title.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "article_title".to_string(),
        });
    }

    if application.designers.is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "designers".to_string(),
        });
    }

    if application.applicant.trim().is_empty() {
        return Err(IntellectualPropertyError::MissingRequiredField {
            field_name: "applicant".to_string(),
        });
    }

    if application.description.trim().is_empty() || application.description.len() < 10 {
        return Err(IntellectualPropertyError::NotRegistrableDesign {
            reason: "Insufficient design description".to_string(),
        });
    }

    // Validate related design if specified
    if application.category == DesignCategory::Related
        && application.related_design_application.is_none()
    {
        return Err(IntellectualPropertyError::ValidationError {
            message: "Related design must specify base design application".to_string(),
        });
    }

    Ok(())
}

/// Validate design registration (意匠登録の検証)
///
/// Validates design registration validity.
///
/// # Arguments
/// * `registration` - Design registration to validate
///
/// # Returns
/// * `Ok(())` if registration is currently valid
/// * `Err(IntellectualPropertyError)` if registration is expired
pub fn validate_design_registration(registration: &DesignRegistration) -> Result<()> {
    // Validate application first
    validate_design_application(&registration.application)?;

    // Check if protection has expired
    if registration.is_protection_expired() {
        let expiry_date = registration.registration_date
            + chrono::Duration::days((DESIGN_PROTECTION_YEARS * 365) as i64);
        return Err(IntellectualPropertyError::DesignExpired {
            expiry_date: format!("{}", expiry_date),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_patent_application_valid() {
        let app = PatentApplication {
            application_number: "2020-123456".to_string(),
            filing_date: Utc::now(),
            title: "Novel Invention".to_string(),
            inventors: vec!["Inventor A".to_string()],
            applicants: vec!["Applicant Corp".to_string()],
            category: InventionCategory::Product,
            claims: vec![
                "A device comprising component X configured to perform function Y".to_string(),
            ],
            abstract_text: "This invention relates to a novel device for improving efficiency"
                .to_string(),
            priority_date: None,
            examination_requested: false,
        };

        assert!(validate_patent_application(&app).is_ok());
    }

    #[test]
    fn test_validate_patent_application_missing_claims() {
        let app = PatentApplication {
            application_number: "2020-123456".to_string(),
            filing_date: Utc::now(),
            title: "Test".to_string(),
            inventors: vec!["Inventor".to_string()],
            applicants: vec!["Applicant".to_string()],
            category: InventionCategory::Product,
            claims: vec![],
            abstract_text: "Abstract text".to_string(),
            priority_date: None,
            examination_requested: false,
        };

        assert!(matches!(
            validate_patent_application(&app),
            Err(IntellectualPropertyError::ClaimsNotSupported)
        ));
    }

    #[test]
    fn test_validate_trademark_application_valid() {
        let app = TrademarkApplication {
            application_number: "2020-123456".to_string(),
            filing_date: Utc::now(),
            trademark_representation: "INNOVATECH".to_string(),
            trademark_type: TrademarkType::Word,
            applicant: "Tech Corp".to_string(),
            designated_classes: vec![9, 42],
            designated_goods_services: vec!["Computer software".to_string()],
        };

        assert!(validate_trademark_application(&app).is_ok());
    }

    #[test]
    fn test_validate_trademark_invalid_class() {
        let app = TrademarkApplication {
            application_number: "2020-123456".to_string(),
            filing_date: Utc::now(),
            trademark_representation: "TEST".to_string(),
            trademark_type: TrademarkType::Word,
            applicant: "Corp".to_string(),
            designated_classes: vec![50], // Invalid - must be 1-45
            designated_goods_services: vec!["Goods".to_string()],
        };

        assert!(matches!(
            validate_trademark_application(&app),
            Err(IntellectualPropertyError::InvalidClassDesignation { .. })
        ));
    }

    #[test]
    fn test_assess_trademark_similarity() {
        assert_eq!(
            assess_trademark_similarity("ACME", "ACME"),
            SimilarityLevel::Identical
        );
        // "ACME" vs "ACNE" - 3/4 match, should be at least Similar
        let acme_acne = assess_trademark_similarity("ACME", "ACNE");
        assert!(matches!(
            acme_acne,
            SimilarityLevel::HighlySimilar | SimilarityLevel::Similar
        ));
        assert!(matches!(
            assess_trademark_similarity("ACME", "WXYZ"),
            SimilarityLevel::NotSimilar
        ));
    }

    #[test]
    fn test_validate_copyrighted_work() {
        let work = CopyrightedWork {
            title: "My Novel".to_string(),
            authors: vec!["Author Name".to_string()],
            category: WorkCategory::Literary,
            creation_date: Utc::now(),
            first_publication_date: Some(Utc::now()),
            copyright_holder: "Publisher Inc".to_string(),
            is_work_for_hire: false,
            derivative_source: None,
        };

        assert!(validate_copyrighted_work(&work).is_ok());
    }

    #[test]
    fn test_validate_design_application() {
        let app = DesignApplication {
            application_number: "2020-001234".to_string(),
            filing_date: Utc::now(),
            article_title: "Chair".to_string(),
            designers: vec!["Designer".to_string()],
            applicant: "Furniture Co".to_string(),
            category: DesignCategory::Product,
            description: "Ergonomic chair with novel backrest design".to_string(),
            related_design_application: None,
        };

        assert!(validate_design_application(&app).is_ok());
    }
}
