//! CSV import and export for statute, dependency and population data.
//!
//! The parser follows RFC 4180 conventions: fields may be quoted with double
//! quotes, an embedded quote is escaped by doubling it (`""`), and quoted fields
//! may contain the delimiter and line breaks. Both `\n` and `\r\n` line endings
//! are accepted. The writer applies the inverse, quoting only fields that need
//! it so that round trips are stable and human-readable.

use std::collections::HashMap;

use super::{effect_type_label, parse_effect_type};
use crate::functions::VizResult;
use crate::types_4::DependencyGraph;
use crate::types_5::{PopulationChart, VizError};
use legalis_core::{Effect, Statute};

/// Dialect controlling how CSV text is parsed.
#[derive(Debug, Clone)]
pub struct CsvDialect {
    /// Field delimiter (default `,`).
    pub delimiter: char,
    /// Whether the first non-blank record is a header row used for column
    /// mapping.
    pub has_header: bool,
}

impl CsvDialect {
    /// Creates a comma-separated dialect with a header row.
    pub fn new() -> Self {
        Self {
            delimiter: ',',
            has_header: true,
        }
    }

    /// Creates a tab-separated dialect with a header row.
    pub fn tab_separated() -> Self {
        Self {
            delimiter: '\t',
            has_header: true,
        }
    }

    /// Sets the field delimiter.
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Marks the input as having no header row (positional columns).
    pub fn without_header(mut self) -> Self {
        self.has_header = false;
        self
    }
}

impl Default for CsvDialect {
    fn default() -> Self {
        Self::new()
    }
}

/// Imports statute and dependency data from CSV text.
#[derive(Debug, Clone)]
pub struct CsvImporter {
    dialect: CsvDialect,
}

impl CsvImporter {
    /// Creates a new importer with the default (comma, header) dialect.
    pub fn new() -> Self {
        Self {
            dialect: CsvDialect::new(),
        }
    }

    /// Overrides the CSV dialect.
    pub fn with_dialect(mut self, dialect: CsvDialect) -> Self {
        self.dialect = dialect;
        self
    }

    /// Imports a list of [`Statute`]s from CSV text.
    ///
    /// A header row is required to map columns. Recognized headers (case- and
    /// space-insensitive) are: `id` and `title` (both required), plus the
    /// optional `effect_type`, `effect_description` (or `description`),
    /// `jurisdiction`, `version` and `discretion_logic` (or `discretion`).
    /// Unknown columns are ignored. Blank rows are skipped.
    ///
    /// # Errors
    ///
    /// Returns [`VizError::InvalidStructure`] when the header is missing, a
    /// required column is absent, an `id` cell is empty or a `version` cell is
    /// not a non-negative integer.
    pub fn import_statutes(&self, csv: &str) -> VizResult<Vec<Statute>> {
        if !self.dialect.has_header {
            return Err(VizError::InvalidStructure(
                "statute CSV import requires a header row to map columns".to_string(),
            ));
        }
        let records = parse_csv(csv, self.dialect.delimiter)?;
        let mut rows = records.into_iter().filter(|r| !is_blank_record(r));
        let header = match rows.next() {
            Some(header) => header,
            None => return Ok(Vec::new()),
        };
        let columns = header_index_map(&header);
        let id_col = columns.get("id").copied().ok_or_else(|| {
            VizError::InvalidStructure("statute CSV missing required 'id' column".to_string())
        })?;
        let title_col = columns.get("title").copied().ok_or_else(|| {
            VizError::InvalidStructure("statute CSV missing required 'title' column".to_string())
        })?;
        let effect_type_col = columns.get("effect_type").copied();
        let description_col = columns
            .get("effect_description")
            .or_else(|| columns.get("description"))
            .copied();
        let jurisdiction_col = columns.get("jurisdiction").copied();
        let version_col = columns.get("version").copied();
        let discretion_col = columns
            .get("discretion_logic")
            .or_else(|| columns.get("discretion"))
            .copied();

        let mut statutes = Vec::new();
        for (offset, row) in rows.enumerate() {
            let row_number = offset + 1;
            let id = field_at(&row, id_col).trim();
            if id.is_empty() {
                return Err(VizError::InvalidStructure(format!(
                    "statute CSV data row {row_number} has an empty 'id'"
                )));
            }
            let title = field_at(&row, title_col).to_string();
            let effect_type = effect_type_col
                .map(|c| parse_effect_type(field_at(&row, c)))
                .unwrap_or(legalis_core::EffectType::Grant);
            let description = description_col
                .map(|c| field_at(&row, c).to_string())
                .unwrap_or_default();
            let mut statute = Statute::new(id, title, Effect::new(effect_type, description));
            if let Some(col) = jurisdiction_col {
                let jurisdiction = field_at(&row, col).trim();
                if !jurisdiction.is_empty() {
                    statute = statute.with_jurisdiction(jurisdiction);
                }
            }
            if let Some(col) = version_col {
                let raw = field_at(&row, col).trim();
                if !raw.is_empty() {
                    let version = raw.parse::<u32>().map_err(|_| {
                        VizError::InvalidStructure(format!(
                            "statute CSV data row {row_number} has an invalid 'version': '{raw}'"
                        ))
                    })?;
                    statute = statute.with_version(version);
                }
            }
            if let Some(col) = discretion_col {
                let discretion = field_at(&row, col).trim();
                if !discretion.is_empty() {
                    statute = statute.with_discretion(discretion);
                }
            }
            statutes.push(statute);
        }
        Ok(statutes)
    }

