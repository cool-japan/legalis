//! Pre-built, versioned and customizable visualization templates.
//!
//! A [`VisualizationTemplate`] captures the *intent* of a visualization — what
//! kind of chart, which colors, how it is laid out, and a bag of renderer
//! options — as serializable, reusable data. Templates never render anything
//! themselves; instead they describe settings that callers apply to the
//! crate's existing renderers (for example by turning a [`TemplateStyle`] into a
//! [`Theme`] with [`TemplateStyle::to_theme`]).
//!
//! Three concerns are handled here:
//!
//! - **Customization** — a [`TemplateCustomization`] is a sparse overlay of
//!   optional overrides. [`VisualizationTemplate::apply`] layers it onto a base
//!   template and returns a new, independent template, leaving the base intact.
//! - **Library & examples** — [`TemplateLibrary::builtin`] returns a curated set
//!   of ready-to-use templates spanning litigation, compliance, legislative and
//!   academic use cases. Libraries can be queried by kind, category or tag.
//! - **Versioning & exchange** — each template carries a semantic
//!   [`TemplateVersion`] and a changelog; [`TemplateLibrary::add_versioned`]
//!   refuses to register an older or equal version of an existing template, and
//!   every type round-trips through JSON via `to_json` / `from_json`.
//!
//! [`Theme`]: crate::Theme

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::functions::VizResult;
use crate::types_5::VizError;
use crate::types_10::Theme;

/// The kind of visualization a template targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemplateKind {
    /// An eligibility / reasoning decision tree.
    DecisionTree,
    /// A statute dependency graph.
    DependencyGraph,
    /// A temporal timeline of legal events.
    Timeline,
    /// A population distribution chart.
    PopulationChart,
    /// A multi-track comparative timeline.
    ComparativeTimeline,
}

impl TemplateKind {
    /// Returns a stable, human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            TemplateKind::DecisionTree => "Decision Tree",
            TemplateKind::DependencyGraph => "Dependency Graph",
            TemplateKind::Timeline => "Timeline",
            TemplateKind::PopulationChart => "Population Chart",
            TemplateKind::ComparativeTimeline => "Comparative Timeline",
        }
    }
}

/// Broad use-case grouping for a template, used for library filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemplateCategory {
    /// Templates for litigation and case work.
    Litigation,
    /// Templates for regulatory compliance reporting.
    Compliance,
    /// Templates for legislative / rule-making processes.
    Legislative,
    /// Templates for academic and research publications.
    Academic,
    /// Templates tuned for slide decks and live presentation.
    Presentation,
    /// General-purpose templates.
    General,
}

impl TemplateCategory {
    /// Returns a stable, human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            TemplateCategory::Litigation => "Litigation",
            TemplateCategory::Compliance => "Compliance",
            TemplateCategory::Legislative => "Legislative",
            TemplateCategory::Academic => "Academic",
            TemplateCategory::Presentation => "Presentation",
            TemplateCategory::General => "General",
        }
    }
}

/// Layout orientation hint for a template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Orientation {
    /// Lay out left-to-right.
    Horizontal,
    /// Lay out top-to-bottom.
    Vertical,
}

/// A semantic `major.minor.patch` version for a template.
///
/// Ordering is the usual lexicographic order over the three components, which
/// is what [`TemplateLibrary::add_versioned`] relies on to reject downgrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TemplateVersion {
    /// Incompatible / breaking changes.
    pub major: u32,
    /// Backwards-compatible additions.
    pub minor: u32,
    /// Backwards-compatible fixes.
    pub patch: u32,
}

impl TemplateVersion {
    /// Creates a new version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parses a `"major.minor.patch"` string.
    ///
    /// Returns [`VizError::InvalidStructure`] if the string does not contain
    /// exactly three dot-separated unsigned integers.
    pub fn parse(text: &str) -> VizResult<Self> {
        let parts: Vec<&str> = text.trim().split('.').collect();
        if parts.len() != 3 {
            return Err(VizError::InvalidStructure(format!(
                "template version must be 'major.minor.patch', got '{}'",
                text
            )));
        }
        let parse_part = |raw: &str| -> VizResult<u32> {
            raw.trim().parse::<u32>().map_err(|_| {
                VizError::InvalidStructure(format!("invalid version component '{}'", raw))
            })
        };
        Ok(Self {
            major: parse_part(parts[0])?,
            minor: parse_part(parts[1])?,
            patch: parse_part(parts[2])?,
        })
    }

