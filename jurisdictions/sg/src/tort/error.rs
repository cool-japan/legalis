//! Tort Law - Error Types
//!
//! Error types for analysis of Singapore tort law. The law of tort in Singapore
//! is predominantly **common law** (received English law as developed by the
//! Singapore courts), supplemented by statute where indicated — chiefly the
//! Defamation Act 1957. Each error attributes the controlling authority: the
//! leading case, or the statutory provision where one applies.
//!
//! Messages are bilingual (English + Chinese/华语), matching the convention of
//! the other Singapore modules.

use thiserror::Error;

/// Result type for tort law operations.
pub type Result<T> = std::result::Result<T, TortError>;

/// Errors arising from analysis of a Singapore-law tort claim.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TortError {
    /// No duty of care is owed on the facts.
    ///
    /// Duty in negligence is determined by the single two-stage test of
    /// factual then legal proximity, subject to policy, established in
    /// *Spandeck Engineering v Defence Science & Technology Agency* \[2007\]
    /// SGCA 37, preceded by a threshold of factual foreseeability.
    #[error(
        "No duty of care owed: {reason} (Spandeck Engineering v DSTA [2007] SGCA 37)\n\
         不负有注意义务: {reason} (Spandeck Engineering v DSTA [2007] SGCA 37)"
    )]
    NoDutyOfCare { reason: String },

    /// The defendant did not breach the standard of care.
    ///
    /// The standard is that of the reasonable person (*Blyth v Birmingham
    /// Waterworks* (1856) 11 Ex 781); for professionals, the *Bolam*/*Bolitho*
    /// standard, refined for medical advice in *Hii Chii Kok v Ooi Peng Jin*
    /// \[2017\] SGCA 38.
    #[error(
        "No breach of the standard of care: {reason}\n\
         未违反注意标准: {reason}"
    )]
    NoBreach { reason: String },

    /// Factual causation is not established.
    ///
    /// The claimant must show the breach was a cause in fact of the loss on the
    /// "but for" test (*Barnett v Chelsea & Kensington Hospital* \[1969\] 1 QB
    /// 428).
    #[error(
        "Factual causation not established (the loss would have occurred anyway): {reason} (Barnett v Chelsea & Kensington Hospital [1969] 1 QB 428)\n\
         事实因果关系不成立: {reason}"
    )]
    NoFactualCausation { reason: String },

    /// The damage is too remote (not reasonably foreseeable).
    ///
    /// Recoverable damage is that of a kind that was reasonably foreseeable
    /// (*The Wagon Mound (No 1)* \[1961\] AC 388).
    #[error(
        "Damage too remote — not a reasonably foreseeable kind of harm: {reason} (The Wagon Mound (No 1) [1961] AC 388)\n\
         损害过于遥远,非合理可预见: {reason}"
    )]
    RemoteDamage { reason: String },

    /// An intervening act breaks the chain of causation (novus actus
    /// interveniens).
    #[error(
        "Chain of causation broken by an intervening act: {detail}\n\
         因果链被介入行为打断: {detail}"
    )]
    NovusActusInterveniens { detail: String },

    /// Negligence is established (the full cause of action is made out).
    #[error(
        "Negligence established: {detail} (Spandeck Engineering v DSTA [2007] SGCA 37)\n\
         过失成立: {detail}"
    )]
    NegligenceEstablished { detail: String },

    /// A defamatory statement actionable in libel (permanent form).
    ///
    /// Libel is actionable per se (without proof of special damage).
    #[error(
        "Defamatory statement actionable as libel: {statement} (Defamation Act 1957)\n\
         构成永久形式诽谤(文字诽谤): {statement} (诽谤法令1957)"
    )]
    Libel { statement: String },

    /// A defamatory statement actionable in slander (transient form).
    ///
    /// Slander generally requires proof of special damage, save for the
    /// exceptions in s. 5 (imputation of a criminal offence) and s. 6
    /// (disparagement in office/profession/trade) of the Defamation Act 1957.
    #[error(
        "Defamatory statement actionable as slander: {statement} ({basis})\n\
         构成短暂形式诽谤(口头诽谤): {statement} ({basis})"
    )]
    Slander { statement: String, basis: String },

    /// A private nuisance: an unlawful interference with the use or enjoyment of
    /// land.
    ///
    /// *Sturges v Bridgman* (1879) 11 Ch D 852; the interference must be
    /// substantial and unreasonable.
    #[error(
        "Actionable private nuisance — substantial and unreasonable interference with land: {detail}\n\
         构成私人妨害 — 对土地使用的实质且不合理干扰: {detail}"
    )]
    PrivateNuisance { detail: String },

    /// A public nuisance affecting a class of the public.
    ///
    /// Actionable in tort by a private claimant only on proof of special damage
    /// over and above that suffered by the public generally.
    #[error(
        "Public nuisance affecting a class of the public: {detail}\n\
         构成公共妨害,影响公众群体: {detail}"
    )]
    PublicNuisance { detail: String },

    /// Breach of an occupier's duty to a visitor.
    ///
    /// An occupier owes a duty to take reasonable care for the safety of lawful
    /// visitors (the common-law duty; cf the position of trespassers — *British
    /// Railways Board v Herrington* \[1972\] AC 877).
    #[error(
        "Breach of occupier's duty to a {visitor_kind}: {detail}\n\
         违反占有人对{visitor_kind}的义务: {detail}"
    )]
    OccupiersLiability {
        visitor_kind: String,
        detail: String,
    },

    /// A defence wholly defeats the claim.
    #[error(
        "Claim defeated by the defence of {defence}: {detail}\n\
         因{defence}抗辩,索赔不成立: {detail}"
    )]
    DefenceSucceeds { defence: String, detail: String },

    /// A monetary value supplied was invalid (e.g. negative).
    #[error(
        "Invalid monetary amount: {detail}\n\
         金额无效: {detail}"
    )]
    InvalidAmount { detail: String },

    /// Generic validation failure with a free-form message.
    #[error(
        "Tort validation error: {message}\n\
         侵权验证错误: {message}"
    )]
    ValidationError { message: String },
}

