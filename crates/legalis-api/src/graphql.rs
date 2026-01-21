//! GraphQL API for Legalis-RS.
//!
//! This module provides a GraphQL interface for querying and mutating statutes,
//! running verifications, and managing the statute registry.

use async_graphql::{Context, FieldResult, Object, Schema, SimpleObject, Subscription};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use futures::{Stream, StreamExt};
use legalis_core::{Effect, EffectType, Statute};
use legalis_dsl::LegalDslParser;
use legalis_verifier::StatuteVerifier;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_stream::wrappers::BroadcastStream;

use crate::auth::{AuthUser, Permission};
use crate::websocket::{WsBroadcaster, WsNotification};

/// GraphQL schema type.
pub type LegalisSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

/// Application state for GraphQL context.
#[derive(Clone)]
pub struct GraphQLState {
    pub statutes: Arc<RwLock<Vec<Statute>>>,
    pub ws_broadcaster: WsBroadcaster,
}

impl GraphQLState {
    pub fn new() -> Self {
        Self {
            statutes: Arc::new(RwLock::new(Vec::new())),
            ws_broadcaster: WsBroadcaster::new(),
        }
    }

    pub fn with_broadcaster(ws_broadcaster: WsBroadcaster) -> Self {
        Self {
            statutes: Arc::new(RwLock::new(Vec::new())),
            ws_broadcaster,
        }
    }
}

impl Default for GraphQLState {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to check permissions in GraphQL resolvers.
/// Returns an error if the user doesn't have the required permission.
fn check_permission(ctx: &Context<'_>, permission: Permission) -> FieldResult<()> {
    if let Ok(auth_user) = ctx.data::<AuthUser>()
        && auth_user.has_permission(permission)
    {
        return Ok(());
    }
    Err("Insufficient permissions".into())
}

/// GraphQL representation of a Statute.
#[derive(SimpleObject, Clone)]
pub struct StatuteObject {
    /// Unique identifier
    pub id: String,
    /// Title of the statute
    pub title: String,
    /// Version number
    pub version: i32,
    /// Jurisdiction identifier
    pub jurisdiction: Option<String>,
    /// Effect description
    pub effect_description: String,
    /// Effect type
    pub effect_type: String,
    /// Number of preconditions
    pub precondition_count: i32,
    /// Has discretion logic
    pub has_discretion: bool,
}

impl From<&Statute> for StatuteObject {
    fn from(statute: &Statute) -> Self {
        Self {
            id: statute.id.clone(),
            title: statute.title.clone(),
            version: statute.version as i32,
            jurisdiction: statute.jurisdiction.clone(),
            effect_description: statute.effect.description.clone(),
            effect_type: format!("{:?}", statute.effect.effect_type),
            precondition_count: statute.preconditions.len() as i32,
            has_discretion: statute.discretion_logic.is_some(),
        }
    }
}

/// Verification result.
#[derive(SimpleObject)]
pub struct VerificationResult {
    /// Whether verification passed
    pub passed: bool,
    /// Error messages
    pub errors: Vec<String>,
    /// Warning messages
    pub warnings: Vec<String>,
    /// Suggestions
    pub suggestions: Vec<String>,
}

/// Relay-style connection for paginated results.
#[derive(SimpleObject)]
pub struct StatuteConnection {
    /// Edges containing nodes and cursors
    pub edges: Vec<StatuteEdge>,
    /// Page information
    pub page_info: PageInfo,
    /// Total count of items
    pub total_count: i32,
}

/// Edge in a relay-style connection.
#[derive(SimpleObject)]
pub struct StatuteEdge {
    /// The node (statute)
    pub node: StatuteObject,
    /// Cursor for this node
    pub cursor: String,
}

/// Page information for cursor-based pagination.
#[derive(SimpleObject)]
pub struct PageInfo {
    /// Whether there is a next page
    pub has_next_page: bool,
    /// Whether there is a previous page
    pub has_previous_page: bool,
    /// Cursor of the first item
    pub start_cursor: Option<String>,
    /// Cursor of the last item
    pub end_cursor: Option<String>,
}

/// Query root.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get all statutes.
    #[graphql(complexity = 10)]
    async fn statutes(&self, ctx: &Context<'_>) -> FieldResult<Vec<StatuteObject>> {
        let state = ctx.data::<GraphQLState>()?;
        let statutes = state.statutes.read().await;
        Ok(statutes.iter().map(StatuteObject::from).collect())
    }

