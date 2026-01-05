//! Inter-agent relationship modeling.
//!
//! This module provides:
//! - Relationship types (family, employment, property, contracts)
//! - Relationship graphs and queries
//! - Organization hierarchies
//! - Property and asset ownership

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Type of relationship between agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Parent-child relationship
    Parent,
    /// Child-parent relationship
    Child,
    /// Spouse relationship
    Spouse,
    /// Sibling relationship
    Sibling,
    /// Employer-employee relationship
    Employer,
    /// Employee-employer relationship
    Employee,
    /// Property owner relationship
    PropertyOwner,
    /// Property tenant relationship
    PropertyTenant,
    /// Contract party relationship
    ContractParty,
    /// Guardian-ward relationship
    Guardian,
    /// Ward-guardian relationship
    Ward,
    /// Business partner relationship
    BusinessPartner,
    /// Creditor-debtor relationship
    Creditor,
    /// Debtor-creditor relationship
    Debtor,
}

impl RelationshipType {
    /// Returns the inverse relationship type.
    pub fn inverse(&self) -> Self {
        match self {
            RelationshipType::Parent => RelationshipType::Child,
            RelationshipType::Child => RelationshipType::Parent,
            RelationshipType::Spouse => RelationshipType::Spouse,
            RelationshipType::Sibling => RelationshipType::Sibling,
            RelationshipType::Employer => RelationshipType::Employee,
            RelationshipType::Employee => RelationshipType::Employer,
            RelationshipType::PropertyOwner => RelationshipType::PropertyTenant,
            RelationshipType::PropertyTenant => RelationshipType::PropertyOwner,
            RelationshipType::ContractParty => RelationshipType::ContractParty,
            RelationshipType::Guardian => RelationshipType::Ward,
            RelationshipType::Ward => RelationshipType::Guardian,
            RelationshipType::BusinessPartner => RelationshipType::BusinessPartner,
            RelationshipType::Creditor => RelationshipType::Debtor,
            RelationshipType::Debtor => RelationshipType::Creditor,
        }
    }

    /// Checks if the relationship is symmetric.
    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            RelationshipType::Spouse
                | RelationshipType::Sibling
                | RelationshipType::ContractParty
                | RelationshipType::BusinessPartner
        )
    }
}

