//! UK Tort Law - Error Types
//!
//! This module defines error types for UK tort law with statutory references.

use std::fmt;

/// Error type for UK tort law operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TortError {
    // ========================================================================
    // Negligence Errors
    // ========================================================================
    /// No duty of care exists - Caparo v Dickman [1990] 2 AC 605
    NoDutyOfCare {
        /// Reason duty does not exist
        reason: String,
        /// Missing element (foreseeability/proximity/fair just reasonable)
        missing_element: String,
    },

    /// Harm was not reasonably foreseeable - Donoghue v Stevenson [1932] AC 562
    HarmNotForeseeable {
        /// Type of harm
        harm_type: String,
        /// Explanation
        explanation: String,
    },

    /// No proximity between parties
    NoProximity {
        /// Relationship type
        relationship: String,
        /// Why proximity lacks
        explanation: String,
    },

    /// Not fair, just and reasonable to impose duty
    NotFairJustReasonable {
        /// Policy considerations
        policy_reasons: Vec<String>,
    },

    /// Standard of care not breached
    NoBreachOfDuty {
        /// Standard expected
        expected_standard: String,
        /// Actual conduct
        actual_conduct: String,
    },

    /// Causation not established
    CausationNotEstablished {
        /// Whether factual or legal causation failed
        causation_type: String,
        /// Explanation
        explanation: String,
    },

    /// But-for test not satisfied
    ButForNotSatisfied {
        /// Alternative cause
        alternative_cause: String,
    },

    /// Damage too remote - The Wagon Mound [1961] AC 388
    DamageTooRemote {
        /// Type of harm
        harm_type: String,
        /// Why unforeseeable
        explanation: String,
    },

    /// Chain of causation broken by novus actus interveniens
    ChainOfCausationBroken {
        /// Type of intervening act
        intervening_act: String,
        /// Why it breaks the chain
        explanation: String,
    },

    /// No actionable damage suffered
    NoActionableDamage {
        /// Type of claimed damage
        claimed_damage: String,
        /// Why not actionable
        reason: String,
    },

    // ========================================================================
    // Psychiatric Injury Errors
    // ========================================================================
    /// Psychiatric injury claim fails - Alcock v Chief Constable [1992] 1 AC 310
    PsychiatricInjuryClaimFails {
        /// Missing Alcock requirement
        missing_requirement: String,
    },

    /// No recognized psychiatric illness
    NoRecognizedIllness {
        /// What was claimed
        claimed_condition: String,
    },

    /// Secondary victim requirements not met
    SecondaryVictimRequirementsNotMet {
        /// Which control mechanism failed
        failed_control: String,
    },

    // ========================================================================
    // Pure Economic Loss Errors
    // ========================================================================
    /// Pure economic loss not recoverable - Murphy v Brentwood [1991] 1 AC 398
    PureEconomicLossNotRecoverable {
        /// Type of economic loss
        loss_type: String,
        /// Why not recoverable
        reason: String,
    },

    /// Hedley Byrne requirements not met - Hedley Byrne v Heller [1964] AC 465
    HedleyByrneNotMet {
        /// Missing requirement
        missing_requirement: String,
    },

    /// No assumption of responsibility
    NoAssumptionOfResponsibility {
        /// Context
        context: String,
    },

    // ========================================================================
    // Occupiers' Liability Errors
    // ========================================================================
    /// Not an occupier under OLA 1957/1984
    NotAnOccupier {
        /// Why not an occupier
        reason: String,
    },

    /// Claimant not a visitor under OLA 1957
    NotAVisitor {
        /// Status of claimant
        status: String,
    },

    /// Warning was adequate under s.2(4)(a) OLA 1957
    AdequateWarning {
        /// Nature of warning
        warning: String,
    },

    /// Obvious risk under OLA 1984
    ObviousRisk {
        /// Nature of the obvious risk
        risk: String,
    },

    /// Risk willingly accepted under s.1(6) OLA 1984
    RiskWillinglyAccepted {
        /// Nature of acceptance
        acceptance: String,
    },

    // ========================================================================
    // Nuisance Errors
    // ========================================================================
    /// No interest in land for private nuisance - Hunter v Canary Wharf [1997] AC 655
    NoInterestInLand {
        /// Claimant's status
        status: String,
    },

    /// Interference is reasonable - Sedleigh-Denfield v O'Callaghan [1940] AC 880
    ReasonableUseOfLand {
        /// Why use is reasonable
        reason: String,
    },

    /// Defendant is not occupier or creator of nuisance
    NotResponsibleForNuisance {
        /// Why not responsible
        reason: String,
    },

    /// Sensitivity of claimant/use abnormal - Robinson v Kilvert (1889)
    AbnormalSensitivity {
        /// Nature of sensitivity
        sensitivity: String,
    },

    // ========================================================================
    // Rylands v Fletcher Errors
    // ========================================================================
    /// Not a non-natural use - Transco v Stockport [2004] 2 AC 1
    NaturalUseOfLand {
        /// Why use is natural
        reason: String,
    },

    /// Nothing likely to cause mischief escaped
    NoEscape {
        /// What was alleged to escape
        thing: String,
    },

    /// Act of God defence
    ActOfGod {
        /// Natural event
        event: String,
    },

    /// Act of stranger defence
    ActOfStranger {
        /// Third party action
        action: String,
    },

    // ========================================================================
    // Defamation Errors
    // ========================================================================
    /// Statement not defamatory under s.1 Defamation Act 2013
    NotDefamatory {
        /// Why not defamatory
        reason: String,
    },

    /// No serious harm to reputation under s.1 DA 2013
    NoSeriousHarm {
        /// Actual harm level
        harm_level: String,
    },

    /// Defendant not the publisher
    NotPublisher {
        /// Role of defendant
        role: String,
    },

    /// Defence of truth under s.2 DA 2013
    TruthDefence {
        /// Substantially true allegation
        allegation: String,
    },

    /// Defence of honest opinion under s.3 DA 2013
    HonestOpinionDefence {
        /// Opinion expressed
        opinion: String,
    },

    /// Defence of public interest under s.4 DA 2013
    PublicInterestDefence {
        /// Matter of public interest
        matter: String,
    },

    /// Privilege defence (absolute or qualified)
    PrivilegeDefence {
        /// Type of privilege
        privilege_type: String,
        /// Context
        context: String,
    },

    /// Website operator defence under s.5 DA 2013
    WebsiteOperatorDefence {
        /// Reason for defence
        reason: String,
    },

    // ========================================================================
    // Economic Torts Errors
    // ========================================================================
    /// Inducing breach of contract fails - OBG v Allan [2008] 1 AC 1
    InducingBreachFails {
        /// Missing element
        missing_element: String,
    },

    /// No intention to cause harm - OBG v Allan
    NoIntentionToHarm {
        /// Explanation
        explanation: String,
    },

    /// Unlawful means tort fails - OBG v Allan
    UnlawfulMeansFails {
        /// Missing requirement
        missing_requirement: String,
    },

    /// Conspiracy claim fails
    ConspiracyFails {
        /// Whether lawful or unlawful means
        conspiracy_type: String,
        /// Missing element
        missing_element: String,
    },

    /// Justification defence applies
    JustificationDefence {
        /// Basis for justification
        basis: String,
    },

    // ========================================================================
    // Defence Errors
    // ========================================================================
    /// Volenti non fit injuria applies
    VolentiApplies {
        /// Nature of consent
        consent: String,
    },

    /// Ex turpi causa applies - Patel v Mirza [2016] UKSC 42
    ExTurpiApplies {
        /// Illegality
        illegality: String,
    },

    /// Claim is statute-barred - Limitation Act 1980
    LimitationExpired {
        /// Limitation period
        period: String,
        /// When expired
        expired_date: String,
    },

    /// Effective exclusion/limitation of liability
    EffectiveExclusion {
        /// Nature of exclusion
        exclusion: String,
        /// Why valid
        validity_reason: String,
    },

    // ========================================================================
    // Procedural Errors
    // ========================================================================
    /// Invalid claim configuration
    InvalidClaimConfiguration {
        /// What is invalid
        field: String,
        /// Why invalid
        reason: String,
    },

    /// Missing required evidence
    MissingEvidence {
        /// What evidence is missing
        evidence_type: String,
    },

    /// Parties not properly identified
    InvalidParties {
        /// Issue with parties
        issue: String,
    },
}

