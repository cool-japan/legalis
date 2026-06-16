//! Accessibility extensions: non-visual and low-load alternatives.
//!
//! These types complement the existing [`AccessibilityEnhancer`] (which makes
//! the *HTML* output accessible) by producing genuinely alternative
//! representations of a visualization:
//!
//! - [`AudioDescriber`] / [`AudioDescription`] turn a [`DecisionTree`],
//!   [`DependencyGraph`] or [`Timeline`] into an ordered, narratable
//!   description that can be emitted as plain text, SSML (for speech
//!   synthesizers) or WebVTT (timed captions). No audio is synthesized here —
//!   the output is a structured *text* alternative that a TTS engine consumes.
//! - [`TactileExporter`] / [`TactileGraphic`] produce a raised-line model
//!   (points, lines, areas with textures) plus Unicode-braille labels via
//!   [`to_braille`]. Rendering physical tactile graphics requires a braille
//!   embosser or swell-paper printer; this module emits a portable *descriptor*
//!   (tactile-conventions SVG + braille text + a structured listing) that such
//!   hardware can consume.
//! - [`CognitiveLoadOptions`] chunk, summarize and de-clutter dense content for
//!   readers who benefit from reduced cognitive load.
//! - [`DyslexiaTextOptions`] reflow and style text with dyslexia-friendly
//!   typography (font, spacing, line length, off-white background).
//!
//! [`AccessibilityEnhancer`]: crate::AccessibilityEnhancer
//! [`DecisionTree`]: crate::DecisionTree
//! [`DependencyGraph`]: crate::DependencyGraph
//! [`Timeline`]: crate::Timeline

use petgraph::visit::Dfs;

use super::{escape_html, format_vtt_timestamp};
use crate::data_exchange::timeline_event_parts;
use crate::types_3::Timeline;
use crate::types_4::DependencyGraph;
use crate::types_5::AccessibilityEnhancer;
use crate::types_12::DecisionTree;

// ===========================================================================
// Audio descriptions (structured text alternatives)
// ===========================================================================

/// One narratable unit of an [`AudioDescription`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AudioSegment {
    /// Short heading announced before the body (e.g. "Step 1", "Overview").
    pub heading: String,
    /// The spoken body text.
    pub text: String,
    /// Pause to insert after this segment, in milliseconds.
    pub pause_ms: u32,
}

impl AudioSegment {
    /// Creates a new segment.
    pub fn new(heading: &str, text: &str, pause_ms: u32) -> Self {
        Self {
            heading: heading.to_string(),
            text: text.to_string(),
            pause_ms,
        }
    }

    /// Returns the spoken word count (heading + text).
    fn word_count(&self) -> u64 {
        (self.heading.split_whitespace().count() + self.text.split_whitespace().count()) as u64
    }
}

/// An ordered, structured spoken-description of a visualization.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AudioDescription {
    /// Title of the described visualization.
    pub title: String,
    /// Ordered narration segments.
    pub segments: Vec<AudioSegment>,
}

impl AudioDescription {
    /// Creates a new, empty description.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            segments: Vec::new(),
        }
    }

    /// Appends a segment.
    pub fn push(&mut self, segment: AudioSegment) {
        self.segments.push(segment);
    }

    /// Returns the number of segments.
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Estimates total narration time at `words_per_minute`, including pauses.
    pub fn estimated_duration_ms(&self, words_per_minute: u32) -> u64 {
        let wpm = words_per_minute.max(1) as u64;
        self.segments
            .iter()
            .map(|s| s.word_count() * 60_000 / wpm + u64::from(s.pause_ms))
            .sum()
    }

    /// Renders a plain-text transcript suitable for any text-to-speech engine.
    pub fn to_plain_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.title);
        out.push_str(".\n\n");
        for segment in &self.segments {
            if !segment.heading.is_empty() {
                out.push_str(&segment.heading);
                out.push_str(". ");
            }
            out.push_str(&segment.text);
            out.push_str("\n\n");
        }
        out
    }

    /// Renders SSML (Speech Synthesis Markup Language) with explicit breaks.
    pub fn to_ssml(&self) -> String {
        let mut out = String::from("<speak>\n");
        out.push_str(&format!("  <p><s>{}</s></p>\n", escape_html(&self.title)));
        for segment in &self.segments {
            out.push_str("  <p>");
            if !segment.heading.is_empty() {
                out.push_str(&format!("<s>{}</s>", escape_html(&segment.heading)));
            }
            out.push_str(&format!("<s>{}</s></p>\n", escape_html(&segment.text)));
            if segment.pause_ms > 0 {
                out.push_str(&format!("  <break time=\"{}ms\"/>\n", segment.pause_ms));
            }
        }
        out.push_str("</speak>");
        out
    }

    /// Renders WebVTT captions with timecodes derived from `words_per_minute`.
    pub fn to_webvtt(&self, words_per_minute: u32) -> String {
        let wpm = words_per_minute.max(1) as u64;
        let mut out = String::from("WEBVTT\n\n");
        let mut cursor = 0u64;
        for (index, segment) in self.segments.iter().enumerate() {
            let duration = (segment.word_count() * 60_000 / wpm).max(1_000);
            let start = cursor;
            let end = cursor + duration;
            out.push_str(&format!("{}\n", index + 1));
            out.push_str(&format!(
                "{} --> {}\n",
                format_vtt_timestamp(start),
                format_vtt_timestamp(end)
            ));
            if !segment.heading.is_empty() {
                out.push_str(&format!("{}: ", segment.heading));
            }
            out.push_str(&segment.text);
            out.push_str("\n\n");
            cursor = end + u64::from(segment.pause_ms);
        }
        out
    }
}

