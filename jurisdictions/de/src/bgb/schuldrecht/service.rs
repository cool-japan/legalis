//! Service Contract (Dienstvertrag) - §§611-630 BGB
//!
//! Type-safe implementation of German service contract law under the BGB.
//!
//! # Legal Context
//!
//! A service contract (Dienstvertrag) is a contract where the person performing
//! the service (Dienstleistender) owes services to the recipient, and the
//! recipient owes remuneration if remuneration was agreed or customary (§611 BGB).
//!
//! ## Key Distinction: Dienstvertrag vs. Werkvertrag
//!
//! - **Dienstvertrag (§§611-630)**: Obligation to perform services (means obligation)
//!   - Focus: Effort and time spent
//!   - Example: Employment, consulting, medical treatment
//!   - No guarantee of specific result
//!
//! - **Werkvertrag (§§631-651)**: Obligation to produce a result (ends obligation)
//!   - Focus: Specific work result
//!   - Example: Construction, repair, software development
//!   - Guarantee of agreed result
//!
//! ## Core Provisions
//!
//! ### §611 BGB - Main Obligations
//! - **Service provider**: Owe promised services
//! - **Recipient**: Pay agreed remuneration
//!
//! ### §612 BGB - Remuneration (Vergütung)
//! - Remuneration presumed if services customarily remunerated
//! - Amount: According to tax schedule or customary rate
//!
//! ### §613-615 BGB - Performance and Payment
//! - Service must be performed personally (§613 BGB)
//! - Remuneration due after performance (§614 BGB)
//! - Acceptance risk: Recipient bears risk if unable to accept (§615 BGB)
//!
//! ### §620-630 BGB - Termination
//! - Fixed-term contracts end automatically
//! - Indefinite contracts: Ordinary termination with notice
//! - Extraordinary termination for important reason (§626 BGB)
//!
//! ## Employment Contracts (Arbeitsvertrag)
//!
//! Employment contracts are a special type of service contract with additional
//! protections under labor law (Arbeitsrecht).

#[cfg(test)]
use chrono::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bgb::schuldrecht::error::{Result, SchuldrechtError};
use crate::bgb::schuldrecht::types::{Contract, ContractTerms, Party};
use crate::gmbhg::Capital;

/// Service contract type (Dienstvertrag)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceContract {
    /// Base contract information
    pub base_contract: Contract,
    /// Service provider (Dienstleistender)
    pub service_provider: Party,
    /// Service recipient (Dienstberechtigter)
    pub service_recipient: Party,
    /// Description of services to be performed
    pub service_description: String,
    /// Remuneration (Vergütung)
    pub remuneration: ServiceRemuneration,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date (None for indefinite contract)
    pub end_date: Option<DateTime<Utc>>,
    /// Whether contract is fixed-term (befristet)
    pub fixed_term: bool,
    /// Whether service must be performed personally (§613 BGB)
    pub personal_performance_required: bool,
    /// Service provider's obligations
    pub provider_obligations: ServiceProviderObligations,
    /// Recipient's obligations
    pub recipient_obligations: ServiceRecipientObligations,
    /// Employment contract special provisions (if applicable)
    pub employment_provisions: Option<EmploymentProvisions>,
}

/// Service remuneration structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRemuneration {
    /// Type of remuneration
    pub remuneration_type: RemunerationType,
    /// Amount
    pub amount: Capital,
    /// Payment frequency
    pub payment_frequency: PaymentFrequency,
    /// Whether remuneration is agreed or presumed (§612 BGB)
    pub explicitly_agreed: bool,
}

/// Type of remuneration (Vergütungsart)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemunerationType {
    /// Fixed salary (Festgehalt)
    FixedSalary,
    /// Hourly rate (Stundenlohn)
    HourlyRate,
    /// Commission (Provision)
    Commission,
    /// Fee (Honorar) - for professionals
    Fee,
    /// Combination (Mischform)
    Combined,
}

/// Payment frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentFrequency {
    /// Hourly
    Hourly,
    /// Daily
    Daily,
    /// Weekly
    Weekly,
    /// Monthly
    Monthly,
    /// Upon completion
    UponCompletion,
    /// Other custom frequency
    Other,
}

