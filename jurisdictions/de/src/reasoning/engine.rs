//! Legal Reasoning Engine for German Labor Law (Rechtsanalyse-Engine).
//!
//! Provides automated compliance analysis and violation detection.
//! Bietet automatisierte Compliance-Analyse und Verstoßerkennung.

use legalis_core::StatuteRegistry;

use super::context::DeEvaluationContext;
use super::error::ReasoningResult;
use super::statute_adapter::all_labor_statutes;
use super::types::{ComplianceStatus, LegalAnalysis, RiskLevel, Violation, ViolationSeverity};

use crate::arbeitsrecht::types::{
    Dismissal, EmploymentContract, LeaveEntitlement, SickLeave, WorkingHours,
};

/// Legal Reasoning Engine for German Labor Law
/// Rechtsanalyse-Engine für deutsches Arbeitsrecht
pub struct LegalReasoningEngine {
    registry: StatuteRegistry,
}

impl Default for LegalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalReasoningEngine {
    /// Create a new reasoning engine with all German labor statutes
    /// Neue Rechtsanalyse-Engine mit allen deutschen Arbeitsgesetzen
    #[must_use]
    pub fn new() -> Self {
        let mut registry = StatuteRegistry::new();
        for statute in all_labor_statutes() {
            registry.add(statute);
        }
        Self { registry }
    }

    /// Create with custom registry
    #[must_use]
    pub fn with_registry(registry: StatuteRegistry) -> Self {
        Self { registry }
    }

    /// Analyze an employment contract for compliance
    /// Arbeitsvertrag auf Compliance prüfen
    pub fn analyze_employment_contract(
        &self,
        contract: &EmploymentContract,
    ) -> ReasoningResult<LegalAnalysis> {
        let _ctx = DeEvaluationContext::new(contract);
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "Arbeitsvertragsanalyse / Employment Contract Analysis",
        );

        // Check written form requirement (§2 NachwG)
        // Schriftformerfordernis prüfen
        if !contract.written {
            analysis.add_violation(
                Violation::new(
                    "NachwG_§2",
                    "Nachweisgesetz / Documentation Act",
                    "Arbeitsvertrag nicht schriftlich / Contract not in writing",
                    ViolationSeverity::Moderate,
                )
                .with_legal_reference("§2 NachwG")
                .with_remediation("Schriftlichen Arbeitsvertrag ausstellen"),
            );
        }

        // Check ArbZG compliance
        // ArbZG-Konformität prüfen
        if !contract.working_hours.complies_with_arbzg() {
            let hours_per_day = contract.working_hours.hours_per_week as f32
                / contract.working_hours.days_per_week as f32;
            analysis.add_violation(
                Violation::new(
                    "ArbZG_§3",
                    "Arbeitszeitgesetz / Working Hours Act",
                    format!(
                        "Tägliche Arbeitszeit {:.1}h übersteigt 10h-Grenze / Daily {} hours exceeds 10h limit",
                        hours_per_day, hours_per_day
                    ),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("§3 ArbZG")
                .with_remediation("Arbeitszeit auf maximal 10 Stunden pro Tag reduzieren"),
            );
        }

        // Check probation period (max 6 months)
        // Probezeit prüfen (max. 6 Monate)
        if let Some(months) = contract.probation_period_months {
            if months > 6 {
                analysis.add_violation(
                    Violation::new(
                        "BGB_§622",
                        "Probezeit / Probation Period",
                        format!(
                            "Probezeit {} Monate übersteigt 6 Monate / {} months exceeds 6-month limit",
                            months, months
                        ),
                        ViolationSeverity::Moderate,
                    )
                    .with_legal_reference("§622 Abs. 3 BGB")
                    .with_remediation("Probezeit auf maximal 6 Monate begrenzen"),
                );
            }
        }

        // Update overall status
        if !analysis.violations.is_empty() {
            let has_major = analysis
                .violations
                .iter()
                .any(|v| v.severity >= ViolationSeverity::Major);

            analysis.status = if has_major {
                ComplianceStatus::NonCompliant
            } else {
                ComplianceStatus::PartiallyCompliant
            };
        }

        Ok(analysis)
    }

