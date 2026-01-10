//! Data Subject Rights (GDPR Chapter III, Articles 15-22)
//!
//! This module implements validation logic for data subject rights requests.

use crate::gdpr::{error::GdprError, types::DataSubjectRight};

/// Data Subject Request (DSR) builder for Chapter III rights
///
/// ## Example
///
/// ```rust
/// use legalis_eu::gdpr::*;
///
/// let request = DataSubjectRequest::new()
///     .with_data_subject("john.doe@example.com")
///     .with_right(DataSubjectRight::Erasure)
///     .with_controller("Acme Corp")
///     .with_grounds("No longer necessary for original purpose");
///
/// let response = request.validate();
/// assert!(response.is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct DataSubjectRequest {
    /// Data subject making the request
    pub data_subject_id: Option<String>,

    /// Right being exercised
    pub right: Option<DataSubjectRight>,

    /// Data controller
    pub controller: Option<String>,

    /// Grounds for the request (for erasure, objection, etc.)
    pub grounds: Option<String>,

    /// Request timestamp (for deadline tracking)
    pub requested_at: Option<String>,
}

impl DataSubjectRequest {
    /// Create a new data subject request
    pub fn new() -> Self {
        Self {
            data_subject_id: None,
            right: None,
            controller: None,
            grounds: None,
            requested_at: None,
        }
    }

    /// Set the data subject ID
    pub fn with_data_subject(mut self, id: impl Into<String>) -> Self {
        self.data_subject_id = Some(id.into());
        self
    }

    /// Set the right being exercised
    pub fn with_right(mut self, right: DataSubjectRight) -> Self {
        self.right = Some(right);
        self
    }

    /// Set the data controller
    pub fn with_controller(mut self, controller: impl Into<String>) -> Self {
        self.controller = Some(controller.into());
        self
    }

    /// Set the grounds for the request
    pub fn with_grounds(mut self, grounds: impl Into<String>) -> Self {
        self.grounds = Some(grounds.into());
        self
    }

    /// Validate the request and determine response requirements
    pub fn validate(&self) -> Result<RequestValidation, GdprError> {
        if self.data_subject_id.is_none() {
            return Err(GdprError::missing_field("data_subject_id"));
        }

        if self.right.is_none() {
            return Err(GdprError::missing_field("right"));
        }

        let right = self.right.as_ref().unwrap();

        // Determine response deadline (Article 12(3): 1 month, extendable by 2 months)
        let deadline_days = 30;

        // Check if grounds are required
        let requires_grounds =
            matches!(right, DataSubjectRight::Erasure | DataSubjectRight::Object);

        if requires_grounds && self.grounds.is_none() {
            return Err(GdprError::invalid_request(format!(
                "Grounds required for {:?} request",
                right
            )));
        }

        // Determine applicable exceptions
        let exceptions = self.check_exceptions(right)?;

        Ok(RequestValidation {
            right: *right,
            deadline_days,
            must_comply: exceptions.is_empty(),
            exceptions,
        })
    }

    /// Check for exceptions to data subject rights
    fn check_exceptions(&self, right: &DataSubjectRight) -> Result<Vec<String>, GdprError> {
        let mut exceptions = Vec::new();

        match right {
            DataSubjectRight::Erasure => {
                // Article 17(3) exceptions
                exceptions
                    .push("Article 17(3)(a): Freedom of expression and information".to_string());
                exceptions.push("Article 17(3)(b): Compliance with legal obligation".to_string());
                exceptions.push("Article 17(3)(c): Public health reasons".to_string());
                exceptions.push("Article 17(3)(d): Archiving, research, statistics".to_string());
                exceptions.push("Article 17(3)(e): Legal claims".to_string());
            }
            DataSubjectRight::Object => {
                // Article 21 - can override objection for compelling legitimate grounds
                exceptions
                    .push("Compelling legitimate grounds override (Article 21(1))".to_string());
                exceptions.push("Legal claims defense (Article 21(1))".to_string());
            }
            DataSubjectRight::DataPortability => {
                // Article 20(3) - doesn't apply to all processing
                exceptions.push(
                    "Only applies to automated processing based on consent or contract".to_string(),
                );
            }
            _ => {}
        }

        Ok(exceptions)
    }
}

impl Default for DataSubjectRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of request validation
#[derive(Debug, Clone)]
pub struct RequestValidation {
    pub right: DataSubjectRight,
    pub deadline_days: u32,
    pub must_comply: bool,
    pub exceptions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_request() {
        let request = DataSubjectRequest::new()
            .with_data_subject("user@example.com")
            .with_right(DataSubjectRight::Access)
            .with_controller("Acme Corp");

        let result = request.validate();
        assert!(result.is_ok());

        let validation = result.unwrap();
        assert_eq!(validation.right, DataSubjectRight::Access);
        assert_eq!(validation.deadline_days, 30);
    }

    #[test]
    fn test_erasure_request_with_grounds() {
        let request = DataSubjectRequest::new()
            .with_data_subject("user@example.com")
            .with_right(DataSubjectRight::Erasure)
            .with_controller("Acme Corp")
            .with_grounds("No longer necessary");

        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_erasure_request_missing_grounds() {
        let request = DataSubjectRequest::new()
            .with_data_subject("user@example.com")
            .with_right(DataSubjectRight::Erasure)
            .with_controller("Acme Corp");
        // No grounds provided

        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_data_subject() {
        let request = DataSubjectRequest::new().with_right(DataSubjectRight::Access);

        let result = request.validate();
        assert!(matches!(result, Err(GdprError::MissingField(_))));
    }

    #[test]
    fn test_erasure_has_exceptions() {
        let request = DataSubjectRequest::new()
            .with_data_subject("user@example.com")
            .with_right(DataSubjectRight::Erasure)
            .with_grounds("Test")
            .with_controller("Acme Corp");

        let validation = request.validate().unwrap();
        assert!(!validation.exceptions.is_empty());
        assert!(
            validation
                .exceptions
                .iter()
                .any(|e| e.contains("Legal claims"))
        );
    }
}
