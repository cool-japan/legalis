//! Company Insolvency
//!
//! This module implements corporate insolvency under Insolvency Act 1986.
//!
//! ## Insolvency Tests
//!
//! ### Cash Flow Test (s.123(1)(e))
//! Company is unable to pay its debts as they fall due.
//!
//! ### Balance Sheet Test (s.123(2))
//! Value of company's assets is less than the amount of its liabilities,
//! taking into account contingent and prospective liabilities.
//!
//! ## Administration (IA 1986 Sch B1)
//!
//! ### Purpose (Para 3)
//! - Rescue company as going concern
//! - Better result for creditors than winding up
//! - Realizing property for secured/preferential creditors
//!
//! ### Entry Routes
//! - Court appointment (Para 11)
//! - Out-of-court appointment by floating charge holder (Para 14)
//! - Out-of-court appointment by company/directors (Para 22)
//!
//! ## Liquidation
//!
//! ### Voluntary (IA 1986 Part IV)
//! - Members' voluntary: solvent (s.89 declaration)
//! - Creditors' voluntary: insolvent (s.90)
//!
//! ### Compulsory (IA 1986 Part IV)
//! - Court order on petition (s.122)
//! - Grounds include: unable to pay debts, just and equitable
//!
//! ## Wrongful Trading (s.214)
//!
//! Director liable if:
//! - Company goes into insolvent liquidation
//! - Director knew or ought to have known no reasonable prospect of avoiding
//! - Did not take every step to minimize loss to creditors
//!
//! ## Fraudulent Trading (s.213)
//!
//! Any person who was knowingly party to carrying on business with intent
//! to defraud creditors may be personally liable.
//!
//! ## Preferences and Transactions at Undervalue
//!
//! - Transaction at undervalue (s.238): within 2 years
//! - Preference (s.239): 6 months (2 years for connected persons)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// ============================================================================
// Insolvency Tests
// ============================================================================

/// Insolvency test type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InsolvencyTest {
    /// Cash flow test (s.123(1)(e))
    CashFlow,
    /// Balance sheet test (s.123(2))
    BalanceSheet,
}

impl InsolvencyTest {
    /// Get description with statutory reference
    pub fn description(&self) -> &'static str {
        match self {
            Self::CashFlow => {
                "Cash flow test (IA 1986 s.123(1)(e)): Company is unable to pay its debts \
                 as they fall due in the ordinary course of business."
            }
            Self::BalanceSheet => {
                "Balance sheet test (IA 1986 s.123(2)): Value of company's assets is less \
                 than amount of liabilities, including contingent and prospective liabilities."
            }
        }
    }
}

/// Cash flow insolvency analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CashFlowAnalysis {
    /// Current cash and equivalents
    pub cash_available: f64,
    /// Debts due within next period
    pub debts_due: f64,
    /// Period for debts (days)
    pub period_days: u32,
    /// Expected receipts in period
    pub expected_receipts: f64,
    /// Is there a statutory demand outstanding?
    pub statutory_demand_outstanding: bool,
    /// Has demand been unsatisfied for 21 days?
    pub demand_unsatisfied_21_days: bool,
    /// Is company cash flow insolvent?
    pub insolvent: bool,
    /// Analysis
    pub analysis: String,
}

