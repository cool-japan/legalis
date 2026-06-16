//! Progressive Web App (PWA) generation for offline diff viewing.
//!
//! This module emits the three artefacts a browser needs to install and run a
//! statute-diff viewer offline, as **real, standards-compliant** files:
//!
//! - a [W3C Web App Manifest](https://www.w3.org/TR/appmanifest/)
//!   ([`PwaManifest::to_json`]),
//! - a [Service Worker](https://www.w3.org/TR/service-workers/)
//!   ([`ServiceWorkerConfig::to_javascript`]) implementing a chosen
//!   [`CacheStrategy`], and
//! - a self-contained `index.html` shell that renders a [`StatuteDiff`] and
//!   registers the service worker.
//!
//! [`PwaBundle`] ties the three together and can [write them to a
//! directory](PwaBundle::write_to_dir). Generation is fully implemented in pure
//! Rust; only *serving* the assets (an HTTP host) and the *browser runtime* are
//! external — this workspace produces the deployable bundle but does not host it.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_diff::{diff, mobile::pwa::PwaBundle};
//!
//! let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "x"));
//! let mut new = old.clone();
//! new.title = "New".to_string();
//! let d = diff(&old, &new).unwrap();
//!
//! let bundle = PwaBundle::from_diff(&d);
//! let files = bundle.files().unwrap();
//! assert_eq!(files.len(), 3);
//! assert!(bundle.index_html.contains("serviceWorker"));
//! ```

use crate::{Change, DiffError, DiffResult, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The `display` member of a Web App Manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DisplayMode {
    /// Full screen, no browser UI.
    Fullscreen,
    /// Standalone app window.
    Standalone,
    /// Minimal browser UI.
    MinimalUi,
    /// Normal browser tab.
    Browser,
}

impl DisplayMode {
    /// The manifest string value for the mode.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fullscreen => "fullscreen",
            Self::Standalone => "standalone",
            Self::MinimalUi => "minimal-ui",
            Self::Browser => "browser",
        }
    }
}

/// An icon entry in a Web App Manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PwaIcon {
    /// Icon URL.
    pub src: String,
    /// Space-separated size list (e.g. `192x192`).
    pub sizes: String,
    /// MIME type (serialized as the manifest `type` member).
    #[serde(rename = "type")]
    pub mime_type: String,
    /// Optional `purpose` (e.g. `any maskable`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}

impl PwaIcon {
    /// Creates an icon entry.
    pub fn new(
        src: impl Into<String>,
        sizes: impl Into<String>,
        mime_type: impl Into<String>,
    ) -> Self {
        Self {
            src: src.into(),
            sizes: sizes.into(),
            mime_type: mime_type.into(),
            purpose: None,
        }
    }

    /// Sets the icon `purpose`.
    #[must_use]
    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }
}

/// A W3C Web App Manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PwaManifest {
    /// Full application name.
    pub name: String,
    /// Short name for the home screen.
    pub short_name: String,
    /// Launch URL.
    pub start_url: String,
    /// Navigation scope.
    pub scope: String,
    /// Display mode.
    pub display: DisplayMode,
    /// Theme colour (CSS colour string).
    pub theme_color: String,
    /// Background colour (CSS colour string).
    pub background_color: String,
    /// Description.
    pub description: String,
    /// BCP-47 language tag.
    pub lang: String,
    /// Optional text direction (`ltr` / `rtl` / `auto`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    /// Optional preferred orientation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    /// Icon set.
    pub icons: Vec<PwaIcon>,
    /// Application categories.
    pub categories: Vec<String>,
}

impl PwaManifest {
    /// Creates a manifest with sensible defaults for a diff viewer.
    pub fn new(name: impl Into<String>, short_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            short_name: short_name.into(),
            start_url: "/".to_string(),
            scope: "/".to_string(),
            display: DisplayMode::Standalone,
            theme_color: "#1565c0".to_string(),
            background_color: "#ffffff".to_string(),
            description: "Statute diff viewer".to_string(),
            lang: "en".to_string(),
            dir: None,
            orientation: None,
            icons: Vec::new(),
            categories: vec!["productivity".to_string(), "utilities".to_string()],
        }
    }

    /// Sets the description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the display mode.
    #[must_use]
    pub fn with_display(mut self, display: DisplayMode) -> Self {
        self.display = display;
        self
    }

    /// Sets the theme colour.
    #[must_use]
    pub fn with_theme_color(mut self, color: impl Into<String>) -> Self {
        self.theme_color = color.into();
        self
    }

    /// Adds an icon.
    #[must_use]
    pub fn with_icon(mut self, icon: PwaIcon) -> Self {
        self.icons.push(icon);
        self
    }

    /// Serializes the manifest to pretty-printed JSON.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the manifest cannot be
    /// encoded.
    pub fn to_json(&self) -> DiffResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| DiffError::SerializationError(format!("failed to encode manifest: {}", e)))
    }
}