/// Builds [`AudioDescription`]s from the crate's visualization models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDescriber {
    /// Pause inserted after each segment, in milliseconds.
    pub pause_ms: u32,
    /// Maximum number of items listed before summarizing the remainder.
    pub max_list_items: usize,
}

impl Default for AudioDescriber {
    fn default() -> Self {
        Self {
            pause_ms: 400,
            max_list_items: 25,
        }
    }
}

impl AudioDescriber {
    /// Creates a describer with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the inter-segment pause.
    pub fn with_pause_ms(mut self, pause_ms: u32) -> Self {
        self.pause_ms = pause_ms;
        self
    }

    /// Sets the maximum number of listed items.
    pub fn with_max_list_items(mut self, max_list_items: usize) -> Self {
        self.max_list_items = max_list_items.max(1);
        self
    }

    /// Describes a decision tree by walking it in reading order.
    pub fn describe_tree(&self, tree: &DecisionTree) -> AudioDescription {
        let mut description = AudioDescription::new("Decision tree");
        let root = match tree.root {
            Some(root) => root,
            None => {
                description.push(AudioSegment::new(
                    "Overview",
                    "This decision tree is empty.",
                    self.pause_ms,
                ));
                return description;
            }
        };
        let enhancer = AccessibilityEnhancer::new();
        let mut dfs = Dfs::new(&tree.graph, root);
        let mut step = 0usize;
        while let Some(node_idx) = dfs.next(&tree.graph) {
            step += 1;
            let label = enhancer.aria_label_for_node(&tree.graph[node_idx]);
            description.push(AudioSegment::new(
                &format!("Node {}", step),
                &label,
                self.pause_ms,
            ));
        }
        description
    }

    /// Describes a dependency graph: an overview, then each node's outgoing
    /// dependencies.
    pub fn describe_graph(&self, graph: &DependencyGraph) -> AudioDescription {
        let mut description = AudioDescription::new("Statute dependency graph");
        let node_count = graph.node_count();
        let edge_count = graph.graph.edge_count();
        description.push(AudioSegment::new(
            "Overview",
            &format!(
                "This dependency graph has {} statute{} and {} relationship{}.",
                node_count,
                plural(node_count),
                edge_count,
                plural(edge_count)
            ),
            self.pause_ms,
        ));

        for (listed, node_idx) in graph.graph.node_indices().enumerate() {
            if listed >= self.max_list_items {
                let remaining = node_count - listed;
                description.push(AudioSegment::new(
                    "More",
                    &format!("And {} further statute{}.", remaining, plural(remaining)),
                    self.pause_ms,
                ));
                break;
            }
            let id = &graph.graph[node_idx];
            let mut deps: Vec<String> = Vec::new();
            for edge in graph.graph.edge_indices() {
                if let Some((source, target)) = graph.graph.edge_endpoints(edge)
                    && source == node_idx
                {
                    deps.push(format!("{} ({})", graph.graph[target], graph.graph[edge]));
                }
            }
            let body = if deps.is_empty() {
                format!("Statute {} has no outgoing dependencies.", id)
            } else {
                format!("Statute {} depends on {}.", id, deps.join(", "))
            };
            description.push(AudioSegment::new(id, &body, self.pause_ms));
        }
        description
    }

    /// Describes a timeline event by event.
    pub fn describe_timeline(&self, timeline: &Timeline) -> AudioDescription {
        let mut description = AudioDescription::new("Legal timeline");
        let total = timeline.events.len();
        description.push(AudioSegment::new(
            "Overview",
            &format!("This timeline has {} event{}.", total, plural(total)),
            self.pause_ms,
        ));
        for (date, event) in &timeline.events {
            let (kind, statute_id, detail) = timeline_event_parts(event);
            let body = match detail {
                Some(text) => format!("On {}, {} {}: {}.", date, kind, statute_id, text),
                None => format!("On {}, {} {}.", date, kind, statute_id),
            };
            description.push(AudioSegment::new(date, &body, self.pause_ms));
        }
        description
    }
}