/// Service provider's obligations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceProviderObligations {
    /// Perform promised services (§611 Abs. 1 BGB)
    pub perform_services: bool,
    /// Perform personally if required (§613 BGB)
    pub perform_personally: bool,
    /// Services performed with due care (Sorgfaltspflicht)
    pub services_performed_carefully: bool,
    /// Duty of loyalty (Treuepflicht) - especially in employment
    pub duty_of_loyalty: bool,
}

/// Service recipient's obligations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRecipientObligations {
    /// Pay agreed remuneration (§611 Abs. 1 BGB)
    pub pay_remuneration: bool,
    /// Accept services (Annahme)
    pub accept_services: bool,
    /// Payment timely (§614 BGB - after performance)
    pub payment_timely: bool,
    /// Bear acceptance risk (§615 BGB - Annahmeverzug)
    pub bears_acceptance_risk: bool,
}

/// Employment contract special provisions (Arbeitsvertrag)
///
/// Employment contracts are service contracts with additional protections
/// under labor law (Arbeitsrecht).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmploymentProvisions {
    /// Whether this is an employment contract (employee relationship)
    pub is_employment_contract: bool,
    /// Employee is subject to employer's instructions (Weisungsgebundenheit)
    pub subject_to_instructions: bool,
    /// Integration into employer's organization
    pub integrated_into_organization: bool,
    /// Job title/position (Stellenbezeichnung)
    pub job_title: String,
    /// Working hours per week
    pub working_hours_per_week: u8,
    /// Probationary period (Probezeit - max 6 months per §622 BGB)
    pub probationary_period_months: Option<u8>,
    /// Notice period for termination (Kündigungsfrist)
    pub notice_period_months: u8,
    /// Whether dismissal protection applies (Kündigungsschutz)
    pub dismissal_protection: bool,
}

/// Service contract termination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceTermination {
    /// Contract being terminated
    pub contract_id: String,
    /// Party terminating
    pub terminating_party: String,
    /// Type of termination
    pub termination_type: ServiceTerminationType,
    /// Notice date
    pub notice_date: DateTime<Utc>,
    /// Effective date
    pub effective_date: DateTime<Utc>,
    /// Notice period observed (in months)
    pub notice_period_months: u8,
    /// Whether in writing (required for employment contracts)
    pub in_writing: bool,
    /// Reason (required for extraordinary termination)
    pub reason: Option<TerminationReason>,
}

/// Type of service contract termination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceTerminationType {
    /// Ordinary termination (ordentliche Kündigung §620 BGB)
    Ordinary,
    /// Extraordinary termination for important reason (außerordentliche Kündigung §626 BGB)
    Extraordinary,
    /// Termination by agreement (Aufhebungsvertrag)
    ByAgreement,
}

/// Reasons for extraordinary termination (§626 BGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationReason {
    /// Serious breach of contract (schwere Pflichtverletzung)
    SeriousBreach,
    /// Loss of trust (Vertrauensverlust)
    LossOfTrust,
    /// Criminal offense (Straftat)
    CriminalOffense,
    /// Persistent inability to perform (dauernde Leistungsunfähigkeit)
    PersistentInability,
    /// Other important reason (sonstiger wichtiger Grund)
    OtherImportantReason,
}

/// Builder for service contracts
#[derive(Debug, Clone, Default)]
pub struct ServiceContractBuilder {
    service_provider: Option<Party>,
    service_recipient: Option<Party>,
    service_description: Option<String>,
    remuneration_type: Option<RemunerationType>,
    amount: Option<Capital>,
    payment_frequency: Option<PaymentFrequency>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    personal_performance_required: bool,
    is_employment_contract: bool,
    job_title: Option<String>,
    working_hours_per_week: Option<u8>,
}

