//! Single sign-on (SSO) integration: SAML 2.0 and OpenID-Connect.
//!
//! This module models the two tokens an identity provider (IdP) issues — a SAML
//! [`SamlAssertion`] and an OIDC [`OidcIdToken`] — and validates them on the
//! service-provider (SP) side. Validation is real, not a stub: tokens are signed
//! and verified with **HMAC-SHA256** (the HS256 JOSE algorithm) over a canonical
//! serialization, and the issuer, audience and validity window are all checked.
//!
//! The [`SsoProvider`] trait abstracts the IdP. [`InMemoryIdentityProvider`]
//! plays both roles for testing — it can *issue* tokens for registered users and
//! *validate* tokens as an SP — while a production deployment can implement
//! `SsoProvider` against a live IdP (SAML redirect/POST binding, OIDC discovery
//! with JWKS, RSA verification) without changing callers. That networked binding
//! is intentionally deferred.
//!
//! # Example
//!
//! ```
//! use chrono::{Duration, Utc};
//! use legalis_diff::governance::sso::{InMemoryIdentityProvider, SsoProvider, SsoToken};
//!
//! let mut idp = InMemoryIdentityProvider::new("https://idp.example", "legalis-diff", b"shared-secret");
//! idp.register_user("alice", "Alice Counsel", Some("alice@example.com"), ["legal-editors"]);
//!
//! let now = Utc::now();
//! let token = idp.issue_oidc("alice", Duration::hours(1), now).unwrap();
//! let session = idp.validate(&SsoToken::Oidc(token), now).unwrap();
//! assert_eq!(session.principal.subject, "alice");
//! assert!(session.principal.is_in_group("legal-editors"));
//! ```

use crate::governance::{Principal, sha256_hex};
use crate::{DiffError, DiffResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// The SSO protocol a token was issued under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SsoProtocol {
    /// SAML 2.0 assertion.
    Saml2,
    /// OpenID Connect ID token.
    Oidc,
}

impl std::fmt::Display for SsoProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Saml2 => f.write_str("SAML2"),
            Self::Oidc => f.write_str("OIDC"),
        }
    }
}

/// HMAC-SHA256 (RFC 2104) over `message` with `key`, returned as raw bytes.
///
/// Implemented directly on top of the workspace `sha2` dependency so no extra
/// crate is required; this is the same construction JOSE calls `HS256`.
fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    const BLOCK: usize = 64;
    let mut k = [0u8; BLOCK];
    if key.len() > BLOCK {
        let digest = Sha256::digest(key);
        k[..32].copy_from_slice(&digest);
    } else {
        k[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; BLOCK];
    let mut opad = [0x5cu8; BLOCK];
    for ((kb, ib), ob) in k.iter().zip(ipad.iter_mut()).zip(opad.iter_mut()) {
        *ib ^= *kb;
        *ob ^= *kb;
    }
    let mut inner = Sha256::new();
    inner.update(ipad);
    inner.update(message);
    let inner_digest = inner.finalize();

    let mut outer = Sha256::new();
    outer.update(opad);
    outer.update(inner_digest);
    let mut out = [0u8; 32];
    out.copy_from_slice(&outer.finalize());
    out
}

/// HMAC-SHA256 hex digest helper.
fn sign_hex(secret: &[u8], canonical: &str) -> String {
    hex::encode(hmac_sha256(secret, canonical.as_bytes()))
}

/// Serializes an ordered attribute map into a canonical, unambiguous string.
fn canonical_attrs(attrs: &BTreeMap<String, Vec<String>>) -> String {
    let mut out = String::new();
    for (key, values) in attrs {
        out.push_str(key);
        out.push('=');
        // Values are already in insertion order; sort a copy for stability.
        let mut sorted = values.clone();
        sorted.sort();
        out.push_str(&sorted.join("\u{1f}"));
        out.push('\u{1e}');
    }
    out
}

/// A SAML 2.0 assertion (the relevant subset for authentication + attributes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SamlAssertion {
    /// The asserting IdP (`Issuer`).
    pub issuer: String,
    /// The authenticated subject (`NameID`).
    pub subject: String,
    /// The intended SP (`AudienceRestriction`).
    pub audience: String,
    /// `Conditions/@NotBefore`.
    pub not_before: DateTime<Utc>,
    /// `Conditions/@NotOnOrAfter`.
    pub not_on_or_after: DateTime<Utc>,
    /// Opaque session index linking the assertion to the IdP session.
    pub session_index: String,
    /// Multi-valued attribute statements (e.g. `groups`, `mail`, `displayName`).
    pub attributes: BTreeMap<String, Vec<String>>,
    /// HMAC-SHA256 signature (hex) over the canonical content.
    pub signature: String,
}

