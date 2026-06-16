//! Insolvency, Restructuring and Dissolution Act 2018 (IRDA) - Type Definitions
//!
//! This module provides type-safe representations of Singapore's insolvency,
//! restructuring and dissolution framework under the Insolvency, Restructuring
//! and Dissolution Act 2018 (No. 40 of 2018), which came into operation on
//! 30 July 2020.
//!
//! ## Statutory Architecture
//!
//! The IRDA consolidated three previously separate regimes:
//! - Corporate winding up (formerly Companies Act (Cap. 50), Part X)
//! - Personal bankruptcy (formerly Bankruptcy Act (Cap. 20))
//! - Corporate rescue (judicial management and schemes of arrangement)
//!
//! ## Regimes Modelled
//!
//! - **Corporate winding up** (IRDA Part 8): compulsory (by the Court) and
//!   voluntary (members' or creditors').
//! - **Judicial management** (IRDA Part 7, ss. 88-113): a corporate rescue regime
//!   with a statutory moratorium.
//! - **Schemes of arrangement** (IRDA Part 5, ss. 64-72; sanction under s. 210):
//!   a court-sanctioned compromise with creditors, with super-priority rescue
//!   financing (s. 67) and the "cram-down" power (s. 70).
//! - **Bankruptcy** (IRDA Part 16): individual insolvency, with the Debt
//!   Repayment Scheme (Part 14) as an alternative for smaller debts.
//!
//! ## Money Representation
//!
//! All monetary amounts are stored as unsigned integer SGD **cents** (`u64`),
//! mirroring the banking module. For example, SGD 15,000 is stored as
//! `1_500_000` cents.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The prescribed minimum debt for a company statutory demand, in SGD cents.
///
/// IRDA s. 125(2)(a): a company is deemed unable to pay its debts where a
/// statutory demand for a sum exceeding the prescribed sum (SGD 15,000) remains
/// unsatisfied for 3 weeks.
pub const COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS: u64 = 15_000 * 100;

/// The number of days a statutory demand must remain unsatisfied (3 weeks).
///
/// IRDA s. 125(2)(a): "3 weeks" is treated as 21 days.
pub const STATUTORY_DEMAND_PERIOD_DAYS: u32 = 21;

/// The bankruptcy debt threshold, in SGD cents.
///
/// IRDA s. 311(1)(a): a creditor's bankruptcy application requires a liquidated
/// debt of at least SGD 15,000.
pub const BANKRUPTCY_DEBT_THRESHOLD_CENTS: u64 = 15_000 * 100;

/// The Debt Repayment Scheme (DRS) debt ceiling, in SGD cents.
///
/// IRDA s. 289: the DRS administered by the Official Assignee is available only
/// where the debtor's aggregate debts do not exceed SGD 150,000.
pub const DEBT_REPAYMENT_SCHEME_CEILING_CENTS: u64 = 150_000 * 100;

/// The scheme of arrangement value-approval threshold (75% in value).
///
/// IRDA s. 210(3AB): a scheme requires 75% in value of the creditors present and
/// voting in each class.
pub const SCHEME_VALUE_THRESHOLD_PERCENT: f64 = 75.0;

/// The automatic moratorium period on a scheme application, in days.
///
/// IRDA s. 64(1): an application for a moratorium triggers an automatic 30-day
/// moratorium pending the Court's determination.
pub const AUTOMATIC_MORATORIUM_DAYS: u32 = 30;

/// Mode by which a company is wound up under the IRDA.
///
/// IRDA Part 8 provides for winding up by the Court (compulsory) and voluntary
/// winding up, which may be either a members' or creditors' voluntary winding up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindingUpMode {
    /// Compulsory winding up by order of the Court (IRDA s. 124).
    CompulsoryByCourt,
    /// Members' voluntary winding up, where the directors make a declaration of
    /// solvency (IRDA s. 160, s. 161).
    MembersVoluntary,
    /// Creditors' voluntary winding up, where the directors cannot make a
    /// declaration of solvency (IRDA s. 160, s. 166).
    CreditorsVoluntary,
}

impl WindingUpMode {
    /// Returns the principal statute reference for this mode.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            WindingUpMode::CompulsoryByCourt => "IRDA s. 124",
            WindingUpMode::MembersVoluntary => "IRDA s. 161",
            WindingUpMode::CreditorsVoluntary => "IRDA s. 166",
        }
    }

    /// Returns a short description of this mode.
    pub fn description(&self) -> &'static str {
        match self {
            WindingUpMode::CompulsoryByCourt => {
                "Winding up by order of the Court on a ground in s. 125(1)"
            }
            WindingUpMode::MembersVoluntary => {
                "Voluntary winding up where directors declare the company solvent"
            }
            WindingUpMode::CreditorsVoluntary => {
                "Voluntary winding up where no declaration of solvency can be made"
            }
        }
    }

    /// Whether this mode requires a directors' declaration of solvency.
    pub fn requires_declaration_of_solvency(&self) -> bool {
        matches!(self, WindingUpMode::MembersVoluntary)
    }
}

