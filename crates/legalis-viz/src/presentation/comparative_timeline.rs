//! Comparative timeline views aligning several legal histories on one axis.
//!
//! Cross-jurisdictional and cross-version analysis often needs more than a
//! single [`Timeline`]: it needs *several* timelines read against a common date
//! axis, so a reader can see what each jurisdiction was doing at the same
//! moment. A [`ComparativeTimelineView`] holds a list of named
//! [`TimelineTrack`]s (each wrapping an existing [`Timeline`]) and renders them
//! together as:
//!
//! - an ASCII grid (rows are dates, columns are tracks),
//! - an aligned HTML table,
//! - an SVG swimlane chart (one lane per track, x-position by date), and
//! - a Mermaid Gantt with one section per track.
//!
//! "Synchronized" dates — those where two or more tracks have events — can be
//! queried and are visually emphasized, supporting the synchronized-navigation
//! story of the cross-jurisdictional comparison feature.
//!
//! This view reuses the crate's existing [`Timeline`] / [`TimelineEvent`] model
//! and the shared event taxonomy; it adds no new event types.
//!
//! [`Timeline`]: crate::Timeline
//! [`TimelineEvent`]: crate::TimelineEvent

use std::collections::BTreeSet;

use super::escape_html;
use crate::data_exchange::timeline_event_parts;
use crate::types_3::Timeline;
use crate::types_5::TimelineEvent;
use crate::types_10::Theme;

/// One labelled track in a comparative view, wrapping a single [`Timeline`].
#[derive(Debug, Clone)]
pub struct TimelineTrack {
    /// Stable identifier for the track (e.g. a jurisdiction code).
    pub id: String,
    /// Human-readable label (e.g. a jurisdiction name).
    pub label: String,
    /// The wrapped timeline.
    pub timeline: Timeline,
}

impl TimelineTrack {
    /// Creates a new track.
    pub fn new(id: &str, label: &str, timeline: Timeline) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            timeline,
        }
    }

    /// Returns the `(date, event)` pairs of this track.
    fn entries(&self) -> &[(String, TimelineEvent)] {
        &self.timeline.events
    }
}

/// Renders a one-line description of a timeline event using the shared taxonomy.
fn describe_event(event: &TimelineEvent) -> String {
    let (kind, statute_id, detail) = timeline_event_parts(event);
    match detail {
        Some(text) => format!("{} {}: {}", kind, statute_id, text),
        None => format!("{} {}", kind, statute_id),
    }
}

/// A multi-track timeline view sharing a single date axis.
#[derive(Debug, Clone)]
pub struct ComparativeTimelineView {
    /// Title shown in rendered output.
    pub title: String,
    /// The tracks being compared.
    pub tracks: Vec<TimelineTrack>,
    /// Theme for visual output.
    pub theme: Theme,
}

