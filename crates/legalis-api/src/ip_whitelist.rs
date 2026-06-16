//! IP whitelisting (allowlist) configuration and middleware.
//!
//! Provides an allowlist of permitted client IP addresses / CIDR ranges and an
//! Axum middleware that rejects requests originating from any address outside
//! the allowlist with `403 Forbidden`.
//!
//! Supports both IPv4 and IPv6, single addresses and CIDR network ranges. The
//! client address is determined from the `ConnectInfo<SocketAddr>` extension
//! when present, with an optional fallback to trusted forwarding headers
//! (`X-Forwarded-For` / `X-Real-IP`) for deployments behind a reverse proxy.

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;

/// A single allowlist rule: either an exact address or a CIDR network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpRule {
    /// An exact IP address.
    Exact(IpAddr),
    /// A CIDR network: base address plus prefix length in bits.
    Cidr { base: IpAddr, prefix_len: u8 },
}

impl IpRule {
    /// Parses a rule from a textual representation.
    ///
    /// Accepts either a bare IP address (`"10.0.0.1"`, `"::1"`) or CIDR notation
    /// (`"10.0.0.0/8"`, `"2001:db8::/32"`).
    pub fn parse(spec: &str) -> Result<Self, IpWhitelistError> {
        let spec = spec.trim();
        if let Some((addr_part, prefix_part)) = spec.split_once('/') {
            let base: IpAddr = addr_part
                .parse()
                .map_err(|_| IpWhitelistError::InvalidRule(spec.to_string()))?;
            let prefix_len: u8 = prefix_part
                .parse()
                .map_err(|_| IpWhitelistError::InvalidRule(spec.to_string()))?;
            let max = match base {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max {
                return Err(IpWhitelistError::InvalidRule(spec.to_string()));
            }
            Ok(IpRule::Cidr { base, prefix_len })
        } else {
            let addr: IpAddr = spec
                .parse()
                .map_err(|_| IpWhitelistError::InvalidRule(spec.to_string()))?;
            Ok(IpRule::Exact(addr))
        }
    }

    /// Returns whether `addr` matches this rule.
    pub fn matches(&self, addr: IpAddr) -> bool {
        match self {
            IpRule::Exact(exact) => *exact == addr,
            IpRule::Cidr { base, prefix_len } => cidr_contains(*base, *prefix_len, addr),
        }
    }
}

/// Returns whether `addr` falls inside the CIDR network `base/prefix_len`.
///
/// Returns `false` if the address families differ.
fn cidr_contains(base: IpAddr, prefix_len: u8, addr: IpAddr) -> bool {
    match (base, addr) {
        (IpAddr::V4(base), IpAddr::V4(addr)) => ipv4_prefix_match(base, addr, prefix_len),
        (IpAddr::V6(base), IpAddr::V6(addr)) => ipv6_prefix_match(base, addr, prefix_len),
        // Mixed families never match.
        _ => false,
    }
}

fn ipv4_prefix_match(base: Ipv4Addr, addr: Ipv4Addr, prefix_len: u8) -> bool {
    if prefix_len == 0 {
        return true;
    }
    let prefix_len = prefix_len.min(32);
    let mask: u32 = if prefix_len == 32 {
        u32::MAX
    } else {
        !(u32::MAX >> prefix_len)
    };
    (u32::from(base) & mask) == (u32::from(addr) & mask)
}

fn ipv6_prefix_match(base: Ipv6Addr, addr: Ipv6Addr, prefix_len: u8) -> bool {
    if prefix_len == 0 {
        return true;
    }
    let prefix_len = prefix_len.min(128);
    let mask: u128 = if prefix_len == 128 {
        u128::MAX
    } else {
        !(u128::MAX >> prefix_len)
    };
    (u128::from(base) & mask) == (u128::from(addr) & mask)
}

/// Errors produced while building or evaluating the IP allowlist.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IpWhitelistError {
    /// A rule could not be parsed.
    #[error("invalid IP rule: {0}")]
    InvalidRule(String),
}

/// An IP allowlist made up of one or more rules.
#[derive(Debug, Clone, Default)]
pub struct IpWhitelist {
    rules: Vec<IpRule>,
    /// When true, trust `X-Forwarded-For` / `X-Real-IP` for the client address.
    trust_forwarded_headers: bool,
}