/// Returns `"s"` unless `count` is exactly one.
fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

// ===========================================================================
// Tactile graphics (structured / SVG-tactile model + braille)
// ===========================================================================

/// The kind of a tactile primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TactilePrimitiveKind {
    /// A connected sequence of raised line segments.
    Line,
    /// A single raised point / dot.
    Point,
    /// A filled (textured) raised area.
    Area,
    /// A text label (rendered separately as braille).
    Label,
}

/// Surface texture for a tactile primitive, distinguishing categories by feel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TactileTexture {
    /// Continuous raised line.
    Solid,
    /// Long dashes.
    Dashed,
    /// Fine dots.
    Dotted,
    /// Cross-hatched fill.
    CrossHatch,
    /// Smooth (no distinguishing pattern).
    Smooth,
}

impl TactileTexture {
    /// Returns the SVG `stroke-dasharray` value for this texture.
    fn dash_array(&self) -> &'static str {
        match self {
            TactileTexture::Solid | TactileTexture::Smooth => "none",
            TactileTexture::Dashed => "10,6",
            TactileTexture::Dotted => "2,8",
            TactileTexture::CrossHatch => "10,4,2,4",
        }
    }
}

/// A single tactile element: points, a label, or a poly-line / poly-area.
#[derive(Debug, Clone, PartialEq)]
pub struct TactilePrimitive {
    /// Kind of element.
    pub kind: TactilePrimitiveKind,
    /// Geometry, in graphic coordinates.
    pub points: Vec<(f64, f64)>,
    /// Optional text label (translated to braille for output).
    pub label: Option<String>,
    /// Surface texture.
    pub texture: TactileTexture,
}

impl TactilePrimitive {
    /// Creates a raised point with a label.
    pub fn point(x: f64, y: f64, label: &str) -> Self {
        Self {
            kind: TactilePrimitiveKind::Point,
            points: vec![(x, y)],
            label: Some(label.to_string()),
            texture: TactileTexture::Solid,
        }
    }

    /// Creates a raised poly-line.
    pub fn line(points: Vec<(f64, f64)>, texture: TactileTexture) -> Self {
        Self {
            kind: TactilePrimitiveKind::Line,
            points,
            label: None,
            texture,
        }
    }

    /// Creates a raised, textured area.
    pub fn area(points: Vec<(f64, f64)>, texture: TactileTexture, label: Option<&str>) -> Self {
        Self {
            kind: TactilePrimitiveKind::Area,
            points,
            label: label.map(str::to_string),
            texture,
        }
    }

    /// Creates a standalone label anchored at a point.
    pub fn label(x: f64, y: f64, text: &str) -> Self {
        Self {
            kind: TactilePrimitiveKind::Label,
            points: vec![(x, y)],
            label: Some(text.to_string()),
            texture: TactileTexture::Smooth,
        }
    }
}

/// A portable tactile-graphic descriptor.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TactileGraphic {
    /// Title of the graphic.
    pub title: String,
    /// Canvas width.
    pub width: f64,
    /// Canvas height.
    pub height: f64,
    /// Ordered primitives.
    pub primitives: Vec<TactilePrimitive>,
}

