//! Excel-compatible export via the Office XML SpreadsheetML 2003 format.
//!
//! SpreadsheetML 2003 is a single, self-contained XML document that Excel,
//! LibreOffice Calc and Google Sheets all open natively as a workbook. Unlike
//! the modern `.xlsx` format it requires no ZIP packaging, so chart and statute
//! data can be handed to spreadsheet tools without pulling in a binary
//! spreadsheet dependency. Each [`Worksheet`] becomes a tab in the workbook.

use super::{effect_type_label, escape_xml, timeline_event_parts};
use crate::types_3::Timeline;
use crate::types_4::DependencyGraph;
use crate::types_5::PopulationChart;
use legalis_core::Statute;

/// A single typed spreadsheet cell value.
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    /// Textual content, emitted as `ss:Type="String"`.
    Text(String),
    /// Numeric content, emitted as `ss:Type="Number"`.
    Number(f64),
}

/// A spreadsheet cell with an optional bold "header" style.
#[derive(Debug, Clone)]
pub struct Cell {
    /// The cell's value.
    pub value: CellValue,
    /// Whether the cell uses the bold header style.
    pub header: bool,
}

impl Cell {
    /// Creates a text cell.
    pub fn text(value: impl Into<String>) -> Self {
        Self {
            value: CellValue::Text(value.into()),
            header: false,
        }
    }

    /// Creates a numeric cell.
    pub fn number(value: f64) -> Self {
        Self {
            value: CellValue::Number(value),
            header: false,
        }
    }

    /// Creates a bold header text cell.
    pub fn header(value: impl Into<String>) -> Self {
        Self {
            value: CellValue::Text(value.into()),
            header: true,
        }
    }
}

/// A worksheet: a named grid of rows of [`Cell`]s.
#[derive(Debug, Clone)]
pub struct Worksheet {
    /// Worksheet (tab) name.
    pub name: String,
    /// Rows of cells.
    pub rows: Vec<Vec<Cell>>,
}

impl Worksheet {
    /// Creates an empty worksheet with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rows: Vec::new(),
        }
    }

    /// Appends a row of cells.
    pub fn add_row(&mut self, row: Vec<Cell>) -> &mut Self {
        self.rows.push(row);
        self
    }

    /// Appends a bold header row from a sequence of strings.
    pub fn add_header_row<I, S>(&mut self, headers: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rows
            .push(headers.into_iter().map(Cell::header).collect());
        self
    }
}

/// Builds a SpreadsheetML 2003 workbook from one or more worksheets.
#[derive(Debug, Clone)]
pub struct SpreadsheetExporter {
    title: String,
    author: String,
    worksheets: Vec<Worksheet>,
}

impl SpreadsheetExporter {
    /// Creates a new, empty workbook exporter.
    pub fn new() -> Self {
        Self {
            title: "Legalis Visualization Export".to_string(),
            author: "legalis-viz".to_string(),
            worksheets: Vec::new(),
        }
    }

    /// Sets the workbook title metadata.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the workbook author metadata.
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    /// Adds a pre-built worksheet.
    pub fn add_worksheet(mut self, worksheet: Worksheet) -> Self {
        self.worksheets.push(worksheet);
        self
    }

    /// Adds a worksheet with a population chart's distribution.
    pub fn add_population_chart(mut self, chart: &PopulationChart) -> Self {
        self.worksheets.push(population_chart_worksheet(chart));
        self
    }

    /// Adds a worksheet listing statutes and their core fields.
    pub fn add_statutes(mut self, statutes: &[Statute]) -> Self {
        self.worksheets.push(statutes_worksheet(statutes));
        self
    }

    /// Adds a worksheet listing timeline events.
    pub fn add_timeline(mut self, timeline: &Timeline) -> Self {
        self.worksheets.push(timeline_worksheet(timeline));
        self
    }

    /// Adds a worksheet listing dependency-graph edges.
    pub fn add_dependency_graph(mut self, graph: &DependencyGraph) -> Self {
        self.worksheets.push(dependency_graph_worksheet(graph));
        self
    }

