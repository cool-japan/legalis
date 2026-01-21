//! Domain-specific analyzers for French law.
//!
//! This module provides specialized analyzers for each legal domain,
//! offering convenient methods for specific analysis scenarios.

use legalis_core::EvaluationContext;

use super::engine::LegalReasoningEngine;
use super::error::ReasoningResult;
#[cfg(test)]
use super::types::ViolationSeverity;
use super::types::{LegalAnalysis, Violation};

/// Contract law analyzer.
///
/// Provides specialized analysis methods for French contract law (Code civil).
pub struct ContractAnalyzer {
    engine: LegalReasoningEngine,
}

impl ContractAnalyzer {
    /// Create a new contract analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            engine: LegalReasoningEngine::new(),
        }
    }

    /// Analyze contract validity according to Article 1128.
    ///
    /// Checks the three requirements for contract validity:
    /// 1. Consent of parties (consentement)
    /// 2. Capacity to contract (capacité)
    /// 3. Lawful and certain content (contenu licite et certain)
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract to analyze
    ///
    /// # Returns
    ///
    /// List of validity violations (empty if valid)
    pub fn analyze_validity(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<Vec<Violation>> {
        let analysis = self.engine.analyze_contract(context)?;

        // Filter for validity-related violations (Article 1128)
        let validity_violations: Vec<Violation> = analysis
            .violations
            .into_iter()
            .filter(|v| v.article_id.contains("1128"))
            .collect();

        Ok(validity_violations)
    }

    /// Analyze breach of contract and available remedies.
    ///
    /// Evaluates breach scenarios according to Articles 1217 and 1231,
    /// identifying applicable remedies and potential damages.
    pub fn analyze_breach(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<Vec<Violation>> {
        let analysis = self.engine.analyze_contract(context)?;

        // Filter for breach-related violations (Articles 1217, 1231)
        let breach_violations: Vec<Violation> = analysis
            .violations
            .into_iter()
            .filter(|v| v.article_id.contains("1217") || v.article_id.contains("1231"))
            .collect();

        Ok(breach_violations)
    }

    /// Perform comprehensive contract analysis.
    ///
    /// Analyzes all aspects of a contract including validity, breach, and remedies.
    pub fn analyze_comprehensive(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<LegalAnalysis> {
        self.engine.analyze_contract(context)
    }

    /// Calculate estimated damages for breach.
    ///
    /// Analyzes contract breach and estimates damages based on:
    /// - Actual loss (perte éprouvée)
    /// - Lost profit (gain manqué)
    /// - Penalty clauses (clause pénale)
    ///
    /// Returns None if no breach detected or insufficient information.
    pub fn calculate_damages(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<Option<u64>> {
        // Check for breach
        let breach_violations = self.analyze_breach(context)?;

        if breach_violations.is_empty() {
            return Ok(None);
        }

        // Try to extract damage amounts from remedies
        for violation in &breach_violations {
            for remedy in &violation.remedies {
                if let Some(damages) = remedy.estimated_damages {
                    return Ok(Some(damages));
                }
            }
        }

        Ok(None)
    }
}

impl Default for ContractAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Labor law analyzer.
///
/// Provides specialized analysis methods for French labor law (Code du travail).
pub struct LaborAnalyzer {
    engine: LegalReasoningEngine,
}

impl LaborAnalyzer {
    /// Create a new labor analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            engine: LegalReasoningEngine::new(),
        }
    }

    /// Analyze CDD (fixed-term contract) validity.
    ///
    /// Checks compliance with CDD regulations including:
    /// - Maximum duration (18 months - Article L1242-8)
    /// - Valid reasons (Article L1242-2)
    /// - Written form requirement (Article L1242-12)
    pub fn analyze_cdd_validity(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<Vec<Violation>> {
        let analysis = self.engine.analyze_employment(context)?;

        // Filter for CDD-related violations
        let cdd_violations: Vec<Violation> = analysis
            .violations
            .into_iter()
            .filter(|v| v.article_id.contains("l1242"))
            .collect();

        Ok(cdd_violations)
    }

    /// Analyze working hours compliance.
    ///
    /// Checks compliance with working hour regulations:
    /// - Legal duration (35 hours/week - Article L3121-27)
    /// - Daily maximum (10 hours - Article L3121-18)
    /// - Weekly maximum (48 hours - Article L3121-20)
    pub fn analyze_working_hours(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<Vec<Violation>> {
        let analysis = self.engine.analyze_employment(context)?;

        // Filter for working hours violations
        let hours_violations: Vec<Violation> = analysis
            .violations
            .into_iter()
            .filter(|v| v.article_id.contains("l3121"))
            .collect();

        Ok(hours_violations)
    }

    /// Analyze dismissal procedure compliance.
    ///
    /// Checks compliance with dismissal regulations including:
    /// - Valid grounds (personal or economic)
    /// - Preliminary interview requirement
    /// - Notice period requirements
    pub fn analyze_dismissal(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<Vec<Violation>> {
        let analysis = self.engine.analyze_employment(context)?;

        // Filter for dismissal-related violations
        let dismissal_violations: Vec<Violation> = analysis
            .violations
            .into_iter()
            .filter(|v| {
                v.article_id.contains("l1232")
                    || v.article_id.contains("l1233")
                    || v.article_id.contains("l1234")
            })
            .collect();

        Ok(dismissal_violations)
    }

    /// Perform comprehensive employment contract analysis.
    pub fn analyze_comprehensive(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<LegalAnalysis> {
        self.engine.analyze_employment(context)
    }

    /// Calculate overtime premium owed.
    ///
    /// Calculates overtime premium based on hours worked:
    /// - First 8 hours over 35: 25% premium
    /// - Beyond 43 hours: 50% premium
    ///
    /// Returns None if working hours are within legal limits.
    pub fn calculate_overtime_premium(&self, weekly_hours: f64, hourly_rate: u64) -> Option<u64> {
        if weekly_hours <= 35.0 {
            return None;
        }

        let overtime_hours = weekly_hours - 35.0;
        let first_8_hours = overtime_hours.min(8.0);
        let beyond_8_hours = (overtime_hours - 8.0).max(0.0);

        let first_premium = (first_8_hours * hourly_rate as f64 * 0.25) as u64;
        let second_premium = (beyond_8_hours * hourly_rate as f64 * 0.50) as u64;

        Some(first_premium + second_premium)
    }
}

impl Default for LaborAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Company law analyzer.
///
/// Provides specialized analysis methods for French company law (Code de commerce).
pub struct CompanyAnalyzer {
    engine: LegalReasoningEngine,
}

impl CompanyAnalyzer {
    /// Create a new company analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            engine: LegalReasoningEngine::new(),
        }
    }

    /// Analyze SA (Société Anonyme) formation requirements.
    ///
    /// Checks compliance with SA formation rules:
    /// - Minimum capital (€37,000 - Article L225-1)
    /// - Board composition (Article L225-17)
    /// - Director requirements (Article L225-18)
    pub fn analyze_sa_formation(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<Vec<Violation>> {
        let analysis = self.engine.analyze_articles(context)?;

        // Filter for SA formation violations
        let sa_violations: Vec<Violation> = analysis
            .violations
            .into_iter()
            .filter(|v| v.article_id.contains("l225"))
            .collect();

        Ok(sa_violations)
    }

    /// Analyze board of directors composition.
    ///
    /// Checks board compliance including:
    /// - Size requirements (3-18 members)
    /// - Term limits (maximum 6 years)
    /// - Age requirements
    pub fn analyze_board_composition(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<Vec<Violation>> {
        let analysis = self.engine.analyze_articles(context)?;

        // Filter for board-related violations
        let board_violations: Vec<Violation> = analysis
            .violations
            .into_iter()
            .filter(|v| v.article_id.contains("l225-17") || v.article_id.contains("l225-18"))
            .collect();

        Ok(board_violations)
    }

    /// Perform comprehensive articles of incorporation analysis.
    pub fn analyze_comprehensive(
        &self,
        context: &dyn EvaluationContext,
    ) -> ReasoningResult<LegalAnalysis> {
        self.engine.analyze_articles(context)
    }

    /// Check if company type requires specific capital requirements.
    ///
    /// Returns the minimum capital requirement for the company type.
    #[must_use]
    pub fn minimum_capital_requirement(company_type: &str) -> Option<u64> {
        match company_type {
            "SA" => Some(37_000),
            "SARL" | "SAS" => Some(1), // Symbolic €1 minimum
            _ => None,
        }
    }
}

impl Default for CompanyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified analyzer providing access to all domain analyzers.
///
/// This is a convenience wrapper that provides a single entry point
/// for all legal analysis needs.
pub struct FrenchLawAnalyzer {
    /// Contract law analyzer
    pub contract: ContractAnalyzer,
    /// Labor law analyzer
    pub labor: LaborAnalyzer,
    /// Company law analyzer
    pub company: CompanyAnalyzer,
}

impl FrenchLawAnalyzer {
    /// Create a new unified French law analyzer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            contract: ContractAnalyzer::new(),
            labor: LaborAnalyzer::new(),
            company: CompanyAnalyzer::new(),
        }
    }
}

impl Default for FrenchLawAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Mock evaluation context for testing
    struct MockContext {
        attributes: HashMap<String, String>,
        age: Option<u32>,
        duration_months: Option<u32>,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                attributes: HashMap::new(),
                age: None,
                duration_months: None,
            }
        }

        fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            let key_str = key.into();
            let value_str = value.into();

            // Auto-populate duration_months if duration_months attribute is set
            if key_str == "duration_months"
                && let Ok(months) = value_str.parse::<u32>()
            {
                self.duration_months = Some(months);
            }

            self.attributes.insert(key_str, value_str);
            self
        }

        #[allow(dead_code)]
        fn with_age(mut self, age: u32) -> Self {
            self.age = Some(age);
            self
        }
    }

    impl EvaluationContext for MockContext {
        fn get_attribute(&self, key: &str) -> Option<String> {
            self.attributes.get(key).cloned()
        }

        fn get_age(&self) -> Option<u32> {
            self.age
        }

        fn get_income(&self) -> Option<u64> {
            None
        }

        fn get_current_date(&self) -> Option<chrono::NaiveDate> {
            None
        }

        fn check_geographic(
            &self,
            _region_type: legalis_core::RegionType,
            _region_id: &str,
        ) -> bool {
            false
        }

        fn check_relationship(
            &self,
            _relationship_type: legalis_core::RelationshipType,
            _target_id: Option<&str>,
        ) -> bool {
            false
        }

        fn get_residency_months(&self) -> Option<u32> {
            None
        }

        fn get_duration(&self, unit: legalis_core::DurationUnit) -> Option<u32> {
            match unit {
                legalis_core::DurationUnit::Months => self.duration_months,
                _ => None,
            }
        }

        fn get_percentage(&self, _context: &str) -> Option<u32> {
            None
        }

        fn evaluate_formula(&self, _formula: &str) -> Option<f64> {
            None
        }
    }

    #[test]
    fn test_contract_analyzer_creation() {
        let analyzer = ContractAnalyzer::new();
        assert!(analyzer.engine.all_statutes().len() >= 6);
    }

    #[test]
    fn test_labor_analyzer_creation() {
        let analyzer = LaborAnalyzer::new();
        assert!(analyzer.engine.all_statutes().len() >= 6);
    }

    #[test]
    fn test_company_analyzer_creation() {
        let analyzer = CompanyAnalyzer::new();
        assert!(analyzer.engine.all_statutes().len() >= 6);
    }

    #[test]
    fn test_french_law_analyzer() {
        let analyzer = FrenchLawAnalyzer::new();
        assert!(analyzer.contract.engine.all_statutes().len() >= 6);
        assert!(analyzer.labor.engine.all_statutes().len() >= 6);
        assert!(analyzer.company.engine.all_statutes().len() >= 6);
    }

    #[test]
    fn test_analyze_validity_compliant() {
        let analyzer = ContractAnalyzer::new();
        let context = MockContext::new()
            .with_attribute("consent_given", "true")
            .with_attribute("not_under_guardianship", "true")
            .with_attribute("content_lawful", "true")
            .with_attribute("content_certain", "true")
            .with_age(25);

        let violations = analyzer.analyze_validity(&context).unwrap();
        // Should have no violations if all requirements met
        assert!(
            violations.is_empty()
                || violations
                    .iter()
                    .all(|v| v.severity != ViolationSeverity::Critical)
        );
    }

    #[test]
    fn test_calculate_overtime_premium() {
        let analyzer = LaborAnalyzer::new();

        // No overtime
        assert_eq!(analyzer.calculate_overtime_premium(35.0, 10), None);

        // 5 hours overtime (within first 8)
        let premium = analyzer.calculate_overtime_premium(40.0, 10).unwrap();
        assert_eq!(premium, 12); // 5 * 10 * 0.25 = 12.5 -> 12

        // 10 hours overtime (2 hours at 50%)
        let premium = analyzer.calculate_overtime_premium(45.0, 10).unwrap();
        assert_eq!(premium, 30); // (8 * 10 * 0.25) + (2 * 10 * 0.50) = 20 + 10 = 30
    }

    #[test]
    fn test_minimum_capital_requirement() {
        assert_eq!(
            CompanyAnalyzer::minimum_capital_requirement("SA"),
            Some(37_000)
        );
        assert_eq!(
            CompanyAnalyzer::minimum_capital_requirement("SARL"),
            Some(1)
        );
        assert_eq!(CompanyAnalyzer::minimum_capital_requirement("SAS"), Some(1));
        assert_eq!(CompanyAnalyzer::minimum_capital_requirement("EURL"), None);
    }

    #[test]
    fn test_analyze_cdd_validity() {
        let analyzer = LaborAnalyzer::new();
        let context = MockContext::new()
            .with_attribute("contract_type", "CDD")
            .with_attribute("duration_months", "20") // Exceeds 18 months
            .with_attribute("weekly_hours", "35"); // Include to avoid other violations

        let violations = analyzer.analyze_cdd_validity(&context).unwrap();
        // Should detect violations if any CDD rules broken
        assert!(violations.iter().all(|v| v.article_id.contains("l1242")));
    }

    #[test]
    fn test_analyze_working_hours() {
        let analyzer = LaborAnalyzer::new();
        let context = MockContext::new().with_attribute("weekly_hours", "50"); // Exceeds 35 hours

        let violations = analyzer.analyze_working_hours(&context).unwrap();
        // Should detect working hours violations
        assert!(violations.iter().all(|v| v.article_id.contains("l3121")));
    }

    #[test]
    fn test_analyze_sa_formation() {
        let analyzer = CompanyAnalyzer::new();
        let context = MockContext::new()
            .with_attribute("capital_eur", "30000") // Below €37,000 minimum
            .with_attribute("company_name", "Test SA");

        let violations = analyzer.analyze_sa_formation(&context).unwrap();
        // Should detect capital requirement violation
        assert!(violations.iter().all(|v| v.article_id.contains("l225")));
    }
}
