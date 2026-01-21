//! Building and Construction Contracts
//!
//! Analysis of building and construction contracts under Australian law:
//! - Home Building Act provisions (NSW and equivalents)
//! - Statutory warranties
//! - Progress payments and payment schedules
//! - Defects liability periods
//! - Practical completion
//! - Security of payment legislation
//! - QBCC and other state regulators
//!
//! Key legislation:
//! - Home Building Act 1989 (NSW)
//! - Building and Construction Industry Security of Payment Act 2002 (NSW)
//! - Domestic Building Contracts Act 1995 (Vic)
//! - Queensland Building and Construction Commission Act 1991 (Qld)
//!
//! Key cases:
//! - Bellgrove v Eldridge (1954) - Cost of rectification
//! - Ruxley Electronics v Forsyth (1996) - Diminution in value
//! - Multiplex v Honeywell (2007) - Practical completion

use serde::{Deserialize, Serialize};

use super::error::ContractResult;

// ============================================================================
// Building Contract Types
// ============================================================================

/// Type of building contract
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingContractType {
    /// Fixed price (lump sum)
    FixedPrice,
    /// Cost plus
    CostPlus,
    /// Schedule of rates
    ScheduleOfRates,
    /// Design and construct
    DesignAndConstruct,
    /// Construction management
    ConstructionManagement,
    /// Domestic building contract
    Domestic,
    /// Commercial building contract
    Commercial,
}

/// Progress payment basis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressPaymentBasis {
    /// Time-based (monthly)
    TimeBased,
    /// Milestone-based
    MilestoneBased,
    /// Percentage completion
    PercentageCompletion,
    /// Value of work completed
    ValueOfWork,
    /// Stage payments
    StagePayments,
}

/// Statutory warranty type (Home Building Act)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatutoryWarranty {
    /// Work performed in proper and workmanlike manner
    ProperWorkmanship,
    /// Work performed in accordance with plans and specifications
    AccordanceWithPlans,
    /// All materials suitable and of good quality
    SuitableMaterials,
    /// Work will be done with due diligence
    DueDiligence,
    /// Work will result in dwelling fit for habitation
    FitForHabitation,
    /// Compliance with all laws and regulations
    LegalCompliance,
    /// Proper supervision of work
    ProperSupervision,
}

/// Defect classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefectClassification {
    /// Major structural defect
    MajorStructural,
    /// Major non-structural defect
    MajorNonStructural,
    /// Minor defect
    Minor,
    /// Patent defect (obvious)
    Patent,
    /// Latent defect (hidden)
    Latent,
}

/// Payment claim status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentClaimStatus {
    /// Claim submitted
    Submitted,
    /// Payment schedule issued
    ScheduleIssued,
    /// Payment made
    Paid,
    /// Disputed
    Disputed,
    /// Adjudication commenced
    AdjudicationCommenced,
    /// Adjudication determined
    AdjudicationDetermined,
}

/// Practical completion status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PracticalCompletionStatus {
    /// Not reached
    NotReached,
    /// Substantially complete
    SubstantiallyComplete,
    /// Practically complete
    PracticallyComplete,
    /// Certificate issued
    CertificateIssued,
    /// Disputed
    Disputed,
}

// ============================================================================
// Building Contract Analysis
// ============================================================================

/// Building contract analyzer
pub struct BuildingContractAnalyzer;

impl BuildingContractAnalyzer {
    /// Analyze building contract validity and compliance
    pub fn analyze(facts: &BuildingContractFacts) -> ContractResult<BuildingContractResult> {
        let valid_contract = Self::check_contract_validity(facts)?;
        let statutory_warranties = Self::determine_statutory_warranties(facts);
        let compliance_issues = Self::check_compliance(facts);

        let reasoning = Self::build_reasoning(facts, valid_contract, &compliance_issues);

        Ok(BuildingContractResult {
            contract_type: facts.contract_type.clone(),
            valid_contract,
            statutory_warranties,
            compliance_issues,
            requires_insurance: facts.contract_value_over_threshold,
            requires_licence: true,
            reasoning,
        })
    }

