//! Interactive HTML reports with client-side filtering and sorting.
//!
//! This module provides functionality for generating interactive HTML reports
//! that allow users to filter, sort, and search audit records in the browser.

use crate::{AuditRecord, AuditResult, ComplianceReport};
use serde_json::json;

/// Configuration for interactive HTML reports.
#[derive(Debug, Clone)]
pub struct InteractiveReportConfig {
    /// Report title
    pub title: String,
    /// Include search box
    pub enable_search: bool,
    /// Enable column sorting
    pub enable_sorting: bool,
    /// Enable filtering by event type
    pub enable_event_filter: bool,
    /// Enable filtering by actor type
    pub enable_actor_filter: bool,
    /// Enable filtering by result type
    pub enable_result_filter: bool,
    /// Enable date range filtering
    pub enable_date_filter: bool,
    /// Theme (light or dark)
    pub theme: String,
    /// Records per page
    pub page_size: usize,
}

impl InteractiveReportConfig {
    /// Creates a new interactive report configuration with default settings.
    pub fn new(title: String) -> Self {
        Self {
            title,
            enable_search: true,
            enable_sorting: true,
            enable_event_filter: true,
            enable_actor_filter: true,
            enable_result_filter: true,
            enable_date_filter: true,
            theme: "light".to_string(),
            page_size: 50,
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: String) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the page size.
    pub fn with_page_size(mut self, size: usize) -> Self {
        self.page_size = size;
        self
    }

    /// Disables search functionality.
    pub fn without_search(mut self) -> Self {
        self.enable_search = false;
        self
    }

    /// Disables sorting functionality.
    pub fn without_sorting(mut self) -> Self {
        self.enable_sorting = false;
        self
    }
}

impl Default for InteractiveReportConfig {
    fn default() -> Self {
        Self::new("Audit Report".to_string())
    }
}

/// Interactive HTML report generator.
pub struct InteractiveReportGenerator;

impl InteractiveReportGenerator {
    /// Generates an interactive HTML report from audit records.
    pub fn generate(
        records: &[AuditRecord],
        compliance: &ComplianceReport,
        config: &InteractiveReportConfig,
    ) -> AuditResult<String> {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str(
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("  <title>{}</title>\n", config.title));

        // Add CSS
        html.push_str(&Self::generate_css(&config.theme));

        html.push_str("</head>\n<body>\n");

        // Header section
        html.push_str(&format!("<h1>{}</h1>\n", config.title));

        // Compliance summary
        html.push_str(&Self::generate_summary(compliance));

        // Filters section
        if config.enable_search
            || config.enable_event_filter
            || config.enable_actor_filter
            || config.enable_result_filter
            || config.enable_date_filter
        {
            html.push_str(&Self::generate_filters(config));
        }

        // Records table
        html.push_str(&Self::generate_table(records, config));

        // Add JavaScript
        html.push_str(&Self::generate_javascript(records, config));

        html.push_str("</body>\n</html>");

        Ok(html)
    }

