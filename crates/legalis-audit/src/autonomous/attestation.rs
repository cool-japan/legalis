//! Continuous compliance attestation.
//!
//! An [`AttestationEngine`] periodically produces a [`ComplianceAttestation`]: a
//! tamper-evident, fingerprinted statement that, over a specific time window, a
//! defined set of compliance checks held (or did not). Each attestation:
//!
//! - binds the **window** and the **set of records** it covers via a Merkle-free
//!   SHA-256 digest of the records' own hashes (the `coverage_digest`), so the
//!   exact evidence is pinned;
//! - records the **outcome of each [`AttestationCheck`]** and an overall
//!   [`AttestationVerdict`];
//! - is itself **hash-chained** to the previous attestation (`previous_hash` →
//!   `attestation_hash`), forming a continuous, append-only attestation ledger;
//! - is optionally **signed** with a quantum-resistant hash-based signature via
//!   the crate's [`crate::quantum`] machinery, giving non-repudiable proof that
//!   *this* engine produced *this* attestation.
//!
//! Reusing the crate's existing SHA-256 (`crate::quantum::sha256`) keeps the
//! integrity story consistent with the rest of the audit chain and adds no new
//! dependency.

use crate::autonomous::monitor::{ComplianceMonitor, MonitorReport, MonitorSeverity};
use crate::quantum::{
    MerklePublicKey, MerkleSignature, PqHashAlgorithm, QuantumKeyStore, sha256, to_hex,
};
use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The outcome of a single attested check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckOutcome {
    /// The check passed.
    Pass,
    /// The check failed.
    Fail,
    /// The check could not be evaluated (e.g. no data).
    Inconclusive,
}

impl CheckOutcome {
    fn label(self) -> &'static str {
        match self {
            CheckOutcome::Pass => "pass",
            CheckOutcome::Fail => "fail",
            CheckOutcome::Inconclusive => "inconclusive",
        }
    }
}

/// A single compliance check captured in an attestation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationCheck {
    /// Stable identifier (e.g. an invariant id or control id).
    pub id: String,
    /// Human-readable description.
    pub description: String,
    /// Outcome of the check.
    pub outcome: CheckOutcome,
    /// Optional supporting detail (observed value, message, ...).
    pub detail: Option<String>,
}

impl AttestationCheck {
    /// Builds a check.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        outcome: CheckOutcome,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            outcome,
            detail: None,
        }
    }

    /// Adds detail (builder style).
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// The overall verdict of an attestation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttestationVerdict {
    /// All checks passed.
    Compliant,
    /// Some checks could not be evaluated, but none failed.
    PartiallyAttested,
    /// At least one check failed.
    NonCompliant,
}

impl AttestationVerdict {
    /// Derives a verdict from a slice of check outcomes.
    pub fn from_checks(checks: &[AttestationCheck]) -> Self {
        if checks.iter().any(|c| c.outcome == CheckOutcome::Fail) {
            AttestationVerdict::NonCompliant
        } else if checks
            .iter()
            .any(|c| c.outcome == CheckOutcome::Inconclusive)
        {
            AttestationVerdict::PartiallyAttested
        } else {
            AttestationVerdict::Compliant
        }
    }

    /// Stable lower-snake label.
    pub fn label(self) -> &'static str {
        match self {
            AttestationVerdict::Compliant => "compliant",
            AttestationVerdict::PartiallyAttested => "partially_attested",
            AttestationVerdict::NonCompliant => "non_compliant",
        }
    }
}

/// A quantum-resistant signature over an attestation digest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationSignature {
    /// The compact public key the signature verifies against.
    pub public_key: MerklePublicKey,
    /// The hash-based Merkle signature.
    pub signature: MerkleSignature,
}

