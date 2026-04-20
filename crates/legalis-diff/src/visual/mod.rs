//! Visual diff reports with charts and graphical representations.
//!
//! This module generates visual representations of statute diffs,
//! including SVG charts, impact graphs, and change timelines.

mod charts;
mod highlight;
mod threeway;

pub use charts::{
    generate_change_distribution_chart, generate_impact_matrix, generate_interactive_diff_viewer,
    generate_severity_gauge, generate_visual_report,
};
pub use highlight::{generate_animated_diff_presentation, generate_syntax_highlighted_diff};
pub use threeway::generate_three_way_diff;