    /// Get a statute by ID.
    async fn statute(&self, ctx: &Context<'_>, id: String) -> FieldResult<Option<StatuteObject>> {
        let state = ctx.data::<GraphQLState>()?;
        let statutes = state.statutes.read().await;
        Ok(statutes
            .iter()
            .find(|s| s.id == id)
            .map(StatuteObject::from))
    }

    /// Search statutes by title.
    #[graphql(complexity = 15)]
    async fn search_statutes(
        &self,
        ctx: &Context<'_>,
        query: String,
    ) -> FieldResult<Vec<StatuteObject>> {
        let state = ctx.data::<GraphQLState>()?;
        let statutes = state.statutes.read().await;
        Ok(statutes
            .iter()
            .filter(|s| {
                s.title.to_lowercase().contains(&query.to_lowercase())
                    || s.id.to_lowercase().contains(&query.to_lowercase())
            })
            .map(StatuteObject::from)
            .collect())
    }

    /// Get statutes by jurisdiction.
    async fn statutes_by_jurisdiction(
        &self,
        ctx: &Context<'_>,
        jurisdiction: String,
    ) -> FieldResult<Vec<StatuteObject>> {
        let state = ctx.data::<GraphQLState>()?;
        let statutes = state.statutes.read().await;
        Ok(statutes
            .iter()
            .filter(|s| s.jurisdiction.as_ref() == Some(&jurisdiction))
            .map(StatuteObject::from)
            .collect())
    }

    /// Verify statutes.
    async fn verify_statutes(
        &self,
        ctx: &Context<'_>,
        statute_ids: Vec<String>,
    ) -> FieldResult<VerificationResult> {
        let state = ctx.data::<GraphQLState>()?;
        let statutes_lock = state.statutes.read().await;

        let statutes_to_verify: Vec<_> = statutes_lock
            .iter()
            .filter(|s| statute_ids.contains(&s.id))
            .cloned()
            .collect();

        if statutes_to_verify.is_empty() {
            return Err("No statutes found with provided IDs".into());
        }

        let verifier = StatuteVerifier::new();
        let result = verifier.verify(&statutes_to_verify);

        Ok(VerificationResult {
            passed: result.passed,
            errors: result.errors.iter().map(|e| e.to_string()).collect(),
            warnings: result.warnings,
            suggestions: result.suggestions,
        })
    }

    /// Get statute count.
    async fn statute_count(&self, ctx: &Context<'_>) -> FieldResult<i32> {
        let state = ctx.data::<GraphQLState>()?;
        let statutes = state.statutes.read().await;
        Ok(statutes.len() as i32)
    }

    /// Get statutes with relay-style cursor pagination.
    #[graphql(complexity = 10)]
    async fn statutes_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> FieldResult<StatuteConnection> {
        let state = ctx.data::<GraphQLState>()?;
        let statutes = state.statutes.read().await;

        // Convert statutes to a vec for easier processing
        let all_statutes: Vec<_> = statutes.iter().collect();
        let total_count = all_statutes.len() as i32;

        // Decode cursors (cursors are base64-encoded indices)
        let after_idx = after
            .as_ref()
            .and_then(|c| BASE64.decode(c).ok())
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|s| s.parse::<usize>().ok());

        let before_idx = before
            .as_ref()
            .and_then(|c| BASE64.decode(c).ok())
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|s| s.parse::<usize>().ok());

        // Determine the slice of items to return
        let mut start = after_idx.map(|i| i + 1).unwrap_or(0);
        let mut end = before_idx.unwrap_or(all_statutes.len());

        // Apply first/last limits
        if let Some(first_count) = first {
            end = std::cmp::min(end, start + first_count as usize);
        }
        if let Some(last_count) = last {
            start = std::cmp::max(start, end.saturating_sub(last_count as usize));
        }

        // Build edges
        let edges: Vec<StatuteEdge> = all_statutes[start..end]
            .iter()
            .enumerate()
            .map(|(offset, statute)| {
                let idx = start + offset;
                let cursor = BASE64.encode(idx.to_string());
                StatuteEdge {
                    node: StatuteObject::from(*statute),
                    cursor,
                }
            })
            .collect();

        // Build page info
        let has_previous_page = start > 0;
        let has_next_page = end < all_statutes.len();
        let start_cursor = edges.first().map(|e| e.cursor.clone());
        let end_cursor = edges.last().map(|e| e.cursor.clone());

        Ok(StatuteConnection {
            edges,
            page_info: PageInfo {
                has_next_page,
                has_previous_page,
                start_cursor,
                end_cursor,
            },
            total_count,
        })
    }
}

