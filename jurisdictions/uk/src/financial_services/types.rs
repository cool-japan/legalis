//! Financial Services Types (Financial Services and Markets Act 2000)
//!
//! This module provides types for UK financial services regulation under FSMA 2000
//! and Financial Conduct Authority (FCA) rules.

#![allow(missing_docs)]

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// FCA authorization status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FcaAuthorization {
    /// FCA firm reference number (FRN)
    pub firm_reference_number: String,

    /// Firm name
    pub firm_name: String,

    /// Authorization status
    pub status: AuthorizationStatus,

    /// Date authorized
    pub authorization_date: NaiveDate,

    /// Regulated activities (permissions)
    pub regulated_activities: Vec<RegulatedActivity>,

    /// Passporting rights (pre-Brexit or equivalence)
    pub passporting_rights: Vec<PassportingRight>,
}

/// FCA authorization status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorizationStatus {
    /// Authorized and able to conduct regulated activities
    Authorized,

    /// Authorization suspended
    Suspended,

    /// Authorization withdrawn
    Withdrawn,

    /// Application pending
    Pending,

    /// Authorized with restrictions
    RestrictedAuthorization,
}

/// Regulated activities under FSMA 2000 (Regulated Activities Order 2001)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulatedActivity {
    /// Accepting deposits (RAO Article 5)
    AcceptingDeposits,

    /// Dealing in investments as principal (RAO Article 14)
    DealingInInvestmentsPrincipal,

    /// Dealing in investments as agent (RAO Article 21)
    DealingInInvestmentsAgent,

    /// Arranging deals in investments (RAO Article 25)
    ArrangingDeals,

    /// Managing investments (RAO Article 37)
    ManagingInvestments,

    /// Safeguarding and administering investments (RAO Article 40)
    SafeguardingInvestments,

    /// Advising on investments (RAO Article 53)
    AdvisingOnInvestments { investment_type: InvestmentType },

    /// Advising on pension transfers (RAO Article 53E)
    AdvisingOnPensionTransfers,

    /// Operating a multilateral trading facility (MTF)
    OperatingMtf,

    /// Operating an organized trading facility (OTF)
    OperatingOtf,

    /// Providing credit (consumer credit)
    ProvidingCredit,

    /// Insurance mediation
    InsuranceMediation,

    /// Mortgage lending
    MortgageLending,

    /// Mortgage administration
    MortgageAdministration,
}

/// Investment types for regulated activities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestmentType {
    /// Shares (equities)
    Shares,

    /// Bonds (debt securities)
    Bonds,

    /// Derivatives
    Derivatives,

    /// Units in collective investment schemes
    CollectiveInvestmentSchemes,

    /// Pension schemes
    PensionSchemes,

    /// Insurance contracts
    InsuranceContracts,
}

/// Passporting rights (EEA passporting - pre-Brexit or equivalence)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassportingRight {
    pub country: String,
    pub activities: Vec<RegulatedActivity>,
    pub status: PassportingStatus,
}

/// Passporting status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassportingStatus {
    /// Active passporting right
    Active,

    /// Suspended
    Suspended,

    /// Withdrawn (post-Brexit)
    Withdrawn,

    /// Temporary permission regime (post-Brexit)
    TemporaryPermission,
}

/// FCA's 11 Principles for Businesses (PRIN)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinciplesCompliance {
    /// Principle 1: Integrity
    pub integrity: PrincipleCompliance,

    /// Principle 2: Skill, care and diligence
    pub skill_care_diligence: PrincipleCompliance,

    /// Principle 3: Management and control
    pub management_control: PrincipleCompliance,

    /// Principle 4: Financial prudence
    pub financial_prudence: PrincipleCompliance,

    /// Principle 5: Market conduct
    pub market_conduct: PrincipleCompliance,

    /// Principle 6: Customers' interests
    pub customers_interests: PrincipleCompliance,

    /// Principle 7: Communications with clients
    pub communications: PrincipleCompliance,

    /// Principle 8: Conflicts of interest
    pub conflicts_of_interest: PrincipleCompliance,

    /// Principle 9: Customers: relationships of trust
    pub customer_trust: PrincipleCompliance,

    /// Principle 10: Clients' assets
    pub client_assets: PrincipleCompliance,

    /// Principle 11: Relations with regulators
    pub relations_with_regulators: PrincipleCompliance,
}

/// Compliance with a principle
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrincipleCompliance {
    pub compliant: bool,
    pub evidence: String,
    pub breach_details: Option<String>,
}

/// Client categorization under COBS (Conduct of Business Sourcebook)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientCategory {
    /// Retail client (highest protection)
    RetailClient,

    /// Professional client (per se or elective)
    ProfessionalClient { elective: bool },

    /// Eligible counterparty (lowest protection)
    EligibleCounterparty,
}