    /// Check contract validity under Home Building Act
    fn check_contract_validity(facts: &BuildingContractFacts) -> ContractResult<bool> {
        // Home Building Act requirements:
        // - Must be in writing if over threshold
        // - Must contain prescribed terms
        // - Must be signed by both parties
        // - Must specify completion date
        // - Must specify contract price or how it will be calculated

        if facts.domestic_building && facts.contract_value_over_threshold {
            if !facts.in_writing {
                return Ok(false);
            }

            if !facts.contains_required_terms {
                return Ok(false);
            }

            if !facts.signed_by_both_parties {
                return Ok(false);
            }

            if !facts.completion_date_specified {
                return Ok(false);
            }

            if !facts.price_or_method_specified {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Determine applicable statutory warranties
    fn determine_statutory_warranties(facts: &BuildingContractFacts) -> Vec<StatutoryWarranty> {
        let mut warranties = Vec::new();

        if facts.domestic_building || facts.residential_construction {
            // All statutory warranties apply
            warranties.push(StatutoryWarranty::ProperWorkmanship);
            warranties.push(StatutoryWarranty::AccordanceWithPlans);
            warranties.push(StatutoryWarranty::SuitableMaterials);
            warranties.push(StatutoryWarranty::DueDiligence);
            warranties.push(StatutoryWarranty::FitForHabitation);
            warranties.push(StatutoryWarranty::LegalCompliance);

            if facts.builder_supervises_work {
                warranties.push(StatutoryWarranty::ProperSupervision);
            }
        }

        warranties
    }

    /// Check compliance issues
    fn check_compliance(facts: &BuildingContractFacts) -> Vec<ComplianceIssue> {
        let mut issues = Vec::new();

        if facts.domestic_building && !facts.builder_licensed {
            issues.push(ComplianceIssue::NoBuilderLicence);
        }

        if facts.contract_value_over_threshold && !facts.insurance_in_place {
            issues.push(ComplianceIssue::NoInsurance);
        }

        if !facts.in_writing && facts.contract_value_over_threshold {
            issues.push(ComplianceIssue::NotInWriting);
        }

        if facts.deposit_exceeds_maximum {
            issues.push(ComplianceIssue::ExcessiveDeposit);
        }

        if facts.progress_payments_not_compliant {
            issues.push(ComplianceIssue::NonCompliantProgressPayments);
        }

        if !facts.contains_cooling_off_notice && facts.domestic_building {
            issues.push(ComplianceIssue::NoCoolingOffNotice);
        }

        issues
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &BuildingContractFacts,
        valid: bool,
        issues: &[ComplianceIssue],
    ) -> String {
        let mut parts = Vec::new();

        if facts.domestic_building {
            parts.push("Domestic building contract analysis (Home Building Act)".to_string());
        } else {
            parts.push("Commercial building contract analysis".to_string());
        }

        if valid {
            parts.push("Contract meets statutory requirements".to_string());
        } else {
            parts.push("Contract does not meet statutory requirements".to_string());
        }

        if !issues.is_empty() {
            parts.push(format!("Compliance issues identified: {}", issues.len()));

            for issue in issues {
                match issue {
                    ComplianceIssue::NoBuilderLicence => {
                        parts.push("Builder not appropriately licensed".to_string());
                    }
                    ComplianceIssue::NoInsurance => {
                        parts.push("Required home warranty insurance not in place".to_string());
                    }
                    ComplianceIssue::NotInWriting => {
                        parts.push("Contract not in writing as required".to_string());
                    }
                    ComplianceIssue::ExcessiveDeposit => {
                        parts.push("Deposit exceeds statutory maximum".to_string());
                    }
                    _ => {}
                }
            }
        }

        parts.join(". ")
    }
}

/// Compliance issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceIssue {
    /// No builder licence
    NoBuilderLicence,
    /// No insurance
    NoInsurance,
    /// Not in writing
    NotInWriting,
    /// Excessive deposit
    ExcessiveDeposit,
    /// Non-compliant progress payments
    NonCompliantProgressPayments,
    /// No cooling-off notice
    NoCoolingOffNotice,
    /// Missing required terms
    MissingRequiredTerms,
}

/// Facts for building contract analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildingContractFacts {
    /// Contract type
    pub contract_type: Option<BuildingContractType>,
    /// Domestic building
    pub domestic_building: bool,
    /// Residential construction
    pub residential_construction: bool,
    /// Contract value over threshold
    pub contract_value_over_threshold: bool,
    /// In writing
    pub in_writing: bool,
    /// Contains required terms
    pub contains_required_terms: bool,
    /// Signed by both parties
    pub signed_by_both_parties: bool,
    /// Completion date specified
    pub completion_date_specified: bool,
    /// Price or method specified
    pub price_or_method_specified: bool,
    /// Builder licensed
    pub builder_licensed: bool,
    /// Insurance in place
    pub insurance_in_place: bool,
    /// Builder supervises work
    pub builder_supervises_work: bool,
    /// Deposit exceeds maximum
    pub deposit_exceeds_maximum: bool,
    /// Progress payments not compliant
    pub progress_payments_not_compliant: bool,
    /// Contains cooling-off notice
    pub contains_cooling_off_notice: bool,
}

/// Result of building contract analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingContractResult {
    /// Contract type
    pub contract_type: Option<BuildingContractType>,
    /// Valid contract
    pub valid_contract: bool,
    /// Statutory warranties
    pub statutory_warranties: Vec<StatutoryWarranty>,
    /// Compliance issues
    pub compliance_issues: Vec<ComplianceIssue>,
    /// Requires insurance
    pub requires_insurance: bool,
    /// Requires licence
    pub requires_licence: bool,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Statutory Warranties
// ============================================================================

/// Statutory warranty analyzer
pub struct StatutoryWarrantyAnalyzer;

impl StatutoryWarrantyAnalyzer {
    /// Analyze breach of statutory warranty
    pub fn analyze(
        warranty: StatutoryWarranty,
        facts: &WarrantyBreachFacts,
    ) -> WarrantyBreachResult {
        let breach = Self::check_breach(&warranty, facts);
        let major_defect = breach && Self::is_major_defect(facts);
        let within_limitation = Self::check_limitation_period(&warranty, facts);

        let remedies = if breach && within_limitation {
            Self::determine_remedies(major_defect, facts)
        } else {
            Vec::new()
        };

        let reasoning =
            Self::build_reasoning(&warranty, facts, breach, major_defect, within_limitation);

        WarrantyBreachResult {
            warranty,
            breached: breach,
            major_defect,
            within_limitation_period: within_limitation,
            available_remedies: remedies,
            reasoning,
        }
    }

