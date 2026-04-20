//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Condition;
#[cfg(feature = "png-export")]
use tiny_skia::Pixmap;

use super::types_3::Timeline;
use super::types_4::DependencyGraph;
use super::types_5::{PopulationChart, VizError};
use super::types_12::DecisionTree;

/// Result type for visualization operations.
pub type VizResult<T> = Result<T, VizError>;
/// Formats a condition for display.
pub(crate) fn format_condition(condition: &Condition) -> String {
    match condition {
        Condition::Age { operator, value } => {
            format!("Age {} {}", format_operator(operator), value)
        }
        Condition::Income { operator, value } => {
            format!("Income {} {}", format_operator(operator), value)
        }
        Condition::HasAttribute { key } => format!("Has '{}'", key),
        Condition::AttributeEquals { key, value } => format!("{} = {}", key, value),
        Condition::DateRange { start, end } => match (start, end) {
            (Some(s), Some(e)) => format!("Date in [{}, {}]", s, e),
            (Some(s), None) => format!("Date ≥ {}", s),
            (None, Some(e)) => format!("Date ≤ {}", e),
            (None, None) => "Any date".to_string(),
        },
        Condition::Geographic {
            region_type,
            region_id,
        } => {
            format!("In {:?}({})", region_type, region_id)
        }
        Condition::EntityRelationship {
            relationship_type,
            target_entity_id,
        } => match target_entity_id {
            Some(id) => format!("{:?} with {}", relationship_type, id),
            None => format!("Has {:?}", relationship_type),
        },
        Condition::ResidencyDuration { operator, months } => {
            format!("Residency {} {} months", format_operator(operator), months)
        }
        Condition::Duration {
            operator,
            value,
            unit,
        } => {
            let unit_str = match unit {
                legalis_core::DurationUnit::Days => "days",
                legalis_core::DurationUnit::Weeks => "weeks",
                legalis_core::DurationUnit::Months => "months",
                legalis_core::DurationUnit::Years => "years",
            };
            format!(
                "Duration {} {} {}",
                format_operator(operator),
                value,
                unit_str
            )
        }
        Condition::Percentage {
            operator,
            value,
            context,
        } => {
            format!("{} {} {}%", context, format_operator(operator), value)
        }
        Condition::SetMembership {
            attribute,
            values,
            negated,
        } => {
            let op = if *negated { "not in" } else { "in" };
            format!("{} {} {{{}}}", attribute, op, values.join(", "))
        }
        Condition::Pattern {
            attribute,
            pattern,
            negated,
        } => {
            let op = if *negated { "!~" } else { "~" };
            format!("{} {} '{}'", attribute, op, pattern)
        }
        Condition::Calculation {
            formula,
            operator,
            value,
        } => {
            format!("{} {} {}", formula, format_operator(operator), value)
        }
        Condition::And(_, _) => "AND condition".to_string(),
        Condition::Or(_, _) => "OR condition".to_string(),
        Condition::Not(_) => "NOT condition".to_string(),
        Condition::Custom { description } => description.clone(),
        Condition::Composite {
            conditions,
            threshold,
        } => {
            format!(
                "Composite ({} conditions, threshold: {})",
                conditions.len(),
                threshold
            )
        }
        Condition::Threshold {
            attributes,
            operator,
            value,
        } => {
            let attrs = attributes
                .iter()
                .map(|(attr, mult)| format!("{}*{}", mult, attr))
                .collect::<Vec<_>>()
                .join(" + ");
            format!("{} {} {}", attrs, format_operator(operator), value)
        }
        Condition::Fuzzy {
            attribute,
            membership_points,
            min_membership,
        } => {
            format!(
                "{} ∈ fuzzy set ({} points, min: {})",
                attribute,
                membership_points.len(),
                min_membership
            )
        }
        Condition::Probabilistic {
            condition: _,
            probability,
            threshold,
        } => {
            format!("Probabilistic (p={}, threshold={})", probability, threshold)
        }
        Condition::Temporal {
            base_value,
            reference_time: _,
            rate,
            operator,
            target_value,
        } => {
            format!(
                "Temporal (base={}, rate={}) {} {}",
                base_value,
                rate,
                format_operator(operator),
                target_value
            )
        }
    }
}
fn format_operator(op: &legalis_core::ComparisonOp) -> &'static str {
    match op {
        legalis_core::ComparisonOp::Equal => "=",
        legalis_core::ComparisonOp::NotEqual => "≠",
        legalis_core::ComparisonOp::GreaterThan => ">",
        legalis_core::ComparisonOp::GreaterOrEqual => "≥",
        legalis_core::ComparisonOp::LessThan => "<",
        legalis_core::ComparisonOp::LessOrEqual => "≤",
    }
}
/// Converts SVG data to PNG format.
#[cfg(feature = "png-export")]
pub(crate) fn svg_to_png(svg_data: &str) -> VizResult<Vec<u8>> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_data, &options)
        .map_err(|e| VizError::RenderError(format!("Failed to parse SVG: {}", e)))?;
    let size = tree.size();
    let width = size.width().ceil() as u32;
    let height = size.height().ceil() as u32;
    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| VizError::RenderError("Failed to create pixmap".to_string()))?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap
        .encode_png()
        .map_err(|e| VizError::RenderError(format!("Failed to encode PNG: {}", e)))
}
/// Plugin trait for custom renderers.
pub trait Renderer {
    /// The output type produced by this renderer.
    type Output;
    /// Renders a decision tree.
    fn render_decision_tree(&self, tree: &DecisionTree) -> VizResult<Self::Output>;
    /// Renders a dependency graph.
    fn render_dependency_graph(&self, graph: &DependencyGraph) -> VizResult<Self::Output>;
    /// Renders a timeline.
    fn render_timeline(&self, timeline: &Timeline) -> VizResult<Self::Output>;
    /// Renders a population chart.
    fn render_population_chart(&self, chart: &PopulationChart) -> VizResult<Self::Output>;
}
/// Helper function for base64 encoding.
pub(crate) fn base64_encode(data: &str) -> String {
    use std::fmt::Write;
    let bytes = data.as_bytes();
    let mut result = String::new();
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;
        write!(&mut result, "{}", CHARS[b1] as char)
            .expect("invariant: writing to String is infallible");
        write!(&mut result, "{}", CHARS[b2] as char)
            .expect("invariant: writing to String is infallible");
        write!(
            &mut result,
            "{}",
            if chunk.len() > 1 {
                CHARS[b3] as char
            } else {
                '='
            }
        )
        .expect("invariant: writing to String is infallible");
        write!(
            &mut result,
            "{}",
            if chunk.len() > 2 {
                CHARS[b4] as char
            } else {
                '='
            }
        )
        .expect("invariant: writing to String is infallible");
    }
    result
}
pub(crate) fn format_change_type(change: &legalis_core::StatuteChange) -> &'static str {
    match change {
        legalis_core::StatuteChange::IdChanged { .. } => "ID Changed",
        legalis_core::StatuteChange::TitleChanged { .. } => "Title Changed",
        legalis_core::StatuteChange::EffectChanged { .. } => "Effect Changed",
        legalis_core::StatuteChange::PreconditionsChanged { .. } => "Preconditions Changed",
        legalis_core::StatuteChange::TemporalValidityChanged => "Temporal Validity Changed",
        legalis_core::StatuteChange::VersionChanged { .. } => "Version Changed",
        legalis_core::StatuteChange::JurisdictionChanged { .. } => "Jurisdiction Changed",
    }
}