impl SamlAssertion {
    fn canonical(&self) -> String {
        format!(
            "saml2|{}|{}|{}|{}|{}|{}|{}",
            self.issuer,
            self.subject,
            self.audience,
            self.not_before.timestamp(),
            self.not_on_or_after.timestamp(),
            self.session_index,
            canonical_attrs(&self.attributes),
        )
    }
}

/// An OpenID-Connect ID token (the standard claim subset).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcIdToken {
    /// `iss` — issuer identifier.
    pub iss: String,
    /// `sub` — subject identifier.
    pub sub: String,
    /// `aud` — audience (the client/SP id).
    pub aud: String,
    /// `exp` — expiry.
    pub exp: DateTime<Utc>,
    /// `iat` — issued-at.
    pub iat: DateTime<Utc>,
    /// `nbf` — not-before (optional).
    pub nbf: Option<DateTime<Utc>>,
    /// `name` claim.
    pub name: Option<String>,
    /// `email` claim.
    pub email: Option<String>,
    /// `groups` claim.
    pub groups: Vec<String>,
    /// Additional string claims.
    pub extra: BTreeMap<String, String>,
    /// HMAC-SHA256 signature (hex) over the canonical content.
    pub signature: String,
}

impl OidcIdToken {
    fn canonical(&self) -> String {
        let mut extra = String::new();
        for (k, v) in &self.extra {
            extra.push_str(k);
            extra.push('=');
            extra.push_str(v);
            extra.push('\u{1e}');
        }
        let mut groups = self.groups.clone();
        groups.sort();
        format!(
            "oidc|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.iss,
            self.sub,
            self.aud,
            self.exp.timestamp(),
            self.iat.timestamp(),
            self.nbf.map(|d| d.timestamp()).unwrap_or(0),
            self.name.clone().unwrap_or_default(),
            self.email.clone().unwrap_or_default(),
            groups.join("\u{1f}"),
            extra,
        )
    }
}

/// A token presented by a relying party for validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SsoToken {
    /// A SAML 2.0 assertion.
    Saml(SamlAssertion),
    /// An OIDC ID token.
    Oidc(OidcIdToken),
}

impl SsoToken {
    /// The protocol of this token.
    pub fn protocol(&self) -> SsoProtocol {
        match self {
            Self::Saml(_) => SsoProtocol::Saml2,
            Self::Oidc(_) => SsoProtocol::Oidc,
        }
    }
}

/// An authenticated SSO session created after successful token validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SsoSession {
    /// Deterministic session identifier (hash of the validated token).
    pub session_id: String,
    /// The protocol the originating token used.
    pub protocol: SsoProtocol,
    /// The authenticated principal.
    pub principal: Principal,
    /// When the session was established.
    pub established_at: DateTime<Utc>,
    /// When the session expires (the token's expiry).
    pub expires_at: DateTime<Utc>,
}

impl SsoSession {
    /// Returns `true` if the session is still active at `now`.
    pub fn is_active(&self, now: DateTime<Utc>) -> bool {
        now < self.expires_at
    }
}

/// Abstraction over an identity provider that validates SSO tokens.
///
/// The pure-Rust [`InMemoryIdentityProvider`] implements this with HMAC-SHA256;
/// a production binding (live SAML/OIDC IdP) can implement it without changing
/// callers.
pub trait SsoProvider {
    /// Validates `token` at time `now`, returning the established session.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::AuthenticationFailed`] if the signature, issuer,
    /// audience or validity window check fails.
    fn validate(&self, token: &SsoToken, now: DateTime<Utc>) -> DiffResult<SsoSession>;

    /// The issuer identifier this provider trusts.
    fn issuer(&self) -> &str;
}

/// A registered IdP user (the IdP's view of an account).
#[derive(Debug, Clone)]
struct IdpUser {
    display_name: String,
    email: Option<String>,
    groups: Vec<String>,
    attributes: BTreeMap<String, String>,
}

