use super::*;

/// Report template type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateType {
    /// Summary report (high-level statistics)
    Summary,
    /// Detailed report (full statute information)
    Detailed,
    /// Compliance report (regulatory focus)
    Compliance,
    /// Audit trail report
    AuditTrail,
    /// Custom template with name
    Custom(String),
}

/// Export format for templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// HTML format
    Html,
    /// Markdown format
    Markdown,
    /// PDF format (requires additional dependencies)
    Pdf,
}

/// Report template configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    /// Template name
    pub name: String,
    /// Template type
    pub template_type: TemplateType,
    /// Export format
    pub format: ExportFormat,
    /// Fields to include
    pub fields: Vec<String>,
    /// Custom filters
    pub filters: HashMap<String, String>,
    /// Sort order
    pub sort_by: Option<String>,
}

impl ReportTemplate {
    /// Creates a new report template.
    pub fn new(name: impl Into<String>, template_type: TemplateType, format: ExportFormat) -> Self {
        Self {
            name: name.into(),
            template_type,
            format,
            fields: Vec::new(),
            filters: HashMap::new(),
            sort_by: None,
        }
    }

    /// Adds a field to include.
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.fields.push(field.into());
        self
    }

    /// Adds a filter.
    pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.insert(key.into(), value.into());
        self
    }

    /// Sets the sort order.
    pub fn with_sort_by(mut self, field: impl Into<String>) -> Self {
        self.sort_by = Some(field.into());
        self
    }

    /// Creates a summary template.
    pub fn summary(format: ExportFormat) -> Self {
        Self::new("Summary Report", TemplateType::Summary, format)
            .with_field("id")
            .with_field("title")
            .with_field("status")
            .with_field("jurisdiction")
    }

    /// Creates a detailed template.
    pub fn detailed(format: ExportFormat) -> Self {
        Self::new("Detailed Report", TemplateType::Detailed, format)
            .with_field("id")
            .with_field("title")
            .with_field("status")
            .with_field("jurisdiction")
            .with_field("tags")
            .with_field("metadata")
            .with_field("created_at")
            .with_field("modified_at")
    }

    /// Creates a compliance template.
    pub fn compliance(format: ExportFormat) -> Self {
        Self::new("Compliance Report", TemplateType::Compliance, format)
            .with_field("id")
            .with_field("title")
            .with_field("status")
            .with_field("effective_date")
            .with_field("expiry_date")
    }
}

/// Template manager.
#[derive(Debug)]
pub struct TemplateManager {
    templates: HashMap<String, ReportTemplate>,
}

impl TemplateManager {
    /// Creates a new template manager.
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// Adds a template.
    pub fn add_template(&mut self, template: ReportTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// Gets a template by name.
    pub fn get_template(&self, name: &str) -> Option<&ReportTemplate> {
        self.templates.get(name)
    }

    /// Removes a template.
    pub fn remove_template(&mut self, name: &str) -> bool {
        self.templates.remove(name).is_some()
    }

    /// Lists all template names.
    pub fn list_templates(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }

    /// Exports registry data using a template.
    pub fn export(
        &self,
        registry: &StatuteRegistry,
        template_name: &str,
    ) -> Result<String, RegistryError> {
        let template = self.get_template(template_name).ok_or_else(|| {
            RegistryError::InvalidOperation(format!("Template '{}' not found", template_name))
        })?;

        match template.format {
            ExportFormat::Json => self.export_json(registry, template),
            ExportFormat::Csv => self.export_csv(registry, template),
            ExportFormat::Html => self.export_html(registry, template),
            ExportFormat::Markdown => self.export_markdown(registry, template),
            ExportFormat::Pdf => Err(RegistryError::InvalidOperation(
                "PDF export not yet implemented".to_string(),
            )),
        }
    }

    fn export_json(
        &self,
        registry: &StatuteRegistry,
        _template: &ReportTemplate,
    ) -> Result<String, RegistryError> {
        let statutes: Vec<_> = registry.iter().collect();
        serde_json::to_string_pretty(&statutes)
            .map_err(|e| RegistryError::InvalidOperation(format!("JSON export failed: {}", e)))
    }

    fn export_csv(
        &self,
        registry: &StatuteRegistry,
        template: &ReportTemplate,
    ) -> Result<String, RegistryError> {
        let mut output = String::new();

        // Header
        if !template.fields.is_empty() {
            output.push_str(&template.fields.join(","));
        } else {
            output.push_str("id,title,status,jurisdiction");
        }
        output.push('\n');

        // Rows
        for entry in registry.iter() {
            let row = format!(
                "{},{},{:?},{}",
                entry.statute.id, entry.statute.title, entry.status, entry.jurisdiction
            );
            output.push_str(&row);
            output.push('\n');
        }

        Ok(output)
    }

    fn export_html(
        &self,
        registry: &StatuteRegistry,
        template: &ReportTemplate,
    ) -> Result<String, RegistryError> {
        let mut html = String::from("<html><head><title>");
        html.push_str(&template.name);
        html.push_str("</title></head><body><h1>");
        html.push_str(&template.name);
        html.push_str("</h1><table border='1'><tr>");

        // Header
        for field in &template.fields {
            html.push_str("<th>");
            html.push_str(field);
            html.push_str("</th>");
        }
        html.push_str("</tr>");

        // Rows
        for entry in registry.iter() {
            html.push_str("<tr>");
            for field in &template.fields {
                html.push_str("<td>");
                match field.as_str() {
                    "id" => html.push_str(&entry.statute.id),
                    "title" => html.push_str(&entry.statute.title),
                    "status" => html.push_str(&format!("{:?}", entry.status)),
                    "jurisdiction" => html.push_str(&entry.jurisdiction),
                    _ => html.push_str("N/A"),
                }
                html.push_str("</td>");
            }
            html.push_str("</tr>");
        }

        html.push_str("</table></body></html>");
        Ok(html)
    }

    fn export_markdown(
        &self,
        registry: &StatuteRegistry,
        template: &ReportTemplate,
    ) -> Result<String, RegistryError> {
        let mut md = format!("# {}\n\n", template.name);

        // Table header
        if !template.fields.is_empty() {
            md.push_str("| ");
            md.push_str(&template.fields.join(" | "));
            md.push_str(" |\n");
            md.push('|');
            for _ in &template.fields {
                md.push_str(" --- |");
            }
            md.push('\n');
        }

        // Rows
        for entry in registry.iter() {
            md.push_str("| ");
            for (i, field) in template.fields.iter().enumerate() {
                if i > 0 {
                    md.push_str(" | ");
                }
                match field.as_str() {
                    "id" => md.push_str(&entry.statute.id),
                    "title" => md.push_str(&entry.statute.title),
                    "status" => md.push_str(&format!("{:?}", entry.status)),
                    "jurisdiction" => md.push_str(&entry.jurisdiction),
                    _ => md.push_str("N/A"),
                }
            }
            md.push_str(" |\n");
        }

        Ok(md)
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new()
    }
}
