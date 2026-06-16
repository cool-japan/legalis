//! Federal Commerce Power Analysis (the *affirmative* Commerce Clause)
//!
//! This module models Congress's affirmative power to regulate under the
//! Commerce Clause (U.S. Const. art. I, § 8, cl. 3), as distinct from the
//! *Dormant* Commerce Clause limits on the states modeled in
//! [`super::commerce_clause`].
//!
//! ## The Three Categories (United States v. Lopez, 514 U.S. 549 (1995))
//!
//! In *Lopez*, the Court restated that Congress may regulate three categories of
//! activity under the commerce power:
//!
//! 1. **Channels of interstate commerce** — the highways, waterways, air traffic,
//!    and instrumentalities-as-conduits through which commerce moves (e.g.,
//!    *Heart of Atlanta Motel v. United States*, 379 U.S. 241 (1964), upholding the
//!    public-accommodations title of the Civil Rights Act of 1964).
//!
//! 2. **Instrumentalities of interstate commerce, and persons or things in
//!    interstate commerce** — even where the threat comes from intrastate
//!    activities (e.g., *Shreveport Rate Cases*, 234 U.S. 342 (1914); regulation
//!    of trains, trucks, and the like).
//!
//! 3. **Activities that substantially affect interstate commerce** — the broadest
//!    and most contested category.
//!
//! ## The Substantial-Effects Category and Aggregation
//!
//! Under *Wickard v. Filburn*, 317 U.S. 111 (1942), Congress may reach purely
//! local, even non-commercial, activity if that activity, **taken in the
//! aggregate**, exerts a substantial economic effect on interstate commerce.
//! Filburn's home-grown wheat, consumed on his own farm, could be regulated
//! because the cumulative effect of many such farmers substantially affected the
//! interstate wheat market. *Gonzales v. Raich*, 545 U.S. 1 (2005), reaffirmed
//! aggregation for the intrastate cultivation of marijuana as part of a
//! comprehensive regulatory scheme (the Controlled Substances Act).
//!
//! ## The Lopez / Morrison Limits
//!
//! *United States v. Lopez* (1995) struck down the Gun-Free School Zones Act and
//! *United States v. Morrison*, 529 U.S. 598 (2000), struck down the civil-remedy
//! provision of the Violence Against Women Act. Together they establish that, in
//! the substantial-effects category:
//!
//! - the regulated activity must be **economic** in nature for aggregation to
//!   apply (possessing a gun near a school, and gender-motivated violence, are
//!   not economic);
//! - a **jurisdictional element** (a case-by-case requirement that the regulated
//!   item or act has a connection to interstate commerce) can save a statute that
//!   would otherwise be vulnerable;
//! - **congressional findings** are relevant but not dispositive; they cannot
//!   substitute for a substantial connection where the activity is non-economic;
//! - the Court will not "pile inference upon inference" — an **attenuated causal
//!   chain** from the activity to a substantial effect on commerce is
//!   insufficient (this would obliterate the distinction between national and
//!   truly local concerns).

use serde::{Deserialize, Serialize};

/// The category of the commerce power under which a regulation is defended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommerceCategory {
    /// Category 1 — regulation of the channels of interstate commerce.
    Channels,
    /// Category 2 — regulation/protection of the instrumentalities of interstate
    /// commerce, or persons or things in interstate commerce.
    Instrumentalities,
    /// Category 3 — regulation of activities that substantially affect interstate
    /// commerce (the *Wickard* aggregation category).
    SubstantialEffects,
}

impl CommerceCategory {
    /// Human-readable description with the leading landmark.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::Channels => {
                "Channels of interstate commerce (e.g., Heart of Atlanta Motel v. United States)"
            }
            Self::Instrumentalities => {
                "Instrumentalities, persons, or things in interstate commerce (e.g., Shreveport \
                 Rate Cases)"
            }
            Self::SubstantialEffects => {
                "Activities substantially affecting interstate commerce (Wickard v. Filburn \
                 aggregation)"
            }
        }
    }

    /// The leading case ordinarily cited for the category.
    #[must_use]
    pub fn leading_case(&self) -> &'static str {
        match self {
            Self::Channels => "Heart of Atlanta Motel v. United States, 379 U.S. 241 (1964)",
            Self::Instrumentalities => "Shreveport Rate Cases, 234 U.S. 342 (1914)",
            Self::SubstantialEffects => "Wickard v. Filburn, 317 U.S. 111 (1942)",
        }
    }
}

