//! Domain-specific simulation models and presets.
//!
//! This module provides ready-to-use simulation templates for common legal domains:
//! - Tax system simulations (income, sales, property, corporate taxes)
//! - Benefit eligibility simulations (unemployment, welfare, social security)
//! - Regulatory compliance simulations (licensing, permits, certifications)
//!
//! These presets help quickly set up realistic simulations without manual configuration.

use serde::{Deserialize, Serialize};

/// Tax system configuration preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxSystemPreset {
    /// Name of the tax system
    pub name: String,
    /// Tax type (income, sales, property, corporate, etc.)
    pub tax_type: TaxType,
    /// Tax brackets (threshold -> rate)
    pub brackets: Vec<TaxBracket>,
    /// Standard deduction amount
    pub standard_deduction: f64,
    /// Exemptions per dependent
    pub exemption_per_dependent: f64,
    /// Credits available
    pub credits: Vec<TaxCredit>,
}

/// Type of tax system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxType {
    /// Income tax (progressive or flat)
    Income,
    /// Sales tax (on purchases)
    Sales,
    /// Property tax (on real estate)
    Property,
    /// Corporate tax (on business profits)
    Corporate,
    /// Capital gains tax
    CapitalGains,
    /// Payroll tax (Social Security, Medicare)
    Payroll,
    /// Estate tax (inheritance)
    Estate,
}

/// Tax bracket definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBracket {
    /// Income threshold for this bracket
    pub threshold: f64,
    /// Tax rate (as decimal, e.g., 0.22 for 22%)
    pub rate: f64,
}

impl TaxBracket {
    /// Creates a new tax bracket
    pub fn new(threshold: f64, rate: f64) -> Self {
        Self { threshold, rate }
    }
}

/// Tax credit definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCredit {
    /// Credit name
    pub name: String,
    /// Credit amount
    pub amount: f64,
    /// Whether credit is refundable
    pub refundable: bool,
    /// Income phase-out threshold (None if no phase-out)
    pub phase_out_threshold: Option<f64>,
}

impl TaxCredit {
    /// Creates a new tax credit
    pub fn new(name: String, amount: f64, refundable: bool) -> Self {
        Self {
            name,
            amount,
            refundable,
            phase_out_threshold: None,
        }
    }

    /// Sets phase-out threshold
    pub fn with_phase_out(mut self, threshold: f64) -> Self {
        self.phase_out_threshold = Some(threshold);
        self
    }
}

impl TaxSystemPreset {
    /// Creates a US federal income tax preset (2024 single filer)
    pub fn us_federal_income_tax_2024() -> Self {
        Self {
            name: "US Federal Income Tax 2024 (Single)".to_string(),
            tax_type: TaxType::Income,
            brackets: vec![
                TaxBracket::new(0.0, 0.10),
                TaxBracket::new(11_000.0, 0.12),
                TaxBracket::new(44_725.0, 0.22),
                TaxBracket::new(95_375.0, 0.24),
                TaxBracket::new(182_100.0, 0.32),
                TaxBracket::new(231_250.0, 0.35),
                TaxBracket::new(578_125.0, 0.37),
            ],
            standard_deduction: 14_600.0,
            exemption_per_dependent: 0.0, // Eliminated in 2018
            credits: vec![
                TaxCredit::new("Earned Income Credit".to_string(), 600.0, true),
                TaxCredit::new("Child Tax Credit".to_string(), 2_000.0, true)
                    .with_phase_out(200_000.0),
            ],
        }
    }

    /// Creates a flat tax preset
    pub fn flat_tax(rate: f64, deduction: f64) -> Self {
        Self {
            name: format!("Flat Tax ({}%)", rate * 100.0),
            tax_type: TaxType::Income,
            brackets: vec![TaxBracket::new(0.0, rate)],
            standard_deduction: deduction,
            exemption_per_dependent: 0.0,
            credits: Vec::new(),
        }
    }

    /// Creates a sales tax preset
    pub fn sales_tax(rate: f64, _exemptions: Vec<String>) -> Self {
        Self {
            name: format!("Sales Tax ({}%)", rate * 100.0),
            tax_type: TaxType::Sales,
            brackets: vec![TaxBracket::new(0.0, rate)],
            standard_deduction: 0.0,
            exemption_per_dependent: 0.0,
            credits: Vec::new(),
        }
    }

    /// Calculates tax owed for a given income
    pub fn calculate_tax(&self, income: f64, dependents: usize) -> TaxCalculation {
        let mut taxable_income =
            income - self.standard_deduction - (self.exemption_per_dependent * dependents as f64);
        taxable_income = taxable_income.max(0.0);

        let mut tax = 0.0;

        // Calculate tax for each bracket
        for i in 0..self.brackets.len() {
            let current_bracket = &self.brackets[i];
            let next_threshold = if i + 1 < self.brackets.len() {
                self.brackets[i + 1].threshold
            } else {
                f64::MAX
            };

            // Determine how much income falls in this bracket
            let bracket_income = if taxable_income <= current_bracket.threshold {
                0.0
            } else if taxable_income < next_threshold {
                taxable_income - current_bracket.threshold
            } else {
                next_threshold - current_bracket.threshold
            };

            tax += bracket_income * current_bracket.rate;

            // Stop if we've accounted for all income
            if taxable_income < next_threshold {
                break;
            }
        }

        // Apply credits
        let mut total_credits = 0.0;
        for credit in &self.credits {
            let credit_amount = if let Some(phase_out) = credit.phase_out_threshold {
                if income > phase_out {
                    0.0
                } else {
                    credit.amount
                }
            } else {
                credit.amount
            };
            total_credits += credit_amount;
        }

        let tax_after_credits = (tax - total_credits).max(0.0);

        TaxCalculation {
            gross_income: income,
            taxable_income,
            tax_before_credits: tax,
            credits_applied: total_credits,
            tax_owed: tax_after_credits,
            effective_rate: if income > 0.0 {
                tax_after_credits / income
            } else {
                0.0
            },
        }
    }
}