/// A service-worker caching strategy for the `fetch` handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheStrategy {
    /// Serve from cache, fall back to network.
    CacheFirst,
    /// Try the network, fall back to cache.
    NetworkFirst,
    /// Serve cache immediately while revalidating in the background.
    StaleWhileRevalidate,
}

impl CacheStrategy {
    fn fetch_body(self) -> &'static str {
        match self {
            Self::CacheFirst => {
                "  event.respondWith(\n\
                 \x20   caches.match(event.request).then((cached) => cached || fetch(event.request))\n\
                 \x20 );"
            }
            Self::NetworkFirst => {
                "  event.respondWith(\n\
                 \x20   fetch(event.request).catch(() => caches.match(event.request))\n\
                 \x20 );"
            }
            Self::StaleWhileRevalidate => {
                "  event.respondWith(\n\
                 \x20   caches.open(CACHE).then((cache) =>\n\
                 \x20     cache.match(event.request).then((cached) => {\n\
                 \x20       const network = fetch(event.request).then((response) => {\n\
                 \x20         cache.put(event.request, response.clone());\n\
                 \x20         return response;\n\
                 \x20       });\n\
                 \x20       return cached || network;\n\
                 \x20     })\n\
                 \x20   )\n\
                 \x20 );"
            }
        }
    }
}

/// Configuration for the generated service worker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceWorkerConfig {
    /// Cache name prefix.
    pub cache_name: String,
    /// Cache version (appended to the name; bump to invalidate).
    pub version: String,
    /// URLs to pre-cache during the `install` phase.
    pub precache: Vec<String>,
    /// The `fetch` caching strategy.
    pub strategy: CacheStrategy,
}

impl ServiceWorkerConfig {
    /// Creates a config with an empty precache list and the stale-while-revalidate
    /// strategy.
    pub fn new(cache_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            cache_name: cache_name.into(),
            version: version.into(),
            precache: Vec::new(),
            strategy: CacheStrategy::StaleWhileRevalidate,
        }
    }

    /// Replaces the precache URL list.
    #[must_use]
    pub fn with_precache<I, S>(mut self, urls: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.precache = urls.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the caching strategy.
    #[must_use]
    pub fn with_strategy(mut self, strategy: CacheStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// The full cache key (`name-version`).
    pub fn cache_key(&self) -> String {
        format!("{}-{}", self.cache_name, self.version)
    }

    /// Generates the `sw.js` service-worker source.
    pub fn to_javascript(&self) -> String {
        let cache_key = js_escape(&self.cache_key());
        let precache = js_string_array(&self.precache);
        let fetch_body = self.strategy.fetch_body();
        format!(
            "// Auto-generated by legalis-diff PWA generator.\n\
             const CACHE = \"{cache_key}\";\n\
             const PRECACHE = {precache};\n\
             \n\
             self.addEventListener('install', (event) => {{\n\
             \x20 event.waitUntil(\n\
             \x20   caches.open(CACHE).then((cache) => cache.addAll(PRECACHE)).then(() => self.skipWaiting())\n\
             \x20 );\n\
             }});\n\
             \n\
             self.addEventListener('activate', (event) => {{\n\
             \x20 event.waitUntil(\n\
             \x20   caches.keys().then((keys) => Promise.all(\n\
             \x20     keys.filter((key) => key !== CACHE).map((key) => caches.delete(key))\n\
             \x20   )).then(() => self.clients.claim())\n\
             \x20 );\n\
             }});\n\
             \n\
             self.addEventListener('fetch', (event) => {{\n\
             {fetch_body}\n\
             }});\n"
        )
    }
}

/// A generated PWA file: its relative path, contents and MIME type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PwaFile {
    /// Path relative to the bundle root.
    pub path: String,
    /// File contents.
    pub contents: String,
    /// MIME type.
    pub mime_type: String,
}

/// A complete, deployable PWA bundle for viewing a diff offline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PwaBundle {
    /// The Web App Manifest.
    pub manifest: PwaManifest,
    /// The service-worker configuration.
    pub service_worker: ServiceWorkerConfig,
    /// The rendered `index.html` shell.
    pub index_html: String,
}

