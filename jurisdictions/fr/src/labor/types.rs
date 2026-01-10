//! Labor law types (Types de droit du travail)
//!
//! This module provides type definitions for French labor law under the Code du travail.

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Employment contract type (Type de contrat de travail)
///
/// The main types of employment contracts in French law.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EmploymentContractType {
    /// CDI - Contrat à Durée Indéterminée (Permanent contract)
    ///
    /// The default and preferred type (Article L1221-2).
    /// No fixed end date, most protective for employees.
    CDI,

    /// CDD - Contrat à Durée Déterminée (Fixed-term contract)
    ///
    /// Only allowed in specific cases (Article L1242-2).
    /// Maximum 18 months, renewable once.
    CDD {
        /// Duration in months (max 18)
        duration_months: u8,
        /// Reason for using CDD
        reason: CDDReason,
        /// End date
        end_date: NaiveDate,
    },

    /// Interim (Temporary agency work / Intérim)
    ///
    /// Work through a temporary employment agency.
    Interim {
        /// Mission duration in days
        mission_duration_days: u16,
        /// Agency name
        agency: String,
        /// User company name
        user_company: String,
    },

    /// Apprenticeship (Contrat d'apprentissage)
    ///
    /// Training contract for young workers.
    Apprenticeship {
        /// Duration in months (6-36 months)
        duration_months: u8,
        /// Training center
        training_center: String,
    },
}

impl EmploymentContractType {
    /// Check if contract is CDI
    #[must_use]
    pub fn is_cdi(&self) -> bool {
        matches!(self, Self::CDI)
    }

    /// Check if contract is CDD
    #[must_use]
    pub fn is_cdd(&self) -> bool {
        matches!(self, Self::CDD { .. })
    }

    /// Get contract type name
    #[must_use]
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::CDI => "CDI (Permanent)",
            Self::CDD { .. } => "CDD (Fixed-term)",
            Self::Interim { .. } => "Interim (Temporary)",
            Self::Apprenticeship { .. } => "Apprenticeship",
        }
    }
}

/// Valid reasons for CDD (Cas de recours au CDD)
///
/// Article L1242-2 lists authorized cases for fixed-term contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CDDReason {
    /// Replacement of absent employee (Remplacement d'un salarié absent)
    ReplacementAbsentEmployee,

    /// Temporary increase in activity (Accroissement temporaire d'activité)
    TemporaryIncreaseActivity,

    /// Seasonal work (Emploi à caractère saisonnier)
    SeasonalWork,

    /// Specific project (Usage défini par accord)
    SpecificProject,

    /// Replacement pending recruitment (Remplacement dans l'attente d'une embauche)
    PendingRecruitment,
}

impl CDDReason {
    /// Get French description
    #[must_use]
    pub fn french_description(self) -> &'static str {
        match self {
            Self::ReplacementAbsentEmployee => "Remplacement d'un salarié absent",
            Self::TemporaryIncreaseActivity => "Accroissement temporaire d'activité",
            Self::SeasonalWork => "Emploi à caractère saisonnier",
            Self::SpecificProject => "Usage défini par accord",
            Self::PendingRecruitment => "Remplacement dans l'attente d'une embauche",
        }
    }
}

/// Trial period duration (Durée de la période d'essai)
///
/// Article L1221-19 sets maximum trial periods by position category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TrialPeriodCategory {
    /// Workers and employees (Ouvriers et employés)
    WorkersEmployees,

    /// Supervisors and technicians (Agents de maîtrise et techniciens)
    SupervisorsTechnicians,

    /// Executives (Cadres)
    Executives,
}

impl TrialPeriodCategory {
    /// Get maximum trial period in months (Article L1221-19)
    #[must_use]
    pub fn max_trial_period_months(self) -> u8 {
        match self {
            Self::WorkersEmployees => 2,       // 2 months
            Self::SupervisorsTechnicians => 3, // 3 months
            Self::Executives => 4,             // 4 months
        }
    }

    /// Get French name
    #[must_use]
    pub fn french_name(self) -> &'static str {
        match self {
            Self::WorkersEmployees => "Ouvriers et employés",
            Self::SupervisorsTechnicians => "Agents de maîtrise et techniciens",
            Self::Executives => "Cadres",
        }
    }
}

