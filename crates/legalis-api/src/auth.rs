//! Authentication and authorization module for Legalis API.
//!
//! Provides JWT-based authentication, API key authentication, and role-based access control.

use axum::{
    extract::{FromRequestParts, Request},
    http::{StatusCode, header, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;
use uuid::Uuid;

/// Authentication errors.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing authentication credentials")]
    MissingCredentials,

    #[error("Invalid authentication token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("User not found")]
    UserNotFound,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InsufficientPermissions => (StatusCode::FORBIDDEN, self.to_string()),
            AuthError::InvalidApiKey => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, self.to_string()),
        };

        (
            status,
            serde_json::json!({
                "error": message,
                "code": status.as_u16()
            })
            .to_string(),
        )
            .into_response()
    }
}

/// User role in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Super administrator with all permissions
    SuperAdmin,
    /// Regular administrator
    Admin,
    /// Legal analyst with verification and analysis permissions
    Analyst,
    /// Read-only viewer
    Viewer,
    /// API client with programmatic access
    ApiClient,
}

impl Role {
    /// Returns the permissions granted by this role.
    pub fn permissions(&self) -> HashSet<Permission> {
        match self {
            Role::SuperAdmin => Permission::all(),
            Role::Admin => {
                let mut perms = Permission::all();
                perms.remove(&Permission::ManageUsers);
                perms.remove(&Permission::ManageApiKeys);
                perms
            }
            Role::Analyst => {
                let mut perms = HashSet::new();
                perms.insert(Permission::ReadStatutes);
                perms.insert(Permission::VerifyStatutes);
                perms.insert(Permission::RunSimulations);
                perms
            }
            Role::Viewer => {
                let mut perms = HashSet::new();
                perms.insert(Permission::ReadStatutes);
                perms
            }
            Role::ApiClient => {
                let mut perms = HashSet::new();
                perms.insert(Permission::ReadStatutes);
                perms.insert(Permission::CreateStatutes);
                perms.insert(Permission::VerifyStatutes);
                perms
            }
        }
    }

    /// Checks if this role has a specific permission.
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }
}

/// Granular permissions in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Read statutes
    ReadStatutes,
    /// Create new statutes
    CreateStatutes,
    /// Update existing statutes
    UpdateStatutes,
    /// Delete statutes
    DeleteStatutes,
    /// Run verifications
    VerifyStatutes,
    /// Run simulations
    RunSimulations,
    /// Access analytics
    ViewAnalytics,
    /// Manage users
    ManageUsers,
    /// Manage API keys
    ManageApiKeys,
}

impl Permission {
    /// Returns all permissions.
    pub fn all() -> HashSet<Permission> {
        let mut perms = HashSet::new();
        perms.insert(Permission::ReadStatutes);
        perms.insert(Permission::CreateStatutes);
        perms.insert(Permission::UpdateStatutes);
        perms.insert(Permission::DeleteStatutes);
        perms.insert(Permission::VerifyStatutes);
        perms.insert(Permission::RunSimulations);
        perms.insert(Permission::ViewAnalytics);
        perms.insert(Permission::ManageUsers);
        perms.insert(Permission::ManageApiKeys);
        perms
    }
}

/// Authenticated user extracted from request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    /// User ID
    pub id: Uuid,
    /// Username
    pub username: String,
    /// User role
    pub role: Role,
    /// Authentication method used
    pub auth_method: AuthMethod,
}

impl AuthUser {
    /// Creates a new authenticated user.
    pub fn new(id: Uuid, username: String, role: Role, auth_method: AuthMethod) -> Self {
        Self {
            id,
            username,
            role,
            auth_method,
        }
    }

    /// Checks if user has a specific permission.
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.role.has_permission(permission)
    }

    /// Requires that the user has a specific permission.
    pub fn require_permission(&self, permission: Permission) -> Result<(), AuthError> {
        if self.has_permission(permission) {
            Ok(())
        } else {
            Err(AuthError::InsufficientPermissions)
        }
    }
}

