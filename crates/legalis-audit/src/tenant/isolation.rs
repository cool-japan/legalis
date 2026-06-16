//! Tenant isolation primitives.
//!
//! Two complementary mechanisms are provided:
//!
//! - [`TenantStore`] — a self-contained multi-tenant store that keeps each
//!   tenant's records in a separate namespace, each with its own append-only
//!   hash chain. Cross-tenant reads are structurally impossible: a lookup is
//!   always scoped to a single tenant's namespace.
//! - [`TenantScopedStorage`] — an [`AuditStorage`] adapter that wraps any
//!   existing backend and scopes it to one tenant via a [`TenantContext`]. It
//!   stamps writes and filters reads, so an [`crate::AuditTrail`] built on it
//!   only ever sees one tenant's data while maintaining a per-tenant chain.

use super::{TenantContext, TenantId, tenant_of};
use crate::storage::AuditStorage;
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use uuid::Uuid;

/// A single tenant's isolated namespace within a [`TenantStore`].
#[derive(Debug, Default)]
struct TenantNamespace {
    records: Vec<AuditRecord>,
    index: HashMap<Uuid, usize>,
    last_hash: Option<String>,
}

impl TenantNamespace {
    fn push(&mut self, record: AuditRecord) {
        let position = self.records.len();
        self.index.insert(record.id, position);
        self.last_hash = Some(record.record_hash.clone());
        self.records.push(record);
    }

    /// Rebuilds the index and re-anchors the chain after a structural mutation
    /// (e.g. a retention purge). Records keep their order; previous-hash links
    /// and record hashes are recomputed so the namespace remains verifiable.
    fn reanchor(&mut self) {
        self.index.clear();
        let mut previous: Option<String> = None;
        for (position, record) in self.records.iter_mut().enumerate() {
            record.relink(previous.clone());
            previous = Some(record.record_hash.clone());
            self.index.insert(record.id, position);
        }
        self.last_hash = previous;
    }
}

/// A multi-tenant, strictly isolated, in-memory audit store.
///
/// Each tenant has its own namespace and hash chain. There is no API that
/// returns records for more than one tenant at a time, and lookups by id are
/// scoped to a tenant, so one tenant can never read or even probe another
/// tenant's records.
#[derive(Default)]
pub struct TenantStore {
    namespaces: RwLock<HashMap<TenantId, TenantNamespace>>,
}

impl TenantStore {
    /// Creates an empty multi-tenant store.
    pub fn new() -> Self {
        Self {
            namespaces: RwLock::new(HashMap::new()),
        }
    }

