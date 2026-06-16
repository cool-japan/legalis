//! NFT generation for important diffs.
//!
//! Significant statute changes can be tokenised as non-fungible tokens. Each
//! [`DiffNft`] has a deterministic `token_id` derived from the diff's content
//! hash (so the same diff always yields the same token and duplicates are
//! rejected), ERC-721-style [`NftMetadata`], an owner [`Address`], and a
//! tamper-evident [`ProvenanceEntry`] chain recording every mint and transfer.
//! The [`NftRegistry`] is the in-memory collection that mints, transfers, burns
//! and indexes tokens by owner; serialising a token's metadata to a token URI
//! is supported for off-chain display.

use super::ledger::DiffRecord;
use super::{Address, current_timestamp, sha256_hex, sha256_parts};
use crate::{DiffError, DiffResult, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// A single ERC-721-style metadata attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NftAttribute {
    /// Attribute name.
    pub trait_type: String,
    /// Attribute value (stringified).
    pub value: String,
}

impl NftAttribute {
    /// Creates an attribute.
    pub fn new(trait_type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            trait_type: trait_type.into(),
            value: value.into(),
        }
    }
}

/// ERC-721-style metadata for a diff NFT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NftMetadata {
    /// Display name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Statute the diff applies to.
    pub statute_id: String,
    /// Content hash of the tokenised diff.
    pub diff_hash: String,
    /// Severity of the tokenised diff.
    pub severity: Severity,
    /// Number of changes in the diff.
    pub change_count: usize,
    /// Additional attributes.
    pub attributes: Vec<NftAttribute>,
    /// UNIX timestamp the metadata was created.
    pub created_at: u64,
}

/// One step in a token's provenance chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    /// Previous owner (`None` for the mint).
    pub from: Option<String>,
    /// New owner after this step.
    pub to: String,
    /// UNIX timestamp of the step.
    pub timestamp: u64,
    /// Hash of the previous entry (zero hash for the mint).
    pub previous_hash: String,
    /// Hash committing to this entry and its predecessor.
    pub entry_hash: String,
}

/// A non-fungible token representing an important diff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffNft {
    /// Deterministic, unique token identifier.
    pub token_id: String,
    /// Current owner.
    pub owner: String,
    /// Token metadata.
    pub metadata: NftMetadata,
    /// UNIX timestamp of minting.
    pub minted_at: u64,
    /// Provenance chain (mint followed by transfers).
    pub provenance: Vec<ProvenanceEntry>,
}

impl DiffNft {
    /// Recomputes and verifies the provenance chain's hash linkage and that the
    /// final entry's owner matches [`DiffNft::owner`].
    pub fn verify_provenance(&self) -> bool {
        let mut previous_hash = ZERO_HASH.to_string();
        for entry in &self.provenance {
            if entry.previous_hash != previous_hash {
                return false;
            }
            let expected = provenance_hash(
                &self.token_id,
                entry.from.as_deref(),
                &entry.to,
                entry.timestamp,
                &entry.previous_hash,
            );
            if expected != entry.entry_hash {
                return false;
            }
            previous_hash = entry.entry_hash.clone();
        }
        match self.provenance.last() {
            Some(last) => last.to == self.owner,
            None => false,
        }
    }

    /// Serialises the metadata to a JSON token URI document.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if serialization fails.
    pub fn token_uri(&self) -> DiffResult<String> {
        serde_json::to_string(&self.metadata).map_err(|e| {
            DiffError::SerializationError(format!("failed to serialize token metadata: {}", e))
        })
    }
}

/// An in-memory registry (collection) of diff NFTs.
#[derive(Debug, Clone, Default)]
pub struct NftRegistry {
    tokens: HashMap<String, DiffNft>,
    owner_index: HashMap<String, Vec<String>>,
}