/// Authentication method used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethod {
    /// JWT bearer token
    Jwt,
    /// API key
    ApiKey,
}

/// JWT claims structure (simplified - in production use a proper JWT library).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (user ID)
    pub sub: Uuid,
    /// Username
    pub username: String,
    /// User role
    pub role: Role,
    /// Issued at timestamp (Unix epoch)
    pub iat: i64,
    /// Expiration timestamp (Unix epoch)
    pub exp: i64,
}

/// API key structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Key ID
    pub id: Uuid,
    /// The actual key value (hashed in storage)
    pub key: String,
    /// Key name/description
    pub name: String,
    /// Owner user ID
    pub owner_id: Uuid,
    /// Associated role
    pub role: Role,
    /// Created at timestamp
    pub created_at: i64,
    /// Last used timestamp
    pub last_used_at: Option<i64>,
    /// Whether the key is active
    pub active: bool,
}

impl ApiKey {
    /// Creates a new API key.
    pub fn new(name: String, owner_id: Uuid, role: Role) -> Self {
        Self {
            id: Uuid::new_v4(),
            key: format!("lgl_{}", Uuid::new_v4().simple()),
            name,
            owner_id,
            role,
            created_at: chrono::Utc::now().timestamp(),
            last_used_at: None,
            active: true,
        }
    }
}

/// Authentication extractor for Axum.
///
/// This allows endpoints to receive `AuthUser` as a parameter,
/// which will automatically validate authentication.
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract from Authorization header
        if let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) {
            let auth_str = auth_header.to_str().map_err(|_| AuthError::InvalidToken)?;

            // Check for Bearer token (JWT)
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return extract_jwt_user(token);
            }

            // Check for API key
            if let Some(key) = auth_str.strip_prefix("ApiKey ") {
                return extract_api_key_user(key);
            }

            return Err(AuthError::InvalidToken);
        }

        // Try to extract from X-API-Key header
        if let Some(api_key_header) = parts.headers.get("X-API-Key") {
            let key = api_key_header
                .to_str()
                .map_err(|_| AuthError::InvalidToken)?;
            return extract_api_key_user(key);
        }

        Err(AuthError::MissingCredentials)
    }
}

/// Extracts user from JWT token.
///
/// NOTE: This is a simplified implementation for demonstration.
/// In production, use a proper JWT library like `jsonwebtoken` with proper signature verification.
fn extract_jwt_user(token: &str) -> Result<AuthUser, AuthError> {
    // In a real implementation, this would:
    // 1. Verify the JWT signature with a secret key
    // 2. Check expiration
    // 3. Validate issuer and audience
    // 4. Look up user in database

    // For now, we'll simulate validation
    if token.is_empty() || token.len() < 10 {
        return Err(AuthError::InvalidToken);
    }

    // Simulate extracting claims (in production, use proper JWT decoding)
    // This is a placeholder - real implementation would decode and verify JWT
    Ok(AuthUser::new(
        Uuid::new_v4(),
        "jwt_user".to_string(),
        Role::Admin,
        AuthMethod::Jwt,
    ))
}

/// Extracts user from API key.
///
/// NOTE: This is a simplified implementation.
/// In production, this would query a database for the API key.
fn extract_api_key_user(key: &str) -> Result<AuthUser, AuthError> {
    // Validate key format
    if !key.starts_with("lgl_") {
        return Err(AuthError::InvalidApiKey);
    }

    // In production, this would:
    // 1. Hash the key
    // 2. Look up in database
    // 3. Check if active
    // 4. Update last_used_at
    // 5. Return associated user/role

    // For now, simulate validation
    if key.len() < 20 {
        return Err(AuthError::InvalidApiKey);
    }

    Ok(AuthUser::new(
        Uuid::new_v4(),
        "api_client".to_string(),
        Role::ApiClient,
        AuthMethod::ApiKey,
    ))
}

