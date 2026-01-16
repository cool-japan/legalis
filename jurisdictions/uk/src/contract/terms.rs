//! Contract Terms Classification and Exclusion Clauses
//!
//! This module implements the law relating to contract terms under English law.
//!
//! ## Term Classification
//!
//! Terms are classified by their importance:
//!
//! ### Conditions (Poussard v Spiers [1876])
//! - Essential terms going to root of contract
//! - Breach entitles innocent party to terminate and claim damages
//!
//! ### Warranties (Bettini v Gye [1876])
//! - Minor terms collateral to main purpose
//! - Breach entitles only to damages, not termination
//!
//! ### Innominate Terms (Hong Kong Fir Shipping v Kawasaki [1962])
//! - Terms of uncertain importance
//! - Effect of breach depends on seriousness of breach
//! - If breach deprives innocent party of substantially whole benefit → terminate
//!
//! ## Incorporation of Terms
//!
//! ### By Signature (L'Estrange v Graucob [1934])
//! - Signed document binds signer to all terms
//! - Exception: Non est factum (very narrow)
//!
//! ### By Notice (Parker v South Eastern Railway [1877])
//! - Reasonable steps to bring terms to attention
//! - More onerous terms require more notice (Interfoto v Stiletto [1989])
//!
//! ### By Course of Dealing (Spurling v Bradshaw [1956])
//! - Consistent previous dealings incorporating terms
//! - Must be regular and consistent
//!
//! ## Exclusion Clauses (UCTA 1977, CRA 2015)
//!
//! ### UCTA 1977 (B2B)
//! - s.2(1): Cannot exclude liability for death/personal injury from negligence
//! - s.2(2): Other negligence liability subject to reasonableness test
//! - s.3: Standard terms subject to reasonableness
//!
//! ### CRA 2015 (B2C)
//! - s.62: Unfair terms not binding on consumer
//! - s.65: Cannot exclude liability for death/personal injury from negligence
//!
//! ## Implied Terms
//!
//! ### Common Law (The Moorcock [1889])
//! - Business efficacy test
//! - Officious bystander test (Shirlaw v Southern Foundries)
//!
//! ### Statute
//! - Sale of Goods Act 1979 (ss.12-15)
//! - Supply of Goods and Services Act 1982

use serde::{Deserialize, Serialize};

// ============================================================================
// Term Classification
// ============================================================================

/// Classification of contract terms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TermType {
    /// Condition - essential term (breach → terminate + damages)
    Condition,
    /// Warranty - minor term (breach → damages only)
    Warranty,
    /// Innominate - effect depends on breach seriousness (Hong Kong Fir)
    Innominate,
}

impl TermType {
    /// Get description with case law reference
    pub fn description(&self) -> &'static str {
        match self {
            Self::Condition => {
                "Essential term going to root of contract (Poussard v Spiers [1876]). \
                 Breach entitles innocent party to terminate and claim damages."
            }
            Self::Warranty => {
                "Minor term collateral to main purpose (Bettini v Gye [1876]). \
                 Breach entitles only to damages, not termination."
            }
            Self::Innominate => {
                "Term of uncertain importance (Hong Kong Fir [1962]). \
                 Effect of breach depends on whether it deprives innocent party of \
                 substantially the whole benefit of the contract."
            }
        }
    }

    /// What remedies are available for breach?
    pub fn breach_remedies(&self) -> &'static str {
        match self {
            Self::Condition => "Termination and damages",
            Self::Warranty => "Damages only",
            Self::Innominate => "Depends on seriousness of breach",
        }
    }
}

/// A contract term
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractTerm {
    /// Content of the term
    pub content: String,
    /// Classification of the term
    pub term_type: TermType,
    /// Source of the term
    pub source: TermSource,
    /// Is this a core/main term?
    pub is_core_term: bool,
    /// Analysis of term classification
    pub classification_analysis: Option<String>,
}

/// Source of a contract term
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TermSource {
    /// Expressly agreed by parties
    Express,
    /// Implied by statute (SGA 1979, SGSA 1982)
    ImpliedByStatute,
    /// Implied by common law (business efficacy)
    ImpliedByCommonLaw,
    /// Implied by custom/trade usage
    ImpliedByCustom,
    /// Implied by previous course of dealing
    ImpliedByCourseOfDealing,
}

