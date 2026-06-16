//! Threaded comments anchored to visualization nodes.
//!
//! Unlike the flat [`SharedAnnotation`](crate::SharedAnnotation) used for live
//! viewing, a [`CommentThread`] supports *nested replies*: each [`Comment`]
//! optionally references a parent comment, forming a discussion tree per node.
//! A [`CommentBoard`] collects one thread per node id and renders the whole
//! discussion as nested HTML. Comments reuse the existing
//! [`CollaborativeUser`](crate::CollaborativeUser) author model.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::data_exchange::escape_xml;
use crate::types_7::CollaborativeUser;
use crate::{VizError, VizResult};

/// A single comment, optionally a reply to another comment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    /// Unique comment id within its thread.
    pub id: String,
    /// Comment author.
    pub author: CollaborativeUser,
    /// Comment body text.
    pub body: String,
    /// Caller-supplied timestamp.
    pub timestamp: u64,
    /// Parent comment id, or `None` for a top-level comment.
    pub parent_id: Option<String>,
    /// Whether the comment has been marked resolved.
    pub resolved: bool,
}

impl Comment {
    /// Creates a top-level comment.
    pub fn root(id: &str, author: CollaborativeUser, body: &str, timestamp: u64) -> Self {
        Self {
            id: id.to_string(),
            author,
            body: body.to_string(),
            timestamp,
            parent_id: None,
            resolved: false,
        }
    }

    /// Creates a reply to `parent_id`.
    pub fn reply(
        id: &str,
        parent_id: &str,
        author: CollaborativeUser,
        body: &str,
        timestamp: u64,
    ) -> Self {
        Self {
            id: id.to_string(),
            author,
            body: body.to_string(),
            timestamp,
            parent_id: Some(parent_id.to_string()),
            resolved: false,
        }
    }
}

/// A discussion thread anchored to a single node id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentThread {
    /// The node this thread is anchored to.
    pub node_id: String,
    /// Comments, in posting order.
    comments: Vec<Comment>,
}

impl CommentThread {
    /// Creates an empty thread for a node.
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            comments: Vec::new(),
        }
    }

    /// Posts a top-level comment.
    pub fn post(&mut self, id: &str, author: CollaborativeUser, body: &str, timestamp: u64) {
        self.comments
            .push(Comment::root(id, author, body, timestamp));
    }

    /// Posts a reply to an existing comment, erroring if the parent is unknown.
    pub fn reply(
        &mut self,
        id: &str,
        parent_id: &str,
        author: CollaborativeUser,
        body: &str,
        timestamp: u64,
    ) -> VizResult<()> {
        if !self.has_comment(parent_id) {
            return Err(VizError::InvalidStructure(format!(
                "cannot reply: no comment {} in thread {}",
                parent_id, self.node_id
            )));
        }
        self.comments
            .push(Comment::reply(id, parent_id, author, body, timestamp));
        Ok(())
    }

    /// Whether a comment with the id exists in this thread.
    pub fn has_comment(&self, id: &str) -> bool {
        self.comments.iter().any(|c| c.id == id)
    }

    /// Marks a comment resolved; returns whether one was found.
    pub fn resolve(&mut self, id: &str) -> bool {
        if let Some(comment) = self.comments.iter_mut().find(|c| c.id == id) {
            comment.resolved = true;
            true
        } else {
            false
        }
    }

    /// All comments, in posting order.
    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }

    /// Top-level comments (no parent).
    pub fn roots(&self) -> Vec<&Comment> {
        self.comments
            .iter()
            .filter(|c| c.parent_id.is_none())
            .collect()
    }

    /// Direct replies to a comment.
    pub fn replies(&self, parent_id: &str) -> Vec<&Comment> {
        self.comments
            .iter()
            .filter(|c| c.parent_id.as_deref() == Some(parent_id))
            .collect()
    }

    /// Total number of comments.
    pub fn count(&self) -> usize {
        self.comments.len()
    }

    /// Number of unresolved comments.
    pub fn unresolved_count(&self) -> usize {
        self.comments.iter().filter(|c| !c.resolved).count()
    }

    /// Renders the thread as a nested HTML list.
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str(&format!(
            "<div class=\"comment-thread\" data-node=\"{}\">\n",
            escape_xml(&self.node_id)
        ));
        let mut visited: BTreeSet<&str> = BTreeSet::new();
        html.push_str("  <ul class=\"comment-list\">\n");
        for root in self.roots() {
            self.render_comment(root, &mut visited, &mut html);
        }
        html.push_str("  </ul>\n</div>\n");
        html
    }

    /// Recursively renders one comment and its replies, guarding against cycles.
    fn render_comment<'a>(
        &'a self,
        comment: &'a Comment,
        visited: &mut BTreeSet<&'a str>,
        out: &mut String,
    ) {
        if !visited.insert(comment.id.as_str()) {
            return;
        }
        let resolved_class = if comment.resolved {
            " comment-resolved"
        } else {
            ""
        };
        out.push_str(&format!(
            "    <li class=\"comment{}\" data-id=\"{}\">\n",
            resolved_class,
            escape_xml(&comment.id)
        ));
        out.push_str(&format!(
            "      <span class=\"comment-author\" style=\"color:{}\">{}</span>: {}\n",
            escape_xml(&comment.author.color),
            escape_xml(&comment.author.display_name),
            escape_xml(&comment.body)
        ));
        let children = self.replies(&comment.id);
        if !children.is_empty() {
            out.push_str("      <ul class=\"comment-replies\">\n");
            for child in children {
                self.render_comment(child, visited, out);
            }
            out.push_str("      </ul>\n");
        }
        out.push_str("    </li>\n");
    }
}

