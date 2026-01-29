//! Work Contract (Werkvertrag) - §§631-651 BGB
//!
//! Type-safe implementation of German work contract law under the BGB.
//!
//! # Legal Context
//!
//! A work contract (Werkvertrag) is a contract where the contractor (Unternehmer)
//! owes the production of a promised result (work), and the client (Besteller)
//! owes remuneration (§631 BGB).
//!
//! ## Key Distinction: Werkvertrag vs. Dienstvertrag
//!
//! - **Werkvertrag (§§631-651)**: Obligation to produce a result (ends obligation)
//!   - Focus: Specific work result (Werk)
//!   - Success required: Contractor must achieve agreed result
//!   - Examples: Construction, repair, software development, manufacturing
//!   - Acceptance (Abnahme) required
//!   - Warranty for defects applies
//!
//! - **Dienstvertrag (§§611-630)**: Obligation to perform services (means obligation)
//!   - Focus: Effort and time
//!   - Success not guaranteed
//!
//! ## Core Provisions
//!
//! ### §631-633 BGB - Main Obligations
//! - **Contractor**: Produce promised work free of defects
//! - **Client**: Accept work and pay remuneration
//!
//! ### §634-639 BGB - Defect Rights (Mängelrechte)
//! When work has defects, client can demand:
//! 1. Supplementary performance (Nacherfüllung §635 BGB) - repair or new production
//! 2. Withdraw from contract (Rücktritt §636, §323 BGB)
//! 3. Reduce remuneration (Minderung §638 BGB)
//! 4. Damages (Schadensersatz §636, §280 BGB)
//!
//! ### §640-641 BGB - Acceptance (Abnahme)
//! - Acceptance required for remuneration claim
//! - Implied acceptance after reasonable time (§640 Abs. 2 BGB)
//! - Acceptance cannot be refused for minor defects (unwesentliche Mängel)
//!
//! ### §647-648a BGB - Entrepreneur's Lien (Unternehmerpfandrecht)
//! - Security interest in produced work and materials
//! - For outstanding remuneration claims
//!
//! ### §650a-650v BGB - Construction Contracts (Bauverträge)
//! - Special rules for building construction
//! - Enhanced client rights
//! - Progress payments (Abschlagszahlungen §632a BGB)

#[cfg(test)]
use chrono::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bgb::schuldrecht::error::{Result, SchuldrechtError};
use crate::bgb::schuldrecht::types::{Contract, ContractTerms, Party};
use crate::gmbhg::Capital;

/// Work contract type (Werkvertrag)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkContract {
    /// Base contract information
    pub base_contract: Contract,
    /// Contractor (Unternehmer/Werkunternehmer)
    pub contractor: Party,
    /// Client (Besteller)
    pub client: Party,
    /// Description of work to be produced (Werkleistung)
    pub work_description: String,
    /// Agreed remuneration (Vergütung)
    pub remuneration: Capital,
    /// Completion deadline
    pub completion_deadline: Option<DateTime<Utc>>,
    /// Whether this is a construction contract (Bauvertrag §650a BGB)
    pub is_construction_contract: bool,
    /// Contractor's obligations
    pub contractor_obligations: ContractorObligations,
    /// Client's obligations
    pub client_obligations: ClientObligations,
    /// Acceptance information
    pub acceptance: Option<AcceptanceInfo>,
    /// Defect information
    pub defect_info: Option<WorkDefectInfo>,
    /// Entrepreneur's lien information
    pub lien_info: Option<EntrepreneursLien>,
}

/// Contractor's main obligations per §631 BGB
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractorObligations {
    /// Produce the promised work (Werk herstellen)
    pub produce_work: bool,
    /// Work must be free of defects (mangelfrei)
    pub work_free_of_defects: bool,
    /// Work completed on time
    pub completed_on_time: bool,
    /// Work meets agreed specifications
    pub meets_specifications: bool,
}

/// Client's main obligations per §631 BGB
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientObligations {
    /// Accept the work (Abnahme §640 BGB)
    pub accept_work: bool,
    /// Pay agreed remuneration (Vergütung zahlen)
    pub pay_remuneration: bool,
    /// Payment timely (after acceptance)
    pub payment_timely: bool,
    /// Cooperate as necessary (Mitwirkungspflicht)
    pub cooperate: bool,
}