impl TactileGraphic {
    /// Creates a new, empty tactile graphic.
    pub fn new(title: &str, width: f64, height: f64) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            primitives: Vec::new(),
        }
    }

    /// Adds a primitive.
    pub fn add(&mut self, primitive: TactilePrimitive) {
        self.primitives.push(primitive);
    }

    /// Returns the number of primitives.
    pub fn primitive_count(&self) -> usize {
        self.primitives.len()
    }

    /// Returns the `(text, braille)` pairs for every labelled primitive.
    pub fn braille_labels(&self) -> Vec<(String, String)> {
        self.primitives
            .iter()
            .filter_map(|p| p.label.as_ref())
            .map(|label| (label.clone(), to_braille(label)))
            .collect()
    }

    /// Renders a tactile-conventions SVG: thick black lines on white, textures
    /// instead of color, and large dots.
    pub fn to_svg(&self) -> String {
        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
            self.width, self.height, self.width, self.height
        ));
        svg.push_str(&format!(
            "  <rect width=\"{}\" height=\"{}\" fill=\"#ffffff\"/>\n",
            self.width, self.height
        ));
        svg.push_str(&format!(
            "  <text x=\"12\" y=\"24\" font-family=\"sans-serif\" font-size=\"16\" fill=\"#000000\">{}</text>\n",
            escape_html(&self.title)
        ));
        for primitive in &self.primitives {
            match primitive.kind {
                TactilePrimitiveKind::Point => {
                    if let Some(&(x, y)) = primitive.points.first() {
                        svg.push_str(&format!(
                            "  <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"7\" fill=\"#000000\"/>\n",
                            x, y
                        ));
                        if let Some(label) = &primitive.label {
                            svg.push_str(&format!(
                                "  <text x=\"{:.2}\" y=\"{:.2}\" font-family=\"sans-serif\" font-size=\"13\" fill=\"#000000\">{}</text>\n",
                                x + 12.0,
                                y + 4.0,
                                escape_html(label)
                            ));
                        }
                    }
                }
                TactilePrimitiveKind::Line => {
                    svg.push_str(&format!(
                        "  <polyline points=\"{}\" fill=\"none\" stroke=\"#000000\" stroke-width=\"3\" stroke-dasharray=\"{}\"/>\n",
                        points_attr(&primitive.points),
                        primitive.texture.dash_array()
                    ));
                }
                TactilePrimitiveKind::Area => {
                    svg.push_str(&format!(
                        "  <polygon points=\"{}\" fill=\"none\" stroke=\"#000000\" stroke-width=\"3\" stroke-dasharray=\"{}\"/>\n",
                        points_attr(&primitive.points),
                        primitive.texture.dash_array()
                    ));
                }
                TactilePrimitiveKind::Label => {
                    if let (Some(&(x, y)), Some(label)) =
                        (primitive.points.first(), primitive.label.as_ref())
                    {
                        svg.push_str(&format!(
                            "  <text x=\"{:.2}\" y=\"{:.2}\" font-family=\"sans-serif\" font-size=\"13\" fill=\"#000000\">{}</text>\n",
                            x,
                            y,
                            escape_html(label)
                        ));
                    }
                }
            }
        }
        svg.push_str("</svg>");
        svg
    }

    /// Renders a structured text descriptor with a braille legend.
    ///
    /// Physical reproduction requires a braille embosser or swell-paper
    /// printer; this descriptor is the portable, hardware-independent input to
    /// such a device.
    pub fn to_descriptor_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Tactile graphic: {}\n", self.title));
        out.push_str(&format!("Canvas: {:.0} x {:.0}\n", self.width, self.height));
        out.push_str(
            "Note: physical embossing requires a braille embosser or swell-paper printer; \
             this is a portable descriptor.\n\n",
        );
        for (index, primitive) in self.primitives.iter().enumerate() {
            let kind = match primitive.kind {
                TactilePrimitiveKind::Point => "point",
                TactilePrimitiveKind::Line => "line",
                TactilePrimitiveKind::Area => "area",
                TactilePrimitiveKind::Label => "label",
            };
            out.push_str(&format!(
                "{}. {} ({:?}, {} vertices)",
                index + 1,
                kind,
                primitive.texture,
                primitive.points.len()
            ));
            if let Some(label) = &primitive.label {
                out.push_str(&format!(" — \"{}\" [{}]", label, to_braille(label)));
            }
            out.push('\n');
        }
        out
    }
}

/// Translates ASCII text into uncontracted (Grade 1) Unicode braille.
///
/// Lower- and upper-case letters, digits and common punctuation are mapped;
/// upper-case letters are prefixed with the capital sign and digit runs with
/// the number sign. Characters with no Grade 1 mapping are dropped, since the
/// result is intended as a braille descriptor rather than a lossless transcode.
pub fn to_braille(text: &str) -> String {
    // Braille letters a..z, in order. The Unicode Braille Patterns block places
    // each cell at 0x2800 + dot bitmask; these are the standard letter cells.
    const LETTERS: &str = "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵";
    const NUMBER_SIGN: char = '⠼';
    const CAPITAL_SIGN: char = '⠠';

    let letters: Vec<char> = LETTERS.chars().collect();
    let mut out = String::new();
    let mut in_number = false;
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            if !in_number {
                out.push(NUMBER_SIGN);
                in_number = true;
            }
            let index = if ch == '0' {
                9
            } else {
                (ch as u8 - b'1') as usize
            };
            out.push(letters[index]);
            continue;
        }
        in_number = false;
        if ch.is_ascii_uppercase() {
            out.push(CAPITAL_SIGN);
            let index = (ch.to_ascii_lowercase() as u8 - b'a') as usize;
            out.push(letters[index]);
        } else if ch.is_ascii_lowercase() {
            let index = (ch as u8 - b'a') as usize;
            out.push(letters[index]);
        } else {
            match ch {
                ' ' => out.push(' '),
                '.' => out.push('⠲'),
                ',' => out.push('⠂'),
                '-' => out.push('⠤'),
                ';' => out.push('⠆'),
                ':' => out.push('⠒'),
                '?' => out.push('⠦'),
                '!' => out.push('⠖'),
                '\'' => out.push('⠄'),
                '/' => out.push('⠌'),
                _ => {}
            }
        }
    }
    out
}

