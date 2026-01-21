//! Multi-tenancy support for Legalis API.
//!
//! This module provides tenant isolation functionality to ensure that
//! resources from different tenants are properly isolated.

use axum::{
    extract::{FromRequestParts, Request},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Tenant context extracted from request headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantContext {
    /// Unique tenant identifier
    pub tenant_id: String,
    /// Tenant name for display
    pub tenant_name: Option<String>,
}

impl TenantContext {
    /// Create a new tenant context.
    pub fn new(tenant_id: String) -> Self {
        Self {
            tenant_id,
            tenant_name: None,
        }
    }

    /// Create a tenant context with a name.
    pub fn with_name(tenant_id: String, tenant_name: String) -> Self {
        Self {
            tenant_id,
            tenant_name: Some(tenant_name),
        }
    }
}

/// Default tenant for development/single-tenant deployments.
impl Default for TenantContext {
    fn default() -> Self {
        Self {
            tenant_id: "default".to_string(),
            tenant_name: Some("Default Tenant".to_string()),
        }
    }
}

impl<S> FromRequestParts<S> for TenantContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract tenant ID from header
        if let Some(tenant_id) = parts.headers.get("X-Tenant-ID")
            && let Ok(tenant_id_str) = tenant_id.to_str()
        {
            let tenant_name = parts
                .headers
                .get("X-Tenant-Name")
                .and_then(|v| v.to_str().ok())
                .map(String::from);

            return Ok(TenantContext {
                tenant_id: tenant_id_str.to_string(),
                tenant_name,
            });
        }

        // Default to "default" tenant if no header is provided
        Ok(TenantContext::default())
    }
}

/// Middleware to enforce tenant isolation.
pub async fn tenant_isolation_middleware(
    tenant_ctx: TenantContext,
    request: Request,
    next: Next,
) -> Response {
    // Add tenant context to request extensions
    let mut request = request;
    request.extensions_mut().insert(Arc::new(tenant_ctx));

    next.run(request).await
}

/// Error response for tenant isolation violations.
#[derive(Debug, Serialize)]
pub struct TenantIsolationError {
    pub error: String,
    pub tenant_id: String,
}

impl IntoResponse for TenantIsolationError {
    fn into_response(self) -> Response {
        (StatusCode::FORBIDDEN, axum::Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_context_creation() {
        let ctx = TenantContext::new("tenant-123".to_string());
        assert_eq!(ctx.tenant_id, "tenant-123");
        assert_eq!(ctx.tenant_name, None);
    }

    #[test]
    fn test_tenant_context_with_name() {
        let ctx = TenantContext::with_name("tenant-123".to_string(), "Acme Corp".to_string());
        assert_eq!(ctx.tenant_id, "tenant-123");
        assert_eq!(ctx.tenant_name, Some("Acme Corp".to_string()));
    }

    #[test]
    fn test_default_tenant() {
        let ctx = TenantContext::default();
        assert_eq!(ctx.tenant_id, "default");
        assert_eq!(ctx.tenant_name, Some("Default Tenant".to_string()));
    }
}
