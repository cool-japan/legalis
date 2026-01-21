//! Tax-specific error types

use thiserror::Error;

/// Tax law error type
#[derive(Debug, Clone, Error)]
pub enum TaxError {
    // =========================================================================
    // Income Tax Errors (ITAA 1997)
    // =========================================================================
    /// Deduction not allowable
    #[error(
        "Deduction not allowable under ITAA 1997 Division 8. Reason: {reason}. \
         Amount: ${amount:.2}"
    )]
    DeductionNotAllowable {
        /// Reason deduction disallowed
        reason: String,
        /// Amount claimed
        amount: f64,
    },

    /// Private expense claimed
    #[error(
        "Private or domestic expense not deductible under s.8-1(2)(b) ITAA 1997. \
         Expense: {expense}"
    )]
    PrivateExpense {
        /// Description of expense
        expense: String,
    },

    /// No nexus to income
    #[error(
        "No nexus between expense and assessable income under s.8-1 ITAA 1997. \
         Expense: {expense}"
    )]
    NoNexusToIncome {
        /// Description of expense
        expense: String,
    },

    /// Capital expense claimed as deduction
    #[error(
        "Capital expense not immediately deductible under s.8-1(2)(a) ITAA 1997. \
         Expense: {expense}. May be eligible for depreciation under Division 40"
    )]
    CapitalExpense {
        /// Description of expense
        expense: String,
    },

    /// Insufficient substantiation
    #[error(
        "Insufficient substantiation under Division 900 ITAA 1997. \
         Deduction: {deduction}. Required: {required}"
    )]
    InsufficientSubstantiation {
        /// Deduction claimed
        deduction: String,
        /// Documentation required
        required: String,
    },

    /// Tax offset not available
    #[error("Tax offset not available. Reason: {reason}")]
    TaxOffsetNotAvailable {
        /// Reason offset not available
        reason: String,
    },

    // =========================================================================
    // GST Errors (GST Act 1999)
    // =========================================================================
    /// Not registered for GST
    #[error(
        "Entity not registered for GST. Registration required if annual turnover \
         exceeds $75,000 threshold under s.23-5 GST Act 1999"
    )]
    NotGstRegistered,

    /// GST charged but not registered
    #[error(
        "GST charged but entity not registered for GST. \
         Contravenes s.105-65 Taxation Administration Act 1953"
    )]
    GstChargedNotRegistered,

    /// Invalid tax invoice
    #[error(
        "Invalid tax invoice. Missing: {missing}. \
         See Division 29 GST Act 1999"
    )]
    InvalidTaxInvoice {
        /// Missing elements
        missing: String,
    },

    /// Input tax credit not available
    #[error(
        "Input tax credit not available. Reason: {reason}. \
         See Division 11 GST Act 1999"
    )]
    ItcNotAvailable {
        /// Reason ITC not available
        reason: String,
    },

    /// Supply not taxable
    #[error("Supply is GST-free under Division 38 GST Act 1999. Category: {category}")]
    GstFreeSupply {
        /// GST-free category
        category: String,
    },

    /// Input taxed supply
    #[error(
        "Supply is input taxed under Division 40 GST Act 1999. \
         No GST charged but no ITC on acquisitions"
    )]
    InputTaxedSupply,

    /// BAS not lodged
    #[error(
        "BAS not lodged by due date. Period: {period}. \
         Due: {due_date}. Late lodgement penalty may apply"
    )]
    BasNotLodged {
        /// BAS period
        period: String,
        /// Due date
        due_date: String,
    },

    // =========================================================================
    // CGT Errors (ITAA 1997 Part 3-1)
    // =========================================================================
    /// CGT asset not identified
    #[error("CGT asset not clearly identified. See s.108-5 ITAA 1997")]
    CgtAssetNotIdentified,

    /// CGT event not recognized
    #[error(
        "CGT event not recognized under Division 104 ITAA 1997. \
         Event type: {event_type}"
    )]
    CgtEventNotRecognized {
        /// Event type description
        event_type: String,
    },

    /// No CGT discount available
    #[error(
        "CGT discount not available. Reason: {reason}. \
         See s.115-25 ITAA 1997"
    )]
    CgtDiscountNotAvailable {
        /// Reason discount not available
        reason: String,
    },

    /// Main residence exemption not available
    #[error(
        "Main residence exemption not available under Subdivision 118-B ITAA 1997. \
         Reason: {reason}"
    )]
    MainResidenceExemptionNotAvailable {
        /// Reason exemption not available
        reason: String,
    },

    /// Cost base not established
    #[error(
        "Cost base not properly established under Division 110 ITAA 1997. \
         Missing: {missing}"
    )]
    CostBaseNotEstablished {
        /// Missing cost base elements
        missing: String,
    },

    // =========================================================================
    // General Errors
    // =========================================================================
    /// Invalid TFN
    #[error("Invalid Tax File Number (TFN). TFN must be 9 digits")]
    InvalidTfn,

    /// Invalid ABN
    #[error("Invalid Australian Business Number (ABN). ABN must be 11 digits with valid checksum")]
    InvalidAbn,

    /// Lodgement deadline missed
    #[error(
        "Tax return lodgement deadline missed. Due: {due_date}. \
         Late lodgement penalty may apply under Div 286 Schedule 1 TAA 1953"
    )]
    LodgementDeadlineMissed {
        /// Due date
        due_date: String,
    },

    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },
}

/// Result type for tax operations
pub type Result<T> = std::result::Result<T, TaxError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deduction_error() {
        let error = TaxError::DeductionNotAllowable {
            reason: "Entertainment expense".to_string(),
            amount: 500.0,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("ITAA 1997"));
        assert!(msg.contains("Division 8"));
    }

    #[test]
    fn test_gst_error() {
        let error = TaxError::NotGstRegistered;
        let msg = format!("{}", error);
        assert!(msg.contains("GST Act 1999"));
        assert!(msg.contains("$75,000"));
    }

    #[test]
    fn test_cgt_error() {
        let error = TaxError::CgtDiscountNotAvailable {
            reason: "Asset held less than 12 months".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("s.115-25"));
    }
}