impl ServiceContractBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the service provider
    pub fn service_provider(mut self, provider: Party) -> Self {
        self.service_provider = Some(provider);
        self
    }

    /// Set the service recipient
    pub fn service_recipient(mut self, recipient: Party) -> Self {
        self.service_recipient = Some(recipient);
        self
    }

    /// Set the service description
    pub fn service_description(mut self, description: String) -> Self {
        self.service_description = Some(description);
        self
    }

    /// Set remuneration type
    pub fn remuneration_type(mut self, rtype: RemunerationType) -> Self {
        self.remuneration_type = Some(rtype);
        self
    }

    /// Set remuneration amount
    pub fn amount(mut self, amount: Capital) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set payment frequency
    pub fn payment_frequency(mut self, frequency: PaymentFrequency) -> Self {
        self.payment_frequency = Some(frequency);
        self
    }

    /// Set start date
    pub fn start_date(mut self, date: DateTime<Utc>) -> Self {
        self.start_date = Some(date);
        self
    }

    /// Set end date (for fixed-term contract)
    pub fn end_date(mut self, date: DateTime<Utc>) -> Self {
        self.end_date = Some(date);
        self
    }

    /// Require personal performance (§613 BGB)
    pub fn personal_performance_required(mut self, required: bool) -> Self {
        self.personal_performance_required = required;
        self
    }

    /// Mark as employment contract
    pub fn employment_contract(mut self, is_employment: bool) -> Self {
        self.is_employment_contract = is_employment;
        self
    }

    /// Set job title (for employment contracts)
    pub fn job_title(mut self, title: String) -> Self {
        self.job_title = Some(title);
        self
    }

    /// Set working hours per week (for employment contracts)
    pub fn working_hours_per_week(mut self, hours: u8) -> Self {
        self.working_hours_per_week = Some(hours);
        self
    }

    /// Build the service contract
    pub fn build(self) -> Result<ServiceContract> {
        let service_provider =
            self.service_provider
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Service provider".to_string()],
                })?;

        let service_recipient =
            self.service_recipient
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Service recipient".to_string()],
                })?;

        let service_description =
            self.service_description
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Service description".to_string()],
                })?;

        let amount = self
            .amount
            .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                missing_terms: vec!["Remuneration amount".to_string()],
            })?;

        let remuneration_type = self
            .remuneration_type
            .unwrap_or(RemunerationType::FixedSalary);
        let payment_frequency = self.payment_frequency.unwrap_or(PaymentFrequency::Monthly);
        let start_date = self.start_date.unwrap_or_else(Utc::now);
        let fixed_term = self.end_date.is_some();

        let employment_provisions = if self.is_employment_contract {
            Some(EmploymentProvisions {
                is_employment_contract: true,
                subject_to_instructions: true,
                integrated_into_organization: true,
                job_title: self.job_title.unwrap_or_else(|| "Employee".to_string()),
                working_hours_per_week: self.working_hours_per_week.unwrap_or(40),
                probationary_period_months: Some(6),
                notice_period_months: 4, // Standard §622 BGB
                dismissal_protection: true,
            })
        } else {
            None
        };

        let contract = Contract {
            contract_id: format!("SERVICE-{}", Utc::now().timestamp()),
            parties: vec![service_provider.clone(), service_recipient.clone()],
            terms: ContractTerms {
                subject_matter: service_description.clone(),
                consideration: Some(amount),
                essential_terms: vec![
                    format!("Provider: {}", service_provider.name),
                    format!("Recipient: {}", service_recipient.name),
                    format!("Services: {}", service_description),
                    format!("Remuneration: € {:.2}", amount.to_euros()),
                ],
                additional_terms: vec![],
                includes_gtc: false,
            },
            concluded_at: Utc::now(),
            status: crate::bgb::schuldrecht::types::ContractStatus::Concluded,
            contract_type: crate::bgb::schuldrecht::types::ContractType::Service,
            obligations: vec![],
            in_writing: false,
        };

        Ok(ServiceContract {
            base_contract: contract,
            service_provider,
            service_recipient,
            service_description,
            remuneration: ServiceRemuneration {
                remuneration_type,
                amount,
                payment_frequency,
                explicitly_agreed: true,
            },
            start_date,
            end_date: self.end_date,
            fixed_term,
            personal_performance_required: self.personal_performance_required,
            provider_obligations: ServiceProviderObligations {
                perform_services: false,
                perform_personally: self.personal_performance_required,
                services_performed_carefully: true,
                duty_of_loyalty: self.is_employment_contract,
            },
            recipient_obligations: ServiceRecipientObligations {
                pay_remuneration: false,
                accept_services: false,
                payment_timely: false,
                bears_acceptance_risk: true,
            },
            employment_provisions,
        })
    }
}

