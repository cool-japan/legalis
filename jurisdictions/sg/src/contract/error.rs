//! Contract Law - Error Types
//!
//! Error types for Singapore contract law analysis. Singapore contract law is
//! predominantly **common law** (received English law, modified by local
//! statute and decisions of the Singapore courts). Where a statutory provision
//! applies it is cited (e.g. the Misrepresentation Act 1967, the Frustrated
//! Contracts Act 1959, the Civil Law Act 1909, the Unfair Contract Terms Act
//! 1977). Otherwise the controlling authority is the leading case, which is
//! given in the error text.
//!
//! Messages are bilingual (English + Chinese/华语), matching the convention of
//! the other Singapore modules.

use thiserror::Error;

/// Result type for contract law operations.
pub type Result<T> = std::result::Result<T, ContractError>;

/// Errors arising from analysis of a Singapore-law contract.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContractError {
    /// No agreement: offer and acceptance do not correspond.
    ///
    /// A binding agreement requires an offer met by an unqualified acceptance
    /// of its exact terms — the "mirror image" rule (*Gay Choon Ing v Loh Sze
    /// Ti Terence Peter* \[2009\] SGCA 3).
    #[error(
        "No agreement reached: {reason} (offer/acceptance must correspond — Gay Choon Ing v Loh Sze Ti [2009] SGCA 3)\n\
         未达成协议: {reason} (要约与承诺须一致)"
    )]
    NoAgreement { reason: String },

    /// A purported acceptance varied the terms and so operates as a counter-offer.
    ///
    /// A counter-offer destroys the original offer, which can no longer be
    /// accepted (*Hyde v Wrench* (1840) 3 Beav 334).
    #[error(
        "Purported acceptance is a counter-offer and rejects the original offer: {detail} (Hyde v Wrench (1840) 3 Beav 334)\n\
         所谓的承诺构成反要约,原要约失效: {detail}"
    )]
    CounterOffer { detail: String },

    /// Acceptance was communicated after the offer had lapsed or been revoked.
    #[error(
        "Offer was no longer open when acceptance was communicated: {reason}\n\
         承诺送达时要约已失效: {reason}"
    )]
    OfferNotOpen { reason: String },

    /// Consideration is absent or insufficient in law.
    ///
    /// A simple contract requires consideration that moves from the promisee
    /// and is of some economic value (*Currie v Misa* (1875) LR 10 Ex 153),
    /// though it need not be adequate.
    #[error(
        "No valuable consideration provided by the promisee: {reason} (Currie v Misa (1875) LR 10 Ex 153)\n\
         承诺人未提供有效对价: {reason}"
    )]
    NoConsideration { reason: String },

    /// Performance of an existing duty offered as consideration without any
    /// practical benefit (no *Williams v Roffey* benefit).
    #[error(
        "Existing duty is not good consideration absent a practical benefit: {reason} (Williams v Roffey Bros [1991] 1 QB 1; Gay Choon Ing [2009] SGCA 3)\n\
         既有义务在无实际利益时不构成对价: {reason}"
    )]
    ExistingDutyConsideration { reason: String },

    /// The parties lacked an intention to create legal relations.
    ///
    /// Rebuttable presumptions apply: commercial agreements are presumed
    /// intended to bind; social/domestic agreements are presumed not
    /// (*Balfour v Balfour* \[1919\] 2 KB 571).
    #[error(
        "No intention to create legal relations: {reason} (Balfour v Balfour [1919] 2 KB 571)\n\
         无订立法律关系的意图: {reason}"
    )]
    NoIntentionToCreateLegalRelations { reason: String },

    /// Certainty/completeness failure: an essential term is missing or too vague.
    #[error(
        "Agreement is incomplete or uncertain: {reason}\n\
         协议不完整或不确定: {reason}"
    )]
    UncertainTerms { reason: String },

    /// An actionable misrepresentation induced the contract.
    ///
    /// Categories: fraudulent (*Derry v Peek* (1889) 14 App Cas 337),
    /// negligent (Misrepresentation Act 1967 s. 2(1)), and innocent.
    #[error(
        "Actionable {category} misrepresentation induced the contract: {statement} ({authority})\n\
         {category}虚假陈述诱使订约: {statement} ({authority})"
    )]
    Misrepresentation {
        category: String,
        statement: String,
        authority: String,
    },

    /// An operative mistake renders the contract void or voidable.
    ///
    /// Common mistake (*Great Peace Shipping v Tsavliris* \[2002\] EWCA Civ
    /// 1407, adopted by the SGCA), mutual mistake, or unilateral mistake
    /// (*Chwee Kin Keong v Digilandmall.com* \[2005\] SGCA 2).
    #[error(
        "Operative {kind} mistake: {detail} ({authority})\n\
         可撤销的{kind}错误: {detail} ({authority})"
    )]
    Mistake {
        kind: String,
        detail: String,
        authority: String,
    },

    /// The contract was procured by duress.
    ///
    /// Includes economic duress: illegitimate pressure that is a significant
    /// cause of entry (*Universe Tankships v ITWF (The Universe Sentinel)*
    /// \[1983\] 1 AC 366; *E C Investment Holding v Ridout Residence* \[2011\]
    /// SGHC 231).
    #[error(
        "Contract procured by {kind} duress: {detail}\n\
         合同因{kind}胁迫而订立: {detail}"
    )]
    Duress { kind: String, detail: String },

    /// Undue influence vitiates consent.
    ///
    /// Actual (Class 1) or presumed (Class 2) undue influence (*Royal Bank of
    /// Scotland v Etridge (No 2)* \[2001\] UKHL 44; *BOM v BOK* \[2018\] SGCA
    /// 83).
    #[error(
        "Contract affected by {kind} undue influence: {detail} (RBS v Etridge (No 2) [2001] UKHL 44)\n\
         合同受{kind}不当影响: {detail}"
    )]
    UndueInfluence { kind: String, detail: String },

    /// A condition (or a term breached so as to go to the root) was breached,
    /// giving rise to a right to terminate.
    ///
    /// *Hongkong Fir Shipping v Kawasaki Kisen Kaisha* \[1962\] 2 QB 26;
    /// *RDC Concrete v Sato Kogyo* \[2007\] SGCA 1 (the four situations).
    #[error(
        "Repudiatory breach entitling the innocent party to terminate: {detail} (RDC Concrete v Sato Kogyo [2007] SGCA 1)\n\
         可解除合同的根本违约: {detail}"
    )]
    RepudiatoryBreach { detail: String },

    /// A warranty (or innominate term breached trivially) was breached; damages
    /// only, no right to terminate.
    #[error(
        "Breach of warranty — damages only, no right to terminate: {detail}\n\
         违反保证条款 — 仅可索赔,不可解约: {detail}"
    )]
    WarrantyBreach { detail: String },

    /// Claimed loss is too remote to be recoverable.
    ///
    /// Recoverable loss is that arising naturally, or in the parties'
    /// reasonable contemplation at formation (*Hadley v Baxendale* (1854) 9
    /// Exch 341; *Robertson Quay Investment v Steen Consultants* \[2008\] SGCA
    /// 8).
    #[error(
        "Loss is too remote under Hadley v Baxendale: {detail} (Hadley v Baxendale (1854) 9 Exch 341)\n\
         损失依Hadley v Baxendale过于遥远: {detail}"
    )]
    RemoteDamage { detail: String },

    /// The claimant failed to mitigate recoverable loss.
    ///
    /// *British Westinghouse v Underground Electric Railways* \[1912\] AC 673.
    #[error(
        "Claimant failed to take reasonable steps to mitigate loss: {detail} (British Westinghouse [1912] AC 673)\n\
         索赔方未采取合理措施减轻损失: {detail}"
    )]
    FailureToMitigate { detail: String },

    /// Specific performance is unavailable on the facts.
    ///
    /// Equitable, discretionary, and refused where damages are adequate or the
    /// subject matter is not unique.
    #[error(
        "Specific performance unavailable: {reason}\n\
         不能强制实际履行: {reason}"
    )]
    SpecificPerformanceUnavailable { reason: String },

    /// Frustration could not be established.
    #[error(
        "Frustration not established: {reason} (Davis Contractors v Fareham UDC [1956] AC 696)\n\
         合同受挫不成立: {reason}"
    )]
    FrustrationNotEstablished { reason: String },

    /// A monetary value supplied was negative or otherwise invalid.
    #[error(
        "Invalid monetary amount: {detail}\n\
         金额无效: {detail}"
    )]
    InvalidAmount { detail: String },

    /// Generic validation failure with a free-form message.
    #[error(
        "Contract validation error: {message}\n\
         合同验证错误: {message}"
    )]
    ValidationError { message: String },
}

