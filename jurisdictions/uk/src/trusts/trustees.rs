//! Trustee Duties and Powers
//!
//! This module implements trustee duties and powers under English law, including:
//! - Common law fiduciary duties (Keech v Sandford, Boardman v Phipps)
//! - Statutory duties under Trustee Act 2000
//! - Investment powers and duties (TA 2000 ss.3-8)
//! - Delegation powers (TA 2000 ss.11-23)
//!
//! ## Fiduciary Duties
//!
//! Trustees owe fiduciary duties to beneficiaries:
//!
//! ### 1. Duty of Care (Speight v Gaunt [1883])
//! Common law: Standard of an ordinary prudent business person
//! Statutory: TA 2000 s.1 duty when exercising specified powers
//!
//! ### 2. Duty of Loyalty - No Conflict (Keech v Sandford [1726])
//! "Inflexible rule" - trustee cannot put themselves in position of conflict
//! - Self-dealing rule: Cannot purchase trust property
//! - Fair-dealing rule: Can deal with beneficiary if full disclosure
//!
//! ### 3. Duty Not to Profit (Boardman v Phipps [1967])
//! No unauthorized profit from trust position
//! Profit must be authorized by:
//! - Trust instrument
//! - Court order
//! - Beneficiary consent (all beneficiaries, fully informed, sui juris)
//!
//! ### 4. Duty of Impartiality (Nestle v National Westminster Bank [1993])
//! Fair treatment between life tenant and remainderman
//! Must balance income vs capital preservation
//!
//! ## Irreducible Core (Armitage v Nurse [1998])
//!
//! Trustee exemption clauses cannot exclude:
//! - Duty to perform trust honestly and in good faith
//! - Cannot exclude liability for fraud, dishonesty, or recklessness
//!
//! ## Key Legislation
//!
//! - **Trustee Act 1925**: Appointment, retirement, powers
//! - **Trustee Act 2000**: Investment, delegation, duty of care
//! - **Trusts of Land and Appointment of Trustees Act 1996**

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::types::{Trustee, TrusteeType};

// ============================================================================
// Trustee Duties
// ============================================================================

/// Fiduciary duties owed by trustees
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrusteeDuty {
    /// Duty of care (Speight v Gaunt, TA 2000 s.1)
    DutyOfCare,
    /// Duty of loyalty - no conflict of interest (Keech v Sandford)
    NoConflict,
    /// Duty not to profit (Boardman v Phipps)
    NoProfit,
    /// Duty of impartiality between beneficiaries (Nestle v NatWest)
    Impartiality,
    /// Duty to act personally (cannot delegate core duties)
    PersonalService,
    /// Duty to keep accounts and provide information
    Accountability,
    /// Duty to distribute correctly
    ProperDistribution,
    /// Duty to preserve trust property
    PreserveTrustProperty,
    /// Duty to invest prudently
    PrudentInvestment,
    /// Duty to act unanimously (unless trust deed provides otherwise)
    Unanimity,
}

impl TrusteeDuty {
    /// Get description with case law reference
    pub fn description(&self) -> &'static str {
        match self {
            Self::DutyOfCare => {
                "Exercise care and skill of ordinary prudent business person (Speight v Gaunt [1883], \
                 TA 2000 s.1)"
            }
            Self::NoConflict => {
                "No conflict of interest - 'inflexible rule' (Keech v Sandford [1726])"
            }
            Self::NoProfit => {
                "No unauthorized profit from trust position (Boardman v Phipps [1967])"
            }
            Self::Impartiality => {
                "Act fairly between beneficiaries with different interests (Nestle v NatWest [1993])"
            }
            Self::PersonalService => {
                "Cannot delegate fiduciary duties (with exceptions under TA 2000 ss.11-23)"
            }
            Self::Accountability => {
                "Maintain proper accounts and provide information to beneficiaries"
            }
            Self::ProperDistribution => "Distribute to correct beneficiaries at correct times",
            Self::PreserveTrustProperty => "Safeguard and preserve trust assets",
            Self::PrudentInvestment => {
                "Invest prudently following standard investment criteria (TA 2000 ss.3-5)"
            }
            Self::Unanimity => {
                "Act unanimously unless trust instrument provides for majority decisions"
            }
        }
    }

    /// Can this duty be excluded by trust instrument?
    pub fn is_excludable(&self) -> bool {
        // Per Armitage v Nurse [1998], only liability for fraud/dishonesty cannot be excluded
        // But duty of care for investment CAN be excluded
        match self {
            Self::DutyOfCare => true,   // Can be modified but not for gross negligence
            Self::NoConflict => false,  // Core fiduciary duty
            Self::NoProfit => true,     // Can authorize specific profits
            Self::Impartiality => true, // Can give power to prefer
            Self::PersonalService => true, // Delegation allowed by TA 2000
            Self::Accountability => false, // Cannot exclude entirely
            Self::ProperDistribution => false,
            Self::PreserveTrustProperty => false,
            Self::PrudentInvestment => true, // Can modify standard
            Self::Unanimity => true,         // Can provide for majority
        }
    }
}

