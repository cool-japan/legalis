//! Lease/Rental Contract (Mietvertrag) - §§535-580a BGB
//!
//! Type-safe implementation of German lease law under the BGB.
//!
//! # Legal Context
//!
//! A lease contract (Mietvertrag) is a contract where the landlord obligates
//! to grant the tenant the use of the leased property for the duration of the
//! lease, and the tenant obligates to pay the agreed rent (§535 BGB).
//!
//! ## Core Provisions
//!
//! ### §535 BGB - Main Obligations
//! - **Landlord (Vermieter)**: Grant use, maintain in usable condition
//! - **Tenant (Mieter)**: Pay rent, use property in contractual manner
//!
//! ### §536-536d BGB - Defects (Mängel)
//! - Landlord's duty to maintain (§535 Abs. 1 S. 2 BGB)
//! - Tenant's right to rent reduction for defects (Mietminderung §536 BGB)
//! - Tenant's duty to notify defects (Mängelanzeige §536c BGB)
//! - Landlord's liability for damages
//!
//! ### §542-575a BGB - Termination (Kündigung)
//! - Ordinary termination (ordentliche Kündigung)
//! - Extraordinary termination (außerordentliche Kündigung §543 BGB)
//! - Notice periods (Kündigungsfristen §573c, §580a BGB)
//! - Tenant protection (Kündigungsschutz)
//!
//! ### Residential vs Commercial
//! - Residential lease (Wohnraummiete): Stronger tenant protections
//! - Commercial lease (Geschäftsraummiete): More contractual freedom

#[cfg(test)]
use chrono::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bgb::schuldrecht::error::{Result, SchuldrechtError};
use crate::bgb::schuldrecht::types::{Contract, ContractTerms, Party};
use crate::gmbhg::Capital;

/// Lease contract type (Mietvertrag)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseContract {
    /// Base contract information
    pub base_contract: Contract,
    /// Landlord (Vermieter)
    pub landlord: Party,
    /// Tenant (Mieter)
    pub tenant: Party,
    /// Leased property description
    pub leased_property: String,
    /// Monthly rent (Miete)
    pub monthly_rent: Capital,
    /// Additional costs (Nebenkosten)
    pub additional_costs: Option<Capital>,
    /// Security deposit (Kaution - max 3 months' rent per §551 BGB)
    pub security_deposit: Option<Capital>,
    /// Lease type (residential vs commercial)
    pub lease_type: LeaseType,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date (None for indefinite lease)
    pub end_date: Option<DateTime<Utc>>,
    /// Whether lease is fixed-term (befristet)
    pub fixed_term: bool,
    /// Landlord's obligations
    pub landlord_obligations: LandlordObligations,
    /// Tenant's obligations
    pub tenant_obligations: TenantObligations,
    /// Defect information
    pub defect_info: Option<DefectInfo>,
    /// Rent adjustment history
    pub rent_adjustments: Vec<RentAdjustment>,
}

/// Type of lease (Wohnraum vs Geschäftsraum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseType {
    /// Residential lease (Wohnraummiete) - §§549-577a BGB
    /// Stronger tenant protections apply
    Residential,
    /// Commercial lease (Geschäftsraummiete) - §§578-580a BGB
    /// More contractual freedom
    Commercial,
}

/// Landlord's main obligations per §535 Abs. 1 BGB
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandlordObligations {
    /// Grant tenant use of property (Gebrauch überlassen)
    pub grant_use: bool,
    /// Maintain property in usable condition (in vertragsgemäßem Zustand erhalten)
    pub maintain_usable_condition: bool,
    /// Property delivered in agreed condition
    pub delivered_in_agreed_condition: bool,
    /// Defects remedied in timely manner
    pub defects_remedied: bool,
}

/// Tenant's main obligations per §535 Abs. 2 BGB
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantObligations {
    /// Pay rent (Miete entrichten)
    pub pay_rent: bool,
    /// Pay additional costs (Nebenkosten zahlen)
    pub pay_additional_costs: bool,
    /// Use property in contractual manner (vertragsgemäßer Gebrauch)
    pub use_property_properly: bool,
    /// Notify landlord of defects (Mängel anzeigen §536c BGB)
    pub notify_defects: bool,
    /// Return property at end of lease
    pub return_property: bool,
}