    /// Imports a [`DependencyGraph`] from CSV edge data.
    ///
    /// With a header, the columns `from` (or `source`) and `to` (or `target`)
    /// are required and `relation` (or `label`) is optional. Without a header,
    /// columns are positional: `from, to, relation`. Rows with an empty `from`
    /// or `to` are skipped; a missing relation defaults to `depends_on`.
    ///
    /// # Errors
    ///
    /// Returns [`VizError::InvalidStructure`] when a required header column is
    /// absent or a quoted field is unterminated.
    pub fn import_dependencies(&self, csv: &str) -> VizResult<DependencyGraph> {
        let records = parse_csv(csv, self.dialect.delimiter)?;
        let mut rows = records.into_iter().filter(|r| !is_blank_record(r));
        let mut graph = DependencyGraph::new();
        if self.dialect.has_header {
            let header = match rows.next() {
                Some(header) => header,
                None => return Ok(graph),
            };
            let columns = header_index_map(&header);
            let from_col = columns
                .get("from")
                .or_else(|| columns.get("source"))
                .copied()
                .ok_or_else(|| {
                    VizError::InvalidStructure(
                        "dependency CSV missing required 'from' column".to_string(),
                    )
                })?;
            let to_col = columns
                .get("to")
                .or_else(|| columns.get("target"))
                .copied()
                .ok_or_else(|| {
                    VizError::InvalidStructure(
                        "dependency CSV missing required 'to' column".to_string(),
                    )
                })?;
            let relation_col = columns
                .get("relation")
                .or_else(|| columns.get("label"))
                .copied();
            for row in rows {
                add_edge_row(&mut graph, &row, from_col, to_col, relation_col);
            }
        } else {
            for row in rows {
                add_edge_row(&mut graph, &row, 0, 1, Some(2));
            }
        }
        Ok(graph)
    }
}

impl Default for CsvImporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Exports statute, dependency and population data to CSV text.
#[derive(Debug, Clone)]
pub struct CsvExporter {
    delimiter: char,
}

impl CsvExporter {
    /// Creates a new comma-separated exporter.
    pub fn new() -> Self {
        Self { delimiter: ',' }
    }

    /// Sets the field delimiter.
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Serializes statutes to CSV using columns that [`CsvImporter`] can read
    /// back, enabling a lossless-for-core-fields round trip.
    pub fn statutes_to_csv(&self, statutes: &[Statute]) -> String {
        let mut out = String::new();
        self.write_row(
            &mut out,
            [
                "id",
                "title",
                "effect_type",
                "effect_description",
                "jurisdiction",
                "version",
                "discretion_logic",
            ]
            .into_iter()
            .map(str::to_string),
        );
        for statute in statutes {
            self.write_row(
                &mut out,
                [
                    statute.id.clone(),
                    statute.title.clone(),
                    effect_type_label(&statute.effect.effect_type).to_string(),
                    statute.effect.description.clone(),
                    statute.jurisdiction.clone().unwrap_or_default(),
                    statute.version.to_string(),
                    statute.discretion_logic.clone().unwrap_or_default(),
                ]
                .into_iter(),
            );
        }
        out
    }

