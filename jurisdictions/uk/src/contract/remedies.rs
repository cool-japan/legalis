//! Contract Remedies
//!
//! This module implements contract remedies under English law.
//!
//! ## Damages
//!
//! The primary remedy for breach of contract.
//!
//! ### Purpose (Robinson v Harman \[1848\])
//! Put the claimant in the position they would have been in had the contract
//! been properly performed (expectation interest).
//!
//! ### Remoteness (Hadley v Baxendale \[1854\])
//! Damages only recoverable for losses that:
//! - **Limb 1**: Arise naturally from the breach (in usual course of things)
//! - **Limb 2**: Were in the reasonable contemplation of both parties at contract formation
//!
//! ### Mitigation
//! Claimant must take reasonable steps to mitigate loss
//! (British Westinghouse v Underground Electric Railways \[1912\])
//!
//! ### Measure of Damages
//! - **Expectation**: What claimant expected to receive
//! - **Reliance**: Expenses incurred in reliance on contract (Anglia TV v Reed)
//! - **Restitution**: Disgorgement of defendant's gain (Attorney General v Blake)
//!
//! ## Specific Performance
//!
//! Equitable remedy - court orders performance of contract.
//!
//! ### Availability
//! - Only where damages inadequate (unique goods, land)
//! - Not for personal service contracts (Co-operative Insurance v Argyll)
//! - Not for contracts requiring constant supervision
//!
//! ### Bars
//! - Delay (laches)
//! - Hardship
//! - Claimant's conduct (he who comes to equity must come with clean hands)
//!
//! ## Injunction
//!
//! Equitable remedy - court orders party to do/not do something.
//!
//! ### Types
//! - **Prohibitory**: Stop doing something
//! - **Mandatory**: Compel positive action
//!
//! ### Availability
//! Similar principles to specific performance.
//!
//! ## Rescission
//!
//! Setting aside the contract ab initio.

use serde::{Deserialize, Serialize};

use super::terms::TermType;

// ============================================================================
// Damages
// ============================================================================

/// Measure of damages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamagesMeasure {
    /// Expectation damages - put in position if contract performed
    Expectation,
    /// Reliance damages - recover wasted expenditure
    Reliance,
    /// Restitution - recover benefit conferred
    Restitution,
    /// Wrotham Park damages - reasonable fee for breach (Experience Hendrix)
    WrothamPark,
    /// Account of profits - disgorge defendant's gain (AG v Blake - rare)
    AccountOfProfits,
}

impl DamagesMeasure {
    /// Get description with case law reference
    pub fn description(&self) -> &'static str {
        match self {
            Self::Expectation => {
                "Put claimant in position they would have been in had contract been \
                 properly performed (Robinson v Harman [1848])"
            }
            Self::Reliance => {
                "Recover expenses incurred in reliance on contract (Anglia Television v Reed \
                 [1972]). Cannot recover where would have made loss anyway."
            }
            Self::Restitution => "Recover value of benefit conferred on defendant",
            Self::WrothamPark => {
                "Reasonable fee defendant would have paid for release from covenant \
                 (Wrotham Park Estate v Parkside Homes [1974])"
            }
            Self::AccountOfProfits => {
                "Exceptional remedy - disgorge defendant's profit from breach \
                 (Attorney General v Blake [2001])"
            }
        }
    }

    /// Is this the primary measure?
    pub fn is_primary(&self) -> bool {
        matches!(self, Self::Expectation)
    }
}

/// Remoteness test under Hadley v Baxendale
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemotenessAnalysis {
    /// The loss claimed
    pub loss_description: String,
    /// Amount of loss
    pub loss_amount: f64,
    /// Does loss arise naturally (Limb 1)?
    pub arises_naturally: bool,
    /// Was loss in contemplation at formation (Limb 2)?
    pub in_contemplation: bool,
    /// Any special knowledge communicated?
    pub special_knowledge: Option<String>,
    /// Is loss too remote?
    pub is_remote: bool,
    /// Analysis
    pub analysis: String,
}