/// Defect information (Mängel)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefectInfo {
    /// Description of defect
    pub description: String,
    /// When defect occurred/discovered
    pub discovered_at: DateTime<Utc>,
    /// Whether tenant notified landlord (§536c BGB)
    pub landlord_notified: bool,
    /// When landlord was notified
    pub notified_at: Option<DateTime<Utc>>,
    /// Whether defect reduces usability
    pub reduces_usability: bool,
    /// Percentage rent reduction (Mietminderung §536 BGB)
    pub rent_reduction_percentage: u8,
    /// Whether landlord liable for damages
    pub landlord_liable: bool,
}

/// Rent adjustment (Mieterhöhung)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RentAdjustment {
    /// Old rent amount
    pub old_rent: Capital,
    /// New rent amount
    pub new_rent: Capital,
    /// Effective date
    pub effective_date: DateTime<Utc>,
    /// Legal basis for adjustment
    pub legal_basis: RentAdjustmentBasis,
    /// Whether tenant consented (if required)
    pub tenant_consent: bool,
}

/// Legal basis for rent adjustment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RentAdjustmentBasis {
    /// Agreement between parties (Vereinbarung)
    Agreement,
    /// Adjustment to local comparative rent (§558 BGB - Ortsübliche Vergleichsmiete)
    ComparativeRent,
    /// Stepped rent (Staffelmiete §557a BGB)
    SteppedRent,
    /// Index-linked rent (Indexmiete §557b BGB)
    IndexLinkedRent,
    /// After modernization (§559 BGB)
    Modernization,
}

/// Termination information (Kündigung)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseTermination {
    /// Contract being terminated
    pub contract_id: String,
    /// Party terminating (landlord or tenant)
    pub terminating_party: String,
    /// Type of termination
    pub termination_type: TerminationType,
    /// Termination date (when notice given)
    pub notice_date: DateTime<Utc>,
    /// Effective date (when lease ends)
    pub effective_date: DateTime<Utc>,
    /// Notice period observed (Kündigungsfrist)
    pub notice_period_months: u8,
    /// Whether in writing (§568 BGB - must be in writing)
    pub in_writing: bool,
    /// Reason (required for landlord ordinary termination)
    pub reason: Option<TerminationReason>,
}

/// Type of termination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationType {
    /// Ordinary termination (ordentliche Kündigung §573 BGB)
    Ordinary,
    /// Extraordinary termination (außerordentliche Kündigung §543 BGB)
    Extraordinary,
}

/// Reasons for termination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationReason {
    /// Landlord's legitimate interest (§573 BGB)
    /// - Personal use (Eigenbedarf)
    /// - Tenant breach
    /// - Reasonable utilization prevented
    LandlordLegitimateInterest,

    /// Tenant breach (Pflichtverletzung §543 Abs. 2 BGB)
    TenantBreach,

    /// Rent arrears (Zahlungsverzug §543 Abs. 2 Nr. 3 BGB)
    /// Extraordinary termination if 2 months' rent unpaid
    RentArrears,

    /// Property damage (Beschädigung der Mietsache)
    PropertyDamage,

    /// No reason required (tenant can always terminate)
    TenantVoluntary,
}

/// Builder for lease contracts
#[derive(Debug, Clone, Default)]
pub struct LeaseContractBuilder {
    landlord: Option<Party>,
    tenant: Option<Party>,
    leased_property: Option<String>,
    monthly_rent: Option<Capital>,
    additional_costs: Option<Capital>,
    security_deposit: Option<Capital>,
    lease_type: Option<LeaseType>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    fixed_term: bool,
}