impl ComparativeTimelineView {
    /// Creates a new, empty comparative view.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            tracks: Vec::new(),
            theme: Theme::light(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Adds a track (builder style).
    pub fn with_track(mut self, track: TimelineTrack) -> Self {
        self.tracks.push(track);
        self
    }

    /// Adds a track.
    pub fn add_track(&mut self, track: TimelineTrack) {
        self.tracks.push(track);
    }

    /// Returns the number of tracks.
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Returns the sorted, de-duplicated set of all dates across all tracks.
    pub fn axis_dates(&self) -> Vec<String> {
        let mut dates: BTreeSet<&str> = BTreeSet::new();
        for track in &self.tracks {
            for (date, _) in track.entries() {
                dates.insert(date.as_str());
            }
        }
        dates.into_iter().map(str::to_string).collect()
    }

    /// Returns the events occurring on `date` in the given track, in order.
    pub fn events_on<'a>(&'a self, track_id: &str, date: &str) -> Vec<&'a TimelineEvent> {
        self.tracks
            .iter()
            .find(|t| t.id == track_id)
            .map(|t| {
                t.entries()
                    .iter()
                    .filter(|(d, _)| d == date)
                    .map(|(_, e)| e)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns the number of tracks that have at least one event on `date`.
    pub fn tracks_active_on(&self, date: &str) -> usize {
        self.tracks
            .iter()
            .filter(|t| t.entries().iter().any(|(d, _)| d == date))
            .count()
    }

    /// Returns the dates on which two or more tracks have events.
    pub fn synchronized_dates(&self) -> Vec<String> {
        self.axis_dates()
            .into_iter()
            .filter(|date| self.tracks_active_on(date) >= 2)
            .collect()
    }

    /// Renders an ASCII grid: each date followed by its per-track events.
    pub fn to_ascii(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Comparative Timeline: {}\n", self.title));
        out.push_str(&"=".repeat(24 + self.title.len()));
        out.push_str("\n\n");

        let dates = self.axis_dates();
        if dates.is_empty() || self.tracks.is_empty() {
            out.push_str("(no events)\n");
            return out;
        }

        for date in &dates {
            let synced = self.tracks_active_on(date) >= 2;
            let marker = if synced { " <== synchronized" } else { "" };
            out.push_str(&format!("{}{}\n", date, marker));
            for track in &self.tracks {
                let events = self.events_on(&track.id, date);
                if events.is_empty() {
                    continue;
                }
                for event in events {
                    out.push_str(&format!("  [{}] {}\n", track.label, describe_event(event)));
                }
            }
            out.push('\n');
        }
        out
    }

    /// Renders an aligned HTML table comparing the tracks date by date.
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!(
            "    <title>{}</title>\n",
            escape_html(&self.title)
        ));
        html.push_str("    <style>\n");
        html.push_str(&format!(
            "        body {{ background-color: {}; color: {}; font-family: 'Segoe UI', Arial, sans-serif; margin: 0; padding: 20px; }}\n",
            self.theme.background_color, self.theme.text_color
        ));
        html.push_str("        table { border-collapse: collapse; width: 100%; }\n");
        html.push_str(&format!(
            "        th, td {{ border: 1px solid {}; padding: 8px; vertical-align: top; }}\n",
            self.theme.link_color
        ));
        html.push_str(&format!(
            "        th {{ background-color: {}; }}\n",
            self.theme.condition_color
        ));
        html.push_str(&format!(
            "        tr.synchronized td {{ outline: 2px solid {}; }}\n",
            self.theme.discretion_color
        ));
        html.push_str("        td.empty { color: #999; }\n");
        html.push_str("        ul { margin: 0; padding-left: 18px; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str(&format!("    <h1>{}</h1>\n", escape_html(&self.title)));

        html.push_str("    <table>\n        <thead>\n            <tr><th>Date</th>");
        for track in &self.tracks {
            html.push_str(&format!("<th>{}</th>", escape_html(&track.label)));
        }
        html.push_str("</tr>\n        </thead>\n        <tbody>\n");

        for date in self.axis_dates() {
            let synced = self.tracks_active_on(&date) >= 2;
            let row_class = if synced {
                " class=\"synchronized\""
            } else {
                ""
            };
            html.push_str(&format!(
                "            <tr{}><td>{}</td>",
                row_class,
                escape_html(&date)
            ));
            for track in &self.tracks {
                let events = self.events_on(&track.id, &date);
                if events.is_empty() {
                    html.push_str("<td class=\"empty\">&middot;</td>");
                } else {
                    html.push_str("<td><ul>");
                    for event in events {
                        html.push_str(&format!("<li>{}</li>", escape_html(&describe_event(event))));
                    }
                    html.push_str("</ul></td>");
                }
            }
            html.push_str("</tr>\n");
        }

        html.push_str("        </tbody>\n    </table>\n</body>\n</html>");
        html
    }

    /// Renders an SVG swimlane chart, one horizontal lane per track.
    pub fn to_svg(&self) -> String {
        let dates = self.axis_dates();
        let lane_height = 70u32;
        let top_margin = 70u32;
        let left_margin = 160u32;
        let right_margin = 40u32;
        let col_width = 140u32;
        let width = left_margin + right_margin + col_width * (dates.len().max(1) as u32);
        let height = top_margin + lane_height * (self.tracks.len().max(1) as u32) + 20;

        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
            width, height, width, height
        ));
        svg.push_str(&format!(
            "  <rect width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
            width, height, self.theme.background_color
        ));
        svg.push_str(&format!(
            "  <text x=\"20\" y=\"32\" font-family=\"sans-serif\" font-size=\"20\" font-weight=\"bold\" fill=\"{}\">{}</text>\n",
            self.theme.text_color,
            escape_html(&self.title)
        ));