/// Result of tax calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCalculation {
    /// Gross income before deductions
    pub gross_income: f64,
    /// Taxable income after deductions
    pub taxable_income: f64,
    /// Tax before credits
    pub tax_before_credits: f64,
    /// Total credits applied
    pub credits_applied: f64,
    /// Final tax owed
    pub tax_owed: f64,
    /// Effective tax rate
    pub effective_rate: f64,
}

/// Benefit eligibility configuration preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitPreset {
    /// Name of the benefit program
    pub name: String,
    /// Benefit type
    pub benefit_type: BenefitType,
    /// Income threshold for eligibility
    pub income_threshold: f64,
    /// Asset threshold for eligibility
    pub asset_threshold: Option<f64>,
    /// Benefit amount or calculation method
    pub benefit_amount: BenefitAmount,
    /// Additional eligibility requirements
    pub requirements: Vec<EligibilityRequirement>,
}

/// Type of benefit program
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BenefitType {
    /// Unemployment insurance
    Unemployment,
    /// Welfare assistance (TANF, etc.)
    Welfare,
    /// Food assistance (SNAP)
    FoodAssistance,
    /// Housing assistance
    Housing,
    /// Healthcare (Medicaid, Medicare)
    Healthcare,
    /// Social Security retirement
    SocialSecurity,
    /// Disability benefits
    Disability,
    /// Child care assistance
    ChildCare,
}

/// Benefit amount calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenefitAmount {
    /// Fixed monthly amount
    Fixed(f64),
    /// Percentage of previous income
    PercentageOfIncome(f64),
    /// Sliding scale based on income
    SlidingScale(Vec<(f64, f64)>), // (income_threshold, benefit_amount)
}

/// Eligibility requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityRequirement {
    /// Requirement name
    pub name: String,
    /// Requirement type
    pub requirement_type: RequirementType,
    /// Required value
    pub required_value: String,
}

/// Type of eligibility requirement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequirementType {
    /// Age requirement
    Age,
    /// Employment status
    EmploymentStatus,
    /// Citizenship status
    Citizenship,
    /// Residency requirement
    Residency,
    /// Disability status
    Disability,
    /// Number of dependents
    Dependents,
    /// Work history
    WorkHistory,
}

impl BenefitPreset {
    /// Creates a US unemployment insurance preset
    pub fn us_unemployment_insurance() -> Self {
        Self {
            name: "US Unemployment Insurance".to_string(),
            benefit_type: BenefitType::Unemployment,
            income_threshold: 75_000.0, // Previous year income
            asset_threshold: None,
            benefit_amount: BenefitAmount::PercentageOfIncome(0.50), // 50% of previous income
            requirements: vec![
                EligibilityRequirement {
                    name: "Work History".to_string(),
                    requirement_type: RequirementType::WorkHistory,
                    required_value: "12 months".to_string(),
                },
                EligibilityRequirement {
                    name: "Employment Status".to_string(),
                    requirement_type: RequirementType::EmploymentStatus,
                    required_value: "unemployed".to_string(),
                },
            ],
        }
    }

    /// Creates a SNAP (food assistance) preset
    pub fn snap_food_assistance() -> Self {
        Self {
            name: "SNAP Food Assistance".to_string(),
            benefit_type: BenefitType::FoodAssistance,
            income_threshold: 36_000.0, // Annual for family of 4
            asset_threshold: Some(2_750.0),
            benefit_amount: BenefitAmount::SlidingScale(vec![
                (0.0, 835.0),
                (15_000.0, 600.0),
                (25_000.0, 400.0),
                (36_000.0, 200.0),
            ]),
            requirements: vec![
                EligibilityRequirement {
                    name: "Citizenship".to_string(),
                    requirement_type: RequirementType::Citizenship,
                    required_value: "US Citizen or Legal Resident".to_string(),
                },
                EligibilityRequirement {
                    name: "Work Requirements".to_string(),
                    requirement_type: RequirementType::EmploymentStatus,
                    required_value: "Working or seeking work".to_string(),
                },
            ],
        }
    }

    /// Creates a Social Security retirement preset
    pub fn social_security_retirement() -> Self {
        Self {
            name: "Social Security Retirement".to_string(),
            benefit_type: BenefitType::SocialSecurity,
            income_threshold: f64::MAX, // No income limit
            asset_threshold: None,
            benefit_amount: BenefitAmount::PercentageOfIncome(0.40), // ~40% of average earnings
            requirements: vec![
                EligibilityRequirement {
                    name: "Age".to_string(),
                    requirement_type: RequirementType::Age,
                    required_value: "62".to_string(),
                },
                EligibilityRequirement {
                    name: "Work Credits".to_string(),
                    requirement_type: RequirementType::WorkHistory,
                    required_value: "40 quarters".to_string(),
                },
            ],
        }
    }

    /// Checks eligibility based on income and assets
    pub fn check_eligibility(&self, income: f64, assets: f64) -> EligibilityResult {
        let mut eligible = income <= self.income_threshold;
        let mut reasons = Vec::new();

        if !eligible {
            reasons.push(format!(
                "Income ${} exceeds threshold ${}",
                income, self.income_threshold
            ));
        }

        if let Some(asset_limit) = self.asset_threshold {
            if assets > asset_limit {
                eligible = false;
                reasons.push(format!(
                    "Assets ${} exceed threshold ${}",
                    assets, asset_limit
                ));
            }
        }

        let benefit_amount = if eligible {
            match &self.benefit_amount {
                BenefitAmount::Fixed(amount) => *amount,
                BenefitAmount::PercentageOfIncome(pct) => income * pct,
                BenefitAmount::SlidingScale(scale) => {
                    let mut benefit = 0.0;
                    for (threshold, amount) in scale {
                        if income <= *threshold {
                            benefit = *amount;
                            break;
                        }
                    }
                    benefit
                }
            }
        } else {
            0.0
        };

        EligibilityResult {
            eligible,
            benefit_amount,
            reasons,
        }
    }
}

