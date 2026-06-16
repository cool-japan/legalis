//! Motor-impairment navigation modes: structured keyboard and navigation
//! descriptors.
//!
//! This module complements the visual and non-visual accessibility helpers with
//! a model for users who navigate with a keyboard only, a single/dual switch, a
//! dwell (eye-gaze or head-pointer) device, or voice control. It produces a
//! portable, serializable [`MotorAccessibilityProfile`] describing the
//! navigation contract — key bindings, minimum interactive-target sizes,
//! scanning timing — plus the CSS and client-side JavaScript that implement that
//! contract in a browser.
//!
//! ## Hardware boundary
//!
//! Physical assistive hardware (single/dual switches, sip-and-puff devices,
//! eye-gaze trackers, head pointers) and their device drivers are out of scope
//! for a pure-Rust library: such devices emit standard keyboard, pointer or
//! click events through the operating system. What this module *does* provide is
//! the software navigation model those events drive — the key map, the dwell and
//! scanning timing, the WCAG target-size geometry — as a descriptor that a host
//! application (or the emitted JavaScript) consumes. No device I/O is performed
//! here.

use serde::{Deserialize, Serialize};

use super::escape_html;
use crate::{VizError, VizResult};

// ===========================================================================
// Navigation modes
// ===========================================================================

/// The primary input modality a motor-impaired user relies on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotorNavigationMode {
    /// Full operation with the keyboard alone (no pointer required).
    KeyboardOnly,
    /// One or two switches drive an automatic highlight scan.
    SwitchScanning,
    /// Hovering/gazing on a target for a dwell period activates it.
    DwellControl,
    /// Spoken commands map to navigation and activation actions.
    VoiceControl,
}

impl MotorNavigationMode {
    /// A short human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            MotorNavigationMode::KeyboardOnly => "Keyboard only",
            MotorNavigationMode::SwitchScanning => "Switch scanning",
            MotorNavigationMode::DwellControl => "Dwell control",
            MotorNavigationMode::VoiceControl => "Voice control",
        }
    }

    /// A stable, lower-case slug suitable for CSS classes or data attributes.
    pub fn slug(&self) -> &'static str {
        match self {
            MotorNavigationMode::KeyboardOnly => "keyboard-only",
            MotorNavigationMode::SwitchScanning => "switch-scanning",
            MotorNavigationMode::DwellControl => "dwell-control",
            MotorNavigationMode::VoiceControl => "voice-control",
        }
    }

    /// A one-sentence description of the modality.
    pub fn description(&self) -> &'static str {
        match self {
            MotorNavigationMode::KeyboardOnly => {
                "Every action is reachable with Tab, arrow keys and Enter; no pointer is needed."
            }
            MotorNavigationMode::SwitchScanning => {
                "A highlight advances automatically through interactive targets; a switch selects \
                 the highlighted target."
            }
            MotorNavigationMode::DwellControl => {
                "Pointing at a target (by eye-gaze or head pointer) and holding for the dwell \
                 period activates it without a click."
            }
            MotorNavigationMode::VoiceControl => {
                "Spoken commands move focus and activate targets; equivalent key bindings remain \
                 available."
            }
        }
    }

    /// Whether this mode typically depends on dedicated assistive hardware.
    ///
    /// Keyboard and voice modes work with commodity input; switch and dwell
    /// modes usually require a switch interface or an eye-gaze/head-pointer
    /// device (the hardware boundary noted at the module level).
    pub fn requires_assistive_hardware(&self) -> bool {
        matches!(
            self,
            MotorNavigationMode::SwitchScanning | MotorNavigationMode::DwellControl
        )
    }
}

// ===========================================================================
// Key bindings
// ===========================================================================

/// A single keyboard (or switch/voice) binding in the navigation contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyBinding {
    /// The key combination or input gesture, e.g. `"Shift + Tab"`.
    pub keys: String,
    /// The action identifier, e.g. `"focus-next"`.
    pub action: String,
    /// A human-readable description of what the binding does.
    pub description: String,
}