/// Working hours (Durée du travail)
///
/// French labor law is built around the 35-hour work week (Article L3121-27).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorkingHours {
    /// Weekly hours worked
    pub weekly_hours: f32,

    /// Daily hours worked (for max daily limit check)
    pub daily_hours: Option<f32>,
}

impl WorkingHours {
    /// Legal weekly working time (Article L3121-27)
    pub const LEGAL_WEEKLY_HOURS: f32 = 35.0;

    /// Maximum daily hours (Article L3121-18)
    pub const MAX_DAILY_HOURS: f32 = 10.0;

    /// Absolute maximum weekly hours (Article L3121-20)
    pub const MAX_WEEKLY_HOURS: f32 = 48.0;

    /// Maximum average over 12 weeks (Article L3121-22)
    pub const MAX_AVERAGE_WEEKLY_HOURS: f32 = 44.0;

    /// Create new working hours
    #[must_use]
    pub fn new(weekly_hours: f32) -> Self {
        Self {
            weekly_hours,
            daily_hours: None,
        }
    }

    /// Set daily hours
    #[must_use]
    pub fn with_daily_hours(mut self, hours: f32) -> Self {
        self.daily_hours = Some(hours);
        self
    }

    /// Calculate overtime hours beyond 35 hours
    #[must_use]
    pub fn overtime_hours(&self) -> f32 {
        (self.weekly_hours - Self::LEGAL_WEEKLY_HOURS).max(0.0)
    }

    /// Check if within legal limits
    #[must_use]
    pub fn is_legal(&self) -> bool {
        self.weekly_hours <= Self::MAX_WEEKLY_HOURS
            && self.daily_hours.is_none_or(|h| h <= Self::MAX_DAILY_HOURS)
    }

    /// Calculate overtime premium rate (Article L3121-33)
    ///
    /// - First 8 hours: +25%
    /// - Beyond 8 hours: +50%
    #[must_use]
    pub fn calculate_overtime_premium(overtime_hours: f32, base_hourly_rate: f32) -> f32 {
        let first_8 = overtime_hours.min(8.0);
        let beyond_8 = (overtime_hours - 8.0).max(0.0);

        first_8 * base_hourly_rate * 0.25  // 25% premium
            + beyond_8 * base_hourly_rate * 0.50 // 50% premium
    }
}

/// Dismissal type (Type de licenciement)
///
/// French law strictly regulates dismissals.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DismissalType {
    /// Personal dismissal (Licenciement pour motif personnel)
    ///
    /// Based on employee's personal conduct or performance.
    Personal {
        /// Specific cause
        cause: PersonalCause,
        /// Whether it's serious misconduct (no notice period)
        serious_misconduct: bool,
    },

    /// Economic dismissal (Licenciement économique)
    ///
    /// Based on economic reasons, not employee's fault.
    Economic {
        /// Economic difficulties
        economic_difficulties: bool,
        /// Job position eliminated
        job_eliminated: bool,
        /// Number of employees affected
        affected_count: u32,
    },
}

impl DismissalType {
    /// Check if dismissal requires notice period
    #[must_use]
    pub fn requires_notice(&self) -> bool {
        match self {
            Self::Personal {
                serious_misconduct, ..
            } => !serious_misconduct,
            Self::Economic { .. } => true,
        }
    }
}

/// Personal cause for dismissal (Cause personnelle)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PersonalCause {
    /// Simple fault (Faute simple)
    SimpleFault,

    /// Serious misconduct (Faute grave) - no notice, no severance
    SeriousMisconduct,

    /// Gross misconduct (Faute lourde) - intent to harm
    GrossMisconduct,

    /// Professional incompetence (Insuffisance professionnelle)
    Incompetence,

    /// Other real and serious cause (Autre cause réelle et sérieuse)
    OtherRealSerious,
}

impl PersonalCause {
    /// Check if cause allows severance pay
    #[must_use]
    pub fn allows_severance(self) -> bool {
        !matches!(self, Self::SeriousMisconduct | Self::GrossMisconduct)
    }

    /// Get French name
    #[must_use]
    pub fn french_name(self) -> &'static str {
        match self {
            Self::SimpleFault => "Faute simple",
            Self::SeriousMisconduct => "Faute grave",
            Self::GrossMisconduct => "Faute lourde",
            Self::Incompetence => "Insuffisance professionnelle",
            Self::OtherRealSerious => "Autre cause réelle et sérieuse",
        }
    }
}

