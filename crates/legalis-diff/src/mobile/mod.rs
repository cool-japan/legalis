//! Mobile & edge computing for statute diffing (v0.5.7).
//!
//! This module brings statute diffing to constrained and disconnected
//! environments — mobile handsets, wearables, edge nodes and browsers. It is
//! organised into five self-contained, pure-Rust sub-modules:
//!
//! - [`sdk`] — a synchronous, JSON-in/JSON-out **mobile SDK facade**
//!   ([`sdk::MobileSdk`]) that a thin iOS/Android language binding can wrap,
//!   with platform services (secure storage, push notifications) abstracted
//!   behind the pluggable [`sdk::MobileBridge`] trait.
//! - [`edge`] — a **latency-budgeted edge diff engine** ([`edge::EdgeDiffer`])
//!   that bounds work and memory for low-latency execution, plus a deterministic
//!   [`edge::EdgeScheduler`] that places jobs on the best available node.
//! - [`offline`] — **offline-first diff computation** ([`offline::OfflineEngine`]):
//!   a local snapshot store and append-only operation queue that computes diffs
//!   optimistically without a network and replays / persists durably.
//! - [`pwa`] — a **Progressive Web App generator** ([`pwa::PwaBundle`]) emitting a
//!   W3C Web App Manifest, a Service Worker and an offline-capable diff-viewer
//!   shell as real, standards-compliant assets.
//! - [`sync`] — **cross-platform synchronization** ([`sync::SyncEngine`]) built on
//!   vector clocks, delta sync and convergent conflict resolution so replicas on
//!   different devices reconcile to an identical state.
//!
//! # Deferred external bindings
//!
//! Two concerns require platforms this offline workspace does not contain, and
//! are abstracted behind traits / generators with pure-Rust backends:
//!
//! - the **native mobile SDK binding** (UniFFI / cbindgen + Swift / Kotlin,
//!   Keychain / Android Keystore, APNs / FCM) is deferred behind
//!   [`sdk::MobileBridge`]; an [`sdk::InMemoryBridge`] implements the full
//!   contract for tests and embedding;
//! - **live hosting / browser runtime** of the generated PWA is external — the
//!   [`pwa`] generator produces deployable, standards-compliant assets but does
//!   not serve them.
//!
//! Everything else (diffing, budgeting, the operation log, vector-clock sync,
//! asset generation) is implemented in full.
//!
//! # Example
//!
//! ```
//! use legalis_diff::mobile::{DeviceClass, DeviceProfile, NetworkQuality};
//!
//! let phone = DeviceProfile::new("pixel-9", DeviceClass::Phone)
//!     .with_battery_saver(true)
//!     .with_network(NetworkQuality::Moderate);
//!
//! // Battery-saver halves the usable compute capacity.
//! assert!(phone.capacity_score() > 0.0);
//! assert!(phone.network_quality.is_connected());
//! ```

use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod edge;
pub mod offline;
pub mod pwa;
pub mod sdk;
pub mod sync;

pub use edge::{
    EdgeBudget, EdgeConfig, EdgeDiffResult, EdgeDiffer, EdgeJob, EdgeNode, EdgeScheduler,
};
pub use offline::{OfflineEngine, OfflineSnapshot, OpSyncState, OperationKind, PendingOperation};
pub use pwa::{
    CacheStrategy, DisplayMode, PwaBundle, PwaFile, PwaIcon, PwaManifest, ServiceWorkerConfig,
};
pub use sdk::{
    InMemoryBridge, MobileBridge, MobileCapability, MobileConfig, MobileRequest, MobileResponse,
    MobileSdk, SdkInfo,
};
pub use sync::{
    ClockOrder, ConflictResolution, SyncConflict, SyncDelta, SyncEngine, SyncOp, SyncOutcome,
    SyncPayload, VectorClock, sync_pair,
};