impl CashFlowAnalysis {
    /// Analyze cash flow solvency
    pub fn analyze(
        cash: f64,
        debts_due: f64,
        expected_receipts: f64,
        statutory_demand: bool,
        demand_21_days: bool,
    ) -> Self {
        // Deemed insolvent if statutory demand unsatisfied for 21 days (s.123(1)(a))
        let deemed_insolvent = statutory_demand && demand_21_days;

        // Commercial cash flow test
        let available_funds = cash + expected_receipts;
        let commercial_insolvent = available_funds < debts_due;

        let insolvent = deemed_insolvent || commercial_insolvent;

        let analysis = if deemed_insolvent {
            "Company INSOLVENT (deemed). Statutory demand unsatisfied for 21+ days \
             (IA 1986 s.123(1)(a)). This creates irrebuttable presumption of insolvency."
                .to_string()
        } else if commercial_insolvent {
            format!(
                "Company INSOLVENT (cash flow). Available funds £{:.2} (cash £{:.2} + \
                 expected receipts £{:.2}) insufficient to meet debts of £{:.2} falling due. \
                 IA 1986 s.123(1)(e) satisfied.",
                available_funds, cash, expected_receipts, debts_due
            )
        } else {
            format!(
                "Company SOLVENT (cash flow). Available funds £{:.2} sufficient to meet \
                 debts of £{:.2} falling due.",
                available_funds, debts_due
            )
        };

        Self {
            cash_available: cash,
            debts_due,
            period_days: 30,
            expected_receipts,
            statutory_demand_outstanding: statutory_demand,
            demand_unsatisfied_21_days: demand_21_days,
            insolvent,
            analysis,
        }
    }
}

/// Balance sheet insolvency analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceSheetAnalysis {
    /// Total assets
    pub total_assets: f64,
    /// Total liabilities
    pub total_liabilities: f64,
    /// Contingent liabilities
    pub contingent_liabilities: f64,
    /// Prospective liabilities
    pub prospective_liabilities: f64,
    /// Net position
    pub net_position: f64,
    /// Is company balance sheet insolvent?
    pub insolvent: bool,
    /// Analysis
    pub analysis: String,
}

impl BalanceSheetAnalysis {
    /// Analyze balance sheet solvency
    pub fn analyze(assets: f64, liabilities: f64, contingent: f64, prospective: f64) -> Self {
        let total_liabilities = liabilities + contingent + prospective;
        let net_position = assets - total_liabilities;
        let insolvent = net_position < 0.0;

        let analysis = if insolvent {
            format!(
                "Company INSOLVENT (balance sheet). Assets £{:.2} less than total \
                 liabilities £{:.2} (actual £{:.2} + contingent £{:.2} + prospective £{:.2}). \
                 Net deficit: £{:.2}. IA 1986 s.123(2) satisfied.",
                assets, total_liabilities, liabilities, contingent, prospective, -net_position
            )
        } else {
            format!(
                "Company SOLVENT (balance sheet). Assets £{:.2} exceed liabilities £{:.2}. \
                 Net surplus: £{:.2}.",
                assets, total_liabilities, net_position
            )
        };

        Self {
            total_assets: assets,
            total_liabilities,
            contingent_liabilities: contingent,
            prospective_liabilities: prospective,
            net_position,
            insolvent,
            analysis,
        }
    }
}

// ============================================================================
// Administration
// ============================================================================

/// Administration purpose (Sch B1 Para 3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdministrationPurpose {
    /// Rescue company as going concern (Para 3(1)(a))
    RescueAsGoingConcern,
    /// Better result for creditors than winding up (Para 3(1)(b))
    BetterResultForCreditors,
    /// Realize property for secured/preferential creditors (Para 3(1)(c))
    RealizeForSecuredCreditors,
}

impl AdministrationPurpose {
    /// Get priority order
    pub fn priority(&self) -> u8 {
        match self {
            Self::RescueAsGoingConcern => 1,
            Self::BetterResultForCreditors => 2,
            Self::RealizeForSecuredCreditors => 3,
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::RescueAsGoingConcern => {
                "Primary objective: rescue company as going concern (Sch B1 Para 3(1)(a))"
            }
            Self::BetterResultForCreditors => {
                "Secondary objective: achieve better result for creditors than winding up \
                 (Sch B1 Para 3(1)(b)) - only if rescue not reasonably practicable"
            }
            Self::RealizeForSecuredCreditors => {
                "Tertiary objective: realize property for secured/preferential creditors \
                 (Sch B1 Para 3(1)(c)) - only if secondary purpose won't prejudice creditors"
            }
        }
    }
}

/// Administration entry route
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdministrationEntryRoute {
    /// Court appointment (Para 11)
    CourtAppointment,
    /// Floating charge holder (Para 14)
    FloatingChargeHolder,
    /// Company/directors (Para 22)
    CompanyOrDirectors,
}

