//! Securities and Futures Act 2001 - Error Types
//!
//! Error types for the **Securities and Futures Act 2001 (SFA)**, administered
//! and enforced by the **Monetary Authority of Singapore (MAS)**. The SFA is the
//! principal statute governing Singapore's capital markets: it regulates capital
//! markets products, the offering of securities (Part 13), market conduct
//! (Part 12) and the licensing of intermediaries (Part 4).
//!
//! All error messages are bilingual, in Singapore's primary languages:
//! - English (official business and administration language)
//! - Chinese/华语 (Chinese community, ~74% of the resident population)
//!
//! Each error exposes a [`SecuritiesError::statute_reference`] and a numeric
//! [`SecuritiesError::severity`] (1-5) to support downstream triage and
//! reporting, mirroring the convention used by the other Singapore modules.

use thiserror::Error;

/// Result type for Securities and Futures Act operations.
pub type Result<T> = std::result::Result<T, SecuritiesError>;

/// Errors arising under the Securities and Futures Act 2001 and MAS enforcement.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SecuritiesError {
    // ----------------------------------------------------------------------
    // Part 13 - Offers of investments / prospectus
    // ----------------------------------------------------------------------
    /// An offer of securities was made without a prospectus registered by MAS,
    /// and no exemption applies (SFA s. 240).
    #[error(
        "Offer of {product} requires a prospectus registered with MAS: {reason} (SFA s. 240)\n\
         {product}的要约须备有经金管局注册的招股说明书: {reason} (证券与期货法第240条)"
    )]
    ProspectusRequired {
        /// The capital markets product being offered.
        product: String,
        /// Why a prospectus is required (no exemption made out).
        reason: String,
    },

    /// A prospectus was lodged but has not been registered by MAS, so the offer
    /// may not proceed (SFA s. 240(1), s. 246).
    #[error(
        "Prospectus has not been registered by MAS - the offer may not be made (SFA s. 246)\n\
         招股说明书尚未获金管局注册,不得进行要约 (证券与期货法第246条)"
    )]
    ProspectusNotRegistered,

    /// A registered prospectus contained a false or misleading statement, or
    /// omitted a required matter (SFA s. 253).
    #[error(
        "Prospectus contains a false or misleading statement or omission: {detail} (SFA s. 253)\n\
         招股说明书含有虚假或误导性陈述或遗漏: {detail} (证券与期货法第253条)"
    )]
    DefectiveProspectus {
        /// Description of the defect.
        detail: String,
    },

    // ----------------------------------------------------------------------
    // Part 12 - Market conduct
    // ----------------------------------------------------------------------
    /// Insider trading: dealing while in possession of non-public, price-sensitive
    /// information (SFA s. 218 for a connected person, s. 219 otherwise).
    #[error(
        "Insider trading prohibited: {detail} (SFA {section})\n\
         禁止内幕交易: {detail} (证券与期货法{section})"
    )]
    InsiderTrading {
        /// Whether the person is a connected person (s. 218) or other (s. 219).
        section: String,
        /// Description of the offending conduct.
        detail: String,
    },

    /// False trading and market rigging transactions creating a false or
    /// misleading appearance of active trading or of the market price
    /// (SFA s. 197).
    #[error(
        "False trading / market rigging prohibited: {detail} (SFA s. 197)\n\
         禁止虚假交易/操纵市场交易: {detail} (证券与期货法第197条)"
    )]
    FalseTrading {
        /// Description of the offending conduct.
        detail: String,
    },

    /// Employment of a manipulative or deceptive device in connection with capital
    /// markets products - the general anti-manipulation / securities-fraud
    /// prohibition (SFA s. 201).
    #[error(
        "Employment of a manipulative or deceptive device prohibited: {detail} (SFA s. 201)\n\
         禁止使用操纵性或欺诈性手段: {detail} (证券与期货法第201条)"
    )]
    MarketManipulation {
        /// Description of the offending conduct.
        detail: String,
    },

    /// Making a false or misleading statement that is likely to induce dealing or
    /// affect the price of capital markets products (SFA s. 199).
    #[error(
        "False or misleading statement likely to affect the market: {detail} (SFA s. 199)\n\
         可能影响市场的虚假或误导性陈述: {detail} (证券与期货法第199条)"
    )]
    MisleadingStatement {
        /// Description of the offending statement.
        detail: String,
    },

    /// Fraudulently or deceptively inducing another person to deal in capital
    /// markets products (SFA s. 200).
    #[error(
        "Fraudulent inducement to deal in capital markets products: {detail} (SFA s. 200)\n\
         欺诈性诱使他人交易资本市场产品: {detail} (证券与期货法第200条)"
    )]
    FraudulentInducement {
        /// Description of the offending conduct.
        detail: String,
    },

    // ----------------------------------------------------------------------
    // Part 4 - Licensing
    // ----------------------------------------------------------------------
    /// Carrying on business in a regulated activity without a Capital Markets
    /// Services (CMS) licence and without an applicable exemption (SFA s. 82).
    #[error(
        "Carrying on a regulated activity ({activity}) without a CMS licence (SFA s. 82)\n\
         未持有资本市场服务执照而从事受监管活动({activity}) (证券与期货法第82条)"
    )]
    UnlicensedRegulatedActivity {
        /// The regulated activity carried on (Second Schedule).
        activity: String,
    },

    /// Acting as a representative for a regulated activity without being an
    /// appointed/provisional/temporary representative on the MAS public register
    /// (SFA s. 99B).
    #[error(
        "Acting as a representative without appointment on the MAS public register: {name} (SFA s. 99B)\n\
         未在金管局公开名册上获委任而担任代表: {name} (证券与期货法第99B条)"
    )]
    UnauthorisedRepresentative {
        /// Name of the person purporting to act as a representative.
        name: String,
    },

    // ----------------------------------------------------------------------
    // Collective investment schemes
    // ----------------------------------------------------------------------
    /// A collective investment scheme was offered to the public without being
    /// authorised (Singapore-constituted, s. 286) or recognised (foreign, s. 287).
    #[error(
        "Collective investment scheme is neither authorised nor recognised by MAS: {scheme} (SFA s. 286/s. 287)\n\
         集体投资计划未获金管局认可或承认: {scheme} (证券与期货法第286/287条)"
    )]
    SchemeNotAuthorised {
        /// Name of the scheme.
        scheme: String,
    },

    // ----------------------------------------------------------------------
    // Enforcement
    // ----------------------------------------------------------------------
    /// A proposed civil penalty exceeds the statutory cap (SFA s. 232): the
    /// greater of three times the profit gained or loss avoided, subject to the
    /// statutory minimum.
    #[error(
        "Civil penalty exceeds the statutory cap (SFA s. 232)\n\
         民事罚款超过法定上限 (证券与期货法第232条)"
    )]
    CivilPenaltyExceedsCap {
        /// The proposed penalty, in SGD cents.
        proposed_cents: u64,
        /// The statutory maximum, in SGD cents.
        maximum_cents: u64,
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
        "Securities and Futures Act validation error: {message}\n\
         证券与期货法验证错误: {message}"
    )]
    ValidationError {
        /// Free-form description of the problem.
        message: String,
    },
}

