//! Basel III compliance format support.
//!
//! Basel III is an international regulatory framework for banks developed by
//! the Basel Committee on Banking Supervision (BCBS). This module supports:
//! - Capital adequacy requirements
//! - Leverage ratio requirements
//! - Liquidity coverage ratio (LCR)
//! - Net stable funding ratio (NSFR)

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Basel III compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Basel3Report {
    /// Report metadata
    pub metadata: Basel3Metadata,
    /// Capital requirements
    pub capital_requirements: Vec<CapitalRequirement>,
    /// Liquidity requirements
    pub liquidity_requirements: Vec<LiquidityRequirement>,
}

/// Basel III metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Basel3Metadata {
    /// Report ID
    pub report_id: String,
    /// Bank identifier (LEI)
    pub bank_lei: String,
    /// Reporting date
    pub reporting_date: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Supervisor
    pub supervisor: String,
}

/// Capital requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalRequirement {
    /// Requirement ID
    pub id: String,
    /// Requirement name
    pub name: String,
    /// Type (e.g., "CET1", "Tier1", "Total")
    pub capital_type: String,
    /// Minimum ratio (percentage)
    pub minimum_ratio: f64,
    /// Current ratio (percentage)
    pub current_ratio: Option<f64>,
    /// Risk-weighted assets (RWA)
    pub risk_weighted_assets: Option<f64>,
    /// Capital amount
    pub capital_amount: Option<f64>,
}

/// Liquidity requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityRequirement {
    /// Requirement ID
    pub id: String,
    /// Requirement name
    pub name: String,
    /// Type (e.g., "LCR", "NSFR")
    pub liquidity_type: String,
    /// Minimum ratio (percentage)
    pub minimum_ratio: f64,
    /// Current ratio (percentage)
    pub current_ratio: Option<f64>,
    /// High-quality liquid assets (HQLA)
    pub hqla: Option<f64>,
    /// Net cash outflows
    pub net_cash_outflows: Option<f64>,
}

/// Basel III importer
pub struct Basel3Importer;

impl Basel3Importer {
    /// Creates a new Basel III importer
    pub fn new() -> Self {
        Self
    }

    fn parse_basel3(&self, source: &str) -> InteropResult<Basel3Report> {
        serde_json::from_str(source)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse Basel III JSON: {}", e)))
    }
}

impl Default for Basel3Importer {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for Basel3Importer {
    fn format(&self) -> LegalFormat {
        LegalFormat::Basel3
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let report = self.parse_basel3(source)?;
        let mut statutes = Vec::new();
        let mut conv_report = ConversionReport::new(LegalFormat::Basel3, LegalFormat::Legalis);

        // Convert capital requirements to statutes
        for req in &report.capital_requirements {
            let mut effect = Effect::new(
                EffectType::Obligation,
                format!(
                    "Maintain {} capital ratio >= {}%",
                    req.capital_type, req.minimum_ratio
                ),
            );

            effect
                .parameters
                .insert("capital_type".to_string(), req.capital_type.clone());
            effect
                .parameters
                .insert("minimum_ratio".to_string(), req.minimum_ratio.to_string());

            if let Some(current) = req.current_ratio {
                effect
                    .parameters
                    .insert("current_ratio".to_string(), current.to_string());
            }

            if let Some(rwa) = req.risk_weighted_assets {
                effect
                    .parameters
                    .insert("risk_weighted_assets".to_string(), rwa.to_string());
            }

            if let Some(capital) = req.capital_amount {
                effect
                    .parameters
                    .insert("capital_amount".to_string(), capital.to_string());
            }

            let mut statute = Statute::new(&req.id, &req.name, effect)
                .with_jurisdiction(&report.metadata.jurisdiction);

            statute
                .effect
                .parameters
                .insert("bank_lei".to_string(), report.metadata.bank_lei.clone());
            statute.effect.parameters.insert(
                "reporting_date".to_string(),
                report.metadata.reporting_date.clone(),
            );
            statute
                .effect
                .parameters
                .insert("requirement_category".to_string(), "capital".to_string());

            // Add condition for minimum ratio
            statute = statute.with_precondition(Condition::Custom {
                description: format!(
                    "{}_ratio >= {}",
                    req.capital_type.to_lowercase(),
                    req.minimum_ratio
                ),
            });

            statutes.push(statute);
        }

        // Convert liquidity requirements to statutes
        for req in &report.liquidity_requirements {
            let mut effect = Effect::new(
                EffectType::Obligation,
                format!("Maintain {} >= {}%", req.liquidity_type, req.minimum_ratio),
            );

            effect
                .parameters
                .insert("liquidity_type".to_string(), req.liquidity_type.clone());
            effect
                .parameters
                .insert("minimum_ratio".to_string(), req.minimum_ratio.to_string());

            if let Some(current) = req.current_ratio {
                effect
                    .parameters
                    .insert("current_ratio".to_string(), current.to_string());
            }

            if let Some(hqla) = req.hqla {
                effect
                    .parameters
                    .insert("hqla".to_string(), hqla.to_string());
            }

            if let Some(outflows) = req.net_cash_outflows {
                effect
                    .parameters
                    .insert("net_cash_outflows".to_string(), outflows.to_string());
            }

            let mut statute = Statute::new(&req.id, &req.name, effect)
                .with_jurisdiction(&report.metadata.jurisdiction);

            statute
                .effect
                .parameters
                .insert("bank_lei".to_string(), report.metadata.bank_lei.clone());
            statute.effect.parameters.insert(
                "reporting_date".to_string(),
                report.metadata.reporting_date.clone(),
            );
            statute
                .effect
                .parameters
                .insert("requirement_category".to_string(), "liquidity".to_string());

            // Add condition for minimum ratio
            statute = statute.with_precondition(Condition::Custom {
                description: format!(
                    "{} >= {}",
                    req.liquidity_type.to_lowercase(),
                    req.minimum_ratio
                ),
            });

            statutes.push(statute);
        }

        conv_report.statutes_converted = statutes.len();
        Ok((statutes, conv_report))
    }