/// Assessment of duty of care compliance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DutyOfCare {
    /// Standard applicable (ordinary trustee or professional)
    pub standard: DutyOfCareStandard,
    /// Specific power being exercised
    pub power_exercised: Option<TrusteePower>,
    /// Was statutory duty of care triggered? (TA 2000 Sch. 1)
    pub statutory_duty_triggered: bool,
    /// Assessment of whether duty met
    pub duty_met: bool,
    /// Reasons for assessment
    pub assessment_reasons: Vec<String>,
    /// Was professional advice obtained?
    pub advice_obtained: bool,
}

/// Standard of care applicable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DutyOfCareStandard {
    /// Ordinary prudent business person (lay trustee)
    OrdinaryPrudent,
    /// Higher standard for professional trustee
    Professional,
    /// Remunerated trustee (intermediate standard)
    Remunerated,
}

impl DutyOfCare {
    /// Assess duty of care for a trustee action
    pub fn assess(
        is_professional: bool,
        is_remunerated: bool,
        power_exercised: Option<TrusteePower>,
        factors: DutyOfCareFactors,
    ) -> Self {
        let standard = if is_professional {
            DutyOfCareStandard::Professional
        } else if is_remunerated {
            DutyOfCareStandard::Remunerated
        } else {
            DutyOfCareStandard::OrdinaryPrudent
        };

        // TA 2000 Schedule 1 - statutory duty applies to certain functions
        let statutory_duty_triggered = power_exercised.is_some_and(|p| {
            matches!(
                p,
                TrusteePower::Investment
                    | TrusteePower::Delegation
                    | TrusteePower::InsuranceOfTrustProperty
                    | TrusteePower::AppointAgent
            )
        });

        let mut assessment_reasons = Vec::new();
        let duty_met = Self::evaluate_factors(&factors, standard, &mut assessment_reasons);

        Self {
            standard,
            power_exercised,
            statutory_duty_triggered,
            duty_met,
            assessment_reasons,
            advice_obtained: factors.advice_obtained,
        }
    }

    fn evaluate_factors(
        factors: &DutyOfCareFactors,
        standard: DutyOfCareStandard,
        reasons: &mut Vec<String>,
    ) -> bool {
        let mut passed = true;

        // Check if professional advice obtained for investment
        if factors.is_investment_decision
            && !factors.advice_obtained
            && !factors.investment_expertise
        {
            reasons.push(
                "TA 2000 s.5 - should obtain proper advice for investment unless \
                 trustee has sufficient expertise"
                    .to_string(),
            );
            passed = false;
        }

        // Check diversification
        if factors.is_investment_decision && !factors.diversification_considered {
            reasons.push(
                "TA 2000 s.4 - standard investment criteria include need for diversification"
                    .to_string(),
            );
            passed = false;
        }

        // Check suitability
        if factors.is_investment_decision && !factors.suitability_assessed {
            reasons.push(
                "TA 2000 s.4(3)(a) - must consider suitability of proposed investment".to_string(),
            );
            passed = false;
        }

        // Professional standard is higher
        if standard == DutyOfCareStandard::Professional && !factors.documented_reasoning {
            reasons
                .push("Professional trustee should document reasoning for decisions".to_string());
            // Not necessarily failure, but noted
        }

        if passed {
            reasons.push("Duty of care requirements satisfied".to_string());
        }

        passed
    }
}