/// Whether the regulated activity is economic or commercial in character. This
/// is the pivotal distinction drawn by *Lopez* and *Morrison* for the
/// substantial-effects category: only economic activity may be aggregated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityCharacter {
    /// The activity is economic / commercial (production, consumption,
    /// distribution of a commodity or service).
    Economic,
    /// The activity is non-economic (e.g., gun possession near a school,
    /// gender-motivated violence).
    NonEconomic,
}

impl ActivityCharacter {
    /// Whether the activity is economic.
    #[must_use]
    pub fn is_economic(&self) -> bool {
        matches!(self, Self::Economic)
    }
}

/// A regulated activity to be analyzed against the federal commerce power.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercePowerAnalysis {
    /// Short name of the federal statute or regulation under review.
    pub statute: String,
    /// Description of the regulated activity.
    pub activity: String,
    /// The category being asserted as the source of power.
    pub category: CommerceCategory,
    /// Whether the activity is economic (only relevant to substantial-effects).
    pub character: ActivityCharacter,
    /// Whether the statute contains a case-by-case jurisdictional element
    /// requiring a connection to interstate commerce.
    pub jurisdictional_element: bool,
    /// Whether Congress made formal findings of an effect on interstate commerce.
    pub congressional_findings: bool,
    /// Whether the activity is part of a broader comprehensive regulatory scheme
    /// of interstate economic activity (Raich / Wickard).
    pub part_of_regulatory_scheme: bool,
    /// Whether the asserted link from the activity to a substantial effect on
    /// interstate commerce requires "piling inference upon inference"
    /// (an attenuated causal chain, condemned in Lopez/Morrison).
    pub attenuated_causal_chain: bool,
}

impl CommercePowerAnalysis {
    /// Create a new commerce-power analysis. Defaults to the substantial-effects
    /// category with economic activity, no jurisdictional element, no findings,
    /// not part of a scheme, and a direct (non-attenuated) causal chain.
    #[must_use]
    pub fn new(statute: impl Into<String>, activity: impl Into<String>) -> Self {
        Self {
            statute: statute.into(),
            activity: activity.into(),
            category: CommerceCategory::SubstantialEffects,
            character: ActivityCharacter::Economic,
            jurisdictional_element: false,
            congressional_findings: false,
            part_of_regulatory_scheme: false,
            attenuated_causal_chain: false,
        }
    }

    /// Set the commerce-power category.
    #[must_use]
    pub fn with_category(mut self, category: CommerceCategory) -> Self {
        self.category = category;
        self
    }

    /// Set the economic/non-economic character of the activity.
    #[must_use]
    pub fn with_character(mut self, character: ActivityCharacter) -> Self {
        self.character = character;
        self
    }

    /// Record that the statute contains a jurisdictional element.
    #[must_use]
    pub fn with_jurisdictional_element(mut self) -> Self {
        self.jurisdictional_element = true;
        self
    }

    /// Record that Congress made formal findings.
    #[must_use]
    pub fn with_congressional_findings(mut self) -> Self {
        self.congressional_findings = true;
        self
    }

    /// Record that the activity is part of a comprehensive regulatory scheme.
    #[must_use]
    pub fn part_of_comprehensive_scheme(mut self) -> Self {
        self.part_of_regulatory_scheme = true;
        self
    }

    /// Record that the causal chain to interstate commerce is attenuated.
    #[must_use]
    pub fn with_attenuated_causal_chain(mut self) -> Self {
        self.attenuated_causal_chain = true;
        self
    }

    /// Analyze the regulation under the federal commerce power.
    #[must_use]
    pub fn analyze(&self) -> CommercePowerResult {
        let mut reasoning = Vec::new();
        reasoning.push(format!(
            "Asserted category: {} — {}",
            category_label(self.category),
            self.category.description()
        ));

        match self.category {
            // Categories 1 and 2 are settled and broad: the power to regulate the
            // channels and instrumentalities of interstate commerce is plenary.
            CommerceCategory::Channels => {
                reasoning.push(
                    "Congress has plenary power over the channels of interstate commerce, and may \
                     keep them free from immoral and injurious uses."
                        .to_string(),
                );
                reasoning.push(format!("See {}.", self.category.leading_case()));
                reasoning.push("Result: regulation is LIKELY VALID.".to_string());
                CommercePowerResult {
                    category: self.category,
                    likely_valid: true,
                    confidence: 0.90,
                    reasoning,
                }
            }
            CommerceCategory::Instrumentalities => {
                reasoning.push(
                    "Congress may regulate and protect the instrumentalities of interstate \
                     commerce, and persons or things in interstate commerce, even against threats \
                     arising from purely intrastate activity."
                        .to_string(),
                );
                reasoning.push(format!("See {}.", self.category.leading_case()));
                reasoning.push("Result: regulation is LIKELY VALID.".to_string());
                CommercePowerResult {
                    category: self.category,
                    likely_valid: true,
                    confidence: 0.88,
                    reasoning,
                }
            }
            CommerceCategory::SubstantialEffects => self.analyze_substantial_effects(reasoning),
        }
    }

