//! Opaque, tamper-evident cursor pagination.
//!
//! This module provides typed, opaque pagination cursors. A cursor encodes the
//! position of the last item returned (a key plus an optional secondary sort
//! value and direction) together with an HMAC-SHA256 signature derived from a
//! server secret. Clients receive only an opaque base64 string and cannot forge
//! or tamper with cursors; the server verifies the signature on decode.
//!
//! Cursors are intentionally decoupled from offset-based pagination: they encode
//! a stable key so that pages remain consistent even as the underlying data set
//! changes (insertions/deletions do not shift subsequent pages the way offsets
//! do).

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as BASE64};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Pagination direction relative to the cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    /// Items after the cursor (forward pagination).
    #[default]
    Forward,
    /// Items before the cursor (backward pagination).
    Backward,
}

/// The decoded payload of a pagination cursor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPayload {
    /// Primary key of the last item on the page (e.g. statute id).
    pub key: String,
    /// Optional secondary sort value (e.g. a version number or timestamp), used
    /// when sorting by a non-unique field with the key as a tie-breaker.
    pub sort_value: Option<String>,
    /// Pagination direction.
    pub direction: Direction,
}

impl CursorPayload {
    /// Creates a forward cursor for a key.
    pub fn forward(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            sort_value: None,
            direction: Direction::Forward,
        }
    }

    /// Creates a backward cursor for a key.
    pub fn backward(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            sort_value: None,
            direction: Direction::Backward,
        }
    }

    /// Adds a secondary sort value.
    pub fn with_sort_value(mut self, value: impl Into<String>) -> Self {
        self.sort_value = Some(value.into());
        self
    }
}

/// Errors produced while decoding or verifying a cursor.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CursorError {
    /// The cursor is not valid base64.
    #[error("cursor is not valid base64")]
    InvalidEncoding,
    /// The cursor payload is malformed.
    #[error("cursor payload is malformed")]
    MalformedPayload,
    /// The signature did not verify (tampering or wrong secret).
    #[error("cursor signature verification failed")]
    SignatureMismatch,
}

/// A codec that signs and verifies pagination cursors with an HMAC secret.
#[derive(Clone)]
pub struct CursorCodec {
    secret: Vec<u8>,
}

impl CursorCodec {
    /// Creates a codec from a server secret. The secret must be kept private;
    /// rotating it invalidates all previously issued cursors.
    pub fn new(secret: impl Into<Vec<u8>>) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    /// Computes an HMAC-SHA256 over `data` using the configured secret.
    ///
    /// Implements HMAC manually over SHA-256 to avoid pulling in an extra
    /// dependency, following RFC 2104 (block size 64 for SHA-256).
    fn hmac(&self, data: &[u8]) -> [u8; 32] {
        const BLOCK_SIZE: usize = 64;
        let mut key = [0u8; BLOCK_SIZE];
        if self.secret.len() > BLOCK_SIZE {
            let mut hasher = Sha256::new();
            hasher.update(&self.secret);
            let digest = hasher.finalize();
            key[..digest.len()].copy_from_slice(&digest);
        } else {
            key[..self.secret.len()].copy_from_slice(&self.secret);
        }

        let mut ipad = [0x36u8; BLOCK_SIZE];
        let mut opad = [0x5cu8; BLOCK_SIZE];
        for i in 0..BLOCK_SIZE {
            ipad[i] ^= key[i];
            opad[i] ^= key[i];
        }

        let mut inner = Sha256::new();
        inner.update(ipad);
        inner.update(data);
        let inner_digest = inner.finalize();

        let mut outer = Sha256::new();
        outer.update(opad);
        outer.update(inner_digest);
        let out = outer.finalize();

        let mut result = [0u8; 32];
        result.copy_from_slice(&out);
        result
    }

    /// Encodes a payload into an opaque, signed cursor string.
    pub fn encode(&self, payload: &CursorPayload) -> String {
        // Serialize payload deterministically as JSON.
        let body = serde_json::to_vec(payload).unwrap_or_default();
        let sig = self.hmac(&body);
        // Frame: 4-byte big-endian body length + body + 32-byte signature.
        let mut framed = Vec::with_capacity(4 + body.len() + 32);
        framed.extend_from_slice(&(body.len() as u32).to_be_bytes());
        framed.extend_from_slice(&body);
        framed.extend_from_slice(&sig);
        BASE64.encode(framed)
    }