    /// Renders the complete SpreadsheetML 2003 workbook XML.
    pub fn to_spreadsheet_ml(&self) -> String {
        let mut out = String::new();
        out.push_str("<?xml version=\"1.0\"?>\n");
        out.push_str("<?mso-application progid=\"Excel.Sheet\"?>\n");
        out.push_str("<Workbook xmlns=\"urn:schemas-microsoft-com:office:spreadsheet\"\n");
        out.push_str(" xmlns:o=\"urn:schemas-microsoft-com:office:office\"\n");
        out.push_str(" xmlns:x=\"urn:schemas-microsoft-com:office:excel\"\n");
        out.push_str(" xmlns:ss=\"urn:schemas-microsoft-com:office:spreadsheet\"\n");
        out.push_str(" xmlns:html=\"http://www.w3.org/TR/REC-html40\">\n");
        out.push_str(" <DocumentProperties xmlns=\"urn:schemas-microsoft-com:office:office\">\n");
        out.push_str(&format!("  <Title>{}</Title>\n", escape_xml(&self.title)));
        out.push_str(&format!(
            "  <Author>{}</Author>\n",
            escape_xml(&self.author)
        ));
        out.push_str(" </DocumentProperties>\n");
        out.push_str(" <Styles>\n");
        out.push_str("  <Style ss:ID=\"Default\" ss:Name=\"Normal\"><Alignment ss:Vertical=\"Bottom\"/></Style>\n");
        out.push_str("  <Style ss:ID=\"Header\"><Font ss:Bold=\"1\"/></Style>\n");
        out.push_str(" </Styles>\n");

        let mut used_names: Vec<String> = Vec::new();
        for worksheet in &self.worksheets {
            let name = unique_sheet_name(&sanitize_sheet_name(&worksheet.name), &mut used_names);
            out.push_str(&format!(" <Worksheet ss:Name=\"{}\">\n", escape_xml(&name)));
            out.push_str("  <Table>\n");
            for row in &worksheet.rows {
                out.push_str("   <Row>\n");
                for cell in row {
                    out.push_str(&render_cell(cell));
                }
                out.push_str("   </Row>\n");
            }
            out.push_str("  </Table>\n");
            out.push_str(" </Worksheet>\n");
        }
        out.push_str("</Workbook>\n");
        out
    }
}

impl Default for SpreadsheetExporter {
    fn default() -> Self {
        Self::new()
    }
}

fn render_cell(cell: &Cell) -> String {
    let style = if cell.header {
        " ss:StyleID=\"Header\""
    } else {
        ""
    };
    let (cell_type, content) = match &cell.value {
        CellValue::Text(text) => ("String", escape_xml(text)),
        CellValue::Number(number) if number.is_finite() => ("Number", format_number(*number)),
        // Non-finite numbers cannot be represented as a SpreadsheetML Number;
        // fall back to a textual rendering so the document stays valid.
        CellValue::Number(number) => ("String", escape_xml(&format_number(*number))),
    };
    format!("    <Cell{style}><Data ss:Type=\"{cell_type}\">{content}</Data></Cell>\n")
}

fn format_number(number: f64) -> String {
    if number.is_finite() && number.fract() == 0.0 && number.abs() < 1e15 {
        format!("{}", number as i64)
    } else {
        format!("{number}")
    }
}

fn sanitize_sheet_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| match c {
            ':' | '\\' | '/' | '?' | '*' | '[' | ']' => '_',
            _ => c,
        })
        .collect();
    let trimmed = cleaned.trim();
    let limited: String = trimmed.chars().take(31).collect();
    if limited.is_empty() {
        "Sheet".to_string()
    } else {
        limited
    }
}

fn unique_sheet_name(base: &str, used: &mut Vec<String>) -> String {
    if !used.iter().any(|existing| existing == base) {
        used.push(base.to_string());
        return base.to_string();
    }
    let mut suffix = 2;
    loop {
        let tag = format!(" ({suffix})");
        let trimmed_len = 31usize.saturating_sub(tag.chars().count());
        let prefix: String = base.chars().take(trimmed_len).collect();
        let candidate = format!("{prefix}{tag}");
        if !used.iter().any(|existing| existing == &candidate) {
            used.push(candidate.clone());
            return candidate;
        }
        suffix += 1;
    }
}

fn population_chart_worksheet(chart: &PopulationChart) -> Worksheet {
    let mut worksheet = Worksheet::new("Population");
    worksheet.add_header_row(["Category", "Count", "Percentage"]);
    for point in &chart.data {
        let percentage = match point.percentage {
            Some(value) => Cell::number(value),
            None => Cell::text(""),
        };
        worksheet.add_row(vec![
            Cell::text(point.category.clone()),
            Cell::number(point.count as f64),
            percentage,
        ]);
    }
    worksheet
}

