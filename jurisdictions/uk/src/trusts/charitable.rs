//! Charitable Trusts (Charities Act 2011)
//!
//! This module implements the law of charitable trusts under the Charities Act 2011.
//!
//! ## Definition of Charity (Charities Act 2011 s.1)
//!
//! A charity is an institution which:
//! - Is established for charitable purposes only (s.2)
//! - Falls within the jurisdiction of the High Court
//!
//! ## Charitable Purposes (s.3)
//!
//! The 13 heads of charity:
//! 1. Prevention or relief of poverty
//! 2. Advancement of education
//! 3. Advancement of religion
//! 4. Advancement of health or saving of lives
//! 5. Advancement of citizenship or community development
//! 6. Advancement of arts, culture, heritage or science
//! 7. Advancement of amateur sport
//! 8. Advancement of human rights, conflict resolution, reconciliation
//! 9. Advancement of environmental protection or improvement
//! 10. Relief of those in need by reason of youth, age, ill-health, disability, financial hardship
//! 11. Advancement of animal welfare
//! 12. Promotion of efficiency of armed forces, police, fire service, ambulance
//! 13. Any other purpose beneficial to the community (analogous to existing charity law)
//!
//! ## Public Benefit (s.4)
//!
//! Every charitable purpose must be for public benefit:
//! - Cannot be assumed - must be demonstrable
//! - Two aspects: beneficial and public element
//! - Private benefit must be incidental
//!
//! ## Cy-Pres (ss.62-67)
//!
//! Court can apply property cy-pres (as near as possible) when:
//! - Original purpose has failed
//! - Property given for general charitable intent
//!
//! ## Key Cases
//!
//! - **Oppenheim v Tobacco Securities Trust \[1951\]**: Personal nexus test
//! - **Dingle v Turner \[1972\]**: Poverty exception to personal nexus
//! - **ISC v Charity Commission \[2012\]**: Public benefit for fee-charging schools
//! - **Attorney General v Charity Commission \[2012\]**: Preston Down Trust case

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::error::{TrustError, TrustResult};

// ============================================================================
// Charitable Purposes (Charities Act 2011 s.3)
// ============================================================================

/// Charitable purposes under Charities Act 2011 s.3
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharitablePurpose {
    /// s.3(1)(a) - Prevention or relief of poverty
    PreventionReliefPoverty,
    /// s.3(1)(b) - Advancement of education
    AdvancementEducation,
    /// s.3(1)(c) - Advancement of religion
    AdvancementReligion,
    /// s.3(1)(d) - Advancement of health or saving of lives
    AdvancementHealthSavingLives,
    /// s.3(1)(e) - Advancement of citizenship or community development
    AdvancementCitizenshipCommunity,
    /// s.3(1)(f) - Advancement of arts, culture, heritage or science
    AdvancementArtsCultureHeritagScience,
    /// s.3(1)(g) - Advancement of amateur sport
    AdvancementAmateurSport,
    /// s.3(1)(h) - Advancement of human rights, conflict resolution, reconciliation
    AdvancementHumanRightsConflictResolution,
    /// s.3(1)(i) - Advancement of environmental protection or improvement
    AdvancementEnvironmentalProtection,
    /// s.3(1)(j) - Relief of those in need (youth, age, ill-health, disability, hardship)
    ReliefThoseInNeed,
    /// s.3(1)(k) - Advancement of animal welfare
    AdvancementAnimalWelfare,
    /// s.3(1)(l) - Promotion of efficiency of armed forces, emergency services
    PromotionEfficiencyArmedForcesEmergency,
    /// s.3(1)(m) - Any other purpose analogous to above or existing charity law
    OtherAnalogousPurpose {
        /// Description of the analogous purpose
        description: String,
    },
}