impl TermSource {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Express => "Expressly agreed between the parties",
            Self::ImpliedByStatute => "Implied by statute (e.g., SGA 1979 ss.12-15, SGSA 1982)",
            Self::ImpliedByCommonLaw => {
                "Implied by common law to give business efficacy (The Moorcock [1889])"
            }
            Self::ImpliedByCustom => "Implied by custom or trade usage",
            Self::ImpliedByCourseOfDealing => {
                "Implied by previous course of dealing between parties"
            }
        }
    }
}

// ============================================================================
// Implied Terms
// ============================================================================

/// Test for implying terms at common law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImplicationTest {
    /// Business efficacy - term necessary to make contract work (The Moorcock)
    BusinessEfficacy,
    /// Officious bystander - so obvious it goes without saying (Shirlaw v Southern Foundries)
    OfficiousBystander,
    /// Both tests must be satisfied (BP Refinery v Shire of Hastings)
    BothTests,
}

/// Implied term analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpliedTermAnalysis {
    /// The proposed implied term
    pub proposed_term: String,
    /// Test applied
    pub test: ImplicationTest,
    /// Is term necessary for business efficacy?
    pub business_efficacy_satisfied: bool,
    /// Would officious bystander say "of course"?
    pub officious_bystander_satisfied: bool,
    /// Does term contradict express terms?
    pub contradicts_express: bool,
    /// Is term capable of clear expression?
    pub capable_of_clear_expression: bool,
    /// Result - should term be implied?
    pub should_imply: bool,
    /// Analysis
    pub analysis: String,
}

impl ImpliedTermAnalysis {
    /// Analyze whether a term should be implied
    pub fn analyze(
        proposed_term: &str,
        test: ImplicationTest,
        business_efficacy: bool,
        officious_bystander: bool,
        contradicts_express: bool,
    ) -> Self {
        // Cannot imply term that contradicts express terms
        let capable_of_clear_expression = true; // Assume yes

        let should_imply = !contradicts_express
            && match test {
                ImplicationTest::BusinessEfficacy => business_efficacy,
                ImplicationTest::OfficiousBystander => officious_bystander,
                ImplicationTest::BothTests => business_efficacy && officious_bystander,
            };

        let analysis = if contradicts_express {
            "Cannot imply term that contradicts express terms.".to_string()
        } else if should_imply {
            format!(
                "Term '{}' should be implied. {}",
                proposed_term,
                match test {
                    ImplicationTest::BusinessEfficacy =>
                        "Business efficacy test satisfied (The Moorcock).",
                    ImplicationTest::OfficiousBystander =>
                        "Officious bystander test satisfied (Shirlaw v Southern Foundries).",
                    ImplicationTest::BothTests =>
                        "Both business efficacy and officious bystander tests satisfied \
                         (BP Refinery v Shire of Hastings).",
                }
            )
        } else {
            format!(
                "Term '{}' should NOT be implied. Tests not satisfied.",
                proposed_term
            )
        };

        Self {
            proposed_term: proposed_term.to_string(),
            test,
            business_efficacy_satisfied: business_efficacy,
            officious_bystander_satisfied: officious_bystander,
            contradicts_express,
            capable_of_clear_expression,
            should_imply,
            analysis,
        }
    }
}

// ============================================================================
// Incorporation of Terms
// ============================================================================

/// Method of incorporating terms into contract
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncorporationMethod {
    /// Incorporated by signature (L'Estrange v Graucob)
    BySignature,
    /// Incorporated by reasonable notice (Parker v South Eastern Railway)
    ByNotice {
        /// Was notice given before/at time of contract?
        timing_adequate: bool,
        /// Was the notice reasonably sufficient?
        notice_sufficient: bool,
    },
    /// Incorporated by course of dealing (Spurling v Bradshaw)
    ByCourseOfDealing {
        /// Number of previous transactions
        previous_transactions: u32,
        /// Were previous dealings consistent?
        consistent: bool,
    },
}

/// Result of incorporation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncorporationAnalysis {
    /// The term in question
    pub term: String,
    /// Method of incorporation attempted
    pub method: IncorporationMethod,
    /// Is the term onerous or unusual?
    pub is_onerous: bool,
    /// Was term successfully incorporated?
    pub incorporated: bool,
    /// Analysis
    pub analysis: String,
}

