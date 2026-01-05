//! MiFID II (Markets in Financial Instruments Directive II) reporting format support.
//!
//! MiFID II is an EU legislative framework for investment intermediaries.
//! This module supports reporting requirements including:
//! - Transaction reporting
//! - Best execution reporting
//! - Client categorization
//! - Product governance

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MiFID II report document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiFID2Report {
    /// Report metadata
    pub metadata: MiFID2Metadata,
    /// Transaction reports
    pub transactions: Vec<MiFID2Transaction>,
    /// Best execution reports
    pub best_execution: Vec<MiFID2BestExecution>,
}

/// MiFID II metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiFID2Metadata {
    /// Report ID
    pub report_id: String,
    /// Reporting entity LEI (Legal Entity Identifier)
    pub entity_lei: String,
    /// Reporting period
    pub period: String,
    /// Submission date
    pub submission_date: String,
    /// Competent authority
    pub competent_authority: String,
}

/// MiFID II transaction report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiFID2Transaction {
    /// Transaction ID
    pub transaction_id: String,
    /// Trading date and time
    pub trading_datetime: String,
    /// Instrument identifier (ISIN)
    pub instrument_id: String,
    /// Buy/Sell indicator
    pub buy_sell: String,
    /// Quantity
    pub quantity: f64,
    /// Price
    pub price: f64,
    /// Venue (trading venue MIC code)
    pub venue: String,
    /// Client LEI
    pub client_lei: Option<String>,
    /// Additional fields
    pub additional_fields: HashMap<String, String>,
}

/// MiFID II best execution report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiFID2BestExecution {
    /// Report ID
    pub report_id: String,
    /// Instrument class
    pub instrument_class: String,
    /// Top execution venues
    pub top_venues: Vec<String>,
    /// Execution quality metrics
    pub quality_metrics: HashMap<String, f64>,
}

/// MiFID II importer
pub struct MiFID2Importer;

impl MiFID2Importer {
    /// Creates a new MiFID II importer
    pub fn new() -> Self {
        Self
    }

    fn parse_mifid2(&self, source: &str) -> InteropResult<MiFID2Report> {
        serde_json::from_str(source)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse MiFID II JSON: {}", e)))
    }
}

impl Default for MiFID2Importer {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for MiFID2Importer {
    fn format(&self) -> LegalFormat {
        LegalFormat::MiFID2
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let report = self.parse_mifid2(source)?;
        let mut statutes = Vec::new();
        let mut conv_report = ConversionReport::new(LegalFormat::MiFID2, LegalFormat::Legalis);

        // Convert transactions to statutes
        for tx in &report.transactions {
            let mut effect = Effect::new(
                EffectType::Obligation,
                format!("Report transaction {}", tx.transaction_id),
            );

            effect
                .parameters
                .insert("transaction_id".to_string(), tx.transaction_id.clone());
            effect
                .parameters
                .insert("instrument_id".to_string(), tx.instrument_id.clone());
            effect
                .parameters
                .insert("buy_sell".to_string(), tx.buy_sell.clone());
            effect
                .parameters
                .insert("quantity".to_string(), tx.quantity.to_string());
            effect
                .parameters
                .insert("price".to_string(), tx.price.to_string());
            effect
                .parameters
                .insert("venue".to_string(), tx.venue.clone());

            let mut statute = Statute::new(
                format!("tx_{}", tx.transaction_id),
                format!("Transaction Report {}", tx.transaction_id),
                effect,
            );

            statute
                .effect
                .parameters
                .insert("entity_lei".to_string(), report.metadata.entity_lei.clone());
            statute
                .effect
                .parameters
                .insert("trading_datetime".to_string(), tx.trading_datetime.clone());
            statute
                .effect
                .parameters
                .insert("report_type".to_string(), "transaction".to_string());

            if let Some(client_lei) = &tx.client_lei {
                statute
                    .effect
                    .parameters
                    .insert("client_lei".to_string(), client_lei.clone());
            }

            statutes.push(statute);
        }

        // Convert best execution reports to statutes
        for be in &report.best_execution {
            let mut effect = Effect::new(
                EffectType::Obligation,
                format!("Best execution report for {}", be.instrument_class),
            );

            effect
                .parameters
                .insert("instrument_class".to_string(), be.instrument_class.clone());
            effect
                .parameters
                .insert("top_venues".to_string(), be.top_venues.join(","));

            for (metric, value) in &be.quality_metrics {
                effect.parameters.insert(metric.clone(), value.to_string());
            }

            let mut statute = Statute::new(
                format!("be_{}", be.report_id),
                format!("Best Execution: {}", be.instrument_class),
                effect,
            );

            statute
                .effect
                .parameters
                .insert("entity_lei".to_string(), report.metadata.entity_lei.clone());
            statute
                .effect
                .parameters
                .insert("report_type".to_string(), "best_execution".to_string());

            statutes.push(statute);
        }

        conv_report.statutes_converted = statutes.len();
        Ok((statutes, conv_report))
    }

    fn validate(&self, source: &str) -> bool {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            value.get("metadata").is_some()
                && (value.get("transactions").is_some() || value.get("best_execution").is_some())
                && value
                    .get("metadata")
                    .and_then(|m| m.get("entity_lei"))
                    .is_some()
        } else {
            false
        }
    }
}

/// MiFID II exporter
pub struct MiFID2Exporter;

impl MiFID2Exporter {
    /// Creates a new MiFID II exporter
    pub fn new() -> Self {
        Self
    }
}