    /// Decodes and verifies an opaque cursor string back into its payload.
    pub fn decode(&self, cursor: &str) -> Result<CursorPayload, CursorError> {
        let framed = BASE64
            .decode(cursor.as_bytes())
            .map_err(|_| CursorError::InvalidEncoding)?;
        if framed.len() < 4 + 32 {
            return Err(CursorError::MalformedPayload);
        }
        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&framed[..4]);
        let body_len = u32::from_be_bytes(len_bytes) as usize;
        if framed.len() != 4 + body_len + 32 {
            return Err(CursorError::MalformedPayload);
        }
        let body = &framed[4..4 + body_len];
        let sig = &framed[4 + body_len..];

        let expected = self.hmac(body);
        if !constant_time_eq(sig, &expected) {
            return Err(CursorError::SignatureMismatch);
        }

        serde_json::from_slice(body).map_err(|_| CursorError::MalformedPayload)
    }
}

/// Constant-time byte slice comparison to avoid timing side-channels on the
/// signature check.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Result of paginating a slice with cursors.
#[derive(Debug, Clone)]
pub struct Page<T> {
    /// Items on this page.
    pub items: Vec<T>,
    /// Cursor pointing to the item after the last one returned, if more exist.
    pub next_cursor: Option<String>,
    /// Cursor pointing before the first item, if a previous page exists.
    pub prev_cursor: Option<String>,
    /// Whether more items follow this page.
    pub has_more: bool,
}