impl KeyBinding {
    /// Creates a new key binding.
    pub fn new(keys: &str, action: &str, description: &str) -> Self {
        Self {
            keys: keys.to_string(),
            action: action.to_string(),
            description: description.to_string(),
        }
    }
}

// ===========================================================================
// Scan configuration (switch access)
// ===========================================================================

/// Timing and switch-count configuration for [`MotorNavigationMode::SwitchScanning`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanConfig {
    /// Milliseconds the highlight rests on each target before advancing
    /// (automatic scanning) — ignored when `switch_count >= 2`.
    pub interval_ms: u32,
    /// Number of switches: `1` = auto-scan + select, `2` = manual step + select.
    pub switch_count: u8,
    /// Whether the scan loops back to the first target after the last.
    pub auto_restart: bool,
    /// Colour of the scan highlight outline.
    pub highlight_color: String,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            interval_ms: 1200,
            switch_count: 1,
            auto_restart: true,
            highlight_color: "#ffbf00".to_string(),
        }
    }
}

impl ScanConfig {
    /// Creates a scan configuration with sensible defaults (1-switch auto-scan).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the auto-scan interval in milliseconds (clamped to at least 200 ms).
    pub fn with_interval_ms(mut self, interval_ms: u32) -> Self {
        self.interval_ms = interval_ms.max(200);
        self
    }

    /// Sets the number of switches (clamped to 1 or 2).
    pub fn with_switch_count(mut self, switch_count: u8) -> Self {
        self.switch_count = switch_count.clamp(1, 2);
        self
    }

    /// Sets the highlight colour.
    pub fn with_highlight_color(mut self, color: &str) -> Self {
        self.highlight_color = color.to_string();
        self
    }

    /// Whether scanning is manually stepped (two switches) rather than timed.
    pub fn is_manual(&self) -> bool {
        self.switch_count >= 2
    }
}

// ===========================================================================
// Motor accessibility profile
// ===========================================================================

/// WCAG 2.1 AAA Success Criterion 2.5.5 minimum target size (CSS pixels).
pub const WCAG_AAA_TARGET_SIZE_PX: u32 = 44;

/// A complete, serializable motor-impairment navigation profile.
///
/// Build it from a [`MotorNavigationMode`] (or one of the presets) and emit CSS,
/// JavaScript, an HTML key-map table, a JSON descriptor or a plain-text summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotorAccessibilityProfile {
    /// Primary input modality.
    pub mode: MotorNavigationMode,
    /// Minimum width/height of interactive targets, in CSS pixels.
    pub min_target_size_px: u32,
    /// Minimum spacing between interactive targets, in CSS pixels.
    pub target_spacing_px: u32,
    /// Whether a visible focus indicator is rendered.
    pub focus_visible: bool,
    /// Whether sticky-keys guidance is advertised (modifiers can be pressed
    /// sequentially rather than chorded).
    pub sticky_keys: bool,
    /// Dwell activation period in milliseconds, if dwell activation is enabled.
    pub dwell_activation_ms: Option<u32>,
    /// Scanning configuration, if switch scanning is enabled.
    pub scanning: Option<ScanConfig>,
    /// Whether a "skip to content" link is provided.
    pub skip_links: bool,
    /// Whether the design avoids requiring multi-point or path-based gestures
    /// (WCAG 2.5.1) — `true` means all gestures have a single-pointer
    /// alternative.
    pub single_pointer_only: bool,
    /// The ordered key/input bindings advertised to the user.
    pub bindings: Vec<KeyBinding>,
}

impl MotorAccessibilityProfile {
    /// Creates a profile for the given mode with default geometry and the mode's
    /// default bindings.
    pub fn new(mode: MotorNavigationMode) -> Self {
        let mut profile = Self {
            mode,
            min_target_size_px: WCAG_AAA_TARGET_SIZE_PX,
            target_spacing_px: 8,
            focus_visible: true,
            sticky_keys: true,
            dwell_activation_ms: None,
            scanning: None,
            skip_links: true,
            single_pointer_only: true,
            bindings: Vec::new(),
        };
        match mode {
            MotorNavigationMode::SwitchScanning => profile.scanning = Some(ScanConfig::default()),
            MotorNavigationMode::DwellControl => profile.dwell_activation_ms = Some(1000),
            _ => {}
        }
        profile.bindings = default_bindings(&profile);
        profile
    }