/// Result of eligibility check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityResult {
    /// Whether the entity is eligible
    pub eligible: bool,
    /// Benefit amount if eligible
    pub benefit_amount: f64,
    /// Reasons for ineligibility
    pub reasons: Vec<String>,
}

/// Regulatory compliance configuration preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompliancePreset {
    /// Name of the regulatory requirement
    pub name: String,
    /// Compliance type
    pub compliance_type: ComplianceType,
    /// Required actions
    pub required_actions: Vec<ComplianceAction>,
    /// Renewal period (in days, None if one-time)
    pub renewal_period: Option<usize>,
    /// Penalties for non-compliance
    pub penalties: Vec<Penalty>,
}

/// Type of regulatory compliance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceType {
    /// Business licensing
    Licensing,
    /// Permit requirements
    Permit,
    /// Professional certification
    Certification,
    /// Environmental regulation
    Environmental,
    /// Health and safety
    HealthSafety,
    /// Data privacy (GDPR, CCPA)
    DataPrivacy,
    /// Financial reporting
    FinancialReporting,
}

/// Compliance action requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAction {
    /// Action name
    pub name: String,
    /// Cost to complete action
    pub cost: f64,
    /// Time required (in days)
    pub time_required: usize,
    /// Whether action is recurring
    pub recurring: bool,
}

impl ComplianceAction {
    /// Creates a new compliance action
    pub fn new(name: String, cost: f64, time_required: usize, recurring: bool) -> Self {
        Self {
            name,
            cost,
            time_required,
            recurring,
        }
    }
}

/// Penalty for non-compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    /// Penalty type
    pub penalty_type: PenaltyType,
    /// Penalty amount or percentage
    pub amount: f64,
    /// Whether penalty is per violation or fixed
    pub per_violation: bool,
}

/// Type of penalty
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyType {
    /// Monetary fine
    Fine,
    /// License suspension
    Suspension,
    /// License revocation
    Revocation,
    /// Criminal charges
    Criminal,
    /// Corrective action required
    CorrectiveAction,
}

impl CompliancePreset {
    /// Creates a business license preset
    pub fn business_license() -> Self {
        Self {
            name: "General Business License".to_string(),
            compliance_type: ComplianceType::Licensing,
            required_actions: vec![
                ComplianceAction::new("Application Filing".to_string(), 150.0, 1, false),
                ComplianceAction::new("Annual Renewal".to_string(), 100.0, 1, true),
            ],
            renewal_period: Some(365),
            penalties: vec![
                Penalty {
                    penalty_type: PenaltyType::Fine,
                    amount: 500.0,
                    per_violation: false,
                },
                Penalty {
                    penalty_type: PenaltyType::Suspension,
                    amount: 0.0,
                    per_violation: false,
                },
            ],
        }
    }

    /// Creates a GDPR compliance preset
    pub fn gdpr_compliance() -> Self {
        Self {
            name: "GDPR Data Privacy Compliance".to_string(),
            compliance_type: ComplianceType::DataPrivacy,
            required_actions: vec![
                ComplianceAction::new(
                    "Privacy Policy Implementation".to_string(),
                    5_000.0,
                    30,
                    false,
                ),
                ComplianceAction::new(
                    "Data Protection Officer Appointment".to_string(),
                    75_000.0, // Annual salary
                    7,
                    true,
                ),
                ComplianceAction::new("Annual Compliance Audit".to_string(), 10_000.0, 14, true),
            ],
            renewal_period: Some(365),
            penalties: vec![Penalty {
                penalty_type: PenaltyType::Fine,
                amount: 20_000_000.0, // Up to €20M or 4% of global revenue
                per_violation: true,
            }],
        }
    }

    /// Creates an environmental permit preset
    pub fn environmental_permit() -> Self {
        Self {
            name: "Environmental Operating Permit".to_string(),
            compliance_type: ComplianceType::Environmental,
            required_actions: vec![
                ComplianceAction::new(
                    "Environmental Impact Assessment".to_string(),
                    15_000.0,
                    60,
                    false,
                ),
                ComplianceAction::new("Permit Application".to_string(), 2_500.0, 7, false),
                ComplianceAction::new(
                    "Quarterly Emissions Reporting".to_string(),
                    1_000.0,
                    2,
                    true,
                ),
            ],
            renewal_period: Some(1825), // 5 years
            penalties: vec![
                Penalty {
                    penalty_type: PenaltyType::Fine,
                    amount: 50_000.0,
                    per_violation: true,
                },
                Penalty {
                    penalty_type: PenaltyType::CorrectiveAction,
                    amount: 100_000.0,
                    per_violation: false,
                },
            ],
        }
    }

    /// Calculates total compliance cost
    pub fn total_cost(&self, time_period_days: usize) -> ComplianceCost {
        let mut one_time_costs = 0.0;
        let mut recurring_costs = 0.0;
        let mut total_time = 0;

        for action in &self.required_actions {
            if action.recurring {
                if let Some(renewal_period) = self.renewal_period {
                    let occurrences = time_period_days.div_ceil(renewal_period);
                    recurring_costs += action.cost * occurrences as f64;
                    total_time += action.time_required * occurrences;
                }
            } else {
                one_time_costs += action.cost;
                total_time += action.time_required;
            }
        }

        ComplianceCost {
            one_time_costs,
            recurring_costs,
            total_costs: one_time_costs + recurring_costs,
            total_time_days: total_time,
        }
    }
}