/// Factors for assessing duty of care
#[derive(Debug, Clone, Default)]
pub struct DutyOfCareFactors {
    /// Is this an investment decision?
    pub is_investment_decision: bool,
    /// Was professional advice obtained?
    pub advice_obtained: bool,
    /// Does trustee have investment expertise?
    pub investment_expertise: bool,
    /// Was diversification considered?
    pub diversification_considered: bool,
    /// Was suitability assessed?
    pub suitability_assessed: bool,
    /// Is reasoning documented?
    pub documented_reasoning: bool,
}

// ============================================================================
// Trustee Powers
// ============================================================================

/// Powers that trustees may exercise
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrusteePower {
    /// Power to invest (TA 2000 s.3 - general power)
    Investment,
    /// Power to delegate (TA 2000 ss.11-23)
    Delegation,
    /// Power to appoint agents (TA 2000 s.11)
    AppointAgent,
    /// Power to appoint nominees (TA 2000 s.16)
    AppointNominee,
    /// Power to appoint custodians (TA 2000 s.17)
    AppointCustodian,
    /// Power to insure trust property (TA 2000 s.19)
    InsuranceOfTrustProperty,
    /// Power of advancement (TA 1925 s.32)
    Advancement,
    /// Power of maintenance (TA 1925 s.31)
    Maintenance,
    /// Power to compound liabilities (TA 1925 s.15)
    CompoundLiabilities,
    /// Power to appropriate assets
    Appropriation,
    /// Power to sell trust property
    Sale,
    /// Power to lease
    Lease,
    /// Power to mortgage
    Mortgage,
    /// Power to pay expenses
    PayExpenses,
    /// Power of remuneration (if authorized)
    Remuneration,
}

impl TrusteePower {
    /// Is this power subject to statutory duty of care?
    pub fn subject_to_statutory_duty(&self) -> bool {
        matches!(
            self,
            Self::Investment
                | Self::Delegation
                | Self::AppointAgent
                | Self::AppointNominee
                | Self::AppointCustodian
                | Self::InsuranceOfTrustProperty
        )
    }

    /// Source of power
    pub fn source(&self) -> &'static str {
        match self {
            Self::Investment => "Trustee Act 2000 s.3 (general power of investment)",
            Self::Delegation => "Trustee Act 2000 ss.11-23",
            Self::AppointAgent => "Trustee Act 2000 s.11",
            Self::AppointNominee => "Trustee Act 2000 s.16",
            Self::AppointCustodian => "Trustee Act 2000 s.17",
            Self::InsuranceOfTrustProperty => "Trustee Act 2000 s.19",
            Self::Advancement => "Trustee Act 1925 s.32",
            Self::Maintenance => "Trustee Act 1925 s.31",
            Self::CompoundLiabilities => "Trustee Act 1925 s.15",
            Self::Appropriation => "Trustee Act 1925 s.41",
            Self::Sale => "Trust deed or Trusts of Land Act 1996",
            Self::Lease => "Trust deed or Trusts of Land Act 1996",
            Self::Mortgage => "Trust deed or Trusts of Land Act 1996",
            Self::PayExpenses => "Inherent power",
            Self::Remuneration => "Trust deed or court order (Cradock v Piper exception)",
        }
    }
}

// ============================================================================
// Conflict of Interest
// ============================================================================

/// Type of conflict of interest
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictType {
    /// Self-dealing - trustee purchases trust property
    SelfDealing {
        /// Description of transaction
        transaction: String,
    },
    /// Fair-dealing - trustee deals with beneficiary
    FairDealing {
        /// Description of dealing
        dealing: String,
        /// Was there full disclosure?
        full_disclosure: bool,
    },
    /// Competing interest - trustee has interest adverse to trust
    CompetingInterest {
        /// Description of competing interest
        interest: String,
    },
    /// Multiple trusts - trustee of multiple trusts with conflicting interests
    MultipleTrusts {
        /// Names of conflicting trusts
        trust_names: Vec<String>,
    },
}

/// Conflict of interest analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictOfInterest {
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Is there an actual or potential conflict?
    pub conflict_exists: bool,
    /// Has conflict been authorized?
    pub authorized: ConflictAuthorization,
    /// Assessment of whether trustee may proceed
    pub may_proceed: bool,
    /// Analysis
    pub analysis: String,
}

