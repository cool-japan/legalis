//! EvaluationContext implementations for German legal entities.
//!
//! Bridges German labor law types to the legalis-core reasoning framework.
//! Verbindet deutsche Arbeitsrechtstypen mit dem legalis-core Reasoning-Framework.

use chrono::NaiveDate;
use legalis_core::{DurationUnit, EvaluationContext, RegionType, RelationshipType};

use crate::arbeitsrecht::types::{
    CompanySize, ContractType, Dismissal, DismissalGrounds, DismissalType, EmploymentContract,
    LeaveEntitlement, SickLeave, WorkingHours,
};

/// Wrapper providing EvaluationContext for German labor law entities
/// Wrapper für EvaluationContext für deutsche Arbeitsrechts-Entitäten
pub struct DeEvaluationContext<'a, T> {
    entity: &'a T,
}

impl<'a, T> DeEvaluationContext<'a, T> {
    /// Create a new evaluation context
    #[must_use]
    pub const fn new(entity: &'a T) -> Self {
        Self { entity }
    }
}

// ============================================================================
// EvaluationContext for EmploymentContract (Arbeitsvertrag)
// ============================================================================

impl EvaluationContext for DeEvaluationContext<'_, EmploymentContract> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "contract_type" | "Vertragsart" => Some(match &self.entity.contract_type {
                ContractType::Unlimited => "unlimited".to_string(),
                ContractType::FixedTerm { reason } => format!("fixed_term:{:?}", reason),
                ContractType::PartTime { hours_per_week } => {
                    format!("part_time:{}", hours_per_week)
                }
                ContractType::TemporaryAgency => "temporary_agency".to_string(),
            }),
            "employee_name" | "Arbeitnehmername" => Some(self.entity.employee.name.clone()),
            "employer_name" | "Arbeitgebername" => Some(self.entity.employer.name.clone()),
            "company_size" | "Betriebsgröße" => Some(match self.entity.employer.company_size {
                CompanySize::Small => "small".to_string(),
                CompanySize::Medium => "medium".to_string(),
                CompanySize::Large => "large".to_string(),
            }),
            "written" | "schriftlich" => Some(self.entity.written.to_string()),
            "has_dismissal_protection" | "Kündigungsschutz" => Some(
                self.entity
                    .employer
                    .company_size
                    .has_dismissal_protection()
                    .to_string(),
            ),
            "duties" | "Aufgaben" => Some(self.entity.duties.clone()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        let today = chrono::Utc::now().date_naive();
        Some(self.entity.employee.age_at(today))
    }

    fn get_income(&self) -> Option<u64> {
        // Return monthly salary in cents
        Some(self.entity.salary.gross_monthly.amount_cents)
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        // DE jurisdiction
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        let today = chrono::Utc::now().date_naive();
        let duration = today - self.entity.start_date;

        match unit {
            DurationUnit::Days => Some(duration.num_days().max(0) as u32),
            DurationUnit::Weeks => Some(duration.num_weeks().max(0) as u32),
            DurationUnit::Months => Some((duration.num_days() / 30).max(0) as u32),
            DurationUnit::Years => Some((duration.num_days() / 365).max(0) as u32),
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "hours_per_week" | "Wochenstunden" => {
                Some(self.entity.working_hours.hours_per_week as f64)
            }
            "days_per_week" | "Wochentage" => Some(self.entity.working_hours.days_per_week as f64),
            "gross_monthly" | "Bruttomonatsgehalt" => {
                Some(self.entity.salary.gross_monthly.amount_cents as f64 / 100.0)
            }
            "gross_annual" | "Bruttojahresgehalt" => {
                Some(self.entity.salary.gross_annual().amount_cents as f64 / 100.0)
            }
            "probation_months" | "Probezeit" => {
                self.entity.probation_period_months.map(|m| m as f64)
            }
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for WorkingHours (Arbeitszeit)
// ============================================================================

impl EvaluationContext for DeEvaluationContext<'_, WorkingHours> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "hours_per_week" | "Wochenstunden" => Some(self.entity.hours_per_week.to_string()),
            "days_per_week" | "Wochentage" => Some(self.entity.days_per_week.to_string()),
            "overtime_allowed" | "Überstunden" => Some(self.entity.overtime_allowed.to_string()),
            "arbzg_compliant" | "ArbZG-konform" => {
                Some(self.entity.complies_with_arbzg().to_string())
            }
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, _unit: DurationUnit) -> Option<u32> {
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "hours_per_week" | "Wochenstunden" => Some(self.entity.hours_per_week as f64),
            "hours_per_day" | "Tagesstunden" => {
                if self.entity.days_per_week > 0 {
                    Some(self.entity.hours_per_week as f64 / self.entity.days_per_week as f64)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for Dismissal (Kündigung)
// ============================================================================

impl EvaluationContext for DeEvaluationContext<'_, Dismissal> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "dismissal_type" | "Kündigungsart" => Some(match self.entity.dismissal_type {
                DismissalType::Ordinary => "ordinary".to_string(),
                DismissalType::Extraordinary => "extraordinary".to_string(),
            }),
            "grounds" | "Grund" => Some(match &self.entity.grounds {
                DismissalGrounds::Conduct { .. } => "conduct".to_string(),
                DismissalGrounds::Personal { .. } => "personal".to_string(),
                DismissalGrounds::Operational { .. } => "operational".to_string(),
                DismissalGrounds::ExtraordinaryCause { .. } => "extraordinary".to_string(),
            }),
            "written" | "schriftlich" => Some(self.entity.written.to_string()),
            "works_council_consulted" | "Betriebsrat" => {
                Some(self.entity.works_council_consulted.to_string())
            }
            "employee_name" | "Arbeitnehmername" => Some(self.entity.employee_name.clone()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        match unit {
            DurationUnit::Weeks => Some(self.entity.notice_period_weeks as u32),
            DurationUnit::Days => {
                let duration = self.entity.effective_date - self.entity.dismissal_date;
                Some(duration.num_days().max(0) as u32)
            }
            _ => None,
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "notice_weeks" | "Kündigungsfrist" => Some(self.entity.notice_period_weeks as f64),
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for LeaveEntitlement (Urlaubsanspruch)
// ============================================================================

impl EvaluationContext for DeEvaluationContext<'_, LeaveEntitlement> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "employee_name" | "Arbeitnehmername" => Some(self.entity.employee_name.clone()),
            "year" | "Jahr" => Some(self.entity.year.to_string()),
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, _unit: DurationUnit) -> Option<u32> {
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        match formula {
            "minimum_days" | "Mindesturlaub" => Some(self.entity.minimum_days as f64),
            "contractual_days" | "Vertraglicher Urlaub" => {
                Some(self.entity.contractual_days as f64)
            }
            "days_taken" | "Genommene Tage" => Some(self.entity.days_taken as f64),
            "days_remaining" | "Resturlaub" => Some(self.entity.remaining_days() as f64),
            "days_carried_over" | "Übertragene Tage" => Some(self.entity.days_carried_over as f64),
            _ => None,
        }
    }
}

// ============================================================================
// EvaluationContext for SickLeave (Krankheit)
// ============================================================================

impl EvaluationContext for DeEvaluationContext<'_, SickLeave> {
    fn get_attribute(&self, key: &str) -> Option<String> {
        match key {
            "employee_name" | "Arbeitnehmername" => Some(self.entity.employee_name.clone()),
            "certificate_provided" | "Attest" => {
                Some(self.entity.medical_certificate_provided.to_string())
            }
            "notification_timely" | "Rechtzeitige Meldung" => {
                Some(self.entity.notification_timely.to_string())
            }
            _ => None,
        }
    }

    fn get_age(&self) -> Option<u32> {
        None
    }

    fn get_income(&self) -> Option<u64> {
        None
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        Some(chrono::Utc::now().date_naive())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        None
    }

    fn get_duration(&self, unit: DurationUnit) -> Option<u32> {
        let today = chrono::Utc::now().date_naive();
        match unit {
            DurationUnit::Days => Some(self.entity.duration_days(today)),
            DurationUnit::Weeks => Some(self.entity.duration_days(today) / 7),
            _ => None,
        }
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        None
    }

    fn evaluate_formula(&self, formula: &str) -> Option<f64> {
        let today = chrono::Utc::now().date_naive();
        match formula {
            "duration_days" | "Krankheitstage" => Some(self.entity.duration_days(today) as f64),
            "entitled_to_pay" | "Lohnfortzahlung" => {
                Some(if self.entity.entitled_to_continued_pay(today) {
                    1.0
                } else {
                    0.0
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbeitsrecht::types::{Employee, Employer, Salary};
    use crate::gmbhg::Capital;

    #[test]
    fn test_employment_contract_context() {
        let contract = EmploymentContract {
            employee: Employee {
                name: "Hans Müller".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1985, 5, 15).expect("valid date"),
                address: "Berlin".to_string(),
                social_security_number: None,
            },
            employer: Employer {
                name: "GmbH ABC".to_string(),
                address: "Berlin".to_string(),
                company_size: CompanySize::Medium,
            },
            contract_type: ContractType::Unlimited,
            start_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date"),
            end_date: None,
            probation_period_months: Some(6),
            salary: Salary {
                gross_monthly: Capital::from_cents(500_000), // 5000 EUR
                payment_day: 25,
                includes_overtime: false,
            },
            working_hours: WorkingHours {
                hours_per_week: 40,
                days_per_week: 5,
                overtime_allowed: true,
            },
            duties: "Software-Entwickler".to_string(),
            written: true,
        };

        let ctx = DeEvaluationContext::new(&contract);
        assert_eq!(
            ctx.get_attribute("contract_type"),
            Some("unlimited".to_string())
        );
        assert_eq!(ctx.get_income(), Some(500_000));
        assert_eq!(ctx.evaluate_formula("hours_per_week"), Some(40.0));
    }
}