/// Validate a service contract per §611 BGB
pub fn validate_service_contract(contract: &ServiceContract) -> Result<()> {
    // Validate provider capacity
    crate::bgb::schuldrecht::validator::validate_party_capacity(&contract.service_provider)?;

    // Validate recipient capacity
    crate::bgb::schuldrecht::validator::validate_party_capacity(&contract.service_recipient)?;

    // Validate service description
    if contract.service_description.is_empty() {
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: vec!["Service description".to_string()],
        });
    }

    // Validate remuneration
    if contract.remuneration.amount.amount_cents == 0 {
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: vec!["Valid remuneration amount".to_string()],
        });
    }

    // Validate employment provisions if present
    if let Some(provisions) = &contract.employment_provisions {
        validate_employment_provisions(provisions)?;
    }

    Ok(())
}

/// Validate employment contract provisions
pub fn validate_employment_provisions(provisions: &EmploymentProvisions) -> Result<()> {
    // Probationary period max 6 months (§622 Abs. 3 BGB)
    if let Some(probation) = provisions.probationary_period_months
        && probation > 6
    {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Probationary period cannot exceed 6 months (§622 Abs. 3 BGB)".to_string(),
        });
    }

    // Working hours should be reasonable (typically max 48 hours per week)
    if provisions.working_hours_per_week > 48 {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Working hours exceed reasonable limit (typically 48 hours/week)".to_string(),
        });
    }

    // Notice period must be at least statutory minimum (§622 BGB)
    if provisions.notice_period_months < 4 {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Notice period below statutory minimum (§622 BGB)".to_string(),
        });
    }

    Ok(())
}

/// Validate service contract termination per §§620-630 BGB
pub fn validate_service_termination(
    termination: &ServiceTermination,
    contract: &ServiceContract,
) -> Result<()> {
    match termination.termination_type {
        ServiceTerminationType::Ordinary => {
            validate_ordinary_service_termination(termination, contract)?;
        }
        ServiceTerminationType::Extraordinary => {
            validate_extraordinary_service_termination(termination)?;
        }
        ServiceTerminationType::ByAgreement => {
            // Always valid if both parties agree
            return Ok(());
        }
    }

    Ok(())
}

/// Validate ordinary termination per §620 BGB
fn validate_ordinary_service_termination(
    termination: &ServiceTermination,
    contract: &ServiceContract,
) -> Result<()> {
    // Fixed-term contracts cannot be ordinarily terminated (§620 Abs. 1 BGB)
    if contract.fixed_term {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Fixed-term service contract cannot be ordinarily terminated (§620 Abs. 1 BGB)"
                .to_string(),
        });
    }

    // Employment contracts require written form (§623 BGB)
    if contract.employment_provisions.is_some() && !termination.in_writing {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Employment contract termination must be in writing (§623 BGB)".to_string(),
        });
    }

    // Validate notice period
    if let Some(provisions) = &contract.employment_provisions
        && termination.notice_period_months < provisions.notice_period_months
    {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: format!(
                "Notice period {} months below contractual requirement {} months (§622 BGB)",
                termination.notice_period_months, provisions.notice_period_months
            ),
        });
    }

    Ok(())
}

/// Validate extraordinary termination per §626 BGB
fn validate_extraordinary_service_termination(termination: &ServiceTermination) -> Result<()> {
    // Extraordinary termination requires important reason (§626 BGB)
    if termination.reason.is_none() {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Extraordinary termination requires important reason (§626 BGB)".to_string(),
        });
    }

    // Must be exercised within 2 weeks of knowledge (§626 Abs. 2 BGB)
    // Note: This would require additional context about when the reason became known

    Ok(())
}

