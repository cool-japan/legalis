//! Cross-border transfer of personal data — Transfer Limitation Obligation
//! (PDPA s. 26) and PDP Regulations 2021, Part 3 (regs. 9-12).
//!
//! An organisation must not transfer personal data to a country or territory
//! outside Singapore except in accordance with prescribed requirements that
//! ensure the transferred data will be afforded a standard of protection
//! **comparable** to the protection under the PDPA (s. 26(1)).
//!
//! Under reg. 10, the transferring organisation must ensure the recipient is
//! bound by **legally enforceable obligations** (reg. 11) to provide a
//! comparable standard, unless a deemed-satisfaction case in reg. 10(2) applies
//! (e.g. the individual's consent, data in transit, or publicly available data).

use super::consent::PersonalDataCategory;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Legal basis relied upon to satisfy the Transfer Limitation Obligation
/// (s. 26 read with PDP Regulations 2021, regs. 10-12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferMechanism {
    /// The individual gave consent to the transfer (reg. 10(2)(b) read with the
    /// Consent Obligation). The individual must have been informed that the data
    /// will be transferred and afforded comparable protection.
    Consent,
    /// Transfer is necessary for the performance of a contract between the
    /// individual and the organisation, or to take pre-contractual steps at the
    /// individual's request (reg. 10(3)(a)).
    ContractualNecessity,
    /// The recipient is bound by a written contract requiring it to provide a
    /// comparable standard of protection and specifying the countries to which
    /// the data may be transferred (reg. 11(2)).
    ContractualClauses,
    /// The recipient (a related corporation) is bound by Binding Corporate Rules
    /// (reg. 11(3)-(4)).
    BindingCorporateRules,
    /// The recipient holds a specified certification — APEC/Global Cross-Border
    /// Privacy Rules (CBPR) or, for a data intermediary, the Privacy Recognition
    /// for Processors (PRP) system (reg. 12).
    SpecifiedCertification,
    /// The data is data in transit, or is publicly available, or another
    /// deemed-satisfaction case in reg. 10(2) applies.
    DeemedSatisfaction,
}

impl TransferMechanism {
    /// Returns the governing PDP Regulations 2021 provision for this mechanism.
    pub fn regulation_reference(&self) -> &'static str {
        match self {
            TransferMechanism::Consent => "PDP Regulations 2021, reg. 10(2)(b)",
            TransferMechanism::ContractualNecessity => "PDP Regulations 2021, reg. 10(3)(a)",
            TransferMechanism::ContractualClauses => "PDP Regulations 2021, reg. 11(2)",
            TransferMechanism::BindingCorporateRules => "PDP Regulations 2021, reg. 11(3)-(4)",
            TransferMechanism::SpecifiedCertification => "PDP Regulations 2021, reg. 12",
            TransferMechanism::DeemedSatisfaction => "PDP Regulations 2021, reg. 10(2)",
        }
    }
}

/// A cross-border transfer of personal data and its compliance posture (s. 26).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTransfer {
    /// Stable identifier for this transfer.
    pub transfer_id: String,
    /// Destination country or territory outside Singapore.
    pub destination_country: String,
    /// Purpose of the transfer.
    pub purpose: String,
    /// The mechanism relied upon to satisfy the Transfer Limitation Obligation.
    pub mechanism: TransferMechanism,
    /// Whether the recipient is in fact bound by legally enforceable obligations
    /// to provide a comparable standard of protection (reg. 10/11). This must be
    /// satisfied for the contractual and BCR mechanisms.
    pub recipient_bound_comparable_protection: bool,
    /// For [`TransferMechanism::Consent`]: whether the individual was informed,
    /// before consenting, that the data would be transferred and afforded
    /// comparable protection (reg. 10(2)(b)).
    pub individual_informed_of_transfer: bool,
    /// When the transfer took place / will take place.
    pub transfer_date: DateTime<Utc>,
    /// Categories of personal data transferred.
    pub data_categories: Vec<PersonalDataCategory>,
    /// Number of individuals whose data is transferred.
    pub affected_individuals: u32,
}

impl DataTransfer {
    /// Creates a cross-border transfer record.
    pub fn new(
        transfer_id: impl Into<String>,
        destination_country: impl Into<String>,
        purpose: impl Into<String>,
        mechanism: TransferMechanism,
    ) -> Self {
        Self {
            transfer_id: transfer_id.into(),
            destination_country: destination_country.into(),
            purpose: purpose.into(),
            mechanism,
            recipient_bound_comparable_protection: false,
            individual_informed_of_transfer: false,
            transfer_date: Utc::now(),
            data_categories: Vec::new(),
            affected_individuals: 0,
        }
    }

    /// Records that the recipient is bound by legally enforceable obligations to
    /// provide a comparable standard of protection (reg. 10/11).
    pub fn with_comparable_protection(mut self) -> Self {
        self.recipient_bound_comparable_protection = true;
        self
    }

    /// Records that the individual was informed of the transfer before consenting
    /// (reg. 10(2)(b)).
    pub fn with_informed_consent(mut self) -> Self {
        self.individual_informed_of_transfer = true;
        self
    }

    /// Adds a transferred personal-data category (idempotent).
    pub fn add_data_category(&mut self, category: PersonalDataCategory) -> &mut Self {
        if !self.data_categories.contains(&category) {
            self.data_categories.push(category);
        }
        self
    }

    /// Sets the number of affected individuals.
    pub fn with_affected_individuals(mut self, n: u32) -> Self {
        self.affected_individuals = n;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mechanism_regulation_references() {
        assert_eq!(
            TransferMechanism::ContractualClauses.regulation_reference(),
            "PDP Regulations 2021, reg. 11(2)"
        );
        assert_eq!(
            TransferMechanism::SpecifiedCertification.regulation_reference(),
            "PDP Regulations 2021, reg. 12"
        );
    }

    #[test]
    fn builder_sets_protection_flags() {
        let t = DataTransfer::new(
            "t1",
            "USA",
            "Cloud backup",
            TransferMechanism::ContractualClauses,
        )
        .with_comparable_protection()
        .with_affected_individuals(1000);
        assert!(t.recipient_bound_comparable_protection);
        assert_eq!(t.affected_individuals, 1000);
    }
}
