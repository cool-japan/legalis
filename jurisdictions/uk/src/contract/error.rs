//! Common Law Contract Error Types

use thiserror::Error;

/// Contract law errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContractError {
    /// No valid offer
    #[error(
        "No valid offer exists.\n\
         \n\
         Common law requirement: Offer must be:\n\
         • Definite and certain\n\
         • Communicated to offeree\n\
         • Still open (not revoked, expired, or rejected)\n\
         \n\
         Issue: {reason}\n\
         \n\
         Note: Invitation to treat is NOT an offer.\n\
         Case law: Pharmaceutical Society v Boots [1953] - goods on shelf are invitation to treat"
    )]
    NoValidOffer { reason: String },

    /// Acceptance does not match offer (mirror image rule)
    #[error(
        "Acceptance does not match offer (mirror image rule violated).\n\
         \n\
         Common law rule: Acceptance must be unqualified and match offer exactly.\n\
         Case law: Hyde v Wrench [1840] - counter-offer destroys original offer.\n\
         \n\
         Offer terms: {offer_terms}\n\
         Acceptance modifications: {modifications}\n\
         \n\
         Effect: This is a COUNTER-OFFER, not acceptance.\n\
         Original offer is destroyed."
    )]
    MirrorImageRuleViolated {
        offer_terms: String,
        modifications: String,
    },

    /// No consideration
    #[error(
        "No valid consideration.\n\
         \n\
         Common law requirement: Consideration must be:\n\
         • Sufficient (but need not be adequate) - Chappell v Nestlé [1960]\n\
         • Not past - Re McArdle [1951]\n\
         • Must move from promisee - Tweddle v Atkinson [1861]\n\
         \n\
         Issue: {reason}\n\
         \n\
         Exception: Deed (formal document under seal) does not require consideration."
    )]
    NoConsideration { reason: String },

    /// Past consideration
    #[error(
        "Past consideration is not valid.\n\
         \n\
         Case law: Re McArdle [1951]\n\
         Rule: Consideration must not be past (already performed before promise made).\n\
         \n\
         Past act: {past_act}\n\
         Later promise: {later_promise}\n\
         \n\
         Exception: Lampleigh v Brathwait [1615] - if:\n\
         • Act done at promisor's request, AND\n\
         • Parties understood payment would be made, AND\n\
         • Promise would be enforceable if made in advance"
    )]
    PastConsideration {
        past_act: String,
        later_promise: String,
    },

    /// No intention to create legal relations
    #[error(
        "No intention to create legal relations.\n\
         \n\
         Context: {context}\n\
         Presumption: {presumption}\n\
         \n\
         Case law:\n\
         • Commercial: Intention presumed - Esso v Commissioners [1976]\n\
         • Domestic: No intention presumed - Balfour v Balfour [1919]\n\
         \n\
         Rebuttal evidence: {rebuttal_evidence}\n\
         \n\
         Conclusion: No legally binding contract."
    )]
    NoIntention {
        context: String,
        presumption: String,
        rebuttal_evidence: String,
    },

    /// Lack of contractual capacity
    #[error(
        "Party lacks contractual capacity.\n\
         \n\
         Party: {party}\n\
         Incapacity type: {incapacity_type}\n\
         \n\
         Legal consequences:\n\
         {consequences}\n\
         \n\
         Protection: Contract may be void or voidable."
    )]
    LackOfCapacity {
        party: String,
        incapacity_type: String,
        consequences: String,
    },

    /// Breach of condition (fundamental term)
    #[error(
        "Breach of condition (essential term).\n\
         \n\
         Case law: Poussard v Spiers [1876]\n\
         Effect: Breach of condition goes to root of contract.\n\
         \n\
         Term breached: {term}\n\
         Breach: {breach_description}\n\
         \n\
         Remedies available:\n\
         • Terminate contract (treat as discharged)\n\
         • Claim damages for losses\n\
         \n\
         Innocent party's choice: Accept breach or affirm contract."
    )]
    BreachOfCondition {
        term: String,
        breach_description: String,
    },

    /// Breach of warranty (minor term)
    #[error(
        "Breach of warranty (minor term).\n\
         \n\
         Case law: Bettini v Gye [1876]\n\
         Effect: Breach of warranty does not go to root of contract.\n\
         \n\
         Term breached: {term}\n\
         Breach: {breach_description}\n\
         \n\
         Remedy: Damages only (cannot terminate contract)."
    )]
    BreachOfWarranty {
        term: String,
        breach_description: String,
    },

    /// Loss too remote (fails Hadley v Baxendale test)
    #[error(
        "Loss is too remote to recover.\n\
         \n\
         Case law: Hadley v Baxendale [1854]\n\
         Test: Damages only recoverable if:\n\
         1. Arising naturally in ordinary course of things, OR\n\
         2. Reasonably in contemplation of both parties at time of contract\n\
         \n\
         Loss claimed: {loss_description}\n\
         Amount: £{loss_amount:.2}\n\
         \n\
         Reason loss is too remote: {reason}\n\
         \n\
         Recoverable damages: £{recoverable:.2}"
    )]
    LossTooRemote {
        loss_description: String,
        loss_amount: f64,
        reason: String,
        recoverable: f64,
    },

    /// Duty to mitigate not satisfied
    #[error(
        "Claimant failed to mitigate loss.\n\
         \n\
         Common law duty: Innocent party must take reasonable steps to mitigate loss.\n\
         Case law: British Westinghouse v Underground Electric Railways [1912]\n\
         \n\
         Failure to mitigate: {failure}\n\
         \n\
         Consequence: Damages reduced by amount that could have been avoided."
    )]
    FailureToMitigate { failure: String },

    /// Contract formation incomplete
    #[error(
        "Contract formation incomplete.\n\
         \n\
         Required elements:\n\
         {missing_elements}\n\
         \n\
         Status: No binding contract formed."
    )]
    FormationIncomplete { missing_elements: String },

    /// Invalid field value
    #[error("Invalid value for {field}: {reason}")]
    InvalidValue { field: String, reason: String },

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingField { field: String },
}

/// Result type for contract operations
pub type Result<T> = std::result::Result<T, ContractError>;

impl ContractError {
    /// Create missing field error
    pub fn missing_field(field: &str) -> Self {
        Self::MissingField {
            field: field.to_string(),
        }
    }

    /// Create invalid value error
    pub fn invalid_value(field: &str, reason: &str) -> Self {
        Self::InvalidValue {
            field: field.to_string(),
            reason: reason.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_image_rule_error() {
        let err = ContractError::MirrorImageRuleViolated {
            offer_terms: "Sell car for £5000".to_string(),
            modifications: "I accept but will pay £4500".to_string(),
        };

        let display = format!("{}", err);
        assert!(display.contains("Hyde v Wrench"));
        assert!(display.contains("COUNTER-OFFER"));
    }

    #[test]
    fn test_past_consideration_error() {
        let err = ContractError::PastConsideration {
            past_act: "Painted house last week".to_string(),
            later_promise: "Promise to pay £500 made today".to_string(),
        };

        let display = format!("{}", err);
        assert!(display.contains("Re McArdle"));
        assert!(display.contains("not valid"));
    }

    #[test]
    fn test_remoteness_error() {
        let err = ContractError::LossTooRemote {
            loss_description: "Loss of profits from related business".to_string(),
            loss_amount: 50000.0,
            reason: "Special circumstances not communicated at time of contract".to_string(),
            recoverable: 5000.0,
        };

        let display = format!("{}", err);
        assert!(display.contains("Hadley v Baxendale"));
        assert!(display.contains("too remote"));
    }
}