/// Formats a list of points as an SVG `points` attribute value.
fn points_attr(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .map(|(x, y)| format!("{:.2},{:.2}", x, y))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Builds [`TactileGraphic`]s from the crate's visualization models.
#[derive(Debug, Clone, PartialEq)]
pub struct TactileExporter {
    /// Canvas size (square) used for laid-out graphics.
    pub canvas: f64,
    /// Margin kept clear around the content.
    pub margin: f64,
}

impl Default for TactileExporter {
    fn default() -> Self {
        Self {
            canvas: 800.0,
            margin: 80.0,
        }
    }
}

impl TactileExporter {
    /// Creates an exporter with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the (square) canvas size.
    pub fn with_canvas(mut self, canvas: f64) -> Self {
        self.canvas = canvas;
        self
    }

    /// Exports a dependency graph: nodes placed on a circle, edges as lines.
    pub fn export_graph(&self, graph: &DependencyGraph) -> TactileGraphic {
        let mut tactile = TactileGraphic::new("Dependency graph", self.canvas, self.canvas);
        let node_indices: Vec<_> = graph.graph.node_indices().collect();
        let count = node_indices.len();
        if count == 0 {
            return tactile;
        }
        let center = self.canvas / 2.0;
        let radius = center - self.margin;
        let mut positions = std::collections::HashMap::new();
        for (i, &node_idx) in node_indices.iter().enumerate() {
            let angle = std::f64::consts::TAU * (i as f64) / (count as f64);
            let x = center + radius * angle.cos();
            let y = center + radius * angle.sin();
            positions.insert(node_idx, (x, y));
        }
        // Edges first so points draw on top in the SVG.
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge)
                && let (Some(&from), Some(&to)) = (positions.get(&source), positions.get(&target))
            {
                tactile.add(TactilePrimitive::line(
                    vec![from, to],
                    TactileTexture::Solid,
                ));
            }
        }
        for &node_idx in &node_indices {
            if let Some(&(x, y)) = positions.get(&node_idx) {
                tactile.add(TactilePrimitive::point(x, y, &graph.graph[node_idx]));
            }
        }
        tactile
    }

    /// Exports a timeline: a horizontal base line with a labelled point per
    /// event.
    pub fn export_timeline(&self, timeline: &Timeline) -> TactileGraphic {
        let mut tactile = TactileGraphic::new("Timeline", self.canvas, self.canvas / 2.0);
        let count = timeline.events.len();
        let baseline_y = self.canvas / 4.0;
        tactile.add(TactilePrimitive::line(
            vec![
                (self.margin, baseline_y),
                (self.canvas - self.margin, baseline_y),
            ],
            TactileTexture::Solid,
        ));
        if count == 0 {
            return tactile;
        }
        let span = self.canvas - 2.0 * self.margin;
        for (i, (date, event)) in timeline.events.iter().enumerate() {
            let x = if count == 1 {
                self.canvas / 2.0
            } else {
                self.margin + span * (i as f64) / ((count - 1) as f64)
            };
            let (kind, statute_id, _) = timeline_event_parts(event);
            tactile.add(TactilePrimitive::point(
                x,
                baseline_y,
                &format!("{} {} {}", date, kind, statute_id),
            ));
        }
        tactile
    }
}

// ===========================================================================
// Cognitive-load reduction
// ===========================================================================

/// Options that reduce the cognitive load of dense content by chunking,
/// summarizing and removing decoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CognitiveLoadOptions {
    /// Maximum items shown before summarizing the remainder.
    pub max_items_per_view: usize,
    /// Items per chunk when paginating.
    pub chunk_size: usize,
    /// Prefer plain language over technical phrasing.
    pub plain_language: bool,
    /// Hide purely decorative elements.
    pub hide_decorative: bool,
    /// Lead with a summary before detail.
    pub summary_first: bool,
    /// Reveal a single concept at a time (forces chunk size to 1).
    pub one_concept_at_a_time: bool,
    /// Use generous whitespace.
    pub extra_whitespace: bool,
}

impl Default for CognitiveLoadOptions {
    fn default() -> Self {
        // Defaults follow common guidance (e.g. roughly seven items per view).
        Self {
            max_items_per_view: 7,
            chunk_size: 5,
            plain_language: true,
            hide_decorative: true,
            summary_first: true,
            one_concept_at_a_time: false,
            extra_whitespace: true,
        }
    }
}

impl CognitiveLoadOptions {
    /// Creates options with sensible defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Light-touch reduction: larger views, no decoration hiding.
    pub fn minimal() -> Self {
        Self {
            max_items_per_view: 20,
            chunk_size: 10,
            plain_language: false,
            hide_decorative: false,
            summary_first: false,
            one_concept_at_a_time: false,
            extra_whitespace: false,
        }
    }

    /// Maximum simplification: tiny views, one concept at a time.
    pub fn maximum_simplicity() -> Self {
        Self {
            max_items_per_view: 3,
            chunk_size: 3,
            plain_language: true,
            hide_decorative: true,
            summary_first: true,
            one_concept_at_a_time: true,
            extra_whitespace: true,
        }
    }