impl RemotenessAnalysis {
    /// Analyze remoteness of loss
    pub fn analyze(
        loss_description: &str,
        loss_amount: f64,
        arises_naturally: bool,
        in_contemplation: bool,
        special_knowledge: Option<&str>,
    ) -> Self {
        let is_remote = !arises_naturally && !in_contemplation;

        let analysis = if arises_naturally {
            format!(
                "Loss of £{:.2} for '{}' is recoverable under Hadley v Baxendale [1854] \
                 Limb 1 - it arises naturally from the breach in the usual course of things.",
                loss_amount, loss_description
            )
        } else if in_contemplation {
            let knowledge_note = special_knowledge
                .map(|k| format!(" Special knowledge communicated: '{}'.", k))
                .unwrap_or_default();
            format!(
                "Loss of £{:.2} for '{}' is recoverable under Hadley v Baxendale [1854] \
                 Limb 2 - it was in the reasonable contemplation of both parties at \
                 contract formation.{}",
                loss_amount, loss_description, knowledge_note
            )
        } else {
            format!(
                "Loss of £{:.2} for '{}' is TOO REMOTE. Following Hadley v Baxendale \
                 [1854], it neither arises naturally nor was in the contemplation of \
                 the parties at formation. Following Victoria Laundry v Newman [1949], \
                 defendant not liable for losses they could not reasonably contemplate.",
                loss_amount, loss_description
            )
        };

        Self {
            loss_description: loss_description.to_string(),
            loss_amount,
            arises_naturally,
            in_contemplation,
            special_knowledge: special_knowledge.map(|s| s.to_string()),
            is_remote,
            analysis,
        }
    }
}

/// Mitigation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MitigationAnalysis {
    /// Steps claimant took to mitigate
    pub steps_taken: Vec<String>,
    /// Were steps reasonable?
    pub steps_reasonable: bool,
    /// Amount saved by mitigation
    pub amount_saved: f64,
    /// Amount that could have been saved
    pub could_have_saved: f64,
    /// Deduction for failure to mitigate
    pub deduction: f64,
    /// Analysis
    pub analysis: String,
}

impl MitigationAnalysis {
    /// Analyze mitigation
    pub fn analyze(
        steps_taken: Vec<String>,
        steps_reasonable: bool,
        amount_saved: f64,
        could_have_saved: f64,
    ) -> Self {
        let deduction = if steps_reasonable {
            0.0
        } else {
            could_have_saved - amount_saved
        };

        let analysis = if steps_reasonable {
            format!(
                "Claimant took reasonable steps to mitigate: {}. No deduction required \
                 (British Westinghouse v Underground Electric Railways [1912]).",
                if steps_taken.is_empty() {
                    "none required".to_string()
                } else {
                    steps_taken.join("; ")
                }
            )
        } else {
            format!(
                "Claimant failed to take reasonable steps to mitigate. Could have saved \
                 £{:.2} more. Damages reduced by this amount. Note: claimant not required \
                 to take unreasonable steps (Pilkington v Wood [1953]).",
                deduction
            )
        };

        Self {
            steps_taken,
            steps_reasonable,
            amount_saved,
            could_have_saved,
            deduction,
            analysis,
        }
    }
}

/// Damages calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamagesCalculation {
    /// Measure applied
    pub measure: DamagesMeasure,
    /// Gross amount before deductions
    pub gross_amount: f64,
    /// Remoteness analysis for each head of loss
    pub remoteness: Vec<RemotenessAnalysis>,
    /// Mitigation analysis
    pub mitigation: MitigationAnalysis,
    /// Amount recoverable for each limb
    pub limb1_recovery: f64,
    /// Limb 2 recovery
    pub limb2_recovery: f64,
    /// Total deductions (remoteness + mitigation)
    pub total_deductions: f64,
    /// Net recoverable damages
    pub net_damages: f64,
    /// Summary
    pub summary: String,
}