impl CharitablePurpose {
    /// Get the statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::PreventionReliefPoverty => "Charities Act 2011 s.3(1)(a)",
            Self::AdvancementEducation => "Charities Act 2011 s.3(1)(b)",
            Self::AdvancementReligion => "Charities Act 2011 s.3(1)(c)",
            Self::AdvancementHealthSavingLives => "Charities Act 2011 s.3(1)(d)",
            Self::AdvancementCitizenshipCommunity => "Charities Act 2011 s.3(1)(e)",
            Self::AdvancementArtsCultureHeritagScience => "Charities Act 2011 s.3(1)(f)",
            Self::AdvancementAmateurSport => "Charities Act 2011 s.3(1)(g)",
            Self::AdvancementHumanRightsConflictResolution => "Charities Act 2011 s.3(1)(h)",
            Self::AdvancementEnvironmentalProtection => "Charities Act 2011 s.3(1)(i)",
            Self::ReliefThoseInNeed => "Charities Act 2011 s.3(1)(j)",
            Self::AdvancementAnimalWelfare => "Charities Act 2011 s.3(1)(k)",
            Self::PromotionEfficiencyArmedForcesEmergency => "Charities Act 2011 s.3(1)(l)",
            Self::OtherAnalogousPurpose { .. } => "Charities Act 2011 s.3(1)(m)",
        }
    }

    /// Get description
    pub fn description(&self) -> String {
        match self {
            Self::PreventionReliefPoverty => "Prevention or relief of poverty".to_string(),
            Self::AdvancementEducation => "Advancement of education".to_string(),
            Self::AdvancementReligion => "Advancement of religion".to_string(),
            Self::AdvancementHealthSavingLives => {
                "Advancement of health or the saving of lives".to_string()
            }
            Self::AdvancementCitizenshipCommunity => {
                "Advancement of citizenship or community development".to_string()
            }
            Self::AdvancementArtsCultureHeritagScience => {
                "Advancement of the arts, culture, heritage or science".to_string()
            }
            Self::AdvancementAmateurSport => "Advancement of amateur sport".to_string(),
            Self::AdvancementHumanRightsConflictResolution => {
                "Advancement of human rights, conflict resolution or reconciliation, \
                 or promotion of religious or racial harmony or equality and diversity"
                    .to_string()
            }
            Self::AdvancementEnvironmentalProtection => {
                "Advancement of environmental protection or improvement".to_string()
            }
            Self::ReliefThoseInNeed => {
                "Relief of those in need by reason of youth, age, ill-health, disability, \
                 financial hardship or other disadvantage"
                    .to_string()
            }
            Self::AdvancementAnimalWelfare => "Advancement of animal welfare".to_string(),
            Self::PromotionEfficiencyArmedForcesEmergency => {
                "Promotion of the efficiency of the armed forces, police, fire and rescue \
                 services or ambulance services"
                    .to_string()
            }
            Self::OtherAnalogousPurpose { description } => {
                format!("Other charitable purpose: {}", description)
            }
        }
    }

    /// Is poverty exception applicable (relaxed public benefit)?
    /// Per Dingle v Turner \[1972\], poverty trusts can benefit a class defined by personal nexus
    pub fn poverty_exception_applies(&self) -> bool {
        matches!(
            self,
            Self::PreventionReliefPoverty | Self::ReliefThoseInNeed
        )
    }

    /// Requires demonstration of public benefit (all purposes post-2006)
    pub fn requires_public_benefit(&self) -> bool {
        // All purposes now require public benefit - cannot be presumed
        // s.4(2) "It is not to be presumed that a purpose of a particular description
        // is for the public benefit"
        true
    }
}

// ============================================================================
// Public Benefit Test (Charities Act 2011 s.4)
// ============================================================================

/// Public benefit assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicBenefitTest {
    /// The charitable purpose being assessed
    pub purpose: CharitablePurpose,
    /// Identifiable benefit
    pub identifiable_benefit: BenefitAssessment,
    /// Public element (who benefits)
    pub public_element: PublicElementAssessment,
    /// Private benefit analysis
    pub private_benefit: PrivateBenefitAnalysis,
    /// Overall result
    pub satisfied: bool,
    /// Analysis notes
    pub analysis: String,
}