    /// Sets the maximum items per view.
    pub fn with_max_items(mut self, max_items: usize) -> Self {
        self.max_items_per_view = max_items.max(1);
        self
    }

    /// Sets the chunk size.
    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size.max(1);
        self
    }

    /// The effective chunk size, accounting for one-concept-at-a-time.
    pub fn effective_chunk_size(&self) -> usize {
        if self.one_concept_at_a_time {
            1
        } else {
            self.chunk_size.max(1)
        }
    }

    /// Splits items into chunks of [`effective_chunk_size`](Self::effective_chunk_size).
    pub fn chunk_items<'a>(&self, items: &'a [String]) -> Vec<&'a [String]> {
        items.chunks(self.effective_chunk_size()).collect()
    }

    /// Truncates a list to `max_items_per_view`, appending an "and N more"
    /// summary line when items were dropped.
    pub fn summarize(&self, items: &[String]) -> Vec<String> {
        if items.len() <= self.max_items_per_view {
            return items.to_vec();
        }
        let mut out: Vec<String> = items[..self.max_items_per_view].to_vec();
        let remaining = items.len() - self.max_items_per_view;
        out.push(format!(
            "… and {} more item{}",
            remaining,
            if remaining == 1 { "" } else { "s" }
        ));
        out
    }

    /// Generates CSS implementing the reduction options.
    pub fn to_css(&self) -> String {
        let mut css = String::from(".cognitive-reduced {\n");
        css.push_str("  max-width: 60ch;\n");
        if self.extra_whitespace {
            css.push_str("  line-height: 1.8;\n");
            css.push_str("  padding: 24px;\n");
        }
        css.push_str("}\n");
        if self.hide_decorative {
            css.push_str(".cognitive-reduced .decorative { display: none; }\n");
        }
        if self.extra_whitespace {
            css.push_str(".cognitive-reduced > * + * { margin-top: 1.2em; }\n");
        }
        css
    }
}

// ===========================================================================
// Dyslexia-friendly text rendering
// ===========================================================================

/// Options for dyslexia-friendly text styling and reflow.
#[derive(Debug, Clone, PartialEq)]
pub struct DyslexiaTextOptions {
    /// Font stack (dyslexia-friendly fonts first).
    pub font_family: String,
    /// Base font size in pixels.
    pub font_size_px: f32,
    /// Extra letter spacing, in `em`.
    pub letter_spacing_em: f32,
    /// Extra word spacing, in `em`.
    pub word_spacing_em: f32,
    /// Line height multiplier.
    pub line_height: f32,
    /// Background color (off-white reduces glare).
    pub background_color: String,
    /// Text color.
    pub text_color: String,
    /// Maximum line length in characters, used by [`reflow`](Self::reflow).
    pub max_line_length_ch: u32,
    /// Avoid italics (render emphasis differently).
    pub avoid_italics: bool,
    /// Use bold for emphasis.
    pub bold_emphasis: bool,
}

impl Default for DyslexiaTextOptions {
    fn default() -> Self {
        Self {
            font_family: "'OpenDyslexic', 'Comic Sans MS', Verdana, Arial, sans-serif".to_string(),
            font_size_px: 18.0,
            letter_spacing_em: 0.05,
            word_spacing_em: 0.16,
            line_height: 1.6,
            background_color: "#faf3e0".to_string(),
            text_color: "#1a1a1a".to_string(),
            max_line_length_ch: 66,
            avoid_italics: true,
            bold_emphasis: true,
        }
    }
}

impl DyslexiaTextOptions {
    /// Creates options with sensible defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// A preset emphasizing the OpenDyslexic font and wide spacing.
    pub fn open_dyslexic() -> Self {
        Self {
            font_family: "'OpenDyslexic', sans-serif".to_string(),
            letter_spacing_em: 0.08,
            word_spacing_em: 0.2,
            ..Self::default()
        }
    }

    /// A high-readability preset: larger text and shorter lines.
    pub fn high_readability() -> Self {
        Self {
            font_size_px: 22.0,
            line_height: 1.8,
            max_line_length_ch: 50,
            ..Self::default()
        }
    }

    /// Sets the font stack.
    pub fn with_font_family(mut self, font_family: &str) -> Self {
        self.font_family = font_family.to_string();
        self
    }

    /// Sets the base font size.
    pub fn with_font_size(mut self, font_size_px: f32) -> Self {
        self.font_size_px = font_size_px;
        self
    }

    /// Sets the maximum line length in characters.
    pub fn with_max_line_length(mut self, max_line_length_ch: u32) -> Self {
        self.max_line_length_ch = max_line_length_ch.max(1);
        self
    }

    /// Sets the background and text colors.
    pub fn with_colors(mut self, background: &str, text: &str) -> Self {
        self.background_color = background.to_string();
        self.text_color = text.to_string();
        self
    }