    /// Analyze working hours for ArbZG compliance
    /// Arbeitszeit auf ArbZG-Konformität prüfen
    pub fn analyze_working_hours(&self, hours: &WorkingHours) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "Arbeitszeitanalyse / Working Hours Analysis",
        );

        // Check daily limit (§3 ArbZG)
        if !hours.complies_with_arbzg() {
            analysis.add_violation(
                Violation::new(
                    "ArbZG_§3",
                    "Arbeitszeit / Working Hours",
                    "Tägliche Arbeitszeit übersteigt gesetzliche Grenze / Daily hours exceed legal limit",
                    ViolationSeverity::Major,
                )
                .with_legal_reference("§3 ArbZG"),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        // Check weekly hours
        if hours.hours_per_week > 48 {
            analysis.add_violation(
                Violation::new(
                    "ArbZG_§3",
                    "Wochenarbeitszeit / Weekly Working Hours",
                    format!(
                        "Wochenarbeitszeit {} Stunden übersteigt 48h / {} hours/week exceeds 48h",
                        hours.hours_per_week, hours.hours_per_week
                    ),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("§3 ArbZG"),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        Ok(analysis)
    }

    /// Analyze dismissal for compliance
    /// Kündigung auf Rechtskonformität prüfen
    pub fn analyze_dismissal(&self, dismissal: &Dismissal) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "Kündigungsanalyse / Dismissal Analysis",
        );

        // Check written form (§623 BGB)
        // Schriftform prüfen
        if !dismissal.written {
            analysis.add_violation(
                Violation::new(
                    "BGB_§623",
                    "Schriftformerfordernis / Written Form Requirement",
                    "Kündigung nicht schriftlich - unwirksam / Dismissal not in writing - void",
                    ViolationSeverity::Critical,
                )
                .with_legal_reference("§623 BGB")
                .with_remediation("Kündigung schriftlich mit Originalunterschrift ausstellen"),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        // Check works council consultation (§102 BetrVG)
        // Betriebsratsanhörung prüfen
        if !dismissal.works_council_consulted {
            analysis.add_violation(
                Violation::new(
                    "BetrVG_§102",
                    "Betriebsratsanhörung / Works Council Consultation",
                    "Betriebsrat nicht angehört (falls vorhanden) / Works council not consulted",
                    ViolationSeverity::Major,
                )
                .with_legal_reference("§102 BetrVG")
                .with_remediation("Betriebsrat vor jeder Kündigung anhören"),
            );
            if analysis.status == ComplianceStatus::Compliant {
                analysis.status = ComplianceStatus::PartiallyCompliant;
            }
        }

        // Check notice period
        // Kündigungsfrist prüfen
        if dismissal.notice_period_weeks < 4 {
            analysis.add_violation(
                Violation::new(
                    "BGB_§622",
                    "Kündigungsfrist / Notice Period",
                    format!(
                        "Kündigungsfrist {} Wochen unter Minimum / {} weeks below minimum",
                        dismissal.notice_period_weeks, dismissal.notice_period_weeks
                    ),
                    ViolationSeverity::Moderate,
                )
                .with_legal_reference("§622 BGB"),
            );
        }

        Ok(analysis)
    }

    /// Analyze leave entitlement for BUrlG compliance
    /// Urlaubsanspruch auf BUrlG-Konformität prüfen
    pub fn analyze_leave_entitlement(
        &self,
        leave: &LeaveEntitlement,
    ) -> ReasoningResult<LegalAnalysis> {
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "Urlaubsanalyse / Leave Analysis",
        );

        // Check minimum leave (§3 BUrlG)
        let statutory_minimum = LeaveEntitlement::calculate_minimum(leave.days_per_week);
        if leave.contractual_days < statutory_minimum {
            analysis.add_violation(
                Violation::new(
                    "BUrlG_§3",
                    "Mindesturlaub / Minimum Leave",
                    format!(
                        "Vertraglicher Urlaub {} Tage unter Minimum {} Tage / Contractual {} days below minimum {}",
                        leave.contractual_days, statutory_minimum, leave.contractual_days, statutory_minimum
                    ),
                    ViolationSeverity::Major,
                )
                .with_legal_reference("§3 BUrlG")
                .with_remediation(format!("Urlaubstage auf mindestens {} erhöhen", statutory_minimum)),
            );
            analysis.status = ComplianceStatus::NonCompliant;
        }

        Ok(analysis)
    }

    /// Analyze sick leave for EFZG compliance
    /// Krankheit auf EFZG-Konformität prüfen
    pub fn analyze_sick_leave(&self, sick: &SickLeave) -> ReasoningResult<LegalAnalysis> {
        let today = chrono::Utc::now().date_naive();
        let mut analysis = LegalAnalysis::new(
            ComplianceStatus::Compliant,
            RiskLevel::None,
            "Krankheitsanalyse / Sick Leave Analysis",
        );

        // Check medical certificate (§5 EFZG - required after 3 days)
        // Attest prüfen (§5 EFZG - nach 3 Tagen erforderlich)
        if sick.duration_days(today) > 3 && !sick.medical_certificate_provided {
            analysis.add_violation(
                Violation::new(
                    "EFZG_§5",
                    "Ärztliches Attest / Medical Certificate",
                    "Attest nach 3 Tagen nicht vorgelegt / Certificate not provided after 3 days",
                    ViolationSeverity::Minor,
                )
                .with_legal_reference("§5 EFZG"),
            );
        }

        // Check timely notification
        if !sick.notification_timely {
            analysis.add_violation(
                Violation::new(
                    "EFZG_§5",
                    "Krankmeldung / Sick Notification",
                    "Keine unverzügliche Krankmeldung / No immediate notification",
                    ViolationSeverity::Minor,
                )
                .with_legal_reference("§5 Abs. 1 EFZG"),
            );
        }

        if !analysis.violations.is_empty() {
            analysis.status = ComplianceStatus::PartiallyCompliant;
        }

        Ok(analysis)
    }

    /// Get the statute registry
    #[must_use]
    pub fn registry(&self) -> &StatuteRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbeitsrecht::types::{CompanySize, ContractType, Employee, Employer, Salary};
    use crate::gmbhg::Capital;
    use chrono::NaiveDate;

    fn create_test_contract() -> EmploymentContract {
        EmploymentContract {
            employee: Employee {
                name: "Hans Müller".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1985, 5, 15).unwrap(),
                address: "Berlin".to_string(),
                social_security_number: None,
            },
            employer: Employer {
                name: "GmbH ABC".to_string(),
                address: "Berlin".to_string(),
                company_size: CompanySize::Medium,
            },
            contract_type: ContractType::Unlimited,
            start_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            end_date: None,
            probation_period_months: Some(6),
            salary: Salary {
                gross_monthly: Capital::from_cents(500_000),
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
        }
    }

    #[test]
    fn test_compliant_contract() {
        let engine = LegalReasoningEngine::new();
        let contract = create_test_contract();
        let analysis = engine.analyze_employment_contract(&contract).unwrap();

        assert_eq!(analysis.status, ComplianceStatus::Compliant);
        assert!(analysis.violations.is_empty());
    }

    #[test]
    fn test_unwritten_contract() {
        let engine = LegalReasoningEngine::new();
        let mut contract = create_test_contract();
        contract.written = false;

        let analysis = engine.analyze_employment_contract(&contract).unwrap();
        assert_eq!(analysis.status, ComplianceStatus::PartiallyCompliant);
        assert!(!analysis.violations.is_empty());
    }

    #[test]
    fn test_excessive_hours() {
        let engine = LegalReasoningEngine::new();
        let hours = WorkingHours {
            hours_per_week: 55, // Exceeds 48h
            days_per_week: 5,
            overtime_allowed: true,
        };

        let analysis = engine.analyze_working_hours(&hours).unwrap();
        assert_eq!(analysis.status, ComplianceStatus::NonCompliant);
    }

    #[test]
    fn test_insufficient_leave() {
        let engine = LegalReasoningEngine::new();
        let leave = LeaveEntitlement {
            employee_name: "Test".to_string(),
            year: 2026,
            days_per_week: 5,
            minimum_days: 20,
            contractual_days: 15, // Below 20-day minimum for 5-day week
            days_taken: 0,
            days_carried_over: 0,
        };

        let analysis = engine.analyze_leave_entitlement(&leave).unwrap();
        assert_eq!(analysis.status, ComplianceStatus::NonCompliant);
    }
}