    fn generate_css(theme: &str) -> String {
        let (bg_color, text_color, table_bg, header_bg, border_color) = if theme == "dark" {
            ("#1a1a1a", "#e0e0e0", "#2a2a2a", "#3a3a3a", "#4a4a4a")
        } else {
            ("#ffffff", "#333333", "#f9f9f9", "#4CAF50", "#ddd")
        };

        format!(
            r#"
  <style>
    body {{
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      margin: 20px;
      background-color: {bg_color};
      color: {text_color};
    }}
    h1 {{
      color: {header_bg};
      border-bottom: 3px solid {header_bg};
      padding-bottom: 10px;
    }}
    .summary {{
      background-color: {table_bg};
      padding: 20px;
      margin: 20px 0;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }}
    .summary-grid {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 15px;
    }}
    .summary-item {{
      padding: 10px;
      background-color: {bg_color};
      border-left: 4px solid {header_bg};
      border-radius: 4px;
    }}
    .summary-label {{
      font-size: 0.9em;
      opacity: 0.8;
    }}
    .summary-value {{
      font-size: 1.5em;
      font-weight: bold;
      margin-top: 5px;
    }}
    .filters {{
      background-color: {table_bg};
      padding: 20px;
      margin: 20px 0;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }}
    .filter-row {{
      display: flex;
      flex-wrap: wrap;
      gap: 15px;
      margin-bottom: 10px;
    }}
    .filter-group {{
      flex: 1;
      min-width: 200px;
    }}
    .filter-group label {{
      display: block;
      margin-bottom: 5px;
      font-weight: 500;
    }}
    .filter-group input, .filter-group select {{
      width: 100%;
      padding: 8px;
      border: 1px solid {border_color};
      border-radius: 4px;
      background-color: {bg_color};
      color: {text_color};
    }}
    table {{
      width: 100%;
      border-collapse: collapse;
      margin: 20px 0;
      background-color: {table_bg};
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }}
    th, td {{
      padding: 12px;
      text-align: left;
      border-bottom: 1px solid {border_color};
    }}
    th {{
      background-color: {header_bg};
      color: white;
      font-weight: 600;
      cursor: pointer;
      user-select: none;
    }}
    th:hover {{
      background-color: {header_bg};
      opacity: 0.9;
    }}
    th.sortable::after {{
      content: ' ⇅';
      opacity: 0.5;
    }}
    th.sorted-asc::after {{
      content: ' ↑';
      opacity: 1;
    }}
    th.sorted-desc::after {{
      content: ' ↓';
      opacity: 1;
    }}
    tr:hover {{
      background-color: {border_color};
    }}
    .pagination {{
      display: flex;
      justify-content: center;
      gap: 10px;
      margin: 20px 0;
    }}
    .pagination button {{
      padding: 8px 16px;
      background-color: {header_bg};
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }}
    .pagination button:disabled {{
      opacity: 0.5;
      cursor: not-allowed;
    }}
    .pagination button:hover:not(:disabled) {{
      opacity: 0.9;
    }}
    .page-info {{
      padding: 8px 16px;
      background-color: {table_bg};
      border-radius: 4px;
    }}
    .hidden {{
      display: none;
    }}
  </style>
"#,
            bg_color = bg_color,
            text_color = text_color,
            table_bg = table_bg,
            header_bg = header_bg,
            border_color = border_color
        )
    }

    fn generate_summary(compliance: &ComplianceReport) -> String {
        format!(
            r#"
  <div class="summary">
    <h2>Compliance Summary</h2>
    <div class="summary-grid">
      <div class="summary-item">
        <div class="summary-label">Total Decisions</div>
        <div class="summary-value">{}</div>
      </div>
      <div class="summary-item">
        <div class="summary-label">Automatic Decisions</div>
        <div class="summary-value">{}</div>
      </div>
      <div class="summary-item">
        <div class="summary-label">Discretionary Decisions</div>
        <div class="summary-value">{}</div>
      </div>
      <div class="summary-item">
        <div class="summary-label">Human Overrides</div>
        <div class="summary-value">{}</div>
      </div>
      <div class="summary-item">
        <div class="summary-label">Integrity Status</div>
        <div class="summary-value">{}</div>
      </div>
      <div class="summary-item">
        <div class="summary-label">Generated At</div>
        <div class="summary-value">{}</div>
      </div>
    </div>
  </div>
"#,
            compliance.total_decisions,
            compliance.automatic_decisions,
            compliance.discretionary_decisions,
            compliance.human_overrides,
            if compliance.integrity_verified {
                "✓ Verified"
            } else {
                "✗ Failed"
            },
            compliance.generated_at.format("%Y-%m-%d %H:%M:%S")
        )
    }

    fn generate_filters(config: &InteractiveReportConfig) -> String {
        let mut filters = String::from("  <div class=\"filters\">\n    <h2>Filters</h2>\n");

        filters.push_str("    <div class=\"filter-row\">\n");

        if config.enable_search {
            filters.push_str(
                r#"      <div class="filter-group">
        <label for="search">Search</label>
        <input type="text" id="search" placeholder="Search records...">
      </div>
"#,
            );
        }

        if config.enable_event_filter {
            filters.push_str(
                r#"      <div class="filter-group">
        <label for="eventFilter">Event Type</label>
        <select id="eventFilter">
          <option value="">All</option>
          <option value="AutomaticDecision">Automatic Decision</option>
          <option value="DiscretionaryReview">Discretionary Review</option>
          <option value="HumanOverride">Human Override</option>
          <option value="Appeal">Appeal</option>
          <option value="StatuteModified">Statute Modified</option>
          <option value="SimulationRun">Simulation Run</option>
        </select>
      </div>
"#,
            );
        }

        if config.enable_result_filter {
            filters.push_str(
                r#"      <div class="filter-group">
        <label for="resultFilter">Result Type</label>
        <select id="resultFilter">
          <option value="">All</option>
          <option value="Deterministic">Deterministic</option>
          <option value="RequiresDiscretion">Requires Discretion</option>
          <option value="Void">Void</option>
          <option value="Overridden">Overridden</option>
        </select>
      </div>
"#,
            );
        }

        filters.push_str("    </div>\n  </div>\n");
        filters
    }