    /// Preset: keyboard-only operation.
    pub fn keyboard_only() -> Self {
        Self::new(MotorNavigationMode::KeyboardOnly)
    }

    /// Preset: single/dual switch scanning.
    pub fn switch_access() -> Self {
        Self::new(MotorNavigationMode::SwitchScanning)
    }

    /// Preset: dwell (eye-gaze / head-pointer) activation.
    pub fn dwell_control() -> Self {
        Self::new(MotorNavigationMode::DwellControl)
    }

    /// Preset: voice control.
    pub fn voice_control() -> Self {
        Self::new(MotorNavigationMode::VoiceControl)
    }

    /// Sets the minimum interactive-target size (clamped to at least 24 px, the
    /// WCAG 2.2 AA floor).
    pub fn with_min_target_size(mut self, px: u32) -> Self {
        self.min_target_size_px = px.max(24);
        self
    }

    /// Sets the spacing between interactive targets.
    pub fn with_target_spacing(mut self, px: u32) -> Self {
        self.target_spacing_px = px;
        self
    }

    /// Enables dwell activation with the given period (clamped to at least
    /// 300 ms) and refreshes the default bindings.
    pub fn with_dwell_activation_ms(mut self, ms: u32) -> Self {
        self.dwell_activation_ms = Some(ms.max(300));
        self.bindings = default_bindings(&self);
        self
    }

    /// Sets the scanning configuration and refreshes the default bindings.
    pub fn with_scanning(mut self, scanning: ScanConfig) -> Self {
        self.scanning = Some(scanning);
        self.bindings = default_bindings(&self);
        self
    }