    /// Check if warranty breached
    fn check_breach(warranty: &StatutoryWarranty, facts: &WarrantyBreachFacts) -> bool {
        match warranty {
            StatutoryWarranty::ProperWorkmanship => {
                !facts.work_performed_properly || facts.workmanship_defects
            }
            StatutoryWarranty::AccordanceWithPlans => {
                facts.deviates_from_plans || !facts.complies_with_specifications
            }
            StatutoryWarranty::SuitableMaterials => {
                facts.unsuitable_materials || facts.poor_quality_materials
            }
            StatutoryWarranty::DueDiligence => {
                facts.unreasonable_delays || !facts.work_progressed_diligently
            }
            StatutoryWarranty::FitForHabitation => {
                !facts.fit_for_habitation || facts.not_safe_to_occupy
            }
            StatutoryWarranty::LegalCompliance => {
                !facts.complies_with_building_code
                    || !facts.complies_with_regulations
                    || facts.non_compliant_work
            }
            StatutoryWarranty::ProperSupervision => {
                !facts.properly_supervised || facts.supervision_inadequate
            }
        }
    }

    /// Check if defect is major
    fn is_major_defect(facts: &WarrantyBreachFacts) -> bool {
        facts.structural_defect
            || facts.not_safe_to_occupy
            || facts.uninhabitable
            || facts.rectification_cost_substantial
    }

    /// Check limitation period
    fn check_limitation_period(warranty: &StatutoryWarranty, facts: &WarrantyBreachFacts) -> bool {
        // NSW: 6 years for breach of warranty (7 years for major structural)
        match warranty {
            StatutoryWarranty::ProperWorkmanship
            | StatutoryWarranty::AccordanceWithPlans
            | StatutoryWarranty::SuitableMaterials => {
                if facts.structural_defect {
                    facts.years_since_completion <= 7
                } else {
                    facts.years_since_completion <= 6
                }
            }
            _ => facts.years_since_completion <= 6,
        }
    }

    /// Determine remedies
    fn determine_remedies(major: bool, facts: &WarrantyBreachFacts) -> Vec<BuildingRemedy> {
        let mut remedies = Vec::new();

        if major {
            // Major defect - broader remedies
            remedies.push(BuildingRemedy::RectificationWorks);
            remedies.push(BuildingRemedy::CostOfRectification);

            if facts.uninhabitable {
                remedies.push(BuildingRemedy::AlternativeAccommodation);
            }
        } else {
            // Minor defect
            remedies.push(BuildingRemedy::RectificationWorks);
            remedies.push(BuildingRemedy::DiminutionInValue);
        }

        // Consequential damages
        if facts.consequential_loss {
            remedies.push(BuildingRemedy::ConsequentialDamages);
        }

        remedies
    }

    /// Build reasoning
    fn build_reasoning(
        warranty: &StatutoryWarranty,
        facts: &WarrantyBreachFacts,
        breach: bool,
        major: bool,
        within_limitation: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!("Statutory warranty analysis: {:?}", warranty));

        if !within_limitation {
            parts.push("Claim outside limitation period".to_string());
            return parts.join(". ");
        }

        if breach {
            parts.push("Warranty breached".to_string());

            match warranty {
                StatutoryWarranty::ProperWorkmanship => {
                    if facts.workmanship_defects {
                        parts.push(
                            "Work not performed in proper and workmanlike manner".to_string(),
                        );
                    }
                }
                StatutoryWarranty::AccordanceWithPlans => {
                    if facts.deviates_from_plans {
                        parts.push("Work deviates from approved plans".to_string());
                    }
                }
                StatutoryWarranty::SuitableMaterials => {
                    if facts.poor_quality_materials {
                        parts.push("Materials not of good quality".to_string());
                    }
                }
                StatutoryWarranty::FitForHabitation => {
                    if !facts.fit_for_habitation {
                        parts.push("Dwelling not fit for habitation".to_string());
                    }
                }
                StatutoryWarranty::LegalCompliance => {
                    if !facts.complies_with_building_code {
                        parts.push("Work does not comply with Building Code".to_string());
                    }
                }
                _ => {}
            }

            if major {
                parts.push("Classified as major defect".to_string());
            } else {
                parts.push("Classified as minor defect".to_string());
            }
        } else {
            parts.push("No breach of warranty".to_string());
        }

        parts.join(". ")
    }
}

/// Facts for warranty breach
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WarrantyBreachFacts {
    // Proper workmanship
    /// Work performed properly
    pub work_performed_properly: bool,
    /// Workmanship defects
    pub workmanship_defects: bool,

    // Accordance with plans
    /// Deviates from plans
    pub deviates_from_plans: bool,
    /// Complies with specifications
    pub complies_with_specifications: bool,

    // Suitable materials
    /// Unsuitable materials
    pub unsuitable_materials: bool,
    /// Poor quality materials
    pub poor_quality_materials: bool,

    // Due diligence
    /// Unreasonable delays
    pub unreasonable_delays: bool,
    /// Work progressed diligently
    pub work_progressed_diligently: bool,

    // Fit for habitation
    /// Fit for habitation
    pub fit_for_habitation: bool,
    /// Not safe to occupy
    pub not_safe_to_occupy: bool,
    /// Uninhabitable
    pub uninhabitable: bool,

    // Legal compliance
    /// Complies with building code
    pub complies_with_building_code: bool,
    /// Complies with regulations
    pub complies_with_regulations: bool,
    /// Non-compliant work
    pub non_compliant_work: bool,

    // Supervision
    /// Properly supervised
    pub properly_supervised: bool,
    /// Supervision inadequate
    pub supervision_inadequate: bool,

    // Defect classification
    /// Structural defect
    pub structural_defect: bool,
    /// Rectification cost substantial
    pub rectification_cost_substantial: bool,

    // Limitation
    /// Years since completion
    pub years_since_completion: u32,

    // Damages
    /// Consequential loss
    pub consequential_loss: bool,
}