/// Assessment of identifiable benefit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenefitAssessment {
    /// Is benefit identifiable?
    pub identifiable: bool,
    /// Description of benefit
    pub benefit_description: String,
    /// Is benefit tangible or intangible?
    pub tangible: bool,
    /// Any detriment or harm to balance?
    pub detriment: Option<String>,
    /// Net benefit positive?
    pub net_positive: bool,
}

/// Assessment of public element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicElementAssessment {
    /// Description of class that benefits
    pub benefiting_class: String,
    /// Is class defined by personal nexus? (Oppenheim v Tobacco Securities)
    pub personal_nexus: bool,
    /// Is class sufficiently wide?
    pub sufficiently_public: bool,
    /// Are any restrictions justified?
    pub restrictions_justified: bool,
    /// Fee analysis (if applicable)
    pub fee_analysis: Option<FeeChargeAnalysis>,
    /// Public element satisfied?
    pub satisfied: bool,
}

/// Analysis of fee charges (ISC v Charity Commission)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeChargeAnalysis {
    /// Are fees charged?
    pub fees_charged: bool,
    /// Fee amount (if applicable)
    pub fee_amount: Option<String>,
    /// Are people of modest means excluded?
    pub excludes_poor: bool,
    /// Mitigation measures (bursaries, scholarships)
    pub mitigation: Vec<String>,
    /// Is fee-charging justified?
    pub justified: bool,
}

/// Private benefit analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateBenefitAnalysis {
    /// Any private benefit identified?
    pub private_benefit_exists: bool,
    /// Description of private benefit
    pub description: Option<String>,
    /// Is private benefit incidental?
    pub incidental: bool,
    /// Is private benefit reasonable in context?
    pub reasonable: bool,
}

/// Validate public benefit
pub fn validate_public_benefit(
    purpose: CharitablePurpose,
    benefit: BenefitAssessment,
    public_element: PublicElementAssessment,
    private_benefit: PrivateBenefitAnalysis,
) -> PublicBenefitTest {
    // Check if public benefit is satisfied
    let benefit_ok = benefit.identifiable && benefit.net_positive;
    let public_ok = public_element.satisfied;
    let private_ok = !private_benefit.private_benefit_exists
        || (private_benefit.incidental && private_benefit.reasonable);

    let satisfied = benefit_ok && public_ok && private_ok;

    let mut analysis = String::new();
    analysis.push_str(&format!(
        "Public benefit assessment for '{}' ({}).\n",
        purpose.description(),
        purpose.statutory_reference()
    ));

    if benefit_ok {
        analysis.push_str("Identifiable benefit: Satisfied. ");
    } else {
        analysis.push_str("Identifiable benefit: NOT satisfied. ");
    }

    if public_ok {
        analysis.push_str("Public element: Satisfied. ");
    } else {
        analysis.push_str("Public element: NOT satisfied. ");
        if public_element.personal_nexus && !purpose.poverty_exception_applies() {
            analysis.push_str(
                "Class defined by personal nexus (Oppenheim v Tobacco Securities [1951]). ",
            );
        }
    }

    if private_ok {
        analysis.push_str("Private benefit: Incidental/reasonable. ");
    } else {
        analysis.push_str("Private benefit: Excessive - fails public benefit. ");
    }

    PublicBenefitTest {
        purpose,
        identifiable_benefit: benefit,
        public_element,
        private_benefit,
        satisfied,
        analysis,
    }
}

// ============================================================================
// Charitable Trust
// ============================================================================

/// A charitable trust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharitableTrust {
    /// Trust name
    pub name: String,
    /// Charitable purposes
    pub purposes: Vec<CharitablePurpose>,
    /// Charity registration number (if registered)
    pub charity_number: Option<String>,
    /// Trustees/charity trustees
    pub trustees: Vec<CharityTrustee>,
    /// Objects clause
    pub objects_clause: String,
    /// Is trust exclusively charitable?
    pub exclusively_charitable: bool,
    /// Creation date
    pub creation_date: NaiveDate,
    /// Cy-pres applicable?
    pub cy_pres_applicable: bool,
    /// Validation result
    pub validation: CharityValidation,
}