impl LeaseContractBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the landlord
    pub fn landlord(mut self, landlord: Party) -> Self {
        self.landlord = Some(landlord);
        self
    }

    /// Set the tenant
    pub fn tenant(mut self, tenant: Party) -> Self {
        self.tenant = Some(tenant);
        self
    }

    /// Set the leased property description
    pub fn leased_property(mut self, property: String) -> Self {
        self.leased_property = Some(property);
        self
    }

    /// Set the monthly rent
    pub fn monthly_rent(mut self, rent: Capital) -> Self {
        self.monthly_rent = Some(rent);
        self
    }

    /// Set additional costs (Nebenkosten)
    pub fn additional_costs(mut self, costs: Capital) -> Self {
        self.additional_costs = Some(costs);
        self
    }

    /// Set security deposit (max 3 months' rent per §551 BGB)
    pub fn security_deposit(mut self, deposit: Capital) -> Self {
        self.security_deposit = Some(deposit);
        self
    }

    /// Set lease type (residential or commercial)
    pub fn lease_type(mut self, lease_type: LeaseType) -> Self {
        self.lease_type = Some(lease_type);
        self
    }

    /// Set start date
    pub fn start_date(mut self, date: DateTime<Utc>) -> Self {
        self.start_date = Some(date);
        self
    }

    /// Set end date (for fixed-term lease)
    pub fn end_date(mut self, date: DateTime<Utc>) -> Self {
        self.end_date = Some(date);
        self.fixed_term = true;
        self
    }

    /// Build the lease contract
    pub fn build(self) -> Result<LeaseContract> {
        let landlord = self
            .landlord
            .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                missing_terms: vec!["Landlord".to_string()],
            })?;

        let tenant = self
            .tenant
            .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                missing_terms: vec!["Tenant".to_string()],
            })?;

        let leased_property =
            self.leased_property
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Leased property".to_string()],
                })?;

        let monthly_rent =
            self.monthly_rent
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Monthly rent".to_string()],
                })?;

        let lease_type = self.lease_type.unwrap_or(LeaseType::Residential);
        let start_date = self.start_date.unwrap_or_else(Utc::now);

        let contract = Contract {
            contract_id: format!("LEASE-{}", Utc::now().timestamp()),
            parties: vec![landlord.clone(), tenant.clone()],
            terms: ContractTerms {
                subject_matter: leased_property.clone(),
                consideration: Some(monthly_rent),
                essential_terms: vec![
                    format!("Landlord: {}", landlord.name),
                    format!("Tenant: {}", tenant.name),
                    format!("Property: {}", leased_property),
                    format!("Monthly rent: € {:.2}", monthly_rent.to_euros()),
                ],
                additional_terms: vec![],
                includes_gtc: false,
            },
            concluded_at: Utc::now(),
            status: crate::bgb::schuldrecht::types::ContractStatus::Concluded,
            contract_type: crate::bgb::schuldrecht::types::ContractType::Lease,
            obligations: vec![],
            in_writing: false,
        };

        Ok(LeaseContract {
            base_contract: contract,
            landlord,
            tenant,
            leased_property,
            monthly_rent,
            additional_costs: self.additional_costs,
            security_deposit: self.security_deposit,
            lease_type,
            start_date,
            end_date: self.end_date,
            fixed_term: self.fixed_term,
            landlord_obligations: LandlordObligations {
                grant_use: false,
                maintain_usable_condition: true,
                delivered_in_agreed_condition: false,
                defects_remedied: false,
            },
            tenant_obligations: TenantObligations {
                pay_rent: false,
                pay_additional_costs: false,
                use_property_properly: true,
                notify_defects: true,
                return_property: false,
            },
            defect_info: None,
            rent_adjustments: vec![],
        })
    }
}

/// Validate a lease contract per §535 BGB
pub fn validate_lease_contract(contract: &LeaseContract) -> Result<()> {
    // Validate landlord capacity
    crate::bgb::schuldrecht::validator::validate_party_capacity(&contract.landlord)?;

    // Validate tenant capacity
    crate::bgb::schuldrecht::validator::validate_party_capacity(&contract.tenant)?;

    // Validate monthly rent is positive
    if contract.monthly_rent.amount_cents == 0 {
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: vec!["Valid monthly rent".to_string()],
        });
    }

    // Validate property description
    if contract.leased_property.is_empty() {
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: vec!["Leased property description".to_string()],
        });
    }

    // Validate security deposit (§551 BGB - max 3 months' rent)
    if let Some(deposit) = contract.security_deposit {
        let max_deposit = Capital::from_cents(contract.monthly_rent.amount_cents * 3);
        if deposit.amount_cents > max_deposit.amount_cents {
            return Err(SchuldrechtError::InvalidContractTerms {
                reason: format!(
                    "Security deposit exceeds 3 months' rent (§551 BGB): € {:.2} > € {:.2}",
                    deposit.to_euros(),
                    max_deposit.to_euros()
                ),
            });
        }
    }

    Ok(())
}

/// Validate defect notification per §536c BGB
pub fn validate_defect_notification(defect: &DefectInfo) -> Result<()> {
    // Tenant must notify landlord of defects (§536c BGB)
    if !defect.landlord_notified {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Tenant must notify landlord of defects (§536c BGB)".to_string(),
        });
    }

    // Notification must have timestamp
    if defect.notified_at.is_none() {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Notification timestamp required".to_string(),
        });
    }

    // Rent reduction percentage must be reasonable (0-100%)
    if defect.rent_reduction_percentage > 100 {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Rent reduction percentage cannot exceed 100%".to_string(),
        });
    }

    Ok(())
}