/// Result of warranty breach analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyBreachResult {
    /// Warranty type
    pub warranty: StatutoryWarranty,
    /// Breached
    pub breached: bool,
    /// Major defect
    pub major_defect: bool,
    /// Within limitation period
    pub within_limitation_period: bool,
    /// Available remedies
    pub available_remedies: Vec<BuildingRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Progress Payments
// ============================================================================

/// Progress payment analyzer
pub struct ProgressPaymentAnalyzer;

impl ProgressPaymentAnalyzer {
    /// Analyze progress payment claim under Security of Payment legislation
    pub fn analyze(facts: &ProgressPaymentFacts) -> ContractResult<ProgressPaymentResult> {
        let valid_claim = Self::check_valid_claim(facts)?;
        let payment_schedule_required = Self::check_schedule_required(facts);
        let adjudication_available = valid_claim && facts.payment_dispute;

        let amount_payable = if valid_claim {
            Self::calculate_payable_amount(facts)
        } else {
            0
        };

        let remedies = if valid_claim && !facts.payment_made {
            Self::determine_payment_remedies(facts)
        } else {
            Vec::new()
        };

        let reasoning = Self::build_reasoning(facts, valid_claim, payment_schedule_required);

        Ok(ProgressPaymentResult {
            valid_payment_claim: valid_claim,
            payment_schedule_required,
            adjudication_available,
            amount_payable,
            available_remedies: remedies,
            reasoning,
        })
    }

    /// Check if payment claim is valid
    fn check_valid_claim(facts: &ProgressPaymentFacts) -> ContractResult<bool> {
        // Security of Payment Act requirements:
        // - Must be in writing
        // - Must identify work/services
        // - Must state claimed amount
        // - Must state reference date
        // - Only one claim per reference period

        if !facts.claim_in_writing {
            return Ok(false);
        }

        if !facts.identifies_work {
            return Ok(false);
        }

        if !facts.states_amount {
            return Ok(false);
        }

        if !facts.states_reference_date {
            return Ok(false);
        }

        if facts.duplicate_claim_for_period {
            return Ok(false);
        }

        Ok(true)
    }

    /// Check if payment schedule required
    fn check_schedule_required(facts: &ProgressPaymentFacts) -> bool {
        // Respondent must issue payment schedule within time
        facts.valid_claim && !facts.payment_schedule_issued_on_time
    }

    /// Calculate payable amount
    fn calculate_payable_amount(facts: &ProgressPaymentFacts) -> u64 {
        if facts.payment_schedule_issued_on_time {
            facts.scheduled_amount
        } else {
            // No payment schedule = claimed amount becomes due
            facts.claimed_amount
        }
    }

    /// Determine payment remedies
    fn determine_payment_remedies(facts: &ProgressPaymentFacts) -> Vec<BuildingRemedy> {
        let mut remedies = Vec::new();

        remedies.push(BuildingRemedy::ProgressPayment);

        if facts.payment_dispute {
            remedies.push(BuildingRemedy::Adjudication);
        }

        if facts.adjudication_determination_made {
            remedies.push(BuildingRemedy::AdjudicatedAmount);
        }

        // Suspension of work available if not paid
        if !facts.payment_made && facts.suspension_notice_given {
            remedies.push(BuildingRemedy::SuspensionOfWork);
        }

        remedies
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &ProgressPaymentFacts,
        valid: bool,
        schedule_required: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push(
            "Security of Payment analysis (Building and Construction Industry SOP Act)".to_string(),
        );

        if valid {
            parts.push("Valid payment claim".to_string());

            if schedule_required {
                parts.push(
                    "Payment schedule not issued within time - claimed amount due".to_string(),
                );
            } else if facts.payment_schedule_issued_on_time {
                parts.push("Payment schedule issued - scheduled amount payable".to_string());
            }

            if facts.payment_dispute {
                parts.push("Payment dispute - adjudication available".to_string());
            }
        } else {
            parts.push("Invalid payment claim".to_string());

            if !facts.claim_in_writing {
                parts.push("Claim not in writing".to_string());
            }
            if facts.duplicate_claim_for_period {
                parts.push("Duplicate claim for reference period".to_string());
            }
        }

        parts.join(". ")
    }
}

/// Facts for progress payment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgressPaymentFacts {
    /// Valid claim
    pub valid_claim: bool,
    /// Claim in writing
    pub claim_in_writing: bool,
    /// Identifies work
    pub identifies_work: bool,
    /// States amount
    pub states_amount: bool,
    /// States reference date
    pub states_reference_date: bool,
    /// Duplicate claim for period
    pub duplicate_claim_for_period: bool,
    /// Claimed amount
    pub claimed_amount: u64,
    /// Payment schedule issued on time
    pub payment_schedule_issued_on_time: bool,
    /// Scheduled amount
    pub scheduled_amount: u64,
    /// Payment made
    pub payment_made: bool,
    /// Payment dispute
    pub payment_dispute: bool,
    /// Adjudication determination made
    pub adjudication_determination_made: bool,
    /// Suspension notice given
    pub suspension_notice_given: bool,
}

