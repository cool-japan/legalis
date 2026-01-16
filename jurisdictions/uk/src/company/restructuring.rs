//! Corporate Restructuring
//!
//! This module implements corporate restructuring mechanisms.
//!
//! ## Schemes of Arrangement (CA 2006 Part 26)
//!
//! ### Mechanism (ss.895-901)
//! Court-sanctioned compromise between company and creditors/members.
//!
//! ### Process
//! 1. Company/administrator/liquidator applies to court
//! 2. Court orders meetings of affected classes
//! 3. Each class must approve by 75% in value and majority in number
//! 4. Court sanctions scheme
//! 5. Scheme registered at Companies House - becomes binding
//!
//! ### Uses
//! - Debt restructuring
//! - Compromises with creditors
//! - Mergers and demergers
//! - Share reorganizations
//!
//! ## Restructuring Plan (CA 2006 Part 26A)
//!
//! ### Cross-Class Cram-Down (s.901G)
//! Court can sanction even if class dissents if:
//! - Members of dissenting class no worse off than in relevant alternative
//! - At least one class that would receive payment in relevant alternative approved
//!
//! ## Mergers (CA 2006 Part 27)
//!
//! ### Merger by Absorption (s.904)
//! Transferor company merges into transferee; transferor dissolved.
//!
//! ### Merger by Formation (s.904A)
//! Two+ companies merge to form new company; original companies dissolved.
//!
//! ## Demergers
//!
//! ### Methods
//! - Direct demerger (dividend in specie)
//! - Indirect demerger (share exchange)
//! - Liquidation demerger

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// ============================================================================
// Schemes of Arrangement (Part 26)
// ============================================================================

/// Scheme of arrangement (CA 2006 ss.895-901)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemeOfArrangement {
    /// Scheme description
    pub description: String,
    /// Type of scheme
    pub scheme_type: SchemeType,
    /// Proposer
    pub proposer: SchemeProposer,
    /// Classes of creditors/members
    pub classes: Vec<SchemeClass>,
    /// Court sanction obtained?
    pub court_sanctioned: bool,
    /// Date of sanction
    pub sanction_date: Option<NaiveDate>,
    /// Effective date
    pub effective_date: Option<NaiveDate>,
    /// Analysis
    pub analysis: String,
}

/// Type of scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemeType {
    /// Compromise with creditors
    CreditorCompromise,
    /// Compromise with members
    MemberCompromise,
    /// Merger/acquisition
    MergerAcquisition,
    /// Demerger/reconstruction
    Reconstruction,
    /// Capital reorganization
    CapitalReorganization,
}

impl SchemeType {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::CreditorCompromise => {
                "Compromise with creditors - typically debt-for-equity swap or payment terms"
            }
            Self::MemberCompromise => "Compromise with shareholders - e.g., cancellation scheme",
            Self::MergerAcquisition => "Scheme to effect merger or acquisition",
            Self::Reconstruction => "Corporate reconstruction or demerger",
            Self::CapitalReorganization => "Reorganization of share capital structure",
        }
    }
}

/// Who can propose scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemeProposer {
    /// Company itself
    Company,
    /// Creditor(s)
    Creditor,
    /// Member(s)
    Member,
    /// Administrator
    Administrator,
    /// Liquidator
    Liquidator,
}

/// Class of creditors/members for scheme voting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemeClass {
    /// Class name
    pub name: String,
    /// Class type
    pub class_type: SchemeClassType,
    /// Number of class members
    pub number_of_members: u32,
    /// Total value of claims/shares
    pub total_value: f64,
    /// Votes for (by value)
    pub votes_for_value: f64,
    /// Votes for (by number)
    pub votes_for_number: u32,
    /// Approved by class?
    pub approved: bool,
}

/// Type of scheme class
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemeClassType {
    /// Secured creditors
    SecuredCreditors,
    /// Unsecured creditors
    UnsecuredCreditors,
    /// Preferential creditors
    PreferentialCreditors,
    /// Shareholders
    Shareholders,
    /// Bondholders
    Bondholders,
    /// Trade creditors
    TradeCreditors,
    /// Financial creditors
    FinancialCreditors,
}

