//! Error types for constitutional validation and operations.
//!
//! This module provides comprehensive error types with bilingual messages
//! (Lao/English) for constitutional law operations.

use crate::constitution::types::{
    AmendmentProposer, CourtLevel, FundamentalRight, ProsecutorLevel,
};
use thiserror::Error;

/// Constitutional error type with article references
/// ປະເພດຄວາມຜິດພາດທາງລັດຖະທຳມະນູນ
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConstitutionalError {
    /// Voting rights violation (Article 35)
    /// ການລະເມີດສິດເລືອກຕັ້ງ
    #[error("Voting rights violation (Article 35): {lao} / {english}")]
    VotingRightsViolation {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Fundamental right violation (Articles 34-51)
    /// ການລະເມີດສິດພື້ນຖານ
    #[error("Fundamental right violation (Article {article}): {lao} / {english}")]
    FundamentalRightViolation {
        /// The violated right
        right: FundamentalRight,
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Rights limitation not justified
    /// ການຈຳກັດສິດບໍ່ຊອບທຳ
    #[error("Unjustified rights limitation: {lao} / {english}")]
    UnjustifiedLimitation {
        /// The right being limited
        right: FundamentalRight,
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Reason for failure
        reason: LimitationFailure,
    },

    /// Invalid state structure (Articles 52-100)
    /// ໂຄງສ້າງລັດບໍ່ຖືກຕ້ອງ
    #[error("Invalid state structure (Article {article}): {lao} / {english}")]
    InvalidStateStructure {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// National Assembly violation (Articles 52-65)
    /// ການລະເມີດຕໍ່ສະພາແຫ່ງຊາດ
    #[error("National Assembly violation (Article {article}): {lao} / {english}")]
    NationalAssemblyViolation {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Presidential power violation (Articles 66-70)
    /// ການລະເມີດອຳນາດປະທານປະເທດ
    #[error("Presidential power violation (Article {article}): {lao} / {english}")]
    PresidentialViolation {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Government power violation (Articles 71-80)
    /// ການລະເມີດອຳນາດລັດຖະບານ
    #[error("Government power violation (Article {article}): {lao} / {english}")]
    GovernmentViolation {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Local administration violation (Articles 81-87)
    /// ການລະເມີດການປົກຄອງທ້ອງຖິ່ນ
    #[error("Local administration violation (Article {article}): {lao} / {english}")]
    LocalAdministrationViolation {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Court organization violation (Articles 88-95)
    /// ການລະເມີດໂຄງສ້າງສານ
    #[error("Court organization violation (Article {article}): {lao} / {english}")]
    CourtViolation {
        /// Court level
        level: CourtLevel,
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Prosecutor organization violation (Articles 96-100)
    /// ການລະເມີດໂຄງສ້າງອົງການໄອຍະການ
    #[error("Prosecutor organization violation (Article {article}): {lao} / {english}")]
    ProsecutorViolation {
        /// Prosecutor level
        level: ProsecutorLevel,
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Judicial independence violation (Article 88)
    /// ການລະເມີດຄວາມເປັນເອກະລາດຂອງສານ
    #[error("Judicial independence violation (Article 88): {lao} / {english}")]
    JudicialIndependenceViolation {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
    },

    /// Constitutional amendment violation (Articles 105-108)
    /// ການລະເມີດຂັ້ນຕອນການແກ້ໄຂລັດຖະທຳມະນູນ
    #[error("Constitutional amendment violation (Article {article}): {lao} / {english}")]
    AmendmentViolation {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
        /// Article reference
        article: u8,
    },

    /// Invalid amendment proposer (Article 105)
    /// ຜູ້ສະເໜີການແກ້ໄຂບໍ່ຖືກຕ້ອງ
    #[error("Invalid amendment proposer (Article 105): {lao} / {english}")]
    InvalidAmendmentProposer {
        /// Proposed by
        proposer: AmendmentProposer,
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
    },

    /// Insufficient votes for amendment (Article 106)
    /// ຄະແນນສຽງບໍ່ພຽງພໍສຳລັບການແກ້ໄຂ
    #[error(
        "Insufficient votes for amendment (Article 106): {votes_received}/{required_votes} - {lao} / {english}"
    )]
    InsufficientAmendmentVotes {
        /// Required votes (2/3 majority)
        required_votes: u32,
        /// Actual votes received
        votes_received: u32,
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
    },

    /// Generic validation error
    /// ຄວາມຜິດພາດໃນການກວດສອບ
    #[error("Validation error: {lao} / {english}")]
    ValidationError {
        /// Lao error message
        lao: String,
        /// English error message
        english: String,
    },
}

/// Reason why rights limitation failed justification test
/// ເຫດຜົນທີ່ການຈຳກັດສິດບໍ່ຜ່ານການທົດສອບ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LimitationFailure {
    /// No legitimate aim
    /// ບໍ່ມີຈຸດປະສົງທີ່ຊອບທຳ
    NoLegitimatAim,

    /// Not necessary
    /// ບໍ່ຈຳເປັນ
    NotNecessary,

    /// Not proportional
    /// ບໍ່ສົມສ່ວນ
    NotProportional,

    /// No legal basis
    /// ບໍ່ມີພື້ນຖານທາງກົດໝາຍ
    NoLegalBasis,
}

impl ConstitutionalError {
    /// Create a voting rights violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດສິດເລືອກຕັ້ງ
    pub fn voting_rights_violation(age: u8, minimum_age: u8) -> Self {
        ConstitutionalError::VotingRightsViolation {
            lao: format!("ອາຍຸ {} ປີ ບໍ່ເຖິງອາຍຸຂັ້ນຕ່ຳ {} ປີສຳລັບການເລືອກຕັ້ງ", age, minimum_age),
            english: format!("Age {} is below minimum voting age of {}", age, minimum_age),
            article: 35,
        }
    }

    /// Create a fundamental right violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດສິດພື້ນຖານ
    pub fn fundamental_right_violation(
        right: FundamentalRight,
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::FundamentalRightViolation {
            right,
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }

    /// Create an unjustified limitation error
    /// ສ້າງຄວາມຜິດພາດການຈຳກັດສິດບໍ່ຊອບທຳ
    pub fn unjustified_limitation(right: FundamentalRight, reason: LimitationFailure) -> Self {
        let (lao, english) = match reason {
            LimitationFailure::NoLegitimatAim => (
                "ບໍ່ມີຈຸດປະສົງທີ່ຊອບທຳ".to_string(),
                "No legitimate aim".to_string(),
            ),
            LimitationFailure::NotNecessary => (
                "ການຈຳກັດບໍ່ຈຳເປັນ".to_string(),
                "Limitation is not necessary".to_string(),
            ),
            LimitationFailure::NotProportional => (
                "ການຈຳກັດບໍ່ສົມສ່ວນກັບຈຸດປະສົງ".to_string(),
                "Limitation is not proportional to the aim".to_string(),
            ),
            LimitationFailure::NoLegalBasis => (
                "ບໍ່ມີພື້ນຖານທາງກົດໝາຍ".to_string(),
                "No legal basis for limitation".to_string(),
            ),
        };

        ConstitutionalError::UnjustifiedLimitation {
            right,
            lao,
            english,
            reason,
        }
    }

    /// Create a National Assembly violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດຕໍ່ສະພາແຫ່ງຊາດ
    pub fn national_assembly_violation(
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::NationalAssemblyViolation {
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }

    /// Create a court organization violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດໂຄງສ້າງສານ
    pub fn court_violation(
        level: CourtLevel,
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::CourtViolation {
            level,
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }

    /// Create a judicial independence violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດຄວາມເປັນເອກະລາດຂອງສານ
    pub fn judicial_independence_violation(
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
    ) -> Self {
        ConstitutionalError::JudicialIndependenceViolation {
            lao: lao_msg.into(),
            english: english_msg.into(),
        }
    }

    /// Create an insufficient votes error for constitutional amendment
    /// ສ້າງຄວາມຜິດພາດຄະແນນສຽງບໍ່ພຽງພໍສຳລັບການແກ້ໄຂລັດຖະທຳມະນູນ
    pub fn insufficient_amendment_votes(required: u32, received: u32) -> Self {
        ConstitutionalError::InsufficientAmendmentVotes {
            required_votes: required,
            votes_received: received,
            lao: format!("ຄະແນນສຽງ {} ບໍ່ເຖິງ 2/3 ທີ່ຕ້ອງການ ({} ສຽງ)", received, required),
            english: format!(
                "Received {} votes, but {} votes (2/3 majority) required",
                received, required
            ),
        }
    }

    /// Create a presidential violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດອຳນາດປະທານປະເທດ
    pub fn presidential_violation(
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::PresidentialViolation {
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }

    /// Create a government violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດອຳນາດລັດຖະບານ
    pub fn government_violation(
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::GovernmentViolation {
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }

    /// Create a local administration violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດການປົກຄອງທ້ອງຖິ່ນ
    pub fn local_administration_violation(
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::LocalAdministrationViolation {
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }

    /// Create a prosecutor violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດໂຄງສ້າງອົງການໄອຍະການ
    pub fn prosecutor_violation(
        level: ProsecutorLevel,
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::ProsecutorViolation {
            level,
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }

    /// Create a constitutional amendment violation error
    /// ສ້າງຄວາມຜິດພາດການລະເມີດຂັ້ນຕອນການແກ້ໄຂລັດຖະທຳມະນູນ
    pub fn amendment_violation(
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::AmendmentViolation {
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }

    /// Create an invalid state structure error
    /// ສ້າງຄວາມຜິດພາດໂຄງສ້າງລັດບໍ່ຖືກຕ້ອງ
    pub fn invalid_state_structure(
        lao_msg: impl Into<String>,
        english_msg: impl Into<String>,
        article: u8,
    ) -> Self {
        ConstitutionalError::InvalidStateStructure {
            lao: lao_msg.into(),
            english: english_msg.into(),
            article,
        }
    }
}

/// Result type for constitutional operations
/// ປະເພດຜົນໄດ້ຮັບສຳລັບການດຳເນີນງານທາງລັດຖະທຳມະນູນ
pub type ConstitutionalResult<T> = Result<T, ConstitutionalError>;