/// Computes a lowercase hex SHA-256 digest over a single byte slice.
pub(crate) fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Computes a lowercase hex SHA-256 digest over several byte slices.
///
/// Each part is length-prefixed before hashing so that, for example,
/// `["ab", "c"]` and `["a", "bc"]` produce different digests (domain separation
/// against trivial concatenation collisions).
pub(crate) fn sha256_parts(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    hex::encode(hasher.finalize())
}

/// Deterministic content fingerprint of a statute, or `None` if it cannot be
/// serialized.
///
/// Used by the [`edge`] fast-path and the [`offline`] store to detect
/// byte-identical content without performing a structural diff. `Statute` does
/// not implement `PartialEq`, so a content-addressed fingerprint is the natural
/// equality test here.
pub(crate) fn statute_fingerprint(statute: &Statute) -> Option<String> {
    serde_json::to_vec(statute)
        .ok()
        .map(|bytes| sha256_hex(&bytes))
}

/// Classes of devices a diff workload can target, ordered roughly by available
/// resources.
///
/// Used to derive sensible default budgets in [`edge`] and to tag participants
/// in [`sync`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceClass {
    /// Smartwatch or very constrained wearable.
    Wearable,
    /// Mobile phone.
    Phone,
    /// Tablet.
    Tablet,
    /// Laptop or desktop.
    Desktop,
    /// Edge compute node (gateway, micro-datacentre).
    EdgeNode,
    /// Cloud server.
    Server,
}

impl DeviceClass {
    /// Typical CPU core count for the class (used as a default profile value).
    pub fn default_cpu_cores(self) -> u32 {
        match self {
            Self::Wearable => 1,
            Self::Phone => 4,
            Self::Tablet => 6,
            Self::Desktop => 8,
            Self::EdgeNode => 8,
            Self::Server => 16,
        }
    }

    /// Typical per-task memory budget in bytes for the class.
    pub fn default_memory_budget_bytes(self) -> u64 {
        match self {
            Self::Wearable => 64 * 1024 * 1024,
            Self::Phone => 512 * 1024 * 1024,
            Self::Tablet => 1024 * 1024 * 1024,
            Self::Desktop => 4 * 1024 * 1024 * 1024,
            Self::EdgeNode => 8 * 1024 * 1024 * 1024,
            Self::Server => 32u64 * 1024 * 1024 * 1024,
        }
    }
}

impl std::fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Wearable => "wearable",
            Self::Phone => "phone",
            Self::Tablet => "tablet",
            Self::Desktop => "desktop",
            Self::EdgeNode => "edge-node",
            Self::Server => "server",
        };
        f.write_str(name)
    }
}

/// Qualitative network quality for a device, from fully offline to excellent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NetworkQuality {
    /// No connectivity.
    Offline,
    /// High latency / lossy link.
    Poor,
    /// Usable but constrained link.
    Moderate,
    /// Good broadband / 4G.
    Good,
    /// Low-latency wired / 5G.
    Excellent,
}

impl NetworkQuality {
    /// Returns `true` for any state other than [`NetworkQuality::Offline`].
    pub fn is_connected(self) -> bool {
        !matches!(self, Self::Offline)
    }

    /// A representative round-trip time in milliseconds, or `None` when offline.
    pub fn typical_rtt_ms(self) -> Option<u32> {
        match self {
            Self::Offline => None,
            Self::Poor => Some(800),
            Self::Moderate => Some(300),
            Self::Good => Some(80),
            Self::Excellent => Some(20),
        }
    }
}

/// A description of a device's compute, memory and network characteristics.
///
/// Profiles drive default budgets in [`edge`] and influence job placement in
/// [`edge::EdgeScheduler`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceProfile {
    /// Stable device identifier.
    pub device_id: String,
    /// Coarse device class.
    pub class: DeviceClass,
    /// Number of CPU cores available to diff workloads.
    pub cpu_cores: u32,
    /// Per-task memory budget in bytes.
    pub memory_budget_bytes: u64,
    /// Whether the device is in a battery-saving mode (halves usable capacity).
    pub battery_saver: bool,
    /// Current network quality.
    pub network_quality: NetworkQuality,
}

