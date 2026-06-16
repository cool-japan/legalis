//! Data exchange: import and export of statute and visualization data in
//! portable interchange formats.
//!
//! Visualizations are only as useful as the data feeding them, and that data
//! rarely originates inside this crate. This module bridges the gap with a set
//! of dependency-free, round-trip-friendly readers and writers that operate
//! directly on the crate's existing model types ([`Statute`], [`DependencyGraph`],
//! [`Timeline`] and [`PopulationChart`]):
//!
//! - [`CsvImporter`] / [`CsvExporter`] read and write tabular statute,
//!   dependency and population data using an RFC-4180 style parser that handles
//!   quoting, escaped quotes and embedded newlines (see [`csv_io`]).
//! - [`SpreadsheetExporter`] emits Office XML SpreadsheetML 2003 workbooks that
//!   open natively in Excel / LibreOffice without any binary `.xlsx` packaging,
//!   so chart data can be handed to spreadsheet tools (see [`spreadsheet`]).
//! - [`JsonLdExporter`] produces JSON-LD linked-data documents with an explicit
//!   `@context` and `@graph`, suitable for semantic-web ingestion (see
//!   [`json_ld`]).
//! - [`XmlExporter`] writes well-formed, indented XML for generic
//!   interoperability (see [`xml_export`]).
//! - [`SqliteExporter`] emits portable SQL text (DDL + `INSERT`s) that loads
//!   into SQLite via `sqlite3 db.sqlite < export.sql`, avoiding a native driver
//!   dependency (see [`sqlite_export`]).
//!
//! All exporters share a single canonical [`EffectType`] vocabulary
//! ([`effect_type_label`] / [`parse_effect_type`]) so that data survives a
//! round trip through any combination of these formats.

mod csv_io;
mod json_ld;
mod spreadsheet;
mod sqlite_export;
mod xml_export;

pub use csv_io::{CsvDialect, CsvExporter, CsvImporter};
pub use json_ld::JsonLdExporter;
pub use spreadsheet::{Cell, CellValue, SpreadsheetExporter, Worksheet};
pub use sqlite_export::SqliteExporter;
pub use xml_export::XmlExporter;

use crate::types_5::TimelineEvent;
use legalis_core::EffectType;

/// Returns the canonical, round-trip-stable label for an [`EffectType`].
///
/// The labels use `CamelCase` matching the Rust variant names so that a value
/// exported by any writer here can be read back by [`parse_effect_type`].
pub(crate) fn effect_type_label(effect_type: &EffectType) -> &'static str {
    match effect_type {
        EffectType::Grant => "Grant",
        EffectType::Revoke => "Revoke",
        EffectType::Obligation => "Obligation",
        EffectType::Prohibition => "Prohibition",
        EffectType::MonetaryTransfer => "MonetaryTransfer",
        EffectType::StatusChange => "StatusChange",
        EffectType::Custom => "Custom",
    }
}

/// Parses an [`EffectType`] from a label, leniently.
///
/// Matching ignores case, whitespace and punctuation, so `"Grant"`, `"GRANT"`,
/// `"monetary_transfer"` and `"Monetary Transfer"` are all accepted. Unknown
/// labels fall back to [`EffectType::Custom`] so import never fails on an
/// unfamiliar effect kind.
pub(crate) fn parse_effect_type(raw: &str) -> EffectType {
    let normalized: String = raw
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    match normalized.as_str() {
        "grant" => EffectType::Grant,
        "revoke" => EffectType::Revoke,
        "obligation" => EffectType::Obligation,
        "prohibition" => EffectType::Prohibition,
        "monetarytransfer" => EffectType::MonetaryTransfer,
        "statuschange" => EffectType::StatusChange,
        _ => EffectType::Custom,
    }
}

/// Escapes a string for safe inclusion in XML element text or attribute values.
pub(crate) fn escape_xml(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Escapes a string for inclusion inside a single-quoted SQL string literal by
/// doubling embedded single quotes (the SQL-standard escape).
pub(crate) fn escape_sql_string(value: &str) -> String {
    value.replace('\'', "''")
}

/// Decomposes a [`TimelineEvent`] into `(type_label, statute_id, detail)`.
///
/// Shared by every exporter that flattens timelines into rows/records so that
/// the event taxonomy stays consistent across formats.
pub(crate) fn timeline_event_parts(event: &TimelineEvent) -> (&'static str, &str, Option<&str>) {
    match event {
        TimelineEvent::Enacted { statute_id, title } => ("Enacted", statute_id, Some(title)),
        TimelineEvent::Amended {
            statute_id,
            description,
        } => ("Amended", statute_id, Some(description)),
        TimelineEvent::Repealed { statute_id } => ("Repealed", statute_id, None),
        TimelineEvent::EffectiveStart { statute_id } => ("EffectiveStart", statute_id, None),
        TimelineEvent::EffectiveEnd { statute_id } => ("EffectiveEnd", statute_id, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::EffectType;

    #[test]
    fn effect_type_label_round_trips_through_parse() {
        let kinds = [
            EffectType::Grant,
            EffectType::Revoke,
            EffectType::Obligation,
            EffectType::Prohibition,
            EffectType::MonetaryTransfer,
            EffectType::StatusChange,
            EffectType::Custom,
        ];
        for kind in kinds {
            let label = effect_type_label(&kind);
            assert_eq!(parse_effect_type(label), kind);
        }
    }

    #[test]
    fn parse_effect_type_is_lenient() {
        assert_eq!(parse_effect_type("GRANT"), EffectType::Grant);
        assert_eq!(
            parse_effect_type("monetary_transfer"),
            EffectType::MonetaryTransfer
        );
        assert_eq!(parse_effect_type("Status Change"), EffectType::StatusChange);
        assert_eq!(parse_effect_type("nonsense"), EffectType::Custom);
    }

    #[test]
    fn escape_xml_replaces_all_special_characters() {
        let escaped = escape_xml("a<b>&\"'");
        assert_eq!(escaped, "a&lt;b&gt;&amp;&quot;&apos;");
    }

    #[test]
    fn escape_sql_string_doubles_single_quotes() {
        assert_eq!(escape_sql_string("O'Brien's"), "O''Brien''s");
        assert_eq!(escape_sql_string("plain"), "plain");
    }
}
