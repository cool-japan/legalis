//! CLT Error Types

use crate::citation::format_clt_citation;
use thiserror::Error;

/// CLT Error types
#[derive(Debug, Clone, Error)]
pub enum CltError {
    /// Working hours violation (Art. 58-59)
    #[error("Jornada de trabalho excedida (Art. 58): {description}")]
    WorkingHoursViolation {
        /// Description
        description: String,
        /// Actual hours
        actual_hours: u32,
        /// Maximum allowed
        max_hours: u32,
    },

    /// Overtime not paid (Art. 59)
    #[error("Hora extra não paga (Art. 59): {hours} horas")]
    OvertimeNotPaid {
        /// Hours not paid
        hours: u32,
    },

    /// Minimum wage violation (CF Art. 7, IV)
    #[error("Salário abaixo do mínimo: R$ {actual}/mês (mínimo: R$ {minimum})")]
    MinimumWageViolation {
        /// Actual salary in centavos
        actual: i64,
        /// Minimum required in centavos
        minimum: i64,
    },

    /// FGTS not deposited
    #[error("FGTS não depositado: {months} meses em atraso")]
    FgtsNotDeposited {
        /// Months in arrears
        months: u32,
    },

    /// CTPS not registered
    #[error("CTPS não registrada - vínculo empregatício sem anotação")]
    CtpsNotRegistered,

    /// Vacation not granted (Art. 134)
    #[error("Férias não concedidas no prazo (Art. 134): {months_overdue} meses em atraso")]
    VacationNotGranted {
        /// Months overdue
        months_overdue: u32,
    },

    /// Rest period violation (Art. 66)
    #[error("Intervalo interjornada violado (Art. 66): apenas {hours}h (mínimo 11h)")]
    RestPeriodViolation {
        /// Actual rest hours
        hours: u32,
    },

    /// Unjust termination
    #[error("Rescisão irregular: {description}")]
    UnjustTermination {
        /// Description
        description: String,
    },

    /// False just cause
    #[error("Justa causa indevida (Art. 482): {claimed_ground}")]
    FalseJustCause {
        /// Claimed ground
        claimed_ground: String,
    },

    /// Notice period violation (Art. 487)
    #[error("Aviso prévio não cumprido/pago (Art. 487)")]
    NoticePeriodViolation {
        /// Days required
        days_required: u32,
        /// Days given
        days_given: u32,
    },

    /// Child labor violation
    #[error("Trabalho infantil (Art. 403): menor de {age} anos")]
    ChildLaborViolation {
        /// Worker age
        age: u32,
    },

    /// Unhealthy conditions without premium (Art. 189-192)
    #[error("Insalubridade sem adicional (Art. 189): grau {level}")]
    UnhealthyConditionsNoPremium {
        /// Insalubrity level
        level: String,
    },

    /// Dangerous conditions without premium (Art. 193)
    #[error("Periculosidade sem adicional de 30% (Art. 193)")]
    DangerousConditionsNoPremium,

    /// Harassment/discrimination
    #[error("Assédio/discriminação no trabalho: {description}")]
    WorkplaceHarassment {
        /// Description
        description: String,
    },

    /// Validation error
    #[error("Erro de validação trabalhista: {message}")]
    ValidationError {
        /// Message
        message: String,
    },
}

impl CltError {
    /// Get relevant CLT citation
    pub fn citation(&self) -> String {
        match self {
            Self::WorkingHoursViolation { .. } => format_clt_citation(58, None, None),
            Self::OvertimeNotPaid { .. } => format_clt_citation(59, None, None),
            Self::MinimumWageViolation { .. } => "CF, Art. 7, IV".to_string(),
            Self::FgtsNotDeposited { .. } => "Lei 8.036/90".to_string(),
            Self::CtpsNotRegistered => format_clt_citation(29, None, None),
            Self::VacationNotGranted { .. } => format_clt_citation(134, None, None),
            Self::RestPeriodViolation { .. } => format_clt_citation(66, None, None),
            Self::UnjustTermination { .. } => format_clt_citation(477, None, None),
            Self::FalseJustCause { .. } => format_clt_citation(482, None, None),
            Self::NoticePeriodViolation { .. } => format_clt_citation(487, None, None),
            Self::ChildLaborViolation { .. } => format_clt_citation(403, None, None),
            Self::UnhealthyConditionsNoPremium { .. } => format_clt_citation(189, None, None),
            Self::DangerousConditionsNoPremium => format_clt_citation(193, None, None),
            Self::WorkplaceHarassment { .. } => format_clt_citation(483, None, None),
            Self::ValidationError { .. } => "CLT".to_string(),
        }
    }

