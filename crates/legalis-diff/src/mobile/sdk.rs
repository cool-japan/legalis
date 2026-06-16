//! Mobile SDK facade: a synchronous, JSON-in / JSON-out diff API for iOS/Android.
//!
//! A native mobile binding (Swift/Kotlin) cannot call arbitrary Rust generics
//! across the foreign-function boundary, but it *can* pass a string in and get a
//! string out. [`MobileSdk::handle_json`] is exactly that boundary: it accepts a
//! serialized [`MobileRequest`], dispatches to the in-process diff engine and
//! returns a serialized [`MobileResponse`], never panicking. A thin
//! UniFFI/cbindgen layer wrapping this one method is all a platform binding
//! needs — which is why that layer is the *only* part deferred here.
//!
//! Platform services that genuinely require the OS — secure key/value storage
//! (Keychain / Android Keystore) and push notifications (APNs / FCM) — are
//! abstracted behind the [`MobileBridge`] trait. [`InMemoryBridge`] implements
//! the full contract in pure Rust for tests and embedding.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_diff::mobile::sdk::{InMemoryBridge, MobileConfig, MobileRequest, MobileSdk};
//!
//! let sdk = MobileSdk::new(MobileConfig::new("phone-1"), InMemoryBridge::new("ios"));
//!
//! let old = Statute::new("s", "Old", Effect::new(EffectType::Grant, "x"));
//! let mut new = old.clone();
//! new.title = "New".to_string();
//!
//! // The native layer builds this JSON and forwards it verbatim to `handle_json`.
//! let request_json = serde_json::to_string(&MobileRequest::ComputeDiff {
//!     old: Box::new(old),
//!     new: Box::new(new),
//! })
//! .unwrap();
//! let response = sdk.handle_json(&request_json);
//! assert!(response.contains("\"type\":\"Diff\""));
//! ```

use crate::mobile::sha256_hex;
use crate::{
    DetailedSummary, DiffError, DiffResult, StatuteDiff, detailed_summary, diff,
    has_breaking_changes, summarize,
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Configuration for a [`MobileSdk`] instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobileConfig {
    /// Stable device identifier (also used as the default storage namespace).
    pub device_id: String,
    /// BCP-47 locale hint forwarded to summary generation.
    pub locale: String,
    /// Whether JSON responses should be pretty-printed (debug builds).
    pub pretty_json: bool,
}

impl MobileConfig {
    /// Creates a configuration for `device_id` with locale `en` and compact JSON.
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            locale: "en".to_string(),
            pretty_json: false,
        }
    }

    /// Sets the locale hint.
    #[must_use]
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = locale.into();
        self
    }

    /// Enables or disables pretty-printed JSON responses.
    #[must_use]
    pub fn with_pretty_json(mut self, pretty: bool) -> Self {
        self.pretty_json = pretty;
        self
    }
}

/// A capability advertised by the SDK to its host binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MobileCapability {
    /// Compute a structural statute diff.
    Diff,
    /// Render a plain-text change summary.
    Summarize,
    /// Render a detailed summary with confidence scores.
    DetailedSummary,
    /// Classify a diff as breaking / non-breaking.
    BreakingCheck,
    /// Durable secure storage of diffs via the bridge.
    SecureStorage,
    /// Push notification delivery via the bridge.
    PushNotifications,
}

impl MobileCapability {
    /// All capabilities, in a stable order.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Diff,
            Self::Summarize,
            Self::DetailedSummary,
            Self::BreakingCheck,
            Self::SecureStorage,
            Self::PushNotifications,
        ]
    }
}

/// Static information about an SDK build, returned by [`MobileRequest::SdkInfo`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkInfo {
    /// Crate version of the underlying diff engine.
    pub engine_version: String,
    /// Name reported by the active [`MobileBridge`] (e.g. `ios`, `android`).
    pub platform: String,
    /// Capabilities exposed to the host binding.
    pub capabilities: Vec<MobileCapability>,
    /// The device identifier the SDK was configured with.
    pub device_id: String,
}

/// A request crossing the mobile FFI boundary.
///
/// Serialized with an internally-tagged `type` discriminator so the JSON is
/// natural for a Swift/Kotlin caller to build.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MobileRequest {
    /// Compute the diff between two statute versions.
    ComputeDiff {
        /// Earlier statute version (boxed to keep the enum small).
        old: Box<Statute>,
        /// Later statute version (boxed to keep the enum small).
        new: Box<Statute>,
    },
    /// Produce a plain-text summary of a previously computed diff.
    Summarize {
        /// The diff to summarize (boxed to keep the enum small).
        diff: Box<StatuteDiff>,
    },
    /// Produce a detailed summary with confidence scores.
    DetailedSummary {
        /// The diff to analyze (boxed to keep the enum small).
        diff: Box<StatuteDiff>,
    },
    /// Determine whether a diff contains breaking changes.
    BreakingCheck {
        /// The diff to classify (boxed to keep the enum small).
        diff: Box<StatuteDiff>,
    },
    /// Request static SDK / platform information.
    SdkInfo,
}