/// A collection of [`CommentThread`]s, one per node id.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentBoard {
    threads: BTreeMap<String, CommentThread>,
}

impl CommentBoard {
    /// Creates an empty board.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the thread for a node, if it exists.
    pub fn thread(&self, node_id: &str) -> Option<&CommentThread> {
        self.threads.get(node_id)
    }

    /// Returns the thread for a node, creating it if necessary.
    pub fn thread_mut(&mut self, node_id: &str) -> &mut CommentThread {
        self.threads
            .entry(node_id.to_string())
            .or_insert_with(|| CommentThread::new(node_id))
    }

    /// Posts a top-level comment on a node (creating the thread if needed).
    pub fn post(
        &mut self,
        node_id: &str,
        id: &str,
        author: CollaborativeUser,
        body: &str,
        timestamp: u64,
    ) {
        self.thread_mut(node_id).post(id, author, body, timestamp);
    }

    /// Posts a reply within a node's thread.
    pub fn reply(
        &mut self,
        node_id: &str,
        id: &str,
        parent_id: &str,
        author: CollaborativeUser,
        body: &str,
        timestamp: u64,
    ) -> VizResult<()> {
        let thread = self
            .threads
            .get_mut(node_id)
            .ok_or_else(|| VizError::InvalidStructure(format!("no thread for node {}", node_id)))?;
        thread.reply(id, parent_id, author, body, timestamp)
    }

    /// Marks a comment resolved; returns whether one was found.
    pub fn resolve(&mut self, node_id: &str, comment_id: &str) -> bool {
        self.threads
            .get_mut(node_id)
            .map(|t| t.resolve(comment_id))
            .unwrap_or(false)
    }

    /// The node ids that have threads, sorted.
    pub fn node_ids(&self) -> Vec<&str> {
        self.threads.keys().map(String::as_str).collect()
    }

    /// Total comments across all threads.
    pub fn total_comments(&self) -> usize {
        self.threads.values().map(CommentThread::count).sum()
    }

    /// Total unresolved comments across all threads.
    pub fn unresolved_comments(&self) -> usize {
        self.threads
            .values()
            .map(CommentThread::unresolved_count)
            .sum()
    }

    /// Renders all threads as HTML.
    pub fn to_html(&self) -> String {
        let mut html = String::from("<div class=\"comment-board\">\n");
        for thread in self.threads.values() {
            html.push_str(&thread.to_html());
        }
        html.push_str("</div>\n");
        html
    }

    /// Serializes the board to pretty JSON.
    pub fn to_json(&self) -> VizResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| VizError::ExportError(format!("comment board to JSON: {}", e)))
    }

    /// Parses a board from JSON.
    pub fn from_json(json: &str) -> VizResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VizError::InvalidStructure(format!("comment board from JSON: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(id: &str) -> CollaborativeUser {
        CollaborativeUser::new(id, id, "#123456")
    }

    #[test]
    fn thread_supports_nested_replies() {
        let mut thread = CommentThread::new("node-1");
        thread.post("c1", user("alice"), "Is this clause current?", 1);
        thread
            .reply("c2", "c1", user("bob"), "Yes, amended in 2020", 2)
            .expect("reply");
        thread
            .reply("c3", "c2", user("alice"), "Thanks", 3)
            .expect("nested reply");
        assert_eq!(thread.count(), 3);
        assert_eq!(thread.roots().len(), 1);
        assert_eq!(thread.replies("c1").len(), 1);
        assert_eq!(thread.replies("c2").len(), 1);
    }

    #[test]
    fn reply_to_unknown_parent_errors() {
        let mut thread = CommentThread::new("node-1");
        let err = thread
            .reply("c2", "missing", user("bob"), "hi", 1)
            .unwrap_err();
        assert!(matches!(err, VizError::InvalidStructure(_)));
    }

    #[test]
    fn resolve_marks_comment_and_counts_update() {
        let mut thread = CommentThread::new("node-1");
        thread.post("c1", user("alice"), "todo", 1);
        thread.post("c2", user("bob"), "todo2", 2);
        assert_eq!(thread.unresolved_count(), 2);
        assert!(thread.resolve("c1"));
        assert!(!thread.resolve("missing"));
        assert_eq!(thread.unresolved_count(), 1);
    }

    #[test]
    fn thread_html_nests_replies_and_escapes() {
        let mut thread = CommentThread::new("n<1>");
        thread.post("c1", user("alice"), "a <b> & c", 1);
        thread
            .reply("c2", "c1", user("bob"), "reply", 2)
            .expect("reply");
        let html = thread.to_html();
        assert!(html.contains("data-node=\"n&lt;1&gt;\""));
        assert!(html.contains("a &lt;b&gt; &amp; c"));
        assert!(html.contains("comment-replies"));
    }

    #[test]
    fn board_groups_threads_per_node() {
        let mut board = CommentBoard::new();
        board.post("n1", "c1", user("alice"), "first", 1);
        board
            .reply("n1", "c2", "c1", user("bob"), "reply", 2)
            .expect("reply");
        board.post("n2", "c3", user("carol"), "other", 3);
        assert_eq!(board.node_ids(), vec!["n1", "n2"]);
        assert_eq!(board.total_comments(), 3);
        assert!(board.resolve("n1", "c1"));
        assert_eq!(board.unresolved_comments(), 2);
        // Reply to a missing node errors.
        assert!(
            board
                .reply("missing", "x", "c1", user("a"), "b", 4)
                .is_err()
        );
    }

    #[test]
    fn board_json_round_trip() {
        let mut board = CommentBoard::new();
        board.post("n1", "c1", user("alice"), "hi", 1);
        let json = board.to_json().expect("to_json");
        let restored = CommentBoard::from_json(&json).expect("from_json");
        assert_eq!(board, restored);
    }
}
