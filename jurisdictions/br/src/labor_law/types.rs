//! CLT Labor Law Types

use crate::citation::format_clt_citation;
use crate::common::BrazilianState;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Employment contract type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentType {
    /// Standard CLT employment (tempo indeterminado)
    Standard,
    /// Fixed-term contract (tempo determinado - Art. 443)
    FixedTerm,
    /// Temporary work (Lei 6.019/74)
    Temporary,
    /// Intermittent work (Art. 443, §3 - post 2017 reform)
    Intermittent,
    /// Part-time work (Art. 58-A)
    PartTime,
    /// Apprentice (Lei 10.097/2000)
    Apprentice,
    /// Domestic worker (LC 150/2015)
    Domestic,
}

impl EmploymentType {
    /// Get maximum duration in months (if applicable)
    pub fn max_duration_months(&self) -> Option<u32> {
        match self {
            Self::FixedTerm => Some(24),  // 2 years
            Self::Temporary => Some(6),   // 180 days (+90 extension)
            Self::Apprentice => Some(24), // 2 years
            _ => None,
        }
    }

    /// Get legal citation
    pub fn citation(&self) -> String {
        match self {
            Self::Standard => format_clt_citation(442, None, None),
            Self::FixedTerm => format_clt_citation(443, None, None),
            Self::Intermittent => format_clt_citation(443, Some(3), None),
            Self::PartTime => format_clt_citation(58, None, None),
            _ => "CLT e legislação específica".to_string(),
        }
    }

    /// Check if FGTS applies
    pub fn has_fgts(&self) -> bool {
        !matches!(self, Self::Apprentice)
    }

    /// Get name in Portuguese
    pub fn nome_pt(&self) -> &'static str {
        match self {
            Self::Standard => "Contrato por tempo indeterminado",
            Self::FixedTerm => "Contrato por tempo determinado",
            Self::Temporary => "Trabalho temporário",
            Self::Intermittent => "Trabalho intermitente",
            Self::PartTime => "Trabalho parcial",
            Self::Apprentice => "Aprendiz",
            Self::Domestic => "Empregado doméstico",
        }
    }
}

/// Working hours configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Daily hours (max 8 for standard)
    pub daily_hours: u32,
    /// Weekly hours (max 44 for standard)
    pub weekly_hours: u32,
    /// Has compensatory time bank (banco de horas)
    pub time_bank: bool,
    /// Work schedule type
    pub schedule_type: ScheduleType,
}

impl WorkingHours {
    /// Standard 44-hour week
    pub fn standard() -> Self {
        Self {
            daily_hours: 8,
            weekly_hours: 44,
            time_bank: false,
            schedule_type: ScheduleType::Standard,
        }
    }

    /// Part-time (max 30 hours)
    pub fn part_time(hours: u32) -> Self {
        Self {
            daily_hours: hours.min(6),
            weekly_hours: hours.min(30),
            time_bank: false,
            schedule_type: ScheduleType::PartTime,
        }
    }

    /// Check if hours exceed legal maximum
    pub fn exceeds_legal_limit(&self) -> bool {
        self.daily_hours > 10 || self.weekly_hours > 44
    }

    /// Calculate weekly overtime hours
    pub fn weekly_overtime(&self) -> u32 {
        self.weekly_hours.saturating_sub(44)
    }
}

/// Work schedule type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleType {
    /// Standard 8h/day, 5-6 days
    Standard,
    /// Part-time (max 30h/week)
    PartTime,
    /// 12x36 shift (12h work, 36h rest)
    Shift12x36,
    /// Night shift (10pm-5am)
    Night,
    /// Flexible hours
    Flexible,
}

impl ScheduleType {
    /// Get night shift bonus percentage
    pub fn night_bonus_percent(&self) -> u32 {
        match self {
            Self::Night => 20, // Art. 73: minimum 20%
            _ => 0,
        }
    }
}

/// Termination type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationType {
    /// Termination without cause (sem justa causa)
    WithoutCause,
    /// Termination for cause (por justa causa - Art. 482)
    ForCause,
    /// Resignation (pedido de demissão)
    Resignation,
    /// Mutual agreement (acordo - Art. 484-A)
    MutualAgreement,
    /// Contract end (for fixed-term)
    ContractEnd,
    /// Employer fault (rescisão indireta - Art. 483)
    IndirectTermination,
    /// Force majeure
    ForceMajeure,
}

