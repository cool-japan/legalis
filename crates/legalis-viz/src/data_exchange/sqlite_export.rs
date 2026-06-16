//! SQLite export as portable SQL text for offline querying.
//!
//! Rather than depending on a native SQLite driver, this exporter emits a plain
//! `.sql` script of `CREATE TABLE` and `INSERT` statements that loads into any
//! SQLite database with `sqlite3 legal.db < export.sql`. The generated schema is
//! deliberately simple and stable so the resulting database can be queried with
//! ordinary SQL. String literals are escaped per the SQL standard (single
//! quotes doubled) and optional values become `NULL`.

use super::{effect_type_label, escape_sql_string, timeline_event_parts};
use crate::types_3::Timeline;
use crate::types_4::DependencyGraph;
use crate::types_5::PopulationChart;
use legalis_core::Statute;

/// Emits portable SQLite-compatible SQL scripts from crate model types.
#[derive(Debug, Clone)]
pub struct SqliteExporter {
    use_transaction: bool,
    if_not_exists: bool,
}

impl SqliteExporter {
    /// Creates a new exporter that wraps output in a transaction and uses
    /// `CREATE TABLE IF NOT EXISTS`.
    pub fn new() -> Self {
        Self {
            use_transaction: true,
            if_not_exists: true,
        }
    }

    /// Disables the surrounding `BEGIN TRANSACTION; ... COMMIT;` wrapper.
    pub fn without_transaction(mut self) -> Self {
        self.use_transaction = false;
        self
    }

    /// Uses bare `CREATE TABLE` instead of `CREATE TABLE IF NOT EXISTS`.
    pub fn without_if_not_exists(mut self) -> Self {
        self.if_not_exists = false;
        self
    }

    /// Emits a script defining and populating a `statutes` table.
    pub fn statutes_to_sql(&self, statutes: &[Statute]) -> String {
        let mut out = String::new();
        self.begin(&mut out);
        out.push_str(&format!(
            "{} statutes (\n  \
             id TEXT PRIMARY KEY,\n  \
             title TEXT NOT NULL,\n  \
             effect_type TEXT NOT NULL,\n  \
             effect_description TEXT,\n  \
             jurisdiction TEXT,\n  \
             version INTEGER NOT NULL,\n  \
             discretion_logic TEXT\n);\n",
            self.create_prefix()
        ));
        for statute in statutes {
            out.push_str(&format!(
                "INSERT INTO statutes (id, title, effect_type, effect_description, jurisdiction, version, discretion_logic) VALUES ({}, {}, {}, {}, {}, {}, {});\n",
                sql_text(&statute.id),
                sql_text(&statute.title),
                sql_text(effect_type_label(&statute.effect.effect_type)),
                sql_text(&statute.effect.description),
                sql_opt_text(statute.jurisdiction.as_deref()),
                statute.version,
                sql_opt_text(statute.discretion_logic.as_deref()),
            ));
        }
        self.commit(&mut out);
        out
    }

    /// Emits a script defining `statute_nodes` and `dependencies` tables.
    pub fn dependency_graph_to_sql(&self, graph: &DependencyGraph) -> String {
        let mut out = String::new();
        self.begin(&mut out);
        out.push_str(&format!(
            "{} statute_nodes (\n  id TEXT PRIMARY KEY\n);\n",
            self.create_prefix()
        ));
        out.push_str(&format!(
            "{} dependencies (\n  \
             from_id TEXT NOT NULL,\n  \
             to_id TEXT NOT NULL,\n  \
             relation TEXT NOT NULL\n);\n",
            self.create_prefix()
        ));
        for index in graph.graph.node_indices() {
            if let Some(id) = graph.graph.node_weight(index) {
                out.push_str(&format!(
                    "INSERT INTO statute_nodes (id) VALUES ({});\n",
                    sql_text(id)
                ));
            }
        }
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                let from = graph.graph.node_weight(source).cloned().unwrap_or_default();
                let to = graph.graph.node_weight(target).cloned().unwrap_or_default();
                let relation = graph.graph.edge_weight(edge).cloned().unwrap_or_default();
                out.push_str(&format!(
                    "INSERT INTO dependencies (from_id, to_id, relation) VALUES ({}, {}, {});\n",
                    sql_text(&from),
                    sql_text(&to),
                    sql_text(&relation),
                ));
            }
        }
        self.commit(&mut out);
        out
    }

    /// Emits a script defining and populating a `timeline_events` table.
    pub fn timeline_to_sql(&self, timeline: &Timeline) -> String {
        let mut out = String::new();
        self.begin(&mut out);
        out.push_str(&format!(
            "{} timeline_events (\n  \
             event_date TEXT NOT NULL,\n  \
             event_type TEXT NOT NULL,\n  \
             statute_id TEXT NOT NULL,\n  \
             detail TEXT\n);\n",
            self.create_prefix()
        ));
        for (date, event) in &timeline.events {
            let (event_type, statute_id, detail) = timeline_event_parts(event);
            out.push_str(&format!(
                "INSERT INTO timeline_events (event_date, event_type, statute_id, detail) VALUES ({}, {}, {}, {});\n",
                sql_text(date),
                sql_text(event_type),
                sql_text(statute_id),
                sql_opt_text(detail),
            ));
        }
        self.commit(&mut out);
        out
    }

    /// Emits a script defining and populating a `population` table.
    pub fn population_chart_to_sql(&self, chart: &PopulationChart) -> String {
        let mut out = String::new();
        self.begin(&mut out);
        out.push_str(&format!(
            "{} population (\n  \
             category TEXT NOT NULL,\n  \
             count INTEGER NOT NULL,\n  \
             percentage REAL\n);\n",
            self.create_prefix()
        ));
        for point in &chart.data {
            out.push_str(&format!(
                "INSERT INTO population (category, count, percentage) VALUES ({}, {}, {});\n",
                sql_text(&point.category),
                point.count,
                sql_real(point.percentage),
            ));
        }
        self.commit(&mut out);
        out
    }

    fn create_prefix(&self) -> &'static str {
        if self.if_not_exists {
            "CREATE TABLE IF NOT EXISTS"
        } else {
            "CREATE TABLE"
        }
    }

    fn begin(&self, out: &mut String) {
        if self.use_transaction {
            out.push_str("BEGIN TRANSACTION;\n");
        }
    }

    fn commit(&self, out: &mut String) {
        if self.use_transaction {
            out.push_str("COMMIT;\n");
        }
    }
}

