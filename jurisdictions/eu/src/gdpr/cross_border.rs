//! GDPR Chapter V - Cross-Border Transfers (Articles 44-49)
//!
//! This module implements the GDPR rules for transferring personal data outside the EEA.
//!
//! ## Key Concepts
//!
//! - **Adequacy Decisions (Article 45)**: Countries/territories deemed by the European Commission
//!   to provide adequate data protection
//! - **Appropriate Safeguards (Article 46)**: Mechanisms for transfers without an adequacy decision
//!   (SCCs, BCRs, Codes of Conduct, etc.)
//! - **Derogations (Article 49)**: Specific situations allowing transfers without safeguards
//!
//! ## Example
//!
//! ```rust
//! use legalis_eu::gdpr::cross_border::*;
//!
//! let transfer = CrossBorderTransfer::new()
//!     .with_origin("EU")
//!     .with_destination_country("US")
//!     .with_safeguard(TransferSafeguard::StandardContractualClauses {
//!         version: "2021".to_string(),
//!         clauses_signed: true,
//!     });
//!
//! match transfer.validate() {
//!     Ok(validation) => println!("Transfer validation: {:?}", validation),
//!     Err(e) => println!("Transfer not allowed: {}", e),
//! }
//! ```

use crate::gdpr::error::GdprError;
use crate::shared::member_states::MemberState;
use chrono::{DateTime, Utc};
use legalis_core::LegalResult;
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Countries and territories with adequacy decisions under Article 45
///
/// This list should be updated when the European Commission adopts new adequacy decisions.
/// As of 2026, the following have adequacy decisions:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AdequateCountry {
    /// Andorra
    Andorra,
    /// Argentina
    Argentina,
    /// Canada (commercial organizations)
    Canada,
    /// Faroe Islands
    FaroeIslands,
    /// Guernsey
    Guernsey,
    /// Israel
    Israel,
    /// Isle of Man
    IsleOfMan,
    /// Jersey
    Jersey,
    /// New Zealand
    NewZealand,
    /// Republic of Korea (South Korea)
    SouthKorea,
    /// Switzerland
    Switzerland,
    /// United Kingdom
    UnitedKingdom,
    /// Uruguay
    Uruguay,
    /// Japan (under EU-Japan mutual adequacy arrangement)
    Japan,
}

impl AdequateCountry {
    /// Get the year when adequacy was granted
    pub fn adequacy_year(&self) -> u16 {
        match self {
            Self::Andorra => 2010,
            Self::Argentina => 2003,
            Self::Canada => 2002,
            Self::FaroeIslands => 2010,
            Self::Guernsey => 2003,
            Self::Israel => 2011,
            Self::IsleOfMan => 2004,
            Self::Jersey => 2008,
            Self::NewZealand => 2013,
            Self::SouthKorea => 2021,
            Self::Switzerland => 2000,
            Self::UnitedKingdom => 2021,
            Self::Uruguay => 2012,
            Self::Japan => 2019,
        }
    }

    /// Check if adequacy is still valid (may be revoked)
    pub fn is_valid(&self) -> bool {
        // All current adequacy decisions remain valid
        // This would need updating if any are suspended/revoked
        true
    }
}

/// Transfer safeguards under Article 46
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TransferSafeguard {
    /// Standard Contractual Clauses (SCCs) - Article 46(2)(c)
    StandardContractualClauses {
        /// SCC version (e.g., "2021" for current version)
        version: String,
        /// Whether clauses have been properly signed
        clauses_signed: bool,
    },

    /// Binding Corporate Rules (BCRs) - Article 46(2)(b)
    BindingCorporateRules {
        /// BCR approval authority
        approved_by: MemberState,
        /// Approval date
        approval_date: DateTime<Utc>,
    },

    /// Code of Conduct - Article 46(2)(e)
    CodeOfConduct {
        /// Code identifier
        code_id: String,
        /// Whether binding and enforceable commitments exist
        binding_commitments: bool,
    },

    /// Certification mechanism - Article 46(2)(f)
    Certification {
        /// Certification body
        certifying_authority: String,
        /// Whether binding and enforceable commitments exist
        binding_commitments: bool,
    },

    /// Contractual clauses approved by supervisory authority - Article 46(3)(a)
    AuthorityApprovedClauses {
        /// Approving supervisory authority
        approved_by: MemberState,
        /// Approval reference
        approval_ref: String,
    },

    /// Administrative arrangement - Article 46(3)(b)
    AdministrativeArrangement {
        /// Public authority arrangement
        arrangement_id: String,
    },
}