/// A response crossing the mobile FFI boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MobileResponse {
    /// Result of [`MobileRequest::ComputeDiff`].
    Diff {
        /// The computed diff (boxed to keep the enum small).
        diff: Box<StatuteDiff>,
    },
    /// Result of [`MobileRequest::Summarize`].
    Text {
        /// The rendered text.
        text: String,
    },
    /// Result of [`MobileRequest::DetailedSummary`].
    Detailed {
        /// The detailed summary (boxed to keep the enum small).
        summary: Box<DetailedSummary>,
    },
    /// Result of [`MobileRequest::BreakingCheck`].
    Breaking {
        /// Whether the diff is breaking.
        breaking: bool,
    },
    /// Result of [`MobileRequest::SdkInfo`].
    Info {
        /// Static SDK information.
        info: SdkInfo,
    },
    /// Any failure, including request-parse failures.
    Error {
        /// Human-readable error message.
        message: String,
    },
}

/// Platform services a native host must provide, deferred behind this trait.
///
/// A production iOS/Android binding implements this against Keychain / Android
/// Keystore (for [`persist`](MobileBridge::persist) / [`load`](MobileBridge::load))
/// and APNs / FCM (for [`notify`](MobileBridge::notify)). [`InMemoryBridge`]
/// provides a complete pure-Rust implementation for tests and embedding.
pub trait MobileBridge {
    /// A short name for the host platform (e.g. `ios`, `android`, `memory`).
    fn platform_name(&self) -> String;

    /// Persists `value` under `key` in secure device storage.
    ///
    /// # Errors
    ///
    /// Returns a [`DiffError`] if the underlying store cannot be written.
    fn persist(&self, key: &str, value: &[u8]) -> DiffResult<()>;

    /// Loads the value previously stored under `key`, if any.
    ///
    /// # Errors
    ///
    /// Returns a [`DiffError`] if the underlying store cannot be read.
    fn load(&self, key: &str) -> DiffResult<Option<Vec<u8>>>;

    /// Removes any value stored under `key` (idempotent).
    ///
    /// # Errors
    ///
    /// Returns a [`DiffError`] if the underlying store cannot be modified.
    fn remove(&self, key: &str) -> DiffResult<()>;

    /// Delivers a push notification with `title` and `body`.
    ///
    /// # Errors
    ///
    /// Returns a [`DiffError`] if the notification cannot be queued.
    fn notify(&self, title: &str, body: &str) -> DiffResult<()>;
}

/// A pure-Rust [`MobileBridge`] backed by in-memory maps.
///
/// Storage and the notification outbox use interior mutability ([`Mutex`]) so the
/// bridge can be shared (e.g. behind an `Arc`) exactly as a real platform bridge
/// would be. Lock poisoning is surfaced as a [`DiffError`] rather than a panic.
#[derive(Debug, Default)]
pub struct InMemoryBridge {
    platform: String,
    store: Mutex<HashMap<String, Vec<u8>>>,
    outbox: Mutex<Vec<(String, String)>>,
}

impl InMemoryBridge {
    /// Creates a bridge reporting `platform` as its platform name.
    pub fn new(platform: impl Into<String>) -> Self {
        Self {
            platform: platform.into(),
            store: Mutex::new(HashMap::new()),
            outbox: Mutex::new(Vec::new()),
        }
    }

    /// Returns the queued `(title, body)` notifications in delivery order.
    ///
    /// # Errors
    ///
    /// Returns a [`DiffError`] if the outbox lock is poisoned.
    pub fn notifications(&self) -> DiffResult<Vec<(String, String)>> {
        let guard = self.outbox.lock().map_err(|_| {
            DiffError::SerializationError("bridge outbox lock poisoned".to_string())
        })?;
        Ok(guard.clone())
    }

    /// Returns the number of stored keys.
    ///
    /// # Errors
    ///
    /// Returns a [`DiffError`] if the store lock is poisoned.
    pub fn stored_keys(&self) -> DiffResult<usize> {
        let guard = self
            .store
            .lock()
            .map_err(|_| DiffError::SerializationError("bridge store lock poisoned".to_string()))?;
        Ok(guard.len())
    }
}

impl MobileBridge for InMemoryBridge {
    fn platform_name(&self) -> String {
        self.platform.clone()
    }

