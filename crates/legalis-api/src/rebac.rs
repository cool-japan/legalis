//! Relationship-Based Access Control (ReBAC) module.
//!
//! Provides fine-grained access control based on relationships between users and resources.
//! Implements a Zanzibar-inspired model for scalable authorization.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Resource type in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    /// Statute resource
    Statute,
    /// Simulation resource
    Simulation,
    /// Verification report
    VerificationReport,
    /// User resource
    User,
    /// Organization/team
    Organization,
    /// API key
    ApiKey,
}

/// Relationship between a user and a resource.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Relation {
    /// Owner of the resource
    Owner,
    /// Editor with modify permissions
    Editor,
    /// Viewer with read-only access
    Viewer,
    /// Administrator of the resource
    Admin,
    /// Member of an organization/team
    Member,
    /// Parent relationship (for hierarchical resources)
    Parent,
}

/// A tuple representing a relationship: (user, relation, resource).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelationTuple {
    /// User ID (subject)
    pub user_id: Uuid,
    /// Relationship type
    pub relation: Relation,
    /// Resource type
    pub resource_type: ResourceType,
    /// Resource ID (object)
    pub resource_id: Uuid,
}

impl RelationTuple {
    /// Creates a new relation tuple.
    pub fn new(
        user_id: Uuid,
        relation: Relation,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) -> Self {
        Self {
            user_id,
            relation,
            resource_type,
            resource_id,
        }
    }
}

/// Action that can be performed on a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    /// Read/view the resource
    Read,
    /// Create a new resource
    Create,
    /// Update/edit the resource
    Update,
    /// Delete the resource
    Delete,
    /// Share the resource with others
    Share,
    /// Manage permissions on the resource
    ManagePermissions,
}

/// Authorization policy engine using ReBAC.
///
/// This stores relationship tuples and evaluates access based on them.
pub struct ReBACEngine {
    /// Storage of relationship tuples
    tuples: HashSet<RelationTuple>,
    /// Cache for computed permissions (user_id, resource_type, resource_id) -> permissions
    permission_cache: HashMap<(Uuid, ResourceType, Uuid), HashSet<Action>>,
}

impl ReBACEngine {
    /// Creates a new ReBAC engine.
    pub fn new() -> Self {
        Self {
            tuples: HashSet::new(),
            permission_cache: HashMap::new(),
        }
    }

    /// Adds a relationship tuple.
    pub fn add_tuple(&mut self, tuple: RelationTuple) {
        self.tuples.insert(tuple.clone());
        // Invalidate cache for this user/resource combination
        self.permission_cache
            .remove(&(tuple.user_id, tuple.resource_type, tuple.resource_id));
    }

    /// Removes a relationship tuple.
    pub fn remove_tuple(&mut self, tuple: &RelationTuple) -> bool {
        let removed = self.tuples.remove(tuple);
        if removed {
            self.permission_cache
                .remove(&(tuple.user_id, tuple.resource_type, tuple.resource_id));
        }
        removed
    }

    /// Checks if a user has a specific relationship with a resource.
    pub fn has_relation(
        &self,
        user_id: Uuid,
        relation: Relation,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) -> bool {
        let tuple = RelationTuple::new(user_id, relation, resource_type, resource_id);
        self.tuples.contains(&tuple)
    }

    /// Gets all relationships a user has with a specific resource.
    pub fn get_user_relations(
        &self,
        user_id: Uuid,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) -> HashSet<Relation> {
        self.tuples
            .iter()
            .filter(|t| {
                t.user_id == user_id
                    && t.resource_type == resource_type
                    && t.resource_id == resource_id
            })
            .map(|t| t.relation.clone())
            .collect()
    }

    /// Gets all resources of a type that a user has a specific relation with.
    pub fn get_user_resources(
        &self,
        user_id: Uuid,
        relation: Relation,
        resource_type: ResourceType,
    ) -> HashSet<Uuid> {
        self.tuples
            .iter()
            .filter(|t| {
                t.user_id == user_id && t.relation == relation && t.resource_type == resource_type
            })
            .map(|t| t.resource_id)
            .collect()
    }

    /// Checks if a user can perform an action on a resource.
    pub fn check_permission(
        &mut self,
        user_id: Uuid,
        action: Action,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) -> bool {
        // Check cache first
        let cache_key = (user_id, resource_type, resource_id);
        if let Some(cached_actions) = self.permission_cache.get(&cache_key) {
            return cached_actions.contains(&action);
        }

        // Compute permissions
        let permissions = self.compute_permissions(user_id, resource_type, resource_id);
        let result = permissions.contains(&action);

        // Cache result
        self.permission_cache.insert(cache_key, permissions);

        result
    }

