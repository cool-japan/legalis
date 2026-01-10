//! Trade Secrets Directive (EU) 2016/943
//!
//! Implements the three-part test for trade secret protection under EU law:
//! 1. Information is secret (not generally known)
//! 2. Has commercial value because it is secret
//! 3. Subject to reasonable steps to keep it secret

use super::error::IpError;
use super::types::TradeSecretCharacteristics;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Trade secret under EU Directive 2016/943
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TradeSecret {
    pub description: Option<String>,
    pub holder: Option<String>,
    pub characteristics: Option<TradeSecretCharacteristics>,
    pub protective_measures: Vec<String>,
}

impl TradeSecret {
    pub fn new() -> Self {
        Self {
            description: None,
            holder: None,
            characteristics: None,
            protective_measures: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_holder(mut self, holder: impl Into<String>) -> Self {
        self.holder = Some(holder.into());
        self
    }

    pub fn with_characteristics(mut self, characteristics: TradeSecretCharacteristics) -> Self {
        self.characteristics = Some(characteristics);
        self
    }

    pub fn add_protective_measure(mut self, measure: impl Into<String>) -> Self {
        self.protective_measures.push(measure.into());
        self
    }

    /// Validate trade secret protection under EU Directive 2016/943
    ///
    /// Applies the three-part test from Article 2(1):
    /// - (a) Information is secret (not generally known or readily accessible)
    /// - (b) Has commercial value because it is secret
    /// - (c) Subject to reasonable steps to keep it secret
    pub fn validate(&self) -> Result<TradeSecretValidation, IpError> {
        // Check required fields
        if self.description.is_none() {
            return Err(IpError::missing_field("description"));
        }

        if self.holder.is_none() {
            return Err(IpError::missing_field("holder"));
        }

        let chars = self
            .characteristics
            .as_ref()
            .ok_or_else(|| IpError::missing_field("characteristics"))?;

        // Article 2(1)(a): Information must be secret
        if !chars.is_secret {
            return Err(IpError::TradeSecretIssue {
                reason:
                    "Information must not be generally known or readily accessible (Art. 2(1)(a))"
                        .to_string(),
            });
        }

        // Article 2(1)(b): Must have commercial value because secret
        if !chars.has_commercial_value {
            return Err(IpError::TradeSecretIssue {
                reason:
                    "Information must have commercial value because it is secret (Art. 2(1)(b))"
                        .to_string(),
            });
        }

        // Article 2(1)(c): Reasonable steps to keep secret
        if !chars.reasonable_steps_taken {
            return Err(IpError::TradeSecretIssue {
                reason: "Reasonable steps must be taken to keep it secret (Art. 2(1)(c))"
                    .to_string(),
            });
        }

        // Assess adequacy of protective measures
        let protective_measures_adequate = !self.protective_measures.is_empty();

        let mut recommendations = Vec::new();
        if self.protective_measures.len() < 3 {
            recommendations.push(
                "Consider implementing additional protective measures (NDAs, access controls, encryption, etc.)".to_string()
            );
        }

        Ok(TradeSecretValidation {
            is_protectable: true,
            three_part_test_passed: true,
            protective_measures_adequate,
            recommendations,
        })
    }

    /// Analyze potential misappropriation
    pub fn analyze_misappropriation(
        &self,
        acquisition_method: AcquisitionMethod,
    ) -> MisappropriationAnalysis {
        let is_unlawful = match acquisition_method {
            AcquisitionMethod::UnauthorizedAccess
            | AcquisitionMethod::Breach
            | AcquisitionMethod::InducingBreach => true,
            AcquisitionMethod::IndependentDiscovery | AcquisitionMethod::ReverseEngineering => {
                false
            }
            AcquisitionMethod::ObservationOfPublicProduct => {
                // Lawful if product is publicly available
                false
            }
        };

        let applicable_articles = if is_unlawful {
            vec![
                "Article 4 - Unlawful acquisition".to_string(),
                "Article 13 - Damages".to_string(),
            ]
        } else {
            vec!["Article 3 - Lawful acquisition".to_string()]
        };

        MisappropriationAnalysis {
            is_unlawful,
            acquisition_method,
            applicable_articles,
            remedies_available: if is_unlawful {
                vec![
                    "Injunction (Art. 12)".to_string(),
                    "Damages (Art. 13)".to_string(),
                    "Product recall (Art. 12(1)(a))".to_string(),
                ]
            } else {
                Vec::new()
            },
        }
    }
}

impl Default for TradeSecret {
    fn default() -> Self {
        Self::new()
    }
}

/// Trade secret validation result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TradeSecretValidation {
    /// Whether information qualifies as a trade secret
    pub is_protectable: bool,

    /// Whether three-part test is passed
    pub three_part_test_passed: bool,

    /// Whether protective measures are adequate
    pub protective_measures_adequate: bool,

    /// Recommendations for strengthening protection
    pub recommendations: Vec<String>,
}

/// Method of acquiring trade secret
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AcquisitionMethod {
    /// Unauthorized access to documents, materials, etc.
    UnauthorizedAccess,
    /// Breach of confidentiality agreement
    Breach,
    /// Inducing breach of confidentiality
    InducingBreach,
    /// Independent discovery or creation
    IndependentDiscovery,
    /// Reverse engineering of lawfully acquired product
    ReverseEngineering,
    /// Observation of publicly available product
    ObservationOfPublicProduct,
}

/// Misappropriation analysis result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MisappropriationAnalysis {
    /// Whether acquisition was unlawful
    pub is_unlawful: bool,