/// How a conflict has been authorized
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictAuthorization {
    /// Not authorized
    None,
    /// Authorized by trust instrument
    TrustInstrument,
    /// Authorized by court order
    CourtOrder,
    /// Authorized by beneficiary consent
    BeneficiaryConsent {
        /// Number of consenting beneficiaries
        consenting: usize,
        /// Total beneficiaries
        total: usize,
        /// Were all beneficiaries sui juris?
        all_sui_juris: bool,
        /// Was full disclosure made?
        full_disclosure: bool,
    },
}

impl ConflictOfInterest {
    /// Assess a self-dealing transaction
    pub fn assess_self_dealing(transaction: &str, authorized: ConflictAuthorization) -> Self {
        let conflict_type = ConflictType::SelfDealing {
            transaction: transaction.to_string(),
        };

        let may_proceed = matches!(
            authorized,
            ConflictAuthorization::TrustInstrument | ConflictAuthorization::CourtOrder
        ) || matches!(
            &authorized,
            ConflictAuthorization::BeneficiaryConsent {
                all_sui_juris: true,
                full_disclosure: true,
                consenting,
                total,
            } if consenting == total
        );

        let analysis = if may_proceed {
            format!(
                "Self-dealing transaction '{}' is authorized. However, trustee must still \
                 ensure fair value (Ex p Lacey [1802]).",
                transaction
            )
        } else {
            format!(
                "Self-dealing transaction '{}' violates the 'inflexible rule' in Keech v \
                 Sandford [1726]. Transaction is voidable at instance of beneficiaries \
                 regardless of fairness.",
                transaction
            )
        };

        Self {
            conflict_type,
            conflict_exists: true,
            authorized,
            may_proceed,
            analysis,
        }
    }

    /// Assess a fair-dealing transaction with beneficiary
    pub fn assess_fair_dealing(
        dealing: &str,
        full_disclosure: bool,
        authorized: ConflictAuthorization,
    ) -> Self {
        let conflict_type = ConflictType::FairDealing {
            dealing: dealing.to_string(),
            full_disclosure,
        };

        // Fair dealing allowed if full disclosure and fair price (Coles v Trecothick)
        let may_proceed = full_disclosure
            && (matches!(
                authorized,
                ConflictAuthorization::TrustInstrument | ConflictAuthorization::CourtOrder
            ) || matches!(
                &authorized,
                ConflictAuthorization::BeneficiaryConsent {
                    full_disclosure: true,
                    ..
                }
            ));

        let analysis = if may_proceed {
            format!(
                "Fair-dealing transaction '{}' may proceed. Full disclosure was made and \
                 transaction authorized. Per Coles v Trecothick [1804], fair dealing with \
                 beneficiary permitted if open and honest.",
                dealing
            )
        } else if !full_disclosure {
            format!(
                "Fair-dealing transaction '{}' requires full disclosure to beneficiary. \
                 Without disclosure, transaction voidable (Coles v Trecothick).",
                dealing
            )
        } else {
            format!(
                "Fair-dealing transaction '{}' not properly authorized despite disclosure.",
                dealing
            )
        };

        Self {
            conflict_type,
            conflict_exists: true,
            authorized,
            may_proceed,
            analysis,
        }
    }
}

/// Assess conflict of interest
pub fn assess_conflict_of_interest(
    conflict_type: ConflictType,
    authorized: ConflictAuthorization,
) -> ConflictOfInterest {
    match conflict_type {
        ConflictType::SelfDealing { ref transaction } => {
            ConflictOfInterest::assess_self_dealing(transaction, authorized)
        }
        ConflictType::FairDealing {
            ref dealing,
            full_disclosure,
        } => ConflictOfInterest::assess_fair_dealing(dealing, full_disclosure, authorized),
        ConflictType::CompetingInterest { ref interest } => {
            let may_proceed = !matches!(authorized, ConflictAuthorization::None);
            let analysis = format!(
                "Competing interest '{}' creates potential conflict. Trustee must obtain \
                 authorization or resign from one position.",
                interest
            );
            ConflictOfInterest {
                conflict_type,
                conflict_exists: true,
                authorized,
                may_proceed,
                analysis,
            }
        }
        ConflictType::MultipleTrusts { ref trust_names } => {
            let analysis = format!(
                "Trustee serves multiple trusts ({}) with potentially conflicting interests. \
                 May need to resign from one or more trusts.",
                trust_names.join(", ")
            );
            ConflictOfInterest {
                conflict_type,
                conflict_exists: true,
                authorized,
                may_proceed: false,
                analysis,
            }
        }
    }
}