    /// The contested category-3 analysis under Wickard / Lopez / Morrison / Raich.
    fn analyze_substantial_effects(&self, mut reasoning: Vec<String>) -> CommercePowerResult {
        // Economic activity: aggregation under Wickard / Raich is available.
        if self.character.is_economic() {
            reasoning.push(
                "The regulated activity is ECONOMIC in nature; under Wickard v. Filburn its effects \
                 may be considered in the aggregate."
                    .to_string(),
            );
            if self.part_of_regulatory_scheme {
                reasoning.push(
                    "The activity is part of a comprehensive scheme regulating an interstate \
                     economic market; excising the intrastate activity could undercut the scheme \
                     (Gonzales v. Raich, 545 U.S. 1 (2005))."
                        .to_string(),
                );
                return CommercePowerResult {
                    category: self.category,
                    likely_valid: true,
                    confidence: 0.85,
                    reasoning,
                };
            }
            if self.attenuated_causal_chain {
                // Even economic-labeled activity fails if the link is built only
                // by piling inference upon inference.
                reasoning.push(
                    "However, the connection to interstate commerce rests on an attenuated causal \
                     chain; the Court will not pile inference upon inference (Lopez; Morrison)."
                        .to_string(),
                );
                let saved = self.jurisdictional_element;
                if saved {
                    reasoning.push(
                        "A case-by-case jurisdictional element nevertheless confines the statute to \
                         items with an interstate-commerce connection, supporting validity."
                            .to_string(),
                    );
                }
                return CommercePowerResult {
                    category: self.category,
                    likely_valid: saved,
                    confidence: if saved { 0.62 } else { 0.55 },
                    reasoning,
                };
            }
            reasoning.push(
                "A rational basis exists for concluding that the aggregate of the economic activity \
                 substantially affects interstate commerce."
                    .to_string(),
            );
            return CommercePowerResult {
                category: self.category,
                likely_valid: true,
                confidence: 0.80,
                reasoning,
            };
        }

        // Non-economic activity: Lopez / Morrison. Aggregation is unavailable.
        reasoning.push(
            "The regulated activity is NON-ECONOMIC; under United States v. Lopez (1995) and United \
             States v. Morrison (2000) its effects on interstate commerce may NOT be aggregated."
                .to_string(),
        );

        if self.jurisdictional_element {
            reasoning.push(
                "The statute contains a jurisdictional element ensuring, case by case, that the \
                 regulated conduct affects interstate commerce; this is the saving feature absent \
                 in Lopez and Morrison."
                    .to_string(),
            );
            return CommercePowerResult {
                category: self.category,
                likely_valid: true,
                confidence: 0.70,
                reasoning,
            };
        }

        if self.congressional_findings {
            reasoning.push(
                "Congressional findings of an aggregate effect are present but are not dispositive; \
                 they cannot supply a substantial effect where the activity is non-economic and the \
                 inferential chain is attenuated (Morrison)."
                    .to_string(),
            );
        }
        if self.attenuated_causal_chain {
            reasoning.push(
                "The asserted effect depends on an attenuated causal chain (e.g., violence → fear → \
                 reduced travel/commerce), which the Court has rejected as a basis for the commerce \
                 power."
                    .to_string(),
            );
        }

        reasoning.push(
            "Result: regulation is LIKELY INVALID as applied to non-economic intrastate activity."
                .to_string(),
        );
        CommercePowerResult {
            category: self.category,
            likely_valid: false,
            confidence: 0.80,
            reasoning,
        }
    }
}

/// Result of a federal commerce-power analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercePowerResult {
    /// The category under which the regulation was analyzed.
    pub category: CommerceCategory,
    /// Whether the regulation is a likely-valid exercise of the commerce power.
    pub likely_valid: bool,
    /// Confidence in the determination (0.0-1.0).
    pub confidence: f64,
    /// Step-by-step reasoning with case citations.
    pub reasoning: Vec<String>,
}

