//! UK Family Law - Domestic Abuse and Protection Orders
//!
//! Implementation of protection from domestic abuse under:
//! - Family Law Act 1996 Part IV (Non-molestation and Occupation orders)
//! - Domestic Abuse Act 2021 (definition of domestic abuse)
//! - Serious Crime Act 2015 s.76 (coercive control)
//! - Protection from Harassment Act 1997
//!
//! # Key Legislation
//!
//! ## Domestic Abuse Act 2021
//!
//! Section 1 defines domestic abuse as behaviour of person A towards person B where:
//! - A and B are personally connected, and
//! - Behaviour is abusive
//!
//! "Abusive" behaviour includes:
//! - Physical or sexual abuse
//! - Violent or threatening behaviour
//! - Controlling or coercive behaviour
//! - Economic abuse
//! - Psychological, emotional or other abuse
//!
//! ## Family Law Act 1996 Part IV
//!
//! ### Non-molestation Orders (s.42)
//!
//! Court may make order prohibiting respondent from:
//! - Molesting associated person
//! - Molesting relevant child
//!
//! ### Occupation Orders (ss.33-41)
//!
//! Different provisions apply depending on:
//! - Whether applicant is entitled to occupy (has property rights)
//! - Nature of relationship (spouse, cohabitant, former)
//!
//! ### Balance of Harm Test (s.33(7))
//!
//! If applicant or child would suffer significant harm if order not made,
//! court MUST make order unless respondent/child would suffer greater harm.

use serde::{Deserialize, Serialize};

use super::error::{FamilyLawError, Result};
use super::types::{
    AbuseType, AssociatedPersonRelationship, BalanceOfHarmOutcome, BalanceOfHarmTest, HarmFactor,
    HarmSeverity, HarmType, OccupationOrderCategory, OccupationOrderProvision,
};

// ============================================================================
// Associated Persons
// ============================================================================

/// Analysis of whether parties are associated persons (FLA 1996 s.62)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssociatedPersonAnalysis {
    /// Applicant
    pub applicant: String,
    /// Respondent
    pub respondent: String,
    /// Claimed relationship
    pub claimed_relationship: AssociatedPersonRelationship,
    /// Is relationship established?
    pub relationship_established: bool,
    /// Are parties associated persons?
    pub are_associated: bool,
    /// Statutory basis
    pub statutory_basis: String,
    /// Analysis
    pub analysis: String,
}

impl AssociatedPersonAnalysis {
    /// Analyze whether parties are associated persons
    pub fn analyze(
        applicant: &str,
        respondent: &str,
        relationship: AssociatedPersonRelationship,
        evidence_supports: bool,
    ) -> Self {
        let statutory_basis = match relationship {
            AssociatedPersonRelationship::SpouseOrCivilPartner => {
                "FLA 1996 s.62(3)(a) - married or civil partners".to_string()
            }
            AssociatedPersonRelationship::FormerSpouseOrCivilPartner => {
                "FLA 1996 s.62(3)(a) - former spouse or civil partner".to_string()
            }
            AssociatedPersonRelationship::Cohabitant => {
                "FLA 1996 s.62(3)(b) - cohabitants".to_string()
            }
            AssociatedPersonRelationship::FormerCohabitant => {
                "FLA 1996 s.62(3)(b) - former cohabitants".to_string()
            }
            AssociatedPersonRelationship::SameHousehold => {
                "FLA 1996 s.62(3)(c) - live/lived in same household (not as tenant/lodger)"
                    .to_string()
            }
            AssociatedPersonRelationship::Relative => "FLA 1996 s.62(3)(d) - relatives".to_string(),
            AssociatedPersonRelationship::EngagedOrAgreed => {
                "FLA 1996 s.62(3)(e) - engaged or agreed to form civil partnership".to_string()
            }
            AssociatedPersonRelationship::IntimateRelationship => {
                "FLA 1996 s.62(3)(ea) - intimate personal relationship of significant duration"
                    .to_string()
            }
            AssociatedPersonRelationship::ParentsOfChild => {
                "FLA 1996 s.62(3)(f) - parents of same child".to_string()
            }
            AssociatedPersonRelationship::FamilyProceedings => {
                "FLA 1996 s.62(3)(g) - parties to same family proceedings".to_string()
            }
        };

        let are_associated = evidence_supports;

        let analysis = if are_associated {
            format!(
                "{} and {} ARE associated persons under {}. \
                 Applicant may apply for non-molestation and/or occupation order.",
                applicant, respondent, statutory_basis
            )
        } else {
            format!(
                "Evidence does NOT establish that {} and {} are associated persons. \
                 Claimed basis: {}. Consider Protection from Harassment Act 1997 instead.",
                applicant, respondent, statutory_basis
            )
        };

        Self {
            applicant: applicant.to_string(),
            respondent: respondent.to_string(),
            claimed_relationship: relationship,
            relationship_established: evidence_supports,
            are_associated,
            statutory_basis,
            analysis,
        }
    }
}

