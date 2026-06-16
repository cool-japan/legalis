//! Collaborative VR exploration sessions.
//!
//! The shared state here is the **view** of a running simulation: a synchronised
//! camera, per-participant 3-D presence cursors (avatars), scene annotations and
//! a sequence-ordered event log. Concurrent updates are reconciled with
//! last-writer-wins using monotonic per-resource sequence numbers, so two
//! replicas that accept the same set of operations converge to an identical
//! [`CollabSession::state_digest`] regardless of the order in which stale updates
//! were rejected.
//!
//! The digest is a dependency-free FNV-based fingerprint (see
//! [`super::digest_hex`]); no `sha2`/`hex` crates are required.

use super::{Vec3, digest_hex};
use crate::{SimResult, SimulationError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A participant's role in a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ParticipantRole {
    /// Can move their own cursor but not drive a followed camera.
    #[default]
    Viewer,
    /// Drives the shared camera while "follow presenter" is on.
    Presenter,
    /// May add and remove annotations.
    Editor,
}

/// A connected participant (a VR avatar).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollabParticipant {
    /// Stable participant id.
    pub id: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Role in the session.
    pub role: ParticipantRole,
}

impl CollabParticipant {
    /// Creates a participant with the [`ParticipantRole::Viewer`] role.
    #[must_use]
    pub fn new(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            role: ParticipantRole::Viewer,
        }
    }

    /// Builder: sets the role.
    #[must_use]
    pub fn with_role(mut self, role: ParticipantRole) -> Self {
        self.role = role;
        self
    }
}

/// The shared camera pose for the session.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SharedCamera {
    /// Eye position.
    pub position: Vec3,
    /// Look-at target.
    pub target: Vec3,
}

impl SharedCamera {
    /// Creates a shared camera pose.
    #[must_use]
    pub fn new(position: Vec3, target: Vec3) -> Self {
        Self { position, target }
    }
}

impl Default for SharedCamera {
    fn default() -> Self {
        Self::new(Vec3::new(0.0, 0.0, 12.0), Vec3::zero())
    }
}

/// A participant's live 3-D cursor / pointer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresenceCursor {
    /// World-space pointer position.
    pub position: Vec3,
    /// The node the participant is currently hovering, if any.
    pub focused_node: Option<String>,
    /// Per-participant monotonic sequence (for conflict resolution).
    pub sequence: u64,
}

impl PresenceCursor {
    /// Creates a cursor at `position` with the given sequence.
    #[must_use]
    pub fn new(position: Vec3, sequence: u64) -> Self {
        Self {
            position,
            focused_node: None,
            sequence,
        }
    }

    /// Builder: sets the focused node.
    #[must_use]
    pub fn with_focus(mut self, node_id: impl Into<String>) -> Self {
        self.focused_node = Some(node_id.into());
        self
    }
}

/// An annotation pinned to the scene (to a node or a free position).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneAnnotation {
    /// Unique annotation id.
    pub id: String,
    /// Author participant id.
    pub author: String,
    /// Node the annotation is attached to, if any.
    pub node_id: Option<String>,
    /// World-space anchor position.
    pub position: Vec3,
    /// Annotation body text.
    pub text: String,
}

impl SceneAnnotation {
    /// Creates an annotation anchored at a free position.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        author: impl Into<String>,
        position: Vec3,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            author: author.into(),
            node_id: None,
            position,
            text: text.into(),
        }
    }

    /// Builder: anchors the annotation to a node id.
    #[must_use]
    pub fn on_node(mut self, node_id: impl Into<String>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
}