impl SchemeClass {
    /// Check if class has approved scheme
    /// Requires: 75% by value AND majority by number (s.899(1))
    pub fn check_approval(&mut self) {
        let value_percent = if self.total_value > 0.0 {
            (self.votes_for_value / self.total_value) * 100.0
        } else {
            0.0
        };

        let number_percent = if self.number_of_members > 0 {
            (self.votes_for_number as f64 / self.number_of_members as f64) * 100.0
        } else {
            0.0
        };

        self.approved = value_percent >= 75.0 && number_percent > 50.0;
    }
}

impl SchemeOfArrangement {
    /// Analyze scheme approval
    pub fn analyze(
        description: &str,
        scheme_type: SchemeType,
        proposer: SchemeProposer,
        mut classes: Vec<SchemeClass>,
    ) -> Self {
        // Check approval for each class
        for class in &mut classes {
            class.check_approval();
        }

        let all_approved = classes.iter().all(|c| c.approved);
        let court_sanctioned = all_approved; // Simplified - court has discretion

        let analysis = if all_approved {
            format!(
                "Scheme of arrangement - all {} classes approved. Each class achieved \
                 75%+ by value and majority by number (s.899(1)). Court may sanction.",
                classes.len()
            )
        } else {
            let failed_classes: Vec<_> = classes
                .iter()
                .filter(|c| !c.approved)
                .map(|c| c.name.as_str())
                .collect();
            format!(
                "Scheme NOT approved by all classes. Failed classes: {}. \
                 Consider Part 26A restructuring plan for cross-class cram-down.",
                failed_classes.join(", ")
            )
        };

        Self {
            description: description.to_string(),
            scheme_type,
            proposer,
            classes,
            court_sanctioned,
            sanction_date: None,
            effective_date: None,
            analysis,
        }
    }
}

// ============================================================================
// Restructuring Plan (Part 26A)
// ============================================================================

/// Restructuring plan with cross-class cram-down (ss.901A-901L)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestructuringPlan {
    /// Plan description
    pub description: String,
    /// Classes
    pub classes: Vec<RestructuringClass>,
    /// Relevant alternative (baseline comparator)
    pub relevant_alternative: RelevantAlternative,
    /// Cross-class cram-down sought?
    pub cram_down_sought: bool,
    /// Cram-down conditions met?
    pub cram_down_available: bool,
    /// Court sanctioned?
    pub court_sanctioned: bool,
    /// Analysis
    pub analysis: String,
}

/// Class in restructuring plan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestructuringClass {
    /// Class name
    pub name: String,
    /// Estimated recovery under plan
    pub plan_recovery_percent: f64,
    /// Estimated recovery under relevant alternative
    pub alternative_recovery_percent: f64,
    /// Would receive payment in relevant alternative
    pub receives_in_alternative: bool,
    /// Class approved?
    pub approved: bool,
    /// Better off under plan?
    pub no_worse_off: bool,
}

impl RestructuringClass {
    /// Check if class is no worse off
    pub fn check_no_worse_off(&mut self) {
        self.no_worse_off = self.plan_recovery_percent >= self.alternative_recovery_percent;
    }
}

/// Relevant alternative for cram-down comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelevantAlternative {
    /// Liquidation
    Liquidation,
    /// Administration
    Administration,
    /// Different restructuring
    DifferentRestructuring,
    /// Continuation (if viable)
    Continuation,
}