impl AdministrationEntryRoute {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::CourtAppointment => {
                "Court appointment on application (Sch B1 Para 11). Court must be satisfied \
                 company is or likely to become unable to pay debts and administration \
                 likely to achieve purpose."
            }
            Self::FloatingChargeHolder => {
                "Out-of-court appointment by qualifying floating charge holder (Sch B1 Para 14). \
                 Requires floating charge created before 15 Sept 2003 or covers substantially \
                 whole of company's property."
            }
            Self::CompanyOrDirectors => {
                "Out-of-court appointment by company or directors (Sch B1 Para 22). \
                 Must give 5 business days notice to qualifying floating charge holders."
            }
        }
    }
}

/// Administration analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdministrationAnalysis {
    /// Entry route
    pub entry_route: AdministrationEntryRoute,
    /// Purpose
    pub purpose: AdministrationPurpose,
    /// Administrator name
    pub administrator: String,
    /// Appointment date
    pub appointment_date: NaiveDate,
    /// Is company insolvent or likely?
    pub insolvency_established: bool,
    /// Is purpose likely achievable?
    pub purpose_achievable: bool,
    /// Moratorium in effect
    pub moratorium: bool,
    /// Analysis
    pub analysis: String,
}

impl AdministrationAnalysis {
    /// Analyze administration appointment
    pub fn analyze(
        entry_route: AdministrationEntryRoute,
        purpose: AdministrationPurpose,
        administrator: &str,
        appointment_date: NaiveDate,
        insolvent: bool,
        purpose_achievable: bool,
    ) -> Self {
        let valid = insolvent && purpose_achievable;

        let analysis = if valid {
            format!(
                "Administration VALID. Entry via {:?}. Purpose: {:?}. Administrator: {}. \
                 Moratorium now in effect (Sch B1 Para 43) - creditors cannot enforce \
                 security or commence proceedings without court permission.",
                entry_route, purpose, administrator
            )
        } else if !insolvent {
            "Administration not available - company is not insolvent or likely to become \
             insolvent. Sch B1 Para 11(a) requirement not met."
                .to_string()
        } else {
            "Administration not available - purpose unlikely to be achieved. \
             Sch B1 Para 11(b) requirement not met."
                .to_string()
        };

        Self {
            entry_route,
            purpose,
            administrator: administrator.to_string(),
            appointment_date,
            insolvency_established: insolvent,
            purpose_achievable,
            moratorium: valid,
            analysis,
        }
    }
}

// ============================================================================
// Liquidation
// ============================================================================

/// Type of liquidation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiquidationType {
    /// Members' voluntary liquidation (solvent)
    MembersVoluntary,
    /// Creditors' voluntary liquidation (insolvent)
    CreditorsVoluntary,
    /// Compulsory liquidation (court order)
    Compulsory,
}

impl LiquidationType {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::MembersVoluntary => {
                "Members' voluntary liquidation (IA 1986 Part IV). Company is solvent - \
                 directors make statutory declaration (s.89) that company can pay debts \
                 within 12 months."
            }
            Self::CreditorsVoluntary => {
                "Creditors' voluntary liquidation (IA 1986 Part IV). Company is insolvent. \
                 Cannot make s.89 declaration. Creditors' meeting must be held."
            }
            Self::Compulsory => {
                "Compulsory liquidation by court order (IA 1986 ss.117-162). \
                 Court makes winding-up order on petition."
            }
        }
    }
}

/// Grounds for compulsory liquidation (s.122)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompulsoryWindingUpGround {
    /// Unable to pay debts (s.122(1)(f))
    UnableToPayDebts,
    /// Just and equitable (s.122(1)(g))
    JustAndEquitable,
    /// Special resolution of company (s.122(1)(a))
    SpecialResolution,
    /// Public company not trading for year (s.122(1)(b))
    PublicCompanyNotTrading,
    /// Number of members below minimum (s.122(1)(c))
    MembersBelowMinimum,
    /// Company unable to pay costs (s.122(1)(d))
    UnableToPayCosts,
}