// ============================================================================
// Investment Decision
// ============================================================================

/// Standard investment criteria (TA 2000 s.4)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardInvestmentCriteria {
    /// Suitability of proposed investment (s.4(3)(a))
    pub suitability: InvestmentSuitability,
    /// Need for diversification (s.4(3)(b))
    pub diversification: DiversificationAssessment,
}

/// Suitability assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestmentSuitability {
    /// Is investment suitable for this trust?
    pub suitable_for_trust: bool,
    /// Is investment suitable as part of portfolio?
    pub suitable_as_portfolio_component: bool,
    /// Risk level
    pub risk_level: InvestmentRiskLevel,
    /// Expected return
    pub expected_return: ExpectedReturn,
    /// Assessment notes
    pub notes: String,
}

/// Investment risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestmentRiskLevel {
    /// Low risk (government bonds, cash)
    Low,
    /// Medium risk (diversified equities, corporate bonds)
    Medium,
    /// High risk (single stocks, speculative investments)
    High,
    /// Very high risk (derivatives, cryptocurrencies)
    VeryHigh,
}

/// Expected return classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExpectedReturn {
    /// Income focused
    IncomeOriented,
    /// Growth focused
    GrowthOriented,
    /// Balanced
    Balanced,
    /// Capital preservation
    CapitalPreservation,
}

/// Diversification assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiversificationAssessment {
    /// Is portfolio sufficiently diversified?
    pub adequately_diversified: bool,
    /// Percentage in single asset
    pub largest_single_holding_pct: f64,
    /// Number of different asset classes
    pub asset_class_count: usize,
    /// Concentration risk
    pub concentration_risk: ConcentrationRisk,
}

/// Concentration risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConcentrationRisk {
    /// Low - well diversified
    Low,
    /// Medium - some concentration
    Medium,
    /// High - significant concentration
    High,
    /// Critical - single asset dominance
    Critical,
}

/// Investment decision and validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestmentDecision {
    /// Description of investment
    pub description: String,
    /// Amount
    pub amount_gbp: f64,
    /// Investment type
    pub investment_type: InvestmentType,
    /// Standard investment criteria assessment
    pub criteria: StandardInvestmentCriteria,
    /// Was proper advice obtained? (TA 2000 s.5)
    pub advice_obtained: ProperAdvice,
    /// Was portfolio reviewed? (TA 2000 s.4(2))
    pub portfolio_reviewed: bool,
    /// Is investment authorized?
    pub is_authorized: bool,
    /// Validation result
    pub validation: InvestmentValidation,
}

/// Type of investment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestmentType {
    /// Cash/deposits
    Cash,
    /// Government bonds
    GovernmentBonds,
    /// Corporate bonds
    CorporateBonds,
    /// Listed equities
    ListedEquities,
    /// Unlisted/private equity
    PrivateEquity,
    /// Property
    Property,
    /// Collective investments (funds)
    CollectiveInvestments,
    /// Alternative investments
    Alternative,
}

/// Whether proper advice was obtained
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProperAdvice {
    /// Advice obtained from qualified person
    Obtained {
        /// Advisor name/firm
        advisor: String,
        /// Qualification
        qualification: String,
    },
    /// Not required (trustee has sufficient expertise)
    NotRequired {
        /// Reason
        reason: String,
    },
    /// Required but not obtained
    NotObtained,
}

/// Investment validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestmentValidation {
    /// Is investment valid?
    pub is_valid: bool,
    /// Issues found
    pub issues: Vec<InvestmentIssue>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Investment validation issue
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestmentIssue {
    /// Not within investment powers
    NotWithinPowers,
    /// Advice not obtained when required
    AdviceNotObtained,
    /// Diversification inadequate
    InsufficientDiversification,
    /// Suitability not assessed
    SuitabilityNotAssessed,
    /// Portfolio not reviewed
    PortfolioNotReviewed,
    /// Risk level too high for trust
    RiskTooHigh,
    /// Investment not suitable
    NotSuitable {
        /// Reason investment is not suitable
        reason: String,
    },
}