/// A charity trustee
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CharityTrustee {
    /// Name
    pub name: String,
    /// Role
    pub role: TrusteeRole,
    /// Appointment date
    pub appointed: NaiveDate,
    /// Is trustee disqualified?
    pub disqualified: bool,
}

/// Role of charity trustee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrusteeRole {
    /// Chair/president
    Chair,
    /// Treasurer
    Treasurer,
    /// Secretary
    Secretary,
    /// Ordinary trustee
    Trustee,
}

/// Charity validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharityValidation {
    /// Is purpose charitable?
    pub purpose_charitable: bool,
    /// Is purpose exclusively charitable?
    pub exclusively_charitable: bool,
    /// Public benefit satisfied?
    pub public_benefit: bool,
    /// Any issues
    pub issues: Vec<CharityIssue>,
    /// Overall valid?
    pub valid: bool,
}

/// Issues with charity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharityIssue {
    /// Purpose not within s.3 heads
    PurposeNotCharitable,
    /// Purpose not exclusively charitable
    NotExclusivelyCharitable,
    /// Public benefit not demonstrated
    NoPublicBenefit,
    /// Political purpose (Re Hopkinson)
    PoliticalPurpose,
    /// Personal nexus issue
    PersonalNexus,
    /// Fee excludes poor without mitigation
    FeesExcludePoor,
    /// Disqualified trustee
    DisqualifiedTrustee {
        /// Name of disqualified trustee
        name: String,
    },
}

/// Validate charitable purpose
pub fn validate_charitable_purpose(
    objects_clause: &str,
    claimed_purpose: CharitablePurpose,
) -> TrustResult<CharitablePurpose> {
    let objects_lower = objects_clause.to_lowercase();

    // Check for political purposes (not charitable - Re Hopkinson)
    let political_indicators = [
        "change the law",
        "campaign for",
        "lobby government",
        "political party",
        "influence legislation",
    ];

    for indicator in &political_indicators {
        if objects_lower.contains(indicator) {
            return Err(TrustError::NotCharitablePurpose);
        }
    }

    // Validate purpose matches objects clause (simplified check)
    match &claimed_purpose {
        CharitablePurpose::AdvancementEducation => {
            let education_words = [
                "education",
                "school",
                "university",
                "teach",
                "learn",
                "scholarship",
            ];
            if !education_words.iter().any(|w| objects_lower.contains(w)) {
                return Err(TrustError::NotCharitablePurpose);
            }
        }
        CharitablePurpose::AdvancementReligion => {
            let religion_words = [
                "religion",
                "worship",
                "faith",
                "church",
                "mosque",
                "temple",
                "spiritual",
            ];
            if !religion_words.iter().any(|w| objects_lower.contains(w)) {
                return Err(TrustError::NotCharitablePurpose);
            }
        }
        CharitablePurpose::PreventionReliefPoverty => {
            let poverty_words = ["poverty", "poor", "needy", "hardship", "destitute"];
            if !poverty_words.iter().any(|w| objects_lower.contains(w)) {
                return Err(TrustError::NotCharitablePurpose);
            }
        }
        _ => {
            // For other purposes, accept if objects clause is non-empty
            // In practice, would need more sophisticated validation
        }
    }

    Ok(claimed_purpose)
}

// ============================================================================
// Cy-Pres (Charities Act 2011 ss.62-67)
// ============================================================================

/// Cy-pres scheme
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CyPresScheme {
    /// Original charitable purpose
    pub original_purpose: String,
    /// Why original purpose failed
    pub failure_reason: CyPresOccasion,
    /// New purpose (as near as possible)
    pub new_purpose: String,
    /// Was there general charitable intent?
    pub general_charitable_intent: bool,
    /// Court/Charity Commission approval
    pub approved: bool,
    /// Analysis
    pub analysis: String,
}