/// A relationship between two entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Source entity ID
    pub from: Uuid,
    /// Target entity ID
    pub to: Uuid,
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Strength of relationship (0.0 to 1.0)
    pub strength: f64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Relationship {
    /// Creates a new relationship.
    pub fn new(from: Uuid, to: Uuid, relationship_type: RelationshipType) -> Self {
        Self {
            from,
            to,
            relationship_type,
            strength: 1.0,
            metadata: HashMap::new(),
        }
    }

    /// Sets the strength of the relationship.
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    /// Adds metadata to the relationship.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Graph of relationships between agents.
#[derive(Debug, Default, Clone)]
pub struct RelationshipGraph {
    /// Adjacency list: entity_id -> [(target_id, relationship_type, strength)]
    edges: HashMap<Uuid, Vec<(Uuid, RelationshipType, f64)>>,
    /// All relationships
    relationships: Vec<Relationship>,
}

impl RelationshipGraph {
    /// Creates a new empty relationship graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a relationship to the graph.
    pub fn add_relationship(&mut self, relationship: Relationship) {
        // Add forward edge
        self.edges.entry(relationship.from).or_default().push((
            relationship.to,
            relationship.relationship_type,
            relationship.strength,
        ));

        // Add inverse edge if relationship is symmetric or add explicit inverse
        if relationship.relationship_type.is_symmetric() {
            self.edges.entry(relationship.to).or_default().push((
                relationship.from,
                relationship.relationship_type,
                relationship.strength,
            ));
        }

        self.relationships.push(relationship);
    }

    /// Gets all relationships of a specific type for an entity.
    pub fn get_relationships(
        &self,
        entity_id: Uuid,
        relationship_type: RelationshipType,
    ) -> Vec<Uuid> {
        self.edges
            .get(&entity_id)
            .map(|edges| {
                edges
                    .iter()
                    .filter(|(_, rel_type, _)| *rel_type == relationship_type)
                    .map(|(target, _, _)| *target)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets all related entities regardless of relationship type.
    pub fn get_all_related(&self, entity_id: Uuid) -> Vec<(Uuid, RelationshipType)> {
        self.edges
            .get(&entity_id)
            .map(|edges| {
                edges
                    .iter()
                    .map(|(target, rel_type, _)| (*target, *rel_type))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Checks if two entities have a specific relationship.
    pub fn has_relationship(
        &self,
        from: Uuid,
        to: Uuid,
        relationship_type: RelationshipType,
    ) -> bool {
        self.edges
            .get(&from)
            .map(|edges| {
                edges
                    .iter()
                    .any(|(target, rel_type, _)| *target == to && *rel_type == relationship_type)
            })
            .unwrap_or(false)
    }

    /// Finds all entities within N degrees of separation.
    pub fn find_connected(&self, entity_id: Uuid, max_depth: usize) -> HashSet<Uuid> {
        let mut visited = HashSet::new();
        let mut current_level = HashSet::new();
        current_level.insert(entity_id);
        visited.insert(entity_id);

        for _ in 0..max_depth {
            let mut next_level = HashSet::new();
            for &current in &current_level {
                if let Some(edges) = self.edges.get(&current) {
                    for (target, _, _) in edges {
                        if visited.insert(*target) {
                            next_level.insert(*target);
                        }
                    }
                }
            }
            current_level = next_level;
            if current_level.is_empty() {
                break;
            }
        }

        visited
    }

    /// Gets the total number of relationships.
    pub fn relationship_count(&self) -> usize {
        self.relationships.len()
    }

    /// Gets all relationships.
    pub fn all_relationships(&self) -> &[Relationship] {
        &self.relationships
    }
}

/// Organization hierarchy (e.g., company structure).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    /// Organization ID
    pub id: Uuid,
    /// Organization name
    pub name: String,
    /// Organization type
    pub org_type: OrganizationType,
    /// Members and their roles
    pub members: HashMap<Uuid, OrganizationRole>,
    /// Reporting structure (employee_id -> manager_id)
    pub hierarchy: HashMap<Uuid, Uuid>,
}

/// Type of organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationType {
    /// For-profit corporation
    Corporation,
    /// Non-profit organization
    NonProfit,
    /// Government agency
    Government,
    /// Partnership
    Partnership,
    /// Sole proprietorship
    SoleProprietorship,
}

/// Role within an organization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationRole {
    /// Executive officer
    Executive,
    /// Manager
    Manager,
    /// Employee
    Employee,
    /// Contractor
    Contractor,
    /// Board member
    BoardMember,
    /// Owner/Shareholder
    Owner,
}

impl Organization {
    /// Creates a new organization.
    pub fn new(name: impl Into<String>, org_type: OrganizationType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            org_type,
            members: HashMap::new(),
            hierarchy: HashMap::new(),
        }
    }

    /// Adds a member to the organization.
    pub fn add_member(&mut self, member_id: Uuid, role: OrganizationRole) {
        self.members.insert(member_id, role);
    }

    /// Sets the manager for an employee.
    pub fn set_manager(&mut self, employee_id: Uuid, manager_id: Uuid) {
        self.hierarchy.insert(employee_id, manager_id);
    }

    /// Gets the manager of an employee.
    pub fn get_manager(&self, employee_id: Uuid) -> Option<Uuid> {
        self.hierarchy.get(&employee_id).copied()
    }

    /// Gets all direct reports of a manager.
    pub fn get_direct_reports(&self, manager_id: Uuid) -> Vec<Uuid> {
        self.hierarchy
            .iter()
            .filter(|&(_, mgr)| *mgr == manager_id)
            .map(|(emp, _)| *emp)
            .collect()
    }

    /// Gets the role of a member.
    pub fn get_role(&self, member_id: Uuid) -> Option<&OrganizationRole> {
        self.members.get(&member_id)
    }

    /// Checks if an entity is a member.
    pub fn is_member(&self, entity_id: Uuid) -> bool {
        self.members.contains_key(&entity_id)
    }
}

/// Property or asset ownership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    /// Property ID
    pub id: Uuid,
    /// Property type
    pub property_type: PropertyType,
    /// Owner ID
    pub owner: Uuid,
    /// Value of the property
    pub value: f64,
    /// Additional attributes
    pub attributes: HashMap<String, String>,
}

/// Type of property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Real estate
    RealEstate,
    /// Vehicle
    Vehicle,
    /// Financial asset
    FinancialAsset,
    /// Intellectual property
    IntellectualProperty,
    /// Personal property
    PersonalProperty,
}

impl Property {
    /// Creates a new property.
    pub fn new(property_type: PropertyType, owner: Uuid, value: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            property_type,
            owner,
            value,
            attributes: HashMap::new(),
        }
    }

    /// Sets an attribute.
    pub fn set_attribute(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(key.into(), value.into());
    }

    /// Gets an attribute.
    pub fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(|s| s.as_str())
    }
}