impl IpWhitelist {
    /// Creates an empty allowlist.
    ///
    /// An empty allowlist denies all addresses; populate it with [`Self::allow`]
    /// or build one from specs via [`Self::from_specs`].
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            trust_forwarded_headers: false,
        }
    }

    /// Builds an allowlist from a list of textual rules (IPs or CIDRs).
    pub fn from_specs<I, S>(specs: I) -> Result<Self, IpWhitelistError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut wl = Self::new();
        for spec in specs {
            wl.rules.push(IpRule::parse(spec.as_ref())?);
        }
        Ok(wl)
    }

    /// Adds a parsed rule.
    pub fn allow(&mut self, rule: IpRule) -> &mut Self {
        self.rules.push(rule);
        self
    }

    /// Adds a rule from a textual spec.
    pub fn allow_spec(&mut self, spec: &str) -> Result<&mut Self, IpWhitelistError> {
        self.rules.push(IpRule::parse(spec)?);
        Ok(self)
    }

    /// Enables trusting forwarding headers for client address resolution.
    pub fn trust_forwarded(mut self, trust: bool) -> Self {
        self.trust_forwarded_headers = trust;
        self
    }

    /// Returns whether forwarding headers are trusted.
    pub fn trusts_forwarded(&self) -> bool {
        self.trust_forwarded_headers
    }

    /// Returns the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Returns whether the allowlist has no rules.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Returns whether `addr` is permitted by any rule.
    pub fn is_allowed(&self, addr: IpAddr) -> bool {
        self.rules.iter().any(|r| r.matches(addr))
    }
}

/// Extracts the client IP from a request, honoring forwarding headers if the
/// allowlist trusts them.
fn extract_client_ip(req: &Request<Body>, trust_forwarded: bool) -> Option<IpAddr> {
    if trust_forwarded {
        // X-Forwarded-For: client, proxy1, proxy2 -> take the first entry.
        let xff = req
            .headers()
            .get("X-Forwarded-For")
            .and_then(|v| v.to_str().ok())
            .and_then(|value| value.split(',').next())
            .and_then(|first| first.trim().parse::<IpAddr>().ok());
        if let Some(ip) = xff {
            return Some(ip);
        }
        let real_ip = req
            .headers()
            .get("X-Real-IP")
            .and_then(|v| v.to_str().ok())
            .and_then(|value| value.trim().parse::<IpAddr>().ok());
        if let Some(ip) = real_ip {
            return Some(ip);
        }
    }
    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0.ip())
}

/// Axum middleware enforcing the IP allowlist.
///
/// Requests from addresses not in the allowlist receive `403 Forbidden`. If the
/// client address cannot be determined, the request is also rejected (fail
/// closed).
pub async fn ip_whitelist_middleware(
    whitelist: Arc<IpWhitelist>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let client_ip = extract_client_ip(&req, whitelist.trusts_forwarded());
    match client_ip {
        Some(ip) if whitelist.is_allowed(ip) => Ok(next.run(req).await),
        _ => Err(StatusCode::FORBIDDEN),
    }
}

/// Extension trait for attaching IP whitelisting to a router.
pub trait IpWhitelistExt {
    /// Adds IP allowlist enforcement to the router.
    fn with_ip_whitelist(self, whitelist: IpWhitelist) -> Self;
}