impl CompulsoryWindingUpGround {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::UnableToPayDebts => "Unable to pay debts (s.122(1)(f))",
            Self::JustAndEquitable => {
                "Just and equitable to wind up (s.122(1)(g)) - e.g., deadlock, \
                 loss of substratum, lack of probity"
            }
            Self::SpecialResolution => "Company passed special resolution (s.122(1)(a))",
            Self::PublicCompanyNotTrading => {
                "Public company not commenced business within year or suspended for year (s.122(1)(b))"
            }
            Self::MembersBelowMinimum => "Number of members below statutory minimum (s.122(1)(c))",
            Self::UnableToPayCosts => "Company unable to pay costs of liquidation (s.122(1)(d))",
        }
    }
}

/// Liquidation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiquidationAnalysis {
    /// Type of liquidation
    pub liquidation_type: LiquidationType,
    /// Ground (for compulsory)
    pub ground: Option<CompulsoryWindingUpGround>,
    /// Liquidator
    pub liquidator: Option<String>,
    /// Date of resolution/order
    pub commencement_date: NaiveDate,
    /// Statutory declaration (for MVL)
    pub statutory_declaration: Option<StatutoryDeclaration>,
    /// Analysis
    pub analysis: String,
}

/// Statutory declaration for MVL (s.89)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatutoryDeclaration {
    /// Date of declaration
    pub date: NaiveDate,
    /// Directors making declaration
    pub directors: Vec<String>,
    /// Can pay debts within 12 months
    pub can_pay_debts: bool,
    /// Attached statement of assets/liabilities
    pub statement_attached: bool,
    /// Is declaration valid?
    pub valid: bool,
}

impl LiquidationAnalysis {
    /// Analyze members' voluntary liquidation
    pub fn analyze_mvl(
        liquidator: &str,
        commencement_date: NaiveDate,
        declaration: StatutoryDeclaration,
    ) -> Self {
        let valid = declaration.valid;

        let analysis = if valid {
            format!(
                "Members' voluntary liquidation VALID. Directors made s.89 statutory \
                 declaration that company can pay debts within 12 months. Liquidator: {}. \
                 Commencement: {}.",
                liquidator, commencement_date
            )
        } else {
            "MVL INVALID. Statutory declaration requirements not met. If company actually \
             insolvent, directors may face liability for false declaration."
                .to_string()
        };

        Self {
            liquidation_type: LiquidationType::MembersVoluntary,
            ground: None,
            liquidator: Some(liquidator.to_string()),
            commencement_date,
            statutory_declaration: Some(declaration),
            analysis,
        }
    }

    /// Analyze compulsory liquidation
    pub fn analyze_compulsory(
        ground: CompulsoryWindingUpGround,
        commencement_date: NaiveDate,
        ground_established: bool,
    ) -> Self {
        let analysis = if ground_established {
            format!(
                "Compulsory liquidation - ground established: {:?}. \
                 Court will make winding-up order. Official Receiver becomes liquidator. \
                 Commencement relates back to presentation of petition.",
                ground
            )
        } else {
            format!(
                "Compulsory liquidation - ground {:?} NOT established. \
                 Petition likely to be dismissed.",
                ground
            )
        };

        Self {
            liquidation_type: LiquidationType::Compulsory,
            ground: Some(ground),
            liquidator: None,
            commencement_date,
            statutory_declaration: None,
            analysis,
        }
    }
}

// ============================================================================
// Wrongful Trading (s.214)
// ============================================================================

/// Wrongful trading analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WrongfulTradingAnalysis {
    /// Director being assessed
    pub director: String,
    /// Date company went into insolvent liquidation
    pub liquidation_date: NaiveDate,
    /// Date director knew (or should have known) no reasonable prospect
    pub knowledge_date: Option<NaiveDate>,
    /// Did director know no reasonable prospect?
    pub knew_no_prospect: bool,
    /// Did director take every step to minimize loss?
    pub minimized_loss: bool,
    /// Steps taken
    pub steps_taken: Vec<String>,
    /// Is director potentially liable?
    pub potentially_liable: bool,
    /// Analysis
    pub analysis: String,
}