/// Compliance cost calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCost {
    /// One-time compliance costs
    pub one_time_costs: f64,
    /// Recurring compliance costs
    pub recurring_costs: f64,
    /// Total compliance costs
    pub total_costs: f64,
    /// Total time required (in days)
    pub total_time_days: usize,
}

/// Court case outcome prediction preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtCasePreset {
    /// Name of the case type
    pub name: String,
    /// Court level
    pub court_level: CourtLevel,
    /// Case factors that influence the outcome
    pub case_factors: Vec<CaseFactor>,
    /// Historical precedents
    pub precedents: Vec<Precedent>,
    /// Base probability of plaintiff/prosecution winning
    pub base_probability: f64,
}

/// Court level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CourtLevel {
    /// Trial court (district, county, circuit)
    Trial,
    /// Appellate court (appeals court)
    Appellate,
    /// Supreme court (state or federal)
    Supreme,
    /// Administrative court
    Administrative,
    /// Specialized court (tax, bankruptcy, etc.)
    Specialized,
}

/// Factor that influences case outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseFactor {
    /// Factor name
    pub name: String,
    /// Factor type
    pub factor_type: FactorType,
    /// Impact on outcome probability (positive = favors plaintiff, negative = favors defendant)
    pub impact: f64,
    /// Weight of this factor (0.0 to 1.0)
    pub weight: f64,
}

/// Type of case factor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorType {
    /// Evidence strength
    Evidence,
    /// Legal precedent
    Precedent,
    /// Witness credibility
    Witness,
    /// Expert testimony
    ExpertTestimony,
    /// Procedural issues
    Procedural,
    /// Jurisdiction-specific factors
    Jurisdictional,
    /// Attorney experience/quality
    Attorney,
    /// Judge tendencies
    Judge,
}

impl CaseFactor {
    /// Creates a new case factor
    pub fn new(name: String, factor_type: FactorType, impact: f64, weight: f64) -> Self {
        Self {
            name,
            factor_type,
            impact,
            weight,
        }
    }
}

/// Legal precedent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precedent {
    /// Case name
    pub case_name: String,
    /// Year decided
    pub year: usize,
    /// Similarity to current case (0.0 to 1.0)
    pub similarity: f64,
    /// Whether precedent favors plaintiff
    pub favors_plaintiff: bool,
    /// Precedent strength (binding vs persuasive)
    pub binding: bool,
}

impl Precedent {
    /// Creates a new precedent
    pub fn new(
        case_name: String,
        year: usize,
        similarity: f64,
        favors_plaintiff: bool,
        binding: bool,
    ) -> Self {
        Self {
            case_name,
            year,
            similarity,
            favors_plaintiff,
            binding,
        }
    }
}

impl CourtCasePreset {
    /// Creates a civil contract dispute preset
    pub fn civil_contract_dispute() -> Self {
        Self {
            name: "Civil Contract Dispute".to_string(),
            court_level: CourtLevel::Trial,
            case_factors: vec![
                CaseFactor::new(
                    "Written Contract Evidence".to_string(),
                    FactorType::Evidence,
                    0.30,
                    0.40,
                ),
                CaseFactor::new(
                    "Email Documentation".to_string(),
                    FactorType::Evidence,
                    0.20,
                    0.25,
                ),
                CaseFactor::new(
                    "Witness Testimony".to_string(),
                    FactorType::Witness,
                    0.15,
                    0.20,
                ),
                CaseFactor::new(
                    "Legal Representation Quality".to_string(),
                    FactorType::Attorney,
                    0.10,
                    0.15,
                ),
            ],
            precedents: vec![
                Precedent::new("Smith v. Jones (2020)".to_string(), 2020, 0.85, true, true),
                Precedent::new(
                    "Acme Corp v. Widget Inc (2018)".to_string(),
                    2018,
                    0.70,
                    false,
                    true,
                ),
            ],
            base_probability: 0.50,
        }
    }

    /// Creates a criminal case preset
    pub fn criminal_case() -> Self {
        Self {
            name: "Criminal Case".to_string(),
            court_level: CourtLevel::Trial,
            case_factors: vec![
                CaseFactor::new(
                    "Physical Evidence".to_string(),
                    FactorType::Evidence,
                    0.40,
                    0.35,
                ),
                CaseFactor::new(
                    "Eyewitness Testimony".to_string(),
                    FactorType::Witness,
                    0.25,
                    0.25,
                ),
                CaseFactor::new(
                    "Forensic Analysis".to_string(),
                    FactorType::ExpertTestimony,
                    0.30,
                    0.30,
                ),
                CaseFactor::new(
                    "Procedural Compliance".to_string(),
                    FactorType::Procedural,
                    -0.20,
                    0.10,
                ),
            ],
            precedents: vec![],
            base_probability: 0.60, // Prosecution typically needs strong case
        }
    }

