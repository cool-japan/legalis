//! Presentation: reusable templates, comparative views and accessible
//! alternative renderings for legal-structure visualizations.
//!
//! Where the rest of the crate is concerned with *producing* a visualization,
//! this module is concerned with *how that visualization is packaged and
//! consumed*. It builds directly on the existing model types ([`DecisionTree`],
//! [`DependencyGraph`], [`Timeline`], [`Theme`]) without duplicating any
//! rendering logic, and adds three cohesive capabilities:
//!
//! - [`templates`] — a versioned, serializable [`VisualizationTemplate`] preset
//!   system with a layered [`TemplateCustomization`] mechanism and a
//!   [`TemplateLibrary`] of ready-to-use examples. Templates round-trip through
//!   JSON for import/export and carry a semantic [`TemplateVersion`] plus a
//!   changelog so presets can evolve safely.
//! - [`comparative_timeline`] — a [`ComparativeTimelineView`] that aligns
//!   several named [`Timeline`] tracks on a single shared date axis, so legal
//!   histories from different jurisdictions (or different statute versions) can
//!   be read side by side as ASCII, HTML, SVG swimlanes or a Mermaid Gantt.
//! - [`accessibility_ext`] — non-visual and low-load alternatives:
//!   [`AudioDescriber`] turns a visualization into a structured
//!   [`AudioDescription`] (plain text / SSML / WebVTT) for text-to-speech;
//!   [`TactileExporter`] emits a [`TactileGraphic`] with raised-line SVG and
//!   Unicode-braille labels (a portable descriptor; physical embossing needs
//!   dedicated hardware); [`CognitiveLoadOptions`] chunks and summarizes dense
//!   content; and [`DyslexiaTextOptions`] reflows and styles text for
//!   readability.
//! - [`motor`] — motor-impairment navigation modes as a serializable
//!   [`MotorAccessibilityProfile`] (keyboard-only, switch scanning, dwell and
//!   voice control) that emits the key map, WCAG target-size CSS and a
//!   navigation-controller script; physical assistive devices are a noted
//!   hardware boundary.
//!
//! [`DecisionTree`]: crate::DecisionTree
//! [`DependencyGraph`]: crate::DependencyGraph
//! [`Timeline`]: crate::Timeline
//! [`Theme`]: crate::Theme

mod accessibility_ext;
mod comparative_timeline;
mod motor;
mod templates;

pub use accessibility_ext::{
    AudioDescriber, AudioDescription, AudioSegment, CognitiveLoadOptions, DyslexiaTextOptions,
    TactileExporter, TactileGraphic, TactilePrimitive, TactilePrimitiveKind, TactileTexture,
    to_braille,
};
pub use comparative_timeline::{ComparativeTimelineView, TimelineTrack};
pub use motor::{
    KeyBinding, MotorAccessibilityProfile, MotorNavigationMode, ScanConfig, WCAG_AAA_TARGET_SIZE_PX,
};
pub use templates::{
    Orientation, TemplateCategory, TemplateChange, TemplateCustomization, TemplateKind,
    TemplateLayout, TemplateLibrary, TemplateStyle, TemplateVersion, VisualizationTemplate,
};

/// Escapes text for safe inclusion in generated HTML/SVG markup.
///
/// Reuses the crate's existing XML escaper (the `&`, `<`, `>`, `"` and `'`
/// replacements it performs are equally valid in HTML5 and SVG) rather than
/// duplicating the logic.
pub(crate) fn escape_html(value: &str) -> String {
    crate::data_exchange::escape_xml(value)
}

/// Formats a millisecond offset as a WebVTT / SRT-style `HH:MM:SS.mmm` cue
/// timestamp.
pub(crate) fn format_vtt_timestamp(total_ms: u64) -> String {
    let hours = total_ms / 3_600_000;
    let minutes = (total_ms % 3_600_000) / 60_000;
    let seconds = (total_ms % 60_000) / 1000;
    let millis = total_ms % 1000;
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_escapes_markup_metacharacters() {
        let escaped = escape_html("a<b> & \"c\" 'd'");
        assert_eq!(escaped, "a&lt;b&gt; &amp; &quot;c&quot; &apos;d&apos;");
    }

    #[test]
    fn format_vtt_timestamp_pads_all_fields() {
        assert_eq!(format_vtt_timestamp(0), "00:00:00.000");
        assert_eq!(format_vtt_timestamp(3_500), "00:00:03.500");
        assert_eq!(format_vtt_timestamp(61_001), "00:01:01.001");
        assert_eq!(format_vtt_timestamp(3_661_250), "01:01:01.250");
    }
}