/// Paginates a pre-sorted slice using an opaque cursor.
///
/// `key_of` extracts the stable key for each item. The cursor (if any) is
/// decoded and verified with `codec`, and items strictly after the matching key
/// are returned (up to `limit`). A `next_cursor` is produced when more items
/// remain.
///
/// The input is assumed to be sorted by the same key used in the cursor.
pub fn paginate<T, F>(
    items: &[T],
    cursor: Option<&str>,
    limit: usize,
    codec: &CursorCodec,
    key_of: F,
) -> Result<Page<T>, CursorError>
where
    T: Clone,
    F: Fn(&T) -> String,
{
    let limit = limit.max(1);
    let start = match cursor {
        None => 0,
        Some(c) => {
            let payload = codec.decode(c)?;
            // Find the item matching the cursor key and start just after it.
            match items.iter().position(|it| key_of(it) == payload.key) {
                Some(pos) => pos + 1,
                // Cursor key no longer present: start from beginning so the
                // client still makes progress rather than getting an error.
                None => 0,
            }
        }
    };

    let slice: Vec<T> = items.iter().skip(start).take(limit + 1).cloned().collect();
    let has_more = slice.len() > limit;
    let mut page_items = slice;
    if has_more {
        page_items.pop();
    }

    let next_cursor = if has_more {
        page_items
            .last()
            .map(|last| codec.encode(&CursorPayload::forward(key_of(last))))
    } else {
        None
    };

    let prev_cursor = if start > 0 {
        page_items
            .first()
            .map(|first| codec.encode(&CursorPayload::backward(key_of(first))))
    } else {
        None
    };

    Ok(Page {
        items: page_items,
        next_cursor,
        prev_cursor,
        has_more,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn codec() -> CursorCodec {
        CursorCodec::new(b"super-secret-key".to_vec())
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let codec = codec();
        let payload = CursorPayload::forward("statute-42").with_sort_value("3");
        let encoded = codec.encode(&payload);
        let decoded = codec.decode(&encoded).expect("decode");
        assert_eq!(decoded, payload);
    }

    #[test]
    fn test_cursor_is_opaque() {
        let codec = codec();
        let payload = CursorPayload::forward("statute-42");
        let encoded = codec.encode(&payload);
        // The opaque form should not contain the raw key.
        assert!(!encoded.contains("statute-42"));
    }

    #[test]
    fn test_tampering_detected() {
        let codec = codec();
        let payload = CursorPayload::forward("statute-42");
        let encoded = codec.encode(&payload);
        // Flip a character in the middle of the cursor.
        let mut bytes = encoded.into_bytes();
        let mid = bytes.len() / 2;
        bytes[mid] = if bytes[mid] == b'A' { b'B' } else { b'A' };
        let tampered = String::from_utf8(bytes).expect("utf8");
        let result = codec.decode(&tampered);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_secret_rejected() {
        let codec_a = CursorCodec::new(b"secret-a".to_vec());
        let codec_b = CursorCodec::new(b"secret-b".to_vec());
        let payload = CursorPayload::forward("x");
        let encoded = codec_a.encode(&payload);
        assert_eq!(
            codec_b.decode(&encoded),
            Err(CursorError::SignatureMismatch)
        );
    }

    #[test]
    fn test_invalid_base64() {
        let codec = codec();
        assert_eq!(
            codec.decode("not valid base64!!!"),
            Err(CursorError::InvalidEncoding)
        );
    }

    #[test]
    fn test_truncated_cursor() {
        let codec = codec();
        let encoded = BASE64.encode([0u8; 3]);
        assert_eq!(codec.decode(&encoded), Err(CursorError::MalformedPayload));
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
    }

    #[test]
    fn test_long_secret_is_hashed() {
        // Secret longer than the 64-byte block size exercises the key-hashing path.
        let long_secret = vec![7u8; 200];
        let codec = CursorCodec::new(long_secret);
        let payload = CursorPayload::forward("k");
        let encoded = codec.encode(&payload);
        assert_eq!(codec.decode(&encoded).expect("decode"), payload);
    }

    #[test]
    fn test_paginate_first_page() {
        let codec = codec();
        let items: Vec<String> = (0..10).map(|i| format!("item-{i:02}")).collect();
        let page = paginate(&items, None, 3, &codec, |s| s.clone()).expect("page");
        assert_eq!(page.items, vec!["item-00", "item-01", "item-02"]);
        assert!(page.has_more);
        assert!(page.next_cursor.is_some());
        assert!(page.prev_cursor.is_none());
    }

    #[test]
    fn test_paginate_follow_cursor() {
        let codec = codec();
        let items: Vec<String> = (0..10).map(|i| format!("item-{i:02}")).collect();
        let page1 = paginate(&items, None, 3, &codec, |s| s.clone()).expect("page1");
        let cursor = page1.next_cursor.expect("next cursor");
        let page2 = paginate(&items, Some(&cursor), 3, &codec, |s| s.clone()).expect("page2");
        assert_eq!(page2.items, vec!["item-03", "item-04", "item-05"]);
        assert!(page2.has_more);
        assert!(page2.prev_cursor.is_some());
    }

    #[test]
    fn test_paginate_last_page() {
        let codec = codec();
        let items: Vec<String> = (0..5).map(|i| format!("item-{i:02}")).collect();
        // Cursor at item-02 -> remaining items 03, 04.
        let cursor = codec.encode(&CursorPayload::forward("item-02"));
        let page = paginate(&items, Some(&cursor), 10, &codec, |s| s.clone()).expect("page");
        assert_eq!(page.items, vec!["item-03", "item-04"]);
        assert!(!page.has_more);
        assert!(page.next_cursor.is_none());
    }

    #[test]
    fn test_paginate_missing_cursor_key_restarts() {
        let codec = codec();
        let items: Vec<String> = (0..5).map(|i| format!("item-{i:02}")).collect();
        // Cursor pointing at a deleted item: pagination restarts from the top.
        let cursor = codec.encode(&CursorPayload::forward("item-99"));
        let page = paginate(&items, Some(&cursor), 2, &codec, |s| s.clone()).expect("page");
        assert_eq!(page.items, vec!["item-00", "item-01"]);
    }

    #[test]
    fn test_paginate_invalid_cursor_errors() {
        let codec = codec();
        let items: Vec<String> = (0..5).map(|i| format!("item-{i:02}")).collect();
        let result = paginate(&items, Some("garbage"), 2, &codec, |s| s.clone());
        assert!(result.is_err());
    }
}