// ============================================================================
// Domestic Abuse Analysis
// ============================================================================

/// Analysis of domestic abuse (DAA 2021 s.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomesticAbuseAnalysis {
    /// Victim
    pub victim: String,
    /// Perpetrator
    pub perpetrator: String,
    /// Types of abuse alleged
    pub abuse_types: Vec<AbuseTypeAssessment>,
    /// Pattern of behaviour identified?
    pub pattern_identified: bool,
    /// Coercive control identified?
    pub coercive_control: bool,
    /// Children affected?
    pub children_affected: bool,
    /// Meets DAA 2021 definition?
    pub meets_daa_definition: bool,
    /// Evidence assessment
    pub evidence_assessment: EvidenceAssessment,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Analysis
    pub analysis: String,
}

/// Assessment of specific abuse type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbuseTypeAssessment {
    /// Type of abuse
    pub abuse_type: AbuseType,
    /// Incidents reported
    pub incidents: Vec<String>,
    /// Evidence available
    pub evidence: Vec<String>,
    /// Severity
    pub severity: AbuseSeverity,
}

/// Severity of abuse
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbuseSeverity {
    /// Severe
    Severe,
    /// Serious
    Serious,
    /// Moderate
    Moderate,
    /// Lower level
    LowerLevel,
}

/// Evidence assessment for domestic abuse
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceAssessment {
    /// Medical evidence available
    pub medical_evidence: bool,
    /// Police reports/crime reference
    pub police_involvement: bool,
    /// Witness statements
    pub witness_statements: bool,
    /// Contemporaneous records
    pub contemporaneous_records: bool,
    /// Photographs
    pub photographs: bool,
    /// Previous court findings
    pub previous_findings: bool,
    /// Overall strength of evidence
    pub evidence_strength: EvidenceStrength,
}

/// Strength of evidence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStrength {
    /// Strong
    Strong,
    /// Moderate
    Moderate,
    /// Limited
    Limited,
    /// Weak
    Weak,
}

/// Risk level assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// High risk
    High,
    /// Medium risk
    Medium,
    /// Standard risk
    Standard,
}