    /// Renders the version as a `"major.minor.patch"` string.
    pub fn version_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// Returns this version with the major component incremented and the lower
    /// components reset.
    pub fn bumped_major(self) -> Self {
        Self::new(self.major + 1, 0, 0)
    }

    /// Returns this version with the minor component incremented and the patch
    /// reset.
    pub fn bumped_minor(self) -> Self {
        Self::new(self.major, self.minor + 1, 0)
    }

    /// Returns this version with the patch component incremented.
    pub fn bumped_patch(self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1)
    }
}

impl Default for TemplateVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

/// A serializable color set, mirroring [`Theme`] without depending on its
/// representation, so templates can be exchanged independently of the live
/// theme type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateStyle {
    /// Color for root nodes.
    pub root_color: String,
    /// Color for condition nodes.
    pub condition_color: String,
    /// Color for discretionary nodes.
    pub discretion_color: String,
    /// Color for outcome nodes.
    pub outcome_color: String,
    /// Color for links / edges.
    pub link_color: String,
    /// Background color.
    pub background_color: String,
    /// Text color.
    pub text_color: String,
}

impl TemplateStyle {
    /// Builds a style from an existing [`Theme`].
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            root_color: theme.root_color.clone(),
            condition_color: theme.condition_color.clone(),
            discretion_color: theme.discretion_color.clone(),
            outcome_color: theme.outcome_color.clone(),
            link_color: theme.link_color.clone(),
            background_color: theme.background_color.clone(),
            text_color: theme.text_color.clone(),
        }
    }

    /// Converts this style into a renderable [`Theme`].
    pub fn to_theme(&self) -> Theme {
        Theme {
            root_color: self.root_color.clone(),
            condition_color: self.condition_color.clone(),
            discretion_color: self.discretion_color.clone(),
            outcome_color: self.outcome_color.clone(),
            link_color: self.link_color.clone(),
            background_color: self.background_color.clone(),
            text_color: self.text_color.clone(),
        }
    }

    /// Overrides a single color by its field name.
    ///
    /// Recognized keys are `root_color`, `condition_color`, `discretion_color`,
    /// `outcome_color`, `link_color`, `background_color` and `text_color`.
    /// Unknown keys return `false` and leave the style unchanged.
    pub fn set_color(&mut self, key: &str, value: &str) -> bool {
        let slot = match key {
            "root_color" => &mut self.root_color,
            "condition_color" => &mut self.condition_color,
            "discretion_color" => &mut self.discretion_color,
            "outcome_color" => &mut self.outcome_color,
            "link_color" => &mut self.link_color,
            "background_color" => &mut self.background_color,
            "text_color" => &mut self.text_color,
            _ => return false,
        };
        *slot = value.to_string();
        true
    }
}

impl Default for TemplateStyle {
    fn default() -> Self {
        Self::from_theme(&Theme::light())
    }
}

/// Layout settings for a template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateLayout {
    /// Canvas width in pixels.
    pub width: u32,
    /// Canvas height in pixels.
    pub height: u32,
    /// Primary layout direction.
    pub orientation: Orientation,
    /// Whether to use a compact, space-saving layout.
    pub compact: bool,
}

impl Default for TemplateLayout {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            orientation: Orientation::Vertical,
            compact: false,
        }
    }
}

/// A single changelog entry recording why a template's version changed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateChange {
    /// The version this note describes.
    pub version: TemplateVersion,
    /// A human-readable description of the change.
    pub note: String,
}

/// A reusable, versioned and customizable visualization preset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationTemplate {
    /// Stable identifier (used as the library key).
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Longer description of the template's intent.
    pub description: String,
    /// The visualization kind this template targets.
    pub kind: TemplateKind,
    /// Use-case category.
    pub category: TemplateCategory,
    /// Semantic version.
    pub version: TemplateVersion,
    /// Color style.
    pub style: TemplateStyle,
    /// Layout settings.
    pub layout: TemplateLayout,
    /// Free-form renderer options (kept ordered for stable serialization).
    pub options: BTreeMap<String, String>,
    /// Search / filter tags.
    pub tags: Vec<String>,
    /// Ordered changelog.
    pub changelog: Vec<TemplateChange>,
}