impl TerminationType {
    /// Get FGTS penalty percentage
    pub fn fgts_penalty_percent(&self) -> u32 {
        match self {
            Self::WithoutCause | Self::IndirectTermination => 40,
            Self::MutualAgreement => 20,
            _ => 0,
        }
    }

    /// Check if unemployment insurance applies
    pub fn unemployment_insurance(&self) -> bool {
        matches!(self, Self::WithoutCause | Self::IndirectTermination)
    }

    /// Check if notice period required
    pub fn requires_notice(&self) -> bool {
        matches!(
            self,
            Self::WithoutCause | Self::Resignation | Self::MutualAgreement
        )
    }

    /// Get citation
    pub fn citation(&self) -> String {
        match self {
            Self::ForCause => format_clt_citation(482, None, None),
            Self::IndirectTermination => format_clt_citation(483, None, None),
            Self::MutualAgreement => format_clt_citation(484, None, None),
            _ => format_clt_citation(477, None, None),
        }
    }

    /// Get name in Portuguese
    pub fn nome_pt(&self) -> &'static str {
        match self {
            Self::WithoutCause => "Dispensa sem justa causa",
            Self::ForCause => "Dispensa por justa causa",
            Self::Resignation => "Pedido de demissão",
            Self::MutualAgreement => "Acordo entre as partes",
            Self::ContractEnd => "Término do contrato",
            Self::IndirectTermination => "Rescisão indireta",
            Self::ForceMajeure => "Força maior",
        }
    }
}

/// Just cause grounds (Art. 482)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustCauseGround {
    /// Dishonesty (ato de improbidade)
    Dishonesty,
    /// Insubordination
    Insubordination,
    /// Bad conduct (incontinência de conduta)
    BadConduct,
    /// Negligence (desídia)
    Negligence,
    /// Habitual intoxication
    HabitualIntoxication,
    /// Company secret disclosure
    SecretDisclosure,
    /// Indiscipline
    Indiscipline,
    /// Job abandonment (abandono de emprego - 30+ days)
    JobAbandonment,
    /// Physical aggression
    PhysicalAggression,
    /// Criminal conviction
    CriminalConviction,
}

impl JustCauseGround {
    /// Get legal citation
    pub fn citation(&self) -> String {
        let alinea = match self {
            Self::Dishonesty => 'a',
            Self::BadConduct => 'b',
            Self::Negligence => 'e',
            Self::HabitualIntoxication => 'f',
            Self::SecretDisclosure => 'g',
            Self::Indiscipline => 'h',
            Self::Insubordination => 'h',
            Self::JobAbandonment => 'i',
            Self::PhysicalAggression => 'j',
            Self::CriminalConviction => 'd',
        };
        format!("CLT, Art. 482, alínea {}", alinea)
    }
}

/// Employment contract
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Contract type
    pub employment_type: EmploymentType,
    /// Start date
    pub data_inicio: NaiveDate,
    /// End date (for fixed-term)
    pub data_fim: Option<NaiveDate>,
    /// Monthly salary in centavos
    pub salario_centavos: i64,
    /// Job title/function
    pub funcao: String,
    /// Work location state
    pub estado: BrazilianState,
    /// Working hours
    pub jornada: WorkingHours,
    /// CTPS registered
    pub ctps_registrada: bool,
}

impl EmploymentContract {
    /// Calculate months worked
    pub fn months_worked(&self, reference_date: NaiveDate) -> u32 {
        let end = self.data_fim.unwrap_or(reference_date);
        let months = (end.year() - self.data_inicio.year()) * 12
            + (end.month() as i32 - self.data_inicio.month() as i32);
        months.max(0) as u32
    }

    /// Calculate notice period in days (Art. 487)
    pub fn notice_period_days(&self, reference_date: NaiveDate) -> u32 {
        let years = self.months_worked(reference_date) / 12;
        30 + (years * 3).min(60) // 30 base + 3 days per year, max 90 days
    }