/// Acceptance information (Abnahme)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptanceInfo {
    /// Whether work has been accepted
    pub accepted: bool,
    /// Type of acceptance
    pub acceptance_type: AcceptanceType,
    /// When work was accepted
    pub accepted_at: Option<DateTime<Utc>>,
    /// Whether acceptance was refused
    pub refused: bool,
    /// Reason for refusal (if refused)
    pub refusal_reason: Option<String>,
    /// Whether minor defects noted at acceptance
    pub minor_defects_noted: bool,
}

/// Type of acceptance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcceptanceType {
    /// Express acceptance (ausdrückliche Abnahme)
    Express,
    /// Implied acceptance (konkludente Abnahme §640 Abs. 1 S. 2 BGB)
    Implied,
    /// Deemed acceptance by lapse of time (§640 Abs. 2 BGB)
    DeemedByLapse,
}

/// Work defect information (Werkmangel)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkDefectInfo {
    /// Description of defect
    pub description: String,
    /// Type of defect
    pub defect_type: WorkDefectType,
    /// When defect was discovered
    pub discovered_at: DateTime<Utc>,
    /// Whether defect is material (wesentlich) or minor (unwesentlich)
    pub is_material: bool,
    /// Client's chosen remedy
    pub chosen_remedy: Option<WorkDefectRemedy>,
    /// Whether supplementary performance attempted
    pub supplementary_performance_attempted: bool,
    /// Whether supplementary performance failed
    pub supplementary_performance_failed: bool,
    /// Defect warranty period (Verjährungsfrist §634a BGB)
    pub warranty_period_months: u32,
}

/// Type of work defect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkDefectType {
    /// Physical defect (Sachmangel §633 Abs. 2 BGB)
    /// Work deviates from agreed quality or lacks fitness for contractual use
    Physical,
    /// Quantity defect (Mengenmangel)
    /// Work quantity less than agreed
    Quantity,
    /// Legal defect (Rechtsmangel §633 Abs. 3 BGB)
    /// Third-party rights burden the work
    Legal,
}

/// Client's remedies for defects per §634 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkDefectRemedy {
    /// Supplementary performance (Nacherfüllung §635 BGB)
    /// - Repair (Nachbesserung) or
    /// - New production (Neuherstellung)
    SupplementaryPerformance,

    /// Remedy defect yourself (Selbstvornahme §637 BGB)
    /// Advance expenses from contractor
    SelfRemedy,

    /// Reduce remuneration (Minderung §638 BGB)
    PriceReduction,

    /// Withdraw from contract (Rücktritt §636, §323 BGB)
    Withdrawal,

    /// Damages (Schadensersatz §636, §280, §281 BGB)
    Damages,

    /// Damages in lieu of performance (Schadensersatz statt der Leistung)
    DamagesInLieu,
}

/// Entrepreneur's lien (Unternehmerpfandrecht §647-648a BGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntrepreneursLien {
    /// Contractor holding lien
    pub contractor: Party,
    /// Description of work/materials subject to lien
    pub subject_description: String,
    /// Amount of outstanding remuneration
    pub outstanding_amount: Capital,
    /// Whether lien is on movable property (§647 BGB)
    pub is_movable: bool,
    /// Whether lien is on land (Bauhandwerkersicherung §648a BGB)
    pub is_land: bool,
    /// When lien was established
    pub established_at: DateTime<Utc>,
}

/// Construction contract special provisions (Bauvertrag §650a BGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstructionContractProvisions {
    /// Type of construction work
    pub construction_type: ConstructionType,
    /// Whether detailed construction description provided (§650b BGB)
    pub detailed_description_provided: bool,
    /// Progress payments agreed (Abschlagszahlungen §632a BGB)
    pub progress_payments: Vec<ProgressPayment>,
    /// Total contract sum
    pub total_contract_sum: Capital,
    /// Whether consumer construction contract (§650i BGB)
    pub is_consumer_contract: bool,
}

/// Type of construction work
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstructionType {
    /// Building construction (Hochbau)
    BuildingConstruction,
    /// Civil engineering (Tiefbau)
    CivilEngineering,
    /// Renovation/modernization (Sanierung/Modernisierung)
    Renovation,
    /// Other construction work
    Other,
}

