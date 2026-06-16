//! Insolvency, Restructuring and Dissolution Act 2018 (IRDA)
//!
//! This module provides type-safe implementations of Singapore's insolvency,
//! restructuring and dissolution framework under the **Insolvency, Restructuring
//! and Dissolution Act 2018** (No. 40 of 2018), which came into operation on
//! 30 July 2020.
//!
//! ## Overview
//!
//! The IRDA is an omnibus statute that consolidated three previously fragmented
//! regimes into a single Act:
//!
//! 1. **Corporate winding up** - formerly Part X of the Companies Act (Cap. 50);
//! 2. **Personal bankruptcy** - formerly the Bankruptcy Act (Cap. 20); and
//! 3. **Corporate rescue** - judicial management and schemes of arrangement,
//!    enhanced with debtor-in-possession style tools.
//!
//! It modernised Singapore's restructuring law, introducing US Chapter 11-style
//! features such as super-priority rescue financing and a cross-class cram-down,
//! cementing Singapore's position as a regional restructuring hub.
//!
//! ## Regimes Modelled
//!
//! ### A. Corporate Winding Up (IRDA Part 8)
//!
//! - **Compulsory** winding up by the Court on a ground in **s. 125(1)** - most
//!   commonly that the company is **unable to pay its debts** (s. 125(1)(e)).
//! - **Members' voluntary** winding up - the directors make a declaration of
//!   solvency (s. 161).
//! - **Creditors' voluntary** winding up - where no declaration of solvency can
//!   be made (s. 166).
//!
//! The "unable to pay its debts" deeming provisions are in **s. 125(2)**: an
//! unsatisfied statutory demand for more than the prescribed sum (SGD 15,000)
//! outstanding for 3 weeks (s. 125(2)(a)); an unsatisfied execution
//! (s. 125(2)(b)); and proof to the Court taking into account contingent and
//! prospective liabilities (s. 125(2)(c)).
//!
//! ### B. Judicial Management (IRDA Part 7, ss. 88-113)
//!
//! A rescue mechanism. The Court (or, since the IRDA, creditors out of court by
//! resolution) may place a company under judicial management where it is or is
//! likely to become unable to pay its debts **and** there is a reasonable
//! probability of achieving one of the three statutory purposes in **s. 89(1)**:
//! survival as a going concern; approval of a scheme; or a more advantageous
//! realisation of assets than on a winding up. A moratorium arises.
//!
//! ### C. Schemes of Arrangement (IRDA Part 5, ss. 64-72; sanction under s. 210)
//!
//! A court-sanctioned compromise between a company and its creditors. Each class
//! must approve by a **majority in number** representing **75% in value** of
//! those present and voting (**s. 210(3AB)**), after which the Court sanctions
//! the scheme. The IRDA adds an automatic 30-day moratorium (**s. 64**),
//! super-priority rescue financing (**s. 67**) and a cross-class cram-down with
//! a "no creditor worse off" safeguard (**s. 70**).
//!
//! ### D. Bankruptcy (IRDA Part 16) and the Debt Repayment Scheme (Part 14)
//!
//! Individual insolvency. A creditor may apply for a bankruptcy order where the
//! debt is a liquidated sum of at least SGD 15,000 and the debtor is unable to
//! pay (**s. 311**). The **Debt Repayment Scheme** (DRS), administered by the
//! Official Assignee, is a voluntary alternative for debtors whose aggregate
//! debts do not exceed SGD 150,000 (**s. 289**).
//!
//! ## Key Sections at a Glance
//!
//! | Section | Provision |
//! |---------|-----------|
//! | s. 64 | Automatic 30-day moratorium on a scheme application |
//! | s. 67 | Super-priority rescue financing |
//! | s. 70 | Cross-class cram-down ("no creditor worse off") |
//! | s. 89(1) | Purposes of judicial management |
//! | s. 124 | Winding up by the Court |
//! | s. 125(1) | Grounds for compulsory winding up |
//! | s. 125(2) | Deeming tests for inability to pay debts |
//! | s. 161 | Declaration of solvency (members' voluntary) |
//! | s. 210(3AB) | Scheme approval threshold (majority in number, 75% in value) |
//! | s. 289 | Debt Repayment Scheme |
//! | s. 311 | Creditor's bankruptcy application |
//!
//! ## Example
//!
//! ```rust
//! use legalis_sg::insolvency::*;
//!
//! // 1. A creditor serves a statutory demand for SGD 20,000, unsatisfied for
//! //    25 days. Is the company deemed unable to pay its debts?
//! let demand = StatutoryDemand::new("SD-2024-001", "DBS Bank Ltd", "Acme Pte Ltd", 2_000_000)
//!     .with_days_unsatisfied(25);
//!
//! let assessment = validate_statutory_demand(&demand).expect("valid demand");
//! assert!(assessment.deemed_unable);
//! assert_eq!(assessment.test, Some(InabilityToPayTest::StatutoryDemand));
//!
//! // 2. The creditor petitions for compulsory winding up under s. 125(1)(e).
//! let petition = WindingUpPetition::new("WU-2024-001", "Acme Pte Ltd", WindingUpMode::CompulsoryByCourt)
//!     .with_ground(WindingUpGround::UnableToPayDebts)
//!     .with_statutory_demand(demand);
//!
//! let report = validate_winding_up_petition(&petition).expect("report");
//! assert!(report.is_valid);
//!
//! // 3. Alternatively, the company attempts a rescue via judicial management.
//! let jm = JudicialManagementApplication::new("JM-2024-001", "Acme Pte Ltd")
//!     .with_insolvency_limb(true)
//!     .with_purpose(JudicialManagementPurpose::SurvivalAsGoingConcern);
//! assert!(validate_judicial_management(&jm).expect("report").is_grounded);
//!
//! // 4. Or it proposes a scheme of arrangement; a class approves if a majority
//! //    in number AND 75% in value vote in favour (s. 210(3AB)).
//! assert!(scheme_class_approved(3, 4, 800_000, 1_000_000));  // 3/4 in number, 80% in value
//! assert!(!scheme_class_approved(3, 4, 600_000, 1_000_000)); // only 60% in value
//! ```
//!
//! ## Statute References
//!
//! Every key enum exposes `statute_reference()` returning a short citation in the
//! same style as the rest of the crate (e.g. `"IRDA s. 125(1)(e)"`). All errors
//! carry bilingual messages (English + Chinese/华语).
//!
//! ## Module Structure
//!
//! - [`types`] - typed models for winding up, judicial management, schemes and
//!   bankruptcy.
//! - [`error`] - the [`error::InsolvencyError`] enum and [`error::Result`] alias.
//! - [`validator`] - validation and assessment functions with report structs.