impl VisualizationTemplate {
    /// Creates a new template with default style, layout and version `1.0.0`.
    pub fn new(id: &str, name: &str, kind: TemplateKind) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            kind,
            category: TemplateCategory::General,
            version: TemplateVersion::default(),
            style: TemplateStyle::default(),
            layout: TemplateLayout::default(),
            options: BTreeMap::new(),
            tags: Vec::new(),
            changelog: Vec::new(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Sets the category.
    pub fn with_category(mut self, category: TemplateCategory) -> Self {
        self.category = category;
        self
    }

    /// Sets the version.
    pub fn with_version(mut self, version: TemplateVersion) -> Self {
        self.version = version;
        self
    }

    /// Sets the style.
    pub fn with_style(mut self, style: TemplateStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the layout.
    pub fn with_layout(mut self, layout: TemplateLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Adds a renderer option.
    pub fn with_option(mut self, key: &str, value: &str) -> Self {
        self.options.insert(key.to_string(), value.to_string());
        self
    }

    /// Adds a tag (ignored if already present).
    pub fn with_tag(mut self, tag: &str) -> Self {
        if !self.tags.iter().any(|t| t == tag) {
            self.tags.push(tag.to_string());
        }
        self
    }

    /// Records a changelog entry and sets the template version to match it.
    pub fn with_change(mut self, version: TemplateVersion, note: &str) -> Self {
        self.version = version;
        self.changelog.push(TemplateChange {
            version,
            note: note.to_string(),
        });
        self
    }

    /// Returns `true` if this template carries the given tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Convenience: the style as a renderable [`Theme`].
    pub fn theme(&self) -> Theme {
        self.style.to_theme()
    }

    /// Applies a [`TemplateCustomization`] overlay, returning a new template and
    /// leaving `self` unchanged.
    pub fn apply(&self, customization: &TemplateCustomization) -> VisualizationTemplate {
        let mut result = self.clone();
        if let Some(name) = &customization.name {
            result.name = name.clone();
        }
        if let Some(description) = &customization.description {
            result.description = description.clone();
        }
        if let Some(category) = customization.category {
            result.category = category;
        }
        if let Some(style) = &customization.style {
            result.style = style.clone();
        }
        for (key, value) in &customization.style_overrides {
            result.style.set_color(key, value);
        }
        if let Some(layout) = &customization.layout {
            result.layout = layout.clone();
        }
        if let Some(width) = customization.width {
            result.layout.width = width;
        }
        if let Some(height) = customization.height {
            result.layout.height = height;
        }
        if let Some(orientation) = customization.orientation {
            result.layout.orientation = orientation;
        }
        if let Some(compact) = customization.compact {
            result.layout.compact = compact;
        }
        for (key, value) in &customization.set_options {
            result.options.insert(key.clone(), value.clone());
        }
        for key in &customization.removed_options {
            result.options.remove(key);
        }
        for tag in &customization.added_tags {
            if !result.tags.iter().any(|t| t == tag) {
                result.tags.push(tag.clone());
            }
        }
        result
    }

    /// Serializes the template to pretty JSON.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| {
            VizError::ExportError(format!("template JSON serialization failed: {}", e))
        })
    }

    /// Serializes the template to compact JSON.
    pub fn to_json_compact(&self) -> VizResult<String> {
        serde_json::to_string(self).map_err(|e| {
            VizError::ExportError(format!("template JSON serialization failed: {}", e))
        })
    }

    /// Deserializes a template from JSON.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::ExportError(format!("template JSON parse failed: {}", e)))
    }
}

/// A sparse overlay of optional overrides applied to a base template.
///
/// Every field is optional; only the ones that are set take effect. This makes
/// it possible to express small, composable tweaks ("use my brand background,
/// make it compact") without copying the entire base template.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateCustomization {
    /// Override the name.
    pub name: Option<String>,
    /// Override the description.
    pub description: Option<String>,
    /// Override the category.
    pub category: Option<TemplateCategory>,
    /// Replace the entire style.
    pub style: Option<TemplateStyle>,
    /// Override individual colors by field name (applied after `style`).
    pub style_overrides: BTreeMap<String, String>,
    /// Replace the entire layout.
    pub layout: Option<TemplateLayout>,
    /// Override the layout width.
    pub width: Option<u32>,
    /// Override the layout height.
    pub height: Option<u32>,
    /// Override the layout orientation.
    pub orientation: Option<Orientation>,
    /// Override the compact flag.
    pub compact: Option<bool>,
    /// Options to set or replace.
    pub set_options: BTreeMap<String, String>,
    /// Option keys to remove.
    pub removed_options: Vec<String>,
    /// Tags to add.
    pub added_tags: Vec<String>,
}