/// Middleware to require authentication.
///
/// This can be applied to routes to enforce authentication.
pub async fn require_auth(request: Request, next: Next) -> Result<Response, AuthError> {
    // Extract auth user from request
    let (mut parts, body) = request.into_parts();
    let _user = AuthUser::from_request_parts(&mut parts, &()).await?;

    // Reconstruct request and continue
    let request = Request::from_parts(parts, body);
    Ok(next.run(request).await)
}

/// Middleware to require a specific permission.
///
/// Returns a closure that can be used as Axum middleware.
pub fn require_permission(
    permission: Permission,
) -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, AuthError>> + Send>>
+ Clone {
    move |request: Request, next: Next| {
        let perm = permission;
        Box::pin(async move {
            let (mut parts, body) = request.into_parts();
            let user = AuthUser::from_request_parts(&mut parts, &()).await?;

            // Check permission
            user.require_permission(perm)?;

            // Continue
            let request = Request::from_parts(parts, body);
            Ok(next.run(request).await)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_permissions() {
        assert!(Role::SuperAdmin.has_permission(Permission::ManageUsers));
        assert!(Role::Admin.has_permission(Permission::CreateStatutes));
        assert!(!Role::Admin.has_permission(Permission::ManageUsers));
        assert!(Role::Analyst.has_permission(Permission::VerifyStatutes));
        assert!(!Role::Analyst.has_permission(Permission::DeleteStatutes));
        assert!(Role::Viewer.has_permission(Permission::ReadStatutes));
        assert!(!Role::Viewer.has_permission(Permission::CreateStatutes));
    }

    #[test]
    fn test_permission_all() {
        let all = Permission::all();
        assert_eq!(all.len(), 9);
        assert!(all.contains(&Permission::ReadStatutes));
        assert!(all.contains(&Permission::ManageUsers));
    }

    #[test]
    fn test_auth_user_permissions() {
        let user = AuthUser::new(
            Uuid::new_v4(),
            "analyst".to_string(),
            Role::Analyst,
            AuthMethod::Jwt,
        );

        assert!(user.has_permission(Permission::ReadStatutes));
        assert!(user.has_permission(Permission::VerifyStatutes));
        assert!(!user.has_permission(Permission::DeleteStatutes));

        assert!(user.require_permission(Permission::ReadStatutes).is_ok());
        assert!(user.require_permission(Permission::DeleteStatutes).is_err());
    }

    #[test]
    fn test_api_key_generation() {
        let key = ApiKey::new("Test Key".to_string(), Uuid::new_v4(), Role::ApiClient);

        assert!(key.key.starts_with("lgl_"));
        assert!(key.active);
        assert_eq!(key.name, "Test Key");
        assert!(key.last_used_at.is_none());
    }

    #[test]
    fn test_jwt_extraction() {
        // Valid token format (simplified)
        let result = extract_jwt_user("valid_jwt_token_placeholder");
        assert!(result.is_ok());

        // Invalid token
        let result = extract_jwt_user("");
        assert!(result.is_err());
    }

    #[test]
    fn test_api_key_extraction() {
        // Valid API key format
        let result = extract_api_key_user("lgl_12345678901234567890");
        assert!(result.is_ok());

        // Invalid format
        let result = extract_api_key_user("invalid_key");
        assert!(result.is_err());

        // Too short
        let result = extract_api_key_user("lgl_short");
        assert!(result.is_err());
    }

    #[test]
    fn test_role_hierarchy() {
        let super_admin_perms = Role::SuperAdmin.permissions();
        let admin_perms = Role::Admin.permissions();
        let analyst_perms = Role::Analyst.permissions();
        let viewer_perms = Role::Viewer.permissions();

        // SuperAdmin has more permissions than Admin
        assert!(super_admin_perms.len() > admin_perms.len());

        // Admin has more permissions than Analyst
        assert!(admin_perms.len() > analyst_perms.len());

        // Analyst has more permissions than Viewer
        assert!(analyst_perms.len() > viewer_perms.len());

        // Viewer permissions are subset of Analyst permissions
        assert!(viewer_perms.is_subset(&analyst_perms));
    }
}