impl IncorporationAnalysis {
    /// Analyze incorporation by signature
    pub fn by_signature(term: &str, signed: bool) -> Self {
        let incorporated = signed;
        let analysis = if signed {
            format!(
                "Term '{}' incorporated by signature. Following L'Estrange v Graucob [1934], \
                 signing a document binds the signer to all its terms regardless of whether \
                 they were read.",
                term
            )
        } else {
            format!("Term '{}' not incorporated - document not signed.", term)
        };

        Self {
            term: term.to_string(),
            method: IncorporationMethod::BySignature,
            is_onerous: false,
            incorporated,
            analysis,
        }
    }

    /// Analyze incorporation by notice
    pub fn by_notice(term: &str, timing_ok: bool, notice_ok: bool, is_onerous: bool) -> Self {
        // Onerous terms require more notice (Interfoto v Stiletto [1989])
        let notice_threshold = if is_onerous {
            timing_ok && notice_ok
        } else {
            timing_ok
        };

        let incorporated = notice_threshold;

        let analysis = if incorporated {
            if is_onerous {
                format!(
                    "Onerous term '{}' incorporated. Following Interfoto v Stiletto [1989], \
                     the more onerous a term, the greater the notice required. Sufficient \
                     notice was given here.",
                    term
                )
            } else {
                format!(
                    "Term '{}' incorporated by reasonable notice (Parker v South Eastern \
                     Railway [1877]).",
                    term
                )
            }
        } else if !timing_ok {
            format!(
                "Term '{}' not incorporated - notice given after contract formation \
                 (Olley v Marlborough Court [1949]).",
                term
            )
        } else {
            format!(
                "Onerous term '{}' not incorporated - insufficient notice given \
                 (Interfoto v Stiletto [1989]).",
                term
            )
        };

        Self {
            term: term.to_string(),
            method: IncorporationMethod::ByNotice {
                timing_adequate: timing_ok,
                notice_sufficient: notice_ok,
            },
            is_onerous,
            incorporated,
            analysis,
        }
    }

    /// Analyze incorporation by course of dealing
    pub fn by_course_of_dealing(term: &str, previous_transactions: u32, consistent: bool) -> Self {
        // Need regular and consistent previous dealings
        let incorporated = previous_transactions >= 3 && consistent;

        let analysis = if incorporated {
            format!(
                "Term '{}' incorporated by course of dealing. Following Spurling v Bradshaw \
                 [1956], {} previous consistent transactions establish incorporation.",
                term, previous_transactions
            )
        } else if previous_transactions < 3 {
            format!(
                "Term '{}' not incorporated - only {} previous transactions (insufficient \
                 course of dealing). Following Hollier v Rambler Motors [1972], 3-4 \
                 transactions over 5 years was insufficient.",
                term, previous_transactions
            )
        } else {
            format!(
                "Term '{}' not incorporated - previous dealings were not consistent.",
                term
            )
        };

        Self {
            term: term.to_string(),
            method: IncorporationMethod::ByCourseOfDealing {
                previous_transactions,
                consistent,
            },
            is_onerous: false,
            incorporated,
            analysis,
        }
    }
}

// ============================================================================
// Exclusion Clauses
// ============================================================================

/// Type of contract for exclusion clause analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractContext {
    /// Business to Business
    B2B,
    /// Business to Consumer
    B2C,
    /// Consumer to Consumer
    C2C,
}

/// Type of liability sought to be excluded
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiabilityType {
    /// Death or personal injury from negligence
    DeathPersonalInjury,
    /// Other loss from negligence
    OtherNegligence,
    /// Breach of express terms
    BreachOfExpressTerms,
    /// Breach of implied terms (SGA 1979)
    BreachOfImpliedTerms,
    /// Consequential loss
    ConsequentialLoss,
}

/// An exclusion or limitation clause
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExclusionClause {
    /// Text of the clause
    pub text: String,
    /// Type of liability excluded
    pub liability_type: LiabilityType,
    /// Contract context
    pub context: ContractContext,
    /// Is clause clear and unambiguous?
    pub clear_and_unambiguous: bool,
    /// Does clause cover the loss in question? (Canada Steamship rules)
    pub covers_loss: bool,
}