impl DamagesCalculation {
    /// Calculate damages
    pub fn calculate(
        measure: DamagesMeasure,
        losses: Vec<RemotenessAnalysis>,
        mitigation: MitigationAnalysis,
    ) -> Self {
        let gross_amount: f64 = losses.iter().map(|l| l.loss_amount).sum();

        let limb1_recovery: f64 = losses
            .iter()
            .filter(|l| l.arises_naturally && !l.is_remote)
            .map(|l| l.loss_amount)
            .sum();

        let limb2_recovery: f64 = losses
            .iter()
            .filter(|l| !l.arises_naturally && l.in_contemplation && !l.is_remote)
            .map(|l| l.loss_amount)
            .sum();

        let remote_losses: f64 = losses
            .iter()
            .filter(|l| l.is_remote)
            .map(|l| l.loss_amount)
            .sum();

        let total_deductions = remote_losses + mitigation.deduction;
        let net_damages = gross_amount - total_deductions;

        let summary = format!(
            "{} measure applied. Gross losses: £{:.2}. Limb 1 recovery: £{:.2}. \
             Limb 2 recovery: £{:.2}. Remote losses (not recoverable): £{:.2}. \
             Mitigation deduction: £{:.2}. Net damages: £{:.2}.",
            match measure {
                DamagesMeasure::Expectation => "Expectation",
                DamagesMeasure::Reliance => "Reliance",
                DamagesMeasure::Restitution => "Restitution",
                DamagesMeasure::WrothamPark => "Wrotham Park",
                DamagesMeasure::AccountOfProfits => "Account of profits",
            },
            gross_amount,
            limb1_recovery,
            limb2_recovery,
            remote_losses,
            mitigation.deduction,
            net_damages
        );

        Self {
            measure,
            gross_amount,
            remoteness: losses,
            mitigation,
            limb1_recovery,
            limb2_recovery,
            total_deductions,
            net_damages,
            summary,
        }
    }
}

// ============================================================================
// Specific Performance
// ============================================================================

/// Availability of specific performance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecificPerformanceAnalysis {
    /// Subject matter of contract
    pub subject_matter: SubjectMatter,
    /// Are damages adequate?
    pub damages_adequate: bool,
    /// Would order require constant supervision?
    pub requires_supervision: bool,
    /// Is it a personal service contract?
    pub personal_service: bool,
    /// Any bars to remedy?
    pub bars: Vec<EquitableBar>,
    /// Is specific performance available?
    pub available: bool,
    /// Analysis
    pub analysis: String,
}

/// Subject matter of contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubjectMatter {
    /// Land (usually available)
    Land,
    /// Unique goods (may be available)
    UniqueGoods,
    /// Generic goods (usually not available)
    GenericGoods,
    /// Personal services (not available)
    PersonalServices,
    /// Shares in private company (may be available)
    PrivateCompanyShares,
    /// Other
    Other,
}

/// Bars to equitable remedies
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquitableBar {
    /// Delay (laches)
    Laches,
    /// Hardship to defendant
    Hardship,
    /// Claimant's misconduct (clean hands)
    UncleansHands,
    /// Claimant already has adequate remedy at law
    AdequateLegalRemedy,
    /// Mutuality lacking
    LackOfMutuality,
    /// Contract unenforceable/illegal
    UnenforceableContract,
}

impl SpecificPerformanceAnalysis {
    /// Analyze availability of specific performance
    pub fn analyze(
        subject_matter: SubjectMatter,
        damages_adequate: bool,
        requires_supervision: bool,
        personal_service: bool,
        bars: Vec<EquitableBar>,
    ) -> Self {
        // Basic availability
        let mut available =
            !damages_adequate && !requires_supervision && !personal_service && bars.is_empty();

        // Subject matter considerations
        match subject_matter {
            SubjectMatter::Land => {
                // Traditionally available for land (each piece unique)
                if bars.is_empty() && !requires_supervision {
                    available = true;
                }
            }
            SubjectMatter::UniqueGoods => {
                // Available if goods genuinely unique
                available = !damages_adequate && bars.is_empty();
            }
            SubjectMatter::PersonalServices | SubjectMatter::GenericGoods => {
                available = false;
            }
            _ => {}
        }

        let analysis = Self::generate_analysis(
            subject_matter,
            damages_adequate,
            requires_supervision,
            personal_service,
            &bars,
            available,
        );

        Self {
            subject_matter,
            damages_adequate,
            requires_supervision,
            personal_service,
            bars,
            available,
            analysis,
        }
    }