impl TransferSafeguard {
    /// Check if safeguard meets Article 46 requirements
    pub fn is_valid(&self) -> Result<bool, GdprError> {
        match self {
            Self::StandardContractualClauses {
                version,
                clauses_signed,
            } => {
                if !clauses_signed {
                    return Err(GdprError::invalid_transfer(
                        "SCCs must be properly signed by all parties",
                    ));
                }

                // Check if using outdated SCC version
                if version != "2021" {
                    return Err(GdprError::invalid_transfer(
                        "SCCs must use the 2021 version (old versions expired June 2022)",
                    ));
                }

                Ok(true)
            }

            Self::BindingCorporateRules {
                approved_by: _,
                approval_date,
            } => {
                // BCRs must have been approved
                if *approval_date > Utc::now() {
                    return Err(GdprError::invalid_transfer(
                        "BCR approval date cannot be in the future",
                    ));
                }
                Ok(true)
            }

            Self::CodeOfConduct {
                binding_commitments,
                ..
            } => {
                if !binding_commitments {
                    return Err(GdprError::invalid_transfer(
                        "Code of conduct must include binding and enforceable commitments",
                    ));
                }
                Ok(true)
            }

            Self::Certification {
                binding_commitments,
                ..
            } => {
                if !binding_commitments {
                    return Err(GdprError::invalid_transfer(
                        "Certification must include binding and enforceable commitments",
                    ));
                }
                Ok(true)
            }

            Self::AuthorityApprovedClauses { .. } => Ok(true),

            Self::AdministrativeArrangement { .. } => Ok(true),
        }
    }
}

/// Derogations for specific situations under Article 49
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TransferDerogation {
    /// Explicit consent - Article 49(1)(a)
    ExplicitConsent,

    /// Contract performance - Article 49(1)(b)
    ContractPerformance,

    /// Contract in data subject's interest - Article 49(1)(b)
    ContractInDataSubjectInterest,

    /// Important public interest - Article 49(1)(d)
    PublicInterest,

    /// Legal claims - Article 49(1)(e)
    LegalClaims,

    /// Vital interests - Article 49(1)(f)
    VitalInterests,

    /// Transfer from public register - Article 49(1)(g)
    PublicRegister,

    /// Compelling legitimate interests (not repetitive) - Article 49(1)(g) second subparagraph
    CompellingLegitimateInterests {
        /// Number of data subjects affected
        affected_data_subjects: u32,
        /// Whether transfer is repetitive
        is_repetitive: bool,
    },
}

impl TransferDerogation {
    /// Validate if derogation can be used
    pub fn validate(&self) -> Result<bool, GdprError> {
        match self {
            Self::CompellingLegitimateInterests {
                affected_data_subjects,
                is_repetitive,
            } => {
                if *is_repetitive {
                    return Err(GdprError::invalid_transfer(
                        "Article 49(1)(g) derogation cannot be used for repetitive transfers",
                    ));
                }

                // Recital 113: Should be limited to occasional transfers
                if *affected_data_subjects > 50 {
                    return Err(GdprError::invalid_transfer(
                        "Compelling legitimate interests derogation should be limited to occasional transfers affecting few data subjects",
                    ));
                }

                Ok(true)
            }
            _ => Ok(true),
        }
    }
}

/// Cross-border data transfer validation
#[derive(Debug, Clone)]
pub struct CrossBorderTransfer {
    /// Origin country/region
    pub origin: Option<String>,

    /// Destination country (ISO code or name)
    pub destination_country: Option<String>,

    /// Transfer safeguard (if applicable)
    pub safeguard: Option<TransferSafeguard>,

    /// Derogation (if applicable)
    pub derogation: Option<TransferDerogation>,