/// Result of exclusion clause validity analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExclusionClauseValidity {
    /// The exclusion clause
    pub clause: ExclusionClause,
    /// Is clause validly incorporated?
    pub incorporated: bool,
    /// Does clause cover the breach? (construction)
    pub covers_breach: bool,
    /// Statutory validity under UCTA 1977 / CRA 2015
    pub statutory_validity: StatutoryValidity,
    /// Overall - is clause effective?
    pub effective: bool,
    /// Analysis
    pub analysis: String,
}

/// Statutory validity of exclusion clause
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatutoryValidity {
    /// Applicable statute
    pub applicable_statute: ApplicableStatute,
    /// Is clause void outright?
    pub void_outright: bool,
    /// Is clause subject to reasonableness test?
    pub subject_to_reasonableness: bool,
    /// If subject to test, does it pass?
    pub passes_reasonableness: Option<bool>,
    /// Is clause an unfair term (CRA 2015)?
    pub unfair_term: Option<bool>,
}

/// Applicable statute for exclusion clause
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApplicableStatute {
    /// UCTA 1977 (B2B)
    Ucta1977,
    /// CRA 2015 (B2C)
    Cra2015,
    /// Neither (C2C or other)
    Neither,
}

/// Validate an exclusion clause
pub fn validate_exclusion_clause(
    clause: &ExclusionClause,
    incorporated: bool,
    reasonableness_factors: &[ReasonablenessFactor],
) -> ExclusionClauseValidity {
    let applicable_statute = match clause.context {
        ContractContext::B2B => ApplicableStatute::Ucta1977,
        ContractContext::B2C => ApplicableStatute::Cra2015,
        ContractContext::C2C => ApplicableStatute::Neither,
    };

    let (void_outright, subject_to_reasonableness) = match applicable_statute {
        ApplicableStatute::Ucta1977 => {
            match clause.liability_type {
                // UCTA s.2(1) - death/personal injury from negligence CANNOT be excluded
                LiabilityType::DeathPersonalInjury => (true, false),
                // UCTA s.2(2) - other negligence subject to reasonableness
                LiabilityType::OtherNegligence => (false, true),
                // UCTA s.3 - standard terms subject to reasonableness
                _ => (false, true),
            }
        }
        ApplicableStatute::Cra2015 => {
            match clause.liability_type {
                // CRA s.65 - death/personal injury CANNOT be excluded
                LiabilityType::DeathPersonalInjury => (true, false),
                // CRA s.62 - unfairness test
                _ => (false, false), // Subject to unfairness, not reasonableness
            }
        }
        ApplicableStatute::Neither => (false, false),
    };

    let passes_reasonableness = if subject_to_reasonableness && !void_outright {
        Some(assess_reasonableness(reasonableness_factors))
    } else {
        None
    };

    let unfair_term = if applicable_statute == ApplicableStatute::Cra2015
        && !void_outright
        && clause.liability_type != LiabilityType::DeathPersonalInjury
    {
        Some(assess_unfairness(clause, reasonableness_factors))
    } else {
        None
    };

    let statutory_validity = StatutoryValidity {
        applicable_statute,
        void_outright,
        subject_to_reasonableness,
        passes_reasonableness,
        unfair_term,
    };

    let effective = incorporated
        && clause.covers_loss
        && !void_outright
        && passes_reasonableness.unwrap_or(true)
        && !unfair_term.unwrap_or(false);

    let analysis = generate_exclusion_analysis(clause, &statutory_validity, effective);

    ExclusionClauseValidity {
        clause: clause.clone(),
        incorporated,
        covers_breach: clause.covers_loss,
        statutory_validity,
        effective,
        analysis,
    }
}