/// A logged session event (each carries the global sequence it was assigned).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollabEvent {
    /// A participant joined.
    Joined { seq: u64, participant: String },
    /// A participant left.
    Left { seq: u64, participant: String },
    /// The presenter was (re)assigned.
    PresenterSet { seq: u64, participant: String },
    /// The shared camera moved.
    CameraMoved { seq: u64, actor: String },
    /// A participant's cursor moved.
    CursorUpdated { seq: u64, actor: String },
    /// An annotation was added.
    AnnotationAdded { seq: u64, id: String },
    /// An annotation was removed.
    AnnotationRemoved { seq: u64, id: String },
    /// "Follow presenter" was toggled.
    FollowToggled { seq: u64, enabled: bool },
}

impl CollabEvent {
    /// The global sequence number assigned to this event.
    #[must_use]
    pub fn sequence(&self) -> u64 {
        match self {
            CollabEvent::Joined { seq, .. }
            | CollabEvent::Left { seq, .. }
            | CollabEvent::PresenterSet { seq, .. }
            | CollabEvent::CameraMoved { seq, .. }
            | CollabEvent::CursorUpdated { seq, .. }
            | CollabEvent::AnnotationAdded { seq, .. }
            | CollabEvent::AnnotationRemoved { seq, .. }
            | CollabEvent::FollowToggled { seq, .. } => *seq,
        }
    }
}

/// A canonical, serialisable snapshot of session state (used for digests/sync).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollabSnapshot {
    /// Session id.
    pub session_id: String,
    /// Participants, sorted by id.
    pub participants: Vec<CollabParticipant>,
    /// Current presenter, if any.
    pub presenter: Option<String>,
    /// Shared camera.
    pub camera: SharedCamera,
    /// Whether follow-presenter is enabled.
    pub follow_presenter: bool,
    /// Cursors, sorted by participant id.
    pub cursors: Vec<(String, PresenceCursor)>,
    /// Annotations, sorted by id.
    pub annotations: Vec<SceneAnnotation>,
}

/// A live collaborative VR exploration session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollabSession {
    session_id: String,
    participants: BTreeMap<String, CollabParticipant>,
    presenter: Option<String>,
    camera: SharedCamera,
    camera_seq: u64,
    follow_presenter: bool,
    follow_seq: u64,
    cursors: BTreeMap<String, PresenceCursor>,
    annotations: BTreeMap<String, SceneAnnotation>,
    log: Vec<CollabEvent>,
    next_seq: u64,
}