    /// Whether destination has adequacy decision
    pub adequate_destination: Option<AdequateCountry>,

    /// Data categories being transferred
    pub data_categories: Vec<String>,

    /// Transfer purpose
    pub purpose: Option<String>,
}

impl CrossBorderTransfer {
    pub fn new() -> Self {
        Self {
            origin: None,
            destination_country: None,
            safeguard: None,
            derogation: None,
            adequate_destination: None,
            data_categories: Vec::new(),
            purpose: None,
        }
    }

    pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn with_destination_country(mut self, country: impl Into<String>) -> Self {
        self.destination_country = Some(country.into());
        self
    }

    pub fn with_safeguard(mut self, safeguard: TransferSafeguard) -> Self {
        self.safeguard = Some(safeguard);
        self
    }

    pub fn with_derogation(mut self, derogation: TransferDerogation) -> Self {
        self.derogation = Some(derogation);
        self
    }

    pub fn with_adequate_destination(mut self, country: AdequateCountry) -> Self {
        self.adequate_destination = Some(country);
        self
    }

    pub fn add_data_category(mut self, category: impl Into<String>) -> Self {
        self.data_categories.push(category.into());
        self
    }

    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }

    /// Validate cross-border transfer compliance
    ///
    /// Returns `LegalResult` indicating whether transfer is permitted under GDPR Chapter V.
    pub fn validate(&self) -> Result<CrossBorderTransferValidation, GdprError> {
        // Check required fields
        if self.destination_country.is_none() {
            return Err(GdprError::missing_field("destination_country"));
        }

        // Article 45: Adequacy decision
        if let Some(adequate_country) = self.adequate_destination {
            if !adequate_country.is_valid() {
                return Err(GdprError::invalid_transfer(
                    "Adequacy decision has been revoked or suspended",
                ));
            }

            return Ok(CrossBorderTransferValidation {
                transfer_permitted: LegalResult::Deterministic(true),
                legal_basis: TransferLegalBasis::AdequacyDecision {
                    country: adequate_country,
                },
                additional_measures_required: false,
                risk_assessment_required: false,
            });
        }

        // Article 46: Appropriate safeguards
        if let Some(ref safeguard) = self.safeguard {
            safeguard.is_valid()?;

            // Schrems II: Transfer Impact Assessment required for US and similar countries
            let risk_assessment_required = matches!(
                self.destination_country.as_deref(),
                Some("US") | Some("United States") | Some("China") | Some("Russia")
            );

            return Ok(CrossBorderTransferValidation {
                transfer_permitted: if risk_assessment_required {
                    LegalResult::JudicialDiscretion {
                        issue: format!(
                            "Transfer to {} requires Transfer Impact Assessment (Schrems II)",
                            self.destination_country.as_deref().unwrap_or("unknown")
                        ),
                        context_id: Uuid::new_v4(),
                        narrative_hint: Some(
                            "Controller must assess whether destination country's laws allow \
                             government access to data that undermines safeguards. \
                             Consider: (1) supplementary measures, (2) encryption, \
                             (3) pseudonymization, (4) legal guarantees from recipient."
                                .to_string(),
                        ),
                    }
                } else {
                    LegalResult::Deterministic(true)
                },
                legal_basis: TransferLegalBasis::AppropriateSafeguards {
                    safeguard: safeguard.clone(),
                },
                additional_measures_required: risk_assessment_required,
                risk_assessment_required,
            });
        }

        // Article 49: Derogations (last resort)
        if let Some(derogation) = self.derogation {
            derogation.validate()?;

            return Ok(CrossBorderTransferValidation {
                transfer_permitted: LegalResult::Deterministic(true),
                legal_basis: TransferLegalBasis::Derogation { derogation },
                additional_measures_required: false,
                risk_assessment_required: false,
            });
        }

        // No legal basis found
        Err(GdprError::invalid_transfer(
            "Cross-border transfer lacks legal basis: no adequacy decision, \
             appropriate safeguards, or applicable derogation",
        ))
    }
}

impl Default for CrossBorderTransfer {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal basis for cross-border transfer
#[derive(Debug, Clone)]
pub enum TransferLegalBasis {
    /// Adequacy decision (Article 45)
    AdequacyDecision { country: AdequateCountry },