/// Validate an investment decision
pub fn validate_investment_decision(
    decision: &InvestmentDecision,
    is_professional: bool,
) -> InvestmentValidation {
    let mut issues = Vec::new();
    let mut recommendations = Vec::new();

    // Check advice requirement (TA 2000 s.5)
    if matches!(decision.advice_obtained, ProperAdvice::NotObtained)
        && !matches!(decision.investment_type, InvestmentType::Cash)
    {
        issues.push(InvestmentIssue::AdviceNotObtained);
        recommendations.push(
            "Obtain proper advice from person reasonably believed to be qualified (TA 2000 s.5)"
                .to_string(),
        );
    }

    // Check diversification (TA 2000 s.4(3)(b))
    if !decision.criteria.diversification.adequately_diversified {
        issues.push(InvestmentIssue::InsufficientDiversification);
        recommendations.push(
            "Review diversification - TA 2000 s.4(3)(b) requires consideration of need for \
                   diversification"
                .to_string(),
        );
    }

    // Check suitability (TA 2000 s.4(3)(a))
    if !decision.criteria.suitability.suitable_for_trust {
        issues.push(InvestmentIssue::NotSuitable {
            reason: decision.criteria.suitability.notes.clone(),
        });
    }

    // Check portfolio review (TA 2000 s.4(2))
    if !decision.portfolio_reviewed {
        issues.push(InvestmentIssue::PortfolioNotReviewed);
        recommendations.push(
            "Review investments from time to time and consider whether they should be varied \
             (TA 2000 s.4(2))"
                .to_string(),
        );
    }

    // Check risk level
    if matches!(
        decision.criteria.suitability.risk_level,
        InvestmentRiskLevel::VeryHigh
    ) {
        issues.push(InvestmentIssue::RiskTooHigh);
        recommendations.push(
            "Very high risk investments may breach duty of care unless specifically authorized"
                .to_string(),
        );
    }

    // Professional trustees held to higher standard
    if is_professional && !issues.is_empty() {
        recommendations.push(
            "As professional trustee, a higher standard of care applies (TA 2000 s.1, Sch.1 para.7)"
                .to_string(),
        );
    }

    InvestmentValidation {
        is_valid: issues.is_empty(),
        issues,
        recommendations,
    }
}

// ============================================================================
// Trustee Appointment
// ============================================================================

/// Appointment of new trustee
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrusteeAppointment {
    /// Appointee details
    pub appointee: Trustee,
    /// Appointing authority
    pub appointed_by: AppointingAuthority,
    /// Reason for appointment
    pub reason: AppointmentReason,
    /// Date of appointment
    pub date: NaiveDate,
    /// Is vesting declaration required?
    pub vesting_declaration_required: bool,
    /// Validation result
    pub validation: AppointmentValidation,
}

/// Who is appointing the new trustee
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppointingAuthority {
    /// Existing trustees (TA 1925 s.36(1))
    ExistingTrustees,
    /// Personal representative of last surviving trustee
    PersonalRepresentative,
    /// Beneficiaries (TLATA 1996 s.19)
    Beneficiaries,
    /// Court (TA 1925 s.41)
    Court,
    /// Person nominated in trust instrument
    Nominated {
        /// Name of the nominator
        name: String,
    },
}

/// Reason for trustee appointment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppointmentReason {
    /// Filling vacancy from death
    Death,
    /// Filling vacancy from retirement
    Retirement,
    /// Filling vacancy from removal
    Removal,
    /// Additional trustee (TA 1925 s.36(6))
    Additional,
    /// Replacing trustee abroad (TA 1925 s.36(1))
    ReplacingAbroad,
    /// Replacing incapacitated trustee
    Incapacity,
    /// Replacing refusing trustee
    Refusal,
    /// Replacing infant trustee (cannot act)
    InfantReplacement,
}

/// Appointment validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppointmentValidation {
    /// Is appointment valid?
    pub is_valid: bool,
    /// Issues
    pub issues: Vec<AppointmentIssue>,
    /// Resulting number of trustees
    pub resulting_trustee_count: usize,
}