impl PwaBundle {
    /// Creates a bundle from explicit parts.
    pub fn new(
        manifest: PwaManifest,
        service_worker: ServiceWorkerConfig,
        index_html: String,
    ) -> Self {
        Self {
            manifest,
            service_worker,
            index_html,
        }
    }

    /// Builds a default bundle that renders `diff` offline.
    pub fn from_diff(diff: &StatuteDiff) -> Self {
        let title = format!("Diff: {}", diff.statute_id);
        let manifest = PwaManifest::new(&title, "Diff")
            .with_description("Offline statute diff viewer")
            .with_icon(PwaIcon::new("/icons/icon-192.png", "192x192", "image/png"))
            .with_icon(
                PwaIcon::new("/icons/icon-512.png", "512x512", "image/png")
                    .with_purpose("any maskable"),
            );
        let service_worker = ServiceWorkerConfig::new("legalis-diff", "1").with_precache([
            "/",
            "/index.html",
            "/manifest.webmanifest",
            "/sw.js",
        ]);
        let index_html = render_index_html(diff, &manifest);
        Self {
            manifest,
            service_worker,
            index_html,
        }
    }

    /// Returns the three bundle files (`manifest.webmanifest`, `sw.js`,
    /// `index.html`).
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the manifest cannot be
    /// encoded.
    pub fn files(&self) -> DiffResult<Vec<PwaFile>> {
        Ok(vec![
            PwaFile {
                path: "manifest.webmanifest".to_string(),
                contents: self.manifest.to_json()?,
                mime_type: "application/manifest+json".to_string(),
            },
            PwaFile {
                path: "sw.js".to_string(),
                contents: self.service_worker.to_javascript(),
                mime_type: "text/javascript".to_string(),
            },
            PwaFile {
                path: "index.html".to_string(),
                contents: self.index_html.clone(),
                mime_type: "text/html".to_string(),
            },
        ])
    }

    /// Writes every bundle file under `dir` (created if necessary). Returns the
    /// list of written paths.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the directory cannot be
    /// created or a file cannot be written.
    pub fn write_to_dir(&self, dir: impl AsRef<Path>) -> DiffResult<Vec<String>> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir).map_err(|e| {
            DiffError::SerializationError(format!("failed to create PWA directory: {}", e))
        })?;
        let mut written = Vec::new();
        for file in self.files()? {
            let path = dir.join(&file.path);
            std::fs::write(&path, file.contents.as_bytes()).map_err(|e| {
                DiffError::SerializationError(format!("failed to write {}: {}", file.path, e))
            })?;
            written.push(path.to_string_lossy().into_owned());
        }
        Ok(written)
    }
}

/// Renders the offline diff-viewer HTML shell.
fn render_index_html(diff: &StatuteDiff, manifest: &PwaManifest) -> String {
    let mut rows = String::new();
    for change in &diff.changes {
        rows.push_str(&render_change_row(change));
    }
    if rows.is_empty() {
        rows.push_str("<li class=\"none\">No changes detected.</li>");
    }

    let mut notes = String::new();
    for note in &diff.impact.notes {
        notes.push_str(&format!("<li>{}</li>", escape_html(note)));
    }

    format!(
        "<!DOCTYPE html>\n\
         <html lang=\"{lang}\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <meta name=\"theme-color\" content=\"{theme}\">\n\
         <link rel=\"manifest\" href=\"manifest.webmanifest\">\n\
         <title>{title}</title>\n\
         </head>\n\
         <body>\n\
         <h1>{title}</h1>\n\
         <p>Severity: <strong>{severity:?}</strong> &middot; Changes: {count}</p>\n\
         <ul class=\"changes\">\n{rows}\n</ul>\n\
         <h2>Impact notes</h2>\n\
         <ul class=\"notes\">\n{notes}\n</ul>\n\
         <script>\n\
         if ('serviceWorker' in navigator) {{\n\
         \x20 navigator.serviceWorker.register('sw.js');\n\
         }}\n\
         </script>\n\
         </body>\n\
         </html>\n",
        lang = escape_html(&manifest.lang),
        theme = escape_html(&manifest.theme_color),
        title = escape_html(&format!("Diff: {}", diff.statute_id)),
        severity = diff.impact.severity,
        count = diff.changes.len(),
        rows = rows,
        notes = notes,
    )
}