    fn persist(&self, key: &str, value: &[u8]) -> DiffResult<()> {
        let mut guard = self
            .store
            .lock()
            .map_err(|_| DiffError::SerializationError("bridge store lock poisoned".to_string()))?;
        guard.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn load(&self, key: &str) -> DiffResult<Option<Vec<u8>>> {
        let guard = self
            .store
            .lock()
            .map_err(|_| DiffError::SerializationError("bridge store lock poisoned".to_string()))?;
        Ok(guard.get(key).cloned())
    }

    fn remove(&self, key: &str) -> DiffResult<()> {
        let mut guard = self
            .store
            .lock()
            .map_err(|_| DiffError::SerializationError("bridge store lock poisoned".to_string()))?;
        guard.remove(key);
        Ok(())
    }

    fn notify(&self, title: &str, body: &str) -> DiffResult<()> {
        let mut guard = self.outbox.lock().map_err(|_| {
            DiffError::SerializationError("bridge outbox lock poisoned".to_string())
        })?;
        guard.push((title.to_string(), body.to_string()));
        Ok(())
    }
}

/// The mobile SDK: a thin, panic-free dispatcher over the diff engine plus a
/// platform [`MobileBridge`].
pub struct MobileSdk<B: MobileBridge> {
    config: MobileConfig,
    bridge: B,
}

impl<B: MobileBridge> MobileSdk<B> {
    /// Creates an SDK from a configuration and a platform bridge.
    pub fn new(config: MobileConfig, bridge: B) -> Self {
        Self { config, bridge }
    }

    /// Returns the SDK configuration.
    pub fn config(&self) -> &MobileConfig {
        &self.config
    }

    /// Returns the underlying platform bridge.
    pub fn bridge(&self) -> &B {
        &self.bridge
    }

    /// Returns static SDK / platform information.
    pub fn info(&self) -> SdkInfo {
        SdkInfo {
            engine_version: env!("CARGO_PKG_VERSION").to_string(),
            platform: self.bridge.platform_name(),
            capabilities: MobileCapability::all(),
            device_id: self.config.device_id.clone(),
        }
    }

    /// Dispatches a typed request to a typed response. Never panics; engine
    /// failures are mapped to [`MobileResponse::Error`].
    pub fn handle(&self, request: MobileRequest) -> MobileResponse {
        match request {
            MobileRequest::ComputeDiff { old, new } => match diff(&old, &new) {
                Ok(d) => MobileResponse::Diff { diff: Box::new(d) },
                Err(e) => MobileResponse::Error {
                    message: e.to_string(),
                },
            },
            MobileRequest::Summarize { diff } => MobileResponse::Text {
                text: summarize(&diff),
            },
            MobileRequest::DetailedSummary { diff } => MobileResponse::Detailed {
                summary: Box::new(detailed_summary(&diff)),
            },
            MobileRequest::BreakingCheck { diff } => MobileResponse::Breaking {
                breaking: has_breaking_changes(&diff),
            },
            MobileRequest::SdkInfo => MobileResponse::Info { info: self.info() },
        }
    }

    /// The canonical FFI boundary: parse a JSON request, dispatch and return a
    /// JSON response. Guaranteed to return a valid JSON string and never panic;
    /// parse and serialization failures become [`MobileResponse::Error`].
    pub fn handle_json(&self, request_json: &str) -> String {
        let response = match serde_json::from_str::<MobileRequest>(request_json) {
            Ok(request) => self.handle(request),
            Err(e) => MobileResponse::Error {
                message: format!("invalid request JSON: {}", e),
            },
        };
        self.encode_response(&response)
    }

    fn encode_response(&self, response: &MobileResponse) -> String {
        let encoded = if self.config.pretty_json {
            serde_json::to_string_pretty(response)
        } else {
            serde_json::to_string(response)
        };
        match encoded {
            Ok(json) => json,
            Err(e) => {
                // Last-resort, hand-built error payload (cannot itself fail).
                format!(
                    "{{\"type\":\"Error\",\"message\":\"failed to encode response: {}\"}}",
                    e.to_string().replace('"', "'")
                )
            }
        }
    }

    /// Persists a diff to secure storage via the bridge, returning the storage
    /// key (a content hash) the diff was stored under.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the diff cannot be encoded,
    /// or any error the bridge raises.
    pub fn persist_diff(&self, diff: &StatuteDiff) -> DiffResult<String> {
        let bytes = serde_json::to_vec(diff)
            .map_err(|e| DiffError::SerializationError(format!("failed to encode diff: {}", e)))?;
        let key = format!("diff:{}", &sha256_hex(&bytes)[..16]);
        self.bridge.persist(&key, &bytes)?;
        Ok(key)
    }

