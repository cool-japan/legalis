//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::AnnotationType;
use super::types_3::CourtNode;
use super::types_4::DependencyGraph;
use super::types_6::GeoCoordinate;
use super::types_7::{HighlightRule, StatuteVersion};
use super::types_10::Theme;
use super::types_11::{CaseCitation, ConceptRelationType, DecisionNode};
use super::types_12::DecisionTree;

/// Court hierarchy visualizer
#[derive(Debug, Clone)]
pub struct CourtHierarchyVisualizer {
    pub(crate) theme: Theme,
}
impl CourtHierarchyVisualizer {
    /// Creates a new court hierarchy visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Renders court hierarchy to HTML.
    #[allow(clippy::too_many_arguments)]
    pub fn to_html(&self, courts: &[CourtNode]) -> String {
        let mut levels: HashMap<String, Vec<&CourtNode>> = HashMap::new();
        for court in courts {
            levels.entry(court.level.clone()).or_default().push(court);
        }
        let mut html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            font-family: Arial, sans-serif;
            background-color: {};
            color: {};
            padding: 20px;
        }}
        .court-hierarchy {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .court-level {{
            margin-bottom: 30px;
            padding: 20px;
            background-color: {};
            border-radius: 8px;
        }}
        .level-title {{
            font-size: 24px;
            font-weight: bold;
            margin-bottom: 15px;
            color: {};
        }}
        .court-container {{
            display: flex;
            flex-wrap: wrap;
            gap: 15px;
        }}
        .court-box {{
            flex: 1 1 300px;
            padding: 15px;
            background-color: {};
            border: 2px solid {};
            border-radius: 6px;
        }}
        .court-name {{
            font-weight: bold;
            font-size: 16px;
            margin-bottom: 8px;
        }}
        .court-info {{
            font-size: 14px;
            color: {};
            margin: 4px 0;
        }}
    </style>
</head>
<body>
    <div class="court-hierarchy">
        <h1>Court Hierarchy</h1>
"#,
            self.theme.background_color,
            self.theme.text_color,
            self.theme.root_color,
            self.theme.condition_color,
            self.theme.outcome_color,
            self.theme.link_color,
            self.theme.text_color,
        );
        let level_order = ["Supreme", "Appellate", "Trial", "District", "Municipal"];
        for level in &level_order {
            if let Some(court_list) = levels.get(*level) {
                html.push_str(&format!(
                    r#"        <div class="court-level">
            <div class="level-title">{} Courts</div>
            <div class="court-container">
"#,
                    level
                ));
                for court in court_list {
                    html.push_str(&format!(
                        r#"                <div class="court-box">
                    <div class="court-name">{}</div>
                    <div class="court-info">Jurisdiction: {}</div>
                    <div class="court-info">Judges: {}</div>
                </div>
"#,
                        court.name, court.jurisdiction, court.judge_count
                    ));
                }
                html.push_str("            </div>\n        </div>\n");
            }
        }
        html.push_str(
            r#"    </div>
</body>
</html>"#,
        );
        html
    }
    /// Renders court hierarchy to Mermaid diagram.
    pub fn to_mermaid(&self, courts: &[CourtNode]) -> String {
        let mut diagram = String::from("graph TD\n");
        let mut levels: HashMap<String, Vec<&CourtNode>> = HashMap::new();
        for court in courts {
            levels.entry(court.level.clone()).or_default().push(court);
        }
        let level_order = ["Supreme", "Appellate", "Trial", "District", "Municipal"];
        for (i, level) in level_order.iter().enumerate() {
            if let Some(court_list) = levels.get(*level) {
                for court in court_list {
                    let node_id = court.id.replace('-', "_");
                    diagram.push_str(&format!(
                        "    {}[\"{}<br/>{}\"]",
                        node_id, court.name, court.jurisdiction
                    ));
                    if i > 0
                        && let Some(prev_level) = level_order.get(i - 1)
                        && let Some(prev_courts) = levels.get(*prev_level)
                    {
                        for prev_court in prev_courts {
                            let prev_id = prev_court.id.replace('-', "_");
                            diagram.push_str(&format!("\n    {} --> {}", prev_id, node_id));
                        }
                    }
                    diagram.push('\n');
                }
            }
        }
        diagram
    }
}
/// Relationship between two legal concepts.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConceptRelationship {
    /// Source concept ID
    pub from_id: String,
    /// Target concept ID
    pub to_id: String,
    /// Type of relationship
    pub relation_type: ConceptRelationType,
    /// Optional description
    pub description: String,
    /// Strength/confidence (0.0 to 1.0)
    pub strength: f64,
}
impl ConceptRelationship {
    /// Creates a new concept relationship.
    pub fn new(from_id: &str, to_id: &str, relation_type: ConceptRelationType) -> Self {
        Self {
            from_id: from_id.to_string(),
            to_id: to_id.to_string(),
            relation_type,
            description: String::new(),
            strength: 1.0,
        }
    }
    /// Sets the description.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }
    /// Sets the strength (clamped to 0.0-1.0).
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }
}
/// Historical comparison view for comparing statute versions.
#[derive(Debug, Clone)]
pub struct HistoricalComparisonView {
    /// Comparison title
    pub title: String,
    /// Statute versions to compare
    pub versions: Vec<StatuteVersion>,
    /// Theme
    pub theme: Theme,
}
impl HistoricalComparisonView {
    /// Creates a new historical comparison view.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            versions: Vec::new(),
            theme: Theme::light(),
        }
    }
    /// Adds a statute version.
    pub fn add_version(&mut self, version: StatuteVersion) {
        self.versions.push(version);
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Generates side-by-side comparison HTML.
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str(
            &format!(
                "        body {{ margin: 20px; background-color: {}; color: {}; font-family: 'Segoe UI', Arial, sans-serif; }}\n",
                self.theme.background_color, self.theme.text_color
            ),
        );
        html.push_str("        .comparison-container { max-width: 1400px; margin: 0 auto; }\n");
        html.push_str("        .versions { display: flex; gap: 20px; overflow-x: auto; }\n");
        html.push_str(
            "        .version-panel { flex: 1; min-width: 350px; background: white; border-radius: 8px; padding: 20px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n",
        );
        html.push_str(
            "        .version-header { background: #3498db; color: white; padding: 15px; margin: -20px -20px 20px -20px; border-radius: 8px 8px 0 0; }\n",
        );
        html.push_str(
            "        .version-title { font-size: 1.3em; font-weight: bold; margin: 0; }\n",
        );
        html.push_str(
            "        .version-date { font-size: 0.9em; opacity: 0.9; margin-top: 5px; }\n",
        );
        html.push_str("        .section-list { list-style: none; padding: 0; }\n");
        html.push_str(
            "        .section-item { padding: 10px; margin: 5px 0; background: #ecf0f1; border-radius: 4px; border-left: 4px solid #3498db; }\n",
        );
        html.push_str(
            "        .content { margin: 20px 0; padding: 15px; background: #f8f9fa; border-radius: 4px; line-height: 1.6; }\n",
        );
        html.push_str(
            "        .metadata { margin-top: 20px; font-size: 0.9em; color: #7f8c8d; }\n",
        );
        html.push_str("        .metadata-item { margin: 5px 0; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("    <div class=\"comparison-container\">\n");
        html.push_str(&format!("        <h1>{}</h1>\n", self.title));
        html.push_str("        <div class=\"versions\">\n");
        for version in &self.versions {
            html.push_str("            <div class=\"version-panel\">\n");
            html.push_str("                <div class=\"version-header\">\n");
            html.push_str(&format!(
                "                    <h2 class=\"version-title\">Version {}</h2>\n",
                version.version
            ));
            html.push_str(&format!(
                "                    <div class=\"version-date\">Effective: {}</div>\n",
                version.effective_date
            ));
            html.push_str("                </div>\n");
            html.push_str("                <h3>Content</h3>\n");
            html.push_str(&format!(
                "                <div class=\"content\">{}</div>\n",
                version.content
            ));
            if !version.sections.is_empty() {
                html.push_str("                <h3>Sections</h3>\n");
                html.push_str("                <ul class=\"section-list\">\n");
                for section in &version.sections {
                    html.push_str(&format!(
                        "                    <li class=\"section-item\">{}</li>\n",
                        section
                    ));
                }
                html.push_str("                </ul>\n");
            }
            if !version.metadata.is_empty() {
                html.push_str("                <div class=\"metadata\">\n");
                html.push_str("                    <h4>Metadata</h4>\n");
                for (key, value) in &version.metadata {
                    html.push_str(
                        &format!(
                            "                    <div class=\"metadata-item\"><strong>{}:</strong> {}</div>\n",
                            key, value
                        ),
                    );
                }
                html.push_str("                </div>\n");
            }
            html.push_str("            </div>\n");
        }
        html.push_str("        </div>\n");
        html.push_str("    </div>\n");
        html.push_str("</body>\n</html>");
        html
    }
    /// Generates a Mermaid comparison diagram.
    pub fn to_mermaid(&self) -> String {
        let mut diagram = String::new();
        diagram.push_str("graph LR\n");
        for (i, version) in self.versions.iter().enumerate() {
            let node_id = format!("V{}", i);
            let next_id = format!("V{}", i + 1);
            diagram.push_str(&format!(
                "    {}[\"Version {}\\n{}\"]\n",
                node_id, version.version, version.effective_date
            ));
            if i < self.versions.len() - 1 {
                diagram.push_str(&format!("    {} -->|Amended| {}\n", node_id, next_id));
            }
        }
        diagram
    }
}
/// CSS variable customization for dynamic theming.
#[derive(Debug, Clone)]
pub struct CssVariableTheme {
    /// CSS variable definitions
    variables: Vec<(String, String)>,
}
impl CssVariableTheme {
    /// Creates a new CSS variable theme.
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }
    /// Adds a CSS variable.
    pub fn add_variable(mut self, name: &str, value: &str) -> Self {
        self.variables.push((name.to_string(), value.to_string()));
        self
    }
    /// Creates CSS variable theme from a Theme.
    pub fn from_theme(theme: &Theme) -> Self {
        Self::new()
            .add_variable("--viz-root-color", &theme.root_color)
            .add_variable("--viz-condition-color", &theme.condition_color)
            .add_variable("--viz-discretion-color", &theme.discretion_color)
            .add_variable("--viz-outcome-color", &theme.outcome_color)
            .add_variable("--viz-link-color", &theme.link_color)
            .add_variable("--viz-background-color", &theme.background_color)
            .add_variable("--viz-text-color", &theme.text_color)
    }
    /// Generates CSS :root block with variables.
    pub fn to_css(&self) -> String {
        let mut css = String::from(":root {\n");
        for (name, value) in &self.variables {
            css.push_str(&format!("  {}: {};\n", name, value));
        }
        css.push_str("}\n");
        css
    }
    /// Generates CSS with custom selector.
    pub fn to_css_with_selector(&self, selector: &str) -> String {
        let mut css = String::from(selector);
        css.push_str(" {\n");
        for (name, value) in &self.variables {
            css.push_str(&format!("  {}: {};\n", name, value));
        }
        css.push_str("}\n");
        css
    }
    /// Gets all variables.
    pub fn variables(&self) -> &[(String, String)] {
        &self.variables
    }
}
/// Map tile provider configuration.
#[derive(Debug, Clone)]
pub enum TileProvider {
    /// OpenStreetMap tiles
    OpenStreetMap,
    /// Mapbox tiles (requires API key)
    Mapbox(String),
    /// Google Maps tiles (requires API key)
    GoogleMaps(String),
    /// Custom tile provider with URL template
    Custom(String),
}
impl TileProvider {
    /// Gets the tile URL template.
    pub fn url_template(&self) -> String {
        match self {
            TileProvider::OpenStreetMap => {
                "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png".to_string()
            }
            TileProvider::Mapbox(api_key) => {
                format!(
                    "https://api.mapbox.com/styles/v1/mapbox/streets-v11/tiles/{{z}}/{{x}}/{{y}}?access_token={}",
                    api_key
                )
            }
            TileProvider::GoogleMaps(api_key) => {
                format!(
                    "https://maps.googleapis.com/maps/vt?pb=!1m5!1m4!1i{{z}}!2i{{x}}!3i{{y}}!4i256!2m3!1e0!2sm!3i{{s}}!3m9!2sen!3sUS!5e18!12m1!1e47!12m3!1e37!2m1!1ssmartmaps!4e0&key={}",
                    api_key
                )
            }
            TileProvider::Custom(template) => template.clone(),
        }
    }
    /// Gets attribution text for the tile provider.
    pub fn attribution(&self) -> &str {
        match self {
            TileProvider::OpenStreetMap => {
                "&copy; <a href='https://www.openstreetmap.org/copyright'>OpenStreetMap</a> contributors"
            }
            TileProvider::Mapbox(_) => {
                "&copy; <a href='https://www.mapbox.com/about/maps/'>Mapbox</a>"
            }
            TileProvider::GoogleMaps(_) => "&copy; Google Maps",
            TileProvider::Custom(_) => "",
        }
    }
}
/// LaTeX/TikZ exporter for academic papers.
#[derive(Debug, Clone)]
pub struct LatexTikzExporter {
    /// Document class (article, paper, beamer)
    pub document_class: String,
    /// Include standalone wrapper
    pub standalone: bool,
    /// Theme
    pub theme: Theme,
}
impl LatexTikzExporter {
    /// Creates a new LaTeX/TikZ exporter.
    pub fn new() -> Self {
        Self {
            document_class: "article".to_string(),
            standalone: false,
            theme: Theme::light(),
        }
    }
    /// Sets the document class.
    pub fn with_document_class(mut self, class: &str) -> Self {
        self.document_class = class.to_string();
        self
    }
    /// Sets standalone mode.
    pub fn with_standalone(mut self, standalone: bool) -> Self {
        self.standalone = standalone;
        self
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Exports a decision tree to LaTeX/TikZ format.
    pub fn export_decision_tree(&self, _tree: &DecisionTree) -> String {
        let mut latex = String::new();
        if self.standalone {
            latex.push_str("\\documentclass[tikz,border=10pt]{standalone}\n");
        } else {
            latex.push_str(&format!("\\documentclass{{{}}}\n", self.document_class));
        }
        latex.push_str("\\usepackage{tikz}\n");
        latex.push_str("\\usetikzlibrary{shapes,arrows,positioning,trees}\n");
        latex.push('\n');
        latex.push_str("\\begin{document}\n");
        latex.push('\n');
        latex.push_str("\\begin{tikzpicture}[\n");
        latex.push_str("    node distance=1.5cm and 2cm,\n");
        latex
            .push_str(
                "    every node/.style={rectangle, draw, rounded corners, align=center, minimum width=3cm, minimum height=1cm},\n",
            );
        latex.push_str("    condition/.style={fill=blue!20},\n");
        latex.push_str("    outcome/.style={fill=green!20},\n");
        latex.push_str("    discretion/.style={fill=red!20},\n");
        latex.push_str("    arrow/.style={->, >=stealth, thick}\n");
        latex.push_str("]\n\n");
        latex.push_str("% Nodes\n");
        latex.push_str("\\node[condition] (root) {Decision Tree};\n");
        latex.push_str("\\node[outcome, below=of root] (outcome1) {Outcome};\n");
        latex.push('\n');
        latex.push_str("% Edges\n");
        latex.push_str("\\draw[arrow] (root) -- (outcome1);\n");
        latex.push('\n');
        latex.push_str("\\end{tikzpicture}\n");
        latex.push('\n');
        latex.push_str("\\end{document}\n");
        latex
    }
    /// Exports a dependency graph to LaTeX/TikZ format.
    pub fn export_dependency_graph(&self, graph: &DependencyGraph) -> String {
        let mut latex = String::new();
        if self.standalone {
            latex.push_str("\\documentclass[tikz,border=10pt]{standalone}\n");
        } else {
            latex.push_str(&format!("\\documentclass{{{}}}\n", self.document_class));
        }
        latex.push_str("\\usepackage{tikz}\n");
        latex.push_str("\\usetikzlibrary{shapes,arrows,positioning,graphs,graphdrawing}\n");
        latex.push_str("\\usegdlibrary{force}\n");
        latex.push('\n');
        latex.push_str("\\begin{document}\n");
        latex.push('\n');
        latex.push_str("\\begin{tikzpicture}[\n");
        latex.push_str("    every node/.style={circle, draw, fill=blue!20, minimum size=1.5cm},\n");
        latex.push_str("    arrow/.style={->, >=stealth, thick}\n");
        latex.push_str("]\n\n");
        latex.push_str("\\graph[spring layout, node distance=3cm] {\n");
        for node_idx in graph.graph.node_indices() {
            let statute_id = &graph.graph[node_idx];
            latex.push_str(&format!("    \"{}\";\n", statute_id.replace('_', "\\_")));
        }
        latex.push_str("};\n\n");
        latex.push_str("\\end{tikzpicture}\n");
        latex.push('\n');
        latex.push_str("\\end{document}\n");
        latex
    }
}
/// Enforcement action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementActionType {
    /// Monetary fine
    Fine,
    /// Warning letter
    Warning,
    /// License suspension
    Suspension,
    /// Settlement agreement
    Settlement,
    /// Investigation initiated
    Investigation,
}
/// Represents a legal concept in the semantic network.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LegalConcept {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Concept description
    pub description: String,
    /// Category (e.g., "rights", "obligations", "procedures")
    pub category: String,
    /// Related statute IDs
    pub statute_ids: Vec<String>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}