    /// Appropriate safeguards (Article 46)
    AppropriateSafeguards { safeguard: TransferSafeguard },

    /// Derogation (Article 49)
    Derogation { derogation: TransferDerogation },
}

/// Cross-border transfer validation result
#[derive(Debug, Clone)]
pub struct CrossBorderTransferValidation {
    /// Whether transfer is permitted
    pub transfer_permitted: LegalResult<bool>,

    /// Legal basis for transfer
    pub legal_basis: TransferLegalBasis,

    /// Whether additional supplementary measures are required (Schrems II)
    pub additional_measures_required: bool,

    /// Whether Transfer Impact Assessment is required
    pub risk_assessment_required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adequacy_transfer_to_switzerland() {
        let transfer = CrossBorderTransfer::new()
            .with_origin("EU")
            .with_destination_country("Switzerland")
            .with_adequate_destination(AdequateCountry::Switzerland);

        let result = transfer.validate().unwrap();
        assert!(matches!(
            result.transfer_permitted,
            LegalResult::Deterministic(true)
        ));
        assert!(!result.additional_measures_required);
    }

    #[test]
    fn test_scc_transfer_valid() {
        let transfer = CrossBorderTransfer::new()
            .with_origin("EU")
            .with_destination_country("US")
            .with_safeguard(TransferSafeguard::StandardContractualClauses {
                version: "2021".to_string(),
                clauses_signed: true,
            });

        let result = transfer.validate().unwrap();
        // Transfer to US requires TIA (Schrems II)
        assert!(matches!(
            result.transfer_permitted,
            LegalResult::JudicialDiscretion { .. }
        ));
        assert!(result.risk_assessment_required);
    }

    #[test]
    fn test_scc_old_version_fails() {
        let transfer = CrossBorderTransfer::new()
            .with_origin("EU")
            .with_destination_country("Singapore")
            .with_safeguard(TransferSafeguard::StandardContractualClauses {
                version: "2010".to_string(), // Old version
                clauses_signed: true,
            });

        let result = transfer.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_scc_unsigned_fails() {
        let transfer = CrossBorderTransfer::new()
            .with_origin("EU")
            .with_destination_country("Singapore")
            .with_safeguard(TransferSafeguard::StandardContractualClauses {
                version: "2021".to_string(),
                clauses_signed: false, // Not signed
            });

        let result = transfer.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_derogation_explicit_consent() {
        let transfer = CrossBorderTransfer::new()
            .with_origin("EU")
            .with_destination_country("Brazil")
            .with_derogation(TransferDerogation::ExplicitConsent);

        let result = transfer.validate().unwrap();
        assert!(matches!(
            result.transfer_permitted,
            LegalResult::Deterministic(true)
        ));
    }

    #[test]
    fn test_compelling_legitimate_interests_repetitive_fails() {
        let transfer = CrossBorderTransfer::new()
            .with_origin("EU")
            .with_destination_country("India")
            .with_derogation(TransferDerogation::CompellingLegitimateInterests {
                affected_data_subjects: 10,
                is_repetitive: true, // Repetitive transfers not allowed
            });

        let result = transfer.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_compelling_legitimate_interests_too_many_subjects_fails() {
        let transfer = CrossBorderTransfer::new()
            .with_origin("EU")
            .with_destination_country("India")
            .with_derogation(TransferDerogation::CompellingLegitimateInterests {
                affected_data_subjects: 100, // Too many subjects
                is_repetitive: false,
            });

        let result = transfer.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_no_legal_basis_fails() {
        let transfer = CrossBorderTransfer::new()
            .with_origin("EU")
            .with_destination_country("Unknown Country");
        // No safeguard, derogation, or adequacy

        let result = transfer.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_adequate_countries() {
        assert_eq!(AdequateCountry::Switzerland.adequacy_year(), 2000);
        assert_eq!(AdequateCountry::Japan.adequacy_year(), 2019);
        assert_eq!(AdequateCountry::UnitedKingdom.adequacy_year(), 2021);
        assert!(AdequateCountry::Switzerland.is_valid());
    }
}