impl fmt::Display for TortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Negligence errors
            TortError::NoDutyOfCare {
                reason,
                missing_element,
            } => {
                write!(
                    f,
                    "No duty of care: {} (Caparo test failed on: {})",
                    reason, missing_element
                )
            }
            TortError::HarmNotForeseeable {
                harm_type,
                explanation,
            } => {
                write!(
                    f,
                    "Harm not foreseeable: {} - {} (Donoghue v Stevenson)",
                    harm_type, explanation
                )
            }
            TortError::NoProximity {
                relationship,
                explanation,
            } => {
                write!(
                    f,
                    "No proximity in {} relationship: {}",
                    relationship, explanation
                )
            }
            TortError::NotFairJustReasonable { policy_reasons } => {
                write!(
                    f,
                    "Not fair, just and reasonable: {}",
                    policy_reasons.join("; ")
                )
            }
            TortError::NoBreachOfDuty {
                expected_standard,
                actual_conduct,
            } => {
                write!(
                    f,
                    "No breach: expected {} but conduct was {}",
                    expected_standard, actual_conduct
                )
            }
            TortError::CausationNotEstablished {
                causation_type,
                explanation,
            } => {
                write!(
                    f,
                    "{} causation not established: {}",
                    causation_type, explanation
                )
            }
            TortError::ButForNotSatisfied { alternative_cause } => {
                write!(
                    f,
                    "But-for test not satisfied: damage would have occurred due to {}",
                    alternative_cause
                )
            }
            TortError::DamageTooRemote {
                harm_type,
                explanation,
            } => {
                write!(
                    f,
                    "Damage too remote: {} - {} (Wagon Mound)",
                    harm_type, explanation
                )
            }
            TortError::ChainOfCausationBroken {
                intervening_act,
                explanation,
            } => {
                write!(f, "Chain broken by {}: {}", intervening_act, explanation)
            }
            TortError::NoActionableDamage {
                claimed_damage,
                reason,
            } => {
                write!(f, "No actionable damage: {} - {}", claimed_damage, reason)
            }

            // Psychiatric injury errors
            TortError::PsychiatricInjuryClaimFails {
                missing_requirement,
            } => {
                write!(
                    f,
                    "Psychiatric injury claim fails: {} (Alcock)",
                    missing_requirement
                )
            }
            TortError::NoRecognizedIllness { claimed_condition } => {
                write!(
                    f,
                    "No recognized psychiatric illness: {}",
                    claimed_condition
                )
            }
            TortError::SecondaryVictimRequirementsNotMet { failed_control } => {
                write!(
                    f,
                    "Secondary victim requirements not met: {}",
                    failed_control
                )
            }

            // Economic loss errors
            TortError::PureEconomicLossNotRecoverable { loss_type, reason } => {
                write!(
                    f,
                    "Pure economic loss not recoverable: {} - {} (Murphy v Brentwood)",
                    loss_type, reason
                )
            }
            TortError::HedleyByrneNotMet {
                missing_requirement,
            } => {
                write!(
                    f,
                    "Hedley Byrne requirements not met: {}",
                    missing_requirement
                )
            }
            TortError::NoAssumptionOfResponsibility { context } => {
                write!(f, "No assumption of responsibility in {}", context)
            }

            // Occupiers' liability errors
            TortError::NotAnOccupier { reason } => {
                write!(f, "Not an occupier: {}", reason)
            }
            TortError::NotAVisitor { status } => {
                write!(f, "Claimant not a visitor under OLA 1957: {}", status)
            }
            TortError::AdequateWarning { warning } => {
                write!(
                    f,
                    "Adequate warning given under s.2(4)(a) OLA 1957: {}",
                    warning
                )
            }
            TortError::ObviousRisk { risk } => {
                write!(f, "Obvious risk under OLA 1984: {}", risk)
            }
            TortError::RiskWillinglyAccepted { acceptance } => {
                write!(
                    f,
                    "Risk willingly accepted under s.1(6) OLA 1984: {}",
                    acceptance
                )
            }

            // Nuisance errors
            TortError::NoInterestInLand { status } => {
                write!(
                    f,
                    "No interest in land for private nuisance: {} (Hunter v Canary Wharf)",
                    status
                )
            }
            TortError::ReasonableUseOfLand { reason } => {
                write!(f, "Reasonable use of land: {}", reason)
            }
            TortError::NotResponsibleForNuisance { reason } => {
                write!(f, "Not responsible for nuisance: {}", reason)
            }
            TortError::AbnormalSensitivity { sensitivity } => {
                write!(
                    f,
                    "Abnormal sensitivity: {} (Robinson v Kilvert)",
                    sensitivity
                )
            }

            // Rylands v Fletcher errors
            TortError::NaturalUseOfLand { reason } => {
                write!(f, "Natural use of land: {} (Transco v Stockport)", reason)
            }
            TortError::NoEscape { thing } => {
                write!(f, "No escape: {} did not escape from land", thing)
            }
            TortError::ActOfGod { event } => {
                write!(f, "Act of God defence: {}", event)
            }
            TortError::ActOfStranger { action } => {
                write!(f, "Act of stranger defence: {}", action)
            }

            // Defamation errors
            TortError::NotDefamatory { reason } => {
                write!(f, "Statement not defamatory under s.1 DA 2013: {}", reason)
            }
            TortError::NoSeriousHarm { harm_level } => {
                write!(
                    f,
                    "No serious harm to reputation under s.1 DA 2013: {}",
                    harm_level
                )
            }
            TortError::NotPublisher { role } => {
                write!(f, "Defendant not publisher: {}", role)
            }
            TortError::TruthDefence { allegation } => {
                write!(f, "Defence of truth under s.2 DA 2013: {}", allegation)
            }
            TortError::HonestOpinionDefence { opinion } => {
                write!(
                    f,
                    "Defence of honest opinion under s.3 DA 2013: {}",
                    opinion
                )
            }
            TortError::PublicInterestDefence { matter } => {
                write!(
                    f,
                    "Defence of public interest under s.4 DA 2013: {}",
                    matter
                )
            }
            TortError::PrivilegeDefence {
                privilege_type,
                context,
            } => {
                write!(f, "{} privilege in {}", privilege_type, context)
            }
            TortError::WebsiteOperatorDefence { reason } => {
                write!(f, "Website operator defence under s.5 DA 2013: {}", reason)
            }

            // Economic torts errors
            TortError::InducingBreachFails { missing_element } => {
                write!(
                    f,
                    "Inducing breach of contract fails: {} (OBG v Allan)",
                    missing_element
                )
            }
            TortError::NoIntentionToHarm { explanation } => {
                write!(f, "No intention to cause harm: {}", explanation)
            }
            TortError::UnlawfulMeansFails {
                missing_requirement,
            } => {
                write!(
                    f,
                    "Unlawful means tort fails: {} (OBG v Allan)",
                    missing_requirement
                )
            }
            TortError::ConspiracyFails {
                conspiracy_type,
                missing_element,
            } => {
                write!(
                    f,
                    "{} conspiracy fails: {}",
                    conspiracy_type, missing_element
                )
            }
            TortError::JustificationDefence { basis } => {
                write!(f, "Justification defence: {}", basis)
            }

            // Defence errors
            TortError::VolentiApplies { consent } => {
                write!(f, "Volenti non fit injuria: {}", consent)
            }
            TortError::ExTurpiApplies { illegality } => {
                write!(f, "Ex turpi causa: {} (Patel v Mirza)", illegality)
            }
            TortError::LimitationExpired {
                period,
                expired_date,
            } => {
                write!(
                    f,
                    "Claim statute-barred: {} period expired on {} (Limitation Act 1980)",
                    period, expired_date
                )
            }
            TortError::EffectiveExclusion {
                exclusion,
                validity_reason,
            } => {
                write!(
                    f,
                    "Effective exclusion: {} (valid because {})",
                    exclusion, validity_reason
                )
            }

            // Procedural errors
            TortError::InvalidClaimConfiguration { field, reason } => {
                write!(f, "Invalid claim configuration: {} - {}", field, reason)
            }
            TortError::MissingEvidence { evidence_type } => {
                write!(f, "Missing evidence: {}", evidence_type)
            }
            TortError::InvalidParties { issue } => {
                write!(f, "Invalid parties: {}", issue)
            }
        }
    }
}

