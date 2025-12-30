//! API versioning support.
//!
//! This module provides URL-based and header-based API versioning
//! to support multiple API versions simultaneously.

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// API version
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ApiVersion {
    /// Version 1 (current)
    V1,
    /// Version 2 (future)
    V2,
}

impl ApiVersion {
    /// Gets the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "v1",
            ApiVersion::V2 => "v2",
        }
    }

    /// Gets the latest stable version
    pub fn latest() -> Self {
        ApiVersion::V1
    }

    /// Checks if this version is deprecated
    pub fn is_deprecated(&self) -> bool {
        match self {
            ApiVersion::V1 => false,
            ApiVersion::V2 => false, // No deprecated versions yet
        }
    }

    /// Gets the sunset date for this version (if deprecated)
    pub fn sunset_date(&self) -> Option<&'static str> {
        match self {
            ApiVersion::V1 => None,
            ApiVersion::V2 => None,
        }
    }
}

impl FromStr for ApiVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "v1" | "1" => Ok(ApiVersion::V1),
            "v2" | "2" => Ok(ApiVersion::V2),
            _ => Err(format!("Unknown API version: {}", s)),
        }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Extracts API version from request
pub fn extract_version_from_request(req: &Request<Body>) -> ApiVersion {
    // Try URL-based versioning first
    let path = req.uri().path();
    if let Some(version) = extract_version_from_path(path) {
        return version;
    }

    // Try header-based versioning
    if let Some(header_version) = req.headers().get("X-API-Version") {
        if let Ok(version_str) = header_version.to_str() {
            if let Ok(version) = ApiVersion::from_str(version_str) {
                return version;
            }
        }
    }

    // Try Accept header versioning (e.g., application/vnd.legalis.v1+json)
    if let Some(accept) = req.headers().get(header::ACCEPT) {
        if let Ok(accept_str) = accept.to_str() {
            if accept_str.contains(".v1+") {
                return ApiVersion::V1;
            } else if accept_str.contains(".v2+") {
                return ApiVersion::V2;
            }
        }
    }

    // Default to latest version
    ApiVersion::latest()
}

/// Extracts version from URL path
fn extract_version_from_path(path: &str) -> Option<ApiVersion> {
    // Match patterns like /api/v1/... or /v1/...
    let parts: Vec<&str> = path.split('/').collect();

    for part in parts {
        if part.starts_with('v') || part.starts_with('V') {
            if let Ok(version) = ApiVersion::from_str(part) {
                return Some(version);
            }
        }
    }

    None
}

/// Middleware to add version headers to responses
pub async fn version_headers_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let version = extract_version_from_request(&req);
    let mut response = next.run(req).await;

    // Add version header
    if let Ok(version_value) = HeaderValue::from_str(version.as_str()) {
        response
            .headers_mut()
            .insert("X-API-Version", version_value);
    }

    // Add deprecation warning if applicable
    if version.is_deprecated() {
        if let Ok(warning_value) =
            HeaderValue::from_str(&format!("299 - \"API version {} is deprecated\"", version))
        {
            response
                .headers_mut()
                .insert(header::WARNING, warning_value);
        }

        if let Some(sunset) = version.sunset_date() {
            if let Ok(sunset_value) = HeaderValue::from_str(sunset) {
                response.headers_mut().insert("Sunset", sunset_value);
            }
        }
    }

    // Add Link header for version discovery
    if let Ok(link_value) = HeaderValue::from_str(&format!(
        "</api/v1>; rel=\"version\", </api/v2>; rel=\"version\""
    )) {
        response.headers_mut().insert(header::LINK, link_value);
    }

    Ok(response)
}

/// Version migration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionMigration {
    /// Source version
    pub from_version: ApiVersion,
    /// Target version
    pub to_version: ApiVersion,
    /// Breaking changes
    pub breaking_changes: Vec<String>,
    /// Deprecations
    pub deprecations: Vec<String>,
    /// New features
    pub new_features: Vec<String>,
    /// Migration guide URL
    pub migration_guide_url: Option<String>,
}

impl VersionMigration {
    /// Creates a migration guide from v1 to v2
    pub fn v1_to_v2() -> Self {
        Self {
            from_version: ApiVersion::V1,
            to_version: ApiVersion::V2,
            breaking_changes: vec![
                "Changed statute ID format to UUID".to_string(),
                "Removed deprecated /legacy endpoints".to_string(),
            ],
            deprecations: vec![
                "Batch operations endpoint deprecated, use streaming instead".to_string(),
            ],
            new_features: vec![
                "Added collaborative editing support".to_string(),
                "Real-time conflict detection".to_string(),
                "Enhanced caching with CDN support".to_string(),
            ],
            migration_guide_url: Some(
                "https://docs.legalis.example/migration/v1-to-v2".to_string(),
            ),
        }
    }
}