/// Result of progress payment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressPaymentResult {
    /// Valid payment claim
    pub valid_payment_claim: bool,
    /// Payment schedule required
    pub payment_schedule_required: bool,
    /// Adjudication available
    pub adjudication_available: bool,
    /// Amount payable
    pub amount_payable: u64,
    /// Available remedies
    pub available_remedies: Vec<BuildingRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Practical Completion
// ============================================================================

/// Practical completion analyzer
pub struct PracticalCompletionAnalyzer;

impl PracticalCompletionAnalyzer {
    /// Analyze practical completion
    pub fn analyze(facts: &PracticalCompletionFacts) -> PracticalCompletionResult {
        let status = Self::determine_status(facts);
        let defects_liability_commenced = matches!(
            status,
            PracticalCompletionStatus::PracticallyComplete
                | PracticalCompletionStatus::CertificateIssued
        );

        let reasoning = Self::build_reasoning(facts, &status);

        PracticalCompletionResult {
            status,
            substantially_complete: facts.substantially_complete,
            minor_defects_only: facts.minor_defects_only,
            habitable: facts.habitable,
            defects_liability_commenced,
            defects_liability_period_months: if defects_liability_commenced {
                Some(facts.defects_liability_period_months)
            } else {
                None
            },
            reasoning,
        }
    }

    /// Determine practical completion status
    fn determine_status(facts: &PracticalCompletionFacts) -> PracticalCompletionStatus {
        if facts.certificate_issued {
            return PracticalCompletionStatus::CertificateIssued;
        }

        if facts.certificate_disputed {
            return PracticalCompletionStatus::Disputed;
        }

        // Multiplex v Honeywell test:
        // - Substantially complete
        // - Minor defects/omissions only
        // - Does not prevent use for intended purpose

        if facts.substantially_complete
            && facts.minor_defects_only
            && facts.habitable
            && !facts.major_works_outstanding
        {
            PracticalCompletionStatus::PracticallyComplete
        } else if facts.substantially_complete {
            PracticalCompletionStatus::SubstantiallyComplete
        } else {
            PracticalCompletionStatus::NotReached
        }
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &PracticalCompletionFacts,
        status: &PracticalCompletionStatus,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Practical completion analysis (Multiplex v Honeywell)".to_string());

        match status {
            PracticalCompletionStatus::CertificateIssued => {
                parts.push("Practical completion certificate issued".to_string());
                parts.push("Defects liability period commenced".to_string());
            }
            PracticalCompletionStatus::PracticallyComplete => {
                parts.push("Practical completion achieved".to_string());

                if facts.substantially_complete {
                    parts.push("Works substantially complete".to_string());
                }
                if facts.minor_defects_only {
                    parts.push("Only minor defects/omissions remain".to_string());
                }
                if facts.habitable {
                    parts.push("Building suitable for intended use".to_string());
                }
            }
            PracticalCompletionStatus::SubstantiallyComplete => {
                parts.push(
                    "Works substantially complete but practical completion not achieved"
                        .to_string(),
                );

                if facts.major_works_outstanding {
                    parts.push("Major works still outstanding".to_string());
                }
                if !facts.habitable {
                    parts.push("Not yet suitable for occupation".to_string());
                }
            }
            PracticalCompletionStatus::NotReached => {
                parts.push("Practical completion not achieved".to_string());
            }
            PracticalCompletionStatus::Disputed => {
                parts.push("Practical completion status disputed".to_string());
            }
        }

        parts.join(". ")
    }
}

/// Facts for practical completion
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PracticalCompletionFacts {
    /// Substantially complete
    pub substantially_complete: bool,
    /// Minor defects only
    pub minor_defects_only: bool,
    /// Habitable
    pub habitable: bool,
    /// Major works outstanding
    pub major_works_outstanding: bool,
    /// Certificate issued
    pub certificate_issued: bool,
    /// Certificate disputed
    pub certificate_disputed: bool,
    /// Defects liability period (months)
    pub defects_liability_period_months: u32,
}

/// Result of practical completion analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticalCompletionResult {
    /// Status
    pub status: PracticalCompletionStatus,
    /// Substantially complete
    pub substantially_complete: bool,
    /// Minor defects only
    pub minor_defects_only: bool,
    /// Habitable
    pub habitable: bool,
    /// Defects liability commenced
    pub defects_liability_commenced: bool,
    /// Defects liability period
    pub defects_liability_period_months: Option<u32>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Defects Liability
// ============================================================================

/// Defects liability analyzer
pub struct DefectsLiabilityAnalyzer;

impl DefectsLiabilityAnalyzer {
    /// Analyze defects liability
    pub fn analyze(facts: &DefectsLiabilityFacts) -> DefectsLiabilityResult {
        let classification = Self::classify_defect(facts);
        let within_liability_period = Self::check_liability_period(facts, &classification);
        let builder_liable = within_liability_period && facts.builder_responsible;

        let remedies = if builder_liable {
            Self::determine_defect_remedies(facts, &classification)
        } else {
            Vec::new()
        };

        let reasoning = Self::build_reasoning(
            facts,
            &classification,
            within_liability_period,
            builder_liable,
        );

        DefectsLiabilityResult {
            defect_classification: classification,
            within_liability_period,
            builder_liable,
            available_remedies: remedies,
            reasoning,
        }
    }

    /// Classify defect
    fn classify_defect(facts: &DefectsLiabilityFacts) -> DefectClassification {
        if facts.structural_defect {
            DefectClassification::MajorStructural
        } else if facts.affects_habitability || facts.significant_cost_to_rectify {
            DefectClassification::MajorNonStructural
        } else if facts.obvious_on_inspection {
            DefectClassification::Patent
        } else if facts.hidden_defect {
            DefectClassification::Latent
        } else {
            DefectClassification::Minor
        }
    }