impl DomesticAbuseAnalysis {
    /// Create domestic abuse analysis
    #[allow(clippy::too_many_arguments)]
    pub fn analyze(
        victim: &str,
        perpetrator: &str,
        abuse_types: Vec<AbuseTypeAssessment>,
        pattern_identified: bool,
        coercive_control: bool,
        children_affected: bool,
        evidence_assessment: EvidenceAssessment,
    ) -> Self {
        // Determine if meets DAA 2021 definition
        let meets_daa_definition = !abuse_types.is_empty();

        // Determine risk level
        let risk_level = if coercive_control
            || abuse_types
                .iter()
                .any(|a| matches!(a.severity, AbuseSeverity::Severe))
        {
            RiskLevel::High
        } else if pattern_identified
            || abuse_types
                .iter()
                .any(|a| matches!(a.severity, AbuseSeverity::Serious))
        {
            RiskLevel::Medium
        } else {
            RiskLevel::Standard
        };

        let analysis = format!(
            "Domestic abuse analysis per DAA 2021 s.1:\n\
             Victim: {}\n\
             Perpetrator: {}\n\
             Abuse types identified: {}\n\
             Pattern of behaviour: {}\n\
             Coercive control (SCA 2015 s.76): {}\n\
             Children affected (DAA 2021 s.3): {}\n\
             Evidence strength: {:?}\n\
             Risk level: {:?}\n\
             Meets DAA 2021 definition: {}",
            victim,
            perpetrator,
            abuse_types.len(),
            if pattern_identified { "Yes" } else { "No" },
            if coercive_control { "Yes" } else { "No" },
            if children_affected {
                "Yes - children are victims under DAA s.3"
            } else {
                "No"
            },
            evidence_assessment.evidence_strength,
            risk_level,
            if meets_daa_definition { "YES" } else { "NO" }
        );

        Self {
            victim: victim.to_string(),
            perpetrator: perpetrator.to_string(),
            abuse_types,
            pattern_identified,
            coercive_control,
            children_affected,
            meets_daa_definition,
            evidence_assessment,
            risk_level,
            analysis,
        }
    }
}

// ============================================================================
// Non-Molestation Order Analysis
// ============================================================================

/// Analysis for non-molestation order application (FLA 1996 s.42)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonMolestationOrderAnalysis {
    /// Applicant
    pub applicant: String,
    /// Respondent
    pub respondent: String,
    /// Are parties associated persons?
    pub associated_persons: bool,
    /// Molestation alleged
    pub molestation_alleged: Vec<String>,
    /// Evidence of molestation
    pub evidence_available: bool,
    /// Need for order
    pub need_for_order: String,
    /// Should order be made?
    pub should_make_order: bool,
    /// Without notice justified?
    pub without_notice_justified: bool,
    /// Without notice reasons
    pub without_notice_reasons: Vec<String>,
    /// Recommended terms
    pub recommended_terms: Vec<String>,
    /// Duration recommendation
    pub duration: String,
    /// Analysis
    pub analysis: String,
}

impl NonMolestationOrderAnalysis {
    /// Analyze non-molestation order application
    #[allow(clippy::too_many_arguments)]
    pub fn analyze(
        applicant: &str,
        respondent: &str,
        associated_persons: bool,
        molestation_alleged: Vec<String>,
        evidence_available: bool,
        risk_of_harm: bool,
        immediate_risk: bool,
        respondent_would_evade: bool,
    ) -> Self {
        let should_make_order = associated_persons && evidence_available;

        // Without notice analysis (FPR 2010 r.10.2)
        let mut without_notice_reasons = Vec::new();
        if immediate_risk {
            without_notice_reasons
                .push("Risk of significant harm if respondent given notice".to_string());
        }
        if respondent_would_evade {
            without_notice_reasons.push(
                "Respondent likely to evade service or take steps to defeat purpose of order"
                    .to_string(),
            );
        }
        let without_notice_justified = !without_notice_reasons.is_empty();

        // Recommended terms
        let recommended_terms = vec![
            format!("Not to use or threaten violence against {}", applicant),
            format!("Not to intimidate, harass or pester {}", applicant),
            format!(
                "Not to contact {} by any means (unless through solicitors)",
                applicant
            ),
            format!(
                "Not to come within [100] metres of {}'s home address",
                applicant
            ),
        ];

        let duration = if risk_of_harm {
            "Until further order (breach is criminal offence - FLA 1996 s.42A)".to_string()
        } else {
            "12 months".to_string()
        };

        let need_for_order = if risk_of_harm {
            "Protection needed - risk of significant harm identified".to_string()
        } else {
            "Protection may be needed - pattern of molestation alleged".to_string()
        };

        let analysis = if should_make_order {
            format!(
                "Non-molestation order SHOULD be made under FLA 1996 s.42.\n\
                 Associated persons: Yes\n\
                 Evidence of molestation: Available\n\
                 Without notice: {}\n\
                 Duration: {}\n\
                 Note: Breach is criminal offence (s.42A) - max 5 years imprisonment.",
                if without_notice_justified {
                    "Justified"
                } else {
                    "Not justified"
                },
                duration
            )
        } else if !associated_persons {
            "Non-molestation order NOT available - parties not associated persons. \
             Consider Protection from Harassment Act 1997 instead."
                .to_string()
        } else {
            "Non-molestation order may not be appropriate - insufficient evidence of molestation."
                .to_string()
        };

        Self {
            applicant: applicant.to_string(),
            respondent: respondent.to_string(),
            associated_persons,
            molestation_alleged,
            evidence_available,
            need_for_order,
            should_make_order,
            without_notice_justified,
            without_notice_reasons,
            recommended_terms,
            duration,
            analysis,
        }
    }
}