impl CommercePowerResult {
    /// Generate a markdown summary of the analysis.
    #[must_use]
    pub fn summary(&self) -> String {
        let mut report = String::new();
        report.push_str("# Federal Commerce Power Analysis\n\n");
        report.push_str(&format!(
            "**Determination**: {}\n\n",
            if self.likely_valid {
                "LIKELY VALID exercise of the commerce power"
            } else {
                "LIKELY INVALID exercise of the commerce power"
            }
        ));
        report.push_str(&format!(
            "**Category**: {}\n\n",
            category_label(self.category)
        ));
        report.push_str(&format!(
            "**Confidence**: {:.1}%\n\n",
            self.confidence * 100.0
        ));
        report.push_str("## Reasoning\n\n");
        for step in &self.reasoning {
            report.push_str(&format!("- {step}\n"));
        }
        report
    }
}

/// Short label for a category.
fn category_label(category: CommerceCategory) -> &'static str {
    match category {
        CommerceCategory::Channels => "Channels",
        CommerceCategory::Instrumentalities => "Instrumentalities",
        CommerceCategory::SubstantialEffects => "Substantial Effects",
    }
}

/// Reconstruct *United States v. Lopez* (1995): the Gun-Free School Zones Act of
/// 1990 criminalized possession of a firearm in a school zone. Possession is
/// non-economic, there was no jurisdictional element, and the chain to commerce
/// was attenuated. Held invalid.
#[must_use]
pub fn lopez_fact_pattern() -> CommercePowerAnalysis {
    CommercePowerAnalysis::new(
        "Gun-Free School Zones Act of 1990",
        "Possession of a firearm within a school zone",
    )
    .with_category(CommerceCategory::SubstantialEffects)
    .with_character(ActivityCharacter::NonEconomic)
    .with_attenuated_causal_chain()
}

/// Reconstruct *United States v. Morrison* (2000): the civil-remedy provision of
/// the Violence Against Women Act. Gender-motivated violence is non-economic;
/// despite extensive congressional findings, the effect rested on an attenuated
/// inferential chain and there was no jurisdictional element. Held invalid.
#[must_use]
pub fn morrison_fact_pattern() -> CommercePowerAnalysis {
    CommercePowerAnalysis::new(
        "Violence Against Women Act § 13981 (civil remedy)",
        "Gender-motivated violence",
    )
    .with_category(CommerceCategory::SubstantialEffects)
    .with_character(ActivityCharacter::NonEconomic)
    .with_congressional_findings()
    .with_attenuated_causal_chain()
}

/// Reconstruct *Wickard v. Filburn* (1942): the Agricultural Adjustment Act
/// limited wheat acreage; Filburn's home-consumed wheat was economic and part of
/// a comprehensive scheme regulating the interstate wheat market. Held valid.
#[must_use]
pub fn wickard_fact_pattern() -> CommercePowerAnalysis {
    CommercePowerAnalysis::new(
        "Agricultural Adjustment Act of 1938",
        "Production of wheat for on-farm consumption",
    )
    .with_category(CommerceCategory::SubstantialEffects)
    .with_character(ActivityCharacter::Economic)
    .part_of_comprehensive_scheme()
}