impl std::error::Error for TortError {}

impl TortError {
    /// Get the primary legal authority for this error
    pub fn legal_authority(&self) -> &'static str {
        match self {
            // Negligence
            TortError::NoDutyOfCare { .. } => "Caparo Industries plc v Dickman [1990] 2 AC 605",
            TortError::HarmNotForeseeable { .. } => "Donoghue v Stevenson [1932] AC 562",
            TortError::NoProximity { .. } => "Caparo Industries plc v Dickman [1990] 2 AC 605",
            TortError::NotFairJustReasonable { .. } => {
                "Caparo Industries plc v Dickman [1990] 2 AC 605"
            }
            TortError::NoBreachOfDuty { .. } => "Blyth v Birmingham Waterworks (1856) 11 Ex 781",
            TortError::CausationNotEstablished { .. } => "Cork v Kirby Maclean [1952] 2 All ER 402",
            TortError::ButForNotSatisfied { .. } => {
                "Barnett v Chelsea & Kensington Hospital [1969] 1 QB 428"
            }
            TortError::DamageTooRemote { .. } => "The Wagon Mound (No. 1) [1961] AC 388",
            TortError::ChainOfCausationBroken { .. } => "Knightley v Johns [1982] 1 WLR 349",
            TortError::NoActionableDamage { .. } => "General principle",

            // Psychiatric injury
            TortError::PsychiatricInjuryClaimFails { .. } => {
                "Alcock v Chief Constable of South Yorkshire [1992] 1 AC 310"
            }
            TortError::NoRecognizedIllness { .. } => "Hinz v Berry [1970] 2 QB 40",
            TortError::SecondaryVictimRequirementsNotMet { .. } => {
                "Alcock v Chief Constable of South Yorkshire [1992] 1 AC 310"
            }

            // Economic loss
            TortError::PureEconomicLossNotRecoverable { .. } => {
                "Murphy v Brentwood DC [1991] 1 AC 398"
            }
            TortError::HedleyByrneNotMet { .. } => {
                "Hedley Byrne & Co Ltd v Heller & Partners Ltd [1964] AC 465"
            }
            TortError::NoAssumptionOfResponsibility { .. } => {
                "Henderson v Merrett Syndicates [1995] 2 AC 145"
            }

            // Occupiers' liability
            TortError::NotAnOccupier { .. } => "Wheat v E Lacon & Co Ltd [1966] AC 552",
            TortError::NotAVisitor { .. } => "Occupiers' Liability Act 1957, s.1(2)",
            TortError::AdequateWarning { .. } => "Occupiers' Liability Act 1957, s.2(4)(a)",
            TortError::ObviousRisk { .. } => "Occupiers' Liability Act 1984, s.1(5)",
            TortError::RiskWillinglyAccepted { .. } => "Occupiers' Liability Act 1984, s.1(6)",

            // Nuisance
            TortError::NoInterestInLand { .. } => "Hunter v Canary Wharf Ltd [1997] AC 655",
            TortError::ReasonableUseOfLand { .. } => {
                "Sedleigh-Denfield v O'Callaghan [1940] AC 880"
            }
            TortError::NotResponsibleForNuisance { .. } => {
                "Sedleigh-Denfield v O'Callaghan [1940] AC 880"
            }
            TortError::AbnormalSensitivity { .. } => "Robinson v Kilvert (1889) 41 Ch D 88",

            // Rylands v Fletcher
            TortError::NaturalUseOfLand { .. } => "Transco plc v Stockport MBC [2004] 2 AC 1",
            TortError::NoEscape { .. } => "Rylands v Fletcher (1868) LR 3 HL 330",
            TortError::ActOfGod { .. } => "Nichols v Marsland (1876) 2 Ex D 1",
            TortError::ActOfStranger { .. } => "Perry v Kendricks Transport Ltd [1956] 1 WLR 85",

            // Defamation
            TortError::NotDefamatory { .. } => "Defamation Act 2013, s.1",
            TortError::NoSeriousHarm { .. } => {
                "Defamation Act 2013, s.1; Lachaux v Independent Print [2019] UKSC 27"
            }
            TortError::NotPublisher { .. } => "Defamation Act 2013, s.10",
            TortError::TruthDefence { .. } => "Defamation Act 2013, s.2",
            TortError::HonestOpinionDefence { .. } => "Defamation Act 2013, s.3",
            TortError::PublicInterestDefence { .. } => "Defamation Act 2013, s.4",
            TortError::PrivilegeDefence { .. } => "Defamation Act 2013, ss.6-7",
            TortError::WebsiteOperatorDefence { .. } => "Defamation Act 2013, s.5",

            // Economic torts
            TortError::InducingBreachFails { .. } => "OBG Ltd v Allan [2008] 1 AC 1",
            TortError::NoIntentionToHarm { .. } => "OBG Ltd v Allan [2008] 1 AC 1",
            TortError::UnlawfulMeansFails { .. } => "OBG Ltd v Allan [2008] 1 AC 1",
            TortError::ConspiracyFails { .. } => {
                "Revenue and Customs v Total Network [2008] 1 AC 1174"
            }
            TortError::JustificationDefence { .. } => {
                "Edwin Hill v First National [1989] 1 WLR 225"
            }

            // Defences
            TortError::VolentiApplies { .. } => "Smith v Baker [1891] AC 325",
            TortError::ExTurpiApplies { .. } => "Patel v Mirza [2016] UKSC 42",
            TortError::LimitationExpired { .. } => "Limitation Act 1980",
            TortError::EffectiveExclusion { .. } => "Unfair Contract Terms Act 1977",

            // Procedural
            TortError::InvalidClaimConfiguration { .. } => "Civil Procedure Rules",
            TortError::MissingEvidence { .. } => "General evidential principles",
            TortError::InvalidParties { .. } => "Civil Procedure Rules",
        }
    }

    /// Get the section of the relevant statute (if applicable)
    pub fn statutory_section(&self) -> Option<&'static str> {
        match self {
            TortError::NotAVisitor { .. } => Some("OLA 1957, s.1(2)"),
            TortError::AdequateWarning { .. } => Some("OLA 1957, s.2(4)(a)"),
            TortError::ObviousRisk { .. } => Some("OLA 1984, s.1(5)"),
            TortError::RiskWillinglyAccepted { .. } => Some("OLA 1984, s.1(6)"),
            TortError::NotDefamatory { .. } => Some("DA 2013, s.1"),
            TortError::NoSeriousHarm { .. } => Some("DA 2013, s.1"),
            TortError::TruthDefence { .. } => Some("DA 2013, s.2"),
            TortError::HonestOpinionDefence { .. } => Some("DA 2013, s.3"),
            TortError::PublicInterestDefence { .. } => Some("DA 2013, s.4"),
            TortError::WebsiteOperatorDefence { .. } => Some("DA 2013, s.5"),
            TortError::LimitationExpired { .. } => Some("LA 1980"),
            _ => None,
        }
    }

    /// Check if this error represents a complete defence
    pub fn is_complete_defence(&self) -> bool {
        matches!(
            self,
            TortError::NoDutyOfCare { .. }
                | TortError::NoBreachOfDuty { .. }
                | TortError::CausationNotEstablished { .. }
                | TortError::ButForNotSatisfied { .. }
                | TortError::ChainOfCausationBroken { .. }
                | TortError::NoActionableDamage { .. }
                | TortError::VolentiApplies { .. }
                | TortError::ExTurpiApplies { .. }
                | TortError::LimitationExpired { .. }
                | TortError::TruthDefence { .. }
                | TortError::HonestOpinionDefence { .. }
                | TortError::PublicInterestDefence { .. }
                | TortError::PrivilegeDefence { .. }
                | TortError::ActOfGod { .. }
                | TortError::ActOfStranger { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_duty_of_care_error() {
        let error = TortError::NoDutyOfCare {
            reason: "Novel duty not established".to_string(),
            missing_element: "fair just and reasonable".to_string(),
        };

        assert!(error.to_string().contains("Caparo"));
        assert_eq!(
            error.legal_authority(),
            "Caparo Industries plc v Dickman [1990] 2 AC 605"
        );
        assert!(error.is_complete_defence());
    }

    #[test]
    fn test_limitation_error() {
        let error = TortError::LimitationExpired {
            period: "3 years".to_string(),
            expired_date: "2023-01-01".to_string(),
        };

        assert!(error.to_string().contains("Limitation Act 1980"));
        assert_eq!(error.statutory_section(), Some("LA 1980"));
        assert!(error.is_complete_defence());
    }

    #[test]
    fn test_defamation_error() {
        let error = TortError::TruthDefence {
            allegation: "The defendant was convicted of fraud".to_string(),
        };

        assert!(error.to_string().contains("s.2 DA 2013"));
        assert_eq!(error.statutory_section(), Some("DA 2013, s.2"));
        assert!(error.is_complete_defence());
    }

    #[test]
    fn test_occupiers_liability_error() {
        let error = TortError::AdequateWarning {
            warning: "Clear signage warning of wet floor".to_string(),
        };

        assert!(error.to_string().contains("s.2(4)(a)"));
        assert_eq!(error.statutory_section(), Some("OLA 1957, s.2(4)(a)"));
    }

    #[test]
    fn test_rylands_v_fletcher_error() {
        let error = TortError::NaturalUseOfLand {
            reason: "Domestic water supply is ordinary use".to_string(),
        };

        assert!(error.to_string().contains("Transco v Stockport"));
        assert!(!error.is_complete_defence()); // Defence, not failure of claim
    }
}