    fn validate(&self, source: &str) -> bool {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            value.get("metadata").is_some()
                && (value.get("capital_requirements").is_some()
                    || value.get("liquidity_requirements").is_some())
                && value
                    .get("metadata")
                    .and_then(|m| m.get("bank_lei"))
                    .is_some()
        } else {
            false
        }
    }
}

/// Basel III exporter
pub struct Basel3Exporter;

impl Basel3Exporter {
    /// Creates a new Basel III exporter
    pub fn new() -> Self {
        Self
    }
}

impl Default for Basel3Exporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for Basel3Exporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Basel3
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut conv_report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Basel3);

        let first_statute = statutes.first();

        let metadata = Basel3Metadata {
            report_id: first_statute
                .and_then(|s| s.effect.parameters.get("report_id"))
                .unwrap_or(&"BASEL3-REPORT-001".to_string())
                .clone(),
            bank_lei: first_statute
                .and_then(|s| s.effect.parameters.get("bank_lei"))
                .unwrap_or(&"BANK123456789012".to_string())
                .clone(),
            reporting_date: first_statute
                .and_then(|s| s.effect.parameters.get("reporting_date"))
                .unwrap_or(&"2024-12-31".to_string())
                .clone(),
            jurisdiction: first_statute
                .and_then(|s| s.jurisdiction.as_ref())
                .unwrap_or(&"Global".to_string())
                .clone(),
            supervisor: first_statute
                .and_then(|s| s.effect.parameters.get("supervisor"))
                .unwrap_or(&"BIS".to_string())
                .clone(),
        };

        let mut capital_requirements = Vec::new();
        let mut liquidity_requirements = Vec::new();

        for statute in statutes {
            let category = statute
                .effect
                .parameters
                .get("requirement_category")
                .map(|s| s.as_str())
                .unwrap_or("capital");

            match category {
                "capital" => {
                    if let (Some(capital_type), Some(minimum_ratio)) = (
                        statute.effect.parameters.get("capital_type"),
                        statute
                            .effect
                            .parameters
                            .get("minimum_ratio")
                            .and_then(|r| r.parse().ok()),
                    ) {
                        capital_requirements.push(CapitalRequirement {
                            id: statute.id.clone(),
                            name: statute.title.clone(),
                            capital_type: capital_type.clone(),
                            minimum_ratio,
                            current_ratio: statute
                                .effect
                                .parameters
                                .get("current_ratio")
                                .and_then(|r| r.parse().ok()),
                            risk_weighted_assets: statute
                                .effect
                                .parameters
                                .get("risk_weighted_assets")
                                .and_then(|r| r.parse().ok()),
                            capital_amount: statute
                                .effect
                                .parameters
                                .get("capital_amount")
                                .and_then(|r| r.parse().ok()),
                        });
                    }
                }
                "liquidity" => {
                    if let (Some(liquidity_type), Some(minimum_ratio)) = (
                        statute.effect.parameters.get("liquidity_type"),
                        statute
                            .effect
                            .parameters
                            .get("minimum_ratio")
                            .and_then(|r| r.parse().ok()),
                    ) {
                        liquidity_requirements.push(LiquidityRequirement {
                            id: statute.id.clone(),
                            name: statute.title.clone(),
                            liquidity_type: liquidity_type.clone(),
                            minimum_ratio,
                            current_ratio: statute
                                .effect
                                .parameters
                                .get("current_ratio")
                                .and_then(|r| r.parse().ok()),
                            hqla: statute
                                .effect
                                .parameters
                                .get("hqla")
                                .and_then(|r| r.parse().ok()),
                            net_cash_outflows: statute
                                .effect
                                .parameters
                                .get("net_cash_outflows")
                                .and_then(|r| r.parse().ok()),
                        });
                    }
                }
                _ => {}
            }
        }

        let report = Basel3Report {
            metadata,
            capital_requirements,
            liquidity_requirements,
        };

        let json = serde_json::to_string_pretty(&report).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize Basel III: {}", e))
        })?;

        conv_report.statutes_converted = statutes.len();
        Ok((json, conv_report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basel3_import() {
        let source = r#"{
            "metadata": {
                "report_id": "BASEL3-2024-Q4",
                "bank_lei": "12345678901234567890",
                "reporting_date": "2024-12-31",
                "jurisdiction": "EU",
                "supervisor": "ECB"
            },
            "capital_requirements": [
                {
                    "id": "cet1",
                    "name": "Common Equity Tier 1",
                    "capital_type": "CET1",
                    "minimum_ratio": 4.5,
                    "current_ratio": 12.5,
                    "risk_weighted_assets": 100000000.0,
                    "capital_amount": 12500000.0
                }
            ],
            "liquidity_requirements": [
                {
                    "id": "lcr",
                    "name": "Liquidity Coverage Ratio",
                    "liquidity_type": "LCR",
                    "minimum_ratio": 100.0,
                    "current_ratio": 125.0,
                    "hqla": 50000000.0,
                    "net_cash_outflows": 40000000.0
                }
            ]
        }"#;

        let importer = Basel3Importer::new();
        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(report.statutes_converted, 2);
        assert!(statutes.iter().any(|s| s.id == "cet1"));
        assert!(statutes.iter().any(|s| s.id == "lcr"));
    }

    #[test]
    fn test_basel3_export() {
        let mut capital_effect = Effect::new(
            EffectType::Obligation,
            "Maintain CET1 capital ratio >= 4.5%",
        );
        capital_effect
            .parameters
            .insert("capital_type".to_string(), "CET1".to_string());
        capital_effect
            .parameters
            .insert("minimum_ratio".to_string(), "4.5".to_string());
        capital_effect
            .parameters
            .insert("current_ratio".to_string(), "10.0".to_string());

        let mut capital_statute = Statute::new("cet1", "Common Equity Tier 1", capital_effect)
            .with_jurisdiction("Global");

        capital_statute
            .effect
            .parameters
            .insert("bank_lei".to_string(), "12345678901234567890".to_string());
        capital_statute
            .effect
            .parameters
            .insert("reporting_date".to_string(), "2024-12-31".to_string());
        capital_statute
            .effect
            .parameters
            .insert("requirement_category".to_string(), "capital".to_string());

        let mut liquidity_effect = Effect::new(EffectType::Obligation, "Maintain LCR >= 100%");
        liquidity_effect
            .parameters
            .insert("liquidity_type".to_string(), "LCR".to_string());
        liquidity_effect
            .parameters
            .insert("minimum_ratio".to_string(), "100.0".to_string());

        let mut liquidity_statute =
            Statute::new("lcr", "Liquidity Coverage Ratio", liquidity_effect)
                .with_jurisdiction("Global");

        liquidity_statute
            .effect
            .parameters
            .insert("bank_lei".to_string(), "12345678901234567890".to_string());
        liquidity_statute
            .effect
            .parameters
            .insert("reporting_date".to_string(), "2024-12-31".to_string());
        liquidity_statute
            .effect
            .parameters
            .insert("requirement_category".to_string(), "liquidity".to_string());

        let exporter = Basel3Exporter::new();
        let (output, report) = exporter
            .export(&[capital_statute, liquidity_statute])
            .unwrap();

        assert_eq!(report.statutes_converted, 2);
        assert!(output.contains("CET1"));
        assert!(output.contains("LCR"));
        assert!(output.contains("4.5"));
        assert!(output.contains("100"));
    }

    #[test]
    fn test_basel3_roundtrip() {
        let source = r#"{
            "metadata": {
                "report_id": "TEST",
                "bank_lei": "123",
                "reporting_date": "2024-12-31",
                "jurisdiction": "US",
                "supervisor": "FED"
            },
            "capital_requirements": [
                {
                    "id": "tier1",
                    "name": "Tier 1 Capital",
                    "capital_type": "Tier1",
                    "minimum_ratio": 6.0,
                    "current_ratio": null,
                    "risk_weighted_assets": null,
                    "capital_amount": null
                }
            ],
            "liquidity_requirements": []
        }"#;

        let importer = Basel3Importer::new();
        let exporter = Basel3Exporter::new();

        let (statutes, _) = importer.import(source).unwrap();
        let (output, _) = exporter.export(&statutes).unwrap();

        let (roundtrip_statutes, _) = importer.import(&output).unwrap();

        assert_eq!(statutes.len(), roundtrip_statutes.len());
        assert_eq!(statutes[0].id, roundtrip_statutes[0].id);
    }

    #[test]
    fn test_basel3_validate() {
        let importer = Basel3Importer::new();

        let valid = r#"{
            "metadata": {"bank_lei": "123"},
            "capital_requirements": []
        }"#;
        assert!(importer.validate(valid));

        let invalid = r#"{"random": "data"}"#;
        assert!(!importer.validate(invalid));
    }
}
