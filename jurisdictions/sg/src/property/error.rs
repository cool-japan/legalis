//! Property Law - Error Types
//!
//! Error types for Singapore property (land) law. Land in Singapore is held under
//! the **Torrens system** of title by registration administered by the Singapore
//! Land Authority (SLA) under the **Land Titles Act 1993 (LTA)**. Conveyancing,
//! leases and security interests are governed by the LTA together with the
//! **Conveyancing and Law of Property Act 1886 (CLPA)** and the **Civil Law Act
//! 1909**, supplemented by common-law and equitable doctrine.
//!
//! Messages are bilingual (English + Chinese/华语), matching the convention of
//! the other Singapore modules. Each error exposes a
//! [`PropertyError::statute_reference`] and a numeric [`PropertyError::severity`]
//! (1-5).

use thiserror::Error;

/// Result type for property law operations.
pub type Result<T> = std::result::Result<T, PropertyError>;

/// Errors arising from analysis of Singapore land law.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PropertyError {
    // ----------------------------------------------------------------------
    // Land Titles Act - indefeasibility and registration
    // ----------------------------------------------------------------------
    /// The registered proprietor's title is not indefeasible because a statutory
    /// exception applies - fraud or forgery to which the proprietor was a party
    /// or privy, or an in personam claim (LTA s. 46(2); *United Overseas Bank v
    /// Bebe* \[2006\] SGCA 30).
    #[error(
        "Title is not indefeasible: {reason} (Land Titles Act s. 46(2))\n\
         所有权不可推翻性丧失: {reason} (土地所有权法令第46(2)条)"
    )]
    TitleDefeasible {
        /// The exception that defeats indefeasibility.
        reason: String,
    },

    /// An instrument has not been registered and so is not effectual to pass the
    /// legal estate or interest (LTA s. 45).
    #[error(
        "Instrument is not registered and does not pass the legal estate: {detail} (Land Titles Act s. 45)\n\
         文书未注册,不能转移法定产权: {detail} (土地所有权法令第45条)"
    )]
    NotRegistered {
        /// Description of the unregistered dealing.
        detail: String,
    },

    // ----------------------------------------------------------------------
    // Caveats
    // ----------------------------------------------------------------------
    /// A caveat was lodged by a person who has no caveatable interest - a
    /// proprietary interest in the land, not a mere personal or contractual right
    /// (LTA s. 115).
    #[error(
        "No caveatable interest to support the caveat: {detail} (Land Titles Act s. 115)\n\
         无可支持警戒书的产权权益: {detail} (土地所有权法令第115条)"
    )]
    NoCaveatableInterest {
        /// Description of the asserted interest.
        detail: String,
    },

    // ----------------------------------------------------------------------
    // Leases
    // ----------------------------------------------------------------------
    /// A lease for a term exceeding 7 years has not been registered and so does
    /// not create a legal leasehold estate; it takes effect, if at all, only in
    /// equity (LTA s. 45; cf the short-lease override in s. 46(1)).
    #[error(
        "Lease for a term exceeding 7 years is not registered - no legal leasehold estate: {years}-year term (Land Titles Act s. 45/s. 46(1))\n\
         超过7年的租约未注册,不产生法定租赁产权: {years}年期 (土地所有权法令第45/46(1)条)"
    )]
    LeaseNotRegistered {
        /// The term of the lease, in years.
        years: u32,
    },

    /// Forfeiture (re-entry) is not available: there is no re-entry clause, or
    /// the statutory notice required before forfeiture for a non-rent breach was
    /// not served (CLPA s. 18).
    #[error(
        "Forfeiture / re-entry not available: {reason} (Conveyancing and Law of Property Act s. 18)\n\
         不能没收租约/重新进入: {reason} (产权转易及财产法令第18条)"
    )]
    ForfeitureNotAvailable {
        /// Why forfeiture is unavailable.
        reason: String,
    },

    // ----------------------------------------------------------------------
    // Interests - easements and mortgages
    // ----------------------------------------------------------------------
    /// A claimed easement does not satisfy the characteristics of an easement
    /// (*Re Ellenborough Park* \[1956\] Ch 131): a dominant and servient
    /// tenement, accommodation of the dominant tenement, diversity of ownership,
    /// and a right capable of forming the subject matter of a grant.
    #[error(
        "Invalid easement - the Re Ellenborough Park characteristics are not satisfied: {reason} (Re Ellenborough Park [1956] Ch 131)\n\
         无效的地役权 - 不符合Re Ellenborough Park要件: {reason}"
    )]
    InvalidEasement {
        /// Which characteristic is not satisfied.
        reason: String,
    },

    /// A registered mortgagee's power of sale is not exercisable - typically
    /// because the mortgage is not registered or the mortgagor is not in default
    /// (LTA s. 68; a Torrens mortgage takes effect as a charge).
    #[error(
        "Mortgagee's power of sale not exercisable: {reason} (Land Titles Act s. 68)\n\
         抵押权人的出售权不可行使: {reason} (土地所有权法令第68条)"
    )]
    PowerOfSaleNotAvailable {
        /// Why the power of sale is unavailable.
        reason: String,
    },

    // ----------------------------------------------------------------------
    // Conveyancing
    // ----------------------------------------------------------------------
    /// A contract for the sale or other disposition of immovable property is
    /// unenforceable because it is not evidenced in writing and signed (Civil Law
    /// Act s. 6(d) - the Singapore equivalent of the Statute of Frauds).
    #[error(
        "Contract for the disposition of land is not evidenced in writing and signed (Civil Law Act s. 6(d))\n\
         土地处置合同未以书面形式订立并签署 (民事法令第6(d)条)"
    )]
    ContractNotInWriting,

    /// An option to purchase was not validly exercised - typically because the
    /// purported exercise was out of time or did not comply with the mode of
    /// acceptance stipulated in the option.
    #[error(
        "Option to purchase not validly exercised: {reason}\n\
         购买选择权未有效行使: {reason}"
    )]
    OptionNotValidlyExercised {
        /// Why exercise was invalid.
        reason: String,
    },

    // ----------------------------------------------------------------------
    // Generic
    // ----------------------------------------------------------------------
    /// An invalid monetary amount was supplied.
    #[error(
        "Invalid monetary amount: {detail}\n\
         金额无效: {detail}"
    )]
    InvalidAmount {
        /// Free-form description of the problem.
        detail: String,
    },

    /// Generic validation error.
    #[error(
        "Property law validation error: {message}\n\
         物业法验证错误: {message}"
    )]
    ValidationError {
        /// Free-form description of the problem.
        message: String,
    },
}