    /// Get administrative penalty range (in BRL)
    pub fn mte_penalty_range(&self) -> (u64, u64) {
        match self {
            Self::CtpsNotRegistered => (800, 3000),
            Self::MinimumWageViolation { .. } => (400, 4000),
            Self::WorkingHoursViolation { .. } => (400, 4000),
            Self::ChildLaborViolation { .. } => (2000, 6000),
            Self::FgtsNotDeposited { .. } => (10, 100), // per worker per month
            _ => (400, 4000),
        }
    }

    /// Check if violation is criminal
    pub fn is_criminal(&self) -> bool {
        matches!(
            self,
            Self::ChildLaborViolation { .. } | Self::WorkplaceHarassment { .. }
        )
    }

    /// Get worker remedy
    pub fn remedy_pt(&self) -> &'static str {
        match self {
            Self::WorkingHoursViolation { .. } | Self::OvertimeNotPaid { .. } => {
                "Pagamento de horas extras com adicional de 50% (100% domingos/feriados)"
            }
            Self::MinimumWageViolation { .. } => "Pagamento de diferenças salariais",
            Self::FgtsNotDeposited { .. } => "Depósito do FGTS com correção e multa",
            Self::CtpsNotRegistered => "Reconhecimento de vínculo e anotação na CTPS",
            Self::VacationNotGranted { .. } => "Pagamento de férias em dobro (Art. 137)",
            Self::RestPeriodViolation { .. } => "Pagamento como hora extra",
            Self::UnjustTermination { .. } | Self::FalseJustCause { .. } => {
                "Verbas rescisórias integrais + FGTS 40%"
            }
            Self::NoticePeriodViolation { .. } => "Pagamento do aviso prévio proporcional",
            Self::UnhealthyConditionsNoPremium { .. } => {
                "Adicional de insalubridade (10/20/40% do salário mínimo)"
            }
            Self::DangerousConditionsNoPremium => "Adicional de periculosidade (30% do salário)",
            Self::WorkplaceHarassment { .. } => "Rescisão indireta + indenização por danos morais",
            _ => "Reparação integral",
        }
    }

    /// Calculate potential back payment (simplified)
    pub fn calculate_back_payment(&self, monthly_salary_centavos: i64) -> i64 {
        match self {
            Self::VacationNotGranted { months_overdue } => {
                // Double vacation payment (Art. 137)
                let vacation_base = monthly_salary_centavos + (monthly_salary_centavos / 3);
                vacation_base * 2 * (*months_overdue as i64 / 12).max(1)
            }
            Self::OvertimeNotPaid { hours } => {
                let hourly_rate = monthly_salary_centavos / 220; // 220 hours/month
                (hourly_rate * 150 / 100) * (*hours as i64) // +50%
            }
            Self::MinimumWageViolation { actual, minimum } => {
                (minimum - actual) * 12 // One year of differences
            }
            _ => 0,
        }
    }
}

/// Result type for CLT operations
pub type CltResult<T> = Result<T, CltError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_hours_citation() {
        let error = CltError::WorkingHoursViolation {
            description: "Excesso de jornada".to_string(),
            actual_hours: 50,
            max_hours: 44,
        };
        let citation = error.citation();
        assert!(citation.contains("58"));
    }

    #[test]
    fn test_penalty_range() {
        let error = CltError::ChildLaborViolation { age: 15 };
        let (min, max) = error.mte_penalty_range();
        assert_eq!(min, 2000);
        assert_eq!(max, 6000);
    }

    #[test]
    fn test_criminal_violation() {
        let error = CltError::ChildLaborViolation { age: 14 };
        assert!(error.is_criminal());

        let error = CltError::OvertimeNotPaid { hours: 10 };
        assert!(!error.is_criminal());
    }

    #[test]
    fn test_back_payment_vacation() {
        let error = CltError::VacationNotGranted { months_overdue: 6 };
        let back_payment = error.calculate_back_payment(500000); // R$ 5,000
        assert!(back_payment > 0);
    }

    #[test]
    fn test_back_payment_overtime() {
        let error = CltError::OvertimeNotPaid { hours: 20 };
        let back_payment = error.calculate_back_payment(440000); // R$ 4,400 (2x min wage)
        assert!(back_payment > 0);
    }

    #[test]
    fn test_remedy() {
        let error = CltError::VacationNotGranted { months_overdue: 6 };
        assert!(error.remedy_pt().contains("dobro"));
    }
}