    /// Generates the CSS declarations (without a selector).
    pub fn to_css_declarations(&self) -> String {
        let mut css = String::new();
        css.push_str(&format!("  font-family: {};\n", self.font_family));
        css.push_str(&format!("  font-size: {}px;\n", self.font_size_px));
        css.push_str(&format!(
            "  letter-spacing: {}em;\n",
            self.letter_spacing_em
        ));
        css.push_str(&format!("  word-spacing: {}em;\n", self.word_spacing_em));
        css.push_str(&format!("  line-height: {};\n", self.line_height));
        css.push_str(&format!("  background-color: {};\n", self.background_color));
        css.push_str(&format!("  color: {};\n", self.text_color));
        css.push_str(&format!("  max-width: {}ch;\n", self.max_line_length_ch));
        css.push_str("  text-align: left;\n");
        css
    }

    /// Generates a complete CSS rule for the given selector.
    pub fn to_css_class(&self, selector: &str) -> String {
        let mut css = format!("{} {{\n", selector);
        css.push_str(&self.to_css_declarations());
        css.push_str("}\n");
        if self.avoid_italics {
            css.push_str(&format!(
                "{} em, {} i {{ font-style: normal;",
                selector, selector
            ));
            if self.bold_emphasis {
                css.push_str(" font-weight: bold;");
            }
            css.push_str(" }\n");
        }
        css
    }

    /// Wraps HTML content in a styled, dyslexia-friendly container.
    pub fn wrap_html(&self, inner_html: &str) -> String {
        format!(
            "<div class=\"dyslexia-friendly\" style=\"{}\">{}</div>",
            self.inline_style(),
            inner_html
        )
    }

    /// The inline-style string equivalent of the CSS declarations.
    fn inline_style(&self) -> String {
        format!(
            "font-family: {}; font-size: {}px; letter-spacing: {}em; word-spacing: {}em; \
             line-height: {}; background-color: {}; color: {}; max-width: {}ch; text-align: left;",
            self.font_family,
            self.font_size_px,
            self.letter_spacing_em,
            self.word_spacing_em,
            self.line_height,
            self.background_color,
            self.text_color,
            self.max_line_length_ch
        )
    }