/// Calculate rent reduction amount per §536 BGB
pub fn calculate_rent_reduction(monthly_rent: Capital, reduction_percentage: u8) -> Capital {
    let reduction_cents = (monthly_rent.amount_cents * u64::from(reduction_percentage)) / 100;
    Capital::from_cents(reduction_cents)
}

/// Validate rent adjustment per §§558-559 BGB
pub fn validate_rent_adjustment(adjustment: &RentAdjustment, lease_type: LeaseType) -> Result<()> {
    // New rent must be higher than old rent
    if adjustment.new_rent.amount_cents <= adjustment.old_rent.amount_cents {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "New rent must be higher than old rent".to_string(),
        });
    }

    match adjustment.legal_basis {
        RentAdjustmentBasis::Agreement => {
            // Agreement always valid if parties consent
            Ok(())
        }
        RentAdjustmentBasis::ComparativeRent => {
            // §558 BGB - only for residential leases
            if lease_type != LeaseType::Residential {
                return Err(SchuldrechtError::InvalidContractTerms {
                    reason: "Comparative rent adjustment only for residential leases (§558 BGB)"
                        .to_string(),
                });
            }
            // Tenant consent required
            if !adjustment.tenant_consent {
                return Err(SchuldrechtError::InvalidContractTerms {
                    reason: "Tenant consent required for comparative rent adjustment (§558 BGB)"
                        .to_string(),
                });
            }
            Ok(())
        }
        RentAdjustmentBasis::SteppedRent | RentAdjustmentBasis::IndexLinkedRent => {
            // Must be agreed in advance in contract
            Ok(())
        }
        RentAdjustmentBasis::Modernization => {
            // §559 BGB - landlord can pass on 8% of modernization costs annually
            Ok(())
        }
    }
}

/// Validate lease termination per §§542-575a BGB
pub fn validate_lease_termination(
    termination: &LeaseTermination,
    lease: &LeaseContract,
) -> Result<()> {
    // Termination must be in writing (§568 BGB)
    if !termination.in_writing {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Lease termination must be in writing (§568 BGB)".to_string(),
        });
    }

    match termination.termination_type {
        TerminationType::Ordinary => {
            validate_ordinary_termination(termination, lease)?;
        }
        TerminationType::Extraordinary => {
            validate_extraordinary_termination(termination)?;
        }
    }

    Ok(())
}

/// Validate ordinary termination per §573 BGB
fn validate_ordinary_termination(
    termination: &LeaseTermination,
    lease: &LeaseContract,
) -> Result<()> {
    // Fixed-term leases cannot be ordinarily terminated (§575 BGB)
    if lease.fixed_term {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Fixed-term lease cannot be ordinarily terminated (§575 BGB)".to_string(),
        });
    }

    // Landlord needs legitimate interest (§573 BGB)
    if termination.terminating_party == lease.landlord.name && termination.reason.is_none() {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Landlord must provide legitimate interest for ordinary termination (§573 BGB)"
                .to_string(),
        });
    }

    // Check notice period
    match lease.lease_type {
        LeaseType::Residential => {
            // §573c BGB - 3 months minimum for residential
            if termination.notice_period_months < 3 {
                return Err(SchuldrechtError::InvalidContractTerms {
                    reason: "Residential lease requires 3 months' notice (§573c BGB)".to_string(),
                });
            }
        }
        LeaseType::Commercial => {
            // §580a BGB - notice periods can be agreed
            // Standard: 6 months
            if termination.notice_period_months < 6 {
                return Err(SchuldrechtError::InvalidContractTerms {
                    reason: "Commercial lease typically requires 6 months' notice (§580a BGB)"
                        .to_string(),
                });
            }
        }
    }

    Ok(())
}