    /// Calculate annual vacation days (Art. 130)
    pub fn vacation_days(&self, reference_date: NaiveDate) -> u32 {
        let months = self.months_worked(reference_date);
        if months >= 12 {
            30 // Full vacation after 12 months
        } else {
            (months * 30) / 12 // Proportional
        }
    }
}

/// Severance calculation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Severance {
    /// Salary balance (saldo de salário)
    pub saldo_salario_centavos: i64,
    /// Notice period payment
    pub aviso_previo_centavos: i64,
    /// Proportional 13th salary
    pub decimo_terceiro_centavos: i64,
    /// Vacation + 1/3
    pub ferias_centavos: i64,
    /// FGTS balance
    pub fgts_saldo_centavos: i64,
    /// FGTS penalty (40% or 20%)
    pub fgts_multa_centavos: i64,
    /// Total severance
    pub total_centavos: i64,
    /// Termination type
    pub termination_type: TerminationType,
}

impl Severance {
    /// Calculate severance for termination
    pub fn calculate(
        monthly_salary_centavos: i64,
        months_worked: u32,
        termination_type: TerminationType,
        worked_days_current_month: u32,
        months_since_last_13th: u32,
        vacation_days_pending: u32,
    ) -> Self {
        // Salary balance (days worked in current month)
        let saldo_salario = (monthly_salary_centavos * worked_days_current_month as i64) / 30;

        // Notice period (if applicable)
        let notice_days = 30 + ((months_worked / 12) * 3).min(60);
        let aviso_previo = if termination_type.requires_notice() {
            match termination_type {
                TerminationType::MutualAgreement => {
                    (monthly_salary_centavos * notice_days as i64) / 60 // 50%
                }
                _ => (monthly_salary_centavos * notice_days as i64) / 30,
            }
        } else {
            0
        };

        // Proportional 13th salary
        let decimo_terceiro = (monthly_salary_centavos * months_since_last_13th as i64) / 12;

        // Vacation + 1/3 bonus
        let ferias_base = (monthly_salary_centavos * vacation_days_pending as i64) / 30;
        let ferias = ferias_base + (ferias_base / 3); // +1/3 constitutional bonus

        // FGTS (8% per month)
        let fgts_saldo = (monthly_salary_centavos * 8 * months_worked as i64) / 100;

        // FGTS penalty
        let fgts_multa = (fgts_saldo * termination_type.fgts_penalty_percent() as i64) / 100;

        let total = saldo_salario + aviso_previo + decimo_terceiro + ferias + fgts_multa;

        Self {
            saldo_salario_centavos: saldo_salario,
            aviso_previo_centavos: aviso_previo,
            decimo_terceiro_centavos: decimo_terceiro,
            ferias_centavos: ferias,
            fgts_saldo_centavos: fgts_saldo,
            fgts_multa_centavos: fgts_multa,
            total_centavos: total,
            termination_type,
        }
    }
}

/// Labor right type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaborRight {
    /// Minimum wage (salário mínimo)
    MinimumWage,
    /// FGTS (8% monthly)
    Fgts,
    /// 13th salary
    ThirteenthSalary,
    /// Paid vacation (30 days + 1/3)
    PaidVacation,
    /// Overtime pay (+50%/+100%)
    OvertimePay,
    /// Rest period (11h between shifts)
    RestPeriod,
    /// Weekly rest (preferably Sunday)
    WeeklyRest,
    /// Maternity leave (120 days)
    MaternityLeave,
    /// Paternity leave (5 days)
    PaternityLeave,
    /// Sick leave (first 15 days employer)
    SickLeave,
    /// Transportation voucher (vale-transporte)
    TransportationVoucher,
}

impl LaborRight {
    /// Get legal citation
    pub fn citation(&self) -> String {
        match self {
            Self::MinimumWage => "CF, Art. 7, IV".to_string(),
            Self::Fgts => "Lei 8.036/90".to_string(),
            Self::ThirteenthSalary => "CF, Art. 7, VIII".to_string(),
            Self::PaidVacation => format_clt_citation(129, None, None),
            Self::OvertimePay => format_clt_citation(59, None, None),
            Self::RestPeriod => format_clt_citation(66, None, None),
            Self::WeeklyRest => format_clt_citation(67, None, None),
            Self::MaternityLeave => format_clt_citation(392, None, None),
            Self::PaternityLeave => "CF, Art. 7, XIX".to_string(),
            Self::SickLeave => format_clt_citation(476, None, None),
            Self::TransportationVoucher => "Lei 7.418/85".to_string(),
        }
    }