    fn generate_analysis(
        subject_matter: SubjectMatter,
        damages_adequate: bool,
        requires_supervision: bool,
        personal_service: bool,
        bars: &[EquitableBar],
        available: bool,
    ) -> String {
        let mut analysis = String::new();

        if damages_adequate {
            analysis.push_str("Damages would be an adequate remedy. ");
        }

        if personal_service {
            analysis.push_str(
                "This is a personal service contract - specific performance not available \
                 (Co-operative Insurance Society v Argyll Stores [1998]). ",
            );
        }

        if requires_supervision {
            analysis.push_str(
                "Order would require constant supervision - courts reluctant to grant \
                 (Ryan v Mutual Tontine Westminster Chambers [1893]). ",
            );
        }

        match subject_matter {
            SubjectMatter::Land => {
                analysis.push_str(
                    "Contract relates to land. Traditionally specific performance available \
                     as each piece of land is unique. ",
                );
            }
            SubjectMatter::UniqueGoods => {
                analysis.push_str(
                    "Contract relates to unique goods. Specific performance may be available \
                     under SGA 1979 s.52. ",
                );
            }
            SubjectMatter::GenericGoods => {
                analysis.push_str(
                    "Contract relates to generic goods - available on market. Damages adequate. ",
                );
            }
            _ => {}
        }

        for bar in bars {
            match bar {
                EquitableBar::Laches => {
                    analysis.push_str("Delay (laches) may bar remedy. ");
                }
                EquitableBar::Hardship => {
                    analysis.push_str(
                        "Would cause hardship to defendant disproportionate to benefit to claimant. ",
                    );
                }
                EquitableBar::UncleansHands => {
                    analysis.push_str("'He who comes to equity must come with clean hands.' ");
                }
                EquitableBar::AdequateLegalRemedy => {
                    analysis.push_str("Adequate remedy at law available. ");
                }
                EquitableBar::LackOfMutuality => {
                    analysis.push_str("Lack of mutuality. ");
                }
                EquitableBar::UnenforceableContract => {
                    analysis.push_str("Contract is unenforceable. ");
                }
            }
        }

        analysis.push_str(&format!(
            "Specific performance is {}.",
            if available {
                "AVAILABLE"
            } else {
                "NOT AVAILABLE"
            }
        ));

        analysis
    }
}

// ============================================================================
// Injunction
// ============================================================================

/// Type of injunction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InjunctionType {
    /// Prohibitory - stop doing something
    Prohibitory,
    /// Mandatory - compel positive action
    Mandatory,
    /// Interim/Interlocutory - pending trial
    Interim,
    /// Final - after trial
    Final,
}

/// Injunction analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjunctionAnalysis {
    /// Type of injunction sought
    pub injunction_type: InjunctionType,
    /// What is sought to be restrained/compelled?
    pub subject: String,
    /// Would damages be adequate?
    pub damages_adequate: bool,
    /// Any bars?
    pub bars: Vec<EquitableBar>,
    /// For interim: American Cyanamid factors
    pub interim_factors: Option<AmericanCyanamidFactors>,
    /// Is injunction available?
    pub available: bool,
    /// Analysis
    pub analysis: String,
}

/// American Cyanamid factors for interim injunctions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmericanCyanamidFactors {
    /// Is there a serious question to be tried?
    pub serious_question: bool,
    /// Would damages be adequate remedy for claimant?
    pub damages_adequate_claimant: bool,
    /// Would damages be adequate remedy for defendant?
    pub damages_adequate_defendant: bool,
    /// Where does balance of convenience lie?
    pub balance_of_convenience: BalanceOfConvenience,
    /// Any special factors?
    pub special_factors: Vec<String>,
}

/// Balance of convenience
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BalanceOfConvenience {
    /// Favors granting injunction
    FavorsInjunction,
    /// Favors refusing injunction
    FavorsRefusal,
    /// Evenly balanced (preserve status quo)
    EvenlyBalanced,
}

impl InjunctionAnalysis {
    /// Analyze interim injunction using American Cyanamid
    pub fn analyze_interim(
        subject: &str,
        factors: AmericanCyanamidFactors,
        bars: Vec<EquitableBar>,
    ) -> Self {
        let available = factors.serious_question
            && !factors.damages_adequate_claimant
            && matches!(
                factors.balance_of_convenience,
                BalanceOfConvenience::FavorsInjunction | BalanceOfConvenience::EvenlyBalanced
            )
            && bars.is_empty();

        let analysis = format!(
            "Interim injunction to {}. Applying American Cyanamid [1975]: \
             Serious question to be tried: {}. Damages adequate for claimant: {}. \
             Balance of convenience: {:?}. Injunction is {}.",
            subject,
            if factors.serious_question {
                "Yes"
            } else {
                "No"
            },
            if factors.damages_adequate_claimant {
                "Yes"
            } else {
                "No"
            },
            factors.balance_of_convenience,
            if available { "GRANTED" } else { "REFUSED" }
        );

        Self {
            injunction_type: InjunctionType::Interim,
            subject: subject.to_string(),
            damages_adequate: factors.damages_adequate_claimant,
            bars,
            interim_factors: Some(factors),
            available,
            analysis,
        }
    }

