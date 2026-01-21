//! Bankruptcy Law Module (Bankruptcy Code Title 11)
//!
//! Comprehensive implementation of US federal bankruptcy law.
//!
//! # Bankruptcy Code (Title 11 USC)
//!
//! The Bankruptcy Code provides several chapters for different types of debtors and situations:
//!
//! ## Chapter 7: Liquidation ("Straight Bankruptcy")
//!
//! Chapter 7 is the most common form of bankruptcy for individuals and businesses.
//!
//! ### Process
//!
//! 1. **Petition filed** - Voluntary (debtor) or involuntary (creditors)
//! 2. **Automatic stay** imposed - Creditors must stop collection activities
//! 3. **Trustee appointed** - Takes control of non-exempt assets
//! 4. **Meeting of creditors** (341 meeting) - Debtor examined under oath
//! 5. **Asset liquidation** - Trustee sells non-exempt assets
//! 6. **Distribution to creditors** - In order of priority
//! 7. **Discharge** - Most debts eliminated (if eligible)
//!
//! ### Eligibility
//!
//! **Individuals**: Must pass means test (income below state median OR disposable income insufficient
//! to fund Chapter 13 plan).
//!
//! **Corporations/Partnerships**: Always eligible for Chapter 7.
//!
//! **Prior Discharge Bar**: Cannot receive Chapter 7 discharge if received discharge in prior Chapter 7
//! within 8 years or Chapter 13 within 6 years.
//!
//! ### Means Test (Section 707(b))
//!
//! For individual debtors with primarily consumer debts:
//!
//! ```text
//! Step 1: Compare current monthly income (CMI) × 12 to state median income
//!         ├─ Below median → Pass means test (presumption of no abuse)
//!         └─ Above median → Proceed to Step 2
//!
//! Step 2: Calculate monthly disposable income (MDI)
//!         MDI = CMI - allowed expenses (IRS standards + actual expenses)
//!
//! Step 3: Calculate projected disposable income over 60 months
//!         PDI = MDI × 60
//!
//! Step 4: Compare PDI to debt thresholds
//!         ├─ PDI ≥ $8,175 → Presumption of abuse (must file Ch. 13 or dismiss)
//!         ├─ PDI < $13,650 AND < 25% of unsecured debt → No presumption
//!         └─ Between thresholds → Calculate 25% of unsecured debt
//!             └─ PDI ≥ 25% of unsecured debt → Presumption of abuse
//! ```
//!
//! If presumption of abuse arises, case may be dismissed or converted to Chapter 13 unless
//! debtor can rebut presumption (special circumstances).
//!
//! ### Property of the Estate (Section 541)
//!
//! The bankruptcy estate includes:
//! - All legal and equitable interests of debtor in property as of petition date
//! - Property acquired within 180 days after petition (inheritance, life insurance, divorce settlement)
//! - Earnings from services performed before case closed (except in Chapter 7)
//!
//! ### Exemptions (Section 522)
//!
//! Debtors may exempt certain property from the estate:
//!
//! **Federal Exemptions** (11 USC § 522(d)) - if state allows:
//! - Homestead: up to $27,900 in residence
//! - Motor vehicle: up to $4,450
//! - Household goods: up to $700 per item, $14,875 total
//! - Jewelry: up to $1,875
//! - Wildcard: up to $1,475 + unused homestead (up to $13,950)
//! - Retirement accounts: Unlimited (ERISA-qualified, IRAs up to $1,512,350)
//!
//! **State Exemptions**: 35 states have "opted out" and require use of state exemptions.
//!
//! **Homestead Exemptions** (vary widely by state):
//! - Unlimited: FL, TX, IA (subject to certain limitations)
//! - High: CA ($600k-$700k), MA ($500k), MN ($390k)
//! - Low: MD ($0), DE ($0), PA ($0) - though may have general exemption
//!
//! ### Discharge (Section 727)
//!
//! Chapter 7 discharge eliminates personal liability for most debts.
//!
//! **Nondischargeable Debts** (Section 523):
//! - Priority tax debts (recent income taxes)
//! - Debts obtained by fraud or false pretenses
//! - Debts not listed in bankruptcy
//! - Domestic support obligations (child support, alimony)
//! - Student loans (unless undue hardship - Brunner test)
//! - Debts for willful and malicious injury
//! - Fines and penalties owed to government
//! - DUI-related death or injury debts
//!
//! **Discharge Denial** (Section 727(a)):
//! - Debtor not an individual (corps receive discharge through liquidation)
//! - Fraudulent transfer or concealment of property
//! - Failure to keep records
//! - False oath or account
//! - Failure to explain loss of assets
//! - Refusal to testify
//! - Prior discharge within time period
//!
//! ## Chapter 13: Individual Debt Adjustment ("Wage Earner Plan")
//!
//! Chapter 13 allows individuals with regular income to propose a repayment plan.
//!
//! ### Advantages over Chapter 7
//!
//! - Keep all property (no liquidation)
//! - Cure mortgage/car loan arrears over time
//! - Strip unsecured junior liens on underwater property
//! - Pay back taxes over 5 years
//! - Co-debtor stay (protects non-filing co-signers)
//!
//! ### Eligibility (Section 109(e))
//!
//! - Individual (or individual + spouse)
//! - Regular income
//! - Secured debt < $2,750,000
//! - Unsecured debt < $2,750,000
//!
//! ### Chapter 13 Plan Requirements (Section 1325)
//!
//! 1. **Feasibility**: Debtor has ability to make plan payments
//! 2. **Good faith**: Plan proposed in good faith
//! 3. **Best interests test**: Unsecured creditors receive at least what they would in Chapter 7
//! 4. **Disposable income test**: Plan commits all disposable income to payments
//! 5. **Priority debts**: Must pay in full (domestic support, recent taxes)
//! 6. **Secured debts**: Must maintain payments or surrender collateral
//!
//! ### Plan Duration (Section 1322(d))
//!
//! - Below-median income: 3 years (36 months) - may extend to 5 years
//! - Above-median income: 5 years (60 months) required
//!
//! ### Discharge (Section 1328)
//!
//! After completing all plan payments, debtor receives discharge of remaining debts (except
//! nondischargeable debts similar to Chapter 7, plus long-term debts and domestic support arrears).
//!
//! **Hardship Discharge** (Section 1328(b)): If debtor cannot complete plan due to circumstances
//! beyond control, may receive limited discharge.
//!
//! ## Chapter 11: Reorganization
//!
//! Chapter 11 allows businesses (and high-debt individuals) to reorganize while continuing operations.
//!
//! ### Debtor in Possession
//!
//! Unlike Chapter 7, debtor typically remains in control as "debtor in possession" (DIP).
//! Trustee appointed only in cases of fraud, dishonesty, incompetence, or gross mismanagement.
//!
//! ### Process
//!
//! 1. **Petition filed** (voluntary or involuntary)
//! 2. **Automatic stay** imposed
//! 3. **Debtor in possession** operates business
//! 4. **Creditors' committee** appointed (unsecured creditors)
//! 5. **Exclusivity period** (120 days) - only debtor can propose plan
//! 6. **Disclosure statement** prepared and approved
//! 7. **Plan of reorganization** proposed and voted on by creditors
//! 8. **Confirmation hearing** - court confirms plan
//! 9. **Plan implementation** - debtor makes plan payments
//! 10. **Discharge** - upon plan confirmation (corporations) or completion (individuals)
//!
//! ### Plan Confirmation (Section 1129)
//!
//! **Consensual Confirmation**: All impaired classes vote to accept plan
//!
//! **Cramdown** (Section 1129(b)): Court can confirm plan over dissenting class if:
//! - Plan does not discriminate unfairly
//! - Plan is fair and equitable:
//!   - **Secured creditors**: Retain liens, receive present value of allowed secured claim
//!   - **Unsecured creditors**: Absolute priority rule - no junior class receives anything unless
//!     dissenting class paid in full
//!
//! ### Small Business Reorganization (Subchapter V)
//!
//! Added in 2019 for small businesses (debt < $7,500,000):
//! - No creditors' committee (unless court orders)
//! - No disclosure statement required
//! - Plan may be confirmed without creditor acceptance if fair and equitable
//! - Debtor retains property despite unsecured creditors not being paid in full
//!
//! ## Chapter 12: Family Farmer or Fisherman Debt Adjustment
//!
//! Similar to Chapter 13 but tailored for family farmers and fishermen.
//!
//! ### Eligibility
//!
//! - Family farmer: 50%+ gross income from farming, debt < $11,097,350 (80%+ from farming)
//! - Family fisherman: 50%+ gross income from fishing, debt < $2,268,550 (80%+ from fishing)
//!
//! ### Advantages
//!
//! - Higher debt limits than Chapter 13
//! - Flexibility in plan payments (seasonal income)
//! - Can modify secured claims on farmland
//!
//! ## Automatic Stay (Section 362)
//!
//! Upon filing bankruptcy, automatic stay immediately stops:
//! - Foreclosures
//! - Repossessions
//! - Wage garnishments
//! - Utility shut-offs
//! - Lawsuits
//! - Collection calls and letters
//!
//! ### Relief from Stay (Section 362(d))
//!
//! Creditor may seek relief from stay by showing:
//! - **Cause** (e.g., lack of adequate protection)
//! - **No equity + not necessary to reorganization** (typically for secured creditors)
//!
//! ### Exceptions to Stay (Section 362(b))
//!
//! Stay does not apply to:
//! - Criminal proceedings
//! - Domestic support obligation collection
//! - Tax audits and assessments
//! - Eviction (in some circumstances)
//!
//! ### Stay Violations
//!
//! Willful violation of stay entitles debtor to:
//! - Actual damages (including attorney fees)
//! - Punitive damages (in some cases)
//! - Contempt of court sanctions
//!
//! ## Priority of Claims (Section 507)
//!
//! In distribution to creditors, claims are paid in this order:
//!
//! ```text
//! 1. Secured claims (to extent of collateral value)
//! 2. Priority unsecured claims:
//!    (a) Domestic support obligations
//!    (b) Administrative expenses (trustee, attorney fees)
//!    (c) Gap creditors (involuntary case)
//!    (d) Wages/salaries (up to $13,650 per employee, within 180 days)
//!    (e) Employee benefit plans (up to $13,650 per employee)
//!    (f) Farmers/fishermen (up to $6,725)
//!    (g) Consumer deposits (up to $3,025)
//!    (h) Tax claims (federal, state, local)
//! 3. General unsecured claims
//! 4. Subordinated debt
//! 5. Equity interests (shareholders)
//! ```
//!
//! ## Adversary Proceedings
//!
//! Certain disputes require adversary proceedings (formal lawsuits within bankruptcy):
//! - Dischargeability complaints (Section 523)
//! - Objections to discharge (Section 727)
//! - Fraudulent transfer actions (Section 548)
//! - Preferential transfer actions (Section 547)
//! - Lien avoidance actions
//!
//! ## Fraudulent Transfers (Section 548)
//!
//! Trustee can avoid transfers made within 2 years before petition if:
//!
//! **Actual Fraud**: Transfer made with intent to hinder, delay, or defraud creditors (badges of fraud)
//!
//! **Constructive Fraud**: Transfer made for less than reasonably equivalent value while debtor was:
//! - Insolvent or became insolvent
//! - Engaged in business with unreasonably small capital
//! - Intended to incur debts beyond ability to pay
//!
//! State fraudulent transfer laws (UFTA/UVTA) may reach back further (4-10 years).
//!
//! ## Preferential Transfers (Section 547)
//!
//! Trustee can avoid transfers to creditors within 90 days before petition (1 year for insiders) if:
//! - Transfer for antecedent debt
//! - Made while debtor insolvent (presumed for 90 days)
//! - Enables creditor to receive more than in Chapter 7 liquidation
//!
//! **Exceptions** (Section 547(c)):
//! - Substantially contemporaneous exchange
//! - Ordinary course of business
//! - Purchase money security interest
//! - Small transfers (< $7,575)
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_us::bankruptcy::*;
//! use chrono::NaiveDate;
//!
//! // Create Chapter 7 bankruptcy case
//! let debtor = Debtor {
//!     name: "John Doe".to_string(),
//!     debtor_type: DebtorType::Individual,
//!     tax_id_last_four: "1234".to_string(),
//!     address: "123 Main St".to_string(),
//!     date_of_birth: Some(NaiveDate::from_ymd_opt(1980, 1, 1).unwrap()),
//!     employment_status: Some(EmploymentStatus {
//!         employed: true,
//!         employer: Some("ACME Corp".to_string()),
//!         occupation: Some("Engineer".to_string()),
//!         employment_start_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
//!     }),
//!     monthly_income: 5000.0,
//!     monthly_expenses: 4000.0,
//!     total_assets: 50000.0,
//!     total_liabilities: 100000.0,
//!     prior_bankruptcies: vec![],
//!     is_joint_filing: false,
//! };
//!
//! let case = BankruptcyCase {
//!     case_number: "24-12345".to_string(),
//!     chapter: BankruptcyChapter::Chapter7,
//!     debtor,
//!     filing_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     status: CaseStatus::Filed,
//!     court_district: "N.D. California".to_string(),
//!     judge: Some("Hon. Jane Smith".to_string()),
//!     trustee: Some(Trustee {
//!         name: "Alice Johnson".to_string(),
//!         trustee_type: TrusteeType::Chapter7Trustee,
//!         appointment_date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
//!         contact: "trustee@example.com".to_string(),
//!     }),
//!     automatic_stay_active: true,
//!     estate: None,
//!     creditors: vec![],
//!     discharge: None,
//!     conversion: None,
//! };
//!
//! // Check if automatic stay is active
//! assert!(case.has_automatic_stay());
//!
//! // Check discharge eligibility
//! assert!(case.is_eligible_for_discharge());
//! ```

pub mod error;
pub mod types;

// Re-export key types
pub use error::{BankruptcyError, Result};
pub use types::{
    Asset, AssetType, AutomaticStay, BankruptcyCase, BankruptcyChapter, BankruptcyEstate,
    CaseStatus, ChapterConversion, ClaimPriority, ClaimType, Creditor, Debtor, DebtorType,
    Discharge, DischargeObjection, DischargeObjectionGrounds, DischargeObjectionOutcome,
    EmploymentStatus, Exemption, ExemptionType, NondischargeableDebt, NondischargeableDebtType,
    ObjectionOutcome, PriorBankruptcy, ReliefFromStay, ReliefOutcome, SecurityInterest,
    SecurityInterestType, StayException, Trustee, TrusteeType,
};