    /// Computes all permissions a user has on a resource.
    fn compute_permissions(
        &self,
        user_id: Uuid,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) -> HashSet<Action> {
        let mut permissions = HashSet::new();
        let relations = self.get_user_relations(user_id, resource_type, resource_id);

        for relation in relations {
            match relation {
                Relation::Owner => {
                    // Owners can do everything
                    permissions.insert(Action::Read);
                    permissions.insert(Action::Update);
                    permissions.insert(Action::Delete);
                    permissions.insert(Action::Share);
                    permissions.insert(Action::ManagePermissions);
                }
                Relation::Admin => {
                    // Admins can do most things except delete
                    permissions.insert(Action::Read);
                    permissions.insert(Action::Update);
                    permissions.insert(Action::Share);
                    permissions.insert(Action::ManagePermissions);
                }
                Relation::Editor => {
                    // Editors can read and update
                    permissions.insert(Action::Read);
                    permissions.insert(Action::Update);
                }
                Relation::Viewer => {
                    // Viewers can only read
                    permissions.insert(Action::Read);
                }
                Relation::Member => {
                    // Members have read access by default
                    permissions.insert(Action::Read);
                }
                Relation::Parent => {
                    // Parent relation might inherit permissions
                    // (this would require recursive lookup in a full implementation)
                    permissions.insert(Action::Read);
                }
            }
        }

        permissions
    }

    /// Grants a specific permission by adding the appropriate relation.
    pub fn grant_permission(
        &mut self,
        user_id: Uuid,
        action: Action,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) {
        // Map action to appropriate relation
        let relation = match action {
            Action::Read => Relation::Viewer,
            Action::Update => Relation::Editor,
            Action::Delete | Action::ManagePermissions => Relation::Owner,
            Action::Share => Relation::Admin,
            Action::Create => Relation::Editor, // For creating sub-resources
        };

        self.add_tuple(RelationTuple::new(
            user_id,
            relation,
            resource_type,
            resource_id,
        ));
    }

    /// Revokes all permissions for a user on a resource.
    pub fn revoke_all_permissions(
        &mut self,
        user_id: Uuid,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) {
        // Remove all tuples for this user/resource combination
        self.tuples.retain(|t| {
            !(t.user_id == user_id
                && t.resource_type == resource_type
                && t.resource_id == resource_id)
        });

        // Clear cache
        self.permission_cache
            .remove(&(user_id, resource_type, resource_id));
    }

    /// Clears the permission cache.
    pub fn clear_cache(&mut self) {
        self.permission_cache.clear();
    }

    /// Returns the total number of relationship tuples.
    pub fn tuple_count(&self) -> usize {
        self.tuples.len()
    }
}

impl Default for ReBACEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to create ownership when a resource is created.
pub fn create_resource_ownership(
    engine: &mut ReBACEngine,
    creator_id: Uuid,
    resource_type: ResourceType,
    resource_id: Uuid,
) {
    engine.add_tuple(RelationTuple::new(
        creator_id,
        Relation::Owner,
        resource_type,
        resource_id,
    ));
}

