//! Australian Common Module
//!
//! Common types, utilities, and calendar functionality for Australian law.

pub mod calendar;
pub mod types;

pub use calendar::{AustralianCalendar, AustralianHoliday, HolidayType, LimitationPeriod};
pub use types::{AustralianCase, Court, JurisdictionLevel, LegalArea, StateTerritory};