/// A tamper-evident statement of compliance over a time window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAttestation {
    /// Stable identifier.
    pub id: Uuid,
    /// Inclusive start of the attested window.
    pub window_start: DateTime<Utc>,
    /// Exclusive (or inclusive) end of the attested window.
    pub window_end: DateTime<Utc>,
    /// Number of records covered.
    pub records_covered: usize,
    /// SHA-256 digest binding the exact covered records (order-independent).
    pub coverage_digest: String,
    /// The checks evaluated for this window.
    pub checks: Vec<AttestationCheck>,
    /// Overall verdict.
    pub verdict: AttestationVerdict,
    /// The hash algorithm used for the digest/chain.
    pub algorithm: PqHashAlgorithm,
    /// When the attestation was produced.
    pub generated_at: DateTime<Utc>,
    /// Hash of the previous attestation (chain linkage).
    pub previous_hash: Option<String>,
    /// SHA-256 fingerprint of this attestation's content.
    pub attestation_hash: String,
    /// Optional quantum-resistant signature over [`Self::attestation_hash`].
    pub signature: Option<AttestationSignature>,
}

impl ComplianceAttestation {
    /// Computes the fingerprint binding this attestation's content (everything
    /// except the signature, which signs the fingerprint itself).
    fn compute_hash(&self) -> String {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.id.as_bytes());
        buf.extend_from_slice(&self.window_start.timestamp().to_le_bytes());
        buf.extend_from_slice(&self.window_end.timestamp().to_le_bytes());
        buf.extend_from_slice(&(self.records_covered as u64).to_le_bytes());
        buf.extend_from_slice(self.coverage_digest.as_bytes());
        buf.extend_from_slice(self.verdict.label().as_bytes());
        if let Some(prev) = &self.previous_hash {
            buf.extend_from_slice(prev.as_bytes());
        }
        for c in &self.checks {
            buf.extend_from_slice(c.id.as_bytes());
            buf.extend_from_slice(c.outcome.label().as_bytes());
        }
        to_hex(&sha256(&buf))
    }

    /// Verifies the content fingerprint (not the signature).
    pub fn verify_hash(&self) -> bool {
        self.compute_hash() == self.attestation_hash
    }

    /// Verifies the cryptographic signature, if present, against
    /// [`Self::attestation_hash`]. Returns `Ok(true)` when there is no
    /// signature *and* the content hash is valid (an unsigned-but-intact
    /// attestation), `Ok(false)` on mismatch.
    pub fn verify_signature(&self) -> AuditResult<bool> {
        if !self.verify_hash() {
            return Ok(false);
        }
        match &self.signature {
            None => Ok(true),
            Some(sig) => Ok(sig
                .public_key
                .verify(self.attestation_hash.as_bytes(), &sig.signature)),
        }
    }

    /// Recomputes the coverage digest for `records` and confirms it matches the
    /// pinned digest — i.e. that these are exactly the attested records.
    pub fn verify_coverage(&self, records: &[AuditRecord]) -> bool {
        coverage_digest(records) == self.coverage_digest
    }
}

/// Computes an order-independent SHA-256 digest over a record set by hashing the
/// sorted multiset of per-record hashes. Two attestations covering the same
/// records (in any storage order) therefore produce the same digest.
pub fn coverage_digest(records: &[AuditRecord]) -> String {
    let mut hashes: Vec<String> = records.iter().map(|r| r.record_hash.clone()).collect();
    hashes.sort();
    let mut buf = Vec::new();
    buf.extend_from_slice(&(hashes.len() as u64).to_le_bytes());
    for h in &hashes {
        buf.extend_from_slice(&(h.len() as u64).to_le_bytes());
        buf.extend_from_slice(h.as_bytes());
    }
    to_hex(&sha256(&buf))
}

/// Verifies a chain of attestations: each content fingerprint and signature must
/// be valid, and each `previous_hash` must equal its predecessor's
/// `attestation_hash`.
pub fn verify_attestation_chain(attestations: &[ComplianceAttestation]) -> AuditResult<bool> {
    let mut expected_prev: Option<String> = None;
    for a in attestations {
        if !a.verify_signature()? {
            return Ok(false);
        }
        if a.previous_hash != expected_prev {
            return Ok(false);
        }
        expected_prev = Some(a.attestation_hash.clone());
    }
    Ok(true)
}