impl CollabSession {
    /// Creates an empty session.
    #[must_use]
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            participants: BTreeMap::new(),
            presenter: None,
            camera: SharedCamera::default(),
            camera_seq: 0,
            follow_presenter: false,
            follow_seq: 0,
            cursors: BTreeMap::new(),
            annotations: BTreeMap::new(),
            log: Vec::new(),
            next_seq: 1,
        }
    }

    /// Allocates the next global sequence number.
    fn bump(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }

    /// The session id.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.session_id
    }

    /// Number of connected participants.
    #[must_use]
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }

    /// The current presenter id, if any.
    #[must_use]
    pub fn presenter(&self) -> Option<&str> {
        self.presenter.as_deref()
    }

    /// The current shared camera.
    #[must_use]
    pub fn camera(&self) -> SharedCamera {
        self.camera
    }

    /// Whether follow-presenter mode is enabled.
    #[must_use]
    pub fn is_following(&self) -> bool {
        self.follow_presenter
    }

    /// The recorded event log.
    #[must_use]
    pub fn log(&self) -> &[CollabEvent] {
        &self.log
    }

    /// A participant's current cursor, if present.
    #[must_use]
    pub fn cursor(&self, participant: &str) -> Option<&PresenceCursor> {
        self.cursors.get(participant)
    }

    /// All annotations, sorted by id.
    #[must_use]
    pub fn annotations(&self) -> Vec<SceneAnnotation> {
        self.annotations.values().cloned().collect()
    }

    /// Validates that `participant` is connected.
    fn require_participant(&self, participant: &str) -> SimResult<()> {
        if self.participants.contains_key(participant) {
            Ok(())
        } else {
            Err(SimulationError::InvalidParameter(format!(
                "unknown participant '{participant}'"
            )))
        }
    }

    /// Adds a participant.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if the id is already present.
    pub fn join(&mut self, participant: CollabParticipant) -> SimResult<()> {
        if self.participants.contains_key(&participant.id) {
            return Err(SimulationError::InvalidParameter(format!(
                "participant '{}' already joined",
                participant.id
            )));
        }
        let id = participant.id.clone();
        let is_presenter = participant.role == ParticipantRole::Presenter;
        self.participants.insert(id.clone(), participant);
        if is_presenter && self.presenter.is_none() {
            self.presenter = Some(id.clone());
        }
        let seq = self.bump();
        self.log.push(CollabEvent::Joined {
            seq,
            participant: id,
        });
        Ok(())
    }

    /// Removes a participant and their cursor. Clears the presenter slot if it was
    /// them. Returns `true` if a participant was removed.
    pub fn leave(&mut self, participant: &str) -> bool {
        if self.participants.remove(participant).is_none() {
            return false;
        }
        self.cursors.remove(participant);
        if self.presenter.as_deref() == Some(participant) {
            self.presenter = None;
        }
        let seq = self.bump();
        self.log.push(CollabEvent::Left {
            seq,
            participant: participant.to_string(),
        });
        true
    }

    /// Assigns the presenter.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if `participant` is not
    /// connected.
    pub fn set_presenter(&mut self, participant: &str) -> SimResult<()> {
        self.require_participant(participant)?;
        self.presenter = Some(participant.to_string());
        let seq = self.bump();
        self.log.push(CollabEvent::PresenterSet {
            seq,
            participant: participant.to_string(),
        });
        Ok(())
    }

    /// Enables or disables follow-presenter mode (last-writer-wins on
    /// `client_seq`).
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if `client_seq` is not newer
    /// than the last accepted toggle (a stale/conflicting update).
    pub fn set_follow(&mut self, enabled: bool, client_seq: u64) -> SimResult<()> {
        if client_seq <= self.follow_seq {
            return Err(SimulationError::InvalidParameter(format!(
                "stale follow toggle: {client_seq} <= {}",
                self.follow_seq
            )));
        }
        self.follow_presenter = enabled;
        self.follow_seq = client_seq;
        let seq = self.bump();
        self.log.push(CollabEvent::FollowToggled { seq, enabled });
        Ok(())
    }

    /// Moves the shared camera.
    ///
    /// When follow-presenter is on and a presenter is set, only the presenter may
    /// move the camera. Updates are last-writer-wins on `client_seq`.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if the actor is unknown,
    /// lacks permission, or `client_seq` is stale.
    pub fn move_camera(
        &mut self,
        actor: &str,
        camera: SharedCamera,
        client_seq: u64,
    ) -> SimResult<()> {
        self.require_participant(actor)?;
        if self.follow_presenter
            && let Some(presenter) = self.presenter.as_deref()
            && presenter != actor
        {
            return Err(SimulationError::InvalidParameter(format!(
                "only presenter '{presenter}' may move the followed camera"
            )));
        }
        if client_seq <= self.camera_seq {
            return Err(SimulationError::InvalidParameter(format!(
                "stale camera update: {client_seq} <= {}",
                self.camera_seq
            )));
        }
        self.camera = camera;
        self.camera_seq = client_seq;
        let seq = self.bump();
        self.log.push(CollabEvent::CameraMoved {
            seq,
            actor: actor.to_string(),
        });
        Ok(())
    }

    /// Updates a participant's cursor (last-writer-wins on the cursor's own
    /// `sequence`).
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if the actor is unknown or
    /// the cursor sequence is stale.
    pub fn update_cursor(&mut self, actor: &str, cursor: PresenceCursor) -> SimResult<()> {
        self.require_participant(actor)?;
        if let Some(existing) = self.cursors.get(actor)
            && cursor.sequence <= existing.sequence
        {
            return Err(SimulationError::InvalidParameter(format!(
                "stale cursor update for '{actor}': {} <= {}",
                cursor.sequence, existing.sequence
            )));
        }
        self.cursors.insert(actor.to_string(), cursor);
        let seq = self.bump();
        self.log.push(CollabEvent::CursorUpdated {
            seq,
            actor: actor.to_string(),
        });
        Ok(())
    }

    /// Adds an annotation.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::InvalidParameter`] if the author is unknown or
    /// the annotation id already exists.
    pub fn add_annotation(&mut self, annotation: SceneAnnotation) -> SimResult<()> {
        self.require_participant(&annotation.author)?;
        if self.annotations.contains_key(&annotation.id) {
            return Err(SimulationError::InvalidParameter(format!(
                "annotation '{}' already exists",
                annotation.id
            )));
        }
        let id = annotation.id.clone();
        self.annotations.insert(id.clone(), annotation);
        let seq = self.bump();
        self.log.push(CollabEvent::AnnotationAdded { seq, id });
        Ok(())
    }

    /// Removes an annotation by id. Returns `true` if one was removed.
    pub fn remove_annotation(&mut self, id: &str) -> bool {
        if self.annotations.remove(id).is_none() {
            return false;
        }
        let seq = self.bump();
        self.log.push(CollabEvent::AnnotationRemoved {
            seq,
            id: id.to_string(),
        });
        true
    }

    /// Produces a canonical, deterministically-ordered snapshot of state.
    #[must_use]
    pub fn snapshot(&self) -> CollabSnapshot {
        CollabSnapshot {
            session_id: self.session_id.clone(),
            participants: self.participants.values().cloned().collect(),
            presenter: self.presenter.clone(),
            camera: self.camera,
            follow_presenter: self.follow_presenter,
            cursors: self
                .cursors
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            annotations: self.annotations.values().cloned().collect(),
        }
    }

    /// A deterministic FNV-based hex digest of the canonical state snapshot.
    ///
    /// Two sessions that have accepted the same set of operations produce the
    /// same digest regardless of the order in which conflicting (stale) updates
    /// were rejected — the basis for cheap replica conflict detection / sync.
    ///
    /// # Errors
    ///
    /// Returns [`SimulationError::Serialization`] if the snapshot cannot be
    /// serialised.
    pub fn state_digest(&self) -> SimResult<String> {
        let json = serde_json::to_vec(&self.snapshot())?;
        Ok(digest_hex(&json))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session_with_two() -> CollabSession {
        let mut s = CollabSession::new("sess-1");
        s.join(CollabParticipant::new("alice", "Alice").with_role(ParticipantRole::Presenter))
            .expect("alice joins");
        s.join(CollabParticipant::new("bob", "Bob"))
            .expect("bob joins");
        s
    }

    #[test]
    fn test_join_leave_and_presenter() {
        let mut s = session_with_two();
        assert_eq!(s.participant_count(), 2);
        assert_eq!(s.presenter(), Some("alice"));
        assert!(s.join(CollabParticipant::new("alice", "Dup")).is_err());
        assert!(s.leave("alice"));
        assert_eq!(s.participant_count(), 1);
        assert_eq!(s.presenter(), None);
        assert!(!s.leave("ghost"));
    }

    #[test]
    fn test_camera_move_last_writer_wins() {
        let mut s = session_with_two();
        let cam = SharedCamera::new(Vec3::new(1.0, 1.0, 1.0), Vec3::zero());
        s.move_camera("alice", cam, 5).expect("first move");
        assert!(s.move_camera("alice", cam, 5).is_err());
        assert!(s.move_camera("alice", cam, 3).is_err());
        let cam2 = SharedCamera::new(Vec3::new(2.0, 2.0, 2.0), Vec3::zero());
        s.move_camera("alice", cam2, 6).expect("newer move");
        assert_eq!(s.camera(), cam2);
    }

    #[test]
    fn test_follow_presenter_restricts_camera() {
        let mut s = session_with_two();
        s.set_follow(true, 1).expect("enable follow");
        assert!(s.is_following());
        let cam = SharedCamera::new(Vec3::new(1.0, 0.0, 0.0), Vec3::zero());
        // Bob (not presenter) cannot move the followed camera.
        assert!(s.move_camera("bob", cam, 1).is_err());
        // Alice (presenter) can.
        s.move_camera("alice", cam, 1).expect("presenter moves");
        // Stale follow toggle rejected.
        assert!(s.set_follow(false, 1).is_err());
    }

    #[test]
    fn test_cursor_and_annotation_lifecycle() {
        let mut s = session_with_two();
        assert!(
            s.update_cursor("mallory", PresenceCursor::new(Vec3::zero(), 1))
                .is_err()
        );
        s.update_cursor("bob", PresenceCursor::new(Vec3::new(1.0, 0.0, 0.0), 2))
            .expect("first cursor");
        assert!(
            s.update_cursor("bob", PresenceCursor::new(Vec3::zero(), 1))
                .is_err()
        );
        s.update_cursor(
            "bob",
            PresenceCursor::new(Vec3::new(2.0, 0.0, 0.0), 3).with_focus("statute::tax"),
        )
        .expect("newer cursor");
        assert_eq!(
            s.cursor("bob").and_then(|c| c.focused_node.clone()),
            Some("statute::tax".to_string())
        );

        let ann = SceneAnnotation::new("a1", "bob", Vec3::zero(), "note").on_node("origin");
        s.add_annotation(ann.clone()).expect("add");
        assert!(s.add_annotation(ann).is_err());
        assert!(
            s.add_annotation(SceneAnnotation::new("a2", "ghost", Vec3::zero(), "x"))
                .is_err()
        );
        assert_eq!(s.annotations().len(), 1);
        assert!(s.remove_annotation("a1"));
        assert!(!s.remove_annotation("a1"));
    }

    #[test]
    fn test_event_log_grows_with_sequence() {
        let mut s = session_with_two();
        s.set_presenter("bob").expect("set presenter");
        let log = s.log();
        // 2 joins + 1 presenter-set = 3 events, sequences strictly increasing.
        assert_eq!(log.len(), 3);
        let seqs: Vec<u64> = log.iter().map(CollabEvent::sequence).collect();
        assert!(seqs.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn test_state_digest_deterministic_and_sorted_snapshot() {
        let s1 = session_with_two();
        let s2 = session_with_two();
        assert_eq!(s1.state_digest().unwrap(), s2.state_digest().unwrap());
        let mut s = CollabSession::new("s");
        s.join(CollabParticipant::new("zoe", "Zoe")).unwrap();
        s.join(CollabParticipant::new("amy", "Amy")).unwrap();
        let snap = s.snapshot();
        let ids: Vec<&str> = snap.participants.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, vec!["amy", "zoe"]);
    }

    #[test]
    fn test_replicas_converge_after_conflict() {
        // Two replicas accept the same winning operations but in different order
        // w.r.t. rejected stale updates; final digests must match.
        let mut a = session_with_two();
        let mut b = session_with_two();
        let cam_lo = SharedCamera::new(Vec3::new(1.0, 0.0, 0.0), Vec3::zero());
        let cam_hi = SharedCamera::new(Vec3::new(9.0, 0.0, 0.0), Vec3::zero());

        // Replica A: winner first, then stale (rejected).
        a.move_camera("alice", cam_hi, 10).expect("hi");
        assert!(a.move_camera("alice", cam_lo, 5).is_err());

        // Replica B: stale first (accepted), then winner (overwrites).
        b.move_camera("alice", cam_lo, 5).expect("lo");
        b.move_camera("alice", cam_hi, 10).expect("hi");

        assert_eq!(a.camera(), cam_hi);
        assert_eq!(b.camera(), cam_hi);
        assert_eq!(a.state_digest().unwrap(), b.state_digest().unwrap());
    }
}