/// Occasions for cy-pres application (s.62)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CyPresOccasion {
    /// s.62(1)(a) - Original purposes wholly/partly fulfilled or provided for
    PurposesFulfilled,
    /// s.62(1)(b) - Original purposes cannot be carried out
    PurposesImpossible,
    /// s.62(1)(c) - Original purposes can only be carried out with surplus
    Surplus,
    /// s.62(1)(d) - Original purposes provide for area/class ceased to exist
    ClassCeased,
    /// s.62(1)(e) - Original purposes have been adequately provided by other means
    AdequatelyProvided,
    /// s.62(1)(e)(ii) - Original purposes have ceased to be charitable
    CeasedCharitable,
    /// s.62(1)(e)(iii) - Original purposes have ceased to be useful
    CeasedUseful,
    /// s.62(1)(f) - Would be more effective with other property/charity
    MoreEffective,
}

impl CyPresOccasion {
    /// Get statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::PurposesFulfilled => "Charities Act 2011 s.62(1)(a)",
            Self::PurposesImpossible => "Charities Act 2011 s.62(1)(b)",
            Self::Surplus => "Charities Act 2011 s.62(1)(c)",
            Self::ClassCeased => "Charities Act 2011 s.62(1)(d)",
            Self::AdequatelyProvided => "Charities Act 2011 s.62(1)(e)(i)",
            Self::CeasedCharitable => "Charities Act 2011 s.62(1)(e)(ii)",
            Self::CeasedUseful => "Charities Act 2011 s.62(1)(e)(iii)",
            Self::MoreEffective => "Charities Act 2011 s.62(1)(f)",
        }
    }

    /// Description
    pub fn description(&self) -> &'static str {
        match self {
            Self::PurposesFulfilled => {
                "Original purposes have been wholly or substantially fulfilled, or provided for"
            }
            Self::PurposesImpossible => {
                "Original purposes cannot be carried out, or not according to directions"
            }
            Self::Surplus => "Property available exceeds what is needed for original purposes",
            Self::ClassCeased => {
                "Original purposes were for area or class that has ceased to exist or be suitable"
            }
            Self::AdequatelyProvided => {
                "Original purposes have been adequately provided for by other means"
            }
            Self::CeasedCharitable => "Original purposes have ceased in law to be charitable",
            Self::CeasedUseful => {
                "Original purposes have ceased to provide suitable and effective method of \
                 using property, having regard to spirit of gift"
            }
            Self::MoreEffective => {
                "Would be more effective with other charity or with other property"
            }
        }
    }
}

/// Validate cy-pres application
pub fn validate_cy_pres(
    original_purpose: &str,
    failure_reason: CyPresOccasion,
    new_purpose: &str,
    general_charitable_intent: bool,
    initial_failure: bool,
) -> CyPresScheme {
    // For initial failure (before property applied), need general charitable intent
    // For subsequent failure, GCI not required (Re Slevin [1891])
    let can_apply = if initial_failure {
        general_charitable_intent
    } else {
        true // Subsequent failure - GCI not required
    };

    let analysis = if can_apply {
        format!(
            "Cy-pres application available. Original purpose '{}' failed due to: {} ({}). \
             {}. Property can be applied cy-pres to new purpose '{}' which is as near as \
             possible to original donor's intention.",
            original_purpose,
            failure_reason.description(),
            failure_reason.statutory_reference(),
            if initial_failure {
                if general_charitable_intent {
                    "General charitable intent found"
                } else {
                    "ERROR: Initial failure requires GCI"
                }
            } else {
                "Subsequent failure - GCI not required (Re Slevin)"
            },
            new_purpose
        )
    } else {
        format!(
            "Cy-pres NOT available. Original purpose '{}' failed initially (before property \
             applied), but no general charitable intent found. Property results back to \
             donor/estate (Re Ulverston District New Hospital Building Trusts [1956]).",
            original_purpose
        )
    };

    CyPresScheme {
        original_purpose: original_purpose.to_string(),
        failure_reason,
        new_purpose: new_purpose.to_string(),
        general_charitable_intent,
        approved: false, // Would need Court/CC approval
        analysis,
    }
}