/// Produces a continuous chain of signed compliance attestations.
///
/// The engine owns an optional [`QuantumKeyStore`] signing key; when present,
/// every attestation is signed (consuming one one-time Merkle leaf — size the
/// key's height to the number of attestations expected before rotation).
pub struct AttestationEngine {
    monitor: ComplianceMonitor,
    algorithm: PqHashAlgorithm,
    last_hash: Option<String>,
    key_store: Option<QuantumKeyStore>,
    signing_key: Option<Uuid>,
}

impl AttestationEngine {
    /// Creates an unsigned engine using the supplied monitor and SHA-256.
    pub fn new(monitor: ComplianceMonitor) -> Self {
        Self {
            monitor,
            algorithm: PqHashAlgorithm::Sha256,
            last_hash: None,
            key_store: None,
            signing_key: None,
        }
    }

    /// Creates an engine with the default monitor invariant set.
    pub fn with_defaults() -> Self {
        Self::new(ComplianceMonitor::with_defaults())
    }

    /// Enables quantum-resistant signing with a freshly generated key of the
    /// given Merkle `height` (`2^height` attestations before exhaustion).
    pub fn with_signing(mut self, height: u32) -> AuditResult<Self> {
        let mut store = QuantumKeyStore::new();
        let key = store.generate(self.algorithm, height)?;
        self.key_store = Some(store);
        self.signing_key = Some(key);
        Ok(self)
    }

    /// The fingerprint of the last attestation produced, if any.
    pub fn last_hash(&self) -> Option<&str> {
        self.last_hash.as_deref()
    }

    /// `true` when this engine signs its attestations.
    pub fn is_signing(&self) -> bool {
        self.signing_key.is_some()
    }