impl NftRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of live tokens.
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Whether the registry holds no tokens.
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Mints an NFT for a diff, owned by `owner`.
    ///
    /// The token id is derived from the diff content, so minting the same diff
    /// twice is rejected.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the diff cannot be hashed, or
    /// [`DiffError::NftError`] if a token for this diff already exists.
    pub fn mint(&mut self, diff: &StatuteDiff, owner: &Address) -> DiffResult<String> {
        let bytes = serde_json::to_vec(diff).map_err(|e| {
            DiffError::SerializationError(format!("failed to serialize diff: {}", e))
        })?;
        let diff_hash = sha256_hex(&bytes);
        self.mint_with_hash(
            &diff.statute_id,
            &diff_hash,
            diff.impact.severity,
            diff.changes.len(),
            owner,
        )
    }

    /// Mints an NFT from a ledger [`DiffRecord`], reusing its content hash.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::NftError`] if a token for this diff already exists.
    pub fn mint_from_record(&mut self, record: &DiffRecord, owner: &Address) -> DiffResult<String> {
        self.mint_with_hash(
            &record.statute_id,
            &record.diff_hash,
            record.severity,
            record.change_count,
            owner,
        )
    }

    fn mint_with_hash(
        &mut self,
        statute_id: &str,
        diff_hash: &str,
        severity: Severity,
        change_count: usize,
        owner: &Address,
    ) -> DiffResult<String> {
        let token_id = token_id_for(diff_hash, statute_id);
        if self.tokens.contains_key(&token_id) {
            return Err(DiffError::NftError(format!(
                "an NFT already exists for diff {} of statute '{}'",
                diff_hash, statute_id
            )));
        }
        let now = current_timestamp();
        let metadata = NftMetadata {
            name: format!("Statute Diff: {}", statute_id),
            description: format!(
                "Tokenised {:?}-severity change set ({} change(s)) for statute '{}'",
                severity, change_count, statute_id
            ),
            statute_id: statute_id.to_string(),
            diff_hash: diff_hash.to_string(),
            severity,
            change_count,
            attributes: vec![
                NftAttribute::new("severity", format!("{:?}", severity)),
                NftAttribute::new("change_count", change_count.to_string()),
                NftAttribute::new("statute_id", statute_id),
            ],
            created_at: now,
        };
        let genesis = make_provenance(&token_id, None, owner.as_str(), now, ZERO_HASH);
        let nft = DiffNft {
            token_id: token_id.clone(),
            owner: owner.to_string(),
            metadata,
            minted_at: now,
            provenance: vec![genesis],
        };
        self.owner_index
            .entry(owner.to_string())
            .or_default()
            .push(token_id.clone());
        self.tokens.insert(token_id.clone(), nft);
        Ok(token_id)
    }

    /// Returns a token by id.
    pub fn get(&self, token_id: &str) -> Option<&DiffNft> {
        self.tokens.get(token_id)
    }

    /// Returns the owner of a token.
    pub fn owner_of(&self, token_id: &str) -> Option<Address> {
        self.tokens
            .get(token_id)
            .map(|n| Address::from_label(n.owner.clone()))
    }

    /// Returns the token ids owned by `owner`.
    pub fn tokens_of(&self, owner: &Address) -> Vec<String> {
        self.owner_index
            .get(owner.as_str())
            .cloned()
            .unwrap_or_default()
    }

    /// Transfers a token from its current owner to `to`.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::NftError`] if the token does not exist or `from` is
    /// not the current owner.
    pub fn transfer(&mut self, token_id: &str, from: &Address, to: &Address) -> DiffResult<()> {
        let nft = self
            .tokens
            .get_mut(token_id)
            .ok_or_else(|| DiffError::NftError(format!("unknown token '{}'", token_id)))?;
        if nft.owner != from.as_str() {
            return Err(DiffError::NftError(format!(
                "'{}' does not own token '{}'",
                from, token_id
            )));
        }
        if from == to {
            return Err(DiffError::NftError(
                "cannot transfer a token to its current owner".to_string(),
            ));
        }
        let previous_hash = nft
            .provenance
            .last()
            .map(|e| e.entry_hash.clone())
            .unwrap_or_else(|| ZERO_HASH.to_string());
        let entry = make_provenance(
            token_id,
            Some(from.as_str()),
            to.as_str(),
            current_timestamp(),
            &previous_hash,
        );
        nft.provenance.push(entry);
        nft.owner = to.to_string();

        // Update ownership index.
        if let Some(list) = self.owner_index.get_mut(from.as_str()) {
            list.retain(|t| t != token_id);
        }
        self.owner_index
            .entry(to.to_string())
            .or_default()
            .push(token_id.to_string());
        Ok(())
    }

    /// Burns (permanently removes) a token owned by `owner`.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::NftError`] if the token does not exist or `owner` is
    /// not the current owner.
    pub fn burn(&mut self, token_id: &str, owner: &Address) -> DiffResult<()> {
        let nft = self
            .tokens
            .get(token_id)
            .ok_or_else(|| DiffError::NftError(format!("unknown token '{}'", token_id)))?;
        if nft.owner != owner.as_str() {
            return Err(DiffError::NftError(format!(
                "'{}' does not own token '{}'",
                owner, token_id
            )));
        }
        self.tokens.remove(token_id);
        if let Some(list) = self.owner_index.get_mut(owner.as_str()) {
            list.retain(|t| t != token_id);
        }
        Ok(())
    }
}