    /// Check liability period
    fn check_liability_period(
        facts: &DefectsLiabilityFacts,
        classification: &DefectClassification,
    ) -> bool {
        match classification {
            DefectClassification::MajorStructural => {
                // 7 years for major structural in NSW
                facts.years_since_practical_completion <= 7
            }
            DefectClassification::MajorNonStructural => {
                // 6 years
                facts.years_since_practical_completion <= 6
            }
            DefectClassification::Minor | DefectClassification::Patent => {
                // Within defects liability period (usually 12 months)
                facts.months_since_practical_completion <= facts.defects_liability_period_months
            }
            DefectClassification::Latent => {
                // 6 years from discovery
                facts.years_since_discovery <= 6
            }
        }
    }

    /// Determine defect remedies
    fn determine_defect_remedies(
        facts: &DefectsLiabilityFacts,
        classification: &DefectClassification,
    ) -> Vec<BuildingRemedy> {
        let mut remedies = Vec::new();

        match classification {
            DefectClassification::MajorStructural | DefectClassification::MajorNonStructural => {
                // Bellgrove v Eldridge: Cost of rectification if reasonable
                remedies.push(BuildingRemedy::CostOfRectification);
                remedies.push(BuildingRemedy::RectificationWorks);

                if facts.consequential_loss {
                    remedies.push(BuildingRemedy::ConsequentialDamages);
                }
            }
            DefectClassification::Minor
            | DefectClassification::Patent
            | DefectClassification::Latent => {
                remedies.push(BuildingRemedy::RectificationWorks);

                if facts.rectification_unreasonable {
                    // Ruxley v Forsyth: Diminution in value if rectification unreasonable
                    remedies.push(BuildingRemedy::DiminutionInValue);
                }
            }
        }

        remedies
    }

    /// Build reasoning
    fn build_reasoning(
        facts: &DefectsLiabilityFacts,
        classification: &DefectClassification,
        within_period: bool,
        liable: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Defects liability analysis".to_string());
        parts.push(format!("Defect classification: {:?}", classification));

        if !within_period {
            parts.push("Defect discovered outside applicable limitation period".to_string());
            return parts.join(". ");
        }

        parts.push("Within limitation period".to_string());

        if liable {
            parts.push("Builder liable for defect".to_string());

            match classification {
                DefectClassification::MajorStructural => {
                    parts.push("Major structural defect - cost of rectification available (Bellgrove v Eldridge)".to_string());
                }
                DefectClassification::MajorNonStructural => {
                    parts.push("Major non-structural defect - rectification required".to_string());
                }
                DefectClassification::Minor => {
                    if facts.rectification_unreasonable {
                        parts.push(
                            "Rectification unreasonable - diminution in value (Ruxley v Forsyth)"
                                .to_string(),
                        );
                    } else {
                        parts.push("Minor defect - rectification works required".to_string());
                    }
                }
                _ => {}
            }
        } else {
            parts.push("Builder not liable".to_string());

            if !facts.builder_responsible {
                parts.push("Defect not caused by builder".to_string());
            }
        }

        parts.join(". ")
    }
}

/// Facts for defects liability
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefectsLiabilityFacts {
    // Defect characteristics
    /// Structural defect
    pub structural_defect: bool,
    /// Affects habitability
    pub affects_habitability: bool,
    /// Significant cost to rectify
    pub significant_cost_to_rectify: bool,
    /// Obvious on inspection
    pub obvious_on_inspection: bool,
    /// Hidden defect
    pub hidden_defect: bool,

    // Responsibility
    /// Builder responsible
    pub builder_responsible: bool,

    // Timing
    /// Years since practical completion
    pub years_since_practical_completion: u32,
    /// Months since practical completion
    pub months_since_practical_completion: u32,
    /// Years since discovery
    pub years_since_discovery: u32,
    /// Defects liability period (months)
    pub defects_liability_period_months: u32,

    // Damages
    /// Consequential loss
    pub consequential_loss: bool,
    /// Rectification unreasonable
    pub rectification_unreasonable: bool,
}

/// Result of defects liability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefectsLiabilityResult {
    /// Defect classification
    pub defect_classification: DefectClassification,
    /// Within liability period
    pub within_liability_period: bool,
    /// Builder liable
    pub builder_liable: bool,
    /// Available remedies
    pub available_remedies: Vec<BuildingRemedy>,
    /// Reasoning
    pub reasoning: String,
}

// ============================================================================
// Remedies
// ============================================================================