/// A ground on which the Court may order a compulsory winding up.
///
/// IRDA s. 125(1) enumerates the grounds. The most commonly invoked is
/// paragraph (e) - that the company is unable to pay its debts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindingUpGround {
    /// s. 125(1)(a): the company has by special resolution resolved to be wound
    /// up by the Court.
    SpecialResolution,
    /// s. 125(1)(d): the company does not commence business within a year of
    /// incorporation, or suspends business for a whole year.
    BusinessNotCommencedOrSuspended,
    /// s. 125(1)(e): the company is unable to pay its debts (the most common
    /// ground).
    UnableToPayDebts,
    /// s. 125(1)(f): the directors have acted in their own interests rather than
    /// in the interests of the members as a whole, or in any manner unfair or
    /// unjust to other members.
    DirectorsActedInOwnInterests,
    /// s. 125(1)(g): an act or omission, or a resolution, is oppressive or
    /// unfairly prejudicial to members.
    OppressiveConduct,
    /// s. 125(1)(i): the Court is of the opinion that it is just and equitable
    /// that the company be wound up.
    JustAndEquitable,
}

impl WindingUpGround {
    /// Returns the precise statute reference for this ground.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            WindingUpGround::SpecialResolution => "IRDA s. 125(1)(a)",
            WindingUpGround::BusinessNotCommencedOrSuspended => "IRDA s. 125(1)(d)",
            WindingUpGround::UnableToPayDebts => "IRDA s. 125(1)(e)",
            WindingUpGround::DirectorsActedInOwnInterests => "IRDA s. 125(1)(f)",
            WindingUpGround::OppressiveConduct => "IRDA s. 125(1)(g)",
            WindingUpGround::JustAndEquitable => "IRDA s. 125(1)(i)",
        }
    }

    /// Returns a short description of this ground.
    pub fn description(&self) -> &'static str {
        match self {
            WindingUpGround::SpecialResolution => {
                "Company has resolved by special resolution to be wound up by the Court"
            }
            WindingUpGround::BusinessNotCommencedOrSuspended => {
                "Business not commenced within a year, or suspended for a whole year"
            }
            WindingUpGround::UnableToPayDebts => "Company is unable to pay its debts",
            WindingUpGround::DirectorsActedInOwnInterests => {
                "Directors have acted in their own interests, unfairly to members"
            }
            WindingUpGround::OppressiveConduct => {
                "Conduct oppressive or unfairly prejudicial to members"
            }
            WindingUpGround::JustAndEquitable => "It is just and equitable to wind up the company",
        }
    }
}

/// A deeming test for a company's inability to pay its debts.
///
/// IRDA s. 125(2) sets out when a company is deemed unable to pay its debts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InabilityToPayTest {
    /// s. 125(2)(a): an unsatisfied statutory demand for more than the prescribed
    /// sum, outstanding for 3 weeks.
    StatutoryDemand,
    /// s. 125(2)(b): execution or other process on a judgment returned
    /// unsatisfied in whole or in part.
    UnsatisfiedExecution,
    /// s. 125(2)(c): proof to the Court's satisfaction that the company is unable
    /// to pay its debts, taking into account contingent and prospective
    /// liabilities (the cash-flow and balance-sheet tests).
    ProvedToCourt,
}

impl InabilityToPayTest {
    /// Returns the precise statute reference for this test.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            InabilityToPayTest::StatutoryDemand => "IRDA s. 125(2)(a)",
            InabilityToPayTest::UnsatisfiedExecution => "IRDA s. 125(2)(b)",
            InabilityToPayTest::ProvedToCourt => "IRDA s. 125(2)(c)",
        }
    }

    /// Returns a short description of this test.
    pub fn description(&self) -> &'static str {
        match self {
            InabilityToPayTest::StatutoryDemand => {
                "Statutory demand for more than the prescribed sum unsatisfied for 3 weeks"
            }
            InabilityToPayTest::UnsatisfiedExecution => {
                "Execution on a judgment returned wholly or partly unsatisfied"
            }
            InabilityToPayTest::ProvedToCourt => {
                "Proved to the Court (cash-flow / balance-sheet), including contingent liabilities"
            }
        }
    }
}