impl SecuritiesError {
    /// Returns the primary statutory reference for this error.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            SecuritiesError::ProspectusRequired { .. } => "SFA s. 240",
            SecuritiesError::ProspectusNotRegistered => "SFA s. 246",
            SecuritiesError::DefectiveProspectus { .. } => "SFA s. 253",
            SecuritiesError::InsiderTrading { .. } => "SFA s. 218/s. 219",
            SecuritiesError::FalseTrading { .. } => "SFA s. 197",
            SecuritiesError::MarketManipulation { .. } => "SFA s. 201",
            SecuritiesError::MisleadingStatement { .. } => "SFA s. 199",
            SecuritiesError::FraudulentInducement { .. } => "SFA s. 200",
            SecuritiesError::UnlicensedRegulatedActivity { .. } => "SFA s. 82",
            SecuritiesError::UnauthorisedRepresentative { .. } => "SFA s. 99B",
            SecuritiesError::SchemeNotAuthorised { .. } => "SFA s. 286/s. 287",
            SecuritiesError::CivilPenaltyExceedsCap { .. } => "SFA s. 232",
            SecuritiesError::InvalidAmount { .. } => "SFA",
            SecuritiesError::ValidationError { .. } => "SFA",
        }
    }

    /// Returns the severity level (1 = informational, 5 = most serious).
    ///
    /// Insider trading and market manipulation attract the highest severity,
    /// consistent with MAS's enforcement priorities for market abuse.
    pub fn severity(&self) -> u8 {
        match self {
            SecuritiesError::InsiderTrading { .. } => 5,
            SecuritiesError::MarketManipulation { .. } => 5,
            SecuritiesError::FalseTrading { .. } => 5,
            SecuritiesError::FraudulentInducement { .. } => 5,
            SecuritiesError::MisleadingStatement { .. } => 4,
            SecuritiesError::DefectiveProspectus { .. } => 4,
            SecuritiesError::UnlicensedRegulatedActivity { .. } => 4,
            SecuritiesError::SchemeNotAuthorised { .. } => 4,
            SecuritiesError::ProspectusRequired { .. } => 3,
            SecuritiesError::ProspectusNotRegistered => 3,
            SecuritiesError::UnauthorisedRepresentative { .. } => 3,
            SecuritiesError::CivilPenaltyExceedsCap { .. } => 2,
            SecuritiesError::InvalidAmount { .. } => 1,
            SecuritiesError::ValidationError { .. } => 1,
        }
    }

    /// Whether this error denotes market abuse under Part 12 (insider trading,
    /// false trading, manipulation, misleading statements, fraudulent
    /// inducement) which may attract both criminal liability and a civil penalty.
    pub fn is_market_abuse(&self) -> bool {
        matches!(
            self,
            SecuritiesError::InsiderTrading { .. }
                | SecuritiesError::FalseTrading { .. }
                | SecuritiesError::MarketManipulation { .. }
                | SecuritiesError::MisleadingStatement { .. }
                | SecuritiesError::FraudulentInducement { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statute_references() {
        let insider = SecuritiesError::InsiderTrading {
            section: "s. 218".to_string(),
            detail: "director dealt on unannounced results".to_string(),
        };
        assert_eq!(insider.statute_reference(), "SFA s. 218/s. 219");

        let prospectus = SecuritiesError::ProspectusRequired {
            product: "shares".to_string(),
            reason: "public offer".to_string(),
        };
        assert_eq!(prospectus.statute_reference(), "SFA s. 240");

        let licence = SecuritiesError::UnlicensedRegulatedActivity {
            activity: "fund management".to_string(),
        };
        assert_eq!(licence.statute_reference(), "SFA s. 82");
    }

    #[test]
    fn test_severity_ordering() {
        let insider = SecuritiesError::InsiderTrading {
            section: "s. 219".to_string(),
            detail: "tippee dealt".to_string(),
        };
        let invalid = SecuritiesError::InvalidAmount {
            detail: "negative".to_string(),
        };
        assert_eq!(insider.severity(), 5);
        assert_eq!(invalid.severity(), 1);
        assert!(insider.severity() > invalid.severity());
    }

    #[test]
    fn test_is_market_abuse() {
        assert!(
            SecuritiesError::FalseTrading {
                detail: "wash trades".to_string()
            }
            .is_market_abuse()
        );
        assert!(
            !SecuritiesError::UnlicensedRegulatedActivity {
                activity: "dealing".to_string()
            }
            .is_market_abuse()
        );
    }

    #[test]
    fn test_display_is_bilingual() {
        let err = SecuritiesError::MarketManipulation {
            detail: "pump and dump scheme".to_string(),
        };
        let text = err.to_string();
        assert!(text.contains("manipulative or deceptive device"));
        assert!(text.contains("SFA s. 201"));
        assert!(text.contains("操纵性或欺诈性手段"));
    }

    #[test]
    fn test_penalty_cap_error() {
        let err = SecuritiesError::CivilPenaltyExceedsCap {
            proposed_cents: 100_000_000,
            maximum_cents: 50_000_000,
        };
        assert_eq!(err.statute_reference(), "SFA s. 232");
        assert_eq!(err.severity(), 2);
    }
}