impl Default for SqliteExporter {
    fn default() -> Self {
        Self::new()
    }
}

fn sql_text(value: &str) -> String {
    format!("'{}'", escape_sql_string(value))
}

fn sql_opt_text(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("'{}'", escape_sql_string(value)),
        None => "NULL".to_string(),
    }
}

fn sql_real(value: Option<f64>) -> String {
    match value {
        Some(value) if value.is_finite() => format!("{value}"),
        _ => "NULL".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types_5::TimelineEvent;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn statutes_sql_creates_table_and_inserts_within_transaction() {
        let statutes = vec![
            Statute::new("s1", "First Law", Effect::new(EffectType::Grant, "Grants"))
                .with_jurisdiction("US")
                .with_version(3),
        ];
        let sql = SqliteExporter::new().statutes_to_sql(&statutes);
        assert!(sql.starts_with("BEGIN TRANSACTION;\n"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS statutes ("));
        assert!(sql.contains(
            "INSERT INTO statutes (id, title, effect_type, effect_description, jurisdiction, version, discretion_logic) VALUES ('s1', 'First Law', 'Grant', 'Grants', 'US', 3, NULL);"
        ));
        assert!(sql.trim_end().ends_with("COMMIT;"));
    }

    #[test]
    fn sql_text_escapes_single_quotes() {
        let statutes = vec![Statute::new(
            "s1",
            "O'Brien's Act",
            Effect::new(EffectType::Grant, "It's fine"),
        )];
        let sql = SqliteExporter::new().statutes_to_sql(&statutes);
        assert!(sql.contains("'O''Brien''s Act'"));
        assert!(sql.contains("'It''s fine'"));
    }

    #[test]
    fn dependency_graph_sql_has_nodes_and_edges() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "requires");
        let sql = SqliteExporter::new().dependency_graph_to_sql(&graph);
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS statute_nodes ("));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS dependencies ("));
        assert!(sql.contains("INSERT INTO statute_nodes (id) VALUES ('a');"));
        assert!(sql.contains(
            "INSERT INTO dependencies (from_id, to_id, relation) VALUES ('a', 'b', 'requires');"
        ));
    }

    #[test]
    fn timeline_sql_inserts_events() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2020-01-01",
            TimelineEvent::Enacted {
                statute_id: "s1".to_string(),
                title: "Enacted".to_string(),
            },
        );
        timeline.add_event(
            "2021-06-01",
            TimelineEvent::Repealed {
                statute_id: "s1".to_string(),
            },
        );
        let sql = SqliteExporter::new().timeline_to_sql(&timeline);
        assert!(sql.contains(
            "INSERT INTO timeline_events (event_date, event_type, statute_id, detail) VALUES ('2020-01-01', 'Enacted', 's1', 'Enacted');"
        ));
        // Repealed has no detail -> NULL.
        assert!(sql.contains("VALUES ('2021-06-01', 'Repealed', 's1', NULL);"));
    }

    #[test]
    fn population_sql_handles_missing_percentage_and_no_transaction() {
        let mut chart = PopulationChart::new("Eligibility");
        chart.add_data("Eligible", 150);
        chart.calculate_percentages();
        chart.add_data("Pending", 25); // no percentage computed for this one
        let sql = SqliteExporter::new()
            .without_transaction()
            .without_if_not_exists()
            .population_chart_to_sql(&chart);
        assert!(!sql.contains("BEGIN TRANSACTION;"));
        assert!(!sql.contains("COMMIT;"));
        assert!(sql.contains("CREATE TABLE population ("));
        assert!(sql.contains(
            "INSERT INTO population (category, count, percentage) VALUES ('Eligible', 150, 100"
        ));
        assert!(sql.contains("VALUES ('Pending', 25, NULL);"));
    }
}