impl ClientCategory {
    /// Get level of regulatory protection (1 = highest, 3 = lowest)
    pub fn protection_level(&self) -> u8 {
        match self {
            Self::RetailClient => 1,
            Self::ProfessionalClient { .. } => 2,
            Self::EligibleCounterparty => 3,
        }
    }

    /// Check if suitability assessment required (COBS 9)
    pub fn requires_suitability_assessment(&self) -> bool {
        matches!(self, Self::RetailClient)
    }

    /// Check if appropriateness assessment required (COBS 10)
    pub fn requires_appropriateness_assessment(&self) -> bool {
        matches!(self, Self::RetailClient | Self::ProfessionalClient { .. })
    }

    /// Check if best execution applies (COBS 11)
    pub fn requires_best_execution(&self) -> bool {
        !matches!(self, Self::EligibleCounterparty)
    }
}

/// Suitability assessment (COBS 9)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuitabilityAssessment {
    pub client_name: String,
    pub client_category: ClientCategory,
    pub assessment_date: NaiveDate,

    /// Knowledge and experience
    pub knowledge_experience: KnowledgeExperience,

    /// Financial situation
    pub financial_situation: FinancialSituation,

    /// Investment objectives
    pub investment_objectives: InvestmentObjectives,

    /// Recommendation
    pub recommendation: Option<InvestmentRecommendation>,

    /// Suitability determination
    pub suitable: bool,

    /// Reasons for recommendation
    pub reasons: String,
}

/// Client knowledge and experience
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeExperience {
    /// Types of instruments client is familiar with
    pub familiar_instruments: Vec<InvestmentType>,

    /// Years of investment experience
    pub years_experience: u32,

    /// Education level (finance-related)
    pub financial_education: EducationLevel,

    /// Professional experience in financial sector
    pub professional_experience: bool,
}

/// Education level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EducationLevel {
    None,
    Secondary,
    Undergraduate,
    Graduate,
    Professional, // CFA, FRM, etc.
}

/// Financial situation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialSituation {
    /// Regular income (GBP per year)
    pub regular_income_gbp: f64,

    /// Net assets (excluding primary residence)
    pub net_assets_gbp: f64,

    /// Source of funds
    pub source_of_funds: String,

    /// Financial commitments (loans, mortgages, etc.)
    pub financial_commitments_gbp: f64,

    /// Amount available for investment
    pub investment_amount_gbp: f64,

    /// Can afford to lose investment
    pub can_afford_loss: bool,
}

/// Investment objectives
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvestmentObjectives {
    /// Primary objective
    pub primary_objective: InvestmentObjective,

    /// Risk tolerance
    pub risk_tolerance: RiskTolerance,

    /// Investment time horizon
    pub time_horizon_years: u32,

    /// Need for liquidity
    pub liquidity_needs: LiquidityNeeds,
}

/// Investment objective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestmentObjective {
    /// Capital preservation
    CapitalPreservation,

    /// Income generation
    Income,

    /// Growth
    Growth,

    /// Speculation
    Speculation,
}

/// Risk tolerance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskTolerance {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Liquidity needs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiquidityNeeds {
    Immediate,   // Need access within days
    ShortTerm,   // Within 1 year
    MediumTerm,  // 1-5 years
    LongTerm,    // 5+ years
    NoLiquidity, // Locked in investments acceptable
}

/// Investment recommendation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestmentRecommendation {
    pub product_name: String,
    pub investment_type: InvestmentType,
    pub amount_gbp: f64,
    pub risk_rating: RiskRating,
    pub expected_return_percent: f64,
    pub charges_percent: f64,
}

/// Risk rating for investment products
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskRating {
    Low,        // 1-2
    MediumLow,  // 3
    Medium,     // 4-5
    MediumHigh, // 6
    High,       // 7
}

impl RiskRating {
    /// Check if risk rating matches client risk tolerance
    pub fn matches_tolerance(&self, tolerance: RiskTolerance) -> bool {
        matches!(
            (self, tolerance),
            (RiskRating::Low, RiskTolerance::VeryLow | RiskTolerance::Low)
                | (
                    RiskRating::MediumLow,
                    RiskTolerance::Low | RiskTolerance::Medium
                )
                | (RiskRating::Medium, RiskTolerance::Medium)
                | (
                    RiskRating::MediumHigh,
                    RiskTolerance::Medium | RiskTolerance::High
                )
                | (
                    RiskRating::High,
                    RiskTolerance::High | RiskTolerance::VeryHigh
                )
        )
    }
}

/// Client assets protection (CASS - Client Assets Sourcebook)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientAssetsProtection {
    /// Client money held (segregated)
    pub client_money_gbp: f64,

    /// Client assets held (custody)
    pub client_assets_value_gbp: f64,

    /// Segregation status
    pub segregated: bool,

    /// Trust arrangement in place
    pub trust_arrangement: bool,

    /// Daily reconciliation performed
    pub daily_reconciliation: bool,

    /// External audit of CASS compliance
    pub cass_audit_date: Option<NaiveDate>,
}