impl WrongfulTradingAnalysis {
    /// Analyze wrongful trading liability
    pub fn analyze(
        director: &str,
        liquidation_date: NaiveDate,
        knowledge_date: Option<NaiveDate>,
        knew_no_prospect: bool,
        steps_taken: Vec<String>,
    ) -> Self {
        // Defense: took every step to minimize loss (s.214(3))
        let minimized_loss = !steps_taken.is_empty()
            && steps_taken.iter().any(|s| {
                s.to_lowercase().contains("minimize") || s.to_lowercase().contains("cease")
            });

        let potentially_liable = knew_no_prospect && !minimized_loss;

        let analysis = if !knew_no_prospect {
            format!(
                "Director {} NOT liable for wrongful trading. No evidence that director \
                 knew or ought to have known there was no reasonable prospect of avoiding \
                 insolvent liquidation (s.214(2)(b)).",
                director
            )
        } else if minimized_loss {
            format!(
                "Director {} NOT liable for wrongful trading. Although knew of no reasonable \
                 prospect, director took every step to minimize potential loss to creditors \
                 (s.214(3) defence). Steps: {}.",
                director,
                steps_taken.join("; ")
            )
        } else {
            format!(
                "Director {} potentially LIABLE for wrongful trading (s.214). \
                 (1) Company in insolvent liquidation; \
                 (2) Director knew or ought to have known no reasonable prospect of avoiding; \
                 (3) Did NOT take every step to minimize loss. \
                 May be required to contribute to company's assets.",
                director
            )
        };

        Self {
            director: director.to_string(),
            liquidation_date,
            knowledge_date,
            knew_no_prospect,
            minimized_loss,
            steps_taken,
            potentially_liable,
            analysis,
        }
    }
}

// ============================================================================
// Fraudulent Trading (s.213)
// ============================================================================

/// Fraudulent trading analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FraudulentTradingAnalysis {
    /// Person being assessed
    pub person: String,
    /// Intent to defraud established?
    pub intent_to_defraud: bool,
    /// Knowingly party to business?
    pub knowingly_party: bool,
    /// Evidence
    pub evidence: Vec<String>,
    /// Potentially liable?
    pub potentially_liable: bool,
    /// Analysis
    pub analysis: String,
}

impl FraudulentTradingAnalysis {
    /// Analyze fraudulent trading liability
    pub fn analyze(
        person: &str,
        intent_to_defraud: bool,
        knowingly_party: bool,
        evidence: Vec<String>,
    ) -> Self {
        let potentially_liable = intent_to_defraud && knowingly_party;

        let analysis = if potentially_liable {
            format!(
                "{} potentially LIABLE for fraudulent trading (s.213). \
                 Business carried on with intent to defraud creditors and {} was \
                 knowingly party. Evidence: {}. May be required to contribute to \
                 assets and criminal liability possible (s.993 CA 2006).",
                person,
                person,
                evidence.join("; ")
            )
        } else if !intent_to_defraud {
            format!(
                "{} NOT liable for fraudulent trading. Intent to defraud not established. \
                 s.213 requires actual intent - not mere negligence or unreasonable optimism.",
                person
            )
        } else {
            format!(
                "{} NOT liable for fraudulent trading. Not shown to be knowingly party \
                 to carrying on business with intent to defraud.",
                person
            )
        };

        Self {
            person: person.to_string(),
            intent_to_defraud,
            knowingly_party,
            evidence,
            potentially_liable,
            analysis,
        }
    }
}

// ============================================================================
// Voidable Transactions
// ============================================================================

/// Transaction at undervalue (s.238)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionAtUndervalue {
    /// Transaction description
    pub description: String,
    /// Value given by company
    pub value_given: f64,
    /// Value received by company
    pub value_received: f64,
    /// Transaction date
    pub transaction_date: NaiveDate,
    /// Liquidation/administration date
    pub insolvency_date: NaiveDate,
    /// Was company insolvent at time?
    pub company_insolvent_at_time: bool,
    /// Is transaction voidable?
    pub voidable: bool,
    /// Analysis
    pub analysis: String,
}