/// A statutory demand served on a company (IRDA s. 125(2)(a)).
///
/// A statutory demand is a written demand requiring the company to pay a debt
/// exceeding the prescribed sum. If it remains unsatisfied for 3 weeks the
/// company is deemed unable to pay its debts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatutoryDemand {
    /// Identifier for the statutory demand.
    pub demand_id: String,
    /// Name of the creditor serving the demand.
    pub creditor_name: String,
    /// Name of the company on which the demand is served.
    pub company_name: String,
    /// The debt demanded, in SGD cents.
    pub debt_cents: u64,
    /// Date the demand was served.
    pub served_date: DateTime<Utc>,
    /// Number of days the demand has remained unsatisfied.
    pub days_unsatisfied: u32,
    /// Whether the debt is disputed by the company on substantial grounds.
    pub disputed: bool,
}

impl StatutoryDemand {
    /// Creates a new statutory demand.
    pub fn new(
        demand_id: impl Into<String>,
        creditor_name: impl Into<String>,
        company_name: impl Into<String>,
        debt_cents: u64,
    ) -> Self {
        Self {
            demand_id: demand_id.into(),
            creditor_name: creditor_name.into(),
            company_name: company_name.into(),
            debt_cents,
            served_date: Utc::now(),
            days_unsatisfied: 0,
            disputed: false,
        }
    }

    /// Sets the number of days the demand has remained unsatisfied.
    pub fn with_days_unsatisfied(mut self, days: u32) -> Self {
        self.days_unsatisfied = days;
        self
    }

    /// Marks the debt as disputed on substantial grounds.
    pub fn with_dispute(mut self, disputed: bool) -> Self {
        self.disputed = disputed;
        self
    }

    /// Sets the date the demand was served.
    pub fn with_served_date(mut self, served_date: DateTime<Utc>) -> Self {
        self.served_date = served_date;
        self
    }

    /// Whether the debt exceeds the prescribed sum (SGD 15,000).
    pub fn exceeds_prescribed_sum(&self) -> bool {
        self.debt_cents > COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS
    }

    /// Whether the 3-week (21-day) period has expired.
    pub fn period_expired(&self) -> bool {
        self.days_unsatisfied >= STATUTORY_DEMAND_PERIOD_DAYS
    }

    /// Converts the demanded debt to whole SGD.
    pub fn debt_in_sgd(&self) -> u64 {
        self.debt_cents / 100
    }
}

/// A petition or resolution to wind up a company.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindingUpPetition {
    /// Identifier for the petition.
    pub petition_id: String,
    /// Name of the company sought to be wound up.
    pub company_name: String,
    /// Mode of winding up.
    pub mode: WindingUpMode,
    /// Ground relied upon (for compulsory winding up).
    pub ground: Option<WindingUpGround>,
    /// Inability-to-pay test relied upon (if the ground is s. 125(1)(e)).
    pub inability_test: Option<InabilityToPayTest>,
    /// Whether the directors have made a declaration of solvency.
    pub declaration_of_solvency: bool,
    /// The statutory demand relied upon (if any).
    pub statutory_demand: Option<StatutoryDemand>,
    /// Date the petition was filed.
    pub filed_date: DateTime<Utc>,
}

impl WindingUpPetition {
    /// Creates a new winding up petition for a given mode.
    pub fn new(
        petition_id: impl Into<String>,
        company_name: impl Into<String>,
        mode: WindingUpMode,
    ) -> Self {
        Self {
            petition_id: petition_id.into(),
            company_name: company_name.into(),
            mode,
            ground: None,
            inability_test: None,
            declaration_of_solvency: false,
            statutory_demand: None,
            filed_date: Utc::now(),
        }
    }

    /// Sets the ground relied upon.
    pub fn with_ground(mut self, ground: WindingUpGround) -> Self {
        self.ground = Some(ground);
        self
    }

    /// Sets the inability-to-pay test relied upon.
    pub fn with_inability_test(mut self, test: InabilityToPayTest) -> Self {
        self.inability_test = Some(test);
        self
    }

    /// Records that a declaration of solvency has been made.
    pub fn with_declaration_of_solvency(mut self, made: bool) -> Self {
        self.declaration_of_solvency = made;
        self
    }

    /// Attaches a statutory demand.
    pub fn with_statutory_demand(mut self, demand: StatutoryDemand) -> Self {
        self.statutory_demand = Some(demand);
        self
    }
}