fn render_change_row(change: &Change) -> String {
    let old = change.old_value.as_deref().unwrap_or("");
    let new = change.new_value.as_deref().unwrap_or("");
    format!(
        "<li class=\"change\"><span class=\"type\">{ct:?}</span> \
         <span class=\"target\">{target}</span>: {desc}\
         <div class=\"old\">{old}</div><div class=\"new\">{new}</div></li>",
        ct = change.change_type,
        target = escape_html(&change.target.to_string()),
        desc = escape_html(&change.description),
        old = escape_html(old),
        new = escape_html(new),
    )
}

/// HTML-escapes a string for safe embedding in element content / attributes.
fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

/// Escapes a string for embedding inside a double-quoted JavaScript string.
fn js_escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out
}

/// Builds a JavaScript array literal of escaped, double-quoted strings.
fn js_string_array(items: &[String]) -> String {
    let mut out = String::from("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(&js_escape(item));
        out.push('"');
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn sample_diff() -> StatuteDiff {
        let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "x"));
        let mut new = old.clone();
        new.title = "New".to_string();
        diff(&old, &new).expect("diff")
    }

    #[test]
    fn test_manifest_json_has_required_members() {
        let manifest = PwaManifest::new("Statute Diff", "Diff")
            .with_display(DisplayMode::Standalone)
            .with_icon(PwaIcon::new("/i.png", "192x192", "image/png"));
        let json = manifest.to_json().expect("json");
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"short_name\""));
        assert!(json.contains("\"start_url\""));
        assert!(json.contains("\"standalone\""));
        assert!(json.contains("\"theme_color\""));
        assert!(json.contains("\"type\": \"image/png\""));
    }

    #[test]
    fn test_display_mode_kebab_case() {
        assert_eq!(DisplayMode::MinimalUi.as_str(), "minimal-ui");
        let json = serde_json::to_string(&DisplayMode::MinimalUi).expect("json");
        assert_eq!(json, "\"minimal-ui\"");
    }

    #[test]
    fn test_service_worker_javascript() {
        let sw = ServiceWorkerConfig::new("legalis", "3")
            .with_precache(["/", "/index.html"])
            .with_strategy(CacheStrategy::NetworkFirst);
        let js = sw.to_javascript();
        assert!(js.contains("const CACHE = \"legalis-3\""));
        assert!(js.contains("addEventListener('install'"));
        assert!(js.contains("addEventListener('activate'"));
        assert!(js.contains("addEventListener('fetch'"));
        assert!(js.contains("/index.html"));
        // NetworkFirst body specifics.
        assert!(js.contains("fetch(event.request).catch"));
        assert_eq!(sw.cache_key(), "legalis-3");
    }

    #[test]
    fn test_from_diff_index_html() {
        let bundle = PwaBundle::from_diff(&sample_diff());
        assert!(bundle.index_html.contains("Diff: law"));
        assert!(bundle.index_html.contains("manifest.webmanifest"));
        assert!(bundle.index_html.contains("serviceWorker"));
        assert!(bundle.index_html.contains("Title")); // change target
    }

    #[test]
    fn test_index_html_escapes_content() {
        let old = Statute::new("law", "<script>", Effect::new(EffectType::Grant, "x"));
        let mut new = old.clone();
        new.title = "safe".to_string();
        let d = diff(&old, &new).expect("diff");
        let bundle = PwaBundle::from_diff(&d);
        assert!(bundle.index_html.contains("&lt;script&gt;"));
        assert!(!bundle.index_html.contains("<script>safe")); // the value is escaped
    }

    #[test]
    fn test_files_have_expected_paths_and_mimes() {
        let bundle = PwaBundle::from_diff(&sample_diff());
        let files = bundle.files().expect("files");
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"manifest.webmanifest"));
        assert!(paths.contains(&"sw.js"));
        assert!(paths.contains(&"index.html"));
        let manifest_file = files
            .iter()
            .find(|f| f.path == "manifest.webmanifest")
            .expect("manifest");
        assert_eq!(manifest_file.mime_type, "application/manifest+json");
    }

    #[test]
    fn test_write_to_dir() {
        let bundle = PwaBundle::from_diff(&sample_diff());
        let mut dir = std::env::temp_dir();
        dir.push(format!("legalis_pwa_{}", std::process::id()));
        let written = bundle.write_to_dir(&dir).expect("write");
        assert_eq!(written.len(), 3);
        let manifest_path = dir.join("manifest.webmanifest");
        let contents = std::fs::read_to_string(&manifest_path).expect("read manifest");
        assert!(contents.contains("\"start_url\""));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