/// Issues with appointment
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppointmentIssue {
    /// Appointee is a minor
    AppointeeIsMinor,
    /// Would result in too many trustees (max 4 for land)
    TooManyTrustees,
    /// Appointing authority not valid
    InvalidAppointingAuthority,
    /// Appointee lacks capacity
    AppointeeLacksCapacity,
    /// Conflict of interest
    ConflictOfInterest,
    /// Formality not complied with
    FormalityNotComplied {
        /// The requirement not met
        requirement: String,
    },
}

/// Validate trustee appointment
pub fn validate_trustee_appointment(
    appointee: &Trustee,
    appointed_by: &AppointingAuthority,
    reason: AppointmentReason,
    current_trustee_count: usize,
    is_land_trust: bool,
) -> AppointmentValidation {
    let mut issues = Vec::new();

    // Check appointee is not a minor
    // (Trustees must be 18+ - TA 1925 s.20)
    if matches!(appointee.trustee_type, TrusteeType::Individual)
        && appointee
            .appointment_date
            .signed_duration_since(chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap_or_default())
            .num_days()
            < 0
    {
        // This is a simplified check - in reality would check age
        // For now, assume all individual appointees are valid age
    }

    // Check maximum trustees for land (4) - Trustee Act 1925 s.34
    let resulting_count = if matches!(reason, AppointmentReason::Additional) {
        current_trustee_count + 1
    } else {
        current_trustee_count // Replacement maintains count
    };

    if is_land_trust && resulting_count > 4 {
        issues.push(AppointmentIssue::TooManyTrustees);
    }

    // Check appointing authority
    let authority_valid = match appointed_by {
        AppointingAuthority::ExistingTrustees => current_trustee_count > 0,
        AppointingAuthority::PersonalRepresentative => current_trustee_count == 0,
        AppointingAuthority::Beneficiaries => true, // TLATA s.19 if conditions met
        AppointingAuthority::Court => true,
        AppointingAuthority::Nominated { .. } => true,
    };

    if !authority_valid {
        issues.push(AppointmentIssue::InvalidAppointingAuthority);
    }

    AppointmentValidation {
        is_valid: issues.is_empty(),
        issues,
        resulting_trustee_count: resulting_count,
    }
}

/// Check duty of care compliance
pub fn check_duty_of_care(
    is_professional: bool,
    is_remunerated: bool,
    power_exercised: Option<TrusteePower>,
    factors: DutyOfCareFactors,
) -> DutyOfCare {
    DutyOfCare::assess(is_professional, is_remunerated, power_exercised, factors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duty_of_care_investment_without_advice() {
        let factors = DutyOfCareFactors {
            is_investment_decision: true,
            advice_obtained: false,
            investment_expertise: false,
            diversification_considered: true,
            suitability_assessed: true,
            documented_reasoning: true,
        };

        let result = check_duty_of_care(false, false, Some(TrusteePower::Investment), factors);
        assert!(!result.duty_met);
    }

    #[test]
    fn test_self_dealing_requires_authorization() {
        let conflict = assess_conflict_of_interest(
            ConflictType::SelfDealing {
                transaction: "Purchase of trust property".to_string(),
            },
            ConflictAuthorization::None,
        );

        assert!(!conflict.may_proceed);
        assert!(conflict.analysis.contains("Keech v Sandford"));
    }

    #[test]
    fn test_fair_dealing_with_disclosure() {
        let conflict = assess_conflict_of_interest(
            ConflictType::FairDealing {
                dealing: "Purchase of beneficiary's share".to_string(),
                full_disclosure: true,
            },
            ConflictAuthorization::BeneficiaryConsent {
                consenting: 1,
                total: 1,
                all_sui_juris: true,
                full_disclosure: true,
            },
        );

        assert!(conflict.may_proceed);
    }

    #[test]
    fn test_trustee_appointment_land_max_four() {
        let appointee = Trustee {
            name: "New Trustee".to_string(),
            trustee_type: TrusteeType::Individual,
            appointment_date: chrono::Local::now().date_naive(),
            removal_date: None,
            is_professional: false,
        };

        let result = validate_trustee_appointment(
            &appointee,
            &AppointingAuthority::ExistingTrustees,
            AppointmentReason::Additional,
            4, // Already at maximum
            true,
        );

        assert!(!result.is_valid);
        assert!(result.issues.contains(&AppointmentIssue::TooManyTrustees));
    }
}