/// Returns whether a diff is significant enough to warrant minting an NFT.
///
/// Important diffs are those with at least [`Severity::Major`] impact or that
/// change the statute's outcome.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::diff;
/// use legalis_diff::blockchain::is_mint_worthy;
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Revoke, "Revoked");
/// let d = diff(&old, &new).unwrap();
/// assert!(is_mint_worthy(&d));
/// ```
pub fn is_mint_worthy(diff: &StatuteDiff) -> bool {
    diff.impact.severity >= Severity::Major || diff.impact.affects_outcome
}

/// Derives a deterministic token id from a diff hash and statute id.
fn token_id_for(diff_hash: &str, statute_id: &str) -> String {
    let digest = sha256_parts(&[diff_hash.as_bytes(), statute_id.as_bytes()]);
    format!("nft-{}", &digest[..40])
}

/// Computes the hash committing to a provenance entry.
fn provenance_hash(
    token_id: &str,
    from: Option<&str>,
    to: &str,
    timestamp: u64,
    previous_hash: &str,
) -> String {
    sha256_parts(&[
        token_id.as_bytes(),
        from.unwrap_or("").as_bytes(),
        to.as_bytes(),
        &timestamp.to_le_bytes(),
        previous_hash.as_bytes(),
    ])
}

/// Builds a provenance entry with its committed hash.
fn make_provenance(
    token_id: &str,
    from: Option<&str>,
    to: &str,
    timestamp: u64,
    previous_hash: &str,
) -> ProvenanceEntry {
    let entry_hash = provenance_hash(token_id, from, to, timestamp, previous_hash);
    ProvenanceEntry {
        from: from.map(|f| f.to_string()),
        to: to.to_string(),
        timestamp,
        previous_hash: previous_hash.to_string(),
        entry_hash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn breaking_diff(id: &str) -> StatuteDiff {
        let old = Statute::new(id, "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        diff(&old, &new).expect("diff")
    }

    fn minor_diff(id: &str) -> StatuteDiff {
        let old = Statute::new(id, "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.title = "New".to_string();
        diff(&old, &new).expect("diff")
    }

    #[test]
    fn test_is_mint_worthy() {
        assert!(is_mint_worthy(&breaking_diff("a")));
        assert!(!is_mint_worthy(&minor_diff("b")));
    }

    #[test]
    fn test_mint_creates_token() {
        let mut registry = NftRegistry::new();
        assert!(registry.is_empty());
        let owner = Address::from_key(b"curator");
        let token_id = registry.mint(&breaking_diff("law"), &owner).expect("mint");
        assert_eq!(registry.len(), 1);
        let nft = registry.get(&token_id).expect("token");
        assert_eq!(nft.owner, owner.to_string());
        assert_eq!(nft.metadata.statute_id, "law");
        assert_eq!(nft.metadata.severity, Severity::Major);
        assert!(token_id.starts_with("nft-"));
    }

    #[test]
    fn test_mint_is_deterministic_and_rejects_duplicate() {
        let mut registry = NftRegistry::new();
        let owner = Address::from_key(b"curator");
        let d = breaking_diff("law");
        let id1 = registry.mint(&d, &owner).expect("mint1");
        // Minting the same diff again must fail (deterministic token id).
        let err = registry.mint(&d, &owner);
        assert!(matches!(err, Err(DiffError::NftError(_))));
        assert_eq!(registry.len(), 1);
        // Determinism: a fresh registry mints the same diff to the same token id.
        let mut other = NftRegistry::new();
        let id2 = other.mint(&d, &owner).expect("mint-other");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_mint_from_record() {
        let mut registry = NftRegistry::new();
        let owner = Address::from_key(b"curator");
        let record = DiffRecord::from_diff(&breaking_diff("rec"), "alice").expect("record");
        let token_id = registry.mint_from_record(&record, &owner).expect("mint");
        let nft = registry.get(&token_id).expect("token");
        assert_eq!(nft.metadata.diff_hash, record.diff_hash);
    }

    #[test]
    fn test_provenance_on_mint() {
        let mut registry = NftRegistry::new();
        let owner = Address::from_key(b"curator");
        let token_id = registry.mint(&breaking_diff("law"), &owner).expect("mint");
        let nft = registry.get(&token_id).expect("token");
        assert_eq!(nft.provenance.len(), 1);
        assert_eq!(nft.provenance[0].from, None);
        assert!(nft.verify_provenance());
    }

    #[test]
    fn test_transfer_updates_owner_and_provenance() {
        let mut registry = NftRegistry::new();
        let alice = Address::from_key(b"alice");
        let bob = Address::from_key(b"bob");
        let token_id = registry.mint(&breaking_diff("law"), &alice).expect("mint");
        registry
            .transfer(&token_id, &alice, &bob)
            .expect("transfer");
        let nft = registry.get(&token_id).expect("token");
        assert_eq!(nft.owner, bob.to_string());
        assert_eq!(nft.provenance.len(), 2);
        assert_eq!(nft.provenance[1].from.as_deref(), Some(alice.as_str()));
        assert!(nft.verify_provenance());
        assert_eq!(registry.tokens_of(&bob), vec![token_id.clone()]);
        assert!(registry.tokens_of(&alice).is_empty());
    }

    #[test]
    fn test_transfer_requires_ownership() {
        let mut registry = NftRegistry::new();
        let alice = Address::from_key(b"alice");
        let bob = Address::from_key(b"bob");
        let mallory = Address::from_key(b"mallory");
        let token_id = registry.mint(&breaking_diff("law"), &alice).expect("mint");
        // Mallory does not own it.
        assert!(registry.transfer(&token_id, &mallory, &bob).is_err());
    }

    #[test]
    fn test_transfer_to_self_rejected() {
        let mut registry = NftRegistry::new();
        let alice = Address::from_key(b"alice");
        let token_id = registry.mint(&breaking_diff("law"), &alice).expect("mint");
        assert!(registry.transfer(&token_id, &alice, &alice).is_err());
    }

    #[test]
    fn test_transfer_unknown_token() {
        let mut registry = NftRegistry::new();
        let alice = Address::from_key(b"alice");
        let bob = Address::from_key(b"bob");
        assert!(registry.transfer("nft-missing", &alice, &bob).is_err());
    }

    #[test]
    fn test_burn() {
        let mut registry = NftRegistry::new();
        let alice = Address::from_key(b"alice");
        let token_id = registry.mint(&breaking_diff("law"), &alice).expect("mint");
        registry.burn(&token_id, &alice).expect("burn");
        assert!(registry.is_empty());
        assert!(registry.get(&token_id).is_none());
        assert!(registry.tokens_of(&alice).is_empty());
    }

    #[test]
    fn test_burn_requires_ownership() {
        let mut registry = NftRegistry::new();
        let alice = Address::from_key(b"alice");
        let mallory = Address::from_key(b"mallory");
        let token_id = registry.mint(&breaking_diff("law"), &alice).expect("mint");
        assert!(registry.burn(&token_id, &mallory).is_err());
    }

    #[test]
    fn test_provenance_tamper_detected() {
        let mut registry = NftRegistry::new();
        let alice = Address::from_key(b"alice");
        let bob = Address::from_key(b"bob");
        let token_id = registry.mint(&breaking_diff("law"), &alice).expect("mint");
        registry
            .transfer(&token_id, &alice, &bob)
            .expect("transfer");
        let nft = registry.tokens.get_mut(&token_id).expect("token");
        // Tamper with a provenance recipient.
        nft.provenance[1].to = "0xhacker".to_string();
        assert!(!nft.verify_provenance());
    }

    #[test]
    fn test_token_uri_roundtrip() {
        let mut registry = NftRegistry::new();
        let alice = Address::from_key(b"alice");
        let token_id = registry.mint(&breaking_diff("law"), &alice).expect("mint");
        let nft = registry.get(&token_id).expect("token");
        let uri = nft.token_uri().expect("uri");
        let parsed: NftMetadata = serde_json::from_str(&uri).expect("parse");
        assert_eq!(parsed, nft.metadata);
        assert!(uri.contains("Statute Diff"));
    }

    #[test]
    fn test_two_statutes_distinct_tokens() {
        let mut registry = NftRegistry::new();
        let owner = Address::from_key(b"curator");
        let id1 = registry
            .mint(&breaking_diff("alpha"), &owner)
            .expect("mint1");
        let id2 = registry
            .mint(&breaking_diff("beta"), &owner)
            .expect("mint2");
        assert_ne!(id1, id2);
        assert_eq!(registry.len(), 2);
        assert_eq!(registry.tokens_of(&owner).len(), 2);
    }
}