/// Progress payment (Abschlagszahlung §632a BGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressPayment {
    /// Payment stage description
    pub stage_description: String,
    /// Payment amount
    pub amount: Capital,
    /// Due date
    pub due_date: DateTime<Utc>,
    /// Whether paid
    pub paid: bool,
    /// When paid (if paid)
    pub paid_at: Option<DateTime<Utc>>,
}

/// Builder for work contracts
#[derive(Debug, Clone, Default)]
pub struct WorkContractBuilder {
    contractor: Option<Party>,
    client: Option<Party>,
    work_description: Option<String>,
    remuneration: Option<Capital>,
    completion_deadline: Option<DateTime<Utc>>,
    is_construction_contract: bool,
    warranty_period_months: Option<u32>,
}

impl WorkContractBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the contractor
    pub fn contractor(mut self, contractor: Party) -> Self {
        self.contractor = Some(contractor);
        self
    }

    /// Set the client
    pub fn client(mut self, client: Party) -> Self {
        self.client = Some(client);
        self
    }

    /// Set work description
    pub fn work_description(mut self, description: String) -> Self {
        self.work_description = Some(description);
        self
    }

    /// Set remuneration
    pub fn remuneration(mut self, amount: Capital) -> Self {
        self.remuneration = Some(amount);
        self
    }

    /// Set completion deadline
    pub fn completion_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.completion_deadline = Some(deadline);
        self
    }

    /// Mark as construction contract (§650a BGB)
    pub fn construction_contract(mut self, is_construction: bool) -> Self {
        self.is_construction_contract = is_construction;
        self
    }

    /// Set warranty period in months (default: 24 months per §634a BGB)
    pub fn warranty_period_months(mut self, months: u32) -> Self {
        self.warranty_period_months = Some(months);
        self
    }

    /// Build the work contract
    pub fn build(self) -> Result<WorkContract> {
        let contractor =
            self.contractor
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Contractor".to_string()],
                })?;

        let client = self
            .client
            .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                missing_terms: vec!["Client".to_string()],
            })?;

        let work_description =
            self.work_description
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Work description".to_string()],
                })?;

        let remuneration =
            self.remuneration
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Remuneration".to_string()],
                })?;

        let contract = Contract {
            contract_id: format!("WORK-{}", Utc::now().timestamp()),
            parties: vec![contractor.clone(), client.clone()],
            terms: ContractTerms {
                subject_matter: work_description.clone(),
                consideration: Some(remuneration),
                essential_terms: vec![
                    format!("Contractor: {}", contractor.name),
                    format!("Client: {}", client.name),
                    format!("Work: {}", work_description),
                    format!("Remuneration: € {:.2}", remuneration.to_euros()),
                ],
                additional_terms: vec![],
                includes_gtc: false,
            },
            concluded_at: Utc::now(),
            status: crate::bgb::schuldrecht::types::ContractStatus::Concluded,
            contract_type: crate::bgb::schuldrecht::types::ContractType::Work,
            obligations: vec![],
            in_writing: false,
        };

        Ok(WorkContract {
            base_contract: contract,
            contractor,
            client,
            work_description,
            remuneration,
            completion_deadline: self.completion_deadline,
            is_construction_contract: self.is_construction_contract,
            contractor_obligations: ContractorObligations {
                produce_work: false,
                work_free_of_defects: true,
                completed_on_time: false,
                meets_specifications: true,
            },
            client_obligations: ClientObligations {
                accept_work: false,
                pay_remuneration: false,
                payment_timely: false,
                cooperate: true,
            },
            acceptance: None,
            defect_info: None,
            lien_info: None,
        })
    }
}

/// Validate a work contract per §631 BGB
pub fn validate_work_contract(contract: &WorkContract) -> Result<()> {
    // Validate contractor capacity
    crate::bgb::schuldrecht::validator::validate_party_capacity(&contract.contractor)?;

    // Validate client capacity
    crate::bgb::schuldrecht::validator::validate_party_capacity(&contract.client)?;

    // Validate work description
    if contract.work_description.is_empty() {
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: vec!["Work description".to_string()],
        });
    }

    // Validate remuneration
    if contract.remuneration.amount_cents == 0 {
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: vec!["Valid remuneration".to_string()],
        });
    }

    Ok(())
}