/// Input type for creating a statute.
#[derive(async_graphql::InputObject)]
pub struct CreateStatuteInput {
    /// Statute ID
    pub id: String,
    /// Statute title
    pub title: String,
    /// Effect description
    pub effect_description: String,
    /// Effect type (Grant, Revoke, Obligation, Prohibition)
    pub effect_type: String,
    /// Jurisdiction
    pub jurisdiction: Option<String>,
    /// Version
    pub version: Option<i32>,
}

/// Input type for updating a statute.
#[derive(async_graphql::InputObject)]
pub struct UpdateStatuteInput {
    /// Statute ID
    pub id: String,
    /// New title (optional)
    pub title: Option<String>,
    /// New jurisdiction (optional)
    pub jurisdiction: Option<String>,
    /// New version (optional)
    pub version: Option<i32>,
}

/// Mutation root.
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new statute.
    /// Requires CreateStatutes permission.
    async fn create_statute(
        &self,
        ctx: &Context<'_>,
        input: CreateStatuteInput,
    ) -> FieldResult<StatuteObject> {
        // Check permissions
        check_permission(ctx, Permission::CreateStatutes)?;

        let state = ctx.data::<GraphQLState>()?;
        let mut statutes = state.statutes.write().await;

        // Check if statute already exists
        if statutes.iter().any(|s| s.id == input.id) {
            return Err(format!("Statute with ID '{}' already exists", input.id).into());
        }

        // Parse effect type
        let effect_type = match input.effect_type.to_lowercase().as_str() {
            "grant" => EffectType::Grant,
            "revoke" => EffectType::Revoke,
            "obligation" => EffectType::Obligation,
            "prohibition" => EffectType::Prohibition,
            _ => {
                return Err(format!("Invalid effect type: {}", input.effect_type).into());
            }
        };

        // Create statute
        let mut statute = Statute::new(
            input.id.clone(),
            input.title.clone(),
            Effect::new(effect_type, input.effect_description),
        );

        if let Some(jur) = input.jurisdiction {
            statute = statute.with_jurisdiction(jur);
        }

        if let Some(ver) = input.version {
            statute = statute.with_version(ver as u32);
        }

        let statute_obj = StatuteObject::from(&statute);
        let statute_id = statute.id.clone();
        let statute_title = statute.title.clone();
        statutes.push(statute);
        drop(statutes);

        // Broadcast WebSocket notification
        let user_id = ctx
            .data::<AuthUser>()
            .map(|u| u.username.clone())
            .unwrap_or_else(|_| "system".to_string());
        state
            .ws_broadcaster
            .broadcast(WsNotification::StatuteCreated {
                statute_id,
                title: statute_title,
                created_by: user_id,
            });

        Ok(statute_obj)
    }

    /// Update an existing statute.
    /// Requires UpdateStatutes permission.
    async fn update_statute(
        &self,
        ctx: &Context<'_>,
        input: UpdateStatuteInput,
    ) -> FieldResult<StatuteObject> {
        // Check permissions
        check_permission(ctx, Permission::UpdateStatutes)?;

        let state = ctx.data::<GraphQLState>()?;
        let mut statutes = state.statutes.write().await;

        // Find statute
        let statute = statutes
            .iter_mut()
            .find(|s| s.id == input.id)
            .ok_or_else(|| format!("Statute with ID '{}' not found", input.id))?;

        // Update fields
        if let Some(title) = input.title {
            statute.title = title;
        }

        if let Some(jurisdiction) = input.jurisdiction {
            statute.jurisdiction = Some(jurisdiction);
        }

        if let Some(version) = input.version {
            statute.version = version as u32;
        }

        let statute_obj = StatuteObject::from(&*statute);
        let statute_id = statute.id.clone();
        let statute_title = statute.title.clone();
        drop(statutes);

        // Broadcast WebSocket notification
        let user_id = ctx
            .data::<AuthUser>()
            .map(|u| u.username.clone())
            .unwrap_or_else(|_| "system".to_string());
        state
            .ws_broadcaster
            .broadcast(WsNotification::StatuteUpdated {
                statute_id,
                title: statute_title,
                updated_by: user_id,
            });

        Ok(statute_obj)
    }

    /// Delete a statute.
    /// Requires DeleteStatutes permission.
    async fn delete_statute(&self, ctx: &Context<'_>, id: String) -> FieldResult<bool> {
        // Check permissions
        check_permission(ctx, Permission::DeleteStatutes)?;

        let state = ctx.data::<GraphQLState>()?;
        let mut statutes = state.statutes.write().await;

        let initial_len = statutes.len();
        statutes.retain(|s| s.id != id);
        let deleted = statutes.len() < initial_len;
        drop(statutes);

        // Broadcast WebSocket notification if deleted
        if deleted {
            let user_id = ctx
                .data::<AuthUser>()
                .map(|u| u.username.clone())
                .unwrap_or_else(|_| "system".to_string());
            state
                .ws_broadcaster
                .broadcast(WsNotification::StatuteDeleted {
                    statute_id: id,
                    deleted_by: user_id,
                });
        }

        Ok(deleted)
    }

    /// Parse and create a statute from DSL.
    async fn parse_statute_dsl(
        &self,
        ctx: &Context<'_>,
        dsl: String,
    ) -> FieldResult<StatuteObject> {
        let state = ctx.data::<GraphQLState>()?;
        let mut statutes = state.statutes.write().await;

        let parser = LegalDslParser::new();
        let statute = parser
            .parse_statute(&dsl)
            .map_err(|e| format!("Parse error: {}", e))?;

        // Check if statute already exists
        if statutes.iter().any(|s| s.id == statute.id) {
            return Err(format!("Statute with ID '{}' already exists", statute.id).into());
        }

        let statute_obj = StatuteObject::from(&statute);
        statutes.push(statute);

        Ok(statute_obj)
    }

    /// Clear all statutes (use with caution!).
    async fn clear_statutes(&self, ctx: &Context<'_>) -> FieldResult<i32> {
        let state = ctx.data::<GraphQLState>()?;
        let mut statutes = state.statutes.write().await;

        let count = statutes.len() as i32;
        statutes.clear();

        Ok(count)
    }
}