impl LegalConcept {
    /// Creates a new legal concept.
    pub fn new(id: &str, name: &str, description: &str, category: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            statute_ids: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }
    /// Adds a statute reference.
    pub fn add_statute(&mut self, statute_id: &str) {
        self.statute_ids.push(statute_id.to_string());
    }
    /// Adds metadata.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}
/// News priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NewsPriority {
    /// Urgent/breaking news
    Urgent,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority
    Low,
}
/// Graph of legal concepts and their relationships.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConceptRelationshipGraph {
    /// Title of the graph
    pub title: String,
    /// Legal concepts in the graph
    pub concepts: Vec<LegalConcept>,
    /// Relationships between concepts
    pub relationships: Vec<ConceptRelationship>,
    /// Theme for visualization
    pub theme: Theme,
}
impl ConceptRelationshipGraph {
    /// Creates a new concept relationship graph.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            concepts: Vec::new(),
            relationships: Vec::new(),
            theme: Theme::light(),
        }
    }
    /// Adds a concept to the graph.
    pub fn add_concept(&mut self, concept: LegalConcept) {
        self.concepts.push(concept);
    }
    /// Adds a relationship to the graph.
    pub fn add_relationship(&mut self, relationship: ConceptRelationship) {
        self.relationships.push(relationship);
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Generates HTML visualization using D3.js force-directed graph.
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <script src=\"https://d3js.org/d3.v7.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; padding: 0; overflow: hidden; }\n");
        html.push_str(&format!(
            "        body {{ background-color: {}; }}\n",
            self.theme.background_color
        ));
        html.push_str("        #graph { width: 100vw; height: 100vh; }\n");
        html.push_str("        .node { cursor: pointer; }\n");
        html.push_str("        .node circle { stroke: #fff; stroke-width: 2px; }\n");
        html.push_str("        .node text { font: 12px sans-serif; pointer-events: none; }\n");
        html.push_str(&format!(
            "        .node text {{ fill: {}; }}\n",
            self.theme.text_color
        ));
        html.push_str("        .link { stroke-opacity: 0.6; fill: none; }\n");
        html.push_str("        .link-label { font: 10px sans-serif; pointer-events: none; }\n");
        html.push_str(&format!(
            "        .link-label {{ fill: {}; }}\n",
            self.theme.text_color
        ));
        html.push_str(
            "        .tooltip { position: absolute; padding: 8px; background: rgba(0,0,0,0.8); color: #fff; border-radius: 4px; pointer-events: none; opacity: 0; }\n",
        );
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("    <div id=\"graph\"></div>\n");
        html.push_str("    <div class=\"tooltip\" id=\"tooltip\"></div>\n");
        html.push_str("    <script>\n");
        html.push_str("        const nodes = [\n");
        for concept in &self.concepts {
            html.push_str(&format!(
                "            {{ id: '{}', name: '{}', category: '{}', description: '{}' }},\n",
                concept.id, concept.name, concept.category, concept.description
            ));
        }
        html.push_str("        ];\n\n");
        html.push_str("        const links = [\n");
        for rel in &self.relationships {
            html.push_str(
                &format!(
                    "            {{ source: '{}', target: '{}', type: '{}', color: '{}', strength: {} }},\n",
                    rel.from_id, rel.to_id, rel.relation_type.label(), rel.relation_type
                    .color(), rel.strength
                ),
            );
        }
        html.push_str("        ];\n\n");
        html.push_str("        const width = window.innerWidth;\n");
        html.push_str("        const height = window.innerHeight;\n\n");
        html.push_str("        const svg = d3.select('#graph').append('svg')\n");
        html.push_str("            .attr('width', width)\n");
        html.push_str("            .attr('height', height);\n\n");
        html.push_str("        const simulation = d3.forceSimulation(nodes)\n");
        html.push_str(
            "            .force('link', d3.forceLink(links).id(d => d.id).distance(150))\n",
        );
        html.push_str("            .force('charge', d3.forceManyBody().strength(-300))\n");
        html.push_str("            .force('center', d3.forceCenter(width / 2, height / 2));\n\n");
        html.push_str("        const link = svg.append('g')\n");
        html.push_str("            .selectAll('line')\n");
        html.push_str("            .data(links)\n");
        html.push_str("            .enter().append('line')\n");
        html.push_str("            .attr('class', 'link')\n");
        html.push_str("            .attr('stroke', d => d.color)\n");
        html.push_str("            .attr('stroke-width', d => d.strength * 2);\n\n");
        html.push_str("        const linkLabel = svg.append('g')\n");
        html.push_str("            .selectAll('text')\n");
        html.push_str("            .data(links)\n");
        html.push_str("            .enter().append('text')\n");
        html.push_str("            .attr('class', 'link-label')\n");
        html.push_str("            .text(d => d.type);\n\n");
        html.push_str("        const node = svg.append('g')\n");
        html.push_str("            .selectAll('g')\n");
        html.push_str("            .data(nodes)\n");
        html.push_str("            .enter().append('g')\n");
        html.push_str("            .attr('class', 'node')\n");
        html.push_str("            .call(d3.drag()\n");
        html.push_str("                .on('start', dragstarted)\n");
        html.push_str("                .on('drag', dragged)\n");
        html.push_str("                .on('end', dragended));\n\n");
        html.push_str("        node.append('circle')\n");
        html.push_str("            .attr('r', 10)\n");
        html.push_str("            .attr('fill', '#3498db');\n\n");
        html.push_str("        node.append('text')\n");
        html.push_str("            .attr('dx', 12)\n");
        html.push_str("            .attr('dy', '.35em')\n");
        html.push_str("            .text(d => d.name);\n\n");
        html.push_str("        const tooltip = d3.select('#tooltip');\n");
        html.push_str("        node.on('mouseover', function(event, d) {\n");
        html.push_str("            tooltip.transition().duration(200).style('opacity', 1);\n");
        html.push_str(
            "            tooltip.html(`<strong>${d.name}</strong><br/>${d.category}<br/>${d.description}`)\n",
        );
        html.push_str("                .style('left', (event.pageX + 10) + 'px')\n");
        html.push_str("                .style('top', (event.pageY - 10) + 'px');\n");
        html.push_str("        }).on('mouseout', function() {\n");
        html.push_str("            tooltip.transition().duration(500).style('opacity', 0);\n");
        html.push_str("        });\n\n");
        html.push_str("        simulation.on('tick', () => {\n");
        html.push_str("            link.attr('x1', d => d.source.x)\n");
        html.push_str("                .attr('y1', d => d.source.y)\n");
        html.push_str("                .attr('x2', d => d.target.x)\n");
        html.push_str("                .attr('y2', d => d.target.y);\n");
        html.push_str("            linkLabel.attr('x', d => (d.source.x + d.target.x) / 2)\n");
        html.push_str("                .attr('y', d => (d.source.y + d.target.y) / 2);\n");
        html.push_str("            node.attr('transform', d => `translate(${d.x},${d.y})`);\n");
        html.push_str("        });\n\n");
        html.push_str("        function dragstarted(event) {\n");
        html.push_str("            if (!event.active) simulation.alphaTarget(0.3).restart();\n");
        html.push_str("            event.subject.fx = event.subject.x;\n");
        html.push_str("            event.subject.fy = event.subject.y;\n");
        html.push_str("        }\n");
        html.push_str("        function dragged(event) {\n");
        html.push_str("            event.subject.fx = event.x;\n");
        html.push_str("            event.subject.fy = event.y;\n");
        html.push_str("        }\n");
        html.push_str("        function dragended(event) {\n");
        html.push_str("            if (!event.active) simulation.alphaTarget(0);\n");
        html.push_str("            event.subject.fx = null;\n");
        html.push_str("            event.subject.fy = null;\n");
        html.push_str("        }\n");
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>");
        html
    }
    /// Generates Mermaid diagram format.
    pub fn to_mermaid(&self) -> String {
        let mut diagram = String::new();
        diagram.push_str("graph TD\n");
        for concept in &self.concepts {
            diagram.push_str(&format!("    {}[\"{}\"]\n", concept.id, concept.name));
        }
        for rel in &self.relationships {
            diagram.push_str(&format!(
                "    {} -->|{}| {}\n",
                rel.from_id,
                rel.relation_type.label(),
                rel.to_id
            ));
        }
        diagram
    }
}
/// Jupyter notebook integration.
#[derive(Debug, Clone)]
pub struct JupyterNotebookIntegration {
    /// Notebook metadata
    pub metadata: HashMap<String, String>,
    /// Kernel name
    pub kernel: String,
}
impl JupyterNotebookIntegration {
    /// Creates a new Jupyter notebook integration.
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
            kernel: "python3".to_string(),
        }
    }
    /// Sets the kernel.
    pub fn with_kernel(mut self, kernel: &str) -> Self {
        self.kernel = kernel.to_string();
        self
    }
    /// Adds metadata.
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    /// Creates a Jupyter notebook with visualization code.
    pub fn create_notebook(&self, title: &str, description: &str) -> String {
        let mut notebook = String::new();
        notebook.push_str("{\n");
        notebook.push_str("  \"cells\": [\n");
        notebook.push_str("    {\n");
        notebook.push_str("      \"cell_type\": \"markdown\",\n");
        notebook.push_str("      \"metadata\": {},\n");
        notebook.push_str("      \"source\": [\n");
        notebook.push_str(&format!("        \"# {}\\n\",\n", title));
        notebook.push_str("        \"\\n\",\n");
        notebook.push_str(&format!("        \"{}\\n\"\n", description));
        notebook.push_str("      ]\n");
        notebook.push_str("    },\n");
        notebook.push_str("    {\n");
        notebook.push_str("      \"cell_type\": \"code\",\n");
        notebook.push_str("      \"execution_count\": null,\n");
        notebook.push_str("      \"metadata\": {},\n");
        notebook.push_str("      \"outputs\": [],\n");
        notebook.push_str("      \"source\": [\n");
        notebook.push_str("        \"# Import required libraries\\n\",\n");
        notebook.push_str("        \"import matplotlib.pyplot as plt\\n\",\n");
        notebook.push_str("        \"import networkx as nx\\n\",\n");
        notebook.push_str("        \"import pandas as pd\\n\",\n");
        notebook.push_str("        \"import numpy as np\\n\",\n");
        notebook.push_str("        \"from IPython.display import display, HTML\\n\"\n");
        notebook.push_str("      ]\n");
        notebook.push_str("    },\n");
        notebook.push_str("    {\n");
        notebook.push_str("      \"cell_type\": \"code\",\n");
        notebook.push_str("      \"execution_count\": null,\n");
        notebook.push_str("      \"metadata\": {},\n");
        notebook.push_str("      \"outputs\": [],\n");
        notebook.push_str("      \"source\": [\n");
        notebook.push_str("        \"# Create visualization\\n\",\n");
        notebook.push_str("        \"G = nx.DiGraph()\\n\",\n");
        notebook
            .push_str("        \"G.add_edges_from([('A', 'B'), ('B', 'C'), ('A', 'C')])\\n\",\n");
        notebook.push_str("        \"\\n\",\n");
        notebook.push_str("        \"plt.figure(figsize=(12, 8))\\n\",\n");
        notebook.push_str("        \"pos = nx.spring_layout(G)\\n\",\n");
        notebook.push_str(
            "        \"nx.draw(G, pos, with_labels=True, node_color='lightblue', \\n\",\n",
        );
        notebook.push_str(
            "        \"        node_size=2000, font_size=16, font_weight='bold', \\n\",\n",
        );
        notebook.push_str("        \"        arrows=True, arrowsize=20)\\n\",\n");
        notebook.push_str("        \"plt.title('Legal Statute Dependencies')\\n\",\n");
        notebook.push_str("        \"plt.axis('off')\\n\",\n");
        notebook.push_str("        \"plt.tight_layout()\\n\",\n");
        notebook.push_str("        \"plt.show()\\n\"\n");
        notebook.push_str("      ]\n");
        notebook.push_str("    }\n");
        notebook.push_str("  ],\n");
        notebook.push_str("  \"metadata\": {\n");
        notebook.push_str("    \"kernelspec\": {\n");
        notebook.push_str("      \"display_name\": \"Python 3\",\n");
        notebook.push_str("      \"language\": \"python\",\n");
        notebook.push_str(&format!("      \"name\": \"{}\"\n", self.kernel));
        notebook.push_str("    },\n");
        notebook.push_str("    \"language_info\": {\n");
        notebook.push_str("      \"codemirror_mode\": {\n");
        notebook.push_str("        \"name\": \"ipython\",\n");
        notebook.push_str("        \"version\": 3\n");
        notebook.push_str("      },\n");
        notebook.push_str("      \"file_extension\": \".py\",\n");
        notebook.push_str("      \"mimetype\": \"text/x-python\",\n");
        notebook.push_str("      \"name\": \"python\",\n");
        notebook.push_str("      \"nbconvert_exporter\": \"python\",\n");
        notebook.push_str("      \"pygments_lexer\": \"ipython3\",\n");
        notebook.push_str("      \"version\": \"3.8.0\"\n");
        notebook.push_str("    }\n");
        notebook.push_str("  },\n");
        notebook.push_str("  \"nbformat\": 4,\n");
        notebook.push_str("  \"nbformat_minor\": 4\n");
        notebook.push_str("}\n");
        notebook
    }
    /// Creates a notebook with decision tree visualization.
    pub fn create_decision_tree_notebook(&self, _tree: &DecisionTree) -> String {
        self.create_notebook(
            "Legal Decision Tree Analysis",
            "Interactive visualization and analysis of legal decision trees.",
        )
    }
    /// Creates a notebook with dependency graph visualization.
    pub fn create_dependency_graph_notebook(&self, graph: &DependencyGraph) -> String {
        let notebook = self.create_notebook(
            "Legal Statute Dependencies",
            "Network analysis of statute dependencies and relationships.",
        );
        let statute_count = graph.graph.node_count();
        notebook.replace(
            "Network analysis",
            &format!("Network analysis of {} statutes", statute_count),
        )
    }
    /// Generates Python code for interactive visualization.
    pub fn generate_python_code(&self, graph: &DependencyGraph) -> String {
        let mut code = String::new();
        code.push_str("# Python code for legal statute visualization\n");
        code.push_str("import networkx as nx\n");
        code.push_str("import matplotlib.pyplot as plt\n");
        code.push_str("from matplotlib.patches import FancyBboxPatch\n\n");
        code.push_str("# Create directed graph\n");
        code.push_str("G = nx.DiGraph()\n\n");
        code.push_str("# Add statute nodes\n");
        for node_idx in graph.graph.node_indices() {
            let statute_id = &graph.graph[node_idx];
            code.push_str(&format!("G.add_node('{}', type='statute')\n", statute_id));
        }
        code.push_str("\n# Add dependencies\n");
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                let source_id = &graph.graph[source];
                let target_id = &graph.graph[target];
                code.push_str(&format!("G.add_edge('{}', '{}')\n", source_id, target_id));
            }
        }
        code.push_str("\n# Create visualization\n");
        code.push_str("plt.figure(figsize=(14, 10))\n");
        code.push_str("pos = nx.spring_layout(G, k=2, iterations=50)\n");
        code.push_str(
            "nx.draw_networkx_nodes(G, pos, node_color='lightblue', node_size=3000, alpha=0.9)\n",
        );
        code.push_str("nx.draw_networkx_labels(G, pos, font_size=10, font_weight='bold')\n");
        code.push_str(
            "nx.draw_networkx_edges(G, pos, edge_color='gray', arrows=True, arrowsize=20, width=2)\n",
        );
        code.push_str(
            "plt.title('Legal Statute Dependency Network', fontsize=16, fontweight='bold')\n",
        );
        code.push_str("plt.axis('off')\n");
        code.push_str("plt.tight_layout()\n");
        code.push_str("plt.show()\n\n");
        code.push_str("# Print network statistics\n");
        code.push_str("print(f'Total statutes: {G.number_of_nodes()}')\n");
        code.push_str("print(f'Total dependencies: {G.number_of_edges()}')\n");
        code.push_str(
            "print(f'Average degree: {sum(dict(G.degree()).values()) / G.number_of_nodes():.2f}')\n",
        );
        code
    }
}
/// Smart data highlighter for visualizations.
pub struct SmartDataHighlighter {
    /// Highlight color
    pub(crate) highlight_color: String,
    /// Minimum importance for highlighting
    pub(crate) min_importance: f32,
}
impl SmartDataHighlighter {
    /// Creates a new smart data highlighter.
    pub fn new() -> Self {
        Self {
            highlight_color: "#ffeb3b".to_string(),
            min_importance: 0.7,
        }
    }
    /// Sets the highlight color.
    pub fn with_color(mut self, color: String) -> Self {
        self.highlight_color = color;
        self
    }
    /// Sets minimum importance threshold.
    pub fn with_min_importance(mut self, min_importance: f32) -> Self {
        self.min_importance = min_importance.clamp(0.0, 1.0);
        self
    }
    /// Generates highlighting rules for a decision tree.
    pub fn highlight_tree(&self, tree: &DecisionTree) -> Vec<HighlightRule> {
        let mut rules = Vec::new();
        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx) {
                match node {
                    DecisionNode::Discretion { .. } => {
                        rules.push(HighlightRule {
                            target_id: format!("node-{}", node_idx.index()),
                            color: "#ff9800".to_string(),
                            importance: 0.9,
                            reason: "Discretionary decision point".to_string(),
                        });
                    }
                    DecisionNode::Condition {
                        is_discretionary: true,
                        ..
                    } => {
                        rules.push(HighlightRule {
                            target_id: format!("node-{}", node_idx.index()),
                            color: "#ffc107".to_string(),
                            importance: 0.8,
                            reason: "Discretionary condition".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
        for node_idx in tree.graph.node_indices() {
            let out_degree = tree.graph.neighbors(node_idx).count();
            if out_degree > 3 {
                rules.push(HighlightRule {
                    target_id: format!("node-{}", node_idx.index()),
                    color: "#e91e63".to_string(),
                    importance: 0.75,
                    reason: format!("Complex node with {} branches", out_degree),
                });
            }
        }
        rules.retain(|r| r.importance >= self.min_importance);
        rules
    }
    /// Generates highlighting rules for a dependency graph.
    pub fn highlight_graph(&self, graph: &DependencyGraph) -> Vec<HighlightRule> {
        let mut rules = Vec::new();
        for node_idx in graph.graph.node_indices() {
            let incoming = graph
                .graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                .count();
            let outgoing = graph.graph.neighbors(node_idx).count();
            if (incoming > 3 || outgoing > 3)
                && let Some(statute_id) = graph.graph.node_weight(node_idx)
            {
                rules.push(HighlightRule {
                    target_id: statute_id.clone(),
                    color: "#9c27b0".to_string(),
                    importance: 0.85,
                    reason: format!("Hub statute ({} in, {} out)", incoming, outgoing),
                });
            }
        }
        rules.retain(|r| r.importance >= self.min_importance);
        rules
    }
}
/// Individual geographic point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    /// Point ID
    pub id: String,
    /// Point location
    pub location: GeoCoordinate,
    /// Point label
    pub label: String,
    /// Point data
    pub data: serde_json::Value,
}
/// Interactive visualization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveConfig {
    /// Enable zoom and pan controls
    pub enable_zoom_pan: bool,
    /// Enable node/edge hover tooltips
    pub enable_tooltips: bool,
    /// Enable click-to-expand for collapsed nodes
    pub enable_click_expand: bool,
    /// Enable search and highlight functionality
    pub enable_search: bool,
    /// Enable mini-map for navigation
    pub enable_minimap: bool,
    /// Initial zoom level (1.0 = 100%)
    pub initial_zoom: f64,
    /// Minimum zoom level
    pub min_zoom: f64,
    /// Maximum zoom level
    pub max_zoom: f64,
    /// Mini-map size (width, height in pixels)
    pub minimap_size: (u32, u32),
}
/// Gesture-based holographic interaction configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureConfig {
    /// Enable hand tracking
    pub enable_hand_tracking: bool,
    /// Enable pinch gestures
    pub enable_pinch: bool,
    /// Enable swipe gestures
    pub enable_swipe: bool,
    /// Enable rotation gestures
    pub enable_rotation: bool,
    /// Gesture sensitivity (0.0 to 1.0)
    pub sensitivity: f32,
}
/// AR legal document overlay visualizer.
pub struct ARDocumentOverlay {
    pub(crate) theme: Theme,
    pub(crate) config: AROverlayConfig,
}
impl ARDocumentOverlay {
    /// Creates a new AR document overlay.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
            config: AROverlayConfig::default(),
        }
    }
    /// Sets the color theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Sets the AR configuration.
    pub fn with_config(mut self, config: AROverlayConfig) -> Self {
        self.config = config;
        self
    }
    /// Generates AR HTML for document overlay.
    pub fn to_ar_html(&self, statute: &Statute) -> String {
        let tree = DecisionTree::from_statute(statute).unwrap_or_else(|_| DecisionTree::new());
        self.to_ar_html_tree(&tree)
    }
    /// Generates AR HTML for a decision tree overlay.
    pub fn to_ar_html_tree(&self, tree: &DecisionTree) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("<title>AR Document Overlay</title>\n");
        html.push_str("<style>\n");
        html.push_str(&self.generate_ar_styles());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("<div id=\"ar-container\">\n");
        html.push_str("<div class=\"controls\">\n");
        html.push_str("<button id=\"start-ar\">Start AR</button>\n");
        html.push_str("<div id=\"ar-status\">AR Ready</div>\n");
        html.push_str("</div>\n");
        html.push_str("<video id=\"camera-feed\" autoplay playsinline></video>\n");
        html.push_str("<canvas id=\"ar-overlay\"></canvas>\n");
        html.push_str("</div>\n");
        html.push_str(
            "<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n",
        );
        html.push_str("<script>\n");
        html.push_str(&self.generate_ar_javascript(tree));
        html.push_str("</script>\n");
        html.push_str("</body>\n</html>");
        html
    }
    fn generate_ar_styles(&self) -> String {
        "body {
    margin: 0;
    padding: 0;
    overflow: hidden;
    font-family: Arial, sans-serif;
}