    /// Predicts the outcome probability based on case factors
    pub fn predict_outcome(&self, factor_values: &[(String, f64)]) -> OutcomePrediction {
        let mut adjusted_probability = self.base_probability;
        let mut factor_contributions = Vec::new();

        // Calculate weighted impact of each factor
        for (factor_name, value) in factor_values {
            if let Some(factor) = self.case_factors.iter().find(|f| f.name == *factor_name) {
                let contribution = factor.impact * factor.weight * value;
                adjusted_probability += contribution;
                factor_contributions.push((factor_name.clone(), contribution));
            }
        }

        // Apply precedent influence
        let mut precedent_impact = 0.0;
        for precedent in &self.precedents {
            let impact = if precedent.binding {
                precedent.similarity * 0.15
            } else {
                precedent.similarity * 0.05
            };
            precedent_impact += if precedent.favors_plaintiff {
                impact
            } else {
                -impact
            };
        }
        adjusted_probability += precedent_impact;

        // Clamp to valid probability range
        adjusted_probability = adjusted_probability.clamp(0.0, 1.0);

        OutcomePrediction {
            plaintiff_win_probability: adjusted_probability,
            defendant_win_probability: 1.0 - adjusted_probability,
            confidence: self.calculate_confidence(factor_values),
            factor_contributions,
            precedent_impact,
        }
    }

    /// Calculates confidence in prediction
    fn calculate_confidence(&self, factor_values: &[(String, f64)]) -> f64 {
        let total_factors = self.case_factors.len();
        let provided_factors = factor_values.len();
        let factor_coverage = provided_factors as f64 / total_factors as f64;

        // Confidence based on factor coverage and precedent availability
        let precedent_boost = (self.precedents.len() as f64 * 0.05).min(0.15);
        (factor_coverage * 0.85 + precedent_boost).min(1.0)
    }
}

/// Court case outcome prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomePrediction {
    /// Probability of plaintiff/prosecution winning
    pub plaintiff_win_probability: f64,
    /// Probability of defendant winning
    pub defendant_win_probability: f64,
    /// Confidence in prediction (0.0 to 1.0)
    pub confidence: f64,
    /// Contribution of each factor to the outcome
    pub factor_contributions: Vec<(String, f64)>,
    /// Impact of precedents on outcome
    pub precedent_impact: f64,
}

/// Legislative impact forecasting preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativePreset {
    /// Name of the legislative body
    pub name: String,
    /// Legislative level
    pub legislative_level: LegislativeLevel,
    /// Total number of legislators
    pub total_legislators: usize,
    /// Current party composition
    pub party_composition: Vec<PartyComposition>,
    /// Historical voting patterns
    pub voting_patterns: Vec<VotingPattern>,
}

/// Legislative level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegislativeLevel {
    /// Federal/national legislature
    Federal,
    /// State/provincial legislature
    State,
    /// Local/municipal legislature
    Local,
    /// International body
    International,
}

/// Party composition in legislature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyComposition {
    /// Party name
    pub party_name: String,
    /// Number of seats held
    pub seats: usize,
    /// Historical support rate for similar legislation
    pub historical_support_rate: f64,
}

impl PartyComposition {
    /// Creates a new party composition
    pub fn new(party_name: String, seats: usize, historical_support_rate: f64) -> Self {
        Self {
            party_name,
            seats,
            historical_support_rate,
        }
    }
}

/// Historical voting pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingPattern {
    /// Issue area
    pub issue_area: String,
    /// Historical passage rate
    pub passage_rate: f64,
    /// Average time to passage (in days)
    pub avg_days_to_passage: usize,
}

impl VotingPattern {
    /// Creates a new voting pattern
    pub fn new(issue_area: String, passage_rate: f64, avg_days_to_passage: usize) -> Self {
        Self {
            issue_area,
            passage_rate,
            avg_days_to_passage,
        }
    }
}

impl LegislativePreset {
    /// Creates a US Congress preset
    pub fn us_congress() -> Self {
        Self {
            name: "US Congress".to_string(),
            legislative_level: LegislativeLevel::Federal,
            total_legislators: 535, // 435 House + 100 Senate
            party_composition: vec![
                PartyComposition::new("Democratic".to_string(), 268, 0.45),
                PartyComposition::new("Republican".to_string(), 267, 0.42),
            ],
            voting_patterns: vec![
                VotingPattern::new("Healthcare".to_string(), 0.35, 180),
                VotingPattern::new("Tax Reform".to_string(), 0.28, 240),
                VotingPattern::new("Infrastructure".to_string(), 0.52, 150),
                VotingPattern::new("Defense".to_string(), 0.68, 90),
            ],
        }
    }

    /// Creates a state legislature preset
    pub fn state_legislature(total_seats: usize) -> Self {
        Self {
            name: "State Legislature".to_string(),
            legislative_level: LegislativeLevel::State,
            total_legislators: total_seats,
            party_composition: vec![
                PartyComposition::new("Democratic".to_string(), total_seats / 2, 0.50),
                PartyComposition::new("Republican".to_string(), total_seats / 2, 0.48),
            ],
            voting_patterns: vec![
                VotingPattern::new("Education".to_string(), 0.60, 120),
                VotingPattern::new("Budget".to_string(), 0.55, 90),
                VotingPattern::new("Criminal Justice".to_string(), 0.42, 150),
            ],
        }
    }

    /// Forecasts the probability of bill passage
    pub fn forecast_passage(&self, bill: &Bill) -> PassageForecast {
        // Calculate expected votes based on party composition
        let mut expected_yes_votes = 0;
        for party in &self.party_composition {
            let support_prob = if bill.party_positions.contains_key(&party.party_name) {
                bill.party_positions[&party.party_name]
            } else {
                party.historical_support_rate
            };

            expected_yes_votes += (party.seats as f64 * support_prob) as usize;
        }

        // Check if bill would pass based on required majority
        let required_votes = match bill.required_majority {
            Majority::Simple => (self.total_legislators as f64 * 0.51) as usize,
            Majority::TwoThirds => (self.total_legislators as f64 * 0.67) as usize,
            Majority::ThreeFifths => (self.total_legislators as f64 * 0.60) as usize,
        };

        let mut passage_probability = (expected_yes_votes as f64) / (required_votes as f64);
        passage_probability = passage_probability.min(1.0);

        // Apply historical voting pattern adjustment
        let total_weight = if let Some(pattern) = self
            .voting_patterns
            .iter()
            .find(|p| p.issue_area == bill.issue_area)
        {
            passage_probability = passage_probability * 0.7 + pattern.passage_rate * 0.3;
            0.7
        } else {
            1.0
        };

        // Estimate time to passage
        let estimated_days = if let Some(pattern) = self
            .voting_patterns
            .iter()
            .find(|p| p.issue_area == bill.issue_area)
        {
            pattern.avg_days_to_passage
        } else {
            180 // Default estimate
        };

        PassageForecast {
            passage_probability,
            expected_yes_votes,
            required_votes,
            estimated_days_to_vote: estimated_days,
            confidence: total_weight,
        }
    }
}