/// Validate extraordinary termination per §543 BGB
fn validate_extraordinary_termination(termination: &LeaseTermination) -> Result<()> {
    // Extraordinary termination requires important reason (wichtiger Grund)
    if termination.reason.is_none() {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Extraordinary termination requires important reason (§543 BGB)".to_string(),
        });
    }

    // No notice period required for extraordinary termination
    // Can be effective immediately or at next rent payment date

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bgb::schuldrecht::types::{LegalCapacity, PartyType};

    fn create_test_landlord() -> Party {
        Party {
            name: "Landlord GmbH".to_string(),
            address: "Berlin".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::LegalEntity,
        }
    }

    fn create_test_tenant() -> Party {
        Party {
            name: "Max Mustermann".to_string(),
            address: "Munich".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::NaturalPerson,
        }
    }

    #[test]
    fn test_lease_contract_builder_valid() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();

        let contract = LeaseContractBuilder::new()
            .landlord(landlord)
            .tenant(tenant)
            .leased_property("2-room apartment, 60 sqm".to_string())
            .monthly_rent(Capital::from_euros(800))
            .lease_type(LeaseType::Residential)
            .build();

        assert!(contract.is_ok());
        let contract = contract.unwrap();
        assert_eq!(contract.monthly_rent, Capital::from_euros(800));
        assert_eq!(contract.lease_type, LeaseType::Residential);
    }

    #[test]
    fn test_lease_contract_builder_missing_landlord() {
        let tenant = create_test_tenant();

        let result = LeaseContractBuilder::new()
            .tenant(tenant)
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_lease_contract_valid() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();

        let contract = LeaseContractBuilder::new()
            .landlord(landlord)
            .tenant(tenant)
            .leased_property("2-room apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .build()
            .unwrap();

        assert!(validate_lease_contract(&contract).is_ok());
    }

    #[test]
    fn test_security_deposit_max_3_months_rent() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();

        let contract = LeaseContractBuilder::new()
            .landlord(landlord)
            .tenant(tenant)
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .security_deposit(Capital::from_euros(2_400)) // Exactly 3 months
            .build()
            .unwrap();

        assert!(validate_lease_contract(&contract).is_ok());
    }

    #[test]
    fn test_security_deposit_exceeds_3_months_fails() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();

        let contract = LeaseContractBuilder::new()
            .landlord(landlord)
            .tenant(tenant)
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .security_deposit(Capital::from_euros(2_500)) // More than 3 months
            .build()
            .unwrap();

        assert!(validate_lease_contract(&contract).is_err());
    }

    #[test]
    fn test_defect_notification_valid() {
        let defect = DefectInfo {
            description: "Heating broken".to_string(),
            discovered_at: Utc::now(),
            landlord_notified: true,
            notified_at: Some(Utc::now()),
            reduces_usability: true,
            rent_reduction_percentage: 20,
            landlord_liable: true,
        };

        assert!(validate_defect_notification(&defect).is_ok());
    }

    #[test]
    fn test_defect_notification_not_notified_fails() {
        let defect = DefectInfo {
            description: "Heating broken".to_string(),
            discovered_at: Utc::now(),
            landlord_notified: false,
            notified_at: None,
            reduces_usability: true,
            rent_reduction_percentage: 20,
            landlord_liable: true,
        };

        assert!(validate_defect_notification(&defect).is_err());
    }

    #[test]
    fn test_defect_rent_reduction_exceeds_100_fails() {
        let defect = DefectInfo {
            description: "Major damage".to_string(),
            discovered_at: Utc::now(),
            landlord_notified: true,
            notified_at: Some(Utc::now()),
            reduces_usability: true,
            rent_reduction_percentage: 150,
            landlord_liable: true,
        };

        assert!(validate_defect_notification(&defect).is_err());
    }

    #[test]
    fn test_calculate_rent_reduction() {
        let monthly_rent = Capital::from_euros(1_000);
        let reduction = calculate_rent_reduction(monthly_rent, 20);
        assert_eq!(reduction, Capital::from_euros(200));
    }

    #[test]
    fn test_rent_adjustment_agreement_valid() {
        let adjustment = RentAdjustment {
            old_rent: Capital::from_euros(800),
            new_rent: Capital::from_euros(850),
            effective_date: Utc::now(),
            legal_basis: RentAdjustmentBasis::Agreement,
            tenant_consent: true,
        };

        assert!(validate_rent_adjustment(&adjustment, LeaseType::Residential).is_ok());
    }

    #[test]
    fn test_rent_adjustment_comparative_rent_requires_consent() {
        let adjustment = RentAdjustment {
            old_rent: Capital::from_euros(800),
            new_rent: Capital::from_euros(850),
            effective_date: Utc::now(),
            legal_basis: RentAdjustmentBasis::ComparativeRent,
            tenant_consent: false,
        };

        assert!(validate_rent_adjustment(&adjustment, LeaseType::Residential).is_err());
    }

    #[test]
    fn test_rent_adjustment_comparative_rent_only_residential() {
        let adjustment = RentAdjustment {
            old_rent: Capital::from_euros(800),
            new_rent: Capital::from_euros(850),
            effective_date: Utc::now(),
            legal_basis: RentAdjustmentBasis::ComparativeRent,
            tenant_consent: true,
        };

        assert!(validate_rent_adjustment(&adjustment, LeaseType::Commercial).is_err());
    }

    #[test]
    fn test_lease_termination_must_be_in_writing() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();
        let lease = LeaseContractBuilder::new()
            .landlord(landlord.clone())
            .tenant(tenant)
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .build()
            .unwrap();

        let termination = LeaseTermination {
            contract_id: "LEASE-123".to_string(),
            terminating_party: landlord.name,
            termination_type: TerminationType::Ordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(90),
            notice_period_months: 3,
            in_writing: false,
            reason: Some(TerminationReason::LandlordLegitimateInterest),
        };

        assert!(validate_lease_termination(&termination, &lease).is_err());
    }

    #[test]
    fn test_ordinary_termination_residential_valid() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();
        let lease = LeaseContractBuilder::new()
            .landlord(landlord.clone())
            .tenant(tenant.clone())
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .lease_type(LeaseType::Residential)
            .build()
            .unwrap();

        let termination = LeaseTermination {
            contract_id: "LEASE-123".to_string(),
            terminating_party: tenant.name,
            termination_type: TerminationType::Ordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(90),
            notice_period_months: 3,
            in_writing: true,
            reason: Some(TerminationReason::TenantVoluntary),
        };

        assert!(validate_lease_termination(&termination, &lease).is_ok());
    }

    #[test]
    fn test_ordinary_termination_fixed_term_fails() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();
        let lease = LeaseContractBuilder::new()
            .landlord(landlord.clone())
            .tenant(tenant.clone())
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .end_date(Utc::now() + Duration::days(365))
            .build()
            .unwrap();

        let termination = LeaseTermination {
            contract_id: "LEASE-123".to_string(),
            terminating_party: tenant.name,
            termination_type: TerminationType::Ordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(90),
            notice_period_months: 3,
            in_writing: true,
            reason: Some(TerminationReason::TenantVoluntary),
        };

        assert!(validate_lease_termination(&termination, &lease).is_err());
    }

    #[test]
    fn test_ordinary_termination_residential_requires_3_months_notice() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();
        let lease = LeaseContractBuilder::new()
            .landlord(landlord.clone())
            .tenant(tenant.clone())
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .lease_type(LeaseType::Residential)
            .build()
            .unwrap();

        let termination = LeaseTermination {
            contract_id: "LEASE-123".to_string(),
            terminating_party: tenant.name,
            termination_type: TerminationType::Ordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(60),
            notice_period_months: 2, // Too short
            in_writing: true,
            reason: Some(TerminationReason::TenantVoluntary),
        };

        assert!(validate_lease_termination(&termination, &lease).is_err());
    }

    #[test]
    fn test_extraordinary_termination_requires_reason() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();
        let lease = LeaseContractBuilder::new()
            .landlord(landlord.clone())
            .tenant(tenant)
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .build()
            .unwrap();

        let termination = LeaseTermination {
            contract_id: "LEASE-123".to_string(),
            terminating_party: landlord.name,
            termination_type: TerminationType::Extraordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now(),
            notice_period_months: 0,
            in_writing: true,
            reason: None,
        };

        assert!(validate_lease_termination(&termination, &lease).is_err());
    }

    #[test]
    fn test_extraordinary_termination_with_reason_valid() {
        let landlord = create_test_landlord();
        let tenant = create_test_tenant();
        let lease = LeaseContractBuilder::new()
            .landlord(landlord.clone())
            .tenant(tenant)
            .leased_property("Apartment".to_string())
            .monthly_rent(Capital::from_euros(800))
            .build()
            .unwrap();

        let termination = LeaseTermination {
            contract_id: "LEASE-123".to_string(),
            terminating_party: landlord.name,
            termination_type: TerminationType::Extraordinary,
            notice_date: Utc::now(),
            effective_date: Utc::now(),
            notice_period_months: 0,
            in_writing: true,
            reason: Some(TerminationReason::RentArrears),
        };

        assert!(validate_lease_termination(&termination, &lease).is_ok());
    }
}