impl RestructuringPlan {
    /// Analyze restructuring plan
    pub fn analyze(
        description: &str,
        mut classes: Vec<RestructuringClass>,
        relevant_alternative: RelevantAlternative,
    ) -> Self {
        // Check no worse off for each class
        for class in &mut classes {
            class.check_no_worse_off();
        }

        let all_approved = classes.iter().all(|c| c.approved);
        let all_no_worse_off = classes.iter().all(|c| c.no_worse_off);

        // At least one in-the-money class must approve (s.901G(3))
        let in_money_class_approved = classes
            .iter()
            .any(|c| c.receives_in_alternative && c.approved);

        // Cram-down available if conditions met (s.901G)
        let cram_down_available = all_no_worse_off && in_money_class_approved;
        let cram_down_sought = !all_approved && cram_down_available;

        let court_sanctioned = all_approved || cram_down_available;

        let analysis = if all_approved {
            "Restructuring plan approved by all classes. No cram-down needed.".to_string()
        } else if cram_down_available {
            let dissenting: Vec<_> = classes
                .iter()
                .filter(|c| !c.approved)
                .map(|c| c.name.as_str())
                .collect();
            format!(
                "Cross-class cram-down AVAILABLE (s.901G). Dissenting classes: {}. \
                 Conditions met: (1) all classes no worse off than {:?}; \
                 (2) at least one in-the-money class approved.",
                dissenting.join(", "),
                relevant_alternative
            )
        } else if !all_no_worse_off {
            let worse_off: Vec<_> = classes
                .iter()
                .filter(|c| !c.no_worse_off)
                .map(|c| c.name.as_str())
                .collect();
            format!(
                "Cram-down NOT available. Classes worse off than alternative: {}. \
                 s.901G(2) condition not satisfied.",
                worse_off.join(", ")
            )
        } else {
            "Cram-down NOT available. No in-the-money class approved (s.901G(3)).".to_string()
        };

        Self {
            description: description.to_string(),
            classes,
            relevant_alternative,
            cram_down_sought,
            cram_down_available,
            court_sanctioned,
            analysis,
        }
    }
}

// ============================================================================
// Mergers (Part 27)
// ============================================================================

/// Type of merger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MergerType {
    /// Absorption - transferor merges into transferee (s.904)
    Absorption,
    /// Formation - companies merge to form new company (s.904A)
    Formation,
}

impl MergerType {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Absorption => {
                "Merger by absorption (s.904): Transferor company merges into existing \
                 transferee company. Transferor dissolved without winding up."
            }
            Self::Formation => {
                "Merger by formation (s.904A): Two or more companies merge to form \
                 new company. All merging companies dissolved without winding up."
            }
        }
    }
}

/// Merger analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergerAnalysis {
    /// Merger type
    pub merger_type: MergerType,
    /// Transferor companies
    pub transferors: Vec<String>,
    /// Transferee company (or new company name)
    pub transferee: String,
    /// Draft terms agreed?
    pub draft_terms_agreed: bool,
    /// Directors' report prepared?
    pub directors_report: bool,
    /// Expert report (if required)?
    pub expert_report: bool,
    /// Member approval obtained?
    pub member_approval: bool,
    /// Creditor protection
    pub creditor_protection: CreditorProtection,
    /// Analysis
    pub analysis: String,
}

/// Creditor protection in merger
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditorProtection {
    /// Notice given to creditors?
    pub notice_given: bool,
    /// Can creditors object?
    pub objection_period: bool,
    /// Security provided for objecting creditors?
    pub security_provided: bool,
}

impl MergerAnalysis {
    /// Analyze merger requirements
    #[allow(clippy::too_many_arguments)]
    pub fn analyze(
        merger_type: MergerType,
        transferors: Vec<String>,
        transferee: &str,
        draft_terms: bool,
        directors_report: bool,
        expert_report: bool,
        member_approval: bool,
        creditor_protection: CreditorProtection,
    ) -> Self {
        let requirements_met =
            draft_terms && directors_report && member_approval && creditor_protection.notice_given;

        let analysis = if requirements_met {
            format!(
                "{:?} merger - requirements MET. {} transferor(s) merging into {}. \
                 Draft terms approved, directors' and expert reports prepared, \
                 member approval obtained. Creditor notice given.",
                merger_type,
                transferors.len(),
                transferee
            )
        } else {
            let mut missing = Vec::new();
            if !draft_terms {
                missing.push("draft terms");
            }
            if !directors_report {
                missing.push("directors' report");
            }
            if !member_approval {
                missing.push("member approval");
            }
            if !creditor_protection.notice_given {
                missing.push("creditor notice");
            }
            format!(
                "Merger requirements NOT MET. Missing: {}.",
                missing.join(", ")
            )
        };

        Self {
            merger_type,
            transferors,
            transferee: transferee.to_string(),
            draft_terms_agreed: draft_terms,
            directors_report,
            expert_report,
            member_approval,
            creditor_protection,
            analysis,
        }
    }
}