    /// Loads a diff previously stored under `key`, if present.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if stored bytes cannot be
    /// decoded, or any error the bridge raises.
    pub fn load_diff(&self, key: &str) -> DiffResult<Option<StatuteDiff>> {
        match self.bridge.load(key)? {
            None => Ok(None),
            Some(bytes) => {
                let diff = serde_json::from_slice(&bytes).map_err(|e| {
                    DiffError::SerializationError(format!("failed to decode diff: {}", e))
                })?;
                Ok(Some(diff))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn old_statute() -> Statute {
        Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"))
    }

    fn new_statute() -> Statute {
        old_statute().with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    fn sdk() -> MobileSdk<InMemoryBridge> {
        MobileSdk::new(MobileConfig::new("dev-1"), InMemoryBridge::new("test"))
    }

    #[test]
    fn test_handle_compute_diff_typed() {
        let response = sdk().handle(MobileRequest::ComputeDiff {
            old: Box::new(old_statute()),
            new: Box::new(new_statute()),
        });
        match response {
            MobileResponse::Diff { diff } => assert_eq!(diff.changes.len(), 1),
            other => panic!("expected Diff, got {:?}", other),
        }
    }

    #[test]
    fn test_handle_json_round_trip() {
        let request = MobileRequest::ComputeDiff {
            old: Box::new(old_statute()),
            new: Box::new(new_statute()),
        };
        let request_json = serde_json::to_string(&request).expect("encode request");
        let response_json = sdk().handle_json(&request_json);
        let response: MobileResponse =
            serde_json::from_str(&response_json).expect("decode response");
        assert!(matches!(response, MobileResponse::Diff { .. }));
    }

    #[test]
    fn test_handle_json_invalid_is_error_not_panic() {
        let response_json = sdk().handle_json("{ not valid json");
        assert!(response_json.contains("\"type\":\"Error\""));
        let response: MobileResponse =
            serde_json::from_str(&response_json).expect("error is valid json");
        assert!(matches!(response, MobileResponse::Error { .. }));
    }

    #[test]
    fn test_summarize_and_breaking_and_detailed() {
        let s = sdk();
        let d = diff(&old_statute(), &new_statute()).expect("diff");

        let text = s.handle(MobileRequest::Summarize {
            diff: Box::new(d.clone()),
        });
        assert!(matches!(text, MobileResponse::Text { .. }));

        let detailed = s.handle(MobileRequest::DetailedSummary {
            diff: Box::new(d.clone()),
        });
        assert!(matches!(detailed, MobileResponse::Detailed { .. }));

        let breaking = s.handle(MobileRequest::BreakingCheck { diff: Box::new(d) });
        match breaking {
            MobileResponse::Breaking { breaking } => assert!(breaking),
            other => panic!("expected Breaking, got {:?}", other),
        }
    }

    #[test]
    fn test_sdk_info() {
        let s = sdk();
        match s.handle(MobileRequest::SdkInfo) {
            MobileResponse::Info { info } => {
                assert_eq!(info.platform, "test");
                assert_eq!(info.device_id, "dev-1");
                assert!(info.capabilities.contains(&MobileCapability::Diff));
                assert!(!info.engine_version.is_empty());
            }
            other => panic!("expected Info, got {:?}", other),
        }
    }

    #[test]
    fn test_persist_and_load_diff_via_bridge() {
        let s = sdk();
        let d = diff(&old_statute(), &new_statute()).expect("diff");
        let key = s.persist_diff(&d).expect("persist");
        assert_eq!(s.bridge().stored_keys().expect("count"), 1);

        let loaded = s.load_diff(&key).expect("load").expect("present");
        assert_eq!(loaded.changes.len(), d.changes.len());
        assert!(s.load_diff("missing").expect("load missing").is_none());
    }

    #[test]
    fn test_bridge_notifications_and_remove() {
        let bridge = InMemoryBridge::new("ios");
        bridge.notify("Diff ready", "1 change").expect("notify");
        bridge.persist("k", b"v").expect("persist");
        bridge.remove("k").expect("remove");
        bridge.remove("k").expect("idempotent remove");
        assert_eq!(bridge.stored_keys().expect("count"), 0);
        let outbox = bridge.notifications().expect("outbox");
        assert_eq!(outbox.len(), 1);
        assert_eq!(outbox[0].0, "Diff ready");
    }

    #[test]
    fn test_pretty_json_config() {
        let s = MobileSdk::new(
            MobileConfig::new("dev")
                .with_pretty_json(true)
                .with_locale("ja"),
            InMemoryBridge::new("android"),
        );
        assert_eq!(s.config().locale, "ja");
        let out = s.handle_json("{\"type\":\"SdkInfo\"}");
        assert!(out.contains('\n')); // pretty-printed spans multiple lines
    }
}