    /// Analyze final injunction
    pub fn analyze_final(
        subject: &str,
        injunction_type: InjunctionType,
        damages_adequate: bool,
        bars: Vec<EquitableBar>,
    ) -> Self {
        let available = !damages_adequate && bars.is_empty();

        let analysis = format!(
            "{:?} injunction to {}. Damages adequate: {}. Bars: {}. Injunction is {}.",
            injunction_type,
            subject,
            if damages_adequate { "Yes" } else { "No" },
            if bars.is_empty() {
                "None".to_string()
            } else {
                format!("{:?}", bars)
            },
            if available {
                "AVAILABLE"
            } else {
                "NOT AVAILABLE"
            }
        );

        Self {
            injunction_type,
            subject: subject.to_string(),
            damages_adequate,
            bars,
            interim_factors: None,
            available,
            analysis,
        }
    }
}

// ============================================================================
// Rescission
// ============================================================================

/// Grounds for rescission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RescissionGround {
    /// Misrepresentation
    Misrepresentation,
    /// Mistake (limited)
    Mistake,
    /// Duress
    Duress,
    /// Undue influence
    UndueInfluence,
    /// Breach of fiduciary duty
    BreachOfFiduciaryDuty,
}

/// Bars to rescission
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RescissionBar {
    /// Affirmation after discovering ground
    Affirmation,
    /// Lapse of time (Leaf v International Galleries)
    LapseOfTime {
        /// Time elapsed
        time_elapsed: String,
    },
    /// Restitutio in integrum impossible
    RestitutioImpossible,
    /// Third party acquired rights
    ThirdPartyRights,
}

/// Rescission analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RescissionAnalysis {
    /// Ground for rescission
    pub ground: RescissionGround,
    /// Any bars?
    pub bars: Vec<RescissionBar>,
    /// Can restitution be made?
    pub restitutio_possible: bool,
    /// Is rescission available?
    pub available: bool,
    /// Analysis
    pub analysis: String,
}

impl RescissionAnalysis {
    /// Analyze availability of rescission
    pub fn analyze(
        ground: RescissionGround,
        bars: Vec<RescissionBar>,
        restitutio_possible: bool,
    ) -> Self {
        let available = bars.is_empty() && restitutio_possible;

        let analysis = format!(
            "Rescission sought on ground of {:?}. Bars: {}. Restitutio in integrum: {}. \
             Rescission is {}. Effect: Contract set aside ab initio.",
            ground,
            if bars.is_empty() {
                "None".to_string()
            } else {
                format!("{:?}", bars)
            },
            if restitutio_possible {
                "Possible"
            } else {
                "Impossible"
            },
            if available { "AVAILABLE" } else { "BARRED" }
        );

        Self {
            ground,
            bars,
            restitutio_possible,
            available,
            analysis,
        }
    }
}

// ============================================================================
// Termination for Breach
// ============================================================================

/// Analysis of right to terminate for breach
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerminationAnalysis {
    /// Type of term breached
    pub term_type: TermType,
    /// Nature of breach
    pub breach_nature: BreachNature,
    /// For innominate terms: deprivation analysis
    pub deprivation_analysis: Option<DeprivationAnalysis>,
    /// Right to terminate?
    pub can_terminate: bool,
    /// Alternative: affirm and claim damages
    pub can_affirm: bool,
    /// Has right to terminate been lost?
    pub right_lost: bool,
    /// Reason if lost
    pub right_lost_reason: Option<String>,
    /// Analysis
    pub analysis: String,
}

/// Nature of the breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachNature {
    /// Actual breach - already occurred
    Actual,
    /// Anticipatory breach - clear refusal before performance due
    Anticipatory,
}

/// Deprivation analysis for innominate terms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeprivationAnalysis {
    /// Benefit claimant expected from contract
    pub expected_benefit: String,
    /// Benefit claimant actually received
    pub actual_benefit: String,
    /// Was claimant deprived of substantially the whole benefit?
    pub substantial_deprivation: bool,
    /// Analysis
    pub analysis: String,
}

