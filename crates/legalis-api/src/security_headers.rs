//! Security headers automation.
//!
//! Provides a configurable policy for HTTP security response headers and an
//! Axum middleware that applies them automatically to every response. Unlike
//! the fixed header set in [`crate::security`], this module exposes a fully
//! configurable [`SecurityHeadersConfig`] (HSTS parameters, CSP directives,
//! frame options, referrer / permissions policy, COOP/CORP/COEP, etc.) plus
//! sensible hardened defaults.

use axum::{
    body::Body,
    http::{HeaderMap, HeaderName, HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

/// `X-Frame-Options` policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameOptions {
    /// Deny all framing.
    Deny,
    /// Allow framing only by same-origin pages.
    SameOrigin,
}

impl FrameOptions {
    fn header_value(&self) -> &'static str {
        match self {
            FrameOptions::Deny => "DENY",
            FrameOptions::SameOrigin => "SAMEORIGIN",
        }
    }
}

/// HTTP Strict Transport Security configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HstsConfig {
    /// Max-age in seconds.
    pub max_age_secs: u64,
    /// Whether to apply to subdomains.
    pub include_subdomains: bool,
    /// Whether to request browser preload-list inclusion.
    pub preload: bool,
}

impl Default for HstsConfig {
    fn default() -> Self {
        Self {
            // 1 year.
            max_age_secs: 31_536_000,
            include_subdomains: true,
            preload: false,
        }
    }
}

impl HstsConfig {
    /// Renders the `Strict-Transport-Security` header value.
    pub fn header_value(&self) -> String {
        let mut value = format!("max-age={}", self.max_age_secs);
        if self.include_subdomains {
            value.push_str("; includeSubDomains");
        }
        if self.preload {
            value.push_str("; preload");
        }
        value
    }
}

/// Configurable security headers policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeadersConfig {
    /// HSTS configuration (None disables the header — appropriate for plain HTTP).
    pub hsts: Option<HstsConfig>,
    /// Content-Security-Policy value (None disables).
    pub content_security_policy: Option<String>,
    /// X-Frame-Options policy (None disables).
    pub frame_options: Option<FrameOptions>,
    /// Whether to emit `X-Content-Type-Options: nosniff`.
    pub content_type_nosniff: bool,
    /// Referrer-Policy value (None disables).
    pub referrer_policy: Option<String>,
    /// Permissions-Policy value (None disables).
    pub permissions_policy: Option<String>,
    /// Cross-Origin-Opener-Policy value (None disables).
    pub cross_origin_opener_policy: Option<String>,
    /// Cross-Origin-Resource-Policy value (None disables).
    pub cross_origin_resource_policy: Option<String>,
    /// Cross-Origin-Embedder-Policy value (None disables).
    pub cross_origin_embedder_policy: Option<String>,
    /// `X-XSS-Protection` value (None disables; deprecated but still requested
    /// by some compliance baselines for legacy browsers).
    pub xss_protection: Option<String>,
    /// Additional arbitrary headers to set (name -> value).
    pub extra_headers: BTreeMap<String, String>,
}

impl Default for SecurityHeadersConfig {
    /// A hardened default suitable for an HTTPS API.
    fn default() -> Self {
        Self {
            hsts: Some(HstsConfig::default()),
            content_security_policy: Some(
                "default-src 'self'; frame-ancestors 'none'; base-uri 'self'; object-src 'none'"
                    .to_string(),
            ),
            frame_options: Some(FrameOptions::Deny),
            content_type_nosniff: true,
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            permissions_policy: Some("geolocation=(), microphone=(), camera=()".to_string()),
            cross_origin_opener_policy: Some("same-origin".to_string()),
            cross_origin_resource_policy: Some("same-origin".to_string()),
            cross_origin_embedder_policy: None,
            xss_protection: Some("0".to_string()),
            extra_headers: BTreeMap::new(),
        }
    }
}

impl SecurityHeadersConfig {
    /// Creates a configuration with all headers disabled; build up explicitly.
    pub fn minimal() -> Self {
        Self {
            hsts: None,
            content_security_policy: None,
            frame_options: None,
            content_type_nosniff: false,
            referrer_policy: None,
            permissions_policy: None,
            cross_origin_opener_policy: None,
            cross_origin_resource_policy: None,
            cross_origin_embedder_policy: None,
            xss_protection: None,
            extra_headers: BTreeMap::new(),
        }
    }

    /// Builds the full set of header name/value pairs to apply.
    ///
    /// Only valid header values are returned; values that cannot be encoded as a
    /// header are silently skipped (they never reach this point with the typed
    /// builders, but `extra_headers` are user-supplied).
    pub fn build_headers(&self) -> Vec<(HeaderName, HeaderValue)> {
        let mut out: Vec<(HeaderName, HeaderValue)> = Vec::new();

        let push = |name: &'static str, value: String, out: &mut Vec<_>| {
            if let (Ok(n), Ok(v)) = (
                HeaderName::from_bytes(name.as_bytes()),
                HeaderValue::from_str(&value),
            ) {
                out.push((n, v));
            }
        };