/// Registry of all properties.
#[derive(Debug, Default, Clone)]
pub struct PropertyRegistry {
    properties: HashMap<Uuid, Property>,
    /// Index by owner
    by_owner: HashMap<Uuid, Vec<Uuid>>,
}

impl PropertyRegistry {
    /// Creates a new property registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a property.
    pub fn register(&mut self, property: Property) {
        let property_id = property.id;
        let owner_id = property.owner;

        self.properties.insert(property_id, property);
        self.by_owner.entry(owner_id).or_default().push(property_id);
    }

    /// Gets a property by ID.
    pub fn get(&self, property_id: Uuid) -> Option<&Property> {
        self.properties.get(&property_id)
    }

    /// Gets all properties owned by an entity.
    pub fn get_by_owner(&self, owner_id: Uuid) -> Vec<&Property> {
        self.by_owner
            .get(&owner_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.properties.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Transfers property ownership.
    pub fn transfer(&mut self, property_id: Uuid, new_owner: Uuid) -> bool {
        if let Some(property) = self.properties.get_mut(&property_id) {
            let old_owner = property.owner;

            // Remove from old owner's list
            if let Some(list) = self.by_owner.get_mut(&old_owner) {
                list.retain(|&id| id != property_id);
            }

            // Add to new owner's list
            self.by_owner
                .entry(new_owner)
                .or_default()
                .push(property_id);

            // Update property
            property.owner = new_owner;
            true
        } else {
            false
        }
    }

    /// Calculates total property value for an owner.
    pub fn total_value(&self, owner_id: Uuid) -> f64 {
        self.get_by_owner(owner_id).iter().map(|p| p.value).sum()
    }
}

/// Status of a contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractStatus {
    /// Contract is being drafted
    Draft,
    /// Contract is active and in force
    Active,
    /// Contract has been fulfilled
    Fulfilled,
    /// Contract has been breached
    Breached,
    /// Contract has been terminated
    Terminated,
    /// Contract has expired
    Expired,
}

/// Type of contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Employment contract
    Employment,
    /// Service contract
    Service,
    /// Sale contract
    Sale,
    /// Lease contract
    Lease,
    /// Loan contract
    Loan,
    /// Partnership agreement
    Partnership,
    /// Non-disclosure agreement
    NDA,
    /// License agreement
    License,
    /// Custom contract type
    Custom,
}

/// A contract between parties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// Unique contract ID
    pub id: Uuid,
    /// Contract type
    pub contract_type: ContractType,
    /// Contract status
    pub status: ContractStatus,
    /// Parties involved (entity IDs)
    pub parties: Vec<Uuid>,
    /// Contract value (monetary)
    pub value: f64,
    /// Start date (ISO format string)
    pub start_date: Option<String>,
    /// End date (ISO format string)
    pub end_date: Option<String>,
    /// Contract terms as key-value pairs
    pub terms: HashMap<String, String>,
    /// Obligations for each party
    pub obligations: HashMap<Uuid, Vec<String>>,
}