/// Bill definition for legislative forecasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bill {
    /// Bill name/number
    pub name: String,
    /// Issue area
    pub issue_area: String,
    /// Party positions on the bill (party name -> support probability)
    pub party_positions: std::collections::HashMap<String, f64>,
    /// Required majority to pass
    pub required_majority: Majority,
}

/// Required voting majority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Majority {
    /// Simple majority (50% + 1)
    Simple,
    /// Three-fifths majority (60%)
    ThreeFifths,
    /// Two-thirds majority (67%)
    TwoThirds,
}

impl Bill {
    /// Creates a new bill
    pub fn new(name: String, issue_area: String, required_majority: Majority) -> Self {
        Self {
            name,
            issue_area,
            party_positions: std::collections::HashMap::new(),
            required_majority,
        }
    }

    /// Sets party position on the bill
    pub fn with_party_position(mut self, party: String, support_probability: f64) -> Self {
        self.party_positions.insert(party, support_probability);
        self
    }
}

/// Legislative passage forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassageForecast {
    /// Probability of bill passage
    pub passage_probability: f64,
    /// Expected number of yes votes
    pub expected_yes_votes: usize,
    /// Required number of votes to pass
    pub required_votes: usize,
    /// Estimated days until vote
    pub estimated_days_to_vote: usize,
    /// Confidence in forecast (0.0 to 1.0)
    pub confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_us_federal_tax_2024() {
        let tax_system = TaxSystemPreset::us_federal_income_tax_2024();
        assert_eq!(tax_system.tax_type, TaxType::Income);
        assert_eq!(tax_system.brackets.len(), 7);
        assert_eq!(tax_system.standard_deduction, 14_600.0);
    }

    #[test]
    fn test_tax_calculation_low_income() {
        let tax_system = TaxSystemPreset::us_federal_income_tax_2024();
        let calc = tax_system.calculate_tax(30_000.0, 0);

        assert_eq!(calc.gross_income, 30_000.0);
        assert!(calc.taxable_income > 0.0);
        assert!(calc.tax_owed >= 0.0);
        assert!(calc.effective_rate >= 0.0 && calc.effective_rate < 1.0);
        // Tax before credits should be positive
        assert!(calc.tax_before_credits > 0.0);
    }

    #[test]
    fn test_tax_calculation_with_dependents() {
        let tax_system = TaxSystemPreset::us_federal_income_tax_2024();
        let calc = tax_system.calculate_tax(50_000.0, 2);

        assert!(calc.taxable_income < 50_000.0);
    }

    #[test]
    fn test_flat_tax() {
        let tax_system = TaxSystemPreset::flat_tax(0.15, 10_000.0);
        assert_eq!(tax_system.brackets.len(), 1);
        assert_eq!(tax_system.brackets[0].rate, 0.15);
    }

    #[test]
    fn test_unemployment_insurance() {
        let benefit = BenefitPreset::us_unemployment_insurance();
        assert_eq!(benefit.benefit_type, BenefitType::Unemployment);
        assert_eq!(benefit.requirements.len(), 2);
    }

    #[test]
    fn test_snap_eligibility() {
        let benefit = BenefitPreset::snap_food_assistance();
        let result = benefit.check_eligibility(20_000.0, 1_000.0);

        assert!(result.eligible);
        assert!(result.benefit_amount > 0.0);
        assert_eq!(result.reasons.len(), 0);
    }

    #[test]
    fn test_snap_ineligible_income() {
        let benefit = BenefitPreset::snap_food_assistance();
        let result = benefit.check_eligibility(40_000.0, 1_000.0);

        assert!(!result.eligible);
        assert_eq!(result.benefit_amount, 0.0);
        assert!(!result.reasons.is_empty());
    }

    #[test]
    fn test_snap_ineligible_assets() {
        let benefit = BenefitPreset::snap_food_assistance();
        let result = benefit.check_eligibility(20_000.0, 5_000.0);

        assert!(!result.eligible);
        assert!(result.reasons.iter().any(|r| r.contains("Assets")));
    }

    #[test]
    fn test_social_security_retirement() {
        let benefit = BenefitPreset::social_security_retirement();
        assert_eq!(benefit.benefit_type, BenefitType::SocialSecurity);
        assert!(benefit.income_threshold > 100_000.0);
    }

    #[test]
    fn test_business_license() {
        let compliance = CompliancePreset::business_license();
        assert_eq!(compliance.compliance_type, ComplianceType::Licensing);
        assert_eq!(compliance.required_actions.len(), 2);
        assert_eq!(compliance.renewal_period, Some(365));
    }

    #[test]
    fn test_gdpr_compliance() {
        let compliance = CompliancePreset::gdpr_compliance();
        assert_eq!(compliance.compliance_type, ComplianceType::DataPrivacy);
        assert!(compliance.required_actions.len() >= 3);
    }

    #[test]
    fn test_compliance_cost_one_year() {
        let compliance = CompliancePreset::business_license();
        let cost = compliance.total_cost(365);

        assert!(cost.one_time_costs > 0.0);
        assert!(cost.recurring_costs > 0.0);
        assert_eq!(cost.total_costs, cost.one_time_costs + cost.recurring_costs);
    }

    #[test]
    fn test_compliance_cost_multi_year() {
        let compliance = CompliancePreset::business_license();
        let cost_1year = compliance.total_cost(365);
        let cost_2year = compliance.total_cost(730);

        // Recurring costs should increase over time
        assert!(cost_2year.recurring_costs > cost_1year.recurring_costs);
        assert!(cost_2year.total_costs > cost_1year.total_costs);
    }

    #[test]
    fn test_environmental_permit() {
        let compliance = CompliancePreset::environmental_permit();
        assert_eq!(compliance.compliance_type, ComplianceType::Environmental);
        assert!(compliance.renewal_period.unwrap() > 365);
    }

    #[test]
    fn test_tax_credit_phase_out() {
        let credit =
            TaxCredit::new("Test Credit".to_string(), 1000.0, true).with_phase_out(50_000.0);

        assert_eq!(credit.phase_out_threshold, Some(50_000.0));
    }

    // Court Case Prediction Tests

    #[test]
    fn test_civil_contract_dispute() {
        let case = CourtCasePreset::civil_contract_dispute();
        assert_eq!(case.court_level, CourtLevel::Trial);
        assert_eq!(case.case_factors.len(), 4);
        assert_eq!(case.precedents.len(), 2);
        assert_eq!(case.base_probability, 0.50);
    }

    #[test]
    fn test_criminal_case() {
        let case = CourtCasePreset::criminal_case();
        assert_eq!(case.court_level, CourtLevel::Trial);
        assert_eq!(case.case_factors.len(), 4);
        assert_eq!(case.base_probability, 0.60);
    }

    #[test]
    fn test_outcome_prediction_basic() {
        let case = CourtCasePreset::civil_contract_dispute();
        let factors = vec![
            ("Written Contract Evidence".to_string(), 1.0),
            ("Email Documentation".to_string(), 0.8),
        ];

        let prediction = case.predict_outcome(&factors);
        assert!(prediction.plaintiff_win_probability >= 0.0);
        assert!(prediction.plaintiff_win_probability <= 1.0);
        assert!(prediction.defendant_win_probability >= 0.0);
        assert!(prediction.defendant_win_probability <= 1.0);
        assert_eq!(
            prediction.plaintiff_win_probability + prediction.defendant_win_probability,
            1.0
        );
        assert!(prediction.confidence > 0.0);
        assert_eq!(prediction.factor_contributions.len(), 2);
    }

    #[test]
    fn test_outcome_prediction_with_precedents() {
        let case = CourtCasePreset::civil_contract_dispute();
        let factors = vec![("Written Contract Evidence".to_string(), 0.5)];

        let prediction = case.predict_outcome(&factors);
        // Should have precedent impact
        assert!(prediction.precedent_impact.abs() > 0.0);
    }

    #[test]
    fn test_outcome_prediction_strong_evidence() {
        let case = CourtCasePreset::civil_contract_dispute();
        let strong_factors = vec![
            ("Written Contract Evidence".to_string(), 1.0),
            ("Email Documentation".to_string(), 1.0),
            ("Witness Testimony".to_string(), 1.0),
            ("Legal Representation Quality".to_string(), 1.0),
        ];

        let prediction = case.predict_outcome(&strong_factors);
        // With all strong factors, plaintiff probability should be high
        assert!(prediction.plaintiff_win_probability > 0.6);
        assert!(prediction.confidence > 0.8); // High confidence with all factors
    }

    #[test]
    fn test_outcome_prediction_weak_evidence() {
        let case = CourtCasePreset::civil_contract_dispute();
        let weak_factors = vec![
            ("Written Contract Evidence".to_string(), 0.1),
            ("Email Documentation".to_string(), 0.1),
        ];

        let prediction = case.predict_outcome(&weak_factors);
        // With weak factors, probability should be moderate to low
        assert!(prediction.plaintiff_win_probability < 0.8);
    }

    #[test]
    fn test_outcome_prediction_confidence() {
        let case = CourtCasePreset::civil_contract_dispute();

        // Few factors = lower confidence
        let few_factors = vec![("Written Contract Evidence".to_string(), 1.0)];
        let pred_few = case.predict_outcome(&few_factors);

        // All factors = higher confidence
        let all_factors = vec![
            ("Written Contract Evidence".to_string(), 1.0),
            ("Email Documentation".to_string(), 1.0),
            ("Witness Testimony".to_string(), 1.0),
            ("Legal Representation Quality".to_string(), 1.0),
        ];
        let pred_all = case.predict_outcome(&all_factors);

        assert!(pred_all.confidence > pred_few.confidence);
    }

    #[test]
    fn test_case_factor_creation() {
        let factor = CaseFactor::new("Test Factor".to_string(), FactorType::Evidence, 0.25, 0.50);

        assert_eq!(factor.name, "Test Factor");
        assert_eq!(factor.factor_type, FactorType::Evidence);
        assert_eq!(factor.impact, 0.25);
        assert_eq!(factor.weight, 0.50);
    }

    #[test]
    fn test_precedent_creation() {
        let precedent = Precedent::new("Test v. Case".to_string(), 2023, 0.90, true, true);

        assert_eq!(precedent.case_name, "Test v. Case");
        assert_eq!(precedent.year, 2023);
        assert_eq!(precedent.similarity, 0.90);
        assert!(precedent.favors_plaintiff);
        assert!(precedent.binding);
    }

    // Legislative Forecasting Tests

    #[test]
    fn test_us_congress() {
        let congress = LegislativePreset::us_congress();
        assert_eq!(congress.legislative_level, LegislativeLevel::Federal);
        assert_eq!(congress.total_legislators, 535);
        assert_eq!(congress.party_composition.len(), 2);
        assert_eq!(congress.voting_patterns.len(), 4);
    }

    #[test]
    fn test_state_legislature() {
        let legislature = LegislativePreset::state_legislature(100);
        assert_eq!(legislature.legislative_level, LegislativeLevel::State);
        assert_eq!(legislature.total_legislators, 100);
        assert_eq!(legislature.party_composition.len(), 2);
    }

    #[test]
    fn test_bill_creation() {
        let bill = Bill::new(
            "HR-1234".to_string(),
            "Healthcare".to_string(),
            Majority::Simple,
        );

        assert_eq!(bill.name, "HR-1234");
        assert_eq!(bill.issue_area, "Healthcare");
        assert_eq!(bill.required_majority, Majority::Simple);
        assert_eq!(bill.party_positions.len(), 0);
    }

    #[test]
    fn test_bill_with_party_positions() {
        let bill = Bill::new(
            "HR-5678".to_string(),
            "Infrastructure".to_string(),
            Majority::Simple,
        )
        .with_party_position("Democratic".to_string(), 0.90)
        .with_party_position("Republican".to_string(), 0.60);

        assert_eq!(bill.party_positions.len(), 2);
        assert_eq!(bill.party_positions["Democratic"], 0.90);
        assert_eq!(bill.party_positions["Republican"], 0.60);
    }

    #[test]
    fn test_passage_forecast_simple_majority() {
        let congress = LegislativePreset::us_congress();
        let bill = Bill::new(
            "HR-1111".to_string(),
            "Infrastructure".to_string(),
            Majority::Simple,
        )
        .with_party_position("Democratic".to_string(), 0.95)
        .with_party_position("Republican".to_string(), 0.30);

        let forecast = congress.forecast_passage(&bill);

        assert!(forecast.passage_probability >= 0.0);
        assert!(forecast.passage_probability <= 1.0);
        assert!(forecast.expected_yes_votes > 0);
        assert!(forecast.required_votes > 0);
        assert!(forecast.estimated_days_to_vote > 0);
        assert!(forecast.confidence > 0.0);
    }

    #[test]
    fn test_passage_forecast_supermajority() {
        let congress = LegislativePreset::us_congress();
        let bill = Bill::new(
            "HR-2222".to_string(),
            "Tax Reform".to_string(),
            Majority::TwoThirds,
        )
        .with_party_position("Democratic".to_string(), 0.50)
        .with_party_position("Republican".to_string(), 0.40);

        let forecast = congress.forecast_passage(&bill);

        // Two-thirds majority is harder to achieve
        let required_two_thirds = (535.0 * 0.67) as usize;
        assert_eq!(forecast.required_votes, required_two_thirds);
    }

    #[test]
    fn test_passage_forecast_with_historical_pattern() {
        let congress = LegislativePreset::us_congress();
        let bill = Bill::new(
            "HR-3333".to_string(),
            "Healthcare".to_string(),
            Majority::Simple,
        )
        .with_party_position("Democratic".to_string(), 0.80)
        .with_party_position("Republican".to_string(), 0.20);

        let forecast = congress.forecast_passage(&bill);

        // Healthcare has historical passage rate and timing in the preset
        assert!(forecast.estimated_days_to_vote > 0);
        // Confidence should be higher when historical pattern exists
        assert!(forecast.confidence > 0.0);
    }

    #[test]
    fn test_passage_forecast_bipartisan_support() {
        let congress = LegislativePreset::us_congress();
        let bill = Bill::new(
            "HR-4444".to_string(),
            "Defense".to_string(),
            Majority::Simple,
        )
        .with_party_position("Democratic".to_string(), 0.85)
        .with_party_position("Republican".to_string(), 0.90);

        let forecast = congress.forecast_passage(&bill);

        // Strong bipartisan support should result in high passage probability
        assert!(forecast.passage_probability > 0.7);
    }

    #[test]
    fn test_passage_forecast_no_support() {
        let congress = LegislativePreset::us_congress();
        let bill = Bill::new(
            "HR-5555".to_string(),
            "Tax Reform".to_string(),
            Majority::Simple,
        )
        .with_party_position("Democratic".to_string(), 0.10)
        .with_party_position("Republican".to_string(), 0.05);

        let forecast = congress.forecast_passage(&bill);

        // Very low support should result in low passage probability
        assert!(forecast.passage_probability < 0.5);
    }

    #[test]
    fn test_party_composition_creation() {
        let party = PartyComposition::new("Test Party".to_string(), 100, 0.75);

        assert_eq!(party.party_name, "Test Party");
        assert_eq!(party.seats, 100);
        assert_eq!(party.historical_support_rate, 0.75);
    }

    #[test]
    fn test_voting_pattern_creation() {
        let pattern = VotingPattern::new("Education".to_string(), 0.65, 120);

        assert_eq!(pattern.issue_area, "Education");
        assert_eq!(pattern.passage_rate, 0.65);
        assert_eq!(pattern.avg_days_to_passage, 120);
    }

    #[test]
    fn test_three_fifths_majority() {
        let congress = LegislativePreset::us_congress();
        let bill = Bill::new(
            "HR-6666".to_string(),
            "Budget".to_string(),
            Majority::ThreeFifths,
        )
        .with_party_position("Democratic".to_string(), 0.70)
        .with_party_position("Republican".to_string(), 0.50);

        let forecast = congress.forecast_passage(&bill);

        // Three-fifths majority requires 60%
        let required_three_fifths = (535.0 * 0.60) as usize;
        assert_eq!(forecast.required_votes, required_three_fifths);
    }
}