/// Validate acceptance per §640 BGB
pub fn validate_acceptance(acceptance: &AcceptanceInfo) -> Result<()> {
    if acceptance.refused {
        // Refusal must have valid reason
        if acceptance.refusal_reason.is_none() {
            return Err(SchuldrechtError::InvalidContractTerms {
                reason: "Acceptance refusal must have valid reason (§640 BGB)".to_string(),
            });
        }
    }

    if acceptance.accepted {
        // Accepted work must have timestamp
        if acceptance.accepted_at.is_none() {
            return Err(SchuldrechtError::InvalidContractTerms {
                reason: "Acceptance timestamp required".to_string(),
            });
        }
    }

    Ok(())
}

/// Check if acceptance can be refused for given defect (§640 Abs. 1 S. 2 BGB)
pub fn can_refuse_acceptance(defect: &WorkDefectInfo) -> bool {
    // Acceptance cannot be refused for minor (unwesentlich) defects
    defect.is_material
}

/// Validate defect remedy choice per §634 BGB
pub fn validate_defect_remedy(remedy: WorkDefectRemedy, defect: &WorkDefectInfo) -> Result<()> {
    match remedy {
        WorkDefectRemedy::SupplementaryPerformance => {
            // Always available as first remedy (§635 BGB)
            Ok(())
        }
        WorkDefectRemedy::SelfRemedy => {
            // Self-remedy after failed supplementary performance or unreasonable delay (§637 BGB)
            if !defect.supplementary_performance_failed {
                return Err(SchuldrechtError::InvalidContractTerms {
                    reason:
                        "Self-remedy requires failed supplementary performance first (§637 BGB)"
                            .to_string(),
                });
            }
            Ok(())
        }
        WorkDefectRemedy::PriceReduction | WorkDefectRemedy::Withdrawal => {
            // Requires failed supplementary performance (§§636, 323 BGB)
            if !defect.supplementary_performance_attempted
                || !defect.supplementary_performance_failed
            {
                return Err(SchuldrechtError::InvalidContractTerms {
                    reason: "Price reduction/withdrawal requires failed supplementary performance (§§636, 323 BGB)".to_string(),
                });
            }
            Ok(())
        }
        WorkDefectRemedy::Damages | WorkDefectRemedy::DamagesInLieu => {
            // Damages available if contractor at fault (§§636, 280, 281 BGB)
            Ok(())
        }
    }
}

/// Validate defect warranty period per §634a BGB
pub fn validate_defect_warranty_period(
    defect: &WorkDefectInfo,
    contract_concluded_at: DateTime<Utc>,
) -> Result<()> {
    // Check if defect discovered within warranty period
    let months_elapsed = (defect.discovered_at - contract_concluded_at).num_days() / 30;

    if months_elapsed > i64::from(defect.warranty_period_months) {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: format!(
                "Defect discovered after warranty period expired (§634a BGB): {} months > {} months",
                months_elapsed, defect.warranty_period_months
            ),
        });
    }

    Ok(())
}

/// Validate entrepreneur's lien per §§647-648a BGB
pub fn validate_entrepreneurs_lien(lien: &EntrepreneursLien) -> Result<()> {
    // Outstanding amount must be positive
    if lien.outstanding_amount.amount_cents == 0 {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Entrepreneur's lien requires positive outstanding amount".to_string(),
        });
    }

    // Subject description must not be empty
    if lien.subject_description.is_empty() {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Entrepreneur's lien requires subject description".to_string(),
        });
    }

    // Either movable or land lien, not both
    if lien.is_movable && lien.is_land {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Lien cannot be both movable and land lien".to_string(),
        });
    }

    if !lien.is_movable && !lien.is_land {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Lien must be either movable (§647 BGB) or land lien (§648a BGB)".to_string(),
        });
    }

    Ok(())
}