/// Notice period (Préavis)
///
/// Duration of notice period before dismissal takes effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NoticePeriod {
    /// Duration in months
    pub months: u8,
}

impl NoticePeriod {
    /// Standard notice for workers/employees: 1-2 months
    pub const WORKERS_EMPLOYEES_MONTHS: u8 = 1;

    /// Standard notice for supervisors: 2-3 months
    pub const SUPERVISORS_MONTHS: u8 = 2;

    /// Standard notice for executives: 3 months
    pub const EXECUTIVES_MONTHS: u8 = 3;

    /// Create new notice period
    #[must_use]
    pub const fn new(months: u8) -> Self {
        Self { months }
    }

    /// Get standard notice for category
    #[must_use]
    pub fn for_category(category: TrialPeriodCategory) -> Self {
        let months = match category {
            TrialPeriodCategory::WorkersEmployees => Self::WORKERS_EMPLOYEES_MONTHS,
            TrialPeriodCategory::SupervisorsTechnicians => Self::SUPERVISORS_MONTHS,
            TrialPeriodCategory::Executives => Self::EXECUTIVES_MONTHS,
        };
        Self::new(months)
    }
}

/// Employment contract (Contrat de travail)
///
/// Builder pattern for creating employment contracts.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmploymentContract {
    /// Contract type
    pub contract_type: EmploymentContractType,

    /// Employee name
    pub employee: String,

    /// Employer name
    pub employer: String,

    /// Position/job title
    pub position: String,

    /// Position category (for trial period calculation)
    pub category: TrialPeriodCategory,

    /// Base hourly rate in euros
    pub hourly_rate: f32,

    /// Weekly working hours
    pub working_hours: WorkingHours,

    /// Trial period in months (optional)
    pub trial_period_months: Option<u8>,

    /// Start date
    pub start_date: NaiveDate,

    /// Written contract exists (required for CDD)
    pub written: bool,
}

impl EmploymentContract {
    /// Create new employment contract
    #[must_use]
    pub fn new(contract_type: EmploymentContractType, employee: String, employer: String) -> Self {
        Self {
            contract_type,
            employee,
            employer,
            position: String::new(),
            category: TrialPeriodCategory::WorkersEmployees,
            hourly_rate: 11.65, // SMIC 2024
            working_hours: WorkingHours::new(35.0),
            trial_period_months: None,
            start_date: chrono::Utc::now().naive_utc().date(),
            written: false,
        }
    }

    /// Set position
    #[must_use]
    pub fn with_position(mut self, position: String) -> Self {
        self.position = position;
        self
    }

    /// Set category
    #[must_use]
    pub fn with_category(mut self, category: TrialPeriodCategory) -> Self {
        self.category = category;
        self
    }

    /// Set hourly rate
    #[must_use]
    pub fn with_hourly_rate(mut self, rate: f32) -> Self {
        self.hourly_rate = rate;
        self
    }

    /// Set working hours
    #[must_use]
    pub fn with_working_hours(mut self, hours: WorkingHours) -> Self {
        self.working_hours = hours;
        self
    }

    /// Set trial period
    #[must_use]
    pub fn with_trial_period(mut self, months: u8) -> Self {
        self.trial_period_months = Some(months);
        self
    }

    /// Set start date
    #[must_use]
    pub fn with_start_date(mut self, date: NaiveDate) -> Self {
        self.start_date = date;
        self
    }

    /// Mark as written
    #[must_use]
    pub fn with_written(mut self, written: bool) -> Self {
        self.written = written;
        self
    }

    /// Calculate monthly gross salary
    #[must_use]
    pub fn monthly_gross_salary(&self) -> f32 {
        self.hourly_rate * self.working_hours.weekly_hours * 52.0 / 12.0
    }

