//! Response streaming helpers.
//!
//! Provides utilities for building streamed (chunked) HTTP responses from async
//! item streams, including newline-delimited JSON (NDJSON) and JSON-array
//! streaming. Streaming lets the server emit large or incrementally-produced
//! result sets without buffering the entire payload in memory, and lets clients
//! begin processing before the full response is available.

use axum::{
    body::Body,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use futures::stream::{Stream, StreamExt};
use serde::Serialize;

/// MIME type for newline-delimited JSON streams.
pub const NDJSON_CONTENT_TYPE: &str = "application/x-ndjson";

/// Serializes a single item to an NDJSON line (JSON + trailing newline).
///
/// On serialization failure, an error object line is produced instead so the
/// stream can continue rather than aborting.
pub fn ndjson_line<T: Serialize>(item: &T) -> Vec<u8> {
    match serde_json::to_string(item) {
        Ok(mut s) => {
            s.push('\n');
            s.into_bytes()
        }
        Err(e) => {
            let err = serde_json::json!({ "error": format!("serialization failed: {e}") });
            let mut s = err.to_string();
            s.push('\n');
            s.into_bytes()
        }
    }
}

/// Builds a streamed NDJSON [`Response`] from a stream of serializable items.
///
/// Each item is emitted as its own JSON line. The response uses chunked transfer
/// encoding implicitly via the streaming body.
pub fn ndjson_response<S, T>(stream: S) -> Response
where
    S: Stream<Item = T> + Send + 'static,
    T: Serialize + Send + 'static,
{
    let byte_stream = stream
        .map(|item| Ok::<_, std::convert::Infallible>(axum::body::Bytes::from(ndjson_line(&item))));
    let body = Body::from_stream(byte_stream);
    match Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, NDJSON_CONTENT_TYPE)
        // Disable proxy buffering so chunks are flushed promptly.
        .header("X-Accel-Buffering", "no")
        .body(body)
    {
        Ok(resp) => resp,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Builds a streamed JSON-array [`Response`] from a stream of items.
///
/// Emits a well-formed JSON array (`[item, item, ...]`) incrementally, inserting
/// commas between elements. This is convenient for clients that expect a single
/// JSON document but still benefits from incremental transfer.
pub fn json_array_response<S, T>(stream: S) -> Response
where
    S: Stream<Item = T> + Send + 'static,
    T: Serialize + Send + 'static,
{
    // Drive the source stream through an `unfold` state machine that emits a
    // well-formed JSON array incrementally: `[`, elements separated by `,`, and
    // a trailing `]`. The empty case yields exactly `[]`.
    enum ArrayState<S> {
        /// Still reading from the source; `first` tracks comma insertion.
        Streaming { source: S, first: bool },
        /// Done — terminate the stream.
        Done,
    }

    let initial = ArrayState::Streaming {
        source: Box::pin(stream),
        first: true,
    };

    let byte_stream = futures::stream::unfold(initial, |state| async move {
        match state {
            ArrayState::Streaming { mut source, first } => match source.next().await {
                Some(item) => {
                    let body = serde_json::to_string(&item).unwrap_or_else(|e| {
                        serde_json::json!({ "error": format!("serialization failed: {e}") })
                            .to_string()
                    });
                    let chunk = if first {
                        format!("[{body}")
                    } else {
                        format!(",{body}")
                    };
                    Some((
                        chunk,
                        ArrayState::Streaming {
                            source,
                            first: false,
                        },
                    ))
                }
                // No items were emitted: produce a valid empty array `[]`.
                None if first => Some(("[]".to_string(), ArrayState::Done)),
                // Items were emitted: close the array.
                None => Some(("]".to_string(), ArrayState::Done)),
            },
            ArrayState::Done => None,
        }
    })
    .map(|s| Ok::<_, std::convert::Infallible>(axum::body::Bytes::from(s.into_bytes())));

    let body = Body::from_stream(byte_stream);

    match Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-Accel-Buffering", "no")
        .body(body)
    {
        Ok(resp) => resp,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Collects an NDJSON byte stream back into individual JSON values.
///
/// Primarily useful in tests / clients to verify streamed output.
pub fn parse_ndjson(bytes: &[u8]) -> Vec<serde_json::Value> {
    String::from_utf8_lossy(bytes)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn body_bytes(resp: Response) -> axum::body::Bytes {
        axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("collect body")
    }

    #[derive(Serialize)]
    struct Item {
        n: u32,
    }

    #[test]
    fn test_ndjson_line() {
        let line = ndjson_line(&Item { n: 5 });
        assert_eq!(line, b"{\"n\":5}\n");
    }

    #[test]
    fn test_parse_ndjson() {
        let data = b"{\"n\":1}\n{\"n\":2}\n";
        let values = parse_ndjson(data);
        assert_eq!(values.len(), 2);
        assert_eq!(values[0]["n"], 1);
        assert_eq!(values[1]["n"], 2);
    }

    #[tokio::test]
    async fn test_ndjson_response() {
        let stream = futures::stream::iter((0..3).map(|n| Item { n }));
        let resp = ndjson_response(stream);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok()),
            Some(NDJSON_CONTENT_TYPE)
        );
        let body = body_bytes(resp).await;
        let values = parse_ndjson(&body);
        assert_eq!(values.len(), 3);
        assert_eq!(values[2]["n"], 2);
    }

    #[tokio::test]
    async fn test_json_array_response() {
        let stream = futures::stream::iter((0..3).map(|n| Item { n }));
        let resp = json_array_response(stream);
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_bytes(resp).await;
        let parsed: serde_json::Value = serde_json::from_slice(&body).expect("valid json array");
        assert!(parsed.is_array());
        let arr = parsed.as_array().expect("array");
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0]["n"], 0);
    }

    #[tokio::test]
    async fn test_json_array_response_empty() {
        let stream = futures::stream::iter(Vec::<Item>::new());
        let resp = json_array_response(stream);
        let body = body_bytes(resp).await;
        let parsed: serde_json::Value = serde_json::from_slice(&body).expect("valid json array");
        assert_eq!(parsed, serde_json::json!([]));
    }

    #[tokio::test]
    async fn test_json_array_response_single() {
        let stream = futures::stream::iter(vec![Item { n: 42 }]);
        let resp = json_array_response(stream);
        let body = body_bytes(resp).await;
        let parsed: serde_json::Value = serde_json::from_slice(&body).expect("valid json array");
        assert_eq!(parsed, serde_json::json!([{"n": 42}]));
    }
}