/// Calculate defect warranty period per §634a BGB
pub fn calculate_warranty_period(is_construction: bool, defect_type: WorkDefectType) -> u32 {
    match (is_construction, defect_type) {
        // Building construction: 5 years for physical defects (§634a Abs. 1 Nr. 2 BGB)
        (true, WorkDefectType::Physical) => 60,
        // Other work: 2 years (§634a Abs. 1 Nr. 3 BGB)
        (false, WorkDefectType::Physical) => 24,
        // Legal defects: Same as physical
        (_, WorkDefectType::Legal) => 24,
        // Quantity defects: Same as physical
        (_, WorkDefectType::Quantity) => 24,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bgb::schuldrecht::types::{LegalCapacity, PartyType};

    fn create_test_contractor() -> Party {
        Party {
            name: "Build Corp GmbH".to_string(),
            address: "Hamburg".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::LegalEntity,
        }
    }

    fn create_test_client() -> Party {
        Party {
            name: "Max Mustermann".to_string(),
            address: "Berlin".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::NaturalPerson,
        }
    }

    #[test]
    fn test_work_contract_builder_valid() {
        let contractor = create_test_contractor();
        let client = create_test_client();

        let contract = WorkContractBuilder::new()
            .contractor(contractor)
            .client(client)
            .work_description("Build garage, 30 sqm".to_string())
            .remuneration(Capital::from_euros(20_000))
            .completion_deadline(Utc::now() + Duration::days(90))
            .build();

        assert!(contract.is_ok());
        let contract = contract.unwrap();
        assert_eq!(contract.remuneration, Capital::from_euros(20_000));
        assert_eq!(contract.work_description, "Build garage, 30 sqm");
    }

    #[test]
    fn test_work_contract_builder_missing_contractor() {
        let client = create_test_client();

        let result = WorkContractBuilder::new()
            .client(client)
            .work_description("Build garage".to_string())
            .remuneration(Capital::from_euros(20_000))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_work_contract_builder_missing_work_description() {
        let contractor = create_test_contractor();
        let client = create_test_client();

        let result = WorkContractBuilder::new()
            .contractor(contractor)
            .client(client)
            .remuneration(Capital::from_euros(20_000))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_work_contract_valid() {
        let contractor = create_test_contractor();
        let client = create_test_client();

        let contract = WorkContractBuilder::new()
            .contractor(contractor)
            .client(client)
            .work_description("Build garage".to_string())
            .remuneration(Capital::from_euros(20_000))
            .build()
            .unwrap();

        assert!(validate_work_contract(&contract).is_ok());
    }

    #[test]
    fn test_acceptance_express_valid() {
        let acceptance = AcceptanceInfo {
            accepted: true,
            acceptance_type: AcceptanceType::Express,
            accepted_at: Some(Utc::now()),
            refused: false,
            refusal_reason: None,
            minor_defects_noted: false,
        };

        assert!(validate_acceptance(&acceptance).is_ok());
    }

    #[test]
    fn test_acceptance_refused_requires_reason() {
        let acceptance = AcceptanceInfo {
            accepted: false,
            acceptance_type: AcceptanceType::Express,
            accepted_at: None,
            refused: true,
            refusal_reason: None,
            minor_defects_noted: false,
        };

        assert!(validate_acceptance(&acceptance).is_err());
    }

    #[test]
    fn test_acceptance_refused_with_reason_valid() {
        let acceptance = AcceptanceInfo {
            accepted: false,
            acceptance_type: AcceptanceType::Express,
            accepted_at: None,
            refused: true,
            refusal_reason: Some("Material defect in foundation".to_string()),
            minor_defects_noted: false,
        };

        assert!(validate_acceptance(&acceptance).is_ok());
    }

    #[test]
    fn test_can_refuse_acceptance_material_defect() {
        let defect = WorkDefectInfo {
            description: "Foundation cracked".to_string(),
            defect_type: WorkDefectType::Physical,
            discovered_at: Utc::now(),
            is_material: true,
            chosen_remedy: None,
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
            warranty_period_months: 24,
        };

        assert!(can_refuse_acceptance(&defect));
    }

    #[test]
    fn test_cannot_refuse_acceptance_minor_defect() {
        let defect = WorkDefectInfo {
            description: "Small paint scratch".to_string(),
            defect_type: WorkDefectType::Physical,
            discovered_at: Utc::now(),
            is_material: false,
            chosen_remedy: None,
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
            warranty_period_months: 24,
        };

        assert!(!can_refuse_acceptance(&defect));
    }

    #[test]
    fn test_supplementary_performance_always_valid() {
        let defect = WorkDefectInfo {
            description: "Defect".to_string(),
            defect_type: WorkDefectType::Physical,
            discovered_at: Utc::now(),
            is_material: true,
            chosen_remedy: None,
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
            warranty_period_months: 24,
        };

        assert!(
            validate_defect_remedy(WorkDefectRemedy::SupplementaryPerformance, &defect).is_ok()
        );
    }

    #[test]
    fn test_price_reduction_requires_failed_supplementary() {
        let defect = WorkDefectInfo {
            description: "Defect".to_string(),
            defect_type: WorkDefectType::Physical,
            discovered_at: Utc::now(),
            is_material: true,
            chosen_remedy: None,
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
            warranty_period_months: 24,
        };

        assert!(validate_defect_remedy(WorkDefectRemedy::PriceReduction, &defect).is_err());
    }

    #[test]
    fn test_price_reduction_after_failed_supplementary_valid() {
        let defect = WorkDefectInfo {
            description: "Defect".to_string(),
            defect_type: WorkDefectType::Physical,
            discovered_at: Utc::now(),
            is_material: true,
            chosen_remedy: None,
            supplementary_performance_attempted: true,
            supplementary_performance_failed: true,
            warranty_period_months: 24,
        };

        assert!(validate_defect_remedy(WorkDefectRemedy::PriceReduction, &defect).is_ok());
    }

    #[test]
    fn test_defect_warranty_period_within_valid() {
        let defect = WorkDefectInfo {
            description: "Defect".to_string(),
            defect_type: WorkDefectType::Physical,
            discovered_at: Utc::now(),
            is_material: true,
            chosen_remedy: None,
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
            warranty_period_months: 24,
        };

        let contract_concluded = Utc::now() - Duration::days(180); // 6 months ago
        assert!(validate_defect_warranty_period(&defect, contract_concluded).is_ok());
    }

    #[test]
    fn test_defect_warranty_period_expired_fails() {
        let defect = WorkDefectInfo {
            description: "Defect".to_string(),
            defect_type: WorkDefectType::Physical,
            discovered_at: Utc::now(),
            is_material: true,
            chosen_remedy: None,
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
            warranty_period_months: 24,
        };

        let contract_concluded = Utc::now() - Duration::days(900); // 30 months ago
        assert!(validate_defect_warranty_period(&defect, contract_concluded).is_err());
    }

    #[test]
    fn test_entrepreneurs_lien_valid() {
        let contractor = create_test_contractor();

        let lien = EntrepreneursLien {
            contractor,
            subject_description: "Custom-built furniture".to_string(),
            outstanding_amount: Capital::from_euros(5_000),
            is_movable: true,
            is_land: false,
            established_at: Utc::now(),
        };

        assert!(validate_entrepreneurs_lien(&lien).is_ok());
    }

    #[test]
    fn test_entrepreneurs_lien_requires_positive_amount() {
        let contractor = create_test_contractor();

        let lien = EntrepreneursLien {
            contractor,
            subject_description: "Furniture".to_string(),
            outstanding_amount: Capital::from_euros(0),
            is_movable: true,
            is_land: false,
            established_at: Utc::now(),
        };

        assert!(validate_entrepreneurs_lien(&lien).is_err());
    }

    #[test]
    fn test_entrepreneurs_lien_cannot_be_both_movable_and_land() {
        let contractor = create_test_contractor();

        let lien = EntrepreneursLien {
            contractor,
            subject_description: "Building".to_string(),
            outstanding_amount: Capital::from_euros(5_000),
            is_movable: true,
            is_land: true,
            established_at: Utc::now(),
        };

        assert!(validate_entrepreneurs_lien(&lien).is_err());
    }

    #[test]
    fn test_calculate_warranty_period_construction_5_years() {
        let period = calculate_warranty_period(true, WorkDefectType::Physical);
        assert_eq!(period, 60); // 5 years = 60 months
    }

    #[test]
    fn test_calculate_warranty_period_non_construction_2_years() {
        let period = calculate_warranty_period(false, WorkDefectType::Physical);
        assert_eq!(period, 24); // 2 years = 24 months
    }
}