/// A statutory purpose of judicial management (IRDA s. 89(1)).
///
/// The Court (or creditors out of court) may place a company under judicial
/// management where there is a reasonable probability of achieving at least one
/// of these purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JudicialManagementPurpose {
    /// s. 89(1)(a): the survival of the company, or the whole or part of its
    /// undertaking, as a going concern.
    SurvivalAsGoingConcern,
    /// s. 89(1)(b): the approval of a compromise or arrangement (scheme) under
    /// s. 210.
    ApprovalOfScheme,
    /// s. 89(1)(c): a more advantageous realisation of the company's assets than
    /// on a winding up.
    MoreAdvantageousRealisation,
}

impl JudicialManagementPurpose {
    /// Returns the precise statute reference for this purpose.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            JudicialManagementPurpose::SurvivalAsGoingConcern => "IRDA s. 89(1)(a)",
            JudicialManagementPurpose::ApprovalOfScheme => "IRDA s. 89(1)(b)",
            JudicialManagementPurpose::MoreAdvantageousRealisation => "IRDA s. 89(1)(c)",
        }
    }

    /// Returns a short description of this purpose.
    pub fn description(&self) -> &'static str {
        match self {
            JudicialManagementPurpose::SurvivalAsGoingConcern => {
                "Survival of the company (or its undertaking) as a going concern"
            }
            JudicialManagementPurpose::ApprovalOfScheme => {
                "Approval of a compromise or arrangement under s. 210"
            }
            JudicialManagementPurpose::MoreAdvantageousRealisation => {
                "More advantageous realisation of assets than on a winding up"
            }
        }
    }
}

/// An application to place a company under judicial management.
///
/// IRDA s. 89(1) requires (i) that the company is or is likely to become unable
/// to pay its debts (the insolvency limb) and (ii) a reasonable probability of
/// achieving at least one statutory purpose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JudicialManagementApplication {
    /// Identifier for the application.
    pub application_id: String,
    /// Name of the company.
    pub company_name: String,
    /// Whether the application is made by creditors' resolution out of court
    /// (rather than by Court order) - introduced by the IRDA.
    pub out_of_court: bool,
    /// Insolvency limb: the company is, or is likely to become, unable to pay its
    /// debts (IRDA s. 89(1) opening words).
    pub is_or_likely_unable_to_pay: bool,
    /// The statutory purposes that are reasonably likely to be achieved.
    pub purposes: Vec<JudicialManagementPurpose>,
    /// Proposed identity of the judicial manager.
    pub proposed_judicial_manager: Option<String>,
    /// Date the application was filed.
    pub filed_date: DateTime<Utc>,
}

impl JudicialManagementApplication {
    /// Creates a new judicial management application.
    pub fn new(application_id: impl Into<String>, company_name: impl Into<String>) -> Self {
        Self {
            application_id: application_id.into(),
            company_name: company_name.into(),
            out_of_court: false,
            is_or_likely_unable_to_pay: false,
            purposes: Vec::new(),
            proposed_judicial_manager: None,
            filed_date: Utc::now(),
        }
    }

    /// Sets whether the application is made out of court by creditors' resolution.
    pub fn with_out_of_court(mut self, out_of_court: bool) -> Self {
        self.out_of_court = out_of_court;
        self
    }

    /// Sets the insolvency limb.
    pub fn with_insolvency_limb(mut self, satisfied: bool) -> Self {
        self.is_or_likely_unable_to_pay = satisfied;
        self
    }

    /// Adds a statutory purpose.
    pub fn add_purpose(&mut self, purpose: JudicialManagementPurpose) {
        if !self.purposes.contains(&purpose) {
            self.purposes.push(purpose);
        }
    }

    /// Builder-style variant of [`Self::add_purpose`].
    pub fn with_purpose(mut self, purpose: JudicialManagementPurpose) -> Self {
        self.add_purpose(purpose);
        self
    }

    /// Sets the proposed judicial manager.
    pub fn with_proposed_judicial_manager(mut self, name: impl Into<String>) -> Self {
        self.proposed_judicial_manager = Some(name.into());
        self
    }
}

/// A single creditor participating in a scheme of arrangement vote.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemeCreditor {
    /// Name of the creditor.
    pub name: String,
    /// Value of the creditor's admitted claim, in SGD cents.
    pub claim_cents: u64,
    /// Whether the creditor voted in favour of the scheme.
    pub voted_in_favour: bool,
    /// Whether the creditor was present and voting.
    pub present_and_voting: bool,
}