/// Calculate statutory notice period per §622 BGB (employment contracts)
pub fn calculate_statutory_notice_period(years_of_service: u8) -> u8 {
    // §622 Abs. 2 BGB - notice period increases with service duration
    match years_of_service {
        0..=1 => 4, // 4 weeks (basic period)
        2 => 1,     // 1 month
        5 => 2,     // 2 months
        8 => 3,     // 3 months
        10 => 4,    // 4 months
        12 => 5,    // 5 months
        15 => 6,    // 6 months
        20.. => 7,  // 7 months
        _ => 4,     // Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bgb::schuldrecht::types::{LegalCapacity, PartyType};

    fn create_test_service_provider() -> Party {
        Party {
            name: "Dr. Anna Schmidt".to_string(),
            address: "Munich".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::NaturalPerson,
        }
    }

    fn create_test_service_recipient() -> Party {
        Party {
            name: "Tech Corp GmbH".to_string(),
            address: "Berlin".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::LegalEntity,
        }
    }

    #[test]
    fn test_service_contract_builder_valid() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider)
            .service_recipient(recipient)
            .service_description("IT consulting services".to_string())
            .remuneration_type(RemunerationType::HourlyRate)
            .amount(Capital::from_euros(150))
            .payment_frequency(PaymentFrequency::Monthly)
            .build();

        assert!(contract.is_ok());
        let contract = contract.unwrap();
        assert_eq!(contract.remuneration.amount, Capital::from_euros(150));
        assert_eq!(
            contract.remuneration.remuneration_type,
            RemunerationType::HourlyRate
        );
    }

    #[test]
    fn test_service_contract_builder_missing_provider() {
        let recipient = create_test_service_recipient();

        let result = ServiceContractBuilder::new()
            .service_recipient(recipient)
            .service_description("Consulting".to_string())
            .amount(Capital::from_euros(100))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_employment_contract_builder() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider)
            .service_recipient(recipient)
            .service_description("Software development".to_string())
            .amount(Capital::from_euros(5_000))
            .payment_frequency(PaymentFrequency::Monthly)
            .employment_contract(true)
            .job_title("Senior Developer".to_string())
            .working_hours_per_week(40)
            .build();

        assert!(contract.is_ok());
        let contract = contract.unwrap();
        assert!(contract.employment_provisions.is_some());
        let provisions = contract.employment_provisions.unwrap();
        assert_eq!(provisions.job_title, "Senior Developer");
        assert_eq!(provisions.working_hours_per_week, 40);
    }

    #[test]
    fn test_validate_service_contract_valid() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider)
            .service_recipient(recipient)
            .service_description("Consulting".to_string())
            .amount(Capital::from_euros(100))
            .build()
            .unwrap();

        assert!(validate_service_contract(&contract).is_ok());
    }

    #[test]
    fn test_employment_provisions_probation_period_max_6_months() {
        let provisions = EmploymentProvisions {
            is_employment_contract: true,
            subject_to_instructions: true,
            integrated_into_organization: true,
            job_title: "Employee".to_string(),
            working_hours_per_week: 40,
            probationary_period_months: Some(6),
            notice_period_months: 4,
            dismissal_protection: true,
        };

        assert!(validate_employment_provisions(&provisions).is_ok());
    }

    #[test]
    fn test_employment_provisions_probation_period_exceeds_6_months_fails() {
        let provisions = EmploymentProvisions {
            is_employment_contract: true,
            subject_to_instructions: true,
            integrated_into_organization: true,
            job_title: "Employee".to_string(),
            working_hours_per_week: 40,
            probationary_period_months: Some(8),
            notice_period_months: 4,
            dismissal_protection: true,
        };

        assert!(validate_employment_provisions(&provisions).is_err());
    }

    #[test]
    fn test_employment_provisions_working_hours_reasonable() {
        let provisions = EmploymentProvisions {
            is_employment_contract: true,
            subject_to_instructions: true,
            integrated_into_organization: true,
            job_title: "Employee".to_string(),
            working_hours_per_week: 48,
            probationary_period_months: Some(6),
            notice_period_months: 4,
            dismissal_protection: true,
        };

        assert!(validate_employment_provisions(&provisions).is_ok());
    }

    #[test]
    fn test_employment_provisions_working_hours_excessive_fails() {
        let provisions = EmploymentProvisions {
            is_employment_contract: true,
            subject_to_instructions: true,
            integrated_into_organization: true,
            job_title: "Employee".to_string(),
            working_hours_per_week: 60,
            probationary_period_months: Some(6),
            notice_period_months: 4,
            dismissal_protection: true,
        };

        assert!(validate_employment_provisions(&provisions).is_err());
    }

    #[test]
    fn test_ordinary_termination_indefinite_contract_valid() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider.clone())
            .service_recipient(recipient)
            .service_description("Consulting".to_string())
            .amount(Capital::from_euros(100))
            .build()
            .unwrap();

        let termination = ServiceTermination {
            contract_id: "SERVICE-123".to_string(),
            terminating_party: provider.name,
            termination_type: ServiceTerminationType::Ordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(120),
            notice_period_months: 4,
            in_writing: true,
            reason: None,
        };

        assert!(validate_service_termination(&termination, &contract).is_ok());
    }

    #[test]
    fn test_ordinary_termination_fixed_term_fails() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider.clone())
            .service_recipient(recipient)
            .service_description("Consulting".to_string())
            .amount(Capital::from_euros(100))
            .end_date(Utc::now() + Duration::days(365))
            .build()
            .unwrap();

        let termination = ServiceTermination {
            contract_id: "SERVICE-123".to_string(),
            terminating_party: provider.name,
            termination_type: ServiceTerminationType::Ordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(120),
            notice_period_months: 4,
            in_writing: true,
            reason: None,
        };

        assert!(validate_service_termination(&termination, &contract).is_err());
    }

    #[test]
    fn test_employment_termination_requires_written_form() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider.clone())
            .service_recipient(recipient)
            .service_description("Software development".to_string())
            .amount(Capital::from_euros(5_000))
            .employment_contract(true)
            .build()
            .unwrap();

        let termination = ServiceTermination {
            contract_id: "SERVICE-123".to_string(),
            terminating_party: provider.name,
            termination_type: ServiceTerminationType::Ordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(120),
            notice_period_months: 4,
            in_writing: false,
            reason: None,
        };

        assert!(validate_service_termination(&termination, &contract).is_err());
    }

    #[test]
    fn test_extraordinary_termination_requires_reason() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider.clone())
            .service_recipient(recipient)
            .service_description("Consulting".to_string())
            .amount(Capital::from_euros(100))
            .build()
            .unwrap();

        let termination = ServiceTermination {
            contract_id: "SERVICE-123".to_string(),
            terminating_party: provider.name,
            termination_type: ServiceTerminationType::Extraordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now(),
            notice_period_months: 0,
            in_writing: true,
            reason: None,
        };

        assert!(validate_service_termination(&termination, &contract).is_err());
    }

    #[test]
    fn test_extraordinary_termination_with_reason_valid() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider.clone())
            .service_recipient(recipient)
            .service_description("Consulting".to_string())
            .amount(Capital::from_euros(100))
            .build()
            .unwrap();

        let termination = ServiceTermination {
            contract_id: "SERVICE-123".to_string(),
            terminating_party: provider.name,
            termination_type: ServiceTerminationType::Extraordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now(),
            notice_period_months: 0,
            in_writing: true,
            reason: Some(TerminationReason::SeriousBreach),
        };

        assert!(validate_service_termination(&termination, &contract).is_ok());
    }

    #[test]
    fn test_termination_by_agreement_always_valid() {
        let provider = create_test_service_provider();
        let recipient = create_test_service_recipient();

        let contract = ServiceContractBuilder::new()
            .service_provider(provider.clone())
            .service_recipient(recipient)
            .service_description("Consulting".to_string())
            .amount(Capital::from_euros(100))
            .build()
            .unwrap();

        let termination = ServiceTermination {
            contract_id: "SERVICE-123".to_string(),
            terminating_party: "Both parties".to_string(),
            termination_type: ServiceTerminationType::ByAgreement,
            notice_date: Utc::now(),
            effective_date: Utc::now(),
            notice_period_months: 0,
            in_writing: true,
            reason: None,
        };

        assert!(validate_service_termination(&termination, &contract).is_ok());
    }

    #[test]
    fn test_calculate_statutory_notice_period() {
        assert_eq!(calculate_statutory_notice_period(0), 4); // 4 weeks
        assert_eq!(calculate_statutory_notice_period(1), 4); // 4 weeks
        assert_eq!(calculate_statutory_notice_period(2), 1); // 1 month
        assert_eq!(calculate_statutory_notice_period(5), 2); // 2 months
        assert_eq!(calculate_statutory_notice_period(8), 3); // 3 months
        assert_eq!(calculate_statutory_notice_period(10), 4); // 4 months
        assert_eq!(calculate_statutory_notice_period(12), 5); // 5 months
        assert_eq!(calculate_statutory_notice_period(15), 6); // 6 months
        assert_eq!(calculate_statutory_notice_period(20), 7); // 7 months
    }
}
