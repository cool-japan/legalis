//! Prefetching hints via HTTP `Link` headers.
//!
//! Generates RFC 8288 `Link` header values carrying resource hints such as
//! `preload`, `prefetch`, `preconnect`, and `dns-prefetch`, plus pagination
//! relations (`next`, `prev`, `first`, `last`). Emitting these hints lets clients
//! (and intermediaries) fetch resources the server anticipates will be needed
//! next, reducing perceived latency.

use axum::http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

/// A relationship type for a [`LinkHint`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LinkRel {
    /// High-priority fetch of a resource needed for the current navigation.
    Preload,
    /// Speculative fetch of a resource likely needed for a future navigation.
    Prefetch,
    /// Establish an early connection (TCP/TLS) to an origin.
    Preconnect,
    /// Resolve DNS for an origin early.
    DnsPrefetch,
    /// Pagination: next page.
    Next,
    /// Pagination: previous page.
    Prev,
    /// Pagination: first page.
    First,
    /// Pagination: last page.
    Last,
    /// A related resource.
    Related,
    /// An arbitrary custom relation.
    Custom(String),
}

impl LinkRel {
    /// Returns the textual relation token.
    pub fn as_token(&self) -> &str {
        match self {
            LinkRel::Preload => "preload",
            LinkRel::Prefetch => "prefetch",
            LinkRel::Preconnect => "preconnect",
            LinkRel::DnsPrefetch => "dns-prefetch",
            LinkRel::Next => "next",
            LinkRel::Prev => "prev",
            LinkRel::First => "first",
            LinkRel::Last => "last",
            LinkRel::Related => "related",
            LinkRel::Custom(s) => s.as_str(),
        }
    }
}

/// The `as` attribute used with `preload`/`prefetch` to declare destination type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AsType {
    /// A document.
    Document,
    /// A stylesheet.
    Style,
    /// A script.
    Script,
    /// An image.
    Image,
    /// A font.
    Font,
    /// A fetch/XHR request (e.g. JSON API resource).
    Fetch,
}

impl AsType {
    fn as_token(&self) -> &'static str {
        match self {
            AsType::Document => "document",
            AsType::Style => "style",
            AsType::Script => "script",
            AsType::Image => "image",
            AsType::Font => "font",
            AsType::Fetch => "fetch",
        }
    }
}

/// A single prefetch/preload link hint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkHint {
    /// Target URI.
    pub uri: String,
    /// Relationship type.
    pub rel: LinkRel,
    /// Optional destination type (`as=`).
    pub as_type: Option<AsType>,
    /// Optional MIME type (`type=`).
    pub mime_type: Option<String>,
    /// Whether the request should be made with CORS (`crossorigin`).
    pub crossorigin: bool,
}

impl LinkHint {
    /// Creates a hint with the given URI and relation.
    pub fn new(uri: impl Into<String>, rel: LinkRel) -> Self {
        Self {
            uri: uri.into(),
            rel,
            as_type: None,
            mime_type: None,
            crossorigin: false,
        }
    }

    /// Convenience constructor for a `preload` hint with a destination type.
    pub fn preload(uri: impl Into<String>, as_type: AsType) -> Self {
        Self {
            uri: uri.into(),
            rel: LinkRel::Preload,
            as_type: Some(as_type),
            mime_type: None,
            crossorigin: false,
        }
    }

    /// Convenience constructor for a `prefetch` hint.
    pub fn prefetch(uri: impl Into<String>) -> Self {
        Self::new(uri, LinkRel::Prefetch)
    }

    /// Sets the `as` destination type.
    pub fn with_as(mut self, as_type: AsType) -> Self {
        self.as_type = Some(as_type);
        self
    }

    /// Sets the MIME `type`.
    pub fn with_mime(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }

    /// Marks the hint as `crossorigin`.
    pub fn with_crossorigin(mut self) -> Self {
        self.crossorigin = true;
        self
    }

    /// Renders this hint as a single `Link` header field value, e.g.
    /// `</next?cursor=abc>; rel="prefetch"; as="fetch"`.
    pub fn render(&self) -> String {
        let mut s = format!("<{}>; rel=\"{}\"", self.uri, self.rel.as_token());
        if let Some(as_type) = &self.as_type {
            s.push_str(&format!("; as=\"{}\"", as_type.as_token()));
        }
        if let Some(mime) = &self.mime_type {
            s.push_str(&format!("; type=\"{mime}\""));
        }
        if self.crossorigin {
            s.push_str("; crossorigin");
        }
        s
    }
}