impl TerminationAnalysis {
    /// Analyze right to terminate
    pub fn analyze(
        term_type: TermType,
        breach_nature: BreachNature,
        deprivation: Option<DeprivationAnalysis>,
        affirmation_occurred: bool,
    ) -> Self {
        let can_terminate = match term_type {
            TermType::Condition => true,
            TermType::Warranty => false,
            TermType::Innominate => deprivation
                .as_ref()
                .map(|d| d.substantial_deprivation)
                .unwrap_or(false),
        };

        let right_lost = affirmation_occurred;
        let right_lost_reason = if affirmation_occurred {
            Some(
                "Right to terminate lost by affirmation (Yukong Line v Rendsburg [1998])"
                    .to_string(),
            )
        } else {
            None
        };

        let analysis = match term_type {
            TermType::Condition => {
                "Breach of condition entitles innocent party to terminate and claim damages \
                 (Poussard v Spiers [1876]). Election must be made - affirm or terminate."
                    .to_string()
            }
            TermType::Warranty => "Breach of warranty entitles only to damages, not termination \
                 (Bettini v Gye [1876])."
                .to_string(),
            TermType::Innominate => {
                if let Some(ref dep) = deprivation {
                    if dep.substantial_deprivation {
                        format!(
                            "Breach of innominate term. Following Hong Kong Fir [1962], \
                             breach deprives innocent party of substantially the whole \
                             benefit. {}. Right to terminate.",
                            dep.analysis
                        )
                    } else {
                        format!(
                            "Breach of innominate term. Following Hong Kong Fir [1962], \
                             breach does NOT deprive innocent party of substantially the \
                             whole benefit. {}. No right to terminate - damages only.",
                            dep.analysis
                        )
                    }
                } else {
                    "Breach of innominate term. Need to assess deprivation to determine \
                     whether termination available (Hong Kong Fir [1962])."
                        .to_string()
                }
            }
        };

        Self {
            term_type,
            breach_nature,
            deprivation_analysis: deprivation,
            can_terminate,
            can_affirm: true,
            right_lost,
            right_lost_reason,
            analysis,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remoteness_limb1() {
        let analysis = RemotenessAnalysis::analyze(
            "Lost profits from cancelled contract",
            50_000.0,
            true,  // arises naturally
            false, // not special contemplation
            None,
        );
        assert!(!analysis.is_remote);
        assert!(analysis.analysis.contains("Limb 1"));
    }

    #[test]
    fn test_remoteness_limb2_special_knowledge() {
        let analysis = RemotenessAnalysis::analyze(
            "Lost lucrative resale contract",
            100_000.0,
            false, // doesn't arise naturally
            true,  // in contemplation
            Some("Buyer informed seller of resale opportunity"),
        );
        assert!(!analysis.is_remote);
        assert!(analysis.analysis.contains("Limb 2"));
    }

    #[test]
    fn test_remoteness_too_remote() {
        let analysis = RemotenessAnalysis::analyze(
            "Loss of factory production",
            500_000.0,
            false, // doesn't arise naturally
            false, // not in contemplation
            None,
        );
        assert!(analysis.is_remote);
        assert!(analysis.analysis.contains("TOO REMOTE"));
    }

    #[test]
    fn test_specific_performance_land() {
        let analysis = SpecificPerformanceAnalysis::analyze(
            SubjectMatter::Land,
            false, // damages not adequate
            false, // no supervision needed
            false, // not personal service
            vec![],
        );
        assert!(analysis.available);
    }

    #[test]
    fn test_specific_performance_personal_service() {
        let analysis = SpecificPerformanceAnalysis::analyze(
            SubjectMatter::PersonalServices,
            false,
            false,
            true, // personal service
            vec![],
        );
        assert!(!analysis.available);
    }

    #[test]
    fn test_termination_condition() {
        let analysis = TerminationAnalysis::analyze(
            TermType::Condition,
            BreachNature::Actual,
            None,
            false, // no affirmation
        );
        assert!(analysis.can_terminate);
    }

    #[test]
    fn test_termination_warranty() {
        let analysis =
            TerminationAnalysis::analyze(TermType::Warranty, BreachNature::Actual, None, false);
        assert!(!analysis.can_terminate);
    }
}