impl DeviceProfile {
    /// Creates a profile for `device_id` with class-derived default resources and
    /// an assumed [`NetworkQuality::Good`] link.
    pub fn new(device_id: impl Into<String>, class: DeviceClass) -> Self {
        Self {
            device_id: device_id.into(),
            class,
            cpu_cores: class.default_cpu_cores(),
            memory_budget_bytes: class.default_memory_budget_bytes(),
            battery_saver: false,
            network_quality: NetworkQuality::Good,
        }
    }

    /// Overrides the CPU core count (clamped to at least one core).
    #[must_use]
    pub fn with_cpu_cores(mut self, cores: u32) -> Self {
        self.cpu_cores = cores.max(1);
        self
    }

    /// Overrides the per-task memory budget in bytes.
    #[must_use]
    pub fn with_memory_budget_bytes(mut self, bytes: u64) -> Self {
        self.memory_budget_bytes = bytes;
        self
    }

    /// Sets the battery-saver flag.
    #[must_use]
    pub fn with_battery_saver(mut self, on: bool) -> Self {
        self.battery_saver = on;
        self
    }

    /// Sets the network quality.
    #[must_use]
    pub fn with_network(mut self, quality: NetworkQuality) -> Self {
        self.network_quality = quality;
        self
    }

    /// A relative, dimensionless compute-capacity score.
    ///
    /// Combines core count with memory (in mebibytes), then halves the result in
    /// battery-saver mode. The value is monotonic in resources and is used only
    /// for *relative* comparison (e.g. scheduling), never as an absolute metric.
    pub fn capacity_score(&self) -> f64 {
        let mib = self.memory_budget_bytes as f64 / (1024.0 * 1024.0);
        let raw = self.cpu_cores as f64 * mib.sqrt();
        if self.battery_saver { raw / 2.0 } else { raw }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_sha256_helpers() {
        let a = sha256_hex(b"legalis");
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(sha256_parts(&[b"ab", b"c"]), sha256_parts(&[b"a", b"bc"]));
    }

    #[test]
    fn test_statute_fingerprint_stable_and_discriminating() {
        let a = Statute::new("s1", "Title", Effect::new(EffectType::Grant, "x"));
        let b = Statute::new("s1", "Title", Effect::new(EffectType::Grant, "x"));
        let c = Statute::new("s1", "Other", Effect::new(EffectType::Grant, "x"));
        assert_eq!(statute_fingerprint(&a), statute_fingerprint(&b));
        assert_ne!(statute_fingerprint(&a), statute_fingerprint(&c));
    }

    #[test]
    fn test_device_class_defaults_monotonic() {
        assert!(
            DeviceClass::Wearable.default_memory_budget_bytes()
                < DeviceClass::Server.default_memory_budget_bytes()
        );
        assert!(DeviceClass::Phone.default_cpu_cores() <= DeviceClass::Server.default_cpu_cores());
        assert_eq!(DeviceClass::EdgeNode.to_string(), "edge-node");
    }

    #[test]
    fn test_network_quality() {
        assert!(!NetworkQuality::Offline.is_connected());
        assert!(NetworkQuality::Good.is_connected());
        assert_eq!(NetworkQuality::Offline.typical_rtt_ms(), None);
        assert!(NetworkQuality::Excellent.typical_rtt_ms() < NetworkQuality::Poor.typical_rtt_ms());
        assert!(NetworkQuality::Excellent > NetworkQuality::Poor);
    }

    #[test]
    fn test_device_profile_capacity_and_battery_saver() {
        let full = DeviceProfile::new("d", DeviceClass::Phone);
        let saver = DeviceProfile::new("d", DeviceClass::Phone).with_battery_saver(true);
        assert!(saver.capacity_score() < full.capacity_score());
        assert!((saver.capacity_score() * 2.0 - full.capacity_score()).abs() < 1e-6);

        let custom = DeviceProfile::new("d", DeviceClass::Wearable)
            .with_cpu_cores(0)
            .with_memory_budget_bytes(1024 * 1024);
        assert_eq!(custom.cpu_cores, 1); // clamped
    }
}