/// GraphQL representation of a notification event.
#[derive(SimpleObject, Clone)]
pub struct NotificationEvent {
    /// Event type
    pub event_type: String,
    /// Event message
    pub message: String,
    /// Related statute ID (if applicable)
    pub statute_id: Option<String>,
    /// Timestamp
    pub timestamp: String,
}

impl From<WsNotification> for NotificationEvent {
    fn from(notif: WsNotification) -> Self {
        match notif {
            WsNotification::StatuteCreated {
                statute_id,
                title,
                created_by,
            } => Self {
                event_type: "statute_created".to_string(),
                message: format!("Statute '{}' created by {}", title, created_by),
                statute_id: Some(statute_id),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            WsNotification::StatuteUpdated {
                statute_id,
                title,
                updated_by,
            } => Self {
                event_type: "statute_updated".to_string(),
                message: format!("Statute '{}' updated by {}", title, updated_by),
                statute_id: Some(statute_id),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            WsNotification::StatuteDeleted {
                statute_id,
                deleted_by,
            } => Self {
                event_type: "statute_deleted".to_string(),
                message: format!("Statute '{}' deleted by {}", statute_id, deleted_by),
                statute_id: Some(statute_id),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            WsNotification::VerificationCompleted {
                job_id,
                passed,
                errors_count,
                warnings_count,
            } => Self {
                event_type: "verification_completed".to_string(),
                message: format!(
                    "Verification job {} completed: {} ({} errors, {} warnings)",
                    job_id,
                    if passed { "PASSED" } else { "FAILED" },
                    errors_count,
                    warnings_count
                ),
                statute_id: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            WsNotification::SimulationCompleted {
                simulation_id,
                total_entities,
                deterministic_rate,
                discretionary_rate,
                void_rate,
            } => Self {
                event_type: "simulation_completed".to_string(),
                message: format!(
                    "Simulation {} completed: {} entities (det: {:.1}%, disc: {:.1}%, void: {:.1}%)",
                    simulation_id,
                    total_entities,
                    deterministic_rate,
                    discretionary_rate,
                    void_rate
                ),
                statute_id: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            WsNotification::SystemStatus { status, message } => Self {
                event_type: "system_status".to_string(),
                message: format!("System status [{}]: {}", status, message),
                statute_id: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            WsNotification::EditOperation {
                document_id,
                operation: _,
                version,
                session_id,
            } => Self {
                event_type: "edit_operation".to_string(),
                message: format!(
                    "Edit operation on document {} (v{}) by session {}",
                    document_id, version, session_id
                ),
                statute_id: Some(document_id),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            WsNotification::EditConflict {
                document_id,
                conflict,
            } => Self {
                event_type: "edit_conflict".to_string(),
                message: format!(
                    "Edit conflict detected on document {}: {}",
                    document_id, conflict.description
                ),
                statute_id: Some(document_id),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            WsNotification::PresenceUpdate {
                resource_id,
                user_id: _,
                username,
                activity,
            } => Self {
                event_type: "presence_update".to_string(),
                message: format!("{} is {} {}", username, activity, resource_id),
                statute_id: Some(resource_id),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        }
    }
}

/// Subscription root.
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    /// Subscribe to all notifications.
    async fn notifications(&self, ctx: &Context<'_>) -> impl Stream<Item = NotificationEvent> {
        let state = ctx.data::<GraphQLState>().expect("GraphQL state not found");
        let rx = state.ws_broadcaster.subscribe();

        BroadcastStream::new(rx)
            .filter_map(|result| async move { result.ok().map(NotificationEvent::from) })
    }

    /// Subscribe to statute events (created, updated, deleted).
    async fn statute_events(&self, ctx: &Context<'_>) -> impl Stream<Item = NotificationEvent> {
        let state = ctx.data::<GraphQLState>().expect("GraphQL state not found");
        let rx = state.ws_broadcaster.subscribe();

        BroadcastStream::new(rx).filter_map(|result| async move {
            result.ok().and_then(|notif| match &notif {
                WsNotification::StatuteCreated { .. }
                | WsNotification::StatuteUpdated { .. }
                | WsNotification::StatuteDeleted { .. } => Some(NotificationEvent::from(notif)),
                _ => None,
            })
        })
    }

    /// Subscribe to verification events.
    async fn verification_events(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = NotificationEvent> {
        let state = ctx.data::<GraphQLState>().expect("GraphQL state not found");
        let rx = state.ws_broadcaster.subscribe();

        BroadcastStream::new(rx).filter_map(|result| async move {
            result.ok().and_then(|notif| match &notif {
                WsNotification::VerificationCompleted { .. } => {
                    Some(NotificationEvent::from(notif))
                }
                _ => None,
            })
        })
    }

    /// Subscribe to simulation events.
    async fn simulation_events(&self, ctx: &Context<'_>) -> impl Stream<Item = NotificationEvent> {
        let state = ctx.data::<GraphQLState>().expect("GraphQL state not found");
        let rx = state.ws_broadcaster.subscribe();

        BroadcastStream::new(rx).filter_map(|result| async move {
            result.ok().and_then(|notif| match &notif {
                WsNotification::SimulationCompleted { .. } => Some(NotificationEvent::from(notif)),
                _ => None,
            })
        })
    }
}

/// Creates a new GraphQL schema with subscription support, query complexity limiting,
/// and depth limiting for security.
/// TODO: Add DataLoader support for N+1 optimization (requires trait signature fixes)
pub fn create_schema(state: GraphQLState) -> LegalisSchema {
    Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(state)
        .limit_complexity(1000) // Maximum query complexity score
        .limit_depth(15) // Maximum query depth
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_statute() {
        let state = GraphQLState::new();
        let schema = create_schema(state.clone());

        // Create an admin user for testing
        use crate::auth::{AuthMethod, AuthUser, Role};
        use uuid::Uuid;
        let admin_user = AuthUser::new(
            Uuid::new_v4(),
            "admin".to_string(),
            Role::Admin,
            AuthMethod::Jwt,
        );

        let query = r#"
            mutation {
                createStatute(input: {
                    id: "test-statute"
                    title: "Test Statute"
                    effectDescription: "Grant benefit"
                    effectType: "Grant"
                    jurisdiction: "US"
                    version: 1
                }) {
                    id
                    title
                    version
                }
            }
        "#;

        let request = async_graphql::Request::new(query).data(admin_user);
        let result = schema.execute(request).await;
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_query_statutes() {
        let state = GraphQLState::new();

        // Add a test statute
        {
            let mut statutes = state.statutes.write().await;
            statutes.push(
                Statute::new(
                    "test-1",
                    "Test Statute 1",
                    Effect::new(EffectType::Grant, "Test benefit"),
                )
                .with_jurisdiction("US"),
            );
        }

        let schema = create_schema(state.clone());

        let query = r#"
            {
                statutes {
                    id
                    title
                    jurisdiction
                }
            }
        "#;

        let result = schema.execute(query).await;
        assert!(result.errors.is_empty());
    }
}
