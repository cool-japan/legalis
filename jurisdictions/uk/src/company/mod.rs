//! Company Law Module (Companies Act 2006)
//!
//! Comprehensive implementation of UK company law under the Companies Act 2006.
//!
//! # Key Legislation
//!
//! ## Companies Act 2006
//!
//! The Companies Act 2006 is the primary source of UK company law. It received Royal
//! Assent on 8 November 2006 and is the longest Act in UK parliamentary history.
//!
//! ### Structure
//!
//! - **Part 1**: General introductory provisions
//! - **Part 2**: Company formation (ss.7-16)
//! - **Part 10**: Directors (ss.154-259)
//! - **Part 10, Chapter 2**: Seven statutory director duties (ss.171-177)
//! - **Part 13**: Resolutions and meetings (ss.281-361)
//! - **Part 15**: Accounts and reports (ss.380-474)
//! - **Part 17**: Share capital (ss.540-657)
//! - **Part 5**: Company name (ss.53-81)
//!
//! # Seven Statutory Director Duties (ss.171-177)
//!
//! The Companies Act 2006 codified common law and equitable duties of directors
//! into seven statutory duties:
//!
//! ## s.171: Duty to Act Within Powers
//!
//! A director must:
//! - Act in accordance with the company's constitution
//! - Only exercise powers for the purposes for which they are conferred
//!
//! ## s.172: Duty to Promote the Success of the Company
//!
//! A director must act in the way they consider, in good faith, would be most
//! likely to promote the success of the company for the benefit of its members
//! as a whole, having regard to:
//!
//! 1. **(a)** Long-term consequences of any decision
//! 2. **(b)** Interests of the company's employees
//! 3. **(c)** Need to foster business relationships with suppliers, customers, and others
//! 4. **(d)** Impact of company's operations on community and environment
//! 5. **(e)** Desirability of maintaining reputation for high standards
//! 6. **(f)** Need to act fairly between members of the company
//!
//! This is known as "enlightened shareholder value" - directors must consider
//! wider stakeholder interests while promoting shareholder value.
//!
//! ## s.173: Duty to Exercise Independent Judgment
//!
//! A director must exercise independent judgment. This duty is not infringed by:
//! - Acting in accordance with an agreement entered into by the company
//! - Acting in a way authorized by the company's constitution
//!
//! ## s.174: Duty to Exercise Reasonable Care, Skill and Diligence
//!
//! A director must exercise reasonable care, skill and diligence. This means
//! the care, skill and diligence that would be exercised by a reasonably
//! diligent person with:
//!
//! - **(Objective test)** General knowledge, skill and experience reasonably
//!   expected from person carrying out functions of director
//! - **(Subjective test)** General knowledge, skill and experience that the
//!   director has
//!
//! The standard is a combination of objective and subjective tests.
//!
//! ## s.175: Duty to Avoid Conflicts of Interest
//!
//! A director must avoid situations where they have, or can have, a direct or
//! indirect interest that conflicts, or possibly may conflict, with the interests
//! of the company.
//!
//! This applies to exploitation of property, information or opportunity, whether
//! or not the company could take advantage of it.
//!
//! Authorization by directors may be given (subject to constitution).
//!
//! ## s.176: Duty Not to Accept Benefits from Third Parties
//!
//! A director must not accept a benefit from a third party conferred by reason of:
//! - Being a director, or
//! - Doing (or not doing) anything as director
//!
//! "Third party" means anyone other than the company, associated body corporate,
//! or person acting on behalf of the company/associated body corporate.
//!
//! ## s.177: Duty to Declare Interest in Proposed Transaction or Arrangement
//!
//! If a director is in any way, directly or indirectly, interested in a proposed
//! transaction or arrangement with the company, they must declare the nature and
//! extent of that interest to the other directors.
//!
//! Declaration must be made:
//! - Before the company enters into the transaction/arrangement
//! - At a meeting of directors, or
//! - By notice to directors (written or general notice)
//!
//! # Company Formation (Part 2)
//!
//! ## Requirements for Registration (s.9)
//!
//! Application must contain:
//! 1. Company name
//! 2. Registered office (England/Wales, Scotland, or NI)
//! 3. Statement of capital and initial shareholdings (if limited by shares)
//! 4. Statement of guarantee (if limited by guarantee)
//! 5. Statement of proposed officers
//! 6. Statement of compliance
//!
//! ## Company Names (ss.53-81)
//!
//! - Private limited company: must end with "Limited" or "Ltd"
//! - Public limited company: must end with "Public Limited Company" or "PLC"
//! - Welsh companies: "Cyfyngedig" or "Cyf" (private), "Cwmni Cyfyngedig Cyhoeddus" or "CCC" (public)
//!
//! Prohibited:
//! - Criminal offence if registered (s.66)
//! - Offensive names
//! - Sensitive words without approval (s.55)
//! - Too similar to existing name (s.66)
//!
//! # Share Capital (Part 17)
//!
//! ## Public Companies (s.763)
//!
//! - Minimum authorized capital: £50,000
//! - Minimum paid up: 25% of nominal value (s.586)
//! - Must obtain trading certificate before doing business (s.761)
//!
//! ## Private Companies
//!
//! - No minimum capital requirement
//! - Can be formed with nominal capital (e.g., £1)
//!
//! # Directors (Part 10)
//!
//! ## Minimum Number (s.154)
//!
//! - Private company: at least 1 director
//! - Public company: at least 2 directors
//!
//! ## Company Secretary
//!
//! - Public company: must have secretary (s.271)
//! - Private company: optional (s.270)
//!
//! # Annual Accounts (Part 15)
//!
//! ## Filing Deadlines (s.442)
//!
//! - Private company: 9 months after year end
//! - Public company: 6 months after year end
//!
//! ## Audit Requirements (s.475)
//!
//! Company is exempt from audit if it qualifies as small company:
//! - Turnover ≤ £10.2 million
//! - Balance sheet total ≤ £5.1 million
//! - Average employees ≤ 50
//!
//! Public companies always require audit.
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::company::*;
//! use chrono::NaiveDate;
//!
//! // Create company formation
//! let formation = CompanyFormation {
//!     company_name: "Acme Trading Ltd".to_string(),
//!     company_type: CompanyType::PrivateLimitedByShares,
//!     registered_office: RegisteredOffice {
//!         address_line_1: "1 High Street".to_string(),
//!         address_line_2: None,
//!         city: "London".to_string(),
//!         county: None,
//!         postcode: "SW1A 1AA".to_string(),
//!         country: RegisteredOfficeCountry::England,
//!     },
//!     share_capital: Some(ShareCapital {
//!         nominal_capital_gbp: 100.0,
//!         paid_up_capital_gbp: 100.0,
//!         number_of_shares: 100,
//!         nominal_value_per_share_gbp: 1.0,
//!         share_classes: vec![],
//!     }),
//!     directors: vec![/* ... */],
//!     shareholders: vec![/* ... */],
//!     secretary: None,
//!     statement_of_compliance: true,
//!     formation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
//! };
//!
//! // Validate formation
//! validate_company_formation(&formation)?;
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export key types
pub use error::{CompanyLawError, Result};
pub use types::{
    AnnualAccountsRequirement, CapitalRights, CompanyFormation, CompanyNameValidation,
    CompanySecretary, CompanyType, ConflictOfInterest, ConflictsCompliance,
    DeclareInterestCompliance, Director, DirectorDutiesCompliance, DirectorType, DividendRights,
    DutyCompliance, InterestDeclaration, MeetingType, PromoteSuccessCompliance,
    ReasonableCareCompliance, RegisteredOffice, RegisteredOfficeCountry, ResolutionType,
    ServiceAddress, ShareCapital, ShareClass, ShareRights, Shareholder,
};
pub use validator::{
    validate_annual_accounts, validate_company_formation, validate_company_name,
    validate_director_duties, validate_directors, validate_resolution, validate_share_capital,
};
