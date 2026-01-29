//! Integration tests for legal reasoning engine.

use legalis_my::reasoning::*;

#[test]
fn test_employment_analysis() {
    let engine = LegalReasoningEngine::new();
    let analysis = engine
        .analyze("Employee works 9 hours per day", LegalDomain::Employment)
        .expect("Analysis succeeds");

    assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
    assert_eq!(analysis.risk_level, RiskLevel::High);
}

#[test]
fn test_pdpa_analysis() {
    let engine = LegalReasoningEngine::new();
    let analysis = engine
        .analyze("No consent obtained", LegalDomain::DataProtection)
        .expect("Analysis succeeds");

    assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
}

#[test]
fn test_islamic_finance_analysis() {
    let engine = LegalReasoningEngine::new();
    let analysis = engine
        .analyze("Riba-based transaction", LegalDomain::Islamic)
        .expect("Analysis succeeds");

    assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
    assert_eq!(analysis.risk_level, RiskLevel::Critical);
}

#[test]
fn test_competition_analysis() {
    let engine = LegalReasoningEngine::new();
    let analysis = engine
        .analyze("Price fixing agreement", LegalDomain::Competition)
        .expect("Analysis succeeds");

    assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
}