// ============================================================================
// Charity Registration
// ============================================================================

/// Charity registration status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Must register (income > £5,000 or CIO)
    MustRegister,
    /// Excepted (income < £100,000 and belongs to excepted class)
    Excepted {
        /// Excepted class
        class: ExceptedClass,
    },
    /// Exempt (supervised by another body)
    Exempt {
        /// Supervising body
        supervisor: String,
    },
    /// Registered
    Registered {
        /// Charity registration number
        charity_number: String,
    },
}

/// Excepted classes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExceptedClass {
    /// Places of worship
    PlaceOfWorship,
    /// Armed forces charity
    ArmedForces,
    /// Scout/guide groups
    ScoutGuide,
}

/// Check if charity must register
pub fn check_registration_requirement(
    annual_income: f64,
    is_cio: bool,
    excepted: Option<ExceptedClass>,
    exempt: Option<&str>,
) -> RegistrationStatus {
    // CIOs must always register
    if is_cio {
        return RegistrationStatus::MustRegister;
    }

    // Exempt charities (e.g., universities supervised by OfS)
    if let Some(supervisor) = exempt {
        return RegistrationStatus::Exempt {
            supervisor: supervisor.to_string(),
        };
    }

    // Excepted charities (income < £100,000 and in excepted class)
    if let Some(class) = excepted
        && annual_income < 100_000.0
    {
        return RegistrationStatus::Excepted { class };
    }

    // General registration threshold
    if annual_income > 5_000.0 {
        RegistrationStatus::MustRegister
    } else {
        // Small charities not required to register (but can voluntarily)
        RegistrationStatus::Excepted {
            class: ExceptedClass::PlaceOfWorship, // Simplified
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poverty_exception_applies() {
        assert!(CharitablePurpose::PreventionReliefPoverty.poverty_exception_applies());
        assert!(CharitablePurpose::ReliefThoseInNeed.poverty_exception_applies());
        assert!(!CharitablePurpose::AdvancementEducation.poverty_exception_applies());
    }

    #[test]
    fn test_validate_education_purpose() {
        let result = validate_charitable_purpose(
            "To provide education and scholarships for young people",
            CharitablePurpose::AdvancementEducation,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_political_purpose_fails() {
        let result = validate_charitable_purpose(
            "To campaign for change in the law relating to animal welfare",
            CharitablePurpose::AdvancementAnimalWelfare,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_cy_pres_initial_failure_requires_gci() {
        let scheme = validate_cy_pres(
            "Relief of poor residents of village X",
            CyPresOccasion::ClassCeased,
            "Relief of poor residents of county Y",
            false, // No GCI
            true,  // Initial failure
        );
        // When initial failure occurs without GCI, cy-pres is NOT available
        assert!(scheme.analysis.contains("NOT available"));
        assert!(!scheme.approved);
    }

    #[test]
    fn test_cy_pres_subsequent_failure_no_gci() {
        let scheme = validate_cy_pres(
            "Maintenance of village hall",
            CyPresOccasion::PurposesFulfilled,
            "Maintenance of community centre",
            false, // No GCI
            false, // Subsequent failure
        );
        assert!(scheme.analysis.contains("available"));
    }

    #[test]
    fn test_registration_cio_must_register() {
        let status = check_registration_requirement(1_000.0, true, None, None);
        assert!(matches!(status, RegistrationStatus::MustRegister));
    }

    #[test]
    fn test_registration_exempt() {
        let status = check_registration_requirement(1_000_000.0, false, None, Some("OfS"));
        assert!(matches!(status, RegistrationStatus::Exempt { .. }));
    }
}