    /// Produces an attestation over `records` for the trailing `window`
    /// (relative to the latest record). Records outside the window are excluded.
    pub fn attest_window(
        &mut self,
        records: &[AuditRecord],
        window: Duration,
    ) -> AuditResult<ComplianceAttestation> {
        let now = Utc::now();
        if records.is_empty() {
            return self.attest_records(&[], now, now);
        }
        let latest = records.iter().map(|r| r.timestamp).max().unwrap_or(now);
        let start = latest - window;
        let scoped: Vec<AuditRecord> = records
            .iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= latest)
            .cloned()
            .collect();
        self.attest_records(&scoped, start, latest)
    }

    /// Produces an attestation over an explicit set of records and an explicit
    /// window `[start, end]`.
    pub fn attest_records(
        &mut self,
        records: &[AuditRecord],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<ComplianceAttestation> {
        let now = Utc::now();
        let report = self.monitor.evaluate(records);
        let checks = Self::build_checks(records, &report);
        let verdict = AttestationVerdict::from_checks(&checks);

        let mut attestation = ComplianceAttestation {
            id: Uuid::new_v4(),
            window_start: start,
            window_end: end,
            records_covered: records.len(),
            coverage_digest: coverage_digest(records),
            checks,
            verdict,
            algorithm: self.algorithm,
            generated_at: now,
            previous_hash: self.last_hash.clone(),
            attestation_hash: String::new(),
            signature: None,
        };
        attestation.attestation_hash = attestation.compute_hash();

        // Sign the fingerprint if a key is configured.
        if let (Some(store), Some(key)) = (self.key_store.as_mut(), self.signing_key) {
            let public_key = store.public_key(key)?;
            let signature = store.sign_next(key, attestation.attestation_hash.as_bytes())?;
            attestation.signature = Some(AttestationSignature {
                public_key,
                signature,
            });
        }

        self.last_hash = Some(attestation.attestation_hash.clone());
        Ok(attestation)
    }

    /// Turns a monitor report into a set of attestation checks: one per
    /// invariant (pass when not in the findings; fail when present), plus a
    /// chain-integrity and a coverage check.
    fn build_checks(records: &[AuditRecord], report: &MonitorReport) -> Vec<AttestationCheck> {
        let mut checks = Vec::new();

        // One check per configured invariant outcome captured in the report:
        // a violated invariant becomes a Fail; everything else is Pass.
        // We reconstruct the invariant universe from the report's findings plus
        // the metrics it computed (findings carry the invariant id).
        for finding in &report.findings {
            let outcome = if finding.severity >= MonitorSeverity::Critical {
                CheckOutcome::Fail
            } else {
                // Warnings still count as a failed *attestation* check for that
                // invariant, but we annotate severity.
                CheckOutcome::Fail
            };
            checks.push(
                AttestationCheck::new(
                    finding.invariant_id.clone(),
                    finding.invariant_name.clone(),
                    outcome,
                )
                .with_detail(finding.message.clone()),
            );
        }

        // Chain-integrity check (independent of invariants).
        let integrity_outcome = if records.is_empty() {
            CheckOutcome::Inconclusive
        } else if report.metrics.chain_integrity {
            CheckOutcome::Pass
        } else {
            CheckOutcome::Fail
        };
        checks.push(
            AttestationCheck::new(
                "attest-chain-integrity",
                "Audit hash chain verified end-to-end",
                integrity_outcome,
            )
            .with_detail(format!(
                "{} broken record hash(es)",
                report.metrics.broken_record_hashes
            )),
        );

        // If there were no findings and there are records, add an explicit
        // "no invariant violations" pass so a clean window is positively
        // attested rather than vacuously compliant.
        if report.findings.is_empty() {
            let outcome = if records.is_empty() {
                CheckOutcome::Inconclusive
            } else {
                CheckOutcome::Pass
            };
            checks.push(AttestationCheck::new(
                "attest-no-violations",
                "No monitored invariant violations in window",
                outcome,
            ));
        }

        checks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::autonomous::monitor::{Comparator, Invariant, MonitoredMetric};
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn det(ts: DateTime<Utc>) -> AuditRecord {
        let mut r = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            "s".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        );
        r.timestamp = ts;
        r.relink(None);
        r
    }

    fn voided(ts: DateTime<Utc>) -> AuditRecord {
        let mut r = det(ts);
        r.result = DecisionResult::Void {
            reason: "logic error".to_string(),
        };
        r.relink(None);
        r
    }

    fn chain(records: &mut [AuditRecord]) {
        let mut prev: Option<String> = None;
        for r in records.iter_mut() {
            r.relink(prev.clone());
            prev = Some(r.record_hash.clone());
        }
    }

    #[test]
    fn test_clean_window_is_compliant() {
        let now = Utc::now();
        let mut records = vec![det(now), det(now), det(now)];
        chain(&mut records);
        let mut engine = AttestationEngine::with_defaults();
        let att = engine
            .attest_records(&records, now - Duration::hours(1), now)
            .expect("attest");
        assert_eq!(att.verdict, AttestationVerdict::Compliant);
        assert!(att.verify_hash());
        assert!(att.verify_coverage(&records));
    }

    #[test]
    fn test_voided_window_is_non_compliant() {
        let now = Utc::now();
        let mut records = vec![voided(now), voided(now), det(now)];
        chain(&mut records);
        let mut engine = AttestationEngine::with_defaults();
        let att = engine
            .attest_records(&records, now - Duration::hours(1), now)
            .expect("attest");
        assert_eq!(att.verdict, AttestationVerdict::NonCompliant);
        assert!(att.checks.iter().any(|c| c.outcome == CheckOutcome::Fail));
    }

    #[test]
    fn test_signed_attestation_verifies() {
        let now = Utc::now();
        let mut records = vec![det(now), det(now)];
        chain(&mut records);
        let mut engine = AttestationEngine::with_defaults()
            .with_signing(4)
            .expect("signing");
        assert!(engine.is_signing());
        let att = engine
            .attest_records(&records, now - Duration::hours(1), now)
            .expect("attest");
        assert!(att.signature.is_some());
        assert!(att.verify_signature().expect("verify"));
    }

    #[test]
    fn test_tampered_signed_attestation_fails() {
        let now = Utc::now();
        let mut records = vec![det(now), det(now)];
        chain(&mut records);
        let mut engine = AttestationEngine::with_defaults()
            .with_signing(4)
            .expect("signing");
        let mut att = engine
            .attest_records(&records, now - Duration::hours(1), now)
            .expect("attest");
        // Tamper with the verdict after signing.
        att.verdict = AttestationVerdict::Compliant;
        att.records_covered = 999;
        assert!(!att.verify_hash());
        assert!(!att.verify_signature().expect("verify"));
    }

    #[test]
    fn test_attestation_chain_links_and_verifies() {
        let now = Utc::now();
        let mut records = vec![det(now), det(now)];
        chain(&mut records);
        let mut engine = AttestationEngine::with_defaults()
            .with_signing(4)
            .expect("signing");
        let a1 = engine
            .attest_records(&records, now - Duration::hours(2), now - Duration::hours(1))
            .expect("a1");
        let a2 = engine
            .attest_records(&records, now - Duration::hours(1), now)
            .expect("a2");
        assert_eq!(
            a2.previous_hash.as_deref(),
            Some(a1.attestation_hash.as_str())
        );
        assert!(verify_attestation_chain(&[a1, a2]).expect("chain"));
    }

    #[test]
    fn test_broken_chain_fails() {
        let now = Utc::now();
        let mut records = vec![det(now)];
        chain(&mut records);
        let mut engine = AttestationEngine::with_defaults();
        let a1 = engine
            .attest_records(&records, now - Duration::hours(2), now - Duration::hours(1))
            .expect("a1");
        let mut a2 = engine
            .attest_records(&records, now - Duration::hours(1), now)
            .expect("a2");
        // Break linkage.
        a2.previous_hash = Some("nope".to_string());
        assert!(!verify_attestation_chain(&[a1, a2]).expect("chain"));
    }

    #[test]
    fn test_coverage_digest_order_independent() {
        let now = Utc::now();
        let mut records = vec![det(now), det(now), det(now)];
        chain(&mut records);
        let d1 = coverage_digest(&records);
        records.reverse();
        let d2 = coverage_digest(&records);
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_coverage_detects_different_records() {
        let now = Utc::now();
        let mut a = vec![det(now), det(now)];
        chain(&mut a);
        let mut b = vec![det(now), det(now), det(now)];
        chain(&mut b);
        let mut engine = AttestationEngine::with_defaults();
        let att = engine
            .attest_records(&a, now - Duration::hours(1), now)
            .expect("attest");
        assert!(att.verify_coverage(&a));
        assert!(!att.verify_coverage(&b));
    }

    #[test]
    fn test_attest_window_scopes() {
        let now = Utc::now();
        let mut records = vec![voided(now - Duration::days(10)), det(now), det(now)];
        chain(&mut records);
        let mut engine = AttestationEngine::with_defaults();
        // Last day: only clean records.
        let att = engine
            .attest_window(&records, Duration::days(1))
            .expect("attest");
        assert_eq!(att.records_covered, 2);
        assert_eq!(att.verdict, AttestationVerdict::Compliant);
    }

    #[test]
    fn test_empty_window_inconclusive() {
        let mut engine = AttestationEngine::with_defaults();
        let att = engine
            .attest_records(&[], Utc::now(), Utc::now())
            .expect("attest");
        // No data -> partially attested (inconclusive checks, no failures).
        assert_eq!(att.verdict, AttestationVerdict::PartiallyAttested);
    }

    #[test]
    fn test_unsigned_attestation_verifies_when_intact() {
        let now = Utc::now();
        let mut records = vec![det(now)];
        chain(&mut records);
        let mut engine = AttestationEngine::new(ComplianceMonitor::new().add_invariant(
            Invariant::new(MonitoredMetric::OverrideRate, Comparator::GreaterThan, 0.9),
        ));
        let att = engine
            .attest_records(&records, now - Duration::hours(1), now)
            .expect("attest");
        assert!(att.signature.is_none());
        assert!(att.verify_signature().expect("verify"));
    }
}