/// Reconstruct *Gonzales v. Raich* (2005): intrastate cultivation of marijuana
/// for personal medical use, regulated under the Controlled Substances Act.
/// Cultivation/consumption of a fungible commodity is economic and part of a
/// comprehensive scheme. Held within the commerce power.
#[must_use]
pub fn raich_fact_pattern() -> CommercePowerAnalysis {
    CommercePowerAnalysis::new(
        "Controlled Substances Act, 21 U.S.C. § 801 et seq.",
        "Intrastate cultivation and possession of marijuana for personal use",
    )
    .with_category(CommerceCategory::SubstantialEffects)
    .with_character(ActivityCharacter::Economic)
    .with_congressional_findings()
    .part_of_comprehensive_scheme()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_metadata() {
        assert!(
            CommerceCategory::Channels
                .leading_case()
                .contains("Heart of Atlanta")
        );
        assert!(
            CommerceCategory::SubstantialEffects
                .leading_case()
                .contains("Wickard")
        );
        assert!(
            CommerceCategory::Instrumentalities
                .description()
                .contains("Instrumentalities")
        );
    }

    #[test]
    fn test_channels_category_valid() {
        let result = CommercePowerAnalysis::new(
            "Civil Rights Act of 1964, Title II",
            "Racial discrimination in public accommodations serving interstate travelers",
        )
        .with_category(CommerceCategory::Channels)
        .analyze();

        assert_eq!(result.category, CommerceCategory::Channels);
        assert!(result.likely_valid);
        assert!(result.confidence >= 0.85);
    }

    #[test]
    fn test_instrumentalities_category_valid() {
        let result = CommercePowerAnalysis::new(
            "Federal railroad rate regulation",
            "Intrastate rail rates affecting interstate carriers",
        )
        .with_category(CommerceCategory::Instrumentalities)
        .analyze();

        assert!(result.likely_valid);
        assert_eq!(result.category, CommerceCategory::Instrumentalities);
    }

    #[test]
    fn test_lopez_is_invalid() {
        let result = lopez_fact_pattern().analyze();
        assert!(!result.likely_valid, "Lopez should be invalid");
        assert_eq!(result.category, CommerceCategory::SubstantialEffects);
        assert!(result.reasoning.iter().any(|r| r.contains("NON-ECONOMIC")));
    }

    #[test]
    fn test_morrison_is_invalid() {
        let result = morrison_fact_pattern().analyze();
        assert!(!result.likely_valid, "Morrison should be invalid");
        // Findings present but not dispositive.
        assert!(
            result
                .reasoning
                .iter()
                .any(|r| r.contains("not dispositive"))
        );
    }

    #[test]
    fn test_wickard_is_valid() {
        let result = wickard_fact_pattern().analyze();
        assert!(result.likely_valid, "Wickard should be valid");
        assert!(result.reasoning.iter().any(|r| r.contains("aggregate")));
    }

    #[test]
    fn test_raich_is_valid() {
        let result = raich_fact_pattern().analyze();
        assert!(result.likely_valid, "Raich should be valid");
        assert!(result.reasoning.iter().any(|r| r.contains("Raich")));
    }

    #[test]
    fn test_jurisdictional_element_saves_noneconomic() {
        // A felon-in-possession statute reaching firearms that have "traveled in
        // interstate commerce" survives because of the jurisdictional element
        // (cf. Scarborough; contrast Lopez).
        let result = CommercePowerAnalysis::new(
            "18 U.S.C. § 922(g) (felon in possession)",
            "Possession of a firearm that previously traveled in interstate commerce",
        )
        .with_category(CommerceCategory::SubstantialEffects)
        .with_character(ActivityCharacter::NonEconomic)
        .with_jurisdictional_element()
        .analyze();

        assert!(result.likely_valid);
        assert!(
            result
                .reasoning
                .iter()
                .any(|r| r.contains("jurisdictional element"))
        );
    }

    #[test]
    fn test_economic_activity_without_scheme_valid() {
        let result = CommercePowerAnalysis::new(
            "Fair Labor Standards Act",
            "Wages paid in a local manufacturing enterprise",
        )
        .with_category(CommerceCategory::SubstantialEffects)
        .with_character(ActivityCharacter::Economic)
        .analyze();

        assert!(result.likely_valid);
        assert!(result.confidence >= 0.75);
    }

    #[test]
    fn test_economic_but_attenuated_without_jurisdictional_element_invalid() {
        let result = CommercePowerAnalysis::new(
            "Hypothetical statute",
            "Economic-labeled activity with only an attenuated link to commerce",
        )
        .with_category(CommerceCategory::SubstantialEffects)
        .with_character(ActivityCharacter::Economic)
        .with_attenuated_causal_chain()
        .analyze();

        assert!(!result.likely_valid);
    }

    #[test]
    fn test_summary_renders() {
        let result = lopez_fact_pattern().analyze();
        let summary = result.summary();
        assert!(summary.contains("Federal Commerce Power Analysis"));
        assert!(summary.contains("LIKELY INVALID"));
        assert!(summary.contains("Substantial Effects"));
    }

    #[test]
    fn test_builder_fields() {
        let analysis = CommercePowerAnalysis::new("S", "A")
            .with_category(CommerceCategory::SubstantialEffects)
            .with_character(ActivityCharacter::NonEconomic)
            .with_jurisdictional_element()
            .with_congressional_findings()
            .part_of_comprehensive_scheme()
            .with_attenuated_causal_chain();

        assert!(analysis.jurisdictional_element);
        assert!(analysis.congressional_findings);
        assert!(analysis.part_of_regulatory_scheme);
        assert!(analysis.attenuated_causal_chain);
        assert!(!analysis.character.is_economic());
    }
}