impl TemplateCustomization {
    /// Creates an empty customization (no overrides).
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Overrides the description.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Overrides the category.
    pub fn with_category(mut self, category: TemplateCategory) -> Self {
        self.category = Some(category);
        self
    }

    /// Replaces the entire style.
    pub fn with_style(mut self, style: TemplateStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Overrides a single color by field name.
    pub fn override_color(mut self, key: &str, value: &str) -> Self {
        self.style_overrides
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Overrides the layout width.
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Overrides the layout height.
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Overrides the orientation.
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Overrides the compact flag.
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = Some(compact);
        self
    }

    /// Sets or replaces a renderer option.
    pub fn set_option(mut self, key: &str, value: &str) -> Self {
        self.set_options.insert(key.to_string(), value.to_string());
        self
    }

    /// Marks an option for removal.
    pub fn remove_option(mut self, key: &str) -> Self {
        self.removed_options.push(key.to_string());
        self
    }

    /// Adds a tag to apply.
    pub fn add_tag(mut self, tag: &str) -> Self {
        self.added_tags.push(tag.to_string());
        self
    }
}

/// An indexed collection of templates with filtering and versioned upgrades.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateLibrary {
    templates: BTreeMap<String, VisualizationTemplate>,
}

impl TemplateLibrary {
    /// Creates an empty library.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a curated library of ready-to-use example templates.
    pub fn builtin() -> Self {
        let mut library = Self::new();
        for template in builtin_templates() {
            library.add(template);
        }
        library
    }

    /// Inserts a template, replacing any existing one with the same id and
    /// returning the replaced template.
    pub fn add(&mut self, template: VisualizationTemplate) -> Option<VisualizationTemplate> {
        self.templates.insert(template.id.clone(), template)
    }

    /// Registers a template only if it is newer than any existing one with the
    /// same id.
    ///
    /// Returns the replaced template (if any) on success, or
    /// [`VizError::ExportError`] if a template with the same id already exists
    /// at an equal or newer version.
    pub fn add_versioned(
        &mut self,
        template: VisualizationTemplate,
    ) -> VizResult<Option<VisualizationTemplate>> {
        if let Some(existing) = self.templates.get(&template.id)
            && existing.version >= template.version
        {
            return Err(VizError::ExportError(format!(
                "cannot register template '{}' at version {}: existing version {} is newer or equal",
                template.id,
                template.version.version_string(),
                existing.version.version_string()
            )));
        }
        Ok(self.add(template))
    }

    /// Looks up a template by id.
    pub fn get(&self, id: &str) -> Option<&VisualizationTemplate> {
        self.templates.get(id)
    }

    /// Removes and returns a template by id.
    pub fn remove(&mut self, id: &str) -> Option<VisualizationTemplate> {
        self.templates.remove(id)
    }

    /// Returns the number of templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Returns `true` if the library is empty.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    /// Returns `true` if a template with the given id exists.
    pub fn contains(&self, id: &str) -> bool {
        self.templates.contains_key(id)
    }

    /// Returns all template ids in sorted order.
    pub fn ids(&self) -> Vec<&str> {
        self.templates.keys().map(String::as_str).collect()
    }

    /// Returns all templates in id order.
    pub fn all(&self) -> Vec<&VisualizationTemplate> {
        self.templates.values().collect()
    }

    /// Returns all templates in the given category.
    pub fn by_category(&self, category: TemplateCategory) -> Vec<&VisualizationTemplate> {
        self.templates
            .values()
            .filter(|t| t.category == category)
            .collect()
    }

    /// Returns all templates targeting the given kind.
    pub fn by_kind(&self, kind: TemplateKind) -> Vec<&VisualizationTemplate> {
        self.templates.values().filter(|t| t.kind == kind).collect()
    }

    /// Returns all templates carrying the given tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&VisualizationTemplate> {
        self.templates.values().filter(|t| t.has_tag(tag)).collect()
    }

    /// Serializes the library to pretty JSON.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VizError::ExportError(format!("library JSON serialization failed: {}", e)))
    }

    /// Deserializes a library from JSON.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::ExportError(format!("library JSON parse failed: {}", e)))
    }
}