impl ContractError {
    /// Returns the controlling authority (statute or leading case) for the error,
    /// where one can be attributed.
    pub fn authority(&self) -> Option<&str> {
        match self {
            ContractError::NoAgreement { .. } => Some("Gay Choon Ing v Loh Sze Ti [2009] SGCA 3"),
            ContractError::CounterOffer { .. } => Some("Hyde v Wrench (1840) 3 Beav 334"),
            ContractError::NoConsideration { .. } => Some("Currie v Misa (1875) LR 10 Ex 153"),
            ContractError::ExistingDutyConsideration { .. } => {
                Some("Williams v Roffey Bros [1991] 1 QB 1")
            }
            ContractError::NoIntentionToCreateLegalRelations { .. } => {
                Some("Balfour v Balfour [1919] 2 KB 571")
            }
            ContractError::Misrepresentation { authority, .. } => Some(authority),
            ContractError::Mistake { authority, .. } => Some(authority),
            ContractError::Duress { .. } => Some("The Universe Sentinel [1983] 1 AC 366"),
            ContractError::UndueInfluence { .. } => Some("RBS v Etridge (No 2) [2001] UKHL 44"),
            ContractError::RepudiatoryBreach { .. } => {
                Some("RDC Concrete v Sato Kogyo [2007] SGCA 1")
            }
            ContractError::RemoteDamage { .. } => Some("Hadley v Baxendale (1854) 9 Exch 341"),
            ContractError::FailureToMitigate { .. } => Some("British Westinghouse [1912] AC 673"),
            ContractError::FrustrationNotEstablished { .. } => {
                Some("Davis Contractors v Fareham UDC [1956] AC 696")
            }
            _ => None,
        }
    }