// ============================================================================
// Occupation Order Analysis
// ============================================================================

/// Analysis for occupation order application (FLA 1996 ss.33-38)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OccupationOrderAnalysis {
    /// Applicant
    pub applicant: String,
    /// Respondent
    pub respondent: String,
    /// Property address
    pub property: String,
    /// Applicant category
    pub category: OccupationOrderCategory,
    /// Is applicant entitled to occupy?
    pub applicant_entitled: bool,
    /// Balance of harm test result
    pub balance_of_harm: BalanceOfHarmTest,
    /// Section 33(6) factors (if discretionary)
    pub section_33_6_factors: Option<Section33_6Factors>,
    /// Should order be made?
    pub should_make_order: bool,
    /// Recommended provisions
    pub recommended_provisions: Vec<OccupationOrderProvision>,
    /// Maximum duration
    pub max_duration: String,
    /// Analysis
    pub analysis: String,
}

/// Section 33(6) factors for discretionary cases
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section33_6Factors {
    /// Housing needs and resources of parties and children
    pub housing_needs: String,
    /// Financial resources of parties
    pub financial_resources: String,
    /// Likely effect of order/no order on health, safety and wellbeing
    pub effect_on_health_safety: String,
    /// Conduct of parties
    pub conduct: String,
}

impl OccupationOrderAnalysis {
    /// Analyze occupation order application
    #[allow(clippy::too_many_arguments)]
    pub fn analyze(
        applicant: &str,
        respondent: &str,
        property: &str,
        applicant_entitled: bool,
        is_spouse_or_civil_partner: bool,
        is_cohabitant: bool,
        balance_of_harm: BalanceOfHarmTest,
        section_33_6_factors: Option<Section33_6Factors>,
    ) -> Self {
        // Determine category
        let category = if applicant_entitled {
            OccupationOrderCategory::Section33Entitled
        } else if is_spouse_or_civil_partner {
            OccupationOrderCategory::Section35FormerSpouse
        } else if is_cohabitant {
            OccupationOrderCategory::Section36Cohabitant
        } else {
            OccupationOrderCategory::Section38NeitherEntitled
        };

        // Determine if order should be made
        let should_make_order = match balance_of_harm.outcome {
            BalanceOfHarmOutcome::MustMakeOrder => true,
            BalanceOfHarmOutcome::MayMakeOrder => true, // Discretionary
            BalanceOfHarmOutcome::ShouldNotMakeOrder => false,
        };

        // Recommended provisions
        let mut recommended_provisions = vec![OccupationOrderProvision::EnforceEntitlement];
        if !applicant_entitled {
            recommended_provisions.push(OccupationOrderProvision::RequirePermitEntry);
        }
        if balance_of_harm.significant_harm_to_applicant {
            recommended_provisions.push(OccupationOrderProvision::ProhibitOccupation);
            recommended_provisions.push(OccupationOrderProvision::RequireLeave);
        }

        // Max duration depends on category
        let max_duration = match category {
            OccupationOrderCategory::Section33Entitled => "Until further order".to_string(),
            OccupationOrderCategory::Section35FormerSpouse => "Until further order".to_string(),
            OccupationOrderCategory::Section36Cohabitant => {
                "6 months (renewable once for further 6 months)".to_string()
            }
            OccupationOrderCategory::Section37NeitherEntitled => {
                "6 months (renewable once)".to_string()
            }
            OccupationOrderCategory::Section38NeitherEntitled => {
                "6 months (renewable once)".to_string()
            }
        };

        let analysis = format!(
            "Occupation order analysis under FLA 1996:\n\
             Category: {:?}\n\
             Applicant entitled: {}\n\
             Balance of harm: {:?}\n\
             Order {} be made.\n\
             Maximum duration: {}",
            category,
            if applicant_entitled {
                "Yes (s.33)"
            } else {
                "No"
            },
            balance_of_harm.outcome,
            if should_make_order {
                "SHOULD"
            } else {
                "should NOT"
            },
            max_duration
        );

        Self {
            applicant: applicant.to_string(),
            respondent: respondent.to_string(),
            property: property.to_string(),
            category,
            applicant_entitled,
            balance_of_harm,
            section_33_6_factors,
            should_make_order,
            recommended_provisions,
            max_duration,
            analysis,
        }
    }
}