impl TransactionAtUndervalue {
    /// Analyze transaction at undervalue
    pub fn analyze(
        description: &str,
        value_given: f64,
        value_received: f64,
        transaction_date: NaiveDate,
        insolvency_date: NaiveDate,
        insolvent_at_time: bool,
    ) -> Self {
        // Within 2 years of insolvency (s.240(1)(a))
        let within_period = (insolvency_date - transaction_date).num_days() <= 730;

        // Significantly less value received
        let undervalue = value_received < value_given * 0.75; // Rough approximation

        let voidable = within_period && undervalue && insolvent_at_time;

        let analysis = if voidable {
            format!(
                "Transaction at undervalue - VOIDABLE (s.238). Company gave £{:.2} but \
                 received only £{:.2}. Within 2-year period. Company was insolvent at time. \
                 Court may restore position.",
                value_given, value_received
            )
        } else if !within_period {
            format!(
                "Transaction NOT voidable - outside 2-year relevant time (s.240). \
                 Transaction was {} days before insolvency.",
                (insolvency_date - transaction_date).num_days()
            )
        } else if !undervalue {
            "Transaction NOT voidable - company received reasonably equivalent value.".to_string()
        } else {
            "Transaction NOT voidable - company was not insolvent at time.".to_string()
        };

        Self {
            description: description.to_string(),
            value_given,
            value_received,
            transaction_date,
            insolvency_date,
            company_insolvent_at_time: insolvent_at_time,
            voidable,
            analysis,
        }
    }
}

/// Preference (s.239)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreferenceAnalysis {
    /// Creditor preferred
    pub creditor: String,
    /// Amount of preference
    pub amount: f64,
    /// Transaction date
    pub transaction_date: NaiveDate,
    /// Insolvency date
    pub insolvency_date: NaiveDate,
    /// Is creditor connected person?
    pub connected_person: bool,
    /// Was company influenced by desire to prefer?
    pub desire_to_prefer: bool,
    /// Is preference voidable?
    pub voidable: bool,
    /// Analysis
    pub analysis: String,
}

impl PreferenceAnalysis {
    /// Analyze preference
    pub fn analyze(
        creditor: &str,
        amount: f64,
        transaction_date: NaiveDate,
        insolvency_date: NaiveDate,
        connected: bool,
        desire_to_prefer: bool,
    ) -> Self {
        let days = (insolvency_date - transaction_date).num_days();

        // Relevant time: 6 months (2 years for connected persons) - s.240(1)(b)
        let within_period = if connected { days <= 730 } else { days <= 183 };

        // Connected person: desire to prefer presumed (s.239(6))
        let presumed_desire = connected || desire_to_prefer;

        let voidable = within_period && presumed_desire;

        let analysis = if voidable {
            format!(
                "Preference to {} of £{:.2} - VOIDABLE (s.239). Within relevant time \
                 ({} days{}). {}. Court may restore position.",
                creditor,
                amount,
                days,
                if connected { ", connected person" } else { "" },
                if connected {
                    "Desire to prefer presumed (s.239(6))"
                } else {
                    "Desire to prefer established"
                }
            )
        } else if !within_period {
            format!(
                "Preference NOT voidable - outside relevant time. {} days before insolvency \
                 (maximum {} for {} person).",
                days,
                if connected { 730 } else { 183 },
                if connected {
                    "connected"
                } else {
                    "unconnected"
                }
            )
        } else {
            "Preference NOT voidable - no desire to prefer established.".to_string()
        };

        Self {
            creditor: creditor.to_string(),
            amount,
            transaction_date,
            insolvency_date,
            connected_person: connected,
            desire_to_prefer: presumed_desire,
            voidable,
            analysis,
        }
    }
}

// ============================================================================
// Priority of Distributions
// ============================================================================