    /// Method of acquisition
    pub acquisition_method: AcquisitionMethod,

    /// Applicable directive articles
    pub applicable_articles: Vec<String>,

    /// Available remedies if unlawful
    pub remedies_available: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_trade_secret() {
        let secret = TradeSecret::new()
            .with_description("Proprietary algorithm for data compression")
            .with_holder("Tech Corp")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: true,
                has_commercial_value: true,
                reasonable_steps_taken: true,
            })
            .add_protective_measure("NDA with employees")
            .add_protective_measure("Access control to source code")
            .add_protective_measure("Encryption at rest");

        let result = secret.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.is_protectable);
        assert!(validation.three_part_test_passed);
        assert!(validation.protective_measures_adequate);
    }

    #[test]
    fn test_trade_secret_not_secret() {
        let secret = TradeSecret::new()
            .with_description("Publicly known information")
            .with_holder("Company")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: false, // Generally known
                has_commercial_value: true,
                reasonable_steps_taken: true,
            });

        let result = secret.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(IpError::TradeSecretIssue { .. })));
    }

    #[test]
    fn test_trade_secret_no_commercial_value() {
        let secret = TradeSecret::new()
            .with_description("Personal information")
            .with_holder("Individual")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: true,
                has_commercial_value: false, // No commercial value
                reasonable_steps_taken: true,
            });

        let result = secret.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_trade_secret_no_reasonable_steps() {
        let secret = TradeSecret::new()
            .with_description("Secret recipe")
            .with_holder("Restaurant")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: true,
                has_commercial_value: true,
                reasonable_steps_taken: false, // No protective measures
            });

        let result = secret.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_unlawful_misappropriation_unauthorized_access() {
        let secret = TradeSecret::new()
            .with_description("Customer database")
            .with_holder("Company A")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: true,
                has_commercial_value: true,
                reasonable_steps_taken: true,
            });

        let analysis = secret.analyze_misappropriation(AcquisitionMethod::UnauthorizedAccess);

        assert!(analysis.is_unlawful);
        assert!(!analysis.remedies_available.is_empty());
        assert!(
            analysis
                .applicable_articles
                .contains(&"Article 4 - Unlawful acquisition".to_string())
        );
    }

    #[test]
    fn test_lawful_acquisition_independent_discovery() {
        let secret = TradeSecret::new()
            .with_description("Algorithm")
            .with_holder("Company A")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: true,
                has_commercial_value: true,
                reasonable_steps_taken: true,
            });

        let analysis = secret.analyze_misappropriation(AcquisitionMethod::IndependentDiscovery);

        assert!(!analysis.is_unlawful);
        assert!(analysis.remedies_available.is_empty());
    }

    #[test]
    fn test_lawful_acquisition_reverse_engineering() {
        let secret = TradeSecret::new()
            .with_description("Product design")
            .with_holder("Manufacturer")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: true,
                has_commercial_value: true,
                reasonable_steps_taken: true,
            });

        let analysis = secret.analyze_misappropriation(AcquisitionMethod::ReverseEngineering);

        assert!(!analysis.is_unlawful);
        assert_eq!(
            analysis.acquisition_method,
            AcquisitionMethod::ReverseEngineering
        );
    }

    #[test]
    fn test_insufficient_protective_measures_warning() {
        let secret = TradeSecret::new()
            .with_description("Trade secret")
            .with_holder("Company")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: true,
                has_commercial_value: true,
                reasonable_steps_taken: true,
            })
            .add_protective_measure("Basic NDA"); // Only 1 measure

        let result = secret.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();

        // Should recommend additional measures
        assert!(!validation.recommendations.is_empty());
    }
}