impl Contract {
    /// Creates a new contract.
    pub fn new(contract_type: ContractType, parties: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            contract_type,
            status: ContractStatus::Draft,
            parties,
            value: 0.0,
            start_date: None,
            end_date: None,
            terms: HashMap::new(),
            obligations: HashMap::new(),
        }
    }

    /// Sets contract value.
    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    /// Sets start date.
    pub fn with_start_date(mut self, date: String) -> Self {
        self.start_date = Some(date);
        self
    }

    /// Sets end date.
    pub fn with_end_date(mut self, date: String) -> Self {
        self.end_date = Some(date);
        self
    }

    /// Adds a term to the contract.
    pub fn with_term(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.terms.insert(key.into(), value.into());
        self
    }

    /// Adds an obligation for a party.
    pub fn with_obligation(mut self, party: Uuid, obligation: impl Into<String>) -> Self {
        self.obligations
            .entry(party)
            .or_default()
            .push(obligation.into());
        self
    }

    /// Activates the contract.
    pub fn activate(&mut self) {
        self.status = ContractStatus::Active;
    }

    /// Marks the contract as fulfilled.
    pub fn fulfill(&mut self) {
        self.status = ContractStatus::Fulfilled;
    }

    /// Marks the contract as breached.
    pub fn breach(&mut self) {
        self.status = ContractStatus::Breached;
    }

    /// Terminates the contract.
    pub fn terminate(&mut self) {
        self.status = ContractStatus::Terminated;
    }

    /// Checks if a party is involved in this contract.
    pub fn involves(&self, party_id: Uuid) -> bool {
        self.parties.contains(&party_id)
    }

    /// Gets obligations for a specific party.
    pub fn get_obligations(&self, party_id: Uuid) -> Vec<&str> {
        self.obligations
            .get(&party_id)
            .map(|obs| obs.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
}

/// Registry for managing contracts.
pub struct ContractRegistry {
    /// All contracts indexed by ID
    contracts: HashMap<Uuid, Contract>,
    /// Contracts by party (party_id -> list of contract_ids)
    by_party: HashMap<Uuid, Vec<Uuid>>,
    /// Contracts by type
    by_type: HashMap<ContractType, Vec<Uuid>>,
    /// Contracts by status
    by_status: HashMap<ContractStatus, Vec<Uuid>>,
}

impl ContractRegistry {
    /// Creates a new contract registry.
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
            by_party: HashMap::new(),
            by_type: HashMap::new(),
            by_status: HashMap::new(),
        }
    }

    /// Registers a contract.
    pub fn register(&mut self, contract: Contract) {
        let contract_id = contract.id;
        let contract_type = contract.contract_type;
        let contract_status = contract.status;

        // Index by parties
        for &party_id in &contract.parties {
            self.by_party.entry(party_id).or_default().push(contract_id);
        }

        // Index by type
        self.by_type
            .entry(contract_type)
            .or_default()
            .push(contract_id);

        // Index by status
        self.by_status
            .entry(contract_status)
            .or_default()
            .push(contract_id);

        // Store contract
        self.contracts.insert(contract_id, contract);
    }

    /// Gets a contract by ID.
    pub fn get(&self, contract_id: Uuid) -> Option<&Contract> {
        self.contracts.get(&contract_id)
    }

    /// Gets a mutable reference to a contract.
    pub fn get_mut(&mut self, contract_id: Uuid) -> Option<&mut Contract> {
        self.contracts.get_mut(&contract_id)
    }

    /// Gets all contracts for a party.
    pub fn get_by_party(&self, party_id: Uuid) -> Vec<&Contract> {
        self.by_party
            .get(&party_id)
            .map(|ids| ids.iter().filter_map(|id| self.contracts.get(id)).collect())
            .unwrap_or_default()
    }

    /// Gets all contracts of a specific type.
    pub fn get_by_type(&self, contract_type: ContractType) -> Vec<&Contract> {
        self.by_type
            .get(&contract_type)
            .map(|ids| ids.iter().filter_map(|id| self.contracts.get(id)).collect())
            .unwrap_or_default()
    }

    /// Gets all contracts with a specific status.
    pub fn get_by_status(&self, status: ContractStatus) -> Vec<&Contract> {
        self.by_status
            .get(&status)
            .map(|ids| ids.iter().filter_map(|id| self.contracts.get(id)).collect())
            .unwrap_or_default()
    }

    /// Updates contract status and reindexes.
    pub fn update_status(&mut self, contract_id: Uuid, new_status: ContractStatus) -> bool {
        if let Some(contract) = self.contracts.get_mut(&contract_id) {
            let old_status = contract.status;
            contract.status = new_status;

            // Update status index
            if let Some(list) = self.by_status.get_mut(&old_status) {
                list.retain(|&id| id != contract_id);
            }
            self.by_status
                .entry(new_status)
                .or_default()
                .push(contract_id);

            true
        } else {
            false
        }
    }

    /// Calculates total contract value for a party.
    pub fn total_value(&self, party_id: Uuid) -> f64 {
        self.get_by_party(party_id).iter().map(|c| c.value).sum()
    }

    /// Gets active contracts for a party.
    pub fn get_active_contracts(&self, party_id: Uuid) -> Vec<&Contract> {
        self.get_by_party(party_id)
            .into_iter()
            .filter(|c| c.status == ContractStatus::Active)
            .collect()
    }
}

