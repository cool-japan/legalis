//! OAuth2/OIDC authentication provider support.
//!
//! This module provides OAuth2 authentication with support for multiple providers:
//! - Keycloak
//! - Auth0
//! - Okta
//! - Google
//! - GitHub
//! - Generic OIDC providers

#[cfg(feature = "oauth2-auth")]
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "oauth2-auth")]
use std::collections::HashMap;
#[cfg(feature = "oauth2-auth")]
use std::sync::Arc;
#[cfg(feature = "oauth2-auth")]
use tokio::sync::RwLock;

/// OAuth2 provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    /// OAuth2 provider type
    pub provider: OAuth2Provider,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Authorization endpoint URL
    pub auth_url: String,
    /// Token endpoint URL
    pub token_url: String,
    /// Redirect URL (callback)
    pub redirect_url: String,
    /// Scopes to request
    pub scopes: Vec<String>,
    /// UserInfo endpoint URL (for OIDC)
    pub userinfo_url: Option<String>,
    /// Enable PKCE (Proof Key for Code Exchange)
    pub use_pkce: bool,
}

/// Supported OAuth2 providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuth2Provider {
    /// Keycloak identity provider
    Keycloak,
    /// Auth0 identity provider
    Auth0,
    /// Okta identity provider
    Okta,
    /// Google OAuth2
    Google,
    /// GitHub OAuth2
    GitHub,
    /// Generic OIDC provider
    Generic,
}

impl OAuth2Config {
    /// Create a configuration for Keycloak.
    pub fn keycloak(
        realm_url: &str,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Self {
        Self {
            provider: OAuth2Provider::Keycloak,
            client_id,
            client_secret,
            auth_url: format!("{}/protocol/openid-connect/auth", realm_url),
            token_url: format!("{}/protocol/openid-connect/token", realm_url),
            userinfo_url: Some(format!("{}/protocol/openid-connect/userinfo", realm_url)),
            redirect_url,
            scopes: vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ],
            use_pkce: true,
        }
    }

    /// Create a configuration for Auth0.
    pub fn auth0(
        domain: &str,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Self {
        Self {
            provider: OAuth2Provider::Auth0,
            client_id,
            client_secret,
            auth_url: format!("https://{}/authorize", domain),
            token_url: format!("https://{}/oauth/token", domain),
            userinfo_url: Some(format!("https://{}/userinfo", domain)),
            redirect_url,
            scopes: vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ],
            use_pkce: true,
        }
    }

    /// Create a configuration for Okta.
    pub fn okta(
        domain: &str,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Self {
        Self {
            provider: OAuth2Provider::Okta,
            client_id,
            client_secret,
            auth_url: format!("https://{}/oauth2/v1/authorize", domain),
            token_url: format!("https://{}/oauth2/v1/token", domain),
            userinfo_url: Some(format!("https://{}/oauth2/v1/userinfo", domain)),
            redirect_url,
            scopes: vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ],
            use_pkce: true,
        }
    }

    /// Create a configuration for Google.
    pub fn google(client_id: String, client_secret: String, redirect_url: String) -> Self {
        Self {
            provider: OAuth2Provider::Google,
            client_id,
            client_secret,
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            userinfo_url: Some("https://www.googleapis.com/oauth2/v3/userinfo".to_string()),
            redirect_url,
            scopes: vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ],
            use_pkce: true,
        }
    }

    /// Create a configuration for GitHub.
    pub fn github(client_id: String, client_secret: String, redirect_url: String) -> Self {
        Self {
            provider: OAuth2Provider::GitHub,
            client_id,
            client_secret,
            auth_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            userinfo_url: Some("https://api.github.com/user".to_string()),
            redirect_url,
            scopes: vec!["user:email".to_string()],
            use_pkce: false, // GitHub doesn't support PKCE
        }
    }
}

/// OAuth2 user information from provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2UserInfo {
    /// User ID from provider
    pub sub: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Display name
    pub name: Option<String>,
    /// Preferred username
    pub preferred_username: Option<String>,
    /// Profile picture URL
    pub picture: Option<String>,
    /// Email verified flag
    pub email_verified: Option<bool>,
}

/// OAuth2 authorization state (stored during auth flow).
#[derive(Debug, Clone)]
pub struct AuthState {
    /// CSRF token for security
    pub csrf_token: String,
    /// PKCE verifier (if PKCE is enabled)
    pub pkce_verifier: Option<String>,
    /// Original request URI to redirect back to
    pub return_url: Option<String>,
}

/// OAuth2 authentication manager.
#[cfg(feature = "oauth2-auth")]
pub struct OAuth2Manager {
    /// Configuration
    config: OAuth2Config,
    /// Parsed URLs for client creation
    auth_url: AuthUrl,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
    /// Pending authorization states (keyed by CSRF token)
    auth_states: Arc<RwLock<HashMap<String, AuthState>>>,
}