        // Date axis labels and synchronized-column highlights.
        for (col, date) in dates.iter().enumerate() {
            let x = left_margin + col_width * (col as u32) + col_width / 2;
            if self.tracks_active_on(date) >= 2 {
                let rx = left_margin + col_width * (col as u32);
                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" opacity=\"0.18\"/>\n",
                    rx,
                    top_margin - 20,
                    col_width,
                    lane_height * (self.tracks.len().max(1) as u32),
                    self.theme.discretion_color
                ));
            }
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" text-anchor=\"middle\" fill=\"{}\">{}</text>\n",
                x,
                top_margin - 28,
                self.theme.text_color,
                escape_html(date)
            ));
        }

        // Lanes.
        for (row, track) in self.tracks.iter().enumerate() {
            let lane_y = top_margin + lane_height * (row as u32);
            let center_y = lane_y + lane_height / 2;
            svg.push_str(&format!(
                "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\"/>\n",
                left_margin,
                center_y,
                width - right_margin,
                center_y,
                self.theme.link_color
            ));
            svg.push_str(&format!(
                "  <text x=\"16\" y=\"{}\" font-family=\"sans-serif\" font-size=\"13\" fill=\"{}\">{}</text>\n",
                center_y + 4,
                self.theme.text_color,
                escape_html(&track.label)
            ));
            for (col, date) in dates.iter().enumerate() {
                let events = self.events_on(&track.id, date);
                if events.is_empty() {
                    continue;
                }
                let cx = left_margin + col_width * (col as u32) + col_width / 2;
                svg.push_str(&format!(
                    "  <circle cx=\"{}\" cy=\"{}\" r=\"9\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\"><title>{}</title></circle>\n",
                    cx,
                    center_y,
                    self.theme.outcome_color,
                    self.theme.text_color,
                    escape_html(
                        &events
                            .iter()
                            .map(|e| describe_event(e))
                            .collect::<Vec<_>>()
                            .join("; ")
                    )
                ));
                if events.len() > 1 {
                    svg.push_str(&format!(
                        "  <text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"10\" text-anchor=\"middle\" fill=\"{}\">{}</text>\n",
                        cx,
                        center_y + 4,
                        self.theme.text_color,
                        events.len()
                    ));
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }

    /// Renders a Mermaid Gantt chart with one section per track.
    pub fn to_mermaid(&self) -> String {
        let mut out = String::from("gantt\n");
        out.push_str(&format!("    title {}\n", self.title));
        out.push_str("    dateFormat YYYY-MM-DD\n");
        for track in &self.tracks {
            out.push_str(&format!("    section {}\n", track.label));
            for (date, event) in track.entries() {
                let (kind, statute_id, _) = timeline_event_parts(event);
                out.push_str(&format!("    {} {} : {}, 1d\n", kind, statute_id, date));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types_5::TimelineEvent;

    fn track(id: &str, label: &str, events: &[(&str, TimelineEvent)]) -> TimelineTrack {
        let mut timeline = Timeline::new();
        for (date, event) in events {
            timeline.add_event(date, event.clone());
        }
        TimelineTrack::new(id, label, timeline)
    }

    fn sample_view() -> ComparativeTimelineView {
        let us = track(
            "US",
            "United States",
            &[
                (
                    "2000-01-01",
                    TimelineEvent::Enacted {
                        statute_id: "us-1".to_string(),
                        title: "Act A".to_string(),
                    },
                ),
                (
                    "2010-06-15",
                    TimelineEvent::Amended {
                        statute_id: "us-1".to_string(),
                        description: "minor fix".to_string(),
                    },
                ),
            ],
        );
        let jp = track(
            "JP",
            "Japan",
            &[
                (
                    "2000-01-01",
                    TimelineEvent::Enacted {
                        statute_id: "jp-1".to_string(),
                        title: "Law B".to_string(),
                    },
                ),
                (
                    "2020-03-01",
                    TimelineEvent::Repealed {
                        statute_id: "jp-1".to_string(),
                    },
                ),
            ],
        );
        ComparativeTimelineView::new("Privacy Law Evolution")
            .with_track(us)
            .with_track(jp)
    }

    #[test]
    fn axis_dates_are_sorted_and_deduplicated() {
        let view = sample_view();
        let dates = view.axis_dates();
        assert_eq!(dates, vec!["2000-01-01", "2010-06-15", "2020-03-01"]);
    }

    #[test]
    fn events_on_returns_matching_track_events() {
        let view = sample_view();
        let events = view.events_on("US", "2000-01-01");
        assert_eq!(events.len(), 1);
        let none = view.events_on("US", "2020-03-01");
        assert!(none.is_empty());
        let missing_track = view.events_on("DE", "2000-01-01");
        assert!(missing_track.is_empty());
    }

    #[test]
    fn synchronized_dates_detects_shared_moments() {
        let view = sample_view();
        let synced = view.synchronized_dates();
        assert_eq!(synced, vec!["2000-01-01"]);
        assert_eq!(view.tracks_active_on("2000-01-01"), 2);
        assert_eq!(view.tracks_active_on("2010-06-15"), 1);
    }

    #[test]
    fn ascii_contains_tracks_and_sync_marker() {
        let view = sample_view();
        let ascii = view.to_ascii();
        assert!(ascii.contains("Comparative Timeline: Privacy Law Evolution"));
        assert!(ascii.contains("[United States]"));
        assert!(ascii.contains("[Japan]"));
        assert!(ascii.contains("synchronized"));
        assert!(ascii.contains("Enacted us-1: Act A"));
    }

    #[test]
    fn html_is_well_formed_and_escapes_content() {
        let mut view = sample_view();
        view.add_track(track(
            "X",
            "A & B <Co>",
            &[(
                "2000-01-01",
                TimelineEvent::Repealed {
                    statute_id: "x".to_string(),
                },
            )],
        ));
        let html = view.to_html();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<table>"));
        assert!(html.contains("A &amp; B &lt;Co&gt;"));
        assert!(html.contains("class=\"synchronized\""));
        assert_eq!(html.matches("<tr").count(), html.matches("</tr>").count());
    }

    #[test]
    fn svg_has_lane_per_track_and_dimensions() {
        let view = sample_view();
        let svg = view.to_svg();
        assert!(svg.starts_with("<svg"));
        assert!(svg.trim_end().ends_with("</svg>"));
        // One baseline <line> per track.
        assert_eq!(svg.matches("<line").count(), view.track_count());
        // Event markers present.
        assert!(svg.contains("<circle"));
    }

    #[test]
    fn mermaid_has_section_per_track() {
        let view = sample_view();
        let mermaid = view.to_mermaid();
        assert!(mermaid.starts_with("gantt"));
        assert_eq!(mermaid.matches("    section ").count(), view.track_count());
        assert!(mermaid.contains("Enacted us-1"));
    }

    #[test]
    fn empty_view_renders_placeholder() {
        let view = ComparativeTimelineView::new("Empty");
        assert!(view.to_ascii().contains("(no events)"));
        assert!(view.axis_dates().is_empty());
        assert!(view.to_svg().starts_with("<svg"));
    }
}