// ============================================================================
// Demergers
// ============================================================================

/// Type of demerger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DemergerType {
    /// Direct demerger - dividend in specie to shareholders
    Direct,
    /// Indirect demerger - share exchange
    Indirect,
    /// Liquidation demerger
    Liquidation,
}

impl DemergerType {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Direct => {
                "Direct demerger: Parent distributes shares in subsidiary to shareholders \
                 as dividend in specie. Requires distributable profits."
            }
            Self::Indirect => {
                "Indirect demerger: New holding company inserted, followed by distribution \
                 of subsidiary. Uses scheme of arrangement."
            }
            Self::Liquidation => {
                "Liquidation demerger: Company liquidated and assets distributed. \
                 Uses s.110 IA 1986 reconstruction."
            }
        }
    }

    /// Tax implications summary
    pub fn tax_considerations(&self) -> &'static str {
        match self {
            Self::Direct => {
                "Direct: May qualify for exemption under CTA 2010 Pt 23 Ch 5 \
                 (substantial shareholding exemption) or TCGA 1992 s.192A."
            }
            Self::Indirect => {
                "Indirect: May use TCGA 1992 s.135/136 share-for-share exchange. \
                 More complex but flexible."
            }
            Self::Liquidation => {
                "Liquidation: ESC C16 may defer capital gains. Members receive \
                 distribution as return of capital."
            }
        }
    }
}

/// Demerger analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemergerAnalysis {
    /// Demerger type
    pub demerger_type: DemergerType,
    /// Parent company
    pub parent: String,
    /// Businesses being demerged
    pub businesses: Vec<String>,
    /// Distributable profits (for direct demerger)
    pub distributable_profits: Option<f64>,
    /// Value of assets to be distributed
    pub distribution_value: f64,
    /// Is demerger feasible?
    pub feasible: bool,
    /// Analysis
    pub analysis: String,
}