    /// Serializes a dependency graph's edges to CSV (`from,to,relation`).
    pub fn dependencies_to_csv(&self, graph: &DependencyGraph) -> String {
        let mut out = String::new();
        self.write_row(
            &mut out,
            ["from", "to", "relation"].into_iter().map(str::to_string),
        );
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                let from = graph.graph.node_weight(source).cloned().unwrap_or_default();
                let to = graph.graph.node_weight(target).cloned().unwrap_or_default();
                let relation = graph.graph.edge_weight(edge).cloned().unwrap_or_default();
                self.write_row(&mut out, [from, to, relation].into_iter());
            }
        }
        out
    }

    /// Serializes a population chart's distribution to CSV
    /// (`category,count,percentage`).
    pub fn population_chart_to_csv(&self, chart: &PopulationChart) -> String {
        let mut out = String::new();
        self.write_row(
            &mut out,
            ["category", "count", "percentage"]
                .into_iter()
                .map(str::to_string),
        );
        for point in &chart.data {
            let percentage = point
                .percentage
                .map(|value| format!("{value:.4}"))
                .unwrap_or_default();
            self.write_row(
                &mut out,
                [point.category.clone(), point.count.to_string(), percentage].into_iter(),
            );
        }
        out
    }

    fn write_row<I: Iterator<Item = String>>(&self, out: &mut String, fields: I) {
        let mut first = true;
        for field in fields {
            if !first {
                out.push(self.delimiter);
            }
            first = false;
            out.push_str(&self.escape_field(&field));
        }
        out.push('\n');
    }

    fn escape_field(&self, field: &str) -> String {
        let needs_quoting = field.contains(self.delimiter)
            || field.contains('"')
            || field.contains('\n')
            || field.contains('\r');
        if needs_quoting {
            format!("\"{}\"", field.replace('"', "\"\""))
        } else {
            field.to_string()
        }
    }
}