    fn read_lock(
        &self,
    ) -> AuditResult<std::sync::RwLockReadGuard<'_, HashMap<TenantId, TenantNamespace>>> {
        self.namespaces
            .read()
            .map_err(|e| AuditError::StorageError(format!("tenant store read lock poisoned: {e}")))
    }

    fn write_lock(
        &self,
    ) -> AuditResult<std::sync::RwLockWriteGuard<'_, HashMap<TenantId, TenantNamespace>>> {
        self.namespaces
            .write()
            .map_err(|e| AuditError::StorageError(format!("tenant store write lock poisoned: {e}")))
    }

    /// Records a decision for the context's tenant, linking it into that
    /// tenant's chain. The record is stamped with the tenant id and its hash is
    /// (re)computed against the tenant's current chain head.
    ///
    /// # Errors
    /// Returns [`AuditError::InvalidRecord`] if the record is already stamped for
    /// a *different* tenant, and [`AuditError::StorageError`] on lock failure.
    pub fn record(&self, context: &TenantContext, mut record: AuditRecord) -> AuditResult<Uuid> {
        if let Some(existing) = tenant_of(&record)
            && existing != context.tenant_id
        {
            return Err(AuditError::InvalidRecord(format!(
                "record {} is bound to tenant '{existing}' and cannot be stored under '{}'",
                record.id, context.tenant_id
            )));
        }
        context.stamp(&mut record);

        let mut namespaces = self.write_lock()?;
        let namespace = namespaces
            .entry(context.tenant_id.clone())
            .or_insert_with(TenantNamespace::default);
        record.relink(namespace.last_hash.clone());
        let id = record.id;
        namespace.push(record);
        Ok(id)
    }

    /// Retrieves a record by id *within a single tenant*.
    ///
    /// # Errors
    /// Returns [`AuditError::RecordNotFound`] if the id does not exist in the
    /// given tenant's namespace — even if a record with that id exists for a
    /// different tenant. This is the core isolation guarantee.
    pub fn get(&self, tenant_id: &TenantId, id: Uuid) -> AuditResult<AuditRecord> {
        let namespaces = self.read_lock()?;
        let record = namespaces
            .get(tenant_id)
            .and_then(|ns| ns.index.get(&id).and_then(|&pos| ns.records.get(pos)))
            .cloned();
        record.ok_or(AuditError::RecordNotFound(id))
    }

    /// Returns `true` if the tenant's namespace contains the id.
    pub fn contains(&self, tenant_id: &TenantId, id: Uuid) -> AuditResult<bool> {
        let namespaces = self.read_lock()?;
        Ok(namespaces
            .get(tenant_id)
            .map(|ns| ns.index.contains_key(&id))
            .unwrap_or(false))
    }

    /// Returns all records for a tenant, in insertion order. Unknown tenants
    /// yield an empty vector.
    pub fn records_for(&self, tenant_id: &TenantId) -> AuditResult<Vec<AuditRecord>> {
        let namespaces = self.read_lock()?;
        Ok(namespaces
            .get(tenant_id)
            .map(|ns| ns.records.clone())
            .unwrap_or_default())
    }

    /// Returns the number of records held for a tenant.
    pub fn count(&self, tenant_id: &TenantId) -> AuditResult<usize> {
        let namespaces = self.read_lock()?;
        Ok(namespaces
            .get(tenant_id)
            .map(|ns| ns.records.len())
            .unwrap_or(0))
    }

    /// Returns the total number of records across all tenants.
    pub fn total_count(&self) -> AuditResult<usize> {
        let namespaces = self.read_lock()?;
        Ok(namespaces.values().map(|ns| ns.records.len()).sum())
    }

    /// Returns the ids of all tenants with at least one record, sorted.
    pub fn tenants(&self) -> AuditResult<Vec<TenantId>> {
        let namespaces = self.read_lock()?;
        let mut ids: Vec<TenantId> = namespaces.keys().cloned().collect();
        ids.sort();
        Ok(ids)
    }

    /// Verifies a single tenant's hash chain (record hashes plus chain links).
    ///
    /// # Errors
    /// Returns [`AuditError::TamperDetected`] if a record hash or chain link is
    /// invalid.
    pub fn verify_tenant(&self, tenant_id: &TenantId) -> AuditResult<bool> {
        let namespaces = self.read_lock()?;
        let Some(namespace) = namespaces.get(tenant_id) else {
            return Ok(true);
        };
        let mut expected_prev: Option<String> = None;
        for record in &namespace.records {
            if !record.verify() {
                return Err(AuditError::TamperDetected(format!(
                    "record {} in tenant '{tenant_id}' has an invalid hash",
                    record.id
                )));
            }
            if record.previous_hash != expected_prev {
                return Err(AuditError::TamperDetected(format!(
                    "record {} in tenant '{tenant_id}' has a broken chain link",
                    record.id
                )));
            }
            expected_prev = Some(record.record_hash.clone());
        }
        Ok(true)
    }

    /// Verifies every tenant's chain.
    pub fn verify_all(&self) -> AuditResult<bool> {
        let ids = self.tenants()?;
        for id in ids {
            self.verify_tenant(&id)?;
        }
        Ok(true)
    }

    /// Removes a tenant and all of its records, returning the number removed.
    pub fn drop_tenant(&self, tenant_id: &TenantId) -> AuditResult<usize> {
        let mut namespaces = self.write_lock()?;
        Ok(namespaces
            .remove(tenant_id)
            .map(|ns| ns.records.len())
            .unwrap_or(0))
    }

    /// Retains only the records of a tenant that satisfy `keep`, then re-anchors
    /// the tenant's chain so it remains verifiable. Returns the number of records
    /// removed. This is the destructive primitive used by tenant-specific
    /// retention.
    pub fn purge_tenant_where<F>(&self, tenant_id: &TenantId, keep: F) -> AuditResult<usize>
    where
        F: Fn(&AuditRecord) -> bool,
    {
        let mut namespaces = self.write_lock()?;
        let Some(namespace) = namespaces.get_mut(tenant_id) else {
            return Ok(0);
        };
        let before = namespace.records.len();
        namespace.records.retain(|r| keep(r));
        let removed = before - namespace.records.len();
        if removed > 0 {
            namespace.reanchor();
        }
        Ok(removed)
    }
}

/// An [`AuditStorage`] adapter that scopes any backend to a single tenant.
///
/// Writes are stamped with the active tenant and reads are filtered to it, so an
/// [`crate::AuditTrail`] constructed over this wrapper behaves exactly like a
/// single-tenant trail while transparently sharing a backend with other tenants.
/// The wrapper maintains its own per-tenant chain head, recovered on
/// construction from any records already present for the tenant.
pub struct TenantScopedStorage<S: AuditStorage> {
    inner: S,
    context: TenantContext,
    last_hash: Option<String>,
}