#ar-container {
    position: relative;
    width: 100vw;
    height: 100vh;
}

#camera-feed {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
}

#ar-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
}

.controls {
    position: absolute;
    top: 20px;
    left: 20px;
    z-index: 1000;
}

#start-ar {
    padding: 12px 24px;
    font-size: 16px;
    font-weight: bold;
    background: #2196f3;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

#ar-status {
    margin-top: 10px;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.7);
    color: white;
    border-radius: 4px;
}
"
        .to_string()
    }
    fn generate_ar_javascript(&self, tree: &DecisionTree) -> String {
        let nodes = self.extract_tree_nodes(tree);
        format!(
            "// AR Document Overlay
const config = {{
    enableMarkers: {},
    enableMarkerless: {},
    enableFaceTracking: {},
    markerSize: {},
    overlayOpacity: {}
}};

const nodes = {};

let video, canvas, ctx;
let scene, camera, renderer;
let arSession = null;

async function init() {{
    video = document.getElementById('camera-feed');
    canvas = document.getElementById('ar-overlay');
    ctx = canvas.getContext('2d');

    // Setup canvas
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // Setup Three.js for AR
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

    renderer = new THREE.WebGLRenderer({{
        canvas: canvas,
        alpha: true,
        antialias: true
    }});
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.xr.enabled = true;

    // Setup AR button
    document.getElementById('start-ar').addEventListener('click', startAR);

    window.addEventListener('resize', onResize);
}}

async function startAR() {{
    try {{
        // Request camera access
        const stream = await navigator.mediaDevices.getUserMedia({{
            video: {{ facingMode: 'environment' }}
        }});

        video.srcObject = stream;
        await video.play();

        document.getElementById('ar-status').textContent = 'AR Active';

        // Check for WebXR AR support
        if (navigator.xr) {{
            const supported = await navigator.xr.isSessionSupported('immersive-ar');

            if (supported) {{
                arSession = await navigator.xr.requestSession('immersive-ar', {{
                    requiredFeatures: ['hit-test'],
                    optionalFeatures: ['dom-overlay']
                }});

                renderer.xr.setSession(arSession);
                createAROverlay();

                arSession.addEventListener('end', () => {{
                    arSession = null;
                    document.getElementById('ar-status').textContent = 'AR Ended';
                }});
            }} else {{
                // Fallback to marker-based AR
                createMarkerBasedAR();
            }}
        }} else {{
            // No WebXR, use camera-based overlay
            createCameraOverlay();
        }}

        render();
    }} catch (error) {{
        console.error('Failed to start AR:', error);
        document.getElementById('ar-status').textContent = 'AR Error: ' + error.message;
    }}
}}

function createAROverlay() {{
    // Create virtual content for AR
    nodes.forEach((node, index) => {{
        const geometry = new THREE.BoxGeometry(0.1, 0.1, 0.1);
        let color;

        switch(node.type) {{
            case 'condition':
                color = 0x3498db;
                break;
            case 'discretion':
                color = 0xe74c3c;
                break;
            case 'outcome':
                color = 0x2ecc71;
                break;
            default:
                color = 0x999999;
        }}

        const material = new THREE.MeshBasicMaterial({{
            color,
            transparent: true,
            opacity: config.overlayOpacity
        }});
        const cube = new THREE.Mesh(geometry, material);

        // Position in a grid
        const row = Math.floor(index / 3);
        const col = index % 3;
        cube.position.set(
            (col - 1) * 0.3,
            1.5 + (row * 0.3),
            -1
        );

        scene.add(cube);
    }});
}}

function createMarkerBasedAR() {{
    // Implement marker-based AR tracking
    console.log('Using marker-based AR');
    drawMarkerOverlay();
}}

function createCameraOverlay() {{
    // Simple camera-based overlay
    console.log('Using camera overlay');
    drawCameraOverlay();
}}

function drawMarkerOverlay() {{
    // Draw AR markers and overlays on canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.globalAlpha = config.overlayOpacity;

    nodes.forEach((node, index) => {{
        const x = 100 + (index * 150);
        const y = 100 + (Math.floor(index / 3) * 100);

        // Draw node box
        ctx.fillStyle = node.type === 'condition' ? '#3498db' :
                        node.type === 'discretion' ? '#e74c3c' : '#2ecc71';
        ctx.fillRect(x, y, 120, 60);

        // Draw text
        ctx.fillStyle = 'white';
        ctx.font = 'bold 14px Arial';
        ctx.fillText(node.label, x + 10, y + 30);
    }});

    ctx.globalAlpha = 1.0;
}}

function drawCameraOverlay() {{
    drawMarkerOverlay();
}}

function render() {{
    if (!arSession) {{
        // Non-WebXR rendering
        drawMarkerOverlay();
        requestAnimationFrame(render);
    }} else {{
        // WebXR AR rendering
        renderer.render(scene, camera);
    }}
}}

function onResize() {{
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
}}

init();
",
            self.config.enable_markers,
            self.config.enable_markerless,
            self.config.enable_face_tracking,
            self.config.marker_size,
            self.config.overlay_opacity,
            serde_json::to_string_pretty(&nodes).unwrap_or_else(|_| "[]".to_string())
        )
    }
    fn extract_tree_nodes(&self, tree: &DecisionTree) -> Vec<serde_json::Value> {
        let mut nodes = Vec::new();
        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx) {
                let (node_type, label) = match node {
                    DecisionNode::Root { statute_id, .. } => ("root", statute_id.clone()),
                    DecisionNode::Condition {
                        description,
                        is_discretionary,
                    } => {
                        let node_type = if *is_discretionary {
                            "discretion"
                        } else {
                            "condition"
                        };
                        (node_type, description.clone())
                    }
                    DecisionNode::Outcome { description } => ("outcome", description.clone()),
                    DecisionNode::Discretion { issue, .. } => ("discretion", issue.clone()),
                };
                nodes.push(serde_json::json!({ "label" : label, "type" : node_type }));
            }
        }
        nodes
    }
}
/// Configuration for accessibility features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    /// Enable WCAG 2.1 AA compliance features
    pub wcag_aa_compliant: bool,
    /// Enable screen reader descriptions (ARIA labels)
    pub enable_screen_reader: bool,
    /// Enable keyboard navigation
    pub enable_keyboard_nav: bool,
    /// Use high contrast colors (minimum 4.5:1 ratio)
    pub high_contrast_mode: bool,
    /// Reduce or disable animations
    pub reduced_motion: bool,
    /// Minimum font size in pixels
    pub min_font_size: f32,
    /// Focus indicator color
    pub focus_color: String,
    /// Tab index for interactive elements
    pub tab_index_start: i32,
}
impl AccessibilityConfig {
    /// Creates a new accessibility configuration with WCAG 2.1 AA compliance.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a configuration optimized for screen readers.
    pub fn screen_reader_optimized() -> Self {
        Self {
            wcag_aa_compliant: true,
            enable_screen_reader: true,
            enable_keyboard_nav: true,
            high_contrast_mode: true,
            reduced_motion: true,
            min_font_size: 18.0,
            focus_color: "#0066cc".to_string(),
            tab_index_start: 0,
        }
    }
    /// Creates a configuration with reduced motion for users sensitive to animation.
    pub fn reduced_motion() -> Self {
        Self {
            reduced_motion: true,
            ..Self::default()
        }
    }
    /// Creates a configuration with high contrast for users with low vision.
    pub fn high_contrast() -> Self {
        Self {
            high_contrast_mode: true,
            min_font_size: 18.0,
            ..Self::default()
        }
    }
}
/// Case citation network visualizer
#[derive(Debug, Clone)]
pub struct CaseCitationNetworkVisualizer {
    pub(crate) theme: Theme,
}
impl CaseCitationNetworkVisualizer {
    /// Creates a new case citation network visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Renders citation network to HTML with D3.js.
    #[allow(clippy::too_many_arguments)]
    pub fn to_html(&self, cases: &[CaseCitation]) -> String {
        let nodes_json = serde_json::to_string(cases).unwrap_or_else(|_| "[]".to_string());
        let html = format!(
            "<!DOCTYPE html>\n\
<html>\n\
<head>\n\
    <meta charset=\"UTF-8\">\n\
    <script src=\"https://d3js.org/d3.v7.min.js\"></script>\n\
    <style>\n\
        body {{\n\
            margin: 0;\n\
            background-color: {};\n\
            color: {};\n\
            font-family: Arial, sans-serif;\n\
        }}\n\
        #graph {{\n\
            width: 100vw;\n\
            height: 100vh;\n\
        }}\n\
        .node {{\n\
            stroke: {};\n\
            stroke-width: 2px;\n\
            cursor: pointer;\n\
        }}\n\
        .link {{\n\
            stroke: {};\n\
            stroke-opacity: 0.6;\n\
            fill: none;\n\
        }}\n\
        .label {{\n\
            font-size: 12px;\n\
            fill: {};\n\
            pointer-events: none;\n\
        }}\n\
    </style>\n\
</head>\n\
<body>\n\
    <svg id=\"graph\"></svg>\n\
    <script>\n\
        const data = {};\n\
\n\
        const width = window.innerWidth;\n\
        const height = window.innerHeight;\n\
\n\
        const svg = d3.select(\"#graph\")\n\
            .attr(\"width\", width)\n\
            .attr(\"height\", height);\n\
\n\
        const nodes = data.map(d => ({{{{ id: d.id, name: d.name, year: d.year, court: d.court }}}}));\n\
        const links = [];\n\
        data.forEach(d => {{\n\
            d.citations.forEach(target => {{\n\
                links.push({{{{ source: d.id, target: target }}}});\n\
            }});\n\
        }});\n\
\n\
        const simulation = d3.forceSimulation(nodes)\n\
            .force(\"link\", d3.forceLink(links).id(d => d.id))\n\
            .force(\"charge\", d3.forceManyBody().strength(-300))\n\
            .force(\"center\", d3.forceCenter(width / 2, height / 2));\n\
\n\
        const link = svg.append(\"g\")\n\
            .selectAll(\"line\")\n\
            .data(links)\n\
            .enter().append(\"line\")\n\
            .attr(\"class\", \"link\");\n\
\n\
        const node = svg.append(\"g\")\n\
            .selectAll(\"circle\")\n\
            .data(nodes)\n\
            .enter().append(\"circle\")\n\
            .attr(\"class\", \"node\")\n\
            .attr(\"r\", 8)\n\
            .attr(\"fill\", \"{}\")\n\
            .call(d3.drag()\n\
                .on(\"start\", dragstarted)\n\
                .on(\"drag\", dragged)\n\
                .on(\"end\", dragended));\n\
\n\
        const label = svg.append(\"g\")\n\
            .selectAll(\"text\")\n\
            .data(nodes)\n\
            .enter().append(\"text\")\n\
            .attr(\"class\", \"label\")\n\
            .text(d => d.name)\n\
            .attr(\"text-anchor\", \"middle\");\n\
\n\
        simulation.on(\"tick\", () => {{\n\
            link.attr(\"x1\", d => d.source.x)\n\
                .attr(\"y1\", d => d.source.y)\n\
                .attr(\"x2\", d => d.target.x)\n\
                .attr(\"y2\", d => d.target.y);\n\
\n\
            node.attr(\"cx\", d => d.x)\n\
                .attr(\"cy\", d => d.y);\n\
\n\
            label.attr(\"x\", d => d.x)\n\
                .attr(\"y\", d => d.y - 12);\n\
        }});\n\
\n\
        function dragstarted(event) {{\n\
            if (!event.active) simulation.alphaTarget(0.3).restart();\n\
            event.subject.fx = event.subject.x;\n\
            event.subject.fy = event.subject.y;\n\
        }}\n\
\n\
        function dragged(event) {{\n\
            event.subject.fx = event.x;\n\
            event.subject.fy = event.y;\n\
        }}\n\
\n\
        function dragended(event) {{\n\
            if (!event.active) simulation.alphaTarget(0);\n\
            event.subject.fx = null;\n\
            event.subject.fy = null;\n\
        }}\n\
    </script>\n\
</body>\n\
</html>",
            self.theme.background_color,
            self.theme.text_color,
            self.theme.link_color,
            self.theme.link_color,
            self.theme.text_color,
            nodes_json,
            self.theme.condition_color,
        );
        html
    }
    /// Renders citation network to Mermaid.
    pub fn to_mermaid(&self, cases: &[CaseCitation]) -> String {
        let mut diagram = String::from("graph LR\n");
        for case in cases {
            let node_id = case.id.replace('-', "_");
            diagram.push_str(&format!("    {}[\"{}\"]\n", node_id, case.name));
            for citation in &case.citations {
                let citation_id = citation.replace('-', "_");
                diagram.push_str(&format!("    {} --> {}\n", node_id, citation_id));
            }
        }
        diagram
    }
}
/// Configuration for AR document overlay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AROverlayConfig {
    /// Enable marker-based AR
    pub enable_markers: bool,
    /// Enable markerless AR (SLAM)
    pub enable_markerless: bool,
    /// Enable face tracking
    pub enable_face_tracking: bool,
    /// Marker size in meters
    pub marker_size: f32,
    /// Overlay opacity (0.0-1.0)
    pub overlay_opacity: f32,
}
/// Annotation for judicial notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Annotation ID
    pub id: String,
    /// Target node or element
    pub target: String,
    /// Annotation text
    pub text: String,
    /// Citation (e.g., case law reference)
    pub citation: Option<String>,
    /// Author (e.g., judge, commentator)
    pub author: Option<String>,
    /// Date of annotation
    pub date: Option<String>,
    /// Annotation type (note, warning, interpretation, etc.)
    pub annotation_type: AnnotationType,
}
impl Annotation {
    /// Creates a new annotation.
    pub fn new(id: &str, target: &str, text: &str) -> Self {
        Self {
            id: id.to_string(),
            target: target.to_string(),
            text: text.to_string(),
            citation: None,
            author: None,
            date: None,
            annotation_type: AnnotationType::Note,
        }
    }
    /// Sets the citation.
    pub fn with_citation(mut self, citation: &str) -> Self {
        self.citation = Some(citation.to_string());
        self
    }
    /// Sets the author.
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }
    /// Sets the date.
    pub fn with_date(mut self, date: &str) -> Self {
        self.date = Some(date.to_string());
        self
    }
    /// Sets the annotation type.
    pub fn with_type(mut self, annotation_type: AnnotationType) -> Self {
        self.annotation_type = annotation_type;
        self
    }
}
/// Types of animations available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    /// Fade in
    FadeIn,
    /// Fade out
    FadeOut,
    /// Slide from left
    SlideInLeft,
    /// Slide from right
    SlideInRight,
    /// Slide from top
    SlideInTop,
    /// Slide from bottom
    SlideInBottom,
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Highlight (color pulse)
    Highlight,
    /// Progressive reveal (for lists)
    ProgressiveReveal,
}