/// Perform balance of harm test (FLA 1996 s.33(7))
pub fn perform_balance_of_harm_test(
    applicant_harm_factors: Vec<HarmFactor>,
    respondent_harm_factors: Vec<HarmFactor>,
) -> BalanceOfHarmTest {
    // Check for significant harm to applicant/child
    let significant_harm_to_applicant = applicant_harm_factors.iter().any(|h| {
        matches!(h.severity, HarmSeverity::Serious | HarmSeverity::Severe)
            || matches!(h.harm_type, HarmType::ImpairmentOfDevelopment)
    });

    // Compare harm
    let applicant_score: u32 = applicant_harm_factors
        .iter()
        .map(|h| match h.severity {
            HarmSeverity::Severe => 4,
            HarmSeverity::Serious => 3,
            HarmSeverity::Moderate => 2,
            HarmSeverity::Minor => 1,
        })
        .sum();

    let respondent_score: u32 = respondent_harm_factors
        .iter()
        .map(|h| match h.severity {
            HarmSeverity::Severe => 4,
            HarmSeverity::Serious => 3,
            HarmSeverity::Moderate => 2,
            HarmSeverity::Minor => 1,
        })
        .sum();

    let balance_favours_applicant = applicant_score > respondent_score;

    let outcome = if significant_harm_to_applicant && balance_favours_applicant {
        BalanceOfHarmOutcome::MustMakeOrder
    } else if significant_harm_to_applicant || applicant_score > 0 {
        // Discretionary - may make order if significant harm or any harm identified
        BalanceOfHarmOutcome::MayMakeOrder
    } else {
        BalanceOfHarmOutcome::ShouldNotMakeOrder
    };

    BalanceOfHarmTest {
        harm_if_not_made: applicant_harm_factors,
        harm_if_made: respondent_harm_factors,
        significant_harm_to_applicant,
        balance_favours_applicant,
        outcome,
    }
}

// ============================================================================
// Undertakings
// ============================================================================

/// Analysis of whether undertaking is appropriate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndertakingAnalysis {
    /// Is undertaking appropriate?
    pub appropriate: bool,
    /// Violence alleged?
    pub violence_alleged: bool,
    /// Threats of violence?
    pub threats_of_violence: bool,
    /// Power of arrest would otherwise be attached?
    pub power_of_arrest_needed: bool,
    /// Respondent likely to comply?
    pub likely_to_comply: bool,
    /// Reasons undertaking not appropriate
    pub inappropriate_reasons: Vec<String>,
    /// Analysis
    pub analysis: String,
}