    /// Get description in Portuguese
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::MinimumWage => "Salário mínimo",
            Self::Fgts => "FGTS (8% mensal)",
            Self::ThirteenthSalary => "13º salário",
            Self::PaidVacation => "Férias (30 dias + 1/3)",
            Self::OvertimePay => "Hora extra (+50%/+100%)",
            Self::RestPeriod => "Intervalo interjornada (11h)",
            Self::WeeklyRest => "Repouso semanal remunerado",
            Self::MaternityLeave => "Licença-maternidade (120 dias)",
            Self::PaternityLeave => "Licença-paternidade (5 dias)",
            Self::SickLeave => "Auxílio-doença",
            Self::TransportationVoucher => "Vale-transporte",
        }
    }
}

/// Labor compliance status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaborCompliance {
    /// Overall compliance
    pub compliant: bool,
    /// CTPS registered
    pub ctps_registered: bool,
    /// Working hours compliant
    pub hours_compliant: bool,
    /// Minimum wage compliant
    pub wage_compliant: bool,
    /// FGTS deposited
    pub fgts_compliant: bool,
    /// Vacation rights respected
    pub vacation_compliant: bool,
    /// Issues found
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_type_fgts() {
        assert!(EmploymentType::Standard.has_fgts());
        assert!(!EmploymentType::Apprentice.has_fgts());
    }

    #[test]
    fn test_termination_fgts_penalty() {
        assert_eq!(TerminationType::WithoutCause.fgts_penalty_percent(), 40);
        assert_eq!(TerminationType::MutualAgreement.fgts_penalty_percent(), 20);
        assert_eq!(TerminationType::ForCause.fgts_penalty_percent(), 0);
    }

    #[test]
    fn test_working_hours_overtime() {
        let hours = WorkingHours {
            daily_hours: 8,
            weekly_hours: 50,
            time_bank: false,
            schedule_type: ScheduleType::Standard,
        };
        assert_eq!(hours.weekly_overtime(), 6);
    }

    #[test]
    fn test_working_hours_standard() {
        let hours = WorkingHours::standard();
        assert_eq!(hours.weekly_hours, 44);
        assert_eq!(hours.weekly_overtime(), 0);
    }

    #[test]
    fn test_severance_calculation() {
        let severance = Severance::calculate(
            500000, // R$ 5,000 monthly
            36,     // 3 years
            TerminationType::WithoutCause,
            15, // 15 days worked
            6,  // 6 months since last 13th
            30, // 30 vacation days pending
        );

        assert!(severance.total_centavos > 0);
        assert!(severance.fgts_multa_centavos > 0); // 40% penalty
    }

    #[test]
    fn test_severance_resignation() {
        let severance = Severance::calculate(500000, 12, TerminationType::Resignation, 10, 3, 0);

        assert_eq!(severance.fgts_multa_centavos, 0); // No penalty for resignation
    }

    #[test]
    fn test_employment_contract_notice() {
        let contract = EmploymentContract {
            employment_type: EmploymentType::Standard,
            data_inicio: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            data_fim: None,
            salario_centavos: 500000,
            funcao: "Desenvolvedor".to_string(),
            estado: BrazilianState::SP,
            jornada: WorkingHours::standard(),
            ctps_registrada: true,
        };

        let reference = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let notice = contract.notice_period_days(reference);
        assert!(notice >= 30); // At least 30 days
        assert!(notice <= 90); // Maximum 90 days
    }

    #[test]
    fn test_just_cause_citation() {
        let ground = JustCauseGround::JobAbandonment;
        let citation = ground.citation();
        assert!(citation.contains("482"));
    }

    #[test]
    fn test_labor_right_citation() {
        let right = LaborRight::PaidVacation;
        let citation = right.citation();
        assert!(citation.contains("129"));
    }
}