impl DemergerAnalysis {
    /// Analyze demerger
    pub fn analyze(
        demerger_type: DemergerType,
        parent: &str,
        businesses: Vec<String>,
        distributable_profits: Option<f64>,
        distribution_value: f64,
    ) -> Self {
        let feasible = match demerger_type {
            DemergerType::Direct => distributable_profits.is_some_and(|p| p >= distribution_value),
            DemergerType::Indirect | DemergerType::Liquidation => true,
        };

        let analysis = match demerger_type {
            DemergerType::Direct => {
                if feasible {
                    format!(
                        "Direct demerger FEASIBLE. {} has distributable profits of £{:.2} \
                         (value to distribute: £{:.2}). Can distribute shares in {} \
                         as dividend in specie.",
                        parent,
                        distributable_profits.unwrap_or(0.0),
                        distribution_value,
                        businesses.join(", ")
                    )
                } else {
                    format!(
                        "Direct demerger NOT FEASIBLE. Insufficient distributable profits \
                         (£{:.2} available, £{:.2} needed). Consider indirect or liquidation demerger.",
                        distributable_profits.unwrap_or(0.0),
                        distribution_value
                    )
                }
            }
            DemergerType::Indirect => {
                format!(
                    "Indirect demerger via scheme of arrangement. New holding company \
                     to be inserted above {}. {} to be demerged. No distributable \
                     profits requirement.",
                    parent,
                    businesses.join(", ")
                )
            }
            DemergerType::Liquidation => {
                format!(
                    "Liquidation demerger under s.110 IA 1986. {} to be placed in members' \
                     voluntary liquidation. Assets ({}) distributed to members. \
                     Requires s.89 solvency declaration.",
                    parent,
                    businesses.join(", ")
                )
            }
        };

        Self {
            demerger_type,
            parent: parent.to_string(),
            businesses,
            distributable_profits,
            distribution_value,
            feasible,
            analysis,
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
    fn test_scheme_class_approval() {
        let mut class = SchemeClass {
            name: "Unsecured Creditors".to_string(),
            class_type: SchemeClassType::UnsecuredCreditors,
            number_of_members: 100,
            total_value: 1_000_000.0,
            votes_for_value: 800_000.0, // 80%
            votes_for_number: 60,       // 60%
            approved: false,
        };
        class.check_approval();
        assert!(class.approved);
    }

    #[test]
    fn test_scheme_class_fails_number() {
        let mut class = SchemeClass {
            name: "Bondholders".to_string(),
            class_type: SchemeClassType::Bondholders,
            number_of_members: 100,
            total_value: 1_000_000.0,
            votes_for_value: 800_000.0, // 80% by value
            votes_for_number: 45,       // Only 45% by number
            approved: false,
        };
        class.check_approval();
        assert!(!class.approved);
    }

    #[test]
    fn test_restructuring_plan_cram_down() {
        let classes = vec![
            RestructuringClass {
                name: "Senior Secured".to_string(),
                plan_recovery_percent: 100.0,
                alternative_recovery_percent: 100.0,
                receives_in_alternative: true,
                approved: true,
                no_worse_off: true,
            },
            RestructuringClass {
                name: "Unsecured".to_string(),
                plan_recovery_percent: 20.0,
                alternative_recovery_percent: 10.0,
                receives_in_alternative: true,
                approved: false, // Dissenting
                no_worse_off: true,
            },
        ];

        let plan = RestructuringPlan::analyze(
            "Debt restructuring",
            classes,
            RelevantAlternative::Liquidation,
        );

        assert!(plan.cram_down_available);
        assert!(plan.court_sanctioned);
    }

    #[test]
    fn test_restructuring_plan_no_cram_down_worse_off() {
        let classes = vec![
            RestructuringClass {
                name: "Senior".to_string(),
                plan_recovery_percent: 100.0,
                alternative_recovery_percent: 100.0,
                receives_in_alternative: true,
                approved: true,
                no_worse_off: true,
            },
            RestructuringClass {
                name: "Junior".to_string(),
                plan_recovery_percent: 5.0,
                alternative_recovery_percent: 15.0, // Worse off
                receives_in_alternative: true,
                approved: false,
                no_worse_off: false,
            },
        ];

        let plan = RestructuringPlan::analyze(
            "Restructuring",
            classes,
            RelevantAlternative::Administration,
        );

        assert!(!plan.cram_down_available);
    }

    #[test]
    fn test_direct_demerger_feasible() {
        let analysis = DemergerAnalysis::analyze(
            DemergerType::Direct,
            "Parent Ltd",
            vec!["Subsidiary A".to_string()],
            Some(5_000_000.0),
            3_000_000.0,
        );
        assert!(analysis.feasible);
    }

    #[test]
    fn test_direct_demerger_insufficient_profits() {
        let analysis = DemergerAnalysis::analyze(
            DemergerType::Direct,
            "Parent Ltd",
            vec!["Subsidiary B".to_string()],
            Some(1_000_000.0),
            3_000_000.0,
        );
        assert!(!analysis.feasible);
    }

    #[test]
    fn test_liquidation_demerger_always_feasible() {
        let analysis = DemergerAnalysis::analyze(
            DemergerType::Liquidation,
            "Parent Ltd",
            vec!["Business 1".to_string(), "Business 2".to_string()],
            None, // No distributable profits needed
            10_000_000.0,
        );
        assert!(analysis.feasible);
    }
}