    fn generate_table(records: &[AuditRecord], config: &InteractiveReportConfig) -> String {
        let sortable_class = if config.enable_sorting {
            " sortable"
        } else {
            ""
        };

        let mut table = format!(
            r#"
  <table id="auditTable">
    <thead>
      <tr>
        <th class="{0}" data-column="timestamp">Timestamp</th>
        <th class="{0}" data-column="event">Event Type</th>
        <th class="{0}" data-column="actor">Actor</th>
        <th class="{0}" data-column="statute">Statute</th>
        <th class="{0}" data-column="subject">Subject</th>
        <th class="{0}" data-column="result">Result</th>
      </tr>
    </thead>
    <tbody id="tableBody">
"#,
            sortable_class
        );

        // Add data rows (these will be filtered/sorted by JavaScript)
        for (index, record) in records.iter().enumerate() {
            let event_type = format!("{:?}", record.event_type);
            let actor = match &record.actor {
                crate::Actor::System { component } => format!("System: {}", component),
                crate::Actor::User { user_id, role } => format!("User: {} ({})", user_id, role),
                crate::Actor::External { system_id } => format!("External: {}", system_id),
            };
            let result_type = match &record.result {
                crate::DecisionResult::Deterministic { .. } => "Deterministic",
                crate::DecisionResult::RequiresDiscretion { .. } => "RequiresDiscretion",
                crate::DecisionResult::Void { .. } => "Void",
                crate::DecisionResult::Overridden { .. } => "Overridden",
            };

            table.push_str(&format!(
                r#"      <tr data-index="{}" data-event="{}" data-result="{}">
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
      </tr>
"#,
                index,
                event_type,
                result_type,
                record.timestamp.format("%Y-%m-%d %H:%M:%S"),
                event_type,
                actor,
                record.statute_id,
                record.subject_id,
                result_type
            ));
        }

        table.push_str("    </tbody>\n  </table>\n");

        // Add pagination
        table.push_str(
            r#"
  <div class="pagination">
    <button id="prevPage">Previous</button>
    <span class="page-info" id="pageInfo">Page 1</span>
    <button id="nextPage">Next</button>
  </div>
"#,
        );

        table
    }