/// A collection of link hints rendered into a single `Link` header value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrefetchHints {
    hints: Vec<LinkHint>,
}

impl PrefetchHints {
    /// Creates an empty hint set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a hint.
    pub fn push(&mut self, hint: LinkHint) -> &mut Self {
        self.hints.push(hint);
        self
    }

    /// Adds a hint, builder-style.
    pub fn with(mut self, hint: LinkHint) -> Self {
        self.hints.push(hint);
        self
    }

    /// Adds a pagination `next` hint pointing at a cursor URL.
    pub fn next_page(mut self, uri: impl Into<String>) -> Self {
        self.hints
            .push(LinkHint::new(uri, LinkRel::Next).with_as(AsType::Fetch));
        self
    }

    /// Adds a pagination `prev` hint pointing at a cursor URL.
    pub fn prev_page(mut self, uri: impl Into<String>) -> Self {
        self.hints
            .push(LinkHint::new(uri, LinkRel::Prev).with_as(AsType::Fetch));
        self
    }

    /// Returns the number of hints.
    pub fn len(&self) -> usize {
        self.hints.len()
    }

    /// Returns whether there are no hints.
    pub fn is_empty(&self) -> bool {
        self.hints.is_empty()
    }

    /// Renders all hints into a single comma-separated `Link` header value.
    ///
    /// Returns `None` if there are no hints.
    pub fn render(&self) -> Option<String> {
        if self.hints.is_empty() {
            return None;
        }
        Some(
            self.hints
                .iter()
                .map(|h| h.render())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }

    /// Produces the `(Link, value)` header pair, if any hints are present and the
    /// value is a valid header value.
    pub fn header(&self) -> Option<(HeaderName, HeaderValue)> {
        let value = self.render()?;
        let header_value = HeaderValue::from_str(&value).ok()?;
        Some((axum::http::header::LINK, header_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_preload() {
        let hint = LinkHint::preload("/styles.css", AsType::Style);
        assert_eq!(
            hint.render(),
            "</styles.css>; rel=\"preload\"; as=\"style\""
        );
    }

    #[test]
    fn test_render_prefetch_with_mime_and_crossorigin() {
        let hint = LinkHint::prefetch("/data.json")
            .with_as(AsType::Fetch)
            .with_mime("application/json")
            .with_crossorigin();
        assert_eq!(
            hint.render(),
            "</data.json>; rel=\"prefetch\"; as=\"fetch\"; type=\"application/json\"; crossorigin"
        );
    }

    #[test]
    fn test_custom_rel() {
        let hint = LinkHint::new("/x", LinkRel::Custom("api-docs".to_string()));
        assert_eq!(hint.render(), "</x>; rel=\"api-docs\"");
    }

    #[test]
    fn test_pagination_relations() {
        let hints = PrefetchHints::new()
            .next_page("/items?cursor=abc")
            .prev_page("/items?cursor=xyz");
        let rendered = hints.render().expect("rendered");
        assert!(rendered.contains("rel=\"next\""));
        assert!(rendered.contains("rel=\"prev\""));
        assert!(rendered.contains("/items?cursor=abc"));
    }

    #[test]
    fn test_render_multiple_joined() {
        let hints = PrefetchHints::new()
            .with(LinkHint::preload("/a.js", AsType::Script))
            .with(LinkHint::prefetch("/b"));
        let rendered = hints.render().expect("rendered");
        assert_eq!(
            rendered,
            "</a.js>; rel=\"preload\"; as=\"script\", </b>; rel=\"prefetch\""
        );
    }

    #[test]
    fn test_empty_renders_none() {
        let hints = PrefetchHints::new();
        assert!(hints.is_empty());
        assert!(hints.render().is_none());
        assert!(hints.header().is_none());
    }

    #[test]
    fn test_header_pair() {
        let hints = PrefetchHints::new().with(LinkHint::prefetch("/next"));
        let (name, value) = hints.header().expect("header");
        assert_eq!(name, axum::http::header::LINK);
        assert!(value.to_str().expect("str").contains("rel=\"prefetch\""));
    }

    #[test]
    fn test_rel_tokens() {
        assert_eq!(LinkRel::Preconnect.as_token(), "preconnect");
        assert_eq!(LinkRel::DnsPrefetch.as_token(), "dns-prefetch");
        assert_eq!(LinkRel::First.as_token(), "first");
        assert_eq!(LinkRel::Last.as_token(), "last");
    }
}