/// Building contract remedies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingRemedy {
    /// Rectification works
    RectificationWorks,
    /// Cost of rectification
    CostOfRectification,
    /// Diminution in value
    DiminutionInValue,
    /// Progress payment
    ProgressPayment,
    /// Adjudication
    Adjudication,
    /// Adjudicated amount
    AdjudicatedAmount,
    /// Suspension of work
    SuspensionOfWork,
    /// Consequential damages
    ConsequentialDamages,
    /// Alternative accommodation
    AlternativeAccommodation,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_domestic_building_contract() {
        let facts = BuildingContractFacts {
            contract_type: Some(BuildingContractType::Domestic),
            domestic_building: true,
            contract_value_over_threshold: true,
            in_writing: true,
            contains_required_terms: true,
            signed_by_both_parties: true,
            completion_date_specified: true,
            price_or_method_specified: true,
            builder_licensed: true,
            insurance_in_place: true,
            contains_cooling_off_notice: true,
            ..Default::default()
        };

        let result = BuildingContractAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.valid_contract);
        assert!(result.compliance_issues.is_empty());
    }

    #[test]
    fn test_invalid_contract_not_in_writing() {
        let facts = BuildingContractFacts {
            domestic_building: true,
            contract_value_over_threshold: true,
            in_writing: false,
            ..Default::default()
        };

        let result = BuildingContractAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.valid_contract);
        assert!(
            result
                .compliance_issues
                .contains(&ComplianceIssue::NotInWriting)
        );
    }

    #[test]
    fn test_statutory_warranty_breach_workmanship() {
        let facts = WarrantyBreachFacts {
            work_performed_properly: false,
            workmanship_defects: true,
            structural_defect: false,
            years_since_completion: 2,
            ..Default::default()
        };

        let result =
            StatutoryWarrantyAnalyzer::analyze(StatutoryWarranty::ProperWorkmanship, &facts);
        assert!(result.breached);
        assert!(result.within_limitation_period);
        assert!(!result.available_remedies.is_empty());
    }

    #[test]
    fn test_statutory_warranty_major_defect() {
        let facts = WarrantyBreachFacts {
            fit_for_habitation: false,
            not_safe_to_occupy: true,
            uninhabitable: true,
            structural_defect: true,
            years_since_completion: 3,
            ..Default::default()
        };

        let result =
            StatutoryWarrantyAnalyzer::analyze(StatutoryWarranty::FitForHabitation, &facts);
        assert!(result.breached);
        assert!(result.major_defect);
        assert!(result.within_limitation_period);
    }

    #[test]
    fn test_progress_payment_valid_claim() {
        let facts = ProgressPaymentFacts {
            valid_claim: true,
            claim_in_writing: true,
            identifies_work: true,
            states_amount: true,
            states_reference_date: true,
            claimed_amount: 50000,
            payment_schedule_issued_on_time: false,
            ..Default::default()
        };

        let result = ProgressPaymentAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.valid_payment_claim);
        assert_eq!(result.amount_payable, 50000);
    }

    #[test]
    fn test_progress_payment_with_schedule() {
        let facts = ProgressPaymentFacts {
            valid_claim: true,
            claim_in_writing: true,
            identifies_work: true,
            states_amount: true,
            states_reference_date: true,
            claimed_amount: 50000,
            payment_schedule_issued_on_time: true,
            scheduled_amount: 40000,
            ..Default::default()
        };

        let result = ProgressPaymentAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.valid_payment_claim);
        assert_eq!(result.amount_payable, 40000);
    }

    #[test]
    fn test_practical_completion_achieved() {
        let facts = PracticalCompletionFacts {
            substantially_complete: true,
            minor_defects_only: true,
            habitable: true,
            major_works_outstanding: false,
            defects_liability_period_months: 12,
            ..Default::default()
        };

        let result = PracticalCompletionAnalyzer::analyze(&facts);
        assert_eq!(
            result.status,
            PracticalCompletionStatus::PracticallyComplete
        );
        assert!(result.defects_liability_commenced);
        assert_eq!(result.defects_liability_period_months, Some(12));
    }

    #[test]
    fn test_practical_completion_not_achieved() {
        let facts = PracticalCompletionFacts {
            substantially_complete: true,
            minor_defects_only: false,
            habitable: false,
            major_works_outstanding: true,
            ..Default::default()
        };

        let result = PracticalCompletionAnalyzer::analyze(&facts);
        assert_eq!(
            result.status,
            PracticalCompletionStatus::SubstantiallyComplete
        );
        assert!(!result.defects_liability_commenced);
    }

    #[test]
    fn test_defects_liability_major_structural() {
        let facts = DefectsLiabilityFacts {
            structural_defect: true,
            builder_responsible: true,
            years_since_practical_completion: 5,
            months_since_practical_completion: 60,
            defects_liability_period_months: 12,
            ..Default::default()
        };

        let result = DefectsLiabilityAnalyzer::analyze(&facts);
        assert_eq!(
            result.defect_classification,
            DefectClassification::MajorStructural
        );
        assert!(result.within_liability_period);
        assert!(result.builder_liable);
        assert!(
            result
                .available_remedies
                .contains(&BuildingRemedy::CostOfRectification)
        );
    }

    #[test]
    fn test_defects_liability_outside_period() {
        let facts = DefectsLiabilityFacts {
            structural_defect: false,
            obvious_on_inspection: true,
            builder_responsible: true,
            months_since_practical_completion: 18,
            defects_liability_period_months: 12,
            ..Default::default()
        };

        let result = DefectsLiabilityAnalyzer::analyze(&facts);
        assert!(!result.within_liability_period);
        assert!(!result.builder_liable);
        assert!(result.available_remedies.is_empty());
    }

    #[test]
    fn test_defects_liability_latent_defect() {
        let facts = DefectsLiabilityFacts {
            hidden_defect: true,
            builder_responsible: true,
            years_since_practical_completion: 8,
            years_since_discovery: 2,
            defects_liability_period_months: 12,
            ..Default::default()
        };

        let result = DefectsLiabilityAnalyzer::analyze(&facts);
        assert_eq!(result.defect_classification, DefectClassification::Latent);
        assert!(result.within_liability_period); // Within 6 years of discovery
        assert!(result.builder_liable);
    }

    #[test]
    fn test_multiple_warranty_breaches() {
        let workmanship_facts = WarrantyBreachFacts {
            work_performed_properly: false,
            workmanship_defects: true,
            years_since_completion: 2,
            ..Default::default()
        };

        let materials_facts = WarrantyBreachFacts {
            unsuitable_materials: true,
            poor_quality_materials: true,
            years_since_completion: 2,
            ..Default::default()
        };

        let workmanship_result = StatutoryWarrantyAnalyzer::analyze(
            StatutoryWarranty::ProperWorkmanship,
            &workmanship_facts,
        );
        let materials_result = StatutoryWarrantyAnalyzer::analyze(
            StatutoryWarranty::SuitableMaterials,
            &materials_facts,
        );

        assert!(workmanship_result.breached);
        assert!(materials_result.breached);
    }

    #[test]
    fn test_building_contract_all_compliance_issues() {
        let facts = BuildingContractFacts {
            domestic_building: true,
            contract_value_over_threshold: true,
            in_writing: false,
            builder_licensed: false,
            insurance_in_place: false,
            deposit_exceeds_maximum: true,
            contains_cooling_off_notice: false,
            ..Default::default()
        };

        let result = BuildingContractAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.valid_contract);
        assert!(result.compliance_issues.len() >= 4);
    }

    #[test]
    fn test_progress_payment_duplicate_claim() {
        let facts = ProgressPaymentFacts {
            claim_in_writing: true,
            identifies_work: true,
            states_amount: true,
            states_reference_date: true,
            duplicate_claim_for_period: true,
            ..Default::default()
        };

        let result = ProgressPaymentAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.valid_payment_claim);
    }

    #[test]
    fn test_adjudication_available() {
        let facts = ProgressPaymentFacts {
            valid_claim: true,
            claim_in_writing: true,
            identifies_work: true,
            states_amount: true,
            states_reference_date: true,
            claimed_amount: 100000,
            payment_made: false,
            payment_dispute: true,
            ..Default::default()
        };

        let result = ProgressPaymentAnalyzer::analyze(&facts);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.adjudication_available);
    }
}

