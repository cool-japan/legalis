//! Criminal Procedure Code (CrPC) 1973 / BNSS 2023 Types
//!
//! Types for India's criminal procedure law (now replaced by BNSS 2023)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Type of arrest under CrPC/BNSS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrestType {
    /// With warrant (Section 41)
    WithWarrant,
    /// Without warrant for cognizable offence (Section 41)
    WithoutWarrant,
    /// Preventive detention
    PreventiveDetention,
}

/// Bail type (Section 436-450)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BailType {
    /// Regular bail (Section 437)
    Regular,
    /// Anticipatory bail (Section 438)
    Anticipatory,
    /// Default bail (Section 167(2))
    Default,
    /// Interim bail
    Interim,
}

/// Investigation stage (Section 154-176)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestigationStage {
    /// FIR registered (Section 154)
    FirRegistered,
    /// Investigation started
    Started,
    /// Evidence collection
    EvidenceCollection,
    /// Chargesheet filed (Section 173)
    ChargesheetFiled,
    /// Closure report filed (Section 169)
    ClosureReport,
    /// Investigation completed
    Completed,
}

/// Trial stage (Section 225-237)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrialStage {
    /// Cognizance taken (Section 190)
    Cognizance,
    /// Framing of charges (Section 228)
    ChargingFramed,
    /// Prosecution evidence
    ProsecutionEvidence,
    /// Section 313 examination
    ExaminationAccused,
    /// Defence evidence
    DefenceEvidence,
    /// Arguments
    Arguments,
    /// Judgment
    Judgment,
    /// Sentence
    Sentence,
}

/// Police custody/remand (Section 167)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoliceRemand {
    /// Accused name
    pub accused_name: String,
    /// FIR number
    pub fir_number: String,
    /// Arrest date
    pub arrest_date: DateTime<Utc>,
    /// Remand period (days)
    pub remand_days: u32,
    /// Maximum remand allowed (15 days total)
    pub max_remand: u32,
    /// Grounds for remand
    pub grounds: String,
    /// Is within 24 hours of arrest
    pub within_24_hours: bool,
}

impl PoliceRemand {
    /// Maximum police custody allowed (Section 167(2))
    pub const MAX_POLICE_CUSTODY: u32 = 15;

    /// Check if remand period is valid
    pub fn is_valid_period(&self) -> bool {
        self.remand_days <= Self::MAX_POLICE_CUSTODY && self.remand_days <= self.max_remand
    }
}

/// Bail application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BailApplication {
    /// Application number
    pub application_number: String,
    /// Bail type
    pub bail_type: BailType,
    /// Accused name
    pub accused_name: String,
    /// FIR number
    pub fir_number: String,
    /// Offence sections
    pub offence_sections: Vec<u32>,
    /// Is offence bailable
    pub is_bailable: bool,
    /// Is offence cognizable
    pub is_cognizable: bool,
    /// Bail amount (if granted)
    pub bail_amount: Option<f64>,
    /// Sureties required
    pub sureties_required: Option<u32>,
    /// Status
    pub status: BailStatus,
}

/// Bail status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BailStatus {
    /// Pending
    Pending,
    /// Granted
    Granted,
    /// Rejected
    Rejected,
    /// Cancelled
    Cancelled,
    /// Sureties pending
    SuretiesPending,
}

/// Chargesheet (Section 173)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chargesheet {
    /// FIR number
    pub fir_number: String,
    /// Date of FIR
    pub fir_date: NaiveDate,
    /// Accused list
    pub accused: Vec<String>,
    /// Offence sections
    pub sections: Vec<u32>,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Is within time limit (60/90 days)
    pub within_time_limit: bool,
    /// Investigation officer
    pub io_name: String,
    /// Police station
    pub police_station: String,
}

impl Chargesheet {
    /// Check if filed within statutory period (Section 167(2))
    /// - 60 days for offences punishable up to 10 years
    /// - 90 days for offences punishable with death/life/10+ years
    pub fn check_statutory_period(&self, max_punishment_years: u32) -> bool {
        let days = (self.filing_date - self.fir_date).num_days();

        if max_punishment_years >= 10 {
            days <= 90
        } else {
            days <= 60
        }
    }
}

/// Trial proceedings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrialProceedings {
    /// Case number
    pub case_number: String,
    /// Court
    pub court: String,
    /// Accused
    pub accused: Vec<String>,
    /// Offence sections
    pub sections: Vec<u32>,
    /// Trial stage
    pub stage: TrialStage,
    /// Cognizance date
    pub cognizance_date: NaiveDate,
    /// Charges framed date
    pub charges_framed_date: Option<NaiveDate>,
    /// Judgment date
    pub judgment_date: Option<NaiveDate>,
    /// Verdict
    pub verdict: Option<Verdict>,
}

/// Trial verdict
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Verdict {
    /// Convicted
    Convicted,
    /// Acquitted
    Acquitted,
    /// Discharged
    Discharged,
    /// Compounded (Section 320)
    Compounded,
}

/// Appeal under CrPC/BNSS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppealType {
    /// Appeal to Sessions Court (Section 374)
    ToSessionsCourt,
    /// Appeal to High Court (Section 374)
    ToHighCourt,
    /// Revision to High Court (Section 397)
    Revision,
    /// Special Leave Petition to Supreme Court (Article 136)
    Slp,
}

/// Criminal appeal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriminalAppeal {
    /// Appeal number
    pub appeal_number: String,
    /// Appeal type
    pub appeal_type: AppealType,
    /// Lower court case number
    pub lower_case_number: String,
    /// Appellant (convict or state)
    pub appellant: String,
    /// Judgment date
    pub judgment_date: NaiveDate,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Is within limitation (30/60/90 days)
    pub within_limitation: bool,
    /// Sentence
    pub sentence: Option<String>,
    /// Status
    pub status: String,
}

/// Compoundable offence (Section 320)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompoundableOffence {
    /// Offence section
    pub section: u32,
    /// Is compoundable
    pub is_compoundable: bool,
    /// Requires court permission
    pub requires_court_permission: bool,
    /// Description
    pub description: String,
}

impl CompoundableOffence {
    /// Check if offence is compoundable
    pub fn is_compoundable_offence(section: u32) -> Option<Self> {
        // Examples from Section 320 CrPC
        match section {
            323 => Some(Self {
                section,
                is_compoundable: true,
                requires_court_permission: false,
                description: "Voluntarily causing hurt".to_string(),
            }),
            324 => Some(Self {
                section,
                is_compoundable: true,
                requires_court_permission: true,
                description: "Voluntarily causing hurt by dangerous weapons".to_string(),
            }),
            379 => Some(Self {
                section,
                is_compoundable: true,
                requires_court_permission: false,
                description: "Theft".to_string(),
            }),
            _ => None,
        }
    }
}

/// Time limits for investigation/trial
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeLimits {
    /// Chargesheet filing (60/90 days)
    pub chargesheet_days: u32,
    /// Trial completion (varies)
    pub trial_completion_days: Option<u32>,
    /// Appeal limitation (30/60/90 days)
    pub appeal_days: u32,
}