impl Default for ContractRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_types() {
        assert_eq!(RelationshipType::Parent.inverse(), RelationshipType::Child);
        assert_eq!(
            RelationshipType::Employer.inverse(),
            RelationshipType::Employee
        );
        assert!(RelationshipType::Spouse.is_symmetric());
        assert!(!RelationshipType::Parent.is_symmetric());
    }

    #[test]
    fn test_relationship_graph() {
        let mut graph = RelationshipGraph::new();

        let person1 = Uuid::new_v4();
        let person2 = Uuid::new_v4();
        let person3 = Uuid::new_v4();

        // person1 is parent of person2
        graph.add_relationship(Relationship::new(
            person1,
            person2,
            RelationshipType::Parent,
        ));

        // person1 is spouse of person3
        graph.add_relationship(Relationship::new(
            person1,
            person3,
            RelationshipType::Spouse,
        ));

        assert!(graph.has_relationship(person1, person2, RelationshipType::Parent));
        assert!(graph.has_relationship(person1, person3, RelationshipType::Spouse));
        // Spouse is symmetric
        assert!(graph.has_relationship(person3, person1, RelationshipType::Spouse));

        let children = graph.get_relationships(person1, RelationshipType::Parent);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], person2);
    }

    #[test]
    fn test_find_connected() {
        let mut graph = RelationshipGraph::new();

        let p1 = Uuid::new_v4();
        let p2 = Uuid::new_v4();
        let p3 = Uuid::new_v4();
        let p4 = Uuid::new_v4();

        graph.add_relationship(Relationship::new(p1, p2, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(p2, p3, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(p3, p4, RelationshipType::Parent));

        let connected = graph.find_connected(p1, 2);
        assert!(connected.contains(&p1));
        assert!(connected.contains(&p2));
        assert!(connected.contains(&p3));
        assert!(!connected.contains(&p4)); // Too far (3 degrees)

        let connected_all = graph.find_connected(p1, 10);
        assert_eq!(connected_all.len(), 4); // All connected
    }

    #[test]
    fn test_organization() {
        let mut org = Organization::new("Acme Corp", OrganizationType::Corporation);

        let ceo = Uuid::new_v4();
        let manager = Uuid::new_v4();
        let employee = Uuid::new_v4();

        org.add_member(ceo, OrganizationRole::Executive);
        org.add_member(manager, OrganizationRole::Manager);
        org.add_member(employee, OrganizationRole::Employee);

        org.set_manager(manager, ceo);
        org.set_manager(employee, manager);

        assert_eq!(org.get_manager(employee), Some(manager));
        assert_eq!(org.get_manager(manager), Some(ceo));

        let reports = org.get_direct_reports(manager);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0], employee);
    }

    #[test]
    fn test_property_registry() {
        let mut registry = PropertyRegistry::new();

        let owner1 = Uuid::new_v4();
        let owner2 = Uuid::new_v4();

        let mut property1 = Property::new(PropertyType::RealEstate, owner1, 500_000.0);
        property1.set_attribute("address", "123 Main St");
        let prop1_id = property1.id;

        let property2 = Property::new(PropertyType::Vehicle, owner1, 30_000.0);

        registry.register(property1);
        registry.register(property2);

        assert_eq!(registry.get_by_owner(owner1).len(), 2);
        assert_eq!(registry.total_value(owner1), 530_000.0);

        // Transfer property
        registry.transfer(prop1_id, owner2);

        assert_eq!(registry.get_by_owner(owner1).len(), 1);
        assert_eq!(registry.get_by_owner(owner2).len(), 1);
        assert_eq!(registry.total_value(owner1), 30_000.0);
        assert_eq!(registry.total_value(owner2), 500_000.0);
    }

    #[test]
    fn test_property_attributes() {
        let owner = Uuid::new_v4();
        let mut property = Property::new(PropertyType::RealEstate, owner, 300_000.0);

        property.set_attribute("city", "San Francisco");
        property.set_attribute("bedrooms", "3");

        assert_eq!(property.get_attribute("city"), Some("San Francisco"));
        assert_eq!(property.get_attribute("bedrooms"), Some("3"));
        assert_eq!(property.get_attribute("nonexistent"), None);
    }
}