impl<S: AuditStorage> TenantScopedStorage<S> {
    /// Wraps `inner`, scoping it to `context`'s tenant.
    ///
    /// # Errors
    /// Propagates backend errors encountered while recovering the tenant's chain
    /// head.
    pub fn new(inner: S, context: TenantContext) -> AuditResult<Self> {
        let last_hash = Self::recover_chain_head(&inner, &context)?;
        Ok(Self {
            inner,
            context,
            last_hash,
        })
    }

    /// Returns the active tenant context.
    pub fn context(&self) -> &TenantContext {
        &self.context
    }

    /// Borrows the wrapped backend.
    pub fn inner(&self) -> &S {
        &self.inner
    }

    /// Computes the chain head for the tenant: the record hash of the tenant's
    /// record that is not referenced as any other tenant record's previous hash.
    fn recover_chain_head(inner: &S, context: &TenantContext) -> AuditResult<Option<String>> {
        let owned: Vec<AuditRecord> = inner
            .get_all()?
            .into_iter()
            .filter(|r| context.owns(r))
            .collect();
        if owned.is_empty() {
            return Ok(None);
        }
        let referenced: HashSet<String> = owned
            .iter()
            .filter_map(|r| r.previous_hash.clone())
            .collect();
        // The tail's hash is referenced by nobody. If several qualify (a forked
        // or pre-existing chain), prefer the most recent by timestamp.
        let mut tail: Option<&AuditRecord> = None;
        for record in &owned {
            if referenced.contains(&record.record_hash) {
                continue;
            }
            tail = match tail {
                Some(current) if current.timestamp >= record.timestamp => Some(current),
                _ => Some(record),
            };
        }
        Ok(tail
            .or_else(|| owned.iter().max_by_key(|r| r.timestamp))
            .map(|r| r.record_hash.clone()))
    }
}