fn generate_exclusion_analysis(
    clause: &ExclusionClause,
    validity: &StatutoryValidity,
    effective: bool,
) -> String {
    let mut analysis = String::new();

    analysis.push_str(&format!(
        "Analyzing exclusion clause: '{}'\n",
        if clause.text.len() > 50 {
            format!("{}...", &clause.text[..50])
        } else {
            clause.text.clone()
        }
    ));

    match validity.applicable_statute {
        ApplicableStatute::Ucta1977 => {
            analysis.push_str("Applicable statute: UCTA 1977 (B2B). ");
        }
        ApplicableStatute::Cra2015 => {
            analysis.push_str("Applicable statute: CRA 2015 (B2C). ");
        }
        ApplicableStatute::Neither => {
            analysis.push_str("No statutory control applies (C2C). ");
        }
    }

    if validity.void_outright {
        analysis.push_str(
            "Clause is VOID. Cannot exclude liability for death/personal injury \
                          from negligence (UCTA s.2(1) / CRA s.65). ",
        );
    } else if let Some(passes) = validity.passes_reasonableness {
        if passes {
            analysis.push_str("Clause passes reasonableness test (UCTA s.11). ");
        } else {
            analysis.push_str("Clause FAILS reasonableness test (UCTA s.11). ");
        }
    }

    if let Some(unfair) = validity.unfair_term {
        if unfair {
            analysis.push_str("Clause is UNFAIR under CRA 2015 s.62 and not binding on consumer. ");
        } else {
            analysis.push_str("Clause is not unfair under CRA 2015 s.62. ");
        }
    }

    analysis.push_str(&format!(
        "Overall: Clause is {}.",
        if effective {
            "EFFECTIVE"
        } else {
            "INEFFECTIVE"
        }
    ));

    analysis
}

// ============================================================================
// Reasonableness Test (UCTA 1977 s.11)
// ============================================================================

/// Factor for assessing reasonableness (UCTA Sch 2)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReasonablenessFactor {
    /// Relative bargaining power of parties
    BargainingPower {
        /// Was there equal bargaining power?
        equal: bool,
    },
    /// Did customer receive inducement (e.g., lower price)?
    Inducement {
        /// Was there an inducement?
        present: bool,
    },
    /// Did customer know or should have known of term?
    CustomerKnowledge {
        /// Did customer have actual/constructive knowledge?
        knew: bool,
    },
    /// Was it practicable to comply with condition?
    PracticalToComply {
        /// Was compliance practical?
        practical: bool,
    },
    /// Were goods specially made/adapted?
    SpeciallyMadeGoods {
        /// Were goods bespoke?
        bespoke: bool,
    },
    /// Insurance availability
    InsuranceAvailable {
        /// Could risk have been insured?
        available: bool,
    },
}

/// Assess reasonableness of exclusion clause
fn assess_reasonableness(factors: &[ReasonablenessFactor]) -> bool {
    // Simplified assessment - count factors in favor
    let positive_factors = factors
        .iter()
        .filter(|f| {
            matches!(
                f,
                ReasonablenessFactor::BargainingPower { equal: true }
                    | ReasonablenessFactor::Inducement { present: true }
                    | ReasonablenessFactor::CustomerKnowledge { knew: true }
                    | ReasonablenessFactor::PracticalToComply { practical: true }
                    | ReasonablenessFactor::InsuranceAvailable { available: false }
            )
        })
        .count();

    // More positive factors than negative suggests reasonable
    positive_factors > factors.len() / 2
}

/// Assess unfairness under CRA 2015
fn assess_unfairness(clause: &ExclusionClause, factors: &[ReasonablenessFactor]) -> bool {
    // CRA 2015 s.62 - unfair if causes significant imbalance to consumer's detriment
    // contrary to good faith

    // Consider similar factors to reasonableness
    let consumer_detriment = factors
        .iter()
        .any(|f| matches!(f, ReasonablenessFactor::BargainingPower { equal: false }));

    let lacks_transparency = !clause.clear_and_unambiguous;

    // Unfair if causes detriment and lacks transparency
    consumer_detriment || lacks_transparency
}

// ============================================================================
// Interpretation Rules
// ============================================================================

/// Rules for interpreting exclusion clauses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterpretationRule {
    /// Contra proferentem - interpreted against drafter
    ContraProferentem,
    /// Canada Steamship rules for negligence
    CanadaSteamship,
    /// Natural and ordinary meaning
    NaturalMeaning,
}