/// Financial promotion (FSMA Section 21)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinancialPromotion {
    pub content: String,
    pub medium: PromotionMedium,
    pub target_audience: PromotionAudience,
    pub promotion_date: NaiveDate,

    /// Contains risk warning
    pub risk_warning_included: bool,

    /// Approved by authorized person
    pub approved_by_authorized_person: bool,

    /// FCA approval (if required)
    pub fca_approval: Option<String>,

    /// Fair, clear and not misleading (COBS 4)
    pub fair_clear_not_misleading: bool,
}

/// Promotion medium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromotionMedium {
    Print,
    Television,
    Radio,
    Online,
    SocialMedia,
    Email,
    DirectMail,
    Telephone,
}

/// Promotion target audience
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromotionAudience {
    /// Retail clients (general public)
    RetailClients,

    /// Professional clients only
    ProfessionalClients,

    /// High net worth individuals (certified/self-certified)
    HighNetWorth,

    /// Sophisticated investors
    SophisticatedInvestors,
}

/// Market abuse (Market Abuse Regulation - UK MAR)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketAbuseReport {
    pub report_id: String,
    pub report_date: NaiveDateTime,
    pub abuse_type: MarketAbuseType,
    pub security_identifier: String,
    pub description: String,
    pub reported_to_fca: bool,
}

/// Types of market abuse
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketAbuseType {
    /// Insider dealing
    InsiderDealing,

    /// Unlawful disclosure of inside information
    UnlawfulDisclosure,

    /// Market manipulation
    MarketManipulation,

    /// Benchmark manipulation (LIBOR, etc.)
    BenchmarkManipulation,
}

/// Senior Managers and Certification Regime (SM&CR)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeniorManager {
    pub name: String,
    pub function: SeniorManagementFunction,
    pub approval_date: NaiveDate,
    pub statement_of_responsibilities: String,

    /// Regulatory reference obtained
    pub regulatory_reference_obtained: bool,

    /// Fit and proper assessment
    pub fit_and_proper: bool,
}

/// Senior Management Functions (SMFs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeniorManagementFunction {
    /// SMF1: Chief Executive
    ChiefExecutive,

    /// SMF2: Chief Finance Officer
    ChiefFinanceOfficer,

    /// SMF3: Executive Director
    ExecutiveDirector,

    /// SMF4: Chief Risk Officer
    ChiefRiskOfficer,

    /// SMF5: Head of Internal Audit
    HeadOfInternalAudit,

    /// SMF16: Compliance Oversight
    ComplianceOversight,

    /// SMF17: Money Laundering Reporting Officer (MLRO)
    MoneyLaunderingReportingOfficer,

    /// SMF24: Chief Operations Officer
    ChiefOperationsOfficer,
}

/// Best execution (COBS 11)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BestExecutionPolicy {
    pub policy_date: NaiveDate,

    /// Execution factors considered (price, costs, speed, likelihood, etc.)
    pub factors: Vec<ExecutionFactor>,

    /// Execution venues
    pub venues: Vec<String>,

    /// Monitoring and review frequency
    pub review_frequency_months: u32,

    /// Policy published to clients
    pub published_to_clients: bool,
}

/// Execution factors for best execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionFactor {
    Price,
    Costs,
    Speed,
    LikelihoodOfExecution,
    LikelihoodOfSettlement,
    Size,
    Nature,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_category_protection_levels() {
        assert_eq!(ClientCategory::RetailClient.protection_level(), 1);
        assert_eq!(
            ClientCategory::ProfessionalClient { elective: false }.protection_level(),
            2
        );
        assert_eq!(ClientCategory::EligibleCounterparty.protection_level(), 3);
    }

    #[test]
    fn test_client_category_requirements() {
        let retail = ClientCategory::RetailClient;
        assert!(retail.requires_suitability_assessment());
        assert!(retail.requires_appropriateness_assessment());
        assert!(retail.requires_best_execution());

        let professional = ClientCategory::ProfessionalClient { elective: false };
        assert!(!professional.requires_suitability_assessment());
        assert!(professional.requires_appropriateness_assessment());
        assert!(professional.requires_best_execution());

        let counterparty = ClientCategory::EligibleCounterparty;
        assert!(!counterparty.requires_suitability_assessment());
        assert!(!counterparty.requires_appropriateness_assessment());
        assert!(!counterparty.requires_best_execution());
    }

    #[test]
    fn test_risk_rating_matches_tolerance() {
        assert!(RiskRating::Low.matches_tolerance(RiskTolerance::VeryLow));
        assert!(RiskRating::Low.matches_tolerance(RiskTolerance::Low));
        assert!(!RiskRating::Low.matches_tolerance(RiskTolerance::High));

        assert!(RiskRating::High.matches_tolerance(RiskTolerance::High));
        assert!(RiskRating::High.matches_tolerance(RiskTolerance::VeryHigh));
        assert!(!RiskRating::High.matches_tolerance(RiskTolerance::Low));
    }
}
