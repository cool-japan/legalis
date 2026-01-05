//! Security testing helpers and OWASP checks.
//!
//! This module provides utilities for security testing, including
//! OWASP Top 10 vulnerability checks and security best practices.

use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};

/// Security check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheckResult {
    /// Check name
    pub check_name: String,
    /// Whether the check passed
    pub passed: bool,
    /// Severity if failed
    pub severity: Option<SecuritySeverity>,
    /// Description of the issue
    pub description: Option<String>,
    /// Recommendation for fix
    pub recommendation: Option<String>,
}

/// Security issue severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SecuritySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Security headers validator
pub struct SecurityHeaders;

impl SecurityHeaders {
    /// Checks for required security headers
    pub fn check_response_headers(headers: &HeaderMap) -> Vec<SecurityCheckResult> {
        vec![
            // Check for X-Content-Type-Options
            Self::check_header(
                headers,
                "X-Content-Type-Options",
                "nosniff",
                SecuritySeverity::Medium,
                "X-Content-Type-Options header missing or incorrect",
                "Add 'X-Content-Type-Options: nosniff' header to prevent MIME type sniffing",
            ),
            // Check for X-Frame-Options
            Self::check_header_exists(
                headers,
                "X-Frame-Options",
                SecuritySeverity::High,
                "X-Frame-Options header missing",
                "Add 'X-Frame-Options: DENY' or 'SAMEORIGIN' to prevent clickjacking",
            ),
            // Check for Strict-Transport-Security
            Self::check_header_exists(
                headers,
                "Strict-Transport-Security",
                SecuritySeverity::High,
                "Strict-Transport-Security header missing",
                "Add 'Strict-Transport-Security: max-age=31536000; includeSubDomains' for HTTPS",
            ),
            // Check for Content-Security-Policy
            Self::check_header_exists(
                headers,
                "Content-Security-Policy",
                SecuritySeverity::Medium,
                "Content-Security-Policy header missing",
                "Add Content-Security-Policy header to prevent XSS and injection attacks",
            ),
            // Check for X-XSS-Protection (deprecated but still useful for older browsers)
            Self::check_header(
                headers,
                "X-XSS-Protection",
                "1; mode=block",
                SecuritySeverity::Low,
                "X-XSS-Protection header missing or incorrect",
                "Add 'X-XSS-Protection: 1; mode=block' for older browser protection",
            ),
        ]
    }

    fn check_header(
        headers: &HeaderMap,
        name: &str,
        expected: &str,
        severity: SecuritySeverity,
        description: &str,
        recommendation: &str,
    ) -> SecurityCheckResult {
        let passed = headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(|v| v == expected)
            .unwrap_or(false);

        SecurityCheckResult {
            check_name: format!("Security Header: {}", name),
            passed,
            severity: if passed { None } else { Some(severity) },
            description: if passed {
                None
            } else {
                Some(description.to_string())
            },
            recommendation: if passed {
                None
            } else {
                Some(recommendation.to_string())
            },
        }
    }

    fn check_header_exists(
        headers: &HeaderMap,
        name: &str,
        severity: SecuritySeverity,
        description: &str,
        recommendation: &str,
    ) -> SecurityCheckResult {
        let passed = headers.contains_key(name);

        SecurityCheckResult {
            check_name: format!("Security Header: {}", name),
            passed,
            severity: if passed { None } else { Some(severity) },
            description: if passed {
                None
            } else {
                Some(description.to_string())
            },
            recommendation: if passed {
                None
            } else {
                Some(recommendation.to_string())
            },
        }
    }
}

/// Input validation helpers
pub struct InputValidator;

impl InputValidator {
    /// Checks for SQL injection patterns
    pub fn check_sql_injection(input: &str) -> SecurityCheckResult {
        let sql_patterns = [
            "UNION SELECT",
            "DROP TABLE",
            "INSERT INTO",
            "DELETE FROM",
            "' OR '1'='1",
            "'; DROP",
            "1=1--",
            "admin'--",
        ];

        let upper_input = input.to_uppercase();
        let has_sql_pattern = sql_patterns
            .iter()
            .any(|pattern| upper_input.contains(pattern));

        SecurityCheckResult {
            check_name: "SQL Injection Check".to_string(),
            passed: !has_sql_pattern,
            severity: if has_sql_pattern {
                Some(SecuritySeverity::Critical)
            } else {
                None
            },
            description: if has_sql_pattern {
                Some("Potential SQL injection pattern detected".to_string())
            } else {
                None
            },
            recommendation: if has_sql_pattern {
                Some("Use parameterized queries and input validation".to_string())
            } else {
                None
            },
        }
    }

    /// Checks for XSS patterns
    pub fn check_xss(input: &str) -> SecurityCheckResult {
        let xss_patterns = ["<script", "javascript:", "onerror=", "onload=", "onclick="];

        let lower_input = input.to_lowercase();
        let has_xss_pattern = xss_patterns
            .iter()
            .any(|pattern| lower_input.contains(pattern));

        SecurityCheckResult {
            check_name: "XSS Check".to_string(),
            passed: !has_xss_pattern,
            severity: if has_xss_pattern {
                Some(SecuritySeverity::High)
            } else {
                None
            },
            description: if has_xss_pattern {
                Some("Potential XSS pattern detected".to_string())
            } else {
                None
            },
            recommendation: if has_xss_pattern {
                Some("Sanitize user input and use Content-Security-Policy".to_string())
            } else {
                None
            },
        }
    }