/// Apply interpretation rules to exclusion clause
pub fn interpret_exclusion_clause(
    clause_text: &str,
    claimed_coverage: LiabilityType,
    rule: InterpretationRule,
) -> InterpretationResult {
    let text_lower = clause_text.to_lowercase();

    let (covers_claimed_liability, analysis) = match rule {
        InterpretationRule::ContraProferentem => {
            // Ambiguity resolved against drafter
            let is_ambiguous = text_lower.contains("all liability")
                || text_lower.contains("any loss")
                || text_lower.contains("howsoever caused");

            if is_ambiguous {
                (
                    false,
                    "Clause is ambiguous. Following contra proferentem rule, interpreted \
                     against the party seeking to rely on it."
                        .to_string(),
                )
            } else {
                (true, "Clause is clear and unambiguous.".to_string())
            }
        }
        InterpretationRule::CanadaSteamship => {
            // Canada Steamship rules for negligence exclusion
            let expressly_mentions_negligence =
                text_lower.contains("negligence") || text_lower.contains("negligent");

            if claimed_coverage == LiabilityType::OtherNegligence
                || claimed_coverage == LiabilityType::DeathPersonalInjury
            {
                if expressly_mentions_negligence {
                    (
                        true,
                        "Following Canada Steamship Lines v The King [1952], clause \
                         expressly refers to negligence and covers it."
                            .to_string(),
                    )
                } else {
                    (
                        false,
                        "Following Canada Steamship Lines v The King [1952], clause does \
                         not expressly refer to negligence. Court will consider whether \
                         words are wide enough to cover negligence, but there is a head \
                         of liability other than negligence."
                            .to_string(),
                    )
                }
            } else {
                (
                    true,
                    "Canada Steamship rules not applicable - not negligence.".to_string(),
                )
            }
        }
        InterpretationRule::NaturalMeaning => {
            // Plain and natural meaning (ICS v West Bromwich)
            (
                true,
                "Clause interpreted according to natural and ordinary meaning \
                 (Investors Compensation Scheme v West Bromwich [1998])."
                    .to_string(),
            )
        }
    };

    InterpretationResult {
        clause_text: clause_text.to_string(),
        claimed_coverage,
        rule_applied: rule,
        covers_claimed_liability,
        analysis,
    }
}

/// Result of interpretation analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterpretationResult {
    /// The clause text
    pub clause_text: String,
    /// The claimed coverage
    pub claimed_coverage: LiabilityType,
    /// Rule applied
    pub rule_applied: InterpretationRule,
    /// Does clause cover claimed liability?
    pub covers_claimed_liability: bool,
    /// Analysis
    pub analysis: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_classification() {
        assert_eq!(
            TermType::Condition.breach_remedies(),
            "Termination and damages"
        );
        assert_eq!(TermType::Warranty.breach_remedies(), "Damages only");
    }

    #[test]
    fn test_incorporation_by_signature() {
        let analysis = IncorporationAnalysis::by_signature("Liability limited to £100", true);
        assert!(analysis.incorporated);
        assert!(analysis.analysis.contains("L'Estrange"));
    }

    #[test]
    fn test_incorporation_by_notice_onerous_term() {
        let analysis = IncorporationAnalysis::by_notice(
            "Customer forfeits £10,000 on cancellation",
            true,  // timing ok
            false, // notice not sufficient
            true,  // onerous
        );
        assert!(!analysis.incorporated);
        assert!(analysis.analysis.contains("Interfoto"));
    }

    #[test]
    fn test_incorporation_by_course_of_dealing() {
        let analysis = IncorporationAnalysis::by_course_of_dealing(
            "Liability excluded",
            5,    // 5 previous transactions
            true, // consistent
        );
        assert!(analysis.incorporated);
        assert!(analysis.analysis.contains("Spurling"));
    }

    #[test]
    fn test_exclusion_death_personal_injury_void() {
        let clause = ExclusionClause {
            text: "We exclude all liability for injury".to_string(),
            liability_type: LiabilityType::DeathPersonalInjury,
            context: ContractContext::B2B,
            clear_and_unambiguous: true,
            covers_loss: true,
        };

        let result = validate_exclusion_clause(&clause, true, &[]);
        assert!(result.statutory_validity.void_outright);
        assert!(!result.effective);
    }

    #[test]
    fn test_implied_term_business_efficacy() {
        let analysis = ImpliedTermAnalysis::analyze(
            "Seller will deliver goods to buyer's premises",
            ImplicationTest::BusinessEfficacy,
            true,  // necessary for business efficacy
            true,  // officious bystander would agree
            false, // no contradiction
        );
        assert!(analysis.should_imply);
    }

    #[test]
    fn test_canada_steamship_express_negligence() {
        let result = interpret_exclusion_clause(
            "We exclude liability for negligence of our employees",
            LiabilityType::OtherNegligence,
            InterpretationRule::CanadaSteamship,
        );
        assert!(result.covers_claimed_liability);
        assert!(result.analysis.contains("expressly refers to negligence"));
    }
}