/// API version compatibility checker
pub struct CompatibilityChecker;

impl CompatibilityChecker {
    /// Checks if a feature is available in a version
    pub fn feature_available(version: ApiVersion, feature: &str) -> bool {
        match (version, feature) {
            (ApiVersion::V1, "collaborative_editing") => false,
            (ApiVersion::V2, "collaborative_editing") => true,
            (ApiVersion::V1, "edge_caching") => false,
            (ApiVersion::V2, "edge_caching") => true,
            (ApiVersion::V1, "basic_crud") => true,
            (ApiVersion::V2, "basic_crud") => true,
            _ => false,
        }
    }

    /// Checks if an endpoint is supported in a version
    pub fn endpoint_supported(version: ApiVersion, endpoint: &str) -> bool {
        match (version, endpoint) {
            // Specific endpoints first
            (ApiVersion::V1, "/api/v1/legacy") => true,
            (ApiVersion::V2, "/api/v2/legacy") => false, // Removed in v2
            // General patterns
            (ApiVersion::V1, path) if path.starts_with("/api/v1/") => true,
            (ApiVersion::V2, path) if path.starts_with("/api/v2/") => true,
            _ => false,
        }
    }
}

/// Version negotiation result
#[derive(Debug, Clone, Serialize)]
pub struct VersionNegotiation {
    /// Requested version
    pub requested: ApiVersion,
    /// Actual version used
    pub actual: ApiVersion,
    /// Whether the requested version is supported
    pub supported: bool,
    /// Fallback reason (if different from requested)
    pub fallback_reason: Option<String>,
}

/// Response for unsupported API version
#[derive(Debug, Clone, Serialize)]
pub struct UnsupportedVersionResponse {
    pub error: String,
    pub requested_version: String,
    pub supported_versions: Vec<String>,
    pub latest_version: String,
}

impl IntoResponse for UnsupportedVersionResponse {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, axum::Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_version_from_str() {
        assert_eq!(ApiVersion::from_str("v1").unwrap(), ApiVersion::V1);
        assert_eq!(ApiVersion::from_str("V1").unwrap(), ApiVersion::V1);
        assert_eq!(ApiVersion::from_str("1").unwrap(), ApiVersion::V1);
        assert_eq!(ApiVersion::from_str("v2").unwrap(), ApiVersion::V2);
        assert!(ApiVersion::from_str("v3").is_err());
    }

    #[test]
    fn test_api_version_display() {
        assert_eq!(format!("{}", ApiVersion::V1), "v1");
        assert_eq!(format!("{}", ApiVersion::V2), "v2");
    }

    #[test]
    fn test_extract_version_from_path() {
        assert_eq!(
            extract_version_from_path("/api/v1/statutes"),
            Some(ApiVersion::V1)
        );
        assert_eq!(
            extract_version_from_path("/api/v2/statutes"),
            Some(ApiVersion::V2)
        );
        assert_eq!(extract_version_from_path("/api/statutes"), None);
    }

    #[test]
    fn test_version_latest() {
        assert_eq!(ApiVersion::latest(), ApiVersion::V1);
    }

    #[test]
    fn test_version_deprecated() {
        assert!(!ApiVersion::V1.is_deprecated());
        assert!(!ApiVersion::V2.is_deprecated());
    }

    #[test]
    fn test_compatibility_checker() {
        assert!(CompatibilityChecker::feature_available(
            ApiVersion::V1,
            "basic_crud"
        ));
        assert!(!CompatibilityChecker::feature_available(
            ApiVersion::V1,
            "collaborative_editing"
        ));
        assert!(CompatibilityChecker::feature_available(
            ApiVersion::V2,
            "collaborative_editing"
        ));
    }

    #[test]
    fn test_endpoint_supported() {
        assert!(CompatibilityChecker::endpoint_supported(
            ApiVersion::V1,
            "/api/v1/statutes"
        ));
        assert!(CompatibilityChecker::endpoint_supported(
            ApiVersion::V1,
            "/api/v1/legacy"
        ));
        assert!(!CompatibilityChecker::endpoint_supported(
            ApiVersion::V2,
            "/api/v2/legacy"
        ));
    }
}
