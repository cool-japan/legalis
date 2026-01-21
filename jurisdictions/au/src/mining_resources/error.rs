//! Mining and resources-specific error types

use thiserror::Error;

/// Mining and resources law error type
#[derive(Debug, Clone, Error)]
pub enum MiningError {
    // =========================================================================
    // Exploration Errors
    // =========================================================================
    /// Exploration licence not granted
    #[error(
        "Exploration licence not granted. Reason: {reason}. \
         Application area may overlap existing tenure or restricted area"
    )]
    ExplorationLicenceNotGranted {
        /// Reason for refusal
        reason: String,
    },

    /// Exploration programme not approved
    #[error(
        "Exploration programme not approved under {state} Mining Act. \
         Missing: {missing}"
    )]
    ExplorationProgrammeNotApproved {
        /// State/territory
        state: String,
        /// Missing requirements
        missing: String,
    },

    /// Exploration report not lodged
    #[error(
        "Exploration report not lodged within required timeframe. \
         Due: {due_date}. Late lodgement may result in licence forfeiture"
    )]
    ExplorationReportNotLodged {
        /// Due date
        due_date: String,
    },

    // =========================================================================
    // Mining Lease Errors
    // =========================================================================
    /// Mining lease not granted
    #[error(
        "Mining lease not granted under {state} Mining Act. \
         Reason: {reason}"
    )]
    MiningLeaseNotGranted {
        /// State/territory
        state: String,
        /// Reason for refusal
        reason: String,
    },

    /// Insufficient financial assurance
    #[error(
        "Insufficient financial assurance/rehabilitation bond. \
         Required: ${required:.2}. Provided: ${provided:.2}"
    )]
    InsufficientFinancialAssurance {
        /// Required amount
        required: f64,
        /// Amount provided
        provided: f64,
    },

    /// Mine plan not approved
    #[error(
        "Mine plan (Plan of Operations) not approved. \
         Missing: {missing}. Environmental impact not adequately addressed"
    )]
    MinePlanNotApproved {
        /// Missing elements
        missing: String,
    },

    /// Mining on restricted land
    #[error(
        "Mining activity on restricted land without required consent. \
         Land category: {land_category}. Consent required from: {consent_from}"
    )]
    MiningOnRestrictedLand {
        /// Category of restricted land
        land_category: String,
        /// Who consent is required from
        consent_from: String,
    },

    // =========================================================================
    // Native Title Errors
    // =========================================================================
    /// Native title process not completed
    #[error(
        "Native title future act process not completed under Native Title Act 1993. \
         Required procedure: {procedure}. Section: {section}"
    )]
    NativeTitleProcessNotCompleted {
        /// Required procedure
        procedure: String,
        /// NTA section
        section: String,
    },

    /// Indigenous land use agreement required
    #[error(
        "Indigenous Land Use Agreement (ILUA) required for mining on native title land. \
         Affected group: {native_title_group}"
    )]
    IluaRequired {
        /// Affected native title group
        native_title_group: String,
    },

    /// Aboriginal heritage site affected
    #[error(
        "Mining activity may affect Aboriginal heritage site. \
         Site: {site_name}. Approval required under {heritage_act}"
    )]
    HeritageApprovalRequired {
        /// Heritage site name/description
        site_name: String,
        /// Applicable heritage act
        heritage_act: String,
    },

    // =========================================================================
    // Environmental Errors
    // =========================================================================
    /// Environmental approval not obtained
    #[error(
        "Environmental approval not obtained. \
         EPBC Act approval required: {epbc_required}. State approval required: {state_required}. \
         Matter of National Environmental Significance: {mnes}"
    )]
    EnvironmentalApprovalNotObtained {
        /// Whether EPBC approval needed
        epbc_required: bool,
        /// Whether state approval needed
        state_required: bool,
        /// MNES description
        mnes: String,
    },

    /// Environmental impact assessment required
    #[error(
        "Environmental Impact Assessment (EIA) required before mining can commence. \
         Level: {assessment_level}. Significant impact on: {impact_on}"
    )]
    EiaRequired {
        /// Level of assessment required
        assessment_level: String,
        /// What the significant impact is on
        impact_on: String,
    },

    /// Rehabilitation obligations not met
    #[error(
        "Rehabilitation obligations not met. \
         Outstanding areas: {outstanding_areas}. Required standard: {required_standard}"
    )]
    RehabilitationObligationsNotMet {
        /// Outstanding areas to be rehabilitated
        outstanding_areas: String,
        /// Required rehabilitation standard
        required_standard: String,
    },

    // =========================================================================
    // Royalty Errors
    // =========================================================================
    /// Royalty not paid
    #[error(
        "Mineral royalty not paid by due date. \
         Mineral: {mineral}. Period: {period}. Amount: ${amount:.2}. \
         Interest and penalties may apply"
    )]
    RoyaltyNotPaid {
        /// Mineral type
        mineral: String,
        /// Royalty period
        period: String,
        /// Amount owed
        amount: f64,
    },

    /// Royalty return not lodged
    #[error(
        "Royalty return not lodged. \
         Due: {due_date}. Estimation and penalties may apply"
    )]
    RoyaltyReturnNotLodged {
        /// Due date
        due_date: String,
    },

    // =========================================================================
    // Safety Errors
    // =========================================================================
    /// Mine safety breach
    #[error(
        "Mine safety breach under {safety_act}. \
         Breach: {breach}. Risk level: {risk_level}"
    )]
    MineSafetyBreach {
        /// Applicable safety act
        safety_act: String,
        /// Description of breach
        breach: String,
        /// Risk level
        risk_level: String,
    },

    /// Principal hazard not managed
    #[error(
        "Principal hazard not adequately managed. \
         Hazard: {hazard}. Required controls: {required_controls}"
    )]
    PrincipalHazardNotManaged {
        /// Hazard description
        hazard: String,
        /// Required controls
        required_controls: String,
    },

    // =========================================================================
    // General Errors
    // =========================================================================
    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },

    /// Tenure lapsed or forfeited
    #[error(
        "Mining tenure has lapsed or been forfeited. \
         Tenure: {tenure_id}. Reason: {reason}"
    )]
    TenureLapsed {
        /// Tenure identifier
        tenure_id: String,
        /// Reason for lapse/forfeiture
        reason: String,
    },
}

/// Result type for mining and resources operations
pub type Result<T> = std::result::Result<T, MiningError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exploration_error() {
        let error = MiningError::ExplorationLicenceNotGranted {
            reason: "Overlap with existing exploration licence".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Exploration licence"));
        assert!(msg.contains("overlap"));
    }

    #[test]
    fn test_native_title_error() {
        let error = MiningError::NativeTitleProcessNotCompleted {
            procedure: "Right to negotiate".to_string(),
            section: "s.31".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Native Title Act 1993"));
        assert!(msg.contains("s.31"));
    }

    #[test]
    fn test_environmental_error() {
        let error = MiningError::EnvironmentalApprovalNotObtained {
            epbc_required: true,
            state_required: true,
            mnes: "Listed threatened species".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("EPBC Act"));
        assert!(msg.contains("threatened species"));
    }
}