impl PropertyError {
    /// Returns the controlling statute or authority for this error.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            PropertyError::TitleDefeasible { .. } => "Land Titles Act s. 46(2)",
            PropertyError::NotRegistered { .. } => "Land Titles Act s. 45",
            PropertyError::NoCaveatableInterest { .. } => "Land Titles Act s. 115",
            PropertyError::LeaseNotRegistered { .. } => "Land Titles Act s. 45/s. 46(1)",
            PropertyError::ForfeitureNotAvailable { .. } => {
                "Conveyancing and Law of Property Act s. 18"
            }
            PropertyError::InvalidEasement { .. } => "Re Ellenborough Park [1956] Ch 131",
            PropertyError::PowerOfSaleNotAvailable { .. } => "Land Titles Act s. 68",
            PropertyError::ContractNotInWriting => "Civil Law Act s. 6(d)",
            PropertyError::OptionNotValidlyExercised { .. } => "Civil Law Act s. 6(d)",
            PropertyError::InvalidAmount { .. } => "Land Titles Act",
            PropertyError::ValidationError { .. } => "Land Titles Act",
        }
    }

    /// Returns the severity level (1 = informational, 5 = most serious).
    ///
    /// A defeasible title (fraud/forgery) is the most serious outcome, going to
    /// the root of ownership.
    pub fn severity(&self) -> u8 {
        match self {
            PropertyError::TitleDefeasible { .. } => 5,
            PropertyError::NotRegistered { .. } => 3,
            PropertyError::LeaseNotRegistered { .. } => 3,
            PropertyError::InvalidEasement { .. } => 3,
            PropertyError::NoCaveatableInterest { .. } => 3,
            PropertyError::ContractNotInWriting => 3,
            PropertyError::ForfeitureNotAvailable { .. } => 2,
            PropertyError::PowerOfSaleNotAvailable { .. } => 2,
            PropertyError::OptionNotValidlyExercised { .. } => 2,
            PropertyError::InvalidAmount { .. } => 1,
            PropertyError::ValidationError { .. } => 1,
        }
    }

    /// Whether the error goes to the validity of title or an interest (as opposed
    /// to a remedial or procedural limitation).
    pub fn affects_title(&self) -> bool {
        matches!(
            self,
            PropertyError::TitleDefeasible { .. }
                | PropertyError::NotRegistered { .. }
                | PropertyError::LeaseNotRegistered { .. }
                | PropertyError::InvalidEasement { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statute_references() {
        let fraud = PropertyError::TitleDefeasible {
            reason: "proprietor party to forgery".to_string(),
        };
        assert_eq!(fraud.statute_reference(), "Land Titles Act s. 46(2)");

        let writing = PropertyError::ContractNotInWriting;
        assert_eq!(writing.statute_reference(), "Civil Law Act s. 6(d)");

        let easement = PropertyError::InvalidEasement {
            reason: "no dominant tenement".to_string(),
        };
        assert_eq!(
            easement.statute_reference(),
            "Re Ellenborough Park [1956] Ch 131"
        );
    }

    #[test]
    fn test_severity_ordering() {
        let fraud = PropertyError::TitleDefeasible {
            reason: "fraud".to_string(),
        };
        let invalid = PropertyError::InvalidAmount {
            detail: "negative".to_string(),
        };
        assert_eq!(fraud.severity(), 5);
        assert_eq!(invalid.severity(), 1);
        assert!(fraud.severity() > invalid.severity());
    }

    #[test]
    fn test_affects_title() {
        assert!(
            PropertyError::TitleDefeasible {
                reason: "fraud".to_string()
            }
            .affects_title()
        );
        assert!(!PropertyError::ContractNotInWriting.affects_title());
    }

    #[test]
    fn test_display_is_bilingual() {
        let err = PropertyError::LeaseNotRegistered { years: 30 };
        let text = err.to_string();
        assert!(text.contains("Lease for a term exceeding 7 years"));
        assert!(text.contains("Land Titles Act s. 45/s. 46(1)"));
        assert!(text.contains("超过7年的租约"));
    }
}