/// Builds the curated set of example templates.
fn builtin_templates() -> Vec<VisualizationTemplate> {
    vec![
        VisualizationTemplate::new(
            "litigation-timeline",
            "Litigation Timeline",
            TemplateKind::Timeline,
        )
        .with_category(TemplateCategory::Litigation)
        .with_description("Chronological case milestones tuned for courtroom presentation.")
        .with_style(TemplateStyle::from_theme(&Theme::dark()))
        .with_layout(TemplateLayout {
            width: 1600,
            height: 600,
            orientation: Orientation::Horizontal,
            compact: false,
        })
        .with_option("show_dates", "true")
        .with_option("marker_style", "diamond")
        .with_tag("litigation")
        .with_tag("timeline")
        .with_change(
            TemplateVersion::new(1, 0, 0),
            "Initial litigation timeline preset.",
        ),
        VisualizationTemplate::new(
            "compliance-status",
            "Compliance Status Board",
            TemplateKind::PopulationChart,
        )
        .with_category(TemplateCategory::Compliance)
        .with_description("Status distribution across compliance requirements.")
        .with_style(TemplateStyle::from_theme(&Theme::high_contrast()))
        .with_option("chart_type", "stacked_bar")
        .with_option("legend", "true")
        .with_tag("compliance")
        .with_tag("dashboard")
        .with_change(
            TemplateVersion::new(1, 0, 0),
            "Initial compliance board preset.",
        ),
        VisualizationTemplate::new(
            "legislative-flow",
            "Legislative Process Flow",
            TemplateKind::DecisionTree,
        )
        .with_category(TemplateCategory::Legislative)
        .with_description("Step-by-step legislative decision flow.")
        .with_layout(TemplateLayout {
            width: 1000,
            height: 1400,
            orientation: Orientation::Vertical,
            compact: true,
        })
        .with_option("collapse_outcomes", "true")
        .with_tag("legislative")
        .with_tag("flow")
        .with_change(
            TemplateVersion::new(1, 0, 0),
            "Initial legislative flow preset.",
        ),
        VisualizationTemplate::new(
            "academic-network",
            "Academic Citation Network",
            TemplateKind::DependencyGraph,
        )
        .with_category(TemplateCategory::Academic)
        .with_description("Colorblind-safe network for publications.")
        .with_style(TemplateStyle::from_theme(&Theme::colorblind_friendly()))
        .with_option("layout", "force_directed")
        .with_option("label_font", "serif")
        .with_tag("academic")
        .with_tag("network")
        .with_change(
            TemplateVersion::new(1, 0, 0),
            "Initial academic network preset.",
        ),
        VisualizationTemplate::new(
            "jurisdiction-compare",
            "Cross-Jurisdiction Comparison",
            TemplateKind::ComparativeTimeline,
        )
        .with_category(TemplateCategory::General)
        .with_description("Side-by-side legal histories across jurisdictions.")
        .with_layout(TemplateLayout {
            width: 1800,
            height: 900,
            orientation: Orientation::Horizontal,
            compact: false,
        })
        .with_option("align_axis", "true")
        .with_option("highlight_sync", "true")
        .with_tag("comparison")
        .with_tag("timeline")
        .with_change(
            TemplateVersion::new(1, 0, 0),
            "Initial cross-jurisdiction comparison preset.",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_parse_round_trips() {
        let version = TemplateVersion::parse("2.5.13").expect("parse");
        assert_eq!(version, TemplateVersion::new(2, 5, 13));
        assert_eq!(version.version_string(), "2.5.13");
    }

    #[test]
    fn version_parse_rejects_malformed() {
        assert!(TemplateVersion::parse("1.2").is_err());
        assert!(TemplateVersion::parse("1.2.x").is_err());
        assert!(TemplateVersion::parse("1.2.3.4").is_err());
    }

    #[test]
    fn version_ordering_and_bumps() {
        let base = TemplateVersion::new(1, 4, 9);
        assert_eq!(base.bumped_patch(), TemplateVersion::new(1, 4, 10));
        assert_eq!(base.bumped_minor(), TemplateVersion::new(1, 5, 0));
        assert_eq!(base.bumped_major(), TemplateVersion::new(2, 0, 0));
        assert!(TemplateVersion::new(1, 0, 0) < TemplateVersion::new(1, 0, 1));
        assert!(TemplateVersion::new(1, 2, 0) < TemplateVersion::new(1, 10, 0));
    }

    #[test]
    fn style_theme_round_trip() {
        let theme = Theme::dark();
        let style = TemplateStyle::from_theme(&theme);
        let restored = style.to_theme();
        assert_eq!(restored.background_color, theme.background_color);
        assert_eq!(restored.condition_color, theme.condition_color);
    }

    #[test]
    fn style_set_color_rejects_unknown_key() {
        let mut style = TemplateStyle::default();
        assert!(style.set_color("background_color", "#000000"));
        assert_eq!(style.background_color, "#000000");
        assert!(!style.set_color("not_a_color", "#fff"));
    }

    #[test]
    fn template_json_round_trips() {
        let template = VisualizationTemplate::new("t1", "Test", TemplateKind::Timeline)
            .with_category(TemplateCategory::Litigation)
            .with_option("a", "1")
            .with_tag("x")
            .with_change(TemplateVersion::new(1, 2, 0), "added options");
        let json = template.to_json().expect("serialize");
        let restored = VisualizationTemplate::from_json(&json).expect("deserialize");
        assert_eq!(template, restored);
        assert_eq!(restored.version, TemplateVersion::new(1, 2, 0));
        assert_eq!(restored.changelog.len(), 1);
    }

    #[test]
    fn customization_layers_without_mutating_base() {
        let base = VisualizationTemplate::new("base", "Base", TemplateKind::DependencyGraph)
            .with_option("keep", "yes")
            .with_option("drop", "no")
            .with_tag("orig");
        let custom = TemplateCustomization::new()
            .with_name("Custom")
            .override_color("background_color", "#101010")
            .with_width(640)
            .with_compact(true)
            .set_option("added", "1")
            .remove_option("drop")
            .add_tag("extra");
        let applied = base.apply(&custom);

        // Base untouched.
        assert_eq!(base.name, "Base");
        assert_eq!(base.layout.width, TemplateLayout::default().width);
        assert!(base.options.contains_key("drop"));

        // Overlay applied.
        assert_eq!(applied.name, "Custom");
        assert_eq!(applied.style.background_color, "#101010");
        assert_eq!(applied.layout.width, 640);
        assert!(applied.layout.compact);
        assert_eq!(applied.options.get("added").map(String::as_str), Some("1"));
        assert!(!applied.options.contains_key("drop"));
        assert!(applied.options.contains_key("keep"));
        assert!(applied.has_tag("extra"));
        assert!(applied.has_tag("orig"));
    }

    #[test]
    fn customization_empty_is_identity() {
        let base =
            VisualizationTemplate::new("b", "B", TemplateKind::Timeline).with_option("k", "v");
        let applied = base.apply(&TemplateCustomization::new());
        assert_eq!(base, applied);
    }

    #[test]
    fn builtin_library_has_expected_examples() {
        let library = TemplateLibrary::builtin();
        assert!(library.len() >= 5);
        assert!(library.contains("litigation-timeline"));
        assert!(library.contains("jurisdiction-compare"));
        // Ids are returned sorted.
        let ids = library.ids();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
    }

    #[test]
    fn library_filters_by_kind_category_tag() {
        let library = TemplateLibrary::builtin();
        assert_eq!(library.by_kind(TemplateKind::Timeline).len(), 1);
        assert_eq!(library.by_category(TemplateCategory::Academic).len(), 1);
        assert!(!library.by_tag("timeline").is_empty());
        assert!(library.by_tag("nonexistent").is_empty());
    }

    #[test]
    fn library_versioned_rejects_downgrade_and_accepts_upgrade() {
        let mut library = TemplateLibrary::new();
        let v1 = VisualizationTemplate::new("p", "P", TemplateKind::Timeline)
            .with_version(TemplateVersion::new(1, 0, 0));
        library.add_versioned(v1).expect("first insert");

        let same = VisualizationTemplate::new("p", "P", TemplateKind::Timeline)
            .with_version(TemplateVersion::new(1, 0, 0));
        assert!(library.add_versioned(same).is_err());

        let older = VisualizationTemplate::new("p", "P", TemplateKind::Timeline)
            .with_version(TemplateVersion::new(0, 9, 0));
        assert!(library.add_versioned(older).is_err());

        let newer = VisualizationTemplate::new("p", "P-2", TemplateKind::Timeline)
            .with_version(TemplateVersion::new(1, 1, 0));
        let replaced = library.add_versioned(newer).expect("upgrade");
        assert!(replaced.is_some());
        assert_eq!(library.get("p").map(|t| t.name.as_str()), Some("P-2"));
    }

    #[test]
    fn library_json_round_trips() {
        let library = TemplateLibrary::builtin();
        let json = library.to_json().expect("serialize");
        let restored = TemplateLibrary::from_json(&json).expect("deserialize");
        assert_eq!(library, restored);
    }
}