impl<S: AuditStorage> AuditStorage for TenantScopedStorage<S> {
    fn store(&mut self, mut record: AuditRecord) -> AuditResult<()> {
        if let Some(existing) = tenant_of(&record)
            && existing != self.context.tenant_id
        {
            return Err(AuditError::InvalidRecord(format!(
                "record {} is bound to tenant '{existing}' and cannot be stored under '{}'",
                record.id, self.context.tenant_id
            )));
        }
        self.context.stamp(&mut record);
        self.inner.store(record)
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let record = self.inner.get(id)?;
        if self.context.owns(&record) {
            Ok(record)
        } else {
            // Do not reveal the existence of another tenant's record.
            Err(AuditError::RecordNotFound(id))
        }
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .inner
            .get_all()?
            .into_iter()
            .filter(|r| self.context.owns(r))
            .collect())
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .inner
            .get_by_statute(statute_id)?
            .into_iter()
            .filter(|r| self.context.owns(r))
            .collect())
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .inner
            .get_by_subject(subject_id)?
            .into_iter()
            .filter(|r| self.context.owns(r))
            .collect())
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        Ok(self
            .inner
            .get_by_time_range(start, end)?
            .into_iter()
            .filter(|r| self.context.owns(r))
            .collect())
    }

    fn count(&self) -> AuditResult<usize> {
        Ok(self.get_all()?.len())
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        Ok(self.last_hash.clone())
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        self.last_hash = hash;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{sample_record, tid};
    use super::*;
    use crate::AuditTrail;
    use crate::storage::memory::MemoryStorage;

    #[test]
    fn test_store_isolation_get_scoped() {
        let store = TenantStore::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let ctx_b = TenantContext::new(tid("tenant-b"));

        let id_a = store
            .record(&ctx_a, sample_record("s-1"))
            .expect("record a");
        let id_b = store
            .record(&ctx_b, sample_record("s-2"))
            .expect("record b");

        // Each tenant sees only its own record.
        assert!(store.get(&tid("tenant-a"), id_a).is_ok());
        assert!(store.get(&tid("tenant-b"), id_b).is_ok());
        // Cross-tenant lookups fail even though the id exists somewhere.
        assert!(store.get(&tid("tenant-a"), id_b).is_err());
        assert!(store.get(&tid("tenant-b"), id_a).is_err());
        assert!(!store.contains(&tid("tenant-a"), id_b).expect("contains"));
    }

    #[test]
    fn test_store_counts_and_tenants() {
        let store = TenantStore::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let ctx_b = TenantContext::new(tid("tenant-b"));
        for _ in 0..3 {
            store.record(&ctx_a, sample_record("s")).expect("a");
        }
        store.record(&ctx_b, sample_record("s")).expect("b");

        assert_eq!(store.count(&tid("tenant-a")).expect("count"), 3);
        assert_eq!(store.count(&tid("tenant-b")).expect("count"), 1);
        assert_eq!(store.total_count().expect("total"), 4);
        assert_eq!(
            store.tenants().expect("tenants"),
            vec![tid("tenant-a"), tid("tenant-b")]
        );
    }

    #[test]
    fn test_store_per_tenant_chain_verifies() {
        let store = TenantStore::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let ctx_b = TenantContext::new(tid("tenant-b"));
        for i in 0..5 {
            store
                .record(&ctx_a, sample_record(&format!("s-{i}")))
                .expect("a");
            store
                .record(&ctx_b, sample_record(&format!("s-{i}")))
                .expect("b");
        }
        assert!(store.verify_tenant(&tid("tenant-a")).expect("verify a"));
        assert!(store.verify_tenant(&tid("tenant-b")).expect("verify b"));
        assert!(store.verify_all().expect("verify all"));

        // Chains are independent: each starts from None.
        let records_a = store.records_for(&tid("tenant-a")).expect("records");
        assert!(records_a[0].previous_hash.is_none());
        assert_eq!(
            records_a[1].previous_hash,
            Some(records_a[0].record_hash.clone())
        );
    }

    #[test]
    fn test_store_reject_cross_tenant_stamped_record() {
        let store = TenantStore::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let mut record = sample_record("s");
        // Pre-stamp for tenant-b, then attempt to record under tenant-a.
        TenantContext::new(tid("tenant-b")).stamp(&mut record);
        assert!(store.record(&ctx_a, record).is_err());
    }

    #[test]
    fn test_store_purge_reanchors_chain() {
        let store = TenantStore::new();
        let ctx = TenantContext::new(tid("tenant-a"));
        let mut keep_ids = Vec::new();
        for i in 0..6 {
            let mut record = sample_record(&format!("s-{i}"));
            if i % 2 == 0 {
                record.statute_id = "keep".to_string();
            }
            let id = store.record(&ctx, record).expect("record");
            if i % 2 == 0 {
                keep_ids.push(id);
            }
        }
        let removed = store
            .purge_tenant_where(&tid("tenant-a"), |r| r.statute_id == "keep")
            .expect("purge");
        assert_eq!(removed, 3);
        assert_eq!(store.count(&tid("tenant-a")).expect("count"), 3);
        // After re-anchoring, the chain must still verify.
        assert!(store.verify_tenant(&tid("tenant-a")).expect("verify"));
        for id in keep_ids {
            assert!(store.get(&tid("tenant-a"), id).is_ok());
        }
    }

    #[test]
    fn test_scoped_storage_over_shared_backend() {
        // A single backend shared by two tenant-scoped trails stays isolated.
        let backend = MemoryStorage::new();
        let ctx_a = TenantContext::new(tid("tenant-a"));
        let ctx_b = TenantContext::new(tid("tenant-b"));

        let mut trail_a = AuditTrail::with_storage(Box::new(
            TenantScopedStorage::new(backend.clone(), ctx_a).expect("wrap a"),
        ));
        let mut trail_b = AuditTrail::with_storage(Box::new(
            TenantScopedStorage::new(backend.clone(), ctx_b).expect("wrap b"),
        ));

        let id_a = trail_a.record(sample_record("s-a")).expect("record a");
        trail_a.record(sample_record("s-a2")).expect("record a2");
        let id_b = trail_b.record(sample_record("s-b")).expect("record b");

        assert_eq!(trail_a.count(), 2);
        assert_eq!(trail_b.count(), 1);
        // Each trail can fetch its own, but not the other's.
        assert!(trail_a.get(id_a).is_ok());
        assert!(trail_a.get(id_b).is_err());
        assert!(trail_b.get(id_b).is_ok());
        assert!(trail_b.get(id_a).is_err());
        // Each per-tenant chain verifies independently.
        assert!(trail_a.verify_integrity().expect("verify a"));
        assert!(trail_b.verify_integrity().expect("verify b"));
    }

    #[test]
    fn test_scoped_storage_recovers_chain_head() {
        let backend = MemoryStorage::new();
        let ctx = TenantContext::new(tid("tenant-a"));
        {
            let mut trail = AuditTrail::with_storage(Box::new(
                TenantScopedStorage::new(backend.clone(), ctx.clone()).expect("wrap"),
            ));
            for i in 0..3 {
                trail.record(sample_record(&format!("s-{i}"))).expect("rec");
            }
        }
        // Re-wrap the same backend: the new wrapper must recover the chain head
        // and continue the chain so it still verifies.
        let mut trail = AuditTrail::with_storage(Box::new(
            TenantScopedStorage::new(backend.clone(), ctx).expect("rewrap"),
        ));
        trail.record(sample_record("s-3")).expect("rec");
        assert_eq!(trail.count(), 4);
        assert!(trail.verify_integrity().expect("verify"));
    }
}