    fn generate_javascript(records: &[AuditRecord], config: &InteractiveReportConfig) -> String {
        // Convert records to JSON for JavaScript
        let records_json = json!(records
            .iter()
            .map(|r| {
                json!({
                    "id": r.id,
                    "timestamp": r.timestamp.to_rfc3339(),
                    "event_type": format!("{:?}", r.event_type),
                    "actor": match &r.actor {
                        crate::Actor::System { component } => format!("System: {}", component),
                        crate::Actor::User { user_id, role } => format!("User: {} ({})", user_id, role),
                        crate::Actor::External { system_id } => format!("External: {}", system_id),
                    },
                    "statute_id": r.statute_id,
                    "subject_id": r.subject_id,
                    "result_type": match &r.result {
                        crate::DecisionResult::Deterministic { .. } => "Deterministic",
                        crate::DecisionResult::RequiresDiscretion { .. } => "RequiresDiscretion",
                        crate::DecisionResult::Void { .. } => "Void",
                        crate::DecisionResult::Overridden { .. } => "Overridden",
                    },
                })
            })
            .collect::<Vec<_>>());

        format!(
            r#"
  <script>
    const allRecords = {};
    const pageSize = {};
    let currentPage = 1;
    let filteredRecords = [...allRecords];
    let sortColumn = null;
    let sortAscending = true;

    function updateTable() {{
      const tbody = document.getElementById('tableBody');
      const startIndex = (currentPage - 1) * pageSize;
      const endIndex = startIndex + pageSize;
      const pageRecords = filteredRecords.slice(startIndex, endIndex);

      tbody.innerHTML = '';
      pageRecords.forEach(record => {{
        const row = tbody.insertRow();
        row.innerHTML = `
          <td>${{record.timestamp}}</td>
          <td>${{record.event_type}}</td>
          <td>${{record.actor}}</td>
          <td>${{record.statute_id}}</td>
          <td>${{record.subject_id}}</td>
          <td>${{record.result_type}}</td>
        `;
      }});

      updatePagination();
    }}

    function updatePagination() {{
      const totalPages = Math.ceil(filteredRecords.length / pageSize);
      document.getElementById('pageInfo').textContent = `Page ${{currentPage}} of ${{totalPages}}`;
      document.getElementById('prevPage').disabled = currentPage === 1;
      document.getElementById('nextPage').disabled = currentPage >= totalPages;
    }}

    function applyFilters() {{
      filteredRecords = allRecords.filter(record => {{
        const searchTerm = document.getElementById('search')?.value.toLowerCase() || '';
        const eventFilter = document.getElementById('eventFilter')?.value || '';
        const resultFilter = document.getElementById('resultFilter')?.value || '';

        if (searchTerm && !JSON.stringify(record).toLowerCase().includes(searchTerm)) {{
          return false;
        }}
        if (eventFilter && record.event_type !== eventFilter) {{
          return false;
        }}
        if (resultFilter && record.result_type !== resultFilter) {{
          return false;
        }}

        return true;
      }});

      currentPage = 1;
      updateTable();
    }}

    function sortTable(column) {{
      if (sortColumn === column) {{
        sortAscending = !sortAscending;
      }} else {{
        sortColumn = column;
        sortAscending = true;
      }}

      filteredRecords.sort((a, b) => {{
        let aVal = a[column];
        let bVal = b[column];

        if (aVal < bVal) return sortAscending ? -1 : 1;
        if (aVal > bVal) return sortAscending ? 1 : -1;
        return 0;
      }});

      // Update sort indicators
      document.querySelectorAll('th').forEach(th => {{
        th.classList.remove('sorted-asc', 'sorted-desc');
      }});
      const th = document.querySelector(`th[data-column="${{column}}"]`);
      if (th) {{
        th.classList.add(sortAscending ? 'sorted-asc' : 'sorted-desc');
      }}

      updateTable();
    }}

    // Event listeners
    document.getElementById('search')?.addEventListener('input', applyFilters);
    document.getElementById('eventFilter')?.addEventListener('change', applyFilters);
    document.getElementById('resultFilter')?.addEventListener('change', applyFilters);

    document.querySelectorAll('th.sortable').forEach(th => {{
      th.addEventListener('click', () => {{
        const column = th.dataset.column;
        if (column === 'timestamp') sortTable('timestamp');
        else if (column === 'event') sortTable('event_type');
        else if (column === 'result') sortTable('result_type');
        else if (column === 'statute') sortTable('statute_id');
      }});
    }});

    document.getElementById('prevPage').addEventListener('click', () => {{
      if (currentPage > 1) {{
        currentPage--;
        updateTable();
      }}
    }});

    document.getElementById('nextPage').addEventListener('click', () => {{
      const totalPages = Math.ceil(filteredRecords.length / pageSize);
      if (currentPage < totalPages) {{
        currentPage++;
        updateTable();
      }}
    }});

    // Initial render
    updateTable();
  </script>
"#,
            records_json, config.page_size
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, AuditRecord, DecisionContext, DecisionResult, EventType};
    use chrono::Utc;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_interactive_report_config() {
        let config = InteractiveReportConfig::new("Test Report".to_string())
            .with_theme("dark".to_string())
            .with_page_size(100)
            .without_search();

        assert_eq!(config.title, "Test Report");
        assert_eq!(config.theme, "dark");
        assert_eq!(config.page_size, 100);
        assert!(!config.enable_search);
    }

    #[test]
    fn test_generate_interactive_report() {
        let records = vec![AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-123".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )];

        let compliance = ComplianceReport {
            total_decisions: 1,
            automatic_decisions: 1,
            discretionary_decisions: 0,
            human_overrides: 0,
            integrity_verified: true,
            generated_at: Utc::now(),
        };

        let config = InteractiveReportConfig::default();
        let html = InteractiveReportGenerator::generate(&records, &compliance, &config).unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Audit Report"));
        assert!(html.contains("statute-123"));
        assert!(html.contains("<script>"));
        assert!(html.contains("function updateTable()"));
    }

    #[test]
    fn test_css_generation() {
        let css_light = InteractiveReportGenerator::generate_css("light");
        assert!(css_light.contains("background-color: #ffffff"));

        let css_dark = InteractiveReportGenerator::generate_css("dark");
        assert!(css_dark.contains("background-color: #1a1a1a"));
    }
}