impl SchemeCreditor {
    /// Creates a new scheme creditor record.
    pub fn new(name: impl Into<String>, claim_cents: u64) -> Self {
        Self {
            name: name.into(),
            claim_cents,
            voted_in_favour: false,
            present_and_voting: true,
        }
    }

    /// Records the creditor's vote.
    pub fn with_vote(mut self, in_favour: bool) -> Self {
        self.voted_in_favour = in_favour;
        self.present_and_voting = true;
        self
    }

    /// Records that the creditor abstained (not present and voting).
    pub fn abstain(mut self) -> Self {
        self.present_and_voting = false;
        self.voted_in_favour = false;
        self
    }
}

/// A class of creditors within a scheme of arrangement.
///
/// IRDA s. 210(3AB): each class must independently approve the scheme by a
/// majority in number representing 75% in value of those present and voting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemeClass {
    /// Name of the class (e.g. "Unsecured creditors").
    pub class_name: String,
    /// Creditors comprised in the class.
    pub creditors: Vec<SchemeCreditor>,
}

impl SchemeClass {
    /// Creates a new, empty scheme class.
    pub fn new(class_name: impl Into<String>) -> Self {
        Self {
            class_name: class_name.into(),
            creditors: Vec::new(),
        }
    }

    /// Adds a creditor to the class.
    pub fn add_creditor(&mut self, creditor: SchemeCreditor) {
        self.creditors.push(creditor);
    }

    /// Builder-style variant of [`Self::add_creditor`].
    pub fn with_creditor(mut self, creditor: SchemeCreditor) -> Self {
        self.add_creditor(creditor);
        self
    }

    /// Number of creditors present and voting.
    pub fn voting_count(&self) -> u32 {
        self.creditors
            .iter()
            .filter(|c| c.present_and_voting)
            .count() as u32
    }

    /// Number of creditors voting in favour.
    pub fn in_favour_count(&self) -> u32 {
        self.creditors
            .iter()
            .filter(|c| c.present_and_voting && c.voted_in_favour)
            .count() as u32
    }

    /// Total value present and voting, in SGD cents.
    pub fn total_voting_value_cents(&self) -> u64 {
        self.creditors
            .iter()
            .filter(|c| c.present_and_voting)
            .map(|c| c.claim_cents)
            .sum()
    }

    /// Value voting in favour, in SGD cents.
    pub fn in_favour_value_cents(&self) -> u64 {
        self.creditors
            .iter()
            .filter(|c| c.present_and_voting && c.voted_in_favour)
            .map(|c| c.claim_cents)
            .sum()
    }
}

/// A scheme of arrangement between a company and its creditors.
///
/// IRDA Part 5 (ss. 64-72) enhances the scheme regime with an automatic
/// moratorium (s. 64), super-priority rescue financing (s. 67) and a cross-class
/// "cram-down" (s. 70). The compromise is ultimately sanctioned by the Court
/// under s. 210.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemeOfArrangement {
    /// Identifier for the scheme.
    pub scheme_id: String,
    /// Name of the company proposing the scheme.
    pub company_name: String,
    /// Creditor classes comprising the scheme.
    pub classes: Vec<SchemeClass>,
    /// Whether an automatic moratorium under s. 64 has been triggered.
    pub moratorium_in_force: bool,
    /// Length of the moratorium sought, in days.
    pub moratorium_days: u32,
    /// Whether super-priority rescue financing is sought (s. 67).
    pub seeks_rescue_financing: bool,
    /// Whether a cross-class cram-down under s. 70 is sought.
    pub seeks_cram_down: bool,
    /// Date the scheme was proposed.
    pub proposed_date: DateTime<Utc>,
}

impl SchemeOfArrangement {
    /// Creates a new scheme of arrangement.
    pub fn new(scheme_id: impl Into<String>, company_name: impl Into<String>) -> Self {
        Self {
            scheme_id: scheme_id.into(),
            company_name: company_name.into(),
            classes: Vec::new(),
            moratorium_in_force: false,
            moratorium_days: 0,
            seeks_rescue_financing: false,
            seeks_cram_down: false,
            proposed_date: Utc::now(),
        }
    }

    /// Adds a creditor class.
    pub fn add_class(&mut self, class: SchemeClass) {
        self.classes.push(class);
    }

    /// Builder-style variant of [`Self::add_class`].
    pub fn with_class(mut self, class: SchemeClass) -> Self {
        self.add_class(class);
        self
    }

    /// Triggers an automatic moratorium of the given length (IRDA s. 64).
    pub fn with_moratorium(mut self, days: u32) -> Self {
        self.moratorium_in_force = true;
        self.moratorium_days = days;
        self
    }