impl Default for CsvExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses CSV text into records of string fields.
///
/// # Errors
///
/// Returns [`VizError::InvalidStructure`] if the input ends with an unterminated
/// quoted field.
pub(crate) fn parse_csv(input: &str, delimiter: char) -> VizResult<Vec<Vec<String>>> {
    let chars: Vec<char> = input.chars().collect();
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut record: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut pending = false;
    let mut index = 0;
    while index < chars.len() {
        let current = chars[index];
        index += 1;
        if in_quotes {
            if current == '"' {
                if chars.get(index) == Some(&'"') {
                    field.push('"');
                    index += 1;
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(current);
            }
        } else if current == '"' {
            in_quotes = true;
            pending = true;
        } else if current == delimiter {
            record.push(std::mem::take(&mut field));
            pending = true;
        } else if current == '\n' || current == '\r' {
            if current == '\r' && chars.get(index) == Some(&'\n') {
                index += 1;
            }
            record.push(std::mem::take(&mut field));
            records.push(std::mem::take(&mut record));
            pending = false;
        } else {
            field.push(current);
            pending = true;
        }
    }
    if in_quotes {
        return Err(VizError::InvalidStructure(
            "unterminated quoted field in CSV input".to_string(),
        ));
    }
    if pending || !field.is_empty() || !record.is_empty() {
        record.push(field);
        records.push(record);
    }
    Ok(records)
}

fn add_edge_row(
    graph: &mut DependencyGraph,
    row: &[String],
    from_col: usize,
    to_col: usize,
    relation_col: Option<usize>,
) {
    let from = field_at(row, from_col).trim();
    let to = field_at(row, to_col).trim();
    if from.is_empty() || to.is_empty() {
        return;
    }
    let relation = relation_col
        .map(|col| field_at(row, col).trim())
        .filter(|value| !value.is_empty())
        .unwrap_or("depends_on");
    graph.add_dependency(from, to, relation);
}

fn is_blank_record(record: &[String]) -> bool {
    record.iter().all(|field| field.trim().is_empty())
}

fn header_index_map(header: &[String]) -> HashMap<String, usize> {
    let mut map = HashMap::with_capacity(header.len());
    for (index, name) in header.iter().enumerate() {
        let key = name.trim().to_ascii_lowercase().replace(' ', "_");
        map.entry(key).or_insert(index);
    }
    map
}

fn field_at(record: &[String], index: usize) -> &str {
    record.get(index).map(String::as_str).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::EffectType;

    #[test]
    fn imports_statutes_with_all_columns() {
        let csv = "id,title,effect_type,effect_description,jurisdiction,version,discretion_logic\n\
                   s1,First Law,Grant,Grants a right,US,3,judge decides\n\
                   s2,Second Law,Prohibition,Forbids action,,1,\n";
        let importer = CsvImporter::new();
        let statutes = importer
            .import_statutes(csv)
            .expect("import should succeed");
        assert_eq!(statutes.len(), 2);
        assert_eq!(statutes[0].id, "s1");
        assert_eq!(statutes[0].title, "First Law");
        assert_eq!(statutes[0].effect.effect_type, EffectType::Grant);
        assert_eq!(statutes[0].jurisdiction.as_deref(), Some("US"));
        assert_eq!(statutes[0].version, 3);
        assert_eq!(
            statutes[0].discretion_logic.as_deref(),
            Some("judge decides")
        );
        // Empty optional cells leave defaults / None.
        assert_eq!(statutes[1].effect.effect_type, EffectType::Prohibition);
        assert!(statutes[1].jurisdiction.is_none());
        assert_eq!(statutes[1].version, 1);
        assert!(statutes[1].discretion_logic.is_none());
    }

    #[test]
    fn imports_quoted_fields_with_commas_quotes_and_newlines() {
        let csv = "title,id\n\
                   \"Tax, Levy and \"\"Duty\"\"\",s9\n\
                   \"multi\nline\",s10\n";
        let importer = CsvImporter::new();
        let statutes = importer
            .import_statutes(csv)
            .expect("import should succeed");
        assert_eq!(statutes.len(), 2);
        assert_eq!(statutes[0].title, "Tax, Levy and \"Duty\"");
        assert_eq!(statutes[0].id, "s9");
        assert_eq!(statutes[1].title, "multi\nline");
        assert_eq!(statutes[1].id, "s10");
    }

    #[test]
    fn missing_required_column_is_an_error() {
        let csv = "name,effect_type\nfoo,Grant\n";
        let importer = CsvImporter::new();
        let result = importer.import_statutes(csv);
        assert!(matches!(result, Err(VizError::InvalidStructure(_))));
    }

    #[test]
    fn unterminated_quote_is_an_error() {
        let csv = "id,title\ns1,\"unterminated\n";
        let importer = CsvImporter::new();
        let result = importer.import_statutes(csv);
        assert!(matches!(result, Err(VizError::InvalidStructure(_))));
    }

    #[test]
    fn invalid_version_is_an_error() {
        let csv = "id,title,version\ns1,Law,not-a-number\n";
        let importer = CsvImporter::new();
        let result = importer.import_statutes(csv);
        assert!(matches!(result, Err(VizError::InvalidStructure(_))));
    }

    #[test]
    fn imports_dependencies_with_header_and_default_relation() {
        let csv = "from,to,relation\na,b,requires\nc,d,\n";
        let importer = CsvImporter::new();
        let graph = importer
            .import_dependencies(csv)
            .expect("import should succeed");
        assert_eq!(graph.node_count(), 4);
        let csv_back = CsvExporter::new().dependencies_to_csv(&graph);
        assert!(csv_back.contains("a,b,requires"));
        assert!(csv_back.contains("c,d,depends_on"));
    }

    #[test]
    fn imports_dependencies_positional_without_header() {
        let csv = "a,b\nb,c,supersedes\n";
        let importer = CsvImporter::new().with_dialect(CsvDialect::new().without_header());
        let graph = importer
            .import_dependencies(csv)
            .expect("import should succeed");
        assert_eq!(graph.node_count(), 3);
    }

    #[test]
    fn statutes_round_trip_through_csv() {
        let original = vec![
            Statute::new("s1", "First, Law", Effect::new(EffectType::Grant, "Grants"))
                .with_jurisdiction("US")
                .with_version(2),
            Statute::new(
                "s2",
                "Second \"quoted\" Law",
                Effect::new(EffectType::MonetaryTransfer, "Pays"),
            )
            .with_discretion("court discretion"),
        ];
        let csv = CsvExporter::new().statutes_to_csv(&original);
        let reimported = CsvImporter::new()
            .import_statutes(&csv)
            .expect("reimport should succeed");
        assert_eq!(reimported.len(), 2);
        assert_eq!(reimported[0].title, "First, Law");
        assert_eq!(reimported[0].jurisdiction.as_deref(), Some("US"));
        assert_eq!(reimported[0].version, 2);
        assert_eq!(reimported[1].title, "Second \"quoted\" Law");
        assert_eq!(
            reimported[1].effect.effect_type,
            EffectType::MonetaryTransfer
        );
        assert_eq!(
            reimported[1].discretion_logic.as_deref(),
            Some("court discretion")
        );
    }

    #[test]
    fn population_chart_export_has_header_and_rows() {
        let mut chart = PopulationChart::new("Eligibility");
        chart.add_data("Eligible", 150);
        chart.add_data("Ineligible", 50);
        chart.calculate_percentages();
        let csv = CsvExporter::new().population_chart_to_csv(&chart);
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines[0], "category,count,percentage");
        assert!(lines[1].starts_with("Eligible,150,"));
        assert_eq!(lines.len(), 3);
    }
}
