//! EU Trademark Regulation (EU) 2017/1001
//!
//! Implements validation for European Union Trademarks (EUTM).

use super::error::IpError;
use super::types::{MarkType, NiceClass, TrademarkStatus};
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// European Union Trademark application/registration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EuTrademark {
    /// Mark representation (text, image description, etc.)
    pub mark_representation: Option<String>,

    /// Type of mark
    pub mark_type: Option<MarkType>,

    /// Applicant/owner name
    pub applicant: Option<String>,

    /// Nice classification classes
    pub nice_classes: Vec<NiceClass>,

    /// Goods and services description per class
    pub goods_services: Vec<String>,

    /// Filing date
    pub filing_date: Option<DateTime<Utc>>,

    /// Status
    pub status: Option<TrademarkStatus>,

    /// Priority date (if claiming priority)
    pub priority_date: Option<DateTime<Utc>>,

    /// Is mark descriptive
    pub is_descriptive: bool,

    /// Is mark generic
    pub is_generic: bool,

    /// Has secondary meaning (acquired distinctiveness)
    pub has_secondary_meaning: bool,
}

impl EuTrademark {
    pub fn new() -> Self {
        Self {
            mark_representation: None,
            mark_type: None,
            applicant: None,
            nice_classes: Vec::new(),
            goods_services: Vec::new(),
            filing_date: None,
            status: None,
            priority_date: None,
            is_descriptive: false,
            is_generic: false,
            has_secondary_meaning: false,
        }
    }

    pub fn with_mark_text(mut self, text: impl Into<String>) -> Self {
        self.mark_representation = Some(text.into());
        self
    }

    pub fn with_mark_type(mut self, mark_type: MarkType) -> Self {
        self.mark_type = Some(mark_type);
        self
    }

    pub fn with_applicant(mut self, applicant: impl Into<String>) -> Self {
        self.applicant = Some(applicant.into());
        self
    }

    pub fn add_nice_class(mut self, class: u8) -> Result<Self, IpError> {
        let nice_class = NiceClass::new(class)?;
        self.nice_classes.push(nice_class);
        Ok(self)
    }

    pub fn add_goods_services(mut self, description: impl Into<String>) -> Self {
        self.goods_services.push(description.into());
        self
    }

    pub fn with_filing_date(mut self, date: DateTime<Utc>) -> Self {
        self.filing_date = Some(date);
        self
    }

    pub fn with_status(mut self, status: TrademarkStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_descriptive(mut self, is_descriptive: bool) -> Self {
        self.is_descriptive = is_descriptive;
        self
    }

    pub fn with_generic(mut self, is_generic: bool) -> Self {
        self.is_generic = is_generic;
        self
    }

    pub fn with_secondary_meaning(mut self, has_secondary_meaning: bool) -> Self {
        self.has_secondary_meaning = has_secondary_meaning;
        self
    }

    /// Validate trademark under EU Trademark Regulation
    ///
    /// Checks:
    /// - Article 4: Absolute grounds for refusal
    /// - Article 7: Distinctiveness requirements
    /// - Nice classification validity
    pub fn validate(&self) -> Result<TrademarkValidation, IpError> {
        // Check required fields
        if self.mark_representation.is_none() {
            return Err(IpError::missing_field("mark_representation"));
        }

        if self.mark_type.is_none() {
            return Err(IpError::missing_field("mark_type"));
        }

        if self.applicant.is_none() {
            return Err(IpError::missing_field("applicant"));
        }

        if self.nice_classes.is_empty() {
            return Err(IpError::invalid_trademark(
                "At least one Nice class required",
            ));
        }

        // Article 7(1)(b): Lack of distinctiveness
        if self.is_descriptive && !self.has_secondary_meaning {
            return Err(IpError::LackOfDistinctiveness {
                reason: "Mark is descriptive and lacks secondary meaning (Art. 7(1)(b))"
                    .to_string(),
            });
        }

        // Article 7(1)(d): Generic marks
        if self.is_generic {
            return Err(IpError::invalid_trademark(
                "Mark is generic and cannot be registered (Art. 7(1)(d))",
            ));
        }

        // Calculate protection period if registered
        let protection_expiry = if let Some(TrademarkStatus::Registered {
            registration_date: _,
            renewal_due,
        }) = &self.status
        {
            Some(*renewal_due)
        } else {
            None
        };

        Ok(TrademarkValidation {
            is_registrable: true,
            distinctiveness_established: !self.is_descriptive || self.has_secondary_meaning,
            nice_classes_valid: true,
            protection_expiry,
            recommendations: Vec::new(),
        })
    }
}

impl Default for EuTrademark {
    fn default() -> Self {
        Self::new()
    }
}

/// Trademark validation result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrademarkValidation {
    /// Whether mark is registrable
    pub is_registrable: bool,

    /// Whether distinctiveness is established
    pub distinctiveness_established: bool,

    /// Whether Nice classes are valid
    pub nice_classes_valid: bool,

    /// Protection expiry date (if registered)
    pub protection_expiry: Option<DateTime<Utc>>,

    /// Recommendations for applicant
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_word_mark() {
        let trademark = EuTrademark::new()
            .with_mark_text("INNOVATIVE")
            .with_mark_type(MarkType::WordMark)
            .with_applicant("Example GmbH")
            .add_nice_class(9)
            .unwrap()
            .add_goods_services("Computer software");

        let result = trademark.validate();
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.is_registrable);
    }

    #[test]
    fn test_descriptive_mark_without_secondary_meaning() {
        let trademark = EuTrademark::new()
            .with_mark_text("FAST")
            .with_mark_type(MarkType::WordMark)
            .with_applicant("Example GmbH")
            .add_nice_class(9)
            .unwrap()
            .with_descriptive(true);

        let result = trademark.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_descriptive_mark_with_secondary_meaning() {
        let trademark = EuTrademark::new()
            .with_mark_text("WINDOWS")
            .with_mark_type(MarkType::WordMark)
            .with_applicant("Software Corp")
            .add_nice_class(9)
            .unwrap()
            .with_descriptive(true)
            .with_secondary_meaning(true);

        let result = trademark.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_generic_mark_rejected() {
        let trademark = EuTrademark::new()
            .with_mark_text("COMPUTER")
            .with_mark_type(MarkType::WordMark)
            .with_applicant("Example GmbH")
            .add_nice_class(9)
            .unwrap()
            .with_generic(true);

        let result = trademark.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_nice_class() {
        let result = EuTrademark::new()
            .with_mark_text("TEST")
            .with_mark_type(MarkType::WordMark)
            .with_applicant("Example GmbH")
            .add_nice_class(99); // Invalid class

        assert!(result.is_err());
    }
}