fn statutes_worksheet(statutes: &[Statute]) -> Worksheet {
    let mut worksheet = Worksheet::new("Statutes");
    worksheet.add_header_row([
        "ID",
        "Title",
        "Effect Type",
        "Description",
        "Jurisdiction",
        "Version",
    ]);
    for statute in statutes {
        worksheet.add_row(vec![
            Cell::text(statute.id.clone()),
            Cell::text(statute.title.clone()),
            Cell::text(effect_type_label(&statute.effect.effect_type)),
            Cell::text(statute.effect.description.clone()),
            Cell::text(statute.jurisdiction.clone().unwrap_or_default()),
            Cell::number(f64::from(statute.version)),
        ]);
    }
    worksheet
}

fn timeline_worksheet(timeline: &Timeline) -> Worksheet {
    let mut worksheet = Worksheet::new("Timeline");
    worksheet.add_header_row(["Date", "Event Type", "Statute ID", "Detail"]);
    for (date, event) in &timeline.events {
        let (event_type, statute_id, detail) = timeline_event_parts(event);
        worksheet.add_row(vec![
            Cell::text(date.clone()),
            Cell::text(event_type),
            Cell::text(statute_id),
            Cell::text(detail.unwrap_or("")),
        ]);
    }
    worksheet
}

fn dependency_graph_worksheet(graph: &DependencyGraph) -> Worksheet {
    let mut worksheet = Worksheet::new("Dependencies");
    worksheet.add_header_row(["From", "To", "Relation"]);
    for edge in graph.graph.edge_indices() {
        if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
            let from = graph.graph.node_weight(source).cloned().unwrap_or_default();
            let to = graph.graph.node_weight(target).cloned().unwrap_or_default();
            let relation = graph.graph.edge_weight(edge).cloned().unwrap_or_default();
            worksheet.add_row(vec![Cell::text(from), Cell::text(to), Cell::text(relation)]);
        }
    }
    worksheet
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn population_workbook_is_wellformed() {
        let mut chart = PopulationChart::new("Eligibility");
        chart.add_data("Eligible", 150);
        chart.add_data("Ineligible", 50);
        let xml = SpreadsheetExporter::new()
            .add_population_chart(&chart)
            .to_spreadsheet_ml();
        assert!(xml.starts_with("<?xml version=\"1.0\"?>"));
        assert!(xml.contains("<?mso-application progid=\"Excel.Sheet\"?>"));
        assert!(xml.contains("<Worksheet ss:Name=\"Population\">"));
        assert!(xml.contains("ss:Type=\"String\">Eligible</Data>"));
        assert!(xml.contains("ss:Type=\"Number\">150</Data>"));
        assert!(xml.trim_end().ends_with("</Workbook>"));
    }

    #[test]
    fn multiple_worksheets_and_names_are_unique() {
        let statutes = vec![Statute::new(
            "s1",
            "Law",
            Effect::new(EffectType::Grant, "Grants"),
        )];
        let exporter = SpreadsheetExporter::new()
            .add_statutes(&statutes)
            .add_worksheet(Worksheet::new("Statutes")) // duplicate name on purpose
            .add_worksheet(Worksheet::new(
                "Bad:Name/Here*Way?Too[Long]To/Fit/In/Excel/Limit",
            ));
        let xml = exporter.to_spreadsheet_ml();
        assert!(xml.contains("ss:Name=\"Statutes\""));
        assert!(xml.contains("ss:Name=\"Statutes (2)\""));
        // Invalid characters are replaced and the name is truncated to 31 chars.
        assert!(!xml.contains("Bad:Name"));
        let count = xml.matches("<Worksheet ss:Name=").count();
        assert_eq!(count, 3);
    }

    #[test]
    fn special_characters_are_escaped() {
        let statutes = vec![Statute::new(
            "s&1",
            "A <b> & \"c\"",
            Effect::new(EffectType::Grant, "x>y"),
        )];
        let xml = SpreadsheetExporter::new()
            .add_statutes(&statutes)
            .to_spreadsheet_ml();
        assert!(xml.contains("s&amp;1"));
        assert!(xml.contains("A &lt;b&gt; &amp; &quot;c&quot;"));
        assert!(xml.contains("x&gt;y"));
    }

    #[test]
    fn statutes_worksheet_has_bold_header_row() {
        let statutes = vec![Statute::new(
            "s1",
            "Law",
            Effect::new(EffectType::Grant, "Grants"),
        )];
        let xml = SpreadsheetExporter::new()
            .add_statutes(&statutes)
            .to_spreadsheet_ml();
        assert!(xml.contains("ss:StyleID=\"Header\"><Data ss:Type=\"String\">ID</Data>"));
        assert!(xml.contains("ss:Type=\"Number\">1</Data>"));
    }
}