    /// Checks for path traversal patterns
    pub fn check_path_traversal(input: &str) -> SecurityCheckResult {
        let has_traversal = input.contains("../") || input.contains("..\\");

        SecurityCheckResult {
            check_name: "Path Traversal Check".to_string(),
            passed: !has_traversal,
            severity: if has_traversal {
                Some(SecuritySeverity::High)
            } else {
                None
            },
            description: if has_traversal {
                Some("Potential path traversal pattern detected".to_string())
            } else {
                None
            },
            recommendation: if has_traversal {
                Some("Validate and sanitize file paths, use allowlist".to_string())
            } else {
                None
            },
        }
    }
}

/// CORS security checker
pub struct CorsChecker;

impl CorsChecker {
    /// Checks CORS configuration for security issues
    pub fn check_cors_config(allowed_origins: &[String]) -> Vec<SecurityCheckResult> {
        let mut results = Vec::new();

        // Check for wildcard origin
        let has_wildcard = allowed_origins.iter().any(|origin| origin == "*");
        results.push(SecurityCheckResult {
            check_name: "CORS Wildcard Check".to_string(),
            passed: !has_wildcard,
            severity: if has_wildcard {
                Some(SecuritySeverity::Medium)
            } else {
                None
            },
            description: if has_wildcard {
                Some("CORS allows all origins (*)".to_string())
            } else {
                None
            },
            recommendation: if has_wildcard {
                Some("Use specific allowed origins instead of wildcard".to_string())
            } else {
                None
            },
        });

        results
    }
}

/// Security middleware to add security headers
pub async fn security_headers_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut response = next.run(req).await;

    let headers = response.headers_mut();

    // Add security headers
    if let Ok(value) = HeaderValue::from_str("nosniff") {
        headers.insert("X-Content-Type-Options", value);
    }

    if let Ok(value) = HeaderValue::from_str("DENY") {
        headers.insert("X-Frame-Options", value);
    }

    if let Ok(value) = HeaderValue::from_str("1; mode=block") {
        headers.insert("X-XSS-Protection", value);
    }

    // Add CSP header (basic policy)
    if let Ok(value) = HeaderValue::from_str("default-src 'self'") {
        headers.insert("Content-Security-Policy", value);
    }

    // Add Referrer-Policy
    if let Ok(value) = HeaderValue::from_str("strict-origin-when-cross-origin") {
        headers.insert("Referrer-Policy", value);
    }

    // Add Permissions-Policy
    if let Ok(value) = HeaderValue::from_str("geolocation=(), microphone=(), camera=()") {
        headers.insert("Permissions-Policy", value);
    }

    Ok(response)
}

/// Rate limiting token bucket for DoS protection
#[derive(Clone)]
pub struct TokenBucket {
    /// Tokens per IP
    tokens: std::sync::Arc<
        tokio::sync::RwLock<std::collections::HashMap<String, (u32, std::time::Instant)>>,
    >,
    /// Maximum tokens
    max_tokens: u32,
    /// Refill rate (tokens per second)
    refill_rate: u32,
}

impl TokenBucket {
    /// Creates a new token bucket
    pub fn new(max_tokens: u32, refill_rate: u32) -> Self {
        Self {
            tokens: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            max_tokens,
            refill_rate,
        }
    }

    /// Checks if a request is allowed
    pub async fn allow(&self, ip: &str) -> bool {
        let mut tokens = self.tokens.write().await;
        let now = std::time::Instant::now();

        let (count, last_refill) = tokens
            .entry(ip.to_string())
            .or_insert((self.max_tokens, now));

        // Refill tokens based on time elapsed
        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        let new_tokens = (elapsed * self.refill_rate as f64) as u32;
        *count = (*count + new_tokens).min(self.max_tokens);
        *last_refill = now;

        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_injection_check() {
        let result = InputValidator::check_sql_injection("' OR '1'='1");
        assert!(!result.passed);
        assert_eq!(result.severity, Some(SecuritySeverity::Critical));

        let result = InputValidator::check_sql_injection("normal input");
        assert!(result.passed);
    }

    #[test]
    fn test_xss_check() {
        let result = InputValidator::check_xss("<script>alert('xss')</script>");
        assert!(!result.passed);
        assert_eq!(result.severity, Some(SecuritySeverity::High));

        let result = InputValidator::check_xss("normal text");
        assert!(result.passed);
    }

    #[test]
    fn test_path_traversal_check() {
        let result = InputValidator::check_path_traversal("../../etc/passwd");
        assert!(!result.passed);
        assert_eq!(result.severity, Some(SecuritySeverity::High));

        let result = InputValidator::check_path_traversal("normal/path");
        assert!(result.passed);
    }

    #[test]
    fn test_cors_wildcard_check() {
        let results = CorsChecker::check_cors_config(&["*".to_string()]);
        assert!(!results[0].passed);
        assert_eq!(results[0].severity, Some(SecuritySeverity::Medium));

        let results = CorsChecker::check_cors_config(&["https://example.com".to_string()]);
        assert!(results[0].passed);
    }

    #[test]
    fn test_security_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Content-Type-Options",
            HeaderValue::from_static("nosniff"),
        );

        let results = SecurityHeaders::check_response_headers(&headers);
        assert!(
            results
                .iter()
                .any(|r| r.check_name.contains("X-Content-Type-Options") && r.passed)
        );
    }

    #[tokio::test]
    async fn test_token_bucket() {
        let bucket = TokenBucket::new(10, 1);

        // Should allow first 10 requests
        for _ in 0..10 {
            assert!(bucket.allow("127.0.0.1").await);
        }

        // 11th request should be denied
        assert!(!bucket.allow("127.0.0.1").await);
    }
}