pub mod error;
pub mod types;
pub mod validator;

pub use error::{InsolvencyError, Result};
pub use types::{
    AUTOMATIC_MORATORIUM_DAYS, BANKRUPTCY_DEBT_THRESHOLD_CENTS, BankruptcyApplicant,
    BankruptcyApplication, COMPANY_STATUTORY_DEMAND_MINIMUM_CENTS,
    DEBT_REPAYMENT_SCHEME_CEILING_CENTS, DebtRepaymentSchemeProfile, InabilityToPayTest,
    JudicialManagementApplication, JudicialManagementPurpose, SCHEME_VALUE_THRESHOLD_PERCENT,
    STATUTORY_DEMAND_PERIOD_DAYS, SchemeClass, SchemeCreditor, SchemeOfArrangement,
    StatutoryDemand, WindingUpGround, WindingUpMode, WindingUpPetition,
};
pub use validator::{
    InabilityAssessment, JudicialManagementReport, SchemeClassResult, SchemeReport,
    WindingUpReport, assess_debt_repayment_scheme, assess_statutory_demand_inability,
    scheme_class_approved, validate_bankruptcy_application, validate_judicial_management,
    validate_moratorium_period, validate_scheme_of_arrangement, validate_statutory_demand,
    validate_winding_up_petition,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_json_roundtrip_winding_up_petition() {
        let demand = StatutoryDemand::new("SD1", "Bank A", "Acme Pte Ltd", 2_000_000)
            .with_days_unsatisfied(25);
        let petition =
            WindingUpPetition::new("WU1", "Acme Pte Ltd", WindingUpMode::CompulsoryByCourt)
                .with_ground(WindingUpGround::UnableToPayDebts)
                .with_inability_test(InabilityToPayTest::StatutoryDemand)
                .with_statutory_demand(demand);

        let json = serde_json::to_string(&petition).expect("serialise");
        let restored: WindingUpPetition = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(petition, restored);
    }

    #[test]
    fn test_serde_json_roundtrip_scheme_of_arrangement() {
        let scheme = SchemeOfArrangement::new("S1", "Restructure Pte Ltd")
            .with_moratorium(30)
            .with_rescue_financing(true)
            .with_cram_down(true)
            .with_class(
                SchemeClass::new("Unsecured")
                    .with_creditor(SchemeCreditor::new("Creditor A", 800_000).with_vote(true))
                    .with_creditor(SchemeCreditor::new("Creditor B", 200_000).with_vote(false)),
            );

        let json = serde_json::to_string(&scheme).expect("serialise");
        let restored: SchemeOfArrangement = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(scheme, restored);
    }

    #[test]
    fn test_serde_json_roundtrip_judicial_management() {
        let app = JudicialManagementApplication::new("JM1", "Distressed Pte Ltd")
            .with_insolvency_limb(true)
            .with_out_of_court(true)
            .with_purpose(JudicialManagementPurpose::SurvivalAsGoingConcern)
            .with_purpose(JudicialManagementPurpose::ApprovalOfScheme)
            .with_proposed_judicial_manager("Mr Tan, Licensed Insolvency Practitioner");

        let json = serde_json::to_string(&app).expect("serialise");
        let restored: JudicialManagementApplication =
            serde_json::from_str(&json).expect("deserialise");
        assert_eq!(app, restored);
    }

    #[test]
    fn test_serde_json_roundtrip_bankruptcy_application() {
        let app =
            BankruptcyApplication::new("B1", "John Tan", BankruptcyApplicant::Creditor, 2_500_000)
                .with_creditor_name("Bank A")
                .with_inability_to_pay(true)
                .with_unsatisfied_statutory_demand(true);

        let json = serde_json::to_string(&app).expect("serialise");
        let restored: BankruptcyApplication = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(app, restored);
    }

    #[test]
    fn test_end_to_end_winding_up_flow() {
        // End-to-end: statutory demand -> deemed inability -> winding up petition.
        let demand = StatutoryDemand::new("SD2", "Bank A", "Acme Pte Ltd", 3_000_000)
            .with_days_unsatisfied(30);
        let petition =
            WindingUpPetition::new("WU2", "Acme Pte Ltd", WindingUpMode::CompulsoryByCourt)
                .with_ground(WindingUpGround::UnableToPayDebts)
                .with_statutory_demand(demand);

        let report = validate_winding_up_petition(&petition).expect("report");
        assert!(report.is_valid);
        assert!(report.inability.expect("inability").deemed_unable);
        assert!(report.errors.is_empty());
    }
}