    /// Calculate overtime premium
    #[must_use]
    pub fn monthly_overtime_premium(&self) -> f32 {
        let overtime = self.working_hours.overtime_hours();
        if overtime > 0.0 {
            WorkingHours::calculate_overtime_premium(overtime, self.hourly_rate) * 52.0 / 12.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_contract_types() {
        let cdi = EmploymentContractType::CDI;
        assert!(cdi.is_cdi());
        assert!(!cdi.is_cdd());
        assert_eq!(cdi.type_name(), "CDI (Permanent)");
    }

    #[test]
    fn test_trial_period_max() {
        assert_eq!(
            TrialPeriodCategory::WorkersEmployees.max_trial_period_months(),
            2
        );
        assert_eq!(
            TrialPeriodCategory::SupervisorsTechnicians.max_trial_period_months(),
            3
        );
        assert_eq!(TrialPeriodCategory::Executives.max_trial_period_months(), 4);
    }

    #[test]
    fn test_working_hours_overtime() {
        let hours = WorkingHours::new(40.0);
        assert_eq!(hours.overtime_hours(), 5.0);

        let hours_no_overtime = WorkingHours::new(30.0);
        assert_eq!(hours_no_overtime.overtime_hours(), 0.0);
    }

    #[test]
    fn test_working_hours_legal() {
        let legal = WorkingHours::new(40.0);
        assert!(legal.is_legal());

        let illegal = WorkingHours::new(50.0);
        assert!(!illegal.is_legal());

        let illegal_daily = WorkingHours::new(35.0).with_daily_hours(11.0);
        assert!(!illegal_daily.is_legal());
    }

    #[test]
    fn test_overtime_premium_calculation() {
        // 5 hours overtime: all at +25%
        let premium = WorkingHours::calculate_overtime_premium(5.0, 15.0);
        assert_eq!(premium, 5.0 * 15.0 * 0.25);

        // 10 hours overtime: 8 at +25%, 2 at +50%
        let premium = WorkingHours::calculate_overtime_premium(10.0, 15.0);
        assert_eq!(premium, 8.0 * 15.0 * 0.25 + 2.0 * 15.0 * 0.50);
    }

    #[test]
    fn test_dismissal_notice_requirement() {
        let simple_fault = DismissalType::Personal {
            cause: PersonalCause::SimpleFault,
            serious_misconduct: false,
        };
        assert!(simple_fault.requires_notice());

        let serious = DismissalType::Personal {
            cause: PersonalCause::SeriousMisconduct,
            serious_misconduct: true,
        };
        assert!(!serious.requires_notice());
    }

    #[test]
    fn test_personal_cause_severance() {
        assert!(PersonalCause::SimpleFault.allows_severance());
        assert!(PersonalCause::Incompetence.allows_severance());
        assert!(!PersonalCause::SeriousMisconduct.allows_severance());
        assert!(!PersonalCause::GrossMisconduct.allows_severance());
    }

    #[test]
    fn test_notice_period_for_category() {
        let workers = NoticePeriod::for_category(TrialPeriodCategory::WorkersEmployees);
        assert_eq!(workers.months, 1);

        let executives = NoticePeriod::for_category(TrialPeriodCategory::Executives);
        assert_eq!(executives.months, 3);
    }

    #[test]
    fn test_employment_contract_builder() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Marie Dupont".to_string(),
            "TechCorp SA".to_string(),
        )
        .with_position("Software Engineer".to_string())
        .with_category(TrialPeriodCategory::Executives)
        .with_hourly_rate(30.0)
        .with_working_hours(WorkingHours::new(39.0))
        .with_trial_period(4)
        .with_written(true);

        assert_eq!(contract.employee, "Marie Dupont");
        assert_eq!(contract.hourly_rate, 30.0);
        assert_eq!(contract.working_hours.weekly_hours, 39.0);
        assert_eq!(contract.trial_period_months, Some(4));
    }

    #[test]
    fn test_monthly_salary_calculation() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Employee".to_string(),
            "Employer".to_string(),
        )
        .with_hourly_rate(15.0)
        .with_working_hours(WorkingHours::new(35.0));

        // 15 € * 35h * 52 weeks / 12 months = 2,275 €
        let salary = contract.monthly_gross_salary();
        assert!((salary - 2275.0).abs() < 0.01);
    }

    #[test]
    fn test_overtime_premium_monthly() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Employee".to_string(),
            "Employer".to_string(),
        )
        .with_hourly_rate(15.0)
        .with_working_hours(WorkingHours::new(40.0)); // 5 hours overtime

        let premium = contract.monthly_overtime_premium();
        // 5h * 15€ * 25% * 52 weeks / 12 months
        let expected = 5.0 * 15.0 * 0.25 * 52.0 / 12.0;
        assert!((premium - expected).abs() < 0.01);
    }
}