        if let Some(hsts) = &self.hsts {
            push("strict-transport-security", hsts.header_value(), &mut out);
        }
        if let Some(csp) = &self.content_security_policy {
            push("content-security-policy", csp.clone(), &mut out);
        }
        if let Some(frame) = &self.frame_options {
            push(
                "x-frame-options",
                frame.header_value().to_string(),
                &mut out,
            );
        }
        if self.content_type_nosniff {
            push("x-content-type-options", "nosniff".to_string(), &mut out);
        }
        if let Some(rp) = &self.referrer_policy {
            push("referrer-policy", rp.clone(), &mut out);
        }
        if let Some(pp) = &self.permissions_policy {
            push("permissions-policy", pp.clone(), &mut out);
        }
        if let Some(coop) = &self.cross_origin_opener_policy {
            push("cross-origin-opener-policy", coop.clone(), &mut out);
        }
        if let Some(corp) = &self.cross_origin_resource_policy {
            push("cross-origin-resource-policy", corp.clone(), &mut out);
        }
        if let Some(coep) = &self.cross_origin_embedder_policy {
            push("cross-origin-embedder-policy", coep.clone(), &mut out);
        }
        if let Some(xss) = &self.xss_protection {
            push("x-xss-protection", xss.clone(), &mut out);
        }
        for (name, value) in &self.extra_headers {
            if let (Ok(n), Ok(v)) = (
                HeaderName::from_bytes(name.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                out.push((n, v));
            }
        }
        out
    }

    /// Applies the configured headers to a header map, overwriting existing
    /// values with the same name.
    pub fn apply(&self, headers: &mut HeaderMap) {
        for (name, value) in self.build_headers() {
            headers.insert(name, value);
        }
    }
}

/// Axum middleware that applies the configured security headers to responses.
pub async fn security_headers_middleware(
    config: Arc<SecurityHeadersConfig>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let mut response = next.run(req).await;
    config.apply(response.headers_mut());
    response
}

/// Extension trait for attaching automated security headers to a router.
pub trait SecurityHeadersExt {
    /// Adds the security-headers middleware with the given config.
    fn with_security_headers(self, config: SecurityHeadersConfig) -> Self;
}

impl SecurityHeadersExt for axum::Router {
    fn with_security_headers(self, config: SecurityHeadersConfig) -> Self {
        let cfg = Arc::new(config);
        self.layer(axum::middleware::from_fn(move |req, next| {
            security_headers_middleware(cfg.clone(), req, next)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsts_header_value() {
        let hsts = HstsConfig {
            max_age_secs: 100,
            include_subdomains: true,
            preload: true,
        };
        assert_eq!(
            hsts.header_value(),
            "max-age=100; includeSubDomains; preload"
        );

        let hsts2 = HstsConfig {
            max_age_secs: 200,
            include_subdomains: false,
            preload: false,
        };
        assert_eq!(hsts2.header_value(), "max-age=200");
    }

    #[test]
    fn test_default_builds_expected_headers() {
        let config = SecurityHeadersConfig::default();
        let headers = config.build_headers();
        let names: Vec<String> = headers
            .iter()
            .map(|(n, _)| n.as_str().to_string())
            .collect();
        assert!(names.contains(&"strict-transport-security".to_string()));
        assert!(names.contains(&"content-security-policy".to_string()));
        assert!(names.contains(&"x-frame-options".to_string()));
        assert!(names.contains(&"x-content-type-options".to_string()));
        assert!(names.contains(&"referrer-policy".to_string()));
        assert!(names.contains(&"permissions-policy".to_string()));
        assert!(names.contains(&"cross-origin-opener-policy".to_string()));
    }

    #[test]
    fn test_minimal_emits_nothing() {
        let config = SecurityHeadersConfig::minimal();
        assert!(config.build_headers().is_empty());
    }

    #[test]
    fn test_frame_options_values() {
        assert_eq!(FrameOptions::Deny.header_value(), "DENY");
        assert_eq!(FrameOptions::SameOrigin.header_value(), "SAMEORIGIN");
    }

    #[test]
    fn test_apply_to_header_map() {
        let config = SecurityHeadersConfig::default();
        let mut headers = HeaderMap::new();
        config.apply(&mut headers);
        assert_eq!(
            headers
                .get("x-content-type-options")
                .and_then(|v| v.to_str().ok()),
            Some("nosniff")
        );
        assert_eq!(
            headers.get("x-frame-options").and_then(|v| v.to_str().ok()),
            Some("DENY")
        );
    }

    #[test]
    fn test_extra_headers() {
        let mut config = SecurityHeadersConfig::minimal();
        config
            .extra_headers
            .insert("X-Custom-Security".to_string(), "enabled".to_string());
        let mut headers = HeaderMap::new();
        config.apply(&mut headers);
        assert_eq!(
            headers
                .get("x-custom-security")
                .and_then(|v| v.to_str().ok()),
            Some("enabled")
        );
    }

    #[test]
    fn test_apply_overwrites_existing() {
        let config = SecurityHeadersConfig::default();
        let mut headers = HeaderMap::new();
        headers.insert("x-frame-options", HeaderValue::from_static("SAMEORIGIN"));
        config.apply(&mut headers);
        // Default config uses DENY and should overwrite.
        assert_eq!(
            headers.get("x-frame-options").and_then(|v| v.to_str().ok()),
            Some("DENY")
        );
    }

    #[tokio::test]
    async fn test_middleware_adds_headers() {
        use axum::{Router, routing::get};
        use tower::ServiceExt;

        let app: Router = Router::new()
            .route("/", get(|| async { "ok" }))
            .with_security_headers(SecurityHeadersConfig::default());

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(
            resp.headers()
                .get("strict-transport-security")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.contains("max-age=")),
            Some(true)
        );
        assert!(resp.headers().contains_key("content-security-policy"));
    }
}