impl Default for MiFID2Exporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for MiFID2Exporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::MiFID2
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut conv_report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::MiFID2);

        let first_statute = statutes.first();

        let metadata = MiFID2Metadata {
            report_id: first_statute
                .and_then(|s| s.effect.parameters.get("report_id"))
                .unwrap_or(&"MIFID2-REPORT-001".to_string())
                .clone(),
            entity_lei: first_statute
                .and_then(|s| s.effect.parameters.get("entity_lei"))
                .unwrap_or(&"ENTITY123456789012".to_string())
                .clone(),
            period: first_statute
                .and_then(|s| s.effect.parameters.get("period"))
                .unwrap_or(&"2024-Q1".to_string())
                .clone(),
            submission_date: first_statute
                .and_then(|s| s.effect.parameters.get("submission_date"))
                .unwrap_or(&"2024-04-30".to_string())
                .clone(),
            competent_authority: first_statute
                .and_then(|s| s.effect.parameters.get("competent_authority"))
                .unwrap_or(&"FCA".to_string())
                .clone(),
        };

        let mut transactions = Vec::new();
        let mut best_execution = Vec::new();

        for statute in statutes {
            let report_type = statute
                .effect
                .parameters
                .get("report_type")
                .map(|s| s.as_str())
                .unwrap_or("transaction");

            match report_type {
                "transaction" => {
                    if let (
                        Some(tx_id),
                        Some(instrument),
                        Some(bs),
                        Some(qty),
                        Some(price),
                        Some(venue),
                    ) = (
                        statute.effect.parameters.get("transaction_id"),
                        statute.effect.parameters.get("instrument_id"),
                        statute.effect.parameters.get("buy_sell"),
                        statute
                            .effect
                            .parameters
                            .get("quantity")
                            .and_then(|q| q.parse().ok()),
                        statute
                            .effect
                            .parameters
                            .get("price")
                            .and_then(|p| p.parse().ok()),
                        statute.effect.parameters.get("venue"),
                    ) {
                        transactions.push(MiFID2Transaction {
                            transaction_id: tx_id.clone(),
                            trading_datetime: statute
                                .effect
                                .parameters
                                .get("trading_datetime")
                                .unwrap_or(&"2024-01-01T12:00:00Z".to_string())
                                .clone(),
                            instrument_id: instrument.clone(),
                            buy_sell: bs.clone(),
                            quantity: qty,
                            price,
                            venue: venue.clone(),
                            client_lei: statute.effect.parameters.get("client_lei").cloned(),
                            additional_fields: HashMap::new(),
                        });
                    }
                }
                "best_execution" => {
                    if let (Some(instrument_class), Some(venues)) = (
                        statute.effect.parameters.get("instrument_class"),
                        statute.effect.parameters.get("top_venues"),
                    ) {
                        let quality_metrics: HashMap<String, f64> = statute
                            .effect
                            .parameters
                            .iter()
                            .filter(|(k, _)| *k != "instrument_class" && *k != "top_venues")
                            .filter_map(|(k, v)| v.parse().ok().map(|v| (k.clone(), v)))
                            .collect();

                        best_execution.push(MiFID2BestExecution {
                            report_id: statute.id.clone(),
                            instrument_class: instrument_class.clone(),
                            top_venues: venues.split(',').map(|s| s.to_string()).collect(),
                            quality_metrics,
                        });
                    }
                }
                _ => {}
            }
        }

        let report = MiFID2Report {
            metadata,
            transactions,
            best_execution,
        };

        let json = serde_json::to_string_pretty(&report).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize MiFID II: {}", e))
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
    fn test_mifid2_import() {
        let source = r#"{
            "metadata": {
                "report_id": "MIFID2-001",
                "entity_lei": "123456789012ABCDEF",
                "period": "2024-Q1",
                "submission_date": "2024-04-30",
                "competent_authority": "FCA"
            },
            "transactions": [
                {
                    "transaction_id": "TX001",
                    "trading_datetime": "2024-01-15T10:30:00Z",
                    "instrument_id": "GB0002374006",
                    "buy_sell": "BUY",
                    "quantity": 1000.0,
                    "price": 125.50,
                    "venue": "XLON",
                    "client_lei": "CLIENT123",
                    "additional_fields": {}
                }
            ],
            "best_execution": []
        }"#;

        let importer = MiFID2Importer::new();
        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert!(statutes[0].id.contains("TX001"));
    }

    #[test]
    fn test_mifid2_export() {
        let mut effect = Effect::new(EffectType::Obligation, "Report transaction TX002");
        effect
            .parameters
            .insert("transaction_id".to_string(), "TX002".to_string());
        effect
            .parameters
            .insert("instrument_id".to_string(), "US0378331005".to_string());
        effect
            .parameters
            .insert("buy_sell".to_string(), "SELL".to_string());
        effect
            .parameters
            .insert("quantity".to_string(), "500".to_string());
        effect
            .parameters
            .insert("price".to_string(), "150.25".to_string());
        effect
            .parameters
            .insert("venue".to_string(), "XNYS".to_string());

        let mut statute = Statute::new("tx_TX002", "Transaction Report TX002", effect);

        statute
            .effect
            .parameters
            .insert("entity_lei".to_string(), "123456789012ABCDEF".to_string());
        statute.effect.parameters.insert(
            "trading_datetime".to_string(),
            "2024-02-01T14:00:00Z".to_string(),
        );
        statute
            .effect
            .parameters
            .insert("report_type".to_string(), "transaction".to_string());

        let exporter = MiFID2Exporter::new();
        let (output, report) = exporter.export(&[statute]).unwrap();

        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("TX002"));
        assert!(output.contains("US0378331005"));
    }

    #[test]
    fn test_mifid2_validate() {
        let importer = MiFID2Importer::new();

        let valid = r#"{
            "metadata": {"entity_lei": "123"},
            "transactions": []
        }"#;
        assert!(importer.validate(valid));

        let invalid = r#"{"random": "data"}"#;
        assert!(!importer.validate(invalid));
    }
}