/// Helper to share a resource with another user.
pub fn share_resource(
    engine: &mut ReBACEngine,
    resource_type: ResourceType,
    resource_id: Uuid,
    target_user_id: Uuid,
    relation: Relation,
) {
    engine.add_tuple(RelationTuple::new(
        target_user_id,
        relation,
        resource_type,
        resource_id,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rebac_basic_operations() {
        let mut engine = ReBACEngine::new();
        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        // Add owner relationship
        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Owner,
            ResourceType::Statute,
            resource_id,
        ));

        // Owner should have all permissions
        assert!(engine.check_permission(user_id, Action::Read, ResourceType::Statute, resource_id));
        assert!(engine.check_permission(
            user_id,
            Action::Update,
            ResourceType::Statute,
            resource_id
        ));
        assert!(engine.check_permission(
            user_id,
            Action::Delete,
            ResourceType::Statute,
            resource_id
        ));
        assert!(engine.check_permission(
            user_id,
            Action::Share,
            ResourceType::Statute,
            resource_id
        ));
    }

    #[test]
    fn test_rebac_viewer_permissions() {
        let mut engine = ReBACEngine::new();
        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        // Add viewer relationship
        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Viewer,
            ResourceType::Statute,
            resource_id,
        ));

        // Viewer should only have read permission
        assert!(engine.check_permission(user_id, Action::Read, ResourceType::Statute, resource_id));
        assert!(!engine.check_permission(
            user_id,
            Action::Update,
            ResourceType::Statute,
            resource_id
        ));
        assert!(!engine.check_permission(
            user_id,
            Action::Delete,
            ResourceType::Statute,
            resource_id
        ));
    }

    #[test]
    fn test_rebac_editor_permissions() {
        let mut engine = ReBACEngine::new();
        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        // Add editor relationship
        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Editor,
            ResourceType::Statute,
            resource_id,
        ));

        // Editor should have read and update
        assert!(engine.check_permission(user_id, Action::Read, ResourceType::Statute, resource_id));
        assert!(engine.check_permission(
            user_id,
            Action::Update,
            ResourceType::Statute,
            resource_id
        ));
        assert!(!engine.check_permission(
            user_id,
            Action::Delete,
            ResourceType::Statute,
            resource_id
        ));
    }

    #[test]
    fn test_rebac_grant_revoke() {
        let mut engine = ReBACEngine::new();
        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        // Grant read permission
        engine.grant_permission(user_id, Action::Read, ResourceType::Statute, resource_id);
        assert!(engine.check_permission(user_id, Action::Read, ResourceType::Statute, resource_id));

        // Revoke all permissions
        engine.revoke_all_permissions(user_id, ResourceType::Statute, resource_id);
        assert!(!engine.check_permission(
            user_id,
            Action::Read,
            ResourceType::Statute,
            resource_id
        ));
    }

    #[test]
    fn test_rebac_get_user_resources() {
        let mut engine = ReBACEngine::new();
        let user_id = Uuid::new_v4();
        let resource1 = Uuid::new_v4();
        let resource2 = Uuid::new_v4();
        let resource3 = Uuid::new_v4();

        // User owns resource1 and resource2
        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Owner,
            ResourceType::Statute,
            resource1,
        ));
        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Owner,
            ResourceType::Statute,
            resource2,
        ));
        // User is viewer of resource3
        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Viewer,
            ResourceType::Statute,
            resource3,
        ));

        let owned = engine.get_user_resources(user_id, Relation::Owner, ResourceType::Statute);
        assert_eq!(owned.len(), 2);
        assert!(owned.contains(&resource1));
        assert!(owned.contains(&resource2));
        assert!(!owned.contains(&resource3));
    }

    #[test]
    fn test_rebac_cache() {
        let mut engine = ReBACEngine::new();
        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Owner,
            ResourceType::Statute,
            resource_id,
        ));

        // First check should compute and cache
        assert!(engine.check_permission(user_id, Action::Read, ResourceType::Statute, resource_id));

        // Cache should now have this entry
        assert_eq!(engine.permission_cache.len(), 1);

        // Second check should use cache
        assert!(engine.check_permission(
            user_id,
            Action::Update,
            ResourceType::Statute,
            resource_id
        ));

        // Clear cache
        engine.clear_cache();
        assert_eq!(engine.permission_cache.len(), 0);
    }

    #[test]
    fn test_resource_sharing() {
        let mut engine = ReBACEngine::new();
        let owner_id = Uuid::new_v4();
        let viewer_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        // Create resource with ownership
        create_resource_ownership(&mut engine, owner_id, ResourceType::Statute, resource_id);

        // Share with another user as viewer
        share_resource(
            &mut engine,
            ResourceType::Statute,
            resource_id,
            viewer_id,
            Relation::Viewer,
        );

        // Both users should have appropriate permissions
        assert!(engine.check_permission(
            owner_id,
            Action::Delete,
            ResourceType::Statute,
            resource_id
        ));
        assert!(engine.check_permission(
            viewer_id,
            Action::Read,
            ResourceType::Statute,
            resource_id
        ));
        assert!(!engine.check_permission(
            viewer_id,
            Action::Delete,
            ResourceType::Statute,
            resource_id
        ));
    }

    #[test]
    fn test_multiple_relations() {
        let mut engine = ReBACEngine::new();
        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        // User can have multiple relations (though typically one would suffice)
        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Viewer,
            ResourceType::Statute,
            resource_id,
        ));
        engine.add_tuple(RelationTuple::new(
            user_id,
            Relation::Editor,
            ResourceType::Statute,
            resource_id,
        ));

        let relations = engine.get_user_relations(user_id, ResourceType::Statute, resource_id);
        assert_eq!(relations.len(), 2);
        assert!(relations.contains(&Relation::Viewer));
        assert!(relations.contains(&Relation::Editor));

        // Should have editor permissions (union of all relations)
        assert!(engine.check_permission(
            user_id,
            Action::Update,
            ResourceType::Statute,
            resource_id
        ));
    }
}