#[cfg(feature = "oauth2-auth")]
impl OAuth2Manager {
    /// Create a new OAuth2 manager.
    pub fn new(config: OAuth2Config) -> Result<Self, String> {
        let auth_url = AuthUrl::new(config.auth_url.clone())
            .map_err(|e| format!("Invalid auth URL: {}", e))?;
        let token_url = TokenUrl::new(config.token_url.clone())
            .map_err(|e| format!("Invalid token URL: {}", e))?;
        let redirect_url = RedirectUrl::new(config.redirect_url.clone())
            .map_err(|e| format!("Invalid redirect URL: {}", e))?;

        Ok(Self {
            config,
            auth_url,
            token_url,
            redirect_url,
            auth_states: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Generate authorization URL for the OAuth2 flow.
    pub async fn authorize_url(&self, return_url: Option<String>) -> (String, String) {
        let client = BasicClient::new(ClientId::new(self.config.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.config.client_secret.clone()))
            .set_auth_uri(self.auth_url.clone())
            .set_token_uri(self.token_url.clone())
            .set_redirect_uri(self.redirect_url.clone());

        let mut auth_request = client.authorize_url(CsrfToken::new_random);

        // Add scopes
        for scope in &self.config.scopes {
            auth_request = auth_request.add_scope(Scope::new(scope.clone()));
        }

        let (pkce_challenge, pkce_verifier) = if self.config.use_pkce {
            let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
            (Some(challenge), Some(verifier))
        } else {
            (None, None)
        };

        if let Some(challenge) = pkce_challenge {
            auth_request = auth_request.set_pkce_challenge(challenge);
        }

        let (url, csrf_token) = auth_request.url();

        // Store auth state
        let state = AuthState {
            csrf_token: csrf_token.secret().clone(),
            pkce_verifier: pkce_verifier.map(|v| v.secret().clone()),
            return_url,
        };

        self.auth_states
            .write()
            .await
            .insert(csrf_token.secret().clone(), state);

        (url.to_string(), csrf_token.secret().clone())
    }

    /// Exchange authorization code for access token.
    pub async fn exchange_code(
        &self,
        code: String,
        csrf_token: String,
    ) -> Result<(String, OAuth2UserInfo), String> {
        // Retrieve and remove auth state
        let auth_state = self
            .auth_states
            .write()
            .await
            .remove(&csrf_token)
            .ok_or("Invalid or expired CSRF token")?;

        // Verify CSRF token matches
        if auth_state.csrf_token != csrf_token {
            return Err("CSRF token mismatch".to_string());
        }

        // Create client for token exchange
        let client = BasicClient::new(ClientId::new(self.config.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.config.client_secret.clone()))
            .set_auth_uri(self.auth_url.clone())
            .set_token_uri(self.token_url.clone())
            .set_redirect_uri(self.redirect_url.clone());

        // Exchange code for token
        let mut token_request = client.exchange_code(AuthorizationCode::new(code));

        if let Some(verifier) = auth_state.pkce_verifier {
            token_request = token_request.set_pkce_verifier(PkceCodeVerifier::new(verifier));
        }

        let http_client = oauth2::reqwest::Client::new();
        let token_result = token_request
            .request_async(&http_client)
            .await
            .map_err(|e| format!("Token exchange failed: {}", e))?;

        let access_token = token_result.access_token().secret().clone();

        // Fetch user info
        let user_info = self.fetch_user_info(&access_token).await?;

        Ok((access_token, user_info))
    }

    /// Fetch user information from the provider.
    async fn fetch_user_info(&self, access_token: &str) -> Result<OAuth2UserInfo, String> {
        let userinfo_url = self
            .config
            .userinfo_url
            .as_ref()
            .ok_or("UserInfo URL not configured")?;

        let client = reqwest::Client::new();
        let response = client
            .get(userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch user info: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("UserInfo request failed: {}", response.status()));
        }

        let user_info: OAuth2UserInfo = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse user info: {}", e))?;

        Ok(user_info)
    }

    /// Clean up expired auth states (older than 10 minutes).
    pub async fn cleanup_expired_states(&self) {
        // In a production system, you'd track timestamps and remove old entries
        // For now, this is a placeholder
        let mut states = self.auth_states.write().await;
        states.clear();
    }
}

/// Stub OAuth2Manager for when the feature is disabled.
#[cfg(not(feature = "oauth2-auth"))]
pub struct OAuth2Manager;

#[cfg(not(feature = "oauth2-auth"))]
impl OAuth2Manager {
    pub fn new(_config: OAuth2Config) -> Result<Self, String> {
        Err("OAuth2 support is not enabled. Enable the 'oauth2-auth' feature.".to_string())
    }
}