    /// Appends an extra binding to the contract.
    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.bindings.push(binding);
    }

    /// Whether the interactive-target size satisfies WCAG 2.1 AAA (44 px).
    pub fn meets_wcag_aaa_target_size(&self) -> bool {
        self.min_target_size_px >= WCAG_AAA_TARGET_SIZE_PX
    }

    /// Returns the CSS that realises the profile's geometry and indicators.
    ///
    /// Selectors are scoped under `.motor-nav` so the rules can be applied to a
    /// container without leaking into the rest of a page.
    pub fn to_css(&self) -> String {
        let mut css = String::new();
        css.push_str("/* Motor-impairment navigation — ");
        css.push_str(self.mode.label());
        css.push_str(" */\n");
        css.push_str(".motor-nav button, .motor-nav a, .motor-nav [role=\"button\"], ");
        css.push_str(".motor-nav [data-motor-item] {\n");
        css.push_str(&format!("  min-width: {}px;\n", self.min_target_size_px));
        css.push_str(&format!("  min-height: {}px;\n", self.min_target_size_px));
        css.push_str(&format!("  margin: {}px;\n", self.target_spacing_px));
        css.push_str("  box-sizing: border-box;\n");
        css.push_str("}\n");
        if self.focus_visible {
            css.push_str(".motor-nav :focus, .motor-nav :focus-visible {\n");
            css.push_str("  outline: 3px solid #1a73e8;\n");
            css.push_str("  outline-offset: 2px;\n");
            css.push_str("}\n");
        }
        if self.skip_links {
            css.push_str(".motor-nav .skip-link {\n");
            css.push_str("  position: absolute;\n  left: -9999px;\n");
            css.push_str("}\n");
            css.push_str(".motor-nav .skip-link:focus {\n");
            css.push_str("  position: static;\n  left: auto;\n");
            css.push_str("}\n");
        }
        if let Some(ms) = self.dwell_activation_ms {
            css.push_str("@keyframes motor-dwell {\n");
            css.push_str("  from { box-shadow: inset 0 0 0 0 rgba(26,115,232,0.35); }\n");
            css.push_str("  to { box-shadow: inset 0 0 0 1000px rgba(26,115,232,0.35); }\n");
            css.push_str("}\n");
            css.push_str(".motor-nav[data-dwell] [data-motor-item]:hover {\n");
            css.push_str(&format!(
                "  animation: motor-dwell {}ms linear forwards;\n",
                ms
            ));
            css.push_str("}\n");
        }
        if let Some(scan) = &self.scanning {
            css.push_str(".motor-nav .motor-scan-active {\n");
            css.push_str(&format!("  outline: 4px solid {};\n", scan.highlight_color));
            css.push_str("  outline-offset: 2px;\n");
            css.push_str("}\n");
        }
        css
    }

    /// Returns a self-contained JavaScript controller implementing the mode.
    ///
    /// The script wires DOM keyboard/pointer events to the navigation contract.
    /// Assistive devices (switches, eye-gaze) feed it through the standard event
    /// stream they already emit — see the module-level hardware-boundary note.
    pub fn to_javascript(&self) -> String {
        let mut js = String::new();
        js.push_str("// Motor navigation controller (mode: ");
        js.push_str(self.mode.slug());
        js.push_str(")\n");
        js.push_str("(function(){\n");
        js.push_str("  const root = document.querySelector('.motor-nav');\n");
        js.push_str("  if (!root) { return; }\n");
        js.push_str(
            "  const items = () => Array.from(root.querySelectorAll('[data-motor-item]'));\n",
        );
        js.push_str("  let idx = 0;\n");
        js.push_str("  function focusAt(i) {\n");
        js.push_str("    const list = items();\n");
        js.push_str("    if (list.length === 0) { return; }\n");
        js.push_str("    idx = (i + list.length) % list.length;\n");
        js.push_str("    list[idx].focus();\n");
        js.push_str("  }\n");
        js.push_str("  root.addEventListener('keydown', (e) => {\n");
        js.push_str("    switch (e.key) {\n");
        js.push_str("      case 'ArrowDown': case 'ArrowRight': e.preventDefault(); focusAt(idx + 1); break;\n");
        js.push_str("      case 'ArrowUp': case 'ArrowLeft': e.preventDefault(); focusAt(idx - 1); break;\n");
        js.push_str("      case 'Home': e.preventDefault(); focusAt(0); break;\n");
        js.push_str("      case 'End': e.preventDefault(); focusAt(items().length - 1); break;\n");
        js.push_str("      case 'Enter': case ' ': { const el = items()[idx]; if (el) { el.click(); } break; }\n");
        js.push_str("    }\n");
        js.push_str("  });\n");
        if let Some(ms) = self.dwell_activation_ms {
            js.push_str("  root.setAttribute('data-dwell', '1');\n");
            js.push_str("  let dwellTimer = null;\n");
            js.push_str("  root.addEventListener('pointerover', (e) => {\n");
            js.push_str(
                "    const el = e.target.closest('[data-motor-item]'); if (!el) { return; }\n",
            );
            js.push_str(&format!(
                "    dwellTimer = setTimeout(() => el.click(), {});\n",
                ms
            ));
            js.push_str("  });\n");
            js.push_str(
                "  root.addEventListener('pointerout', () => { if (dwellTimer) { clearTimeout(dwellTimer); dwellTimer = null; } });\n",
            );
        }
        if let Some(scan) = &self.scanning {
            js.push_str("  let scanPos = 0;\n");
            js.push_str("  function paintScan() {\n");
            js.push_str("    const list = items();\n");
            js.push_str(
                "    list.forEach((el, i) => el.classList.toggle('motor-scan-active', i === scanPos));\n",
            );
            js.push_str("  }\n");
            js.push_str("  function selectScan() { const el = items()[scanPos]; if (el) { el.click(); } }\n");
            if scan.is_manual() {
                js.push_str("  // Two-switch: one switch steps, one selects.\n");
                js.push_str("  root.addEventListener('keydown', (e) => {\n");
                js.push_str("    if (e.key === 'Enter') { e.preventDefault(); selectScan(); }\n");
                js.push_str(
                    "    else if (e.key === ' ') { e.preventDefault(); scanPos = (scanPos + 1) % Math.max(items().length, 1); paintScan(); }\n",
                );
                js.push_str("  });\n");
                js.push_str("  paintScan();\n");
            } else {
                js.push_str("  // One-switch: timed auto-scan, switch selects.\n");
                let restart = if scan.auto_restart { "true" } else { "false" };
                js.push_str(&format!(
                    "  setInterval(() => {{ const n = Math.max(items().length, 1); scanPos = (scanPos + 1) % n; if (scanPos === 0 && !{}) {{ scanPos = n - 1; }} paintScan(); }}, {});\n",
                    restart, scan.interval_ms
                ));
                js.push_str(
                    "  root.addEventListener('keydown', (e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); selectScan(); } });\n",
                );
                js.push_str("  paintScan();\n");
            }
        }
        js.push_str("})();\n");
        js
    }

    /// Renders the binding contract as an accessible HTML table.
    pub fn to_keyboard_help_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<table class=\"motor-keymap\">\n");
        html.push_str(&format!(
            "  <caption>{} navigation keys</caption>\n",
            escape_html(self.mode.label())
        ));
        html.push_str("  <thead><tr><th scope=\"col\">Input</th><th scope=\"col\">Action</th><th scope=\"col\">Description</th></tr></thead>\n");
        html.push_str("  <tbody>\n");
        for binding in &self.bindings {
            html.push_str(&format!(
                "    <tr><td><kbd>{}</kbd></td><td>{}</td><td>{}</td></tr>\n",
                escape_html(&binding.keys),
                escape_html(&binding.action),
                escape_html(&binding.description)
            ));
        }
        html.push_str("  </tbody>\n</table>\n");
        html
    }

    /// Serializes the profile to a pretty-printed JSON descriptor.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VizError::ExportError(format!("motor profile to JSON: {}", e)))
    }

    /// Parses a profile from a JSON descriptor.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::InvalidStructure(format!("motor profile from JSON: {}", e)))
    }

    /// Renders a plain-text summary of the navigation contract, including the
    /// hardware-boundary note for modes that need assistive devices.
    pub fn to_descriptor_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Motor navigation profile: {}\n",
            self.mode.label()
        ));
        out.push_str(&format!("  {}\n", self.mode.description()));
        out.push_str(&format!(
            "  Minimum target size: {}px ({} WCAG 2.1 AAA)\n",
            self.min_target_size_px,
            if self.meets_wcag_aaa_target_size() {
                "meets"
            } else {
                "below"
            }
        ));
        out.push_str(&format!("  Target spacing: {}px\n", self.target_spacing_px));
        out.push_str(&format!(
            "  Visible focus: {}; sticky keys: {}; skip links: {}; single-pointer only: {}\n",
            self.focus_visible, self.sticky_keys, self.skip_links, self.single_pointer_only
        ));
        if let Some(ms) = self.dwell_activation_ms {
            out.push_str(&format!("  Dwell activation: {}ms\n", ms));
        }
        if let Some(scan) = &self.scanning {
            out.push_str(&format!(
                "  Scanning: {} switch(es), {}\n",
                scan.switch_count,
                if scan.is_manual() {
                    "manual step".to_string()
                } else {
                    format!("auto every {}ms", scan.interval_ms)
                }
            ));
        }
        out.push_str(&format!("  Bindings ({}):\n", self.bindings.len()));
        for binding in &self.bindings {
            out.push_str(&format!(
                "    {} -> {} ({})\n",
                binding.keys, binding.action, binding.description
            ));
        }
        if self.mode.requires_assistive_hardware() {
            out.push_str(
                "  Note: this mode is driven by assistive hardware (switch interface or \
                 eye-gaze/head-pointer device). Such devices emit standard key/pointer/click \
                 events; this profile is the software model they drive — no device I/O is \
                 performed by this library.\n",
            );
        }
        out
    }
}