    /// Indicates that super-priority rescue financing is sought (IRDA s. 67).
    pub fn with_rescue_financing(mut self, sought: bool) -> Self {
        self.seeks_rescue_financing = sought;
        self
    }

    /// Indicates that a cross-class cram-down is sought (IRDA s. 70).
    pub fn with_cram_down(mut self, sought: bool) -> Self {
        self.seeks_cram_down = sought;
        self
    }
}

/// The party making a bankruptcy application (IRDA Part 16).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankruptcyApplicant {
    /// A creditor's application (IRDA s. 311).
    Creditor,
    /// The debtor's own application (IRDA s. 310).
    DebtorOwn,
}

impl BankruptcyApplicant {
    /// Returns the precise statute reference for this applicant.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            BankruptcyApplicant::Creditor => "IRDA s. 311",
            BankruptcyApplicant::DebtorOwn => "IRDA s. 310",
        }
    }

    /// Returns a short description of this applicant.
    pub fn description(&self) -> &'static str {
        match self {
            BankruptcyApplicant::Creditor => "Creditor's bankruptcy application",
            BankruptcyApplicant::DebtorOwn => "Debtor's own bankruptcy application",
        }
    }
}

/// An application for a bankruptcy order against an individual debtor.
///
/// IRDA Part 16 (formerly the Bankruptcy Act). A creditor may apply where the
/// debt is a liquidated sum of at least SGD 15,000 and the debtor is unable to
/// pay.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankruptcyApplication {
    /// Identifier for the application.
    pub application_id: String,
    /// Name of the debtor.
    pub debtor_name: String,
    /// Name of the applicant creditor (if a creditor's application).
    pub creditor_name: Option<String>,
    /// Which party is making the application.
    pub applicant: BankruptcyApplicant,
    /// The debt relied upon, in SGD cents.
    pub debt_cents: u64,
    /// Whether the debtor is unable to pay the debt.
    pub debtor_unable_to_pay: bool,
    /// Whether an unsatisfied statutory demand supports the application.
    pub statutory_demand_unsatisfied: bool,
    /// Date the application was filed.
    pub filed_date: DateTime<Utc>,
}

impl BankruptcyApplication {
    /// Creates a new bankruptcy application.
    pub fn new(
        application_id: impl Into<String>,
        debtor_name: impl Into<String>,
        applicant: BankruptcyApplicant,
        debt_cents: u64,
    ) -> Self {
        Self {
            application_id: application_id.into(),
            debtor_name: debtor_name.into(),
            creditor_name: None,
            applicant,
            debt_cents,
            debtor_unable_to_pay: false,
            statutory_demand_unsatisfied: false,
            filed_date: Utc::now(),
        }
    }

    /// Sets the applicant creditor's name.
    pub fn with_creditor_name(mut self, name: impl Into<String>) -> Self {
        self.creditor_name = Some(name.into());
        self
    }

    /// Records that the debtor is unable to pay the debt.
    pub fn with_inability_to_pay(mut self, unable: bool) -> Self {
        self.debtor_unable_to_pay = unable;
        self
    }

    /// Records that an unsatisfied statutory demand supports the application.
    pub fn with_unsatisfied_statutory_demand(mut self, unsatisfied: bool) -> Self {
        self.statutory_demand_unsatisfied = unsatisfied;
        self
    }

    /// Whether the debt meets the bankruptcy threshold (SGD 15,000).
    pub fn meets_debt_threshold(&self) -> bool {
        self.debt_cents >= BANKRUPTCY_DEBT_THRESHOLD_CENTS
    }

    /// Converts the debt to whole SGD.
    pub fn debt_in_sgd(&self) -> u64 {
        self.debt_cents / 100
    }
}

/// A debtor's profile for assessing Debt Repayment Scheme (DRS) eligibility.
///
/// IRDA Part 14 (ss. 289-308): the DRS, administered by the Official Assignee, is
/// a voluntary alternative to bankruptcy for debtors whose aggregate debts do not
/// exceed SGD 150,000.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebtRepaymentSchemeProfile {
    /// Name of the debtor.
    pub debtor_name: String,
    /// Aggregate debts of the debtor, in SGD cents.
    pub aggregate_debt_cents: u64,
    /// Whether the debtor has a regular source of income.
    pub has_regular_income: bool,
    /// Whether the debtor is an undischarged bankrupt (disqualifying).
    pub is_undischarged_bankrupt: bool,
}