impl IpWhitelistExt for axum::Router {
    fn with_ip_whitelist(self, whitelist: IpWhitelist) -> Self {
        let wl = Arc::new(whitelist);
        self.layer(axum::middleware::from_fn(move |req, next| {
            ip_whitelist_middleware(wl.clone(), req, next)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_exact_v4() {
        let rule = IpRule::parse("10.0.0.1").expect("valid v4");
        assert!(rule.matches("10.0.0.1".parse().expect("ip")));
        assert!(!rule.matches("10.0.0.2".parse().expect("ip")));
    }

    #[test]
    fn test_parse_exact_v6() {
        let rule = IpRule::parse("::1").expect("valid v6");
        assert!(rule.matches("::1".parse().expect("ip")));
        assert!(!rule.matches("::2".parse().expect("ip")));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(IpRule::parse("not-an-ip").is_err());
        assert!(IpRule::parse("10.0.0.0/99").is_err());
        assert!(IpRule::parse("::1/200").is_err());
    }

    #[test]
    fn test_cidr_v4_match() {
        let rule = IpRule::parse("10.0.0.0/8").expect("cidr");
        assert!(rule.matches("10.255.255.255".parse().expect("ip")));
        assert!(rule.matches("10.0.0.1".parse().expect("ip")));
        assert!(!rule.matches("11.0.0.1".parse().expect("ip")));
    }

    #[test]
    fn test_cidr_v4_24() {
        let rule = IpRule::parse("192.168.1.0/24").expect("cidr");
        assert!(rule.matches("192.168.1.50".parse().expect("ip")));
        assert!(!rule.matches("192.168.2.50".parse().expect("ip")));
    }

    #[test]
    fn test_cidr_v4_full_prefix() {
        let rule = IpRule::parse("192.168.1.1/32").expect("cidr");
        assert!(rule.matches("192.168.1.1".parse().expect("ip")));
        assert!(!rule.matches("192.168.1.2".parse().expect("ip")));
    }

    #[test]
    fn test_cidr_v4_zero_prefix() {
        let rule = IpRule::parse("0.0.0.0/0").expect("cidr");
        assert!(rule.matches("8.8.8.8".parse().expect("ip")));
        assert!(rule.matches("127.0.0.1".parse().expect("ip")));
    }

    #[test]
    fn test_cidr_v6_match() {
        let rule = IpRule::parse("2001:db8::/32").expect("cidr");
        assert!(rule.matches("2001:db8::1".parse().expect("ip")));
        assert!(rule.matches("2001:db8:ffff::1".parse().expect("ip")));
        assert!(!rule.matches("2001:db9::1".parse().expect("ip")));
    }

    #[test]
    fn test_mixed_family_no_match() {
        let rule = IpRule::parse("10.0.0.0/8").expect("cidr");
        assert!(!rule.matches("::1".parse().expect("ip")));
    }

    #[test]
    fn test_whitelist_from_specs() {
        let wl = IpWhitelist::from_specs(["10.0.0.0/8", "127.0.0.1", "::1"]).expect("valid");
        assert_eq!(wl.len(), 3);
        assert!(wl.is_allowed("10.5.5.5".parse().expect("ip")));
        assert!(wl.is_allowed("127.0.0.1".parse().expect("ip")));
        assert!(wl.is_allowed("::1".parse().expect("ip")));
        assert!(!wl.is_allowed("8.8.8.8".parse().expect("ip")));
    }

    #[test]
    fn test_empty_whitelist_denies_all() {
        let wl = IpWhitelist::new();
        assert!(wl.is_empty());
        assert!(!wl.is_allowed("127.0.0.1".parse().expect("ip")));
    }

    #[test]
    fn test_whitelist_from_specs_invalid() {
        let result = IpWhitelist::from_specs(["10.0.0.0/8", "garbage"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_trust_forwarded_flag() {
        let wl = IpWhitelist::new().trust_forwarded(true);
        assert!(wl.trusts_forwarded());
        let wl2 = IpWhitelist::new();
        assert!(!wl2.trusts_forwarded());
    }

    #[tokio::test]
    async fn test_middleware_allows_listed_ip() {
        use axum::{Router, routing::get};
        use tower::ServiceExt;

        let wl = IpWhitelist::from_specs(["127.0.0.0/8"]).expect("valid");
        let app: Router = Router::new()
            .route("/", get(|| async { "ok" }))
            .with_ip_whitelist(wl);

        // Build a request with ConnectInfo extension.
        let mut req = Request::builder()
            .uri("/")
            .body(Body::empty())
            .expect("request");
        let addr: SocketAddr = "127.0.0.1:5000".parse().expect("addr");
        req.extensions_mut().insert(ConnectInfo(addr));

        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_middleware_blocks_unlisted_ip() {
        use axum::{Router, routing::get};
        use tower::ServiceExt;

        let wl = IpWhitelist::from_specs(["10.0.0.0/8"]).expect("valid");
        let app: Router = Router::new()
            .route("/", get(|| async { "ok" }))
            .with_ip_whitelist(wl);

        let mut req = Request::builder()
            .uri("/")
            .body(Body::empty())
            .expect("request");
        let addr: SocketAddr = "8.8.8.8:5000".parse().expect("addr");
        req.extensions_mut().insert(ConnectInfo(addr));

        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_middleware_fail_closed_without_connect_info() {
        use axum::{Router, routing::get};
        use tower::ServiceExt;

        let wl = IpWhitelist::from_specs(["10.0.0.0/8"]).expect("valid");
        let app: Router = Router::new()
            .route("/", get(|| async { "ok" }))
            .with_ip_whitelist(wl);

        // No ConnectInfo and no trusted headers -> rejected.
        let req = Request::builder()
            .uri("/")
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_middleware_trusts_forwarded_header() {
        use axum::{Router, routing::get};
        use tower::ServiceExt;

        let wl = IpWhitelist::from_specs(["203.0.113.0/24"])
            .expect("valid")
            .trust_forwarded(true);
        let app: Router = Router::new()
            .route("/", get(|| async { "ok" }))
            .with_ip_whitelist(wl);

        let req = Request::builder()
            .uri("/")
            .header("X-Forwarded-For", "203.0.113.5, 10.0.0.1")
            .body(Body::empty())
            .expect("request");
        let resp = app.oneshot(req).await.expect("response");
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