/// Builds the default binding set for a profile, tailored to its mode and the
/// presence of dwell/scanning.
fn default_bindings(profile: &MotorAccessibilityProfile) -> Vec<KeyBinding> {
    let mut bindings = vec![
        KeyBinding::new(
            "Tab",
            "focus-next",
            "Move focus to the next interactive element",
        ),
        KeyBinding::new(
            "Shift + Tab",
            "focus-previous",
            "Move focus to the previous interactive element",
        ),
        KeyBinding::new(
            "Arrow Down / Arrow Right",
            "navigate-next",
            "Move to the next sibling node",
        ),
        KeyBinding::new(
            "Arrow Up / Arrow Left",
            "navigate-previous",
            "Move to the previous sibling node",
        ),
        KeyBinding::new("Home", "navigate-first", "Jump to the first element"),
        KeyBinding::new("End", "navigate-last", "Jump to the last element"),
        KeyBinding::new("Enter / Space", "activate", "Activate the focused element"),
        KeyBinding::new("/", "focus-search", "Move focus to the search field"),
        KeyBinding::new("Escape", "dismiss", "Close the open panel or clear focus"),
    ];
    if profile.skip_links {
        bindings.push(KeyBinding::new(
            "Tab (from top)",
            "skip-to-content",
            "Reveal and follow the skip-to-content link",
        ));
    }
    match profile.mode {
        MotorNavigationMode::SwitchScanning => {
            if let Some(scan) = &profile.scanning {
                if scan.is_manual() {
                    bindings.push(KeyBinding::new(
                        "Switch 1 (Space)",
                        "scan-step",
                        "Advance the scan highlight to the next target",
                    ));
                    bindings.push(KeyBinding::new(
                        "Switch 2 (Enter)",
                        "scan-select",
                        "Select the currently highlighted target",
                    ));
                } else {
                    bindings.push(KeyBinding::new(
                        "Switch (Enter / Space)",
                        "scan-select",
                        "Select the currently highlighted target during auto-scan",
                    ));
                }
            }
        }
        MotorNavigationMode::DwellControl => {
            if let Some(ms) = profile.dwell_activation_ms {
                bindings.push(KeyBinding::new(
                    &format!("Dwell ({}ms)", ms),
                    "dwell-activate",
                    "Hold the pointer/gaze on a target for the dwell period to activate it",
                ));
            }
        }
        MotorNavigationMode::VoiceControl => {
            bindings.push(KeyBinding::new(
                "\"next\" / \"previous\"",
                "voice-navigate",
                "Spoken commands move focus between elements",
            ));
            bindings.push(KeyBinding::new(
                "\"click\" / \"select\"",
                "voice-activate",
                "Spoken command activates the focused element",
            ));
        }
        MotorNavigationMode::KeyboardOnly => {}
    }
    bindings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_only_profile_has_core_bindings_and_no_hardware() {
        let profile = MotorAccessibilityProfile::keyboard_only();
        assert_eq!(profile.mode, MotorNavigationMode::KeyboardOnly);
        assert!(!profile.mode.requires_assistive_hardware());
        assert!(profile.scanning.is_none());
        assert!(profile.dwell_activation_ms.is_none());
        // Core bindings present.
        assert!(profile.bindings.iter().any(|b| b.action == "focus-next"));
        assert!(profile.bindings.iter().any(|b| b.action == "activate"));
        assert!(profile.bindings.iter().any(|b| b.action == "focus-search"));
    }

    #[test]
    fn switch_access_default_includes_scan_and_select_binding() {
        let profile = MotorAccessibilityProfile::switch_access();
        assert!(profile.mode.requires_assistive_hardware());
        let scan = profile.scanning.as_ref().expect("scan config");
        assert_eq!(scan.switch_count, 1);
        assert!(!scan.is_manual());
        assert!(profile.bindings.iter().any(|b| b.action == "scan-select"));
        // One-switch auto-scan should not advertise a manual step.
        assert!(!profile.bindings.iter().any(|b| b.action == "scan-step"));
    }

    #[test]
    fn two_switch_scanning_advertises_step_and_select() {
        let profile = MotorAccessibilityProfile::switch_access()
            .with_scanning(ScanConfig::new().with_switch_count(2));
        assert!(profile.bindings.iter().any(|b| b.action == "scan-step"));
        assert!(profile.bindings.iter().any(|b| b.action == "scan-select"));
        let js = profile.to_javascript();
        assert!(js.contains("Two-switch"));
    }

    #[test]
    fn dwell_profile_emits_dwell_css_and_js() {
        let profile = MotorAccessibilityProfile::dwell_control().with_dwell_activation_ms(800);
        assert_eq!(profile.dwell_activation_ms, Some(800));
        assert!(
            profile
                .bindings
                .iter()
                .any(|b| b.action == "dwell-activate")
        );
        let css = profile.to_css();
        assert!(css.contains("@keyframes motor-dwell"));
        assert!(css.contains("800ms"));
        let js = profile.to_javascript();
        assert!(js.contains("setTimeout"));
        assert!(js.contains("data-dwell"));
    }

    #[test]
    fn voice_profile_has_voice_bindings() {
        let profile = MotorAccessibilityProfile::voice_control();
        assert!(
            profile
                .bindings
                .iter()
                .any(|b| b.action == "voice-navigate")
        );
        assert!(
            profile
                .bindings
                .iter()
                .any(|b| b.action == "voice-activate")
        );
        assert!(!profile.mode.requires_assistive_hardware());
    }

    #[test]
    fn target_size_clamps_and_reports_wcag() {
        let profile = MotorAccessibilityProfile::keyboard_only().with_min_target_size(10);
        // Clamped to the WCAG 2.2 AA floor of 24px.
        assert_eq!(profile.min_target_size_px, 24);
        assert!(!profile.meets_wcag_aaa_target_size());
        let big = MotorAccessibilityProfile::keyboard_only().with_min_target_size(48);
        assert!(big.meets_wcag_aaa_target_size());
        let css = big.to_css();
        assert!(css.contains("min-width: 48px"));
    }

    #[test]
    fn scan_interval_and_switch_count_are_clamped() {
        let scan = ScanConfig::new().with_interval_ms(10).with_switch_count(9);
        assert_eq!(scan.interval_ms, 200);
        assert_eq!(scan.switch_count, 2);
        assert!(scan.is_manual());
    }

    #[test]
    fn keyboard_help_html_escapes_and_lists_bindings() {
        let mut profile = MotorAccessibilityProfile::keyboard_only();
        profile.add_binding(KeyBinding::new("Ctrl + <", "custom", "Go <back> & forward"));
        let html = profile.to_keyboard_help_html();
        assert!(html.contains("<table class=\"motor-keymap\">"));
        assert!(html.contains("Go &lt;back&gt; &amp; forward"));
        assert!(html.contains("<kbd>Ctrl + &lt;</kbd>"));
    }

    #[test]
    fn descriptor_text_notes_hardware_for_dwell_mode() {
        let profile = MotorAccessibilityProfile::dwell_control();
        let text = profile.to_descriptor_text();
        assert!(text.contains("Dwell control"));
        assert!(text.contains("no device I/O"));
        // Keyboard-only must not carry the hardware note.
        let kb = MotorAccessibilityProfile::keyboard_only().to_descriptor_text();
        assert!(!kb.contains("no device I/O"));
    }

    #[test]
    fn json_round_trip_preserves_profile() {
        let profile = MotorAccessibilityProfile::switch_access()
            .with_min_target_size(50)
            .with_target_spacing(12)
            .with_scanning(ScanConfig::new().with_interval_ms(900).with_switch_count(2));
        let json = profile.to_json().expect("to_json");
        let restored = MotorAccessibilityProfile::from_json(&json).expect("from_json");
        assert_eq!(profile, restored);
    }

    #[test]
    fn from_json_rejects_garbage() {
        let err = MotorAccessibilityProfile::from_json("{ not json").unwrap_err();
        assert!(matches!(err, VizError::InvalidStructure(_)));
    }
}