// ============================================================================
// Cost of Rectification Analysis
// ============================================================================

/// Cost of rectification analyzer (Bellgrove v Eldridge principles)
pub struct RectificationCostAnalyzer;

impl RectificationCostAnalyzer {
    /// Analyze cost of rectification claim
    pub fn analyze(facts: &RectificationCostFacts) -> RectificationCostResult {
        let reasonable = Self::is_rectification_reasonable(facts);
        let amount_recoverable = if reasonable {
            facts.cost_of_rectification
        } else {
            facts.diminution_in_value
        };

        let reasoning = Self::build_reasoning(facts, reasonable);

        RectificationCostResult {
            rectification_reasonable: reasonable,
            cost_of_rectification: facts.cost_of_rectification,
            diminution_in_value: facts.diminution_in_value,
            amount_recoverable,
            reasoning,
        }
    }

    /// Check if rectification is reasonable (Bellgrove v Eldridge)
    fn is_rectification_reasonable(facts: &RectificationCostFacts) -> bool {
        // Bellgrove: Owner entitled to cost of rectification if reasonable
        // Not reasonable if: grossly disproportionate to benefit

        if facts.rectification_impossible {
            return false;
        }

        if facts.cost_grossly_disproportionate_to_benefit {
            return false;
        }

        // Owner's subjective preference not determinative (Ruxley v Forsyth)
        if facts.purely_aesthetic_preference && !facts.affects_functionality {
            return false;
        }

        true
    }

    /// Build reasoning
    fn build_reasoning(facts: &RectificationCostFacts, reasonable: bool) -> String {
        let mut parts = Vec::new();

        parts.push("Cost of rectification analysis (Bellgrove v Eldridge)".to_string());

        if reasonable {
            parts.push("Rectification is reasonable measure of damages".to_string());
            parts.push(format!(
                "Cost of rectification: ${}",
                facts.cost_of_rectification
            ));
        } else {
            parts.push("Rectification unreasonable".to_string());

            if facts.rectification_impossible {
                parts.push("Rectification physically impossible".to_string());
            }

            if facts.cost_grossly_disproportionate_to_benefit {
                parts.push("Cost grossly disproportionate to benefit".to_string());
            }

            if facts.purely_aesthetic_preference {
                parts.push("Purely aesthetic preference - diminution in value appropriate (Ruxley v Forsyth)".to_string());
            }

            parts.push(format!(
                "Diminution in value: ${}",
                facts.diminution_in_value
            ));
        }

        parts.join(". ")
    }
}

/// Facts for rectification cost analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RectificationCostFacts {
    /// Cost of rectification
    pub cost_of_rectification: u64,
    /// Diminution in value
    pub diminution_in_value: u64,
    /// Rectification impossible
    pub rectification_impossible: bool,
    /// Cost grossly disproportionate to benefit
    pub cost_grossly_disproportionate_to_benefit: bool,
    /// Purely aesthetic preference
    pub purely_aesthetic_preference: bool,
    /// Affects functionality
    pub affects_functionality: bool,
}

/// Result of rectification cost analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectificationCostResult {
    /// Rectification reasonable
    pub rectification_reasonable: bool,
    /// Cost of rectification
    pub cost_of_rectification: u64,
    /// Diminution in value
    pub diminution_in_value: u64,
    /// Amount recoverable
    pub amount_recoverable: u64,
    /// Reasoning
    pub reasoning: String,
}