impl UndertakingAnalysis {
    /// Analyze whether undertaking is appropriate
    pub fn analyze(
        violence_alleged: bool,
        threats_of_violence: bool,
        likely_to_comply: bool,
    ) -> Self {
        let mut inappropriate_reasons = Vec::new();

        // FLA 1996 s.46(3A) - undertaking not appropriate if violence/threats
        let power_of_arrest_needed = violence_alleged || threats_of_violence;

        if violence_alleged {
            inappropriate_reasons.push(
                "FLA 1996 s.46(3A) - undertaking not to be accepted where violence alleged"
                    .to_string(),
            );
        }
        if threats_of_violence {
            inappropriate_reasons.push(
                "FLA 1996 s.46(3A) - undertaking not to be accepted where threats of violence"
                    .to_string(),
            );
        }
        if !likely_to_comply {
            inappropriate_reasons
                .push("Respondent unlikely to comply with undertaking".to_string());
        }

        let appropriate = inappropriate_reasons.is_empty();

        let analysis = if appropriate {
            "Undertaking MAY be appropriate. No violence or threats alleged, \
             respondent likely to comply. Court may accept undertaking instead of order."
                .to_string()
        } else {
            format!(
                "Undertaking NOT appropriate under FLA 1996 s.46(3A):\n{}",
                inappropriate_reasons.join("\n")
            )
        };

        Self {
            appropriate,
            violence_alleged,
            threats_of_violence,
            power_of_arrest_needed,
            likely_to_comply,
            inappropriate_reasons,
            analysis,
        }
    }
}

// ============================================================================
// Forced Marriage and FGM Protection
// ============================================================================

/// Forced marriage protection order analysis (FLA 1996 Part 4A)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForcedMarriageAnalysis {
    /// Person to be protected
    pub person_to_protect: String,
    /// Is person being forced into marriage?
    pub being_forced: bool,
    /// Has person been forced into marriage?
    pub already_forced: bool,
    /// Persons involved in forcing
    pub persons_forcing: Vec<String>,
    /// Overseas element?
    pub overseas_element: bool,
    /// Order should be made?
    pub should_make_order: bool,
    /// Recommended terms
    pub recommended_terms: Vec<String>,
    /// Analysis
    pub analysis: String,
}

impl ForcedMarriageAnalysis {
    /// Analyze forced marriage protection order application
    pub fn analyze(
        person_to_protect: &str,
        being_forced: bool,
        already_forced: bool,
        persons_forcing: Vec<String>,
        overseas_element: bool,
    ) -> Self {
        let should_make_order = being_forced || already_forced;

        let mut recommended_terms = Vec::new();
        if being_forced {
            recommended_terms.push(format!(
                "Prohibit {} from taking steps to arrange marriage of {}",
                persons_forcing.join(", "),
                person_to_protect
            ));
            recommended_terms.push(format!(
                "Prohibit {} from using violence/threats against {}",
                persons_forcing.join(", "),
                person_to_protect
            ));
        }
        if overseas_element {
            recommended_terms.push(format!(
                "Prohibit removal of {} from England and Wales",
                person_to_protect
            ));
            recommended_terms.push(format!(
                "Require surrender of {}'s passport",
                person_to_protect
            ));
        }

        let analysis = format!(
            "Forced marriage protection order analysis (FLA 1996 Part 4A):\n\
             Person to protect: {}\n\
             Being forced: {}\n\
             Already forced: {}\n\
             Overseas element: {}\n\
             Order {} be made.\n\
             Note: Breach is criminal offence - Anti-social Behaviour, Crime and Policing Act 2014.",
            person_to_protect,
            if being_forced { "Yes" } else { "No" },
            if already_forced { "Yes" } else { "No" },
            if overseas_element { "Yes" } else { "No" },
            if should_make_order {
                "SHOULD"
            } else {
                "should NOT"
            }
        );

        Self {
            person_to_protect: person_to_protect.to_string(),
            being_forced,
            already_forced,
            persons_forcing,
            overseas_element,
            should_make_order,
            recommended_terms,
            analysis,
        }
    }
}