impl DebtRepaymentSchemeProfile {
    /// Creates a new DRS profile.
    pub fn new(debtor_name: impl Into<String>, aggregate_debt_cents: u64) -> Self {
        Self {
            debtor_name: debtor_name.into(),
            aggregate_debt_cents,
            has_regular_income: true,
            is_undischarged_bankrupt: false,
        }
    }

    /// Sets whether the debtor has a regular source of income.
    pub fn with_regular_income(mut self, has_income: bool) -> Self {
        self.has_regular_income = has_income;
        self
    }

    /// Sets whether the debtor is an undischarged bankrupt.
    pub fn with_undischarged_bankrupt(mut self, undischarged: bool) -> Self {
        self.is_undischarged_bankrupt = undischarged;
        self
    }

    /// Whether the aggregate debt is within the DRS ceiling (SGD 150,000).
    pub fn within_debt_ceiling(&self) -> bool {
        self.aggregate_debt_cents <= DEBT_REPAYMENT_SCHEME_CEILING_CENTS
    }

    /// Converts the aggregate debt to whole SGD.
    pub fn aggregate_debt_in_sgd(&self) -> u64 {
        self.aggregate_debt_cents / 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_constants_are_consistent() {
        assert_eq!(COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS, 1_500_000);
        assert_eq!(BANKRUPTCY_DEBT_THRESHOLD_CENTS, 1_500_000);
        assert_eq!(DEBT_REPAYMENT_SCHEME_CEILING_CENTS, 15_000_000);
        assert_eq!(STATUTORY_DEMAND_PERIOD_DAYS, 21);
        assert_eq!(AUTOMATIC_MORATORIUM_DAYS, 30);
    }

    #[test]
    fn test_winding_up_mode_references() {
        assert_eq!(
            WindingUpMode::CompulsoryByCourt.statute_reference(),
            "IRDA s. 124"
        );
        assert!(WindingUpMode::MembersVoluntary.requires_declaration_of_solvency());
        assert!(!WindingUpMode::CreditorsVoluntary.requires_declaration_of_solvency());
        assert!(!WindingUpMode::CompulsoryByCourt.description().is_empty());
    }

    #[test]
    fn test_winding_up_ground_references() {
        assert_eq!(
            WindingUpGround::UnableToPayDebts.statute_reference(),
            "IRDA s. 125(1)(e)"
        );
        assert_eq!(
            WindingUpGround::JustAndEquitable.statute_reference(),
            "IRDA s. 125(1)(i)"
        );
        assert_eq!(
            WindingUpGround::SpecialResolution.statute_reference(),
            "IRDA s. 125(1)(a)"
        );
        assert!(!WindingUpGround::OppressiveConduct.description().is_empty());
    }

    #[test]
    fn test_inability_test_references() {
        assert_eq!(
            InabilityToPayTest::StatutoryDemand.statute_reference(),
            "IRDA s. 125(2)(a)"
        );
        assert_eq!(
            InabilityToPayTest::UnsatisfiedExecution.statute_reference(),
            "IRDA s. 125(2)(b)"
        );
        assert_eq!(
            InabilityToPayTest::ProvedToCourt.statute_reference(),
            "IRDA s. 125(2)(c)"
        );
    }

    #[test]
    fn test_statutory_demand_thresholds() {
        let demand = StatutoryDemand::new("sd1", "Bank A", "Acme Pte Ltd", 2_000_000)
            .with_days_unsatisfied(25);
        assert!(demand.exceeds_prescribed_sum());
        assert!(demand.period_expired());
        assert_eq!(demand.debt_in_sgd(), 20_000);

        let small = StatutoryDemand::new("sd2", "Bank A", "Acme Pte Ltd", 1_000_000);
        assert!(!small.exceeds_prescribed_sum());
        assert!(!small.period_expired());
    }

    #[test]
    fn test_statutory_demand_at_exact_prescribed_sum() {
        // Exactly SGD 15,000 does NOT exceed the prescribed sum (must be >).
        let demand = StatutoryDemand::new("sd3", "Bank", "Co", 1_500_000);
        assert!(!demand.exceeds_prescribed_sum());

        let just_over = StatutoryDemand::new("sd4", "Bank", "Co", 1_500_001);
        assert!(just_over.exceeds_prescribed_sum());
    }

    #[test]
    fn test_winding_up_petition_builder() {
        let petition =
            WindingUpPetition::new("p1", "Acme Pte Ltd", WindingUpMode::CompulsoryByCourt)
                .with_ground(WindingUpGround::UnableToPayDebts)
                .with_inability_test(InabilityToPayTest::StatutoryDemand);
        assert_eq!(petition.ground, Some(WindingUpGround::UnableToPayDebts));
        assert_eq!(
            petition.inability_test,
            Some(InabilityToPayTest::StatutoryDemand)
        );
    }

    #[test]
    fn test_judicial_management_purposes() {
        assert_eq!(
            JudicialManagementPurpose::SurvivalAsGoingConcern.statute_reference(),
            "IRDA s. 89(1)(a)"
        );
        assert_eq!(
            JudicialManagementPurpose::ApprovalOfScheme.statute_reference(),
            "IRDA s. 89(1)(b)"
        );
        assert_eq!(
            JudicialManagementPurpose::MoreAdvantageousRealisation.statute_reference(),
            "IRDA s. 89(1)(c)"
        );
    }

    #[test]
    fn test_judicial_management_application_builder() {
        let mut app = JudicialManagementApplication::new("jm1", "Distressed Pte Ltd")
            .with_insolvency_limb(true)
            .with_out_of_court(true);
        app.add_purpose(JudicialManagementPurpose::SurvivalAsGoingConcern);
        // Adding a duplicate purpose should not increase the count.
        app.add_purpose(JudicialManagementPurpose::SurvivalAsGoingConcern);
        assert_eq!(app.purposes.len(), 1);
        assert!(app.is_or_likely_unable_to_pay);
        assert!(app.out_of_court);
    }

    #[test]
    fn test_scheme_class_tallies() {
        let class = SchemeClass::new("Unsecured")
            .with_creditor(SchemeCreditor::new("A", 600_000).with_vote(true))
            .with_creditor(SchemeCreditor::new("B", 300_000).with_vote(true))
            .with_creditor(SchemeCreditor::new("C", 100_000).with_vote(false));

        assert_eq!(class.voting_count(), 3);
        assert_eq!(class.in_favour_count(), 2);
        assert_eq!(class.total_voting_value_cents(), 1_000_000);
        assert_eq!(class.in_favour_value_cents(), 900_000);
    }

    #[test]
    fn test_scheme_class_excludes_abstentions() {
        let class = SchemeClass::new("Unsecured")
            .with_creditor(SchemeCreditor::new("A", 600_000).with_vote(true))
            .with_creditor(SchemeCreditor::new("B", 300_000).abstain());

        // Abstaining creditor is not counted as present and voting.
        assert_eq!(class.voting_count(), 1);
        assert_eq!(class.total_voting_value_cents(), 600_000);
    }

    #[test]
    fn test_scheme_of_arrangement_builder() {
        let scheme = SchemeOfArrangement::new("s1", "Restructure Pte Ltd")
            .with_moratorium(30)
            .with_rescue_financing(true)
            .with_cram_down(true)
            .with_class(SchemeClass::new("Secured"));

        assert!(scheme.moratorium_in_force);
        assert_eq!(scheme.moratorium_days, 30);
        assert!(scheme.seeks_rescue_financing);
        assert!(scheme.seeks_cram_down);
        assert_eq!(scheme.classes.len(), 1);
    }

    #[test]
    fn test_bankruptcy_application_threshold() {
        let app =
            BankruptcyApplication::new("b1", "John Tan", BankruptcyApplicant::Creditor, 2_000_000)
                .with_creditor_name("Bank A")
                .with_inability_to_pay(true);
        assert!(app.meets_debt_threshold());
        assert_eq!(app.debt_in_sgd(), 20_000);
        assert_eq!(app.applicant.statute_reference(), "IRDA s. 311");

        let small =
            BankruptcyApplication::new("b2", "Jane Lim", BankruptcyApplicant::Creditor, 1_000_000);
        assert!(!small.meets_debt_threshold());
    }

    #[test]
    fn test_bankruptcy_applicant_references() {
        assert_eq!(
            BankruptcyApplicant::Creditor.statute_reference(),
            "IRDA s. 311"
        );
        assert_eq!(
            BankruptcyApplicant::DebtorOwn.statute_reference(),
            "IRDA s. 310"
        );
    }

    #[test]
    fn test_drs_profile_ceiling() {
        let eligible = DebtRepaymentSchemeProfile::new("Debtor A", 10_000_000);
        assert!(eligible.within_debt_ceiling());
        assert_eq!(eligible.aggregate_debt_in_sgd(), 100_000);

        let over = DebtRepaymentSchemeProfile::new("Debtor B", 20_000_000);
        assert!(!over.within_debt_ceiling());

        let exactly =
            DebtRepaymentSchemeProfile::new("Debtor C", DEBT_REPAYMENT_SCHEME_CEILING_CENTS);
        assert!(exactly.within_debt_ceiling());
    }
}