    /// Returns whether the error indicates the contract is, or may be, void or
    /// voidable (as opposed to a mere breach or remedial limitation).
    pub fn affects_validity(&self) -> bool {
        matches!(
            self,
            ContractError::NoAgreement { .. }
                | ContractError::CounterOffer { .. }
                | ContractError::OfferNotOpen { .. }
                | ContractError::NoConsideration { .. }
                | ContractError::ExistingDutyConsideration { .. }
                | ContractError::NoIntentionToCreateLegalRelations { .. }
                | ContractError::UncertainTerms { .. }
                | ContractError::Misrepresentation { .. }
                | ContractError::Mistake { .. }
                | ContractError::Duress { .. }
                | ContractError::UndueInfluence { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_consideration_attributes_currie_v_misa() {
        let err = ContractError::NoConsideration {
            reason: "gratuitous promise".to_string(),
        };
        assert_eq!(err.authority(), Some("Currie v Misa (1875) LR 10 Ex 153"));
        assert!(err.affects_validity());
    }

    #[test]
    fn remote_damage_attributes_hadley_v_baxendale() {
        let err = ContractError::RemoteDamage {
            detail: "lost sub-contract profit".to_string(),
        };
        assert_eq!(
            err.authority(),
            Some("Hadley v Baxendale (1854) 9 Exch 341")
        );
        // Remoteness limits remedy; it does not affect formation validity.
        assert!(!err.affects_validity());
    }

    #[test]
    fn misrepresentation_carries_its_own_authority() {
        let err = ContractError::Misrepresentation {
            category: "negligent".to_string(),
            statement: "the engine was reconditioned".to_string(),
            authority: "Misrepresentation Act 1967 s. 2(1)".to_string(),
        };
        assert_eq!(err.authority(), Some("Misrepresentation Act 1967 s. 2(1)"));
        assert!(err.affects_validity());
    }

    #[test]
    fn display_is_bilingual() {
        let err = ContractError::RepudiatoryBreach {
            detail: "failure to deliver".to_string(),
        };
        let text = err.to_string();
        assert!(text.contains("RDC Concrete v Sato Kogyo [2007] SGCA 1"));
        assert!(text.contains("根本违约"));
    }
}