/// FGM protection order analysis (FGM Act 2003 Sch 2)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FGMProtectionAnalysis {
    /// Girl to be protected
    pub girl_to_protect: String,
    /// At risk of FGM?
    pub at_risk: bool,
    /// FGM already performed?
    pub already_performed: bool,
    /// Risk factors identified
    pub risk_factors: Vec<String>,
    /// Overseas element?
    pub overseas_element: bool,
    /// Order should be made?
    pub should_make_order: bool,
    /// Recommended terms
    pub recommended_terms: Vec<String>,
    /// Analysis
    pub analysis: String,
}

impl FGMProtectionAnalysis {
    /// Analyze FGM protection order application
    pub fn analyze(
        girl_to_protect: &str,
        at_risk: bool,
        already_performed: bool,
        risk_factors: Vec<String>,
        overseas_element: bool,
    ) -> Self {
        let should_make_order = at_risk;

        let mut recommended_terms = Vec::new();
        if at_risk {
            recommended_terms.push(format!(
                "Prohibit taking {} to undergo FGM",
                girl_to_protect
            ));
            recommended_terms.push(format!("Prohibit arranging FGM for {}", girl_to_protect));
        }
        if overseas_element {
            recommended_terms.push(format!(
                "Prohibit removal of {} from England and Wales",
                girl_to_protect
            ));
            recommended_terms.push(format!(
                "Require surrender of {}'s passport",
                girl_to_protect
            ));
        }

        let analysis = format!(
            "FGM protection order analysis (FGM Act 2003 Schedule 2):\n\
             Girl to protect: {}\n\
             At risk: {}\n\
             Risk factors: {}\n\
             Overseas element: {}\n\
             Order {} be made.\n\
             Note: Breach is criminal offence - max 5 years imprisonment.",
            girl_to_protect,
            if at_risk { "Yes" } else { "No" },
            risk_factors.join(", "),
            if overseas_element { "Yes" } else { "No" },
            if should_make_order {
                "SHOULD"
            } else {
                "should NOT"
            }
        );

        Self {
            girl_to_protect: girl_to_protect.to_string(),
            at_risk,
            already_performed,
            risk_factors,
            overseas_element,
            should_make_order,
            recommended_terms,
            analysis,
        }
    }
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate parties are associated persons
pub fn validate_associated_persons(
    claimed_relationship: &AssociatedPersonRelationship,
    evidence_supports: bool,
) -> Result<()> {
    if !evidence_supports {
        return Err(FamilyLawError::NotAssociatedPerson {
            relationship: format!("{:?}", claimed_relationship),
        });
    }
    Ok(())
}

/// Validate undertaking is appropriate
pub fn validate_undertaking_appropriate(
    violence_alleged: bool,
    threats_of_violence: bool,
) -> Result<()> {
    if violence_alleged || threats_of_violence {
        return Err(FamilyLawError::UndertakingInappropriate {
            reason: "Violence or threats of violence alleged - FLA 1996 s.46(3A)".to_string(),
        });
    }
    Ok(())
}

/// Validate without notice application
pub fn validate_without_notice(immediate_risk: bool, respondent_would_evade: bool) -> Result<()> {
    if !immediate_risk && !respondent_would_evade {
        return Err(FamilyLawError::WithoutNoticeNotJustified {
            reason: "No immediate risk and respondent would not evade service".to_string(),
        });
    }
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associated_person_spouse() {
        let analysis = AssociatedPersonAnalysis::analyze(
            "Wife",
            "Husband",
            AssociatedPersonRelationship::SpouseOrCivilPartner,
            true,
        );

        assert!(analysis.are_associated);
        assert!(analysis.statutory_basis.contains("s.62(3)(a)"));
    }

    #[test]
    fn test_associated_person_intimate_relationship() {
        let analysis = AssociatedPersonAnalysis::analyze(
            "Applicant",
            "Respondent",
            AssociatedPersonRelationship::IntimateRelationship,
            true,
        );

        assert!(analysis.are_associated);
        assert!(analysis.statutory_basis.contains("s.62(3)(ea)"));
    }

    #[test]
    fn test_non_molestation_order_should_be_made() {
        let analysis = NonMolestationOrderAnalysis::analyze(
            "Applicant",
            "Respondent",
            true,
            vec!["Harassment".to_string(), "Threats".to_string()],
            true,
            true,
            true,
            false,
        );

        assert!(analysis.should_make_order);
        assert!(analysis.without_notice_justified);
    }

    #[test]
    fn test_non_molestation_not_associated() {
        let analysis = NonMolestationOrderAnalysis::analyze(
            "Applicant",
            "Respondent",
            false, // Not associated
            vec!["Harassment".to_string()],
            true,
            true,
            false,
            false,
        );

        assert!(!analysis.should_make_order);
        assert!(analysis.analysis.contains("NOT available"));
    }

    #[test]
    fn test_balance_of_harm_must_make_order() {
        let applicant_harm = vec![HarmFactor {
            person: "Applicant".to_string(),
            harm_type: HarmType::Physical,
            description: "Risk of violence".to_string(),
            severity: HarmSeverity::Severe,
        }];

        let respondent_harm = vec![HarmFactor {
            person: "Respondent".to_string(),
            harm_type: HarmType::Financial,
            description: "Inconvenience of finding new accommodation".to_string(),
            severity: HarmSeverity::Minor,
        }];

        let result = perform_balance_of_harm_test(applicant_harm, respondent_harm);

        assert!(result.significant_harm_to_applicant);
        assert!(result.balance_favours_applicant);
        assert_eq!(result.outcome, BalanceOfHarmOutcome::MustMakeOrder);
    }

    #[test]
    fn test_balance_of_harm_may_make_order() {
        let applicant_harm = vec![HarmFactor {
            person: "Applicant".to_string(),
            harm_type: HarmType::Emotional,
            description: "Distress".to_string(),
            severity: HarmSeverity::Moderate,
        }];

        let respondent_harm = vec![];

        let result = perform_balance_of_harm_test(applicant_harm, respondent_harm);

        assert!(!result.significant_harm_to_applicant);
        assert_eq!(result.outcome, BalanceOfHarmOutcome::MayMakeOrder);
    }

    #[test]
    fn test_undertaking_not_appropriate_violence() {
        let analysis = UndertakingAnalysis::analyze(true, false, true);

        assert!(!analysis.appropriate);
        assert!(analysis.power_of_arrest_needed);
        assert!(!analysis.inappropriate_reasons.is_empty());
    }

    #[test]
    fn test_undertaking_appropriate() {
        let analysis = UndertakingAnalysis::analyze(false, false, true);

        assert!(analysis.appropriate);
        assert!(!analysis.power_of_arrest_needed);
        assert!(analysis.inappropriate_reasons.is_empty());
    }

    #[test]
    fn test_forced_marriage_analysis() {
        let analysis = ForcedMarriageAnalysis::analyze(
            "Victim",
            true,
            false,
            vec!["Father".to_string(), "Uncle".to_string()],
            true,
        );

        assert!(analysis.should_make_order);
        assert!(analysis.overseas_element);
        assert!(!analysis.recommended_terms.is_empty());
    }

    #[test]
    fn test_fgm_protection_analysis() {
        let analysis = FGMProtectionAnalysis::analyze(
            "Child",
            true,
            false,
            vec!["Family history".to_string(), "Planned trip".to_string()],
            true,
        );

        assert!(analysis.should_make_order);
        assert!(analysis.overseas_element);
        assert!(!analysis.recommended_terms.is_empty());
    }

    #[test]
    fn test_validate_associated_persons_ok() {
        let result =
            validate_associated_persons(&AssociatedPersonRelationship::SpouseOrCivilPartner, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_associated_persons_fail() {
        let result =
            validate_associated_persons(&AssociatedPersonRelationship::IntimateRelationship, false);
        assert!(matches!(
            result,
            Err(FamilyLawError::NotAssociatedPerson { .. })
        ));
    }

    #[test]
    fn test_validate_undertaking_violence() {
        let result = validate_undertaking_appropriate(true, false);
        assert!(matches!(
            result,
            Err(FamilyLawError::UndertakingInappropriate { .. })
        ));
    }
}