/// Creditor priority in distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreditorPriority {
    /// Fixed charge holders (highest priority)
    FixedCharge,
    /// Expenses of insolvency proceedings
    InsolvencyExpenses,
    /// Preferential creditors (employees, pension contributions)
    Preferential,
    /// Prescribed part for unsecured (s.176A)
    PrescribedPart,
    /// Floating charge holders
    FloatingCharge,
    /// Unsecured creditors
    Unsecured,
    /// Shareholders (lowest priority)
    Shareholders,
}

impl CreditorPriority {
    /// Get priority rank (1 = highest)
    pub fn rank(&self) -> u8 {
        match self {
            Self::FixedCharge => 1,
            Self::InsolvencyExpenses => 2,
            Self::Preferential => 3,
            Self::PrescribedPart => 4,
            Self::FloatingCharge => 5,
            Self::Unsecured => 6,
            Self::Shareholders => 7,
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::FixedCharge => "Fixed charge holders - paid from charged assets",
            Self::InsolvencyExpenses => "Expenses of the insolvency proceedings",
            Self::Preferential => {
                "Preferential creditors (IA 1986 Sch 6): employees' wages/holiday pay, \
                 pension scheme contributions"
            }
            Self::PrescribedPart => {
                "Prescribed part - up to £800,000 ring-fenced from floating charge \
                 for unsecured creditors (s.176A)"
            }
            Self::FloatingCharge => "Floating charge holders",
            Self::Unsecured => "Unsecured/trade creditors",
            Self::Shareholders => "Shareholders - only after all creditors paid in full",
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cash_flow_insolvent() {
        let analysis = CashFlowAnalysis::analyze(10_000.0, 50_000.0, 5_000.0, false, false);
        assert!(analysis.insolvent);
    }

    #[test]
    fn test_cash_flow_solvent() {
        let analysis = CashFlowAnalysis::analyze(50_000.0, 30_000.0, 10_000.0, false, false);
        assert!(!analysis.insolvent);
    }

    #[test]
    fn test_deemed_insolvent_statutory_demand() {
        let analysis = CashFlowAnalysis::analyze(100_000.0, 10_000.0, 0.0, true, true);
        assert!(analysis.insolvent);
        assert!(analysis.analysis.contains("deemed"));
    }

    #[test]
    fn test_balance_sheet_insolvent() {
        let analysis = BalanceSheetAnalysis::analyze(100_000.0, 80_000.0, 30_000.0, 10_000.0);
        assert!(analysis.insolvent);
        assert!(analysis.net_position < 0.0);
    }

    #[test]
    fn test_balance_sheet_solvent() {
        let analysis = BalanceSheetAnalysis::analyze(200_000.0, 100_000.0, 20_000.0, 10_000.0);
        assert!(!analysis.insolvent);
    }

    #[test]
    fn test_wrongful_trading_liable() {
        let analysis = WrongfulTradingAnalysis::analyze(
            "John Smith",
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            true,
            vec!["Continued trading".to_string()],
        );
        assert!(analysis.potentially_liable);
    }

    #[test]
    fn test_wrongful_trading_defense() {
        let analysis = WrongfulTradingAnalysis::analyze(
            "Jane Doe",
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            true,
            vec![
                "Ceased trading immediately".to_string(),
                "Took steps to minimize loss".to_string(),
            ],
        );
        assert!(!analysis.potentially_liable);
    }

    #[test]
    fn test_transaction_at_undervalue() {
        let analysis = TransactionAtUndervalue::analyze(
            "Sale of property to director",
            500_000.0,
            100_000.0,
            NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            true,
        );
        assert!(analysis.voidable);
    }

    #[test]
    fn test_preference_connected() {
        let analysis = PreferenceAnalysis::analyze(
            "Director's loan account",
            50_000.0,
            NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            true, // Connected person
            false,
        );
        assert!(analysis.voidable);
        assert!(analysis.desire_to_prefer); // Presumed for connected
    }

    #[test]
    fn test_creditor_priority_order() {
        assert!(CreditorPriority::FixedCharge.rank() < CreditorPriority::Preferential.rank());
        assert!(CreditorPriority::Preferential.rank() < CreditorPriority::Unsecured.rank());
        assert!(CreditorPriority::Unsecured.rank() < CreditorPriority::Shareholders.rank());
    }
}