/// A pure-Rust in-memory identity provider.
///
/// Acts as both IdP (it can [`issue_saml`](Self::issue_saml) /
/// [`issue_oidc`](Self::issue_oidc) tokens for registered users) and SP (it
/// implements [`SsoProvider::validate`]). Tokens are signed/verified with a
/// shared secret using HMAC-SHA256.
#[derive(Debug, Clone)]
pub struct InMemoryIdentityProvider {
    issuer: String,
    audience: String,
    secret: Vec<u8>,
    clock_skew_seconds: i64,
    users: BTreeMap<String, IdpUser>,
}

impl InMemoryIdentityProvider {
    /// Creates a provider for the given issuer/audience pair and shared secret.
    pub fn new(issuer: impl Into<String>, audience: impl Into<String>, secret: &[u8]) -> Self {
        Self {
            issuer: issuer.into(),
            audience: audience.into(),
            secret: secret.to_vec(),
            clock_skew_seconds: 60,
            users: BTreeMap::new(),
        }
    }

    /// Sets the tolerated clock skew (seconds) for validity-window checks.
    #[must_use]
    pub fn with_clock_skew_seconds(mut self, seconds: i64) -> Self {
        self.clock_skew_seconds = seconds.max(0);
        self
    }

    /// Registers (or replaces) a user known to the IdP.
    pub fn register_user<I, S>(
        &mut self,
        subject: impl Into<String>,
        display_name: impl Into<String>,
        email: Option<&str>,
        groups: I,
    ) where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.users.insert(
            subject.into(),
            IdpUser {
                display_name: display_name.into(),
                email: email.map(str::to_string),
                groups: groups.into_iter().map(Into::into).collect(),
                attributes: BTreeMap::new(),
            },
        );
    }

    /// Adds an attribute to a registered user (no-op if unknown).
    pub fn set_user_attribute(
        &mut self,
        subject: &str,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        if let Some(user) = self.users.get_mut(subject) {
            user.attributes.insert(key.into(), value.into());
        }
    }

    /// Issues a signed SAML assertion for a registered user.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::AuthenticationFailed`] if the subject is unknown.
    pub fn issue_saml(
        &self,
        subject: &str,
        validity: Duration,
        now: DateTime<Utc>,
    ) -> DiffResult<SamlAssertion> {
        let user = self.user(subject)?;
        let mut attributes: BTreeMap<String, Vec<String>> = BTreeMap::new();
        if !user.groups.is_empty() {
            attributes.insert("groups".to_string(), user.groups.clone());
        }
        if let Some(email) = &user.email {
            attributes.insert("mail".to_string(), vec![email.clone()]);
        }
        attributes.insert("displayName".to_string(), vec![user.display_name.clone()]);
        for (k, v) in &user.attributes {
            attributes.insert(k.clone(), vec![v.clone()]);
        }
        let mut assertion = SamlAssertion {
            issuer: self.issuer.clone(),
            subject: subject.to_string(),
            audience: self.audience.clone(),
            not_before: now,
            not_on_or_after: now + validity,
            session_index: sha256_hex(format!("{subject}:{}", now.timestamp()).as_bytes()),
            attributes,
            signature: String::new(),
        };
        assertion.signature = sign_hex(&self.secret, &assertion.canonical());
        Ok(assertion)
    }

    /// Issues a signed OIDC ID token for a registered user.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::AuthenticationFailed`] if the subject is unknown.
    pub fn issue_oidc(
        &self,
        subject: &str,
        validity: Duration,
        now: DateTime<Utc>,
    ) -> DiffResult<OidcIdToken> {
        let user = self.user(subject)?;
        let mut token = OidcIdToken {
            iss: self.issuer.clone(),
            sub: subject.to_string(),
            aud: self.audience.clone(),
            exp: now + validity,
            iat: now,
            nbf: Some(now),
            name: Some(user.display_name.clone()),
            email: user.email.clone(),
            groups: user.groups.clone(),
            extra: user.attributes.clone(),
            signature: String::new(),
        };
        token.signature = sign_hex(&self.secret, &token.canonical());
        Ok(token)
    }

    fn user(&self, subject: &str) -> DiffResult<&IdpUser> {
        self.users
            .get(subject)
            .ok_or_else(|| DiffError::AuthenticationFailed(format!("unknown subject '{subject}'")))
    }

    fn skew(&self) -> Duration {
        Duration::seconds(self.clock_skew_seconds)
    }

    fn check_window(
        &self,
        not_before: DateTime<Utc>,
        not_after: DateTime<Utc>,
        now: DateTime<Utc>,
    ) -> DiffResult<()> {
        let skew = self.skew();
        if now + skew < not_before {
            return Err(DiffError::AuthenticationFailed(
                "token is not yet valid (not_before in the future)".to_string(),
            ));
        }
        if now - skew >= not_after {
            return Err(DiffError::AuthenticationFailed(
                "token has expired".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_saml(&self, a: &SamlAssertion, now: DateTime<Utc>) -> DiffResult<SsoSession> {
        if a.issuer != self.issuer {
            return Err(DiffError::AuthenticationFailed(format!(
                "unexpected issuer '{}' (expected '{}')",
                a.issuer, self.issuer
            )));
        }
        if a.audience != self.audience {
            return Err(DiffError::AuthenticationFailed(format!(
                "audience '{}' does not match '{}'",
                a.audience, self.audience
            )));
        }
        let expected = sign_hex(&self.secret, &a.canonical());
        if expected != a.signature {
            return Err(DiffError::AuthenticationFailed(
                "SAML signature verification failed".to_string(),
            ));
        }
        self.check_window(a.not_before, a.not_on_or_after, now)?;

        let mut principal = Principal::new(&a.subject);
        if let Some(names) = a.attributes.get("displayName").and_then(|v| v.first()) {
            principal.display_name = names.clone();
        }
        if let Some(mail) = a.attributes.get("mail").and_then(|v| v.first()) {
            principal.email = Some(mail.clone());
        }
        if let Some(groups) = a.attributes.get("groups") {
            principal.groups = groups.clone();
        }
        for (k, v) in &a.attributes {
            if k == "displayName" || k == "mail" || k == "groups" {
                continue;
            }
            if let Some(first) = v.first() {
                principal.attributes.insert(k.clone(), first.clone());
            }
        }
        Ok(SsoSession {
            session_id: sha256_hex(format!("saml:{}", a.canonical()).as_bytes()),
            protocol: SsoProtocol::Saml2,
            principal,
            established_at: now,
            expires_at: a.not_on_or_after,
        })
    }

    fn validate_oidc(&self, t: &OidcIdToken, now: DateTime<Utc>) -> DiffResult<SsoSession> {
        if t.iss != self.issuer {
            return Err(DiffError::AuthenticationFailed(format!(
                "unexpected issuer '{}' (expected '{}')",
                t.iss, self.issuer
            )));
        }
        if t.aud != self.audience {
            return Err(DiffError::AuthenticationFailed(format!(
                "audience '{}' does not match '{}'",
                t.aud, self.audience
            )));
        }
        let expected = sign_hex(&self.secret, &t.canonical());
        if expected != t.signature {
            return Err(DiffError::AuthenticationFailed(
                "OIDC signature verification failed".to_string(),
            ));
        }
        let not_before = t.nbf.unwrap_or(t.iat);
        self.check_window(not_before, t.exp, now)?;

        let mut principal = Principal::new(&t.sub);
        if let Some(name) = &t.name {
            principal.display_name = name.clone();
        }
        principal.email = t.email.clone();
        principal.groups = t.groups.clone();
        principal.attributes = t.extra.clone();
        Ok(SsoSession {
            session_id: sha256_hex(format!("oidc:{}", t.canonical()).as_bytes()),
            protocol: SsoProtocol::Oidc,
            principal,
            established_at: now,
            expires_at: t.exp,
        })
    }
}

impl SsoProvider for InMemoryIdentityProvider {
    fn validate(&self, token: &SsoToken, now: DateTime<Utc>) -> DiffResult<SsoSession> {
        match token {
            SsoToken::Saml(a) => self.validate_saml(a, now),
            SsoToken::Oidc(t) => self.validate_oidc(t, now),
        }
    }

    fn issuer(&self) -> &str {
        &self.issuer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provider() -> InMemoryIdentityProvider {
        let mut idp =
            InMemoryIdentityProvider::new("https://idp.example", "legalis-diff", b"top-secret-key");
        idp.register_user(
            "alice",
            "Alice Counsel",
            Some("alice@example.com"),
            ["legal-editors", "reviewers"],
        );
        idp.set_user_attribute("alice", "department", "legal");
        idp
    }

    #[test]
    fn test_hmac_sha256_known_vector() {
        // RFC 4231 test case 1: key = 0x0b*20, data = "Hi There".
        let key = [0x0bu8; 20];
        let mac = hmac_sha256(&key, b"Hi There");
        assert_eq!(
            hex::encode(mac),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
    }

    #[test]
    fn test_oidc_issue_and_validate() {
        let idp = provider();
        let now = Utc::now();
        let token = idp.issue_oidc("alice", Duration::hours(1), now).unwrap();
        let session = idp.validate(&SsoToken::Oidc(token), now).unwrap();
        assert_eq!(session.protocol, SsoProtocol::Oidc);
        assert_eq!(session.principal.subject, "alice");
        assert_eq!(session.principal.display_name, "Alice Counsel");
        assert!(session.principal.is_in_group("legal-editors"));
        assert_eq!(
            session
                .principal
                .attributes
                .get("department")
                .map(String::as_str),
            Some("legal")
        );
        assert!(session.is_active(now));
    }

    #[test]
    fn test_saml_issue_and_validate() {
        let idp = provider();
        let now = Utc::now();
        let assertion = idp.issue_saml("alice", Duration::hours(2), now).unwrap();
        let session = idp.validate(&SsoToken::Saml(assertion), now).unwrap();
        assert_eq!(session.protocol, SsoProtocol::Saml2);
        assert_eq!(
            session.principal.email.as_deref(),
            Some("alice@example.com")
        );
        assert!(session.principal.is_in_group("reviewers"));
    }

    #[test]
    fn test_expired_token_rejected() {
        let idp = provider();
        let now = Utc::now();
        let token = idp.issue_oidc("alice", Duration::minutes(5), now).unwrap();
        let later = now + Duration::hours(1);
        let result = idp.validate(&SsoToken::Oidc(token), later);
        assert!(matches!(result, Err(DiffError::AuthenticationFailed(_))));
    }

    #[test]
    fn test_tampered_signature_rejected() {
        let idp = provider();
        let now = Utc::now();
        let mut token = idp.issue_oidc("alice", Duration::hours(1), now).unwrap();
        token.email = Some("attacker@evil.example".to_string()); // tamper after signing
        let result = idp.validate(&SsoToken::Oidc(token), now);
        assert!(matches!(result, Err(DiffError::AuthenticationFailed(_))));
    }

    #[test]
    fn test_wrong_audience_rejected() {
        let idp = provider();
        let other = InMemoryIdentityProvider::new(
            "https://idp.example",
            "different-app",
            b"top-secret-key",
        );
        let now = Utc::now();
        let token = idp.issue_oidc("alice", Duration::hours(1), now).unwrap();
        let result = other.validate(&SsoToken::Oidc(token), now);
        assert!(matches!(result, Err(DiffError::AuthenticationFailed(_))));
    }

    #[test]
    fn test_wrong_secret_rejected() {
        let idp = provider();
        let now = Utc::now();
        let token = idp.issue_oidc("alice", Duration::hours(1), now).unwrap();
        let imposter =
            InMemoryIdentityProvider::new("https://idp.example", "legalis-diff", b"other-secret");
        let result = imposter.validate(&SsoToken::Oidc(token), now);
        assert!(matches!(result, Err(DiffError::AuthenticationFailed(_))));
    }

    #[test]
    fn test_unknown_subject_cannot_issue() {
        let idp = provider();
        let result = idp.issue_oidc("mallory", Duration::hours(1), Utc::now());
        assert!(matches!(result, Err(DiffError::AuthenticationFailed(_))));
    }

    #[test]
    fn test_clock_skew_tolerance() {
        let idp = provider().with_clock_skew_seconds(120);
        let now = Utc::now();
        let token = idp.issue_oidc("alice", Duration::minutes(1), now).unwrap();
        // 90s after a 60s token: within the 120s skew tolerance.
        let slightly_late = now + Duration::seconds(90);
        assert!(idp.validate(&SsoToken::Oidc(token), slightly_late).is_ok());
    }

    #[test]
    fn test_token_protocol_accessor() {
        let idp = provider();
        let now = Utc::now();
        let oidc = SsoToken::Oidc(idp.issue_oidc("alice", Duration::hours(1), now).unwrap());
        let saml = SsoToken::Saml(idp.issue_saml("alice", Duration::hours(1), now).unwrap());
        assert_eq!(oidc.protocol(), SsoProtocol::Oidc);
        assert_eq!(saml.protocol(), SsoProtocol::Saml2);
        assert_eq!(idp.issuer(), "https://idp.example");
    }
}