    /// Reflows text to the configured maximum line length, breaking only at
    /// word boundaries (a word longer than the limit gets its own line).
    pub fn reflow(&self, text: &str) -> String {
        let max = self.max_line_length_ch.max(1) as usize;
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for word in text.split_whitespace() {
            let word_len = word.chars().count();
            if current.is_empty() {
                current.push_str(word);
            } else if current.chars().count() + 1 + word_len <= max {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(std::mem::take(&mut current));
                current.push_str(word);
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types_5::TimelineEvent;
    use legalis_core::{Effect, EffectType, Statute};

    fn sample_statute() -> Statute {
        use legalis_core::{ComparisonOp, Condition};
        Statute::new(
            "s-1",
            "Sample",
            Effect::new(EffectType::Grant, "Grants a benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_discretion("officer reviews")
    }

    fn sample_graph() -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", "b", "requires");
        graph.add_dependency("a", "c", "amends");
        graph
    }

    fn sample_timeline() -> Timeline {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2000-01-01",
            TimelineEvent::Enacted {
                statute_id: "s-1".to_string(),
                title: "Sample Act".to_string(),
            },
        );
        timeline.add_event(
            "2010-01-01",
            TimelineEvent::Repealed {
                statute_id: "s-1".to_string(),
            },
        );
        timeline
    }

    #[test]
    fn audio_describe_tree_produces_segments() {
        let tree = DecisionTree::from_statute(&sample_statute()).expect("tree");
        let description = AudioDescriber::new().describe_tree(&tree);
        assert!(description.segment_count() >= 2);
        let text = description.to_plain_text();
        assert!(text.contains("Decision tree"));
        assert!(text.contains("Root node"));
    }

    #[test]
    fn audio_describe_empty_tree_is_handled() {
        let tree = DecisionTree::new();
        let description = AudioDescriber::new().describe_tree(&tree);
        assert_eq!(description.segment_count(), 1);
        assert!(description.to_plain_text().contains("empty"));
    }

    #[test]
    fn audio_describe_graph_lists_dependencies() {
        let description = AudioDescriber::new().describe_graph(&sample_graph());
        let text = description.to_plain_text();
        assert!(text.contains("3 statutes") || text.contains("3 statute"));
        assert!(text.contains("depends on"));
    }

    #[test]
    fn audio_graph_respects_max_list_items() {
        let mut graph = DependencyGraph::new();
        for i in 0..10 {
            graph.add_statute(&format!("n{}", i));
        }
        let describer = AudioDescriber::new().with_max_list_items(3);
        let description = describer.describe_graph(&graph);
        let text = description.to_plain_text();
        assert!(text.contains("further statute"));
    }

    #[test]
    fn audio_ssml_and_vtt_are_structured() {
        let description = AudioDescriber::new().describe_timeline(&sample_timeline());
        let ssml = description.to_ssml();
        assert!(ssml.starts_with("<speak>"));
        assert!(ssml.trim_end().ends_with("</speak>"));
        assert!(ssml.contains("<break time="));

        let vtt = description.to_webvtt(150);
        assert!(vtt.starts_with("WEBVTT"));
        assert!(vtt.contains("-->"));
        assert!(description.estimated_duration_ms(150) > 0);
    }

    #[test]
    fn braille_maps_letters_digits_and_capitals() {
        assert_eq!(to_braille("abc"), "⠁⠃⠉");
        // Capital sign before an upper-case letter.
        assert_eq!(to_braille("A"), "⠠⠁");
        // Number sign once before a digit run.
        assert_eq!(to_braille("12"), "⠼⠁⠃");
        // Number state resets after a space.
        assert_eq!(to_braille("1 2"), "⠼⠁ ⠼⠃");
        // Unknown characters are dropped.
        assert_eq!(to_braille("a@b"), "⠁⠃");
    }

    #[test]
    fn tactile_export_graph_has_points_and_lines() {
        let tactile = TactileExporter::new().export_graph(&sample_graph());
        let points = tactile
            .primitives
            .iter()
            .filter(|p| p.kind == TactilePrimitiveKind::Point)
            .count();
        let lines = tactile
            .primitives
            .iter()
            .filter(|p| p.kind == TactilePrimitiveKind::Line)
            .count();
        assert_eq!(points, 3);
        assert_eq!(lines, 2);

        let svg = tactile.to_svg();
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("fill=\"#ffffff\""));
        assert!(svg.contains("<circle"));

        let labels = tactile.braille_labels();
        assert_eq!(labels.len(), 3);
    }

    #[test]
    fn tactile_descriptor_notes_hardware() {
        let tactile = TactileExporter::new().export_timeline(&sample_timeline());
        let descriptor = tactile.to_descriptor_text();
        assert!(descriptor.contains("embosser"));
        assert!(descriptor.contains("Timeline"));
        // baseline + one point per event.
        assert_eq!(tactile.primitive_count(), 3);
    }

    #[test]
    fn cognitive_chunk_and_summarize() {
        let items: Vec<String> = (0..10).map(|i| format!("item {}", i)).collect();
        let options = CognitiveLoadOptions::new()
            .with_max_items(4)
            .with_chunk_size(3);
        let chunks = options.chunk_items(&items);
        assert_eq!(chunks.len(), 4); // 3+3+3+1
        let summary = options.summarize(&items);
        assert_eq!(summary.len(), 5); // 4 + summary line
        assert!(summary.last().expect("last").contains("6 more"));
    }

    #[test]
    fn cognitive_one_concept_forces_chunk_one() {
        let items: Vec<String> = (0..3).map(|i| i.to_string()).collect();
        let options = CognitiveLoadOptions::maximum_simplicity();
        assert_eq!(options.effective_chunk_size(), 1);
        assert_eq!(options.chunk_items(&items).len(), 3);
        let css = options.to_css();
        assert!(css.contains("decorative"));
    }

    #[test]
    fn cognitive_summarize_keeps_short_lists_intact() {
        let items: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let options = CognitiveLoadOptions::new();
        assert_eq!(options.summarize(&items), items);
    }

    #[test]
    fn dyslexia_reflow_wraps_at_word_boundaries() {
        let options = DyslexiaTextOptions::new().with_max_line_length(10);
        let reflowed = options.reflow("the quick brown fox jumps");
        for line in reflowed.lines() {
            assert!(line.chars().count() <= 10, "line too long: {:?}", line);
        }
        // No word is split.
        assert!(reflowed.replace('\n', " ") == "the quick brown fox jumps");
    }

    #[test]
    fn dyslexia_reflow_handles_overlong_word() {
        let options = DyslexiaTextOptions::new().with_max_line_length(5);
        let reflowed = options.reflow("antidisestablishmentarianism is long");
        let lines: Vec<&str> = reflowed.lines().collect();
        assert_eq!(lines[0], "antidisestablishmentarianism");
    }

    #[test]
    fn dyslexia_css_and_wrap_contain_typography() {
        let options = DyslexiaTextOptions::open_dyslexic();
        let css = options.to_css_class(".reader");
        assert!(css.contains(".reader {"));
        assert!(css.contains("OpenDyslexic"));
        assert!(css.contains("font-style: normal"));
        let wrapped = options.wrap_html("<p>hello</p>");
        assert!(wrapped.contains("dyslexia-friendly"));
        assert!(wrapped.contains("<p>hello</p>"));
    }
}
