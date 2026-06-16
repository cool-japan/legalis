//! Do Not Call (DNC) Registry — PDPA Part 9 (ss. 36-48).
//!
//! Part 9 establishes the Do Not Call Registry, comprising three separate
//! registers — one for each kind of *specified message* (s. 39):
//!
//! * the **No Voice Call Register**,
//! * the **No Text Message Register**, and
//! * the **No Fax Message Register**.
//!
//! Before sending a specified message (i.e. a marketing message) to a Singapore
//! telephone number, the sender must, within the prescribed duration, have
//! obtained confirmation from the Commission that the number is **not** listed
//! on the relevant register (s. 43(1)/(2)). The prescribed duration is
//! **21 days** for messages sent on or after 1 February 2021 (PDP (Do Not Call
//! Registry) Regulations 2013, reg. 15, as amended by S 67/2021).

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Validity period (in days) of a DNC non-registration confirmation obtained
/// from the Commission before sending a specified message (PDPA s. 43(2)(a) read
/// with reg. 15 of the PDP (Do Not Call Registry) Regulations 2013, as amended
/// by S 67/2021, in force from 1 February 2021).
pub const DNC_CONFIRMATION_VALIDITY_DAYS: i64 = 21;

/// Kind of specified message, each mapping to one of the three DNC registers
/// (PDPA s. 39).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DncRegisterKind {
    /// Marketing voice call — No Voice Call Register.
    VoiceCall,
    /// Marketing text message (SMS/MMS) — No Text Message Register.
    TextMessage,
    /// Marketing fax — No Fax Message Register.
    Fax,
}

impl DncRegisterKind {
    /// Returns the human-readable register name (s. 39).
    pub fn register_name(&self) -> &'static str {
        match self {
            DncRegisterKind::VoiceCall => "No Voice Call Register",
            DncRegisterKind::TextMessage => "No Text Message Register",
            DncRegisterKind::Fax => "No Fax Message Register",
        }
    }
}

/// A Singapore telephone number's registration status across the three DNC
/// registers (the data an organisation would maintain after querying the
/// Commission's registry).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DncRegistration {
    /// Singapore telephone number in `+65########` form.
    pub phone_number: String,
    /// Registers on which the number is currently listed.
    pub registered_on: Vec<DncRegisterKind>,
}

impl DncRegistration {
    /// Creates a registration record for a number that is not yet listed on any
    /// register.
    pub fn new(phone_number: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
            registered_on: Vec::new(),
        }
    }

    /// Marks the number as listed on the given register (idempotent).
    pub fn register(&mut self, kind: DncRegisterKind) -> &mut Self {
        if !self.registered_on.contains(&kind) {
            self.registered_on.push(kind);
        }
        self
    }

    /// Returns whether the number is listed on the register for the given
    /// message kind.
    pub fn is_listed_on(&self, kind: DncRegisterKind) -> bool {
        self.registered_on.contains(&kind)
    }
}

/// A confirmation obtained from the Commission that a given number is *not*
/// listed on a given register (PDPA s. 43(2)(a)). Such a confirmation is valid
/// for [`DNC_CONFIRMATION_VALIDITY_DAYS`] days.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DncCheckConfirmation {
    /// The number that was checked.
    pub phone_number: String,
    /// The register the confirmation relates to.
    pub register: DncRegisterKind,
    /// When the confirmation of non-registration was obtained.
    pub checked_at: DateTime<Utc>,
}

impl DncCheckConfirmation {
    /// Records a confirmation obtained now.
    pub fn new(phone_number: impl Into<String>, register: DncRegisterKind) -> Self {
        Self {
            phone_number: phone_number.into(),
            register,
            checked_at: Utc::now(),
        }
    }

    /// Records a confirmation obtained at a specific time (useful for testing the
    /// 21-day validity window deterministically).
    pub fn at(
        phone_number: impl Into<String>,
        register: DncRegisterKind,
        checked_at: DateTime<Utc>,
    ) -> Self {
        Self {
            phone_number: phone_number.into(),
            register,
            checked_at,
        }
    }

    /// Returns the instant at which this confirmation expires (21 days after it
    /// was obtained).
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.checked_at + Duration::days(DNC_CONFIRMATION_VALIDITY_DAYS)
    }

    /// Returns whether this confirmation is still valid at instant `now`
    /// (within the 21-day window) and covers the given number and register.
    pub fn is_valid_for(
        &self,
        phone_number: &str,
        register: DncRegisterKind,
        now: DateTime<Utc>,
    ) -> bool {
        self.phone_number == phone_number && self.register == register && now <= self.expires_at()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_names() {
        assert_eq!(
            DncRegisterKind::VoiceCall.register_name(),
            "No Voice Call Register"
        );
        assert_eq!(
            DncRegisterKind::TextMessage.register_name(),
            "No Text Message Register"
        );
        assert_eq!(
            DncRegisterKind::Fax.register_name(),
            "No Fax Message Register"
        );
    }

    #[test]
    fn registration_per_register() {
        let mut reg = DncRegistration::new("+6591234567");
        reg.register(DncRegisterKind::VoiceCall);
        assert!(reg.is_listed_on(DncRegisterKind::VoiceCall));
        assert!(!reg.is_listed_on(DncRegisterKind::TextMessage));
    }

    #[test]
    fn confirmation_valid_for_21_days() {
        let checked = DateTime::parse_from_rfc3339("2026-02-02T10:00:00Z")
            .expect("valid")
            .with_timezone(&Utc);
        let conf = DncCheckConfirmation::at("+6591234567", DncRegisterKind::VoiceCall, checked);

        // Day 0 and day 21 inclusive are valid; day 22 is not.
        let day21 = checked + Duration::days(21);
        let day22 = checked + Duration::days(22);
        assert!(conf.is_valid_for("+6591234567", DncRegisterKind::VoiceCall, checked));
        assert!(conf.is_valid_for("+6591234567", DncRegisterKind::VoiceCall, day21));
        assert!(!conf.is_valid_for("+6591234567", DncRegisterKind::VoiceCall, day22));
    }

    #[test]
    fn confirmation_must_match_register_and_number() {
        let checked = Utc::now();
        let conf = DncCheckConfirmation::at("+6591234567", DncRegisterKind::VoiceCall, checked);
        assert!(!conf.is_valid_for("+6599999999", DncRegisterKind::VoiceCall, checked));
        assert!(!conf.is_valid_for("+6591234567", DncRegisterKind::TextMessage, checked));
    }
}
