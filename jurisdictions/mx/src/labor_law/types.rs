//! Labor law types

use crate::common::MexicanCurrency;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Employment contract (Contrato de trabajo)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Employee name
    pub trabajador: String,
    /// Employer name
    pub patron: String,
    /// Employment type
    pub tipo: EmploymentType,
    /// Salary
    pub salario: MexicanCurrency,
    /// Work schedule
    pub jornada: WorkSchedule,
    /// Start date
    pub fecha_inicio: DateTime<Utc>,
    /// End date (if applicable)
    pub fecha_fin: Option<DateTime<Utc>>,
}

/// Employment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentType {
    /// Indefinite term (Por tiempo indeterminado)
    Indefinite,
    /// Fixed term (Por tiempo determinado)
    FixedTerm,
    /// Seasonal (Por temporada)
    Seasonal,
    /// Trial period (A prueba)
    Trial,
    /// Training (Capacitación inicial)
    Training,
}

/// Work schedule (Jornada de trabajo)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkSchedule {
    /// Type of work day
    pub tipo_jornada: WorkDayType,
    /// Hours per day
    pub horas_diarias: u8,
    /// Days per week
    pub dias_semana: u8,
}

/// Work day type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkDayType {
    /// Day shift (Diurna) - 6am to 8pm
    Day,
    /// Night shift (Nocturna) - 8pm to 6am
    Night,
    /// Mixed shift (Mixta)
    Mixed,
}

/// Labor right
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaborRight {
    /// Maximum working hours
    MaximumWorkingHours,
    /// Weekly rest day
    WeeklyRest,
    /// Paid vacation
    PaidVacation,
    /// Christmas bonus (Aguinaldo)
    ChristmasBonus,
    /// Profit sharing (PTU)
    ProfitSharing,
    /// Social security
    SocialSecurity,
}

impl EmploymentContract {
    /// Create new employment contract
    pub fn new(
        trabajador: String,
        patron: String,
        tipo: EmploymentType,
        salario: MexicanCurrency,
        jornada: WorkSchedule,
        fecha_inicio: DateTime<Utc>,
    ) -> Self {
        Self {
            trabajador,
            patron,
            tipo,
            salario,
            jornada,
            fecha_inicio,
            fecha_fin: None,
        }
    }

    /// Check if contract is indefinite
    pub fn is_indefinite(&self) -> bool {
        matches!(self.tipo, EmploymentType::Indefinite)
    }
}

impl WorkSchedule {
    /// Create standard day schedule
    pub fn standard_day() -> Self {
        Self {
            tipo_jornada: WorkDayType::Day,
            horas_diarias: 8,
            dias_semana: 5,
        }
    }

    /// Calculate weekly hours
    pub fn weekly_hours(&self) -> u8 {
        self.horas_diarias * self.dias_semana
    }

    /// Check if schedule exceeds legal maximum
    pub fn exceeds_legal_maximum(&self) -> bool {
        match self.tipo_jornada {
            WorkDayType::Day => self.horas_diarias > 8 || self.weekly_hours() > 48,
            WorkDayType::Night => self.horas_diarias > 7 || self.weekly_hours() > 42,
            WorkDayType::Mixed => self.horas_diarias > 7 || self.weekly_hours() > 45,
        }
    }
}