impl TortError {
    /// Returns the controlling authority (statute or leading case) where one can
    /// be attributed.
    pub fn authority(&self) -> Option<&str> {
        match self {
            TortError::NoDutyOfCare { .. } | TortError::NegligenceEstablished { .. } => {
                Some("Spandeck Engineering v DSTA [2007] SGCA 37")
            }
            TortError::NoBreach { .. } => Some("Blyth v Birmingham Waterworks (1856) 11 Ex 781"),
            TortError::NoFactualCausation { .. } => {
                Some("Barnett v Chelsea & Kensington Hospital [1969] 1 QB 428")
            }
            TortError::RemoteDamage { .. } => Some("The Wagon Mound (No 1) [1961] AC 388"),
            TortError::Libel { .. } => Some("Defamation Act 1957"),
            TortError::Slander { basis, .. } => Some(basis),
            TortError::PrivateNuisance { .. } => Some("Sturges v Bridgman (1879) 11 Ch D 852"),
            _ => None,
        }
    }

    /// Returns whether the error records that a cause of action is established
    /// (as opposed to a failed element or a successful defence).
    pub fn is_liability_finding(&self) -> bool {
        matches!(
            self,
            TortError::NegligenceEstablished { .. }
                | TortError::Libel { .. }
                | TortError::Slander { .. }
                | TortError::PrivateNuisance { .. }
                | TortError::PublicNuisance { .. }
                | TortError::OccupiersLiability { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duty_error_attributes_spandeck() {
        let err = TortError::NoDutyOfCare {
            reason: "no proximity".to_string(),
        };
        assert_eq!(
            err.authority(),
            Some("Spandeck Engineering v DSTA [2007] SGCA 37")
        );
        assert!(!err.is_liability_finding());
    }

    #[test]
    fn libel_is_a_liability_finding_under_the_act() {
        let err = TortError::Libel {
            statement: "X is a fraud".to_string(),
        };
        assert_eq!(err.authority(), Some("Defamation Act 1957"));
        assert!(err.is_liability_finding());
    }

    #[test]
    fn slander_carries_its_own_statutory_basis() {
        let err = TortError::Slander {
            statement: "X stole from the till".to_string(),
            basis: "Defamation Act 1957 s. 5".to_string(),
        };
        assert_eq!(err.authority(), Some("Defamation Act 1957 s. 5"));
    }

    #[test]
    fn display_is_bilingual() {
        let err = TortError::RemoteDamage {
            reason: "unforeseeable explosion".to_string(),
        };
        let text = err.to_string();
        assert!(text.contains("The Wagon Mound (No 1) [1961] AC 388"));
        assert!(text.contains("损害过于遥远"));
    }
}
