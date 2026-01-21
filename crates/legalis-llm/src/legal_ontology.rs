//! Legal Ontology Integration
//!
//! Deep integration with legal ontologies like LegalRuleML, LKIF, and custom legal taxonomies.
//! Enables semantic reasoning over legal concepts and relationships.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Legal concept in an ontology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegalConcept {
    /// Unique identifier (URI)
    pub id: String,
    /// Human-readable label
    pub label: String,
    /// Description
    pub description: String,
    /// Type of concept
    pub concept_type: ConceptType,
    /// Properties
    pub properties: HashMap<String, String>,
}

impl LegalConcept {
    /// Creates a new legal concept.
    pub fn new(id: impl Into<String>, label: impl Into<String>, concept_type: ConceptType) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: String::new(),
            concept_type,
            properties: HashMap::new(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Adds a property.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Type of legal concept
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConceptType {
    /// Legal norm (rule, principle, standard)
    Norm,
    /// Legal actor (person, organization, court)
    Actor,
    /// Legal object (property, right, obligation)
    Object,
    /// Legal event (transaction, verdict, contract formation)
    Event,
    /// Legal status (validity, enforceability)
    Status,
    /// Legal procedure (litigation, arbitration)
    Procedure,
    /// Legal document type (statute, case law, contract)
    DocumentType,
    /// Legal jurisdiction
    Jurisdiction,
}

/// Relationship between legal concepts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LegalRelation {
    /// Source concept ID
    pub from: String,
    /// Target concept ID
    pub to: String,
    /// Type of relationship
    pub relation_type: RelationType,
    /// Strength (0.0-1.0)
    pub strength: f64,
    /// Properties
    pub properties: HashMap<String, String>,
}

impl LegalRelation {
    /// Creates a new legal relation.
    pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
        relation_type: RelationType,
    ) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            relation_type,
            strength: 1.0,
            properties: HashMap::new(),
        }
    }

    /// Sets the strength.
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    /// Adds a property.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Type of relationship between legal concepts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    /// Subsumption (is-a, subclass-of)
    IsA,
    /// Parthood (part-of, component-of)
    PartOf,
    /// Causation (causes, leads-to)
    Causes,
    /// Opposition (conflicts-with, contradicts)
    ConflictsWith,
    /// Support (supports, reinforces)
    Supports,
    /// Reference (cites, refers-to)
    References,
    /// Temporal precedence (precedes, follows)
    Precedes,
    /// Jurisdiction (applies-in, governs)
    AppliesIn,
    /// Modification (amends, repeals)
    Modifies,
    /// Implementation (implements, realizes)
    Implements,
}

/// Legal ontology
pub struct LegalOntology {
    /// Name of the ontology
    name: String,
    /// Concepts indexed by ID
    concepts: HashMap<String, LegalConcept>,
    /// Relations
    relations: Vec<LegalRelation>,
    /// Inference rules
    inference_rules: Vec<InferenceRule>,
}

impl LegalOntology {
    /// Creates a new ontology.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            concepts: HashMap::new(),
            relations: Vec::new(),
            inference_rules: Vec::new(),
        }
    }

    /// Adds a concept to the ontology.
    pub fn add_concept(&mut self, concept: LegalConcept) {
        self.concepts.insert(concept.id.clone(), concept);
    }

    /// Adds a relation to the ontology.
    pub fn add_relation(&mut self, relation: LegalRelation) -> Result<()> {
        // Validate that both concepts exist
        if !self.concepts.contains_key(&relation.from) {
            return Err(anyhow!("Source concept not found: {}", relation.from));
        }
        if !self.concepts.contains_key(&relation.to) {
            return Err(anyhow!("Target concept not found: {}", relation.to));
        }
        self.relations.push(relation);
        Ok(())
    }

    /// Adds an inference rule.
    pub fn add_inference_rule(&mut self, rule: InferenceRule) {
        self.inference_rules.push(rule);
    }

    /// Gets a concept by ID.
    pub fn get_concept(&self, id: &str) -> Option<&LegalConcept> {
        self.concepts.get(id)
    }

    /// Gets all concepts of a given type.
    pub fn get_concepts_by_type(&self, concept_type: ConceptType) -> Vec<&LegalConcept> {
        self.concepts
            .values()
            .filter(|c| c.concept_type == concept_type)
            .collect()
    }

    /// Gets all relations from a concept.
    pub fn get_relations_from(&self, concept_id: &str) -> Vec<&LegalRelation> {
        self.relations
            .iter()
            .filter(|r| r.from == concept_id)
            .collect()
    }

    /// Gets all relations to a concept.
    pub fn get_relations_to(&self, concept_id: &str) -> Vec<&LegalRelation> {
        self.relations
            .iter()
            .filter(|r| r.to == concept_id)
            .collect()
    }

    /// Finds all ancestors of a concept via is-a relations.
    pub fn get_ancestors(&self, concept_id: &str) -> Vec<&LegalConcept> {
        let mut ancestors = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = vec![concept_id];

        while let Some(current_id) = queue.pop() {
            if visited.contains(current_id) {
                continue;
            }
            visited.insert(current_id);

            for relation in self.get_relations_from(current_id) {
                if relation.relation_type == RelationType::IsA
                    && let Some(ancestor) = self.get_concept(&relation.to)
                {
                    ancestors.push(ancestor);
                    queue.push(&relation.to);
                }
            }
        }

        ancestors
    }

    /// Finds all descendants of a concept via is-a relations.
    pub fn get_descendants(&self, concept_id: &str) -> Vec<&LegalConcept> {
        let mut descendants = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = vec![concept_id];

        while let Some(current_id) = queue.pop() {
            if visited.contains(current_id) {
                continue;
            }
            visited.insert(current_id);

            for relation in self.get_relations_to(current_id) {
                if relation.relation_type == RelationType::IsA
                    && let Some(descendant) = self.get_concept(&relation.from)
                {
                    descendants.push(descendant);
                    queue.push(&relation.from);
                }
            }
        }

        descendants
    }

    /// Checks if one concept is a subtype of another.
    pub fn is_subtype_of(&self, subtype_id: &str, supertype_id: &str) -> bool {
        if subtype_id == supertype_id {
            return true;
        }

        let ancestors = self.get_ancestors(subtype_id);
        ancestors.iter().any(|a| a.id == supertype_id)
    }

    /// Performs inference to derive new relations.
    pub fn infer(&mut self) -> usize {
        let mut new_relations = Vec::new();

        for rule in &self.inference_rules {
            new_relations.extend(rule.apply(self));
        }

        let initial_count = self.relations.len();
        for relation in new_relations {
            if !self.relations.contains(&relation) {
                self.relations.push(relation);
            }
        }

        self.relations.len() - initial_count
    }

    /// Exports the ontology to RDF/Turtle format.
    pub fn to_turtle(&self) -> String {
        let mut turtle = String::new();
        turtle.push_str(&format!(
            "@prefix : <http://legalis.ai/ontology/{}#> .\n",
            self.name
        ));
        turtle.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
        turtle.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n");

        // Export concepts
        for concept in self.concepts.values() {
            turtle.push_str(&format!(
                ":{} a :{};\n",
                concept.id,
                concept_type_to_string(concept.concept_type)
            ));
            turtle.push_str(&format!("  rdfs:label \"{}\";\n", concept.label));
            if !concept.description.is_empty() {
                turtle.push_str(&format!("  rdfs:comment \"{}\";\n", concept.description));
            }
            turtle.push_str("  .\n\n");
        }

        // Export relations
        for relation in &self.relations {
            turtle.push_str(&format!(
                ":{} :{} :{}.\n",
                relation.from,
                relation_type_to_string(relation.relation_type),
                relation.to
            ));
        }

        turtle
    }

    /// Gets statistics about the ontology.
    pub fn statistics(&self) -> OntologyStatistics {
        let mut concepts_by_type = HashMap::new();
        for concept in self.concepts.values() {
            *concepts_by_type.entry(concept.concept_type).or_insert(0) += 1;
        }

        let mut relations_by_type = HashMap::new();
        for relation in &self.relations {
            *relations_by_type.entry(relation.relation_type).or_insert(0) += 1;
        }

        OntologyStatistics {
            total_concepts: self.concepts.len(),
            total_relations: self.relations.len(),
            concepts_by_type,
            relations_by_type,
        }
    }
}

/// Ontology statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyStatistics {
    /// Total number of concepts
    pub total_concepts: usize,
    /// Total number of relations
    pub total_relations: usize,
    /// Concepts grouped by type
    pub concepts_by_type: HashMap<ConceptType, usize>,
    /// Relations grouped by type
    pub relations_by_type: HashMap<RelationType, usize>,
}

/// Inference rule for deriving new relations
pub struct InferenceRule {
    /// Name of the rule
    pub name: String,
    /// Rule application function
    pub apply: Box<dyn Fn(&LegalOntology) -> Vec<LegalRelation> + Send + Sync>,
}

impl InferenceRule {
    /// Creates a new inference rule.
    pub fn new<F>(name: impl Into<String>, apply: F) -> Self
    where
        F: Fn(&LegalOntology) -> Vec<LegalRelation> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            apply: Box::new(apply),
        }
    }

    /// Applies the rule to an ontology.
    pub fn apply(&self, ontology: &LegalOntology) -> Vec<LegalRelation> {
        (self.apply)(ontology)
    }
}

/// Common inference rules
pub mod inference_rules {
    use super::*;

    /// Transitivity rule: if A isA B and B isA C, then A isA C
    pub fn transitivity() -> InferenceRule {
        InferenceRule::new("transitivity", |ontology| {
            let mut new_relations = Vec::new();

            for r1 in &ontology.relations {
                if r1.relation_type == RelationType::IsA {
                    for r2 in &ontology.relations {
                        if r2.relation_type == RelationType::IsA && r1.to == r2.from {
                            new_relations.push(
                                LegalRelation::new(
                                    r1.from.clone(),
                                    r2.to.clone(),
                                    RelationType::IsA,
                                )
                                .with_strength(r1.strength * r2.strength),
                            );
                        }
                    }
                }
            }

            new_relations
        })
    }

    /// Symmetry rule: if A conflictsWith B, then B conflictsWith A
    pub fn symmetry() -> InferenceRule {
        InferenceRule::new("symmetry", |ontology| {
            let mut new_relations = Vec::new();

            for r in &ontology.relations {
                if r.relation_type == RelationType::ConflictsWith {
                    new_relations.push(
                        LegalRelation::new(
                            r.to.clone(),
                            r.from.clone(),
                            RelationType::ConflictsWith,
                        )
                        .with_strength(r.strength),
                    );
                }
            }

            new_relations
        })
    }
}

/// Converts concept type to string for RDF export.
fn concept_type_to_string(concept_type: ConceptType) -> &'static str {
    match concept_type {
        ConceptType::Norm => "Norm",
        ConceptType::Actor => "Actor",
        ConceptType::Object => "Object",
        ConceptType::Event => "Event",
        ConceptType::Status => "Status",
        ConceptType::Procedure => "Procedure",
        ConceptType::DocumentType => "DocumentType",
        ConceptType::Jurisdiction => "Jurisdiction",
    }
}

/// Converts relation type to string for RDF export.
fn relation_type_to_string(relation_type: RelationType) -> &'static str {
    match relation_type {
        RelationType::IsA => "isA",
        RelationType::PartOf => "partOf",
        RelationType::Causes => "causes",
        RelationType::ConflictsWith => "conflictsWith",
        RelationType::Supports => "supports",
        RelationType::References => "references",
        RelationType::Precedes => "precedes",
        RelationType::AppliesIn => "appliesIn",
        RelationType::Modifies => "modifies",
        RelationType::Implements => "implements",
    }
}

/// Legal ontology builder for common legal domains
pub struct LegalOntologyBuilder;

impl LegalOntologyBuilder {
    /// Builds a contract law ontology.
    pub fn build_contract_ontology() -> LegalOntology {
        let mut ontology = LegalOntology::new("contract_law");

        // Core concepts
        let contract = LegalConcept::new("contract", "Contract", ConceptType::Object)
            .with_description("A legally binding agreement between parties");
        let offer = LegalConcept::new("offer", "Offer", ConceptType::Event)
            .with_description("A promise to enter into a contract");
        let acceptance = LegalConcept::new("acceptance", "Acceptance", ConceptType::Event)
            .with_description("Agreement to the terms of an offer");
        let consideration =
            LegalConcept::new("consideration", "Consideration", ConceptType::Object)
                .with_description("Something of value exchanged in a contract");

        ontology.add_concept(contract);
        ontology.add_concept(offer);
        ontology.add_concept(acceptance);
        ontology.add_concept(consideration);

        // Relations
        ontology
            .add_relation(LegalRelation::new(
                "offer",
                "contract",
                RelationType::PartOf,
            ))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new(
                "acceptance",
                "contract",
                RelationType::PartOf,
            ))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new(
                "consideration",
                "contract",
                RelationType::PartOf,
            ))
            .unwrap();

        ontology
    }

    /// Builds a tort law ontology.
    pub fn build_tort_ontology() -> LegalOntology {
        let mut ontology = LegalOntology::new("tort_law");

        // Core concepts
        let tort = LegalConcept::new("tort", "Tort", ConceptType::Event)
            .with_description("A civil wrong causing harm or loss");
        let negligence = LegalConcept::new("negligence", "Negligence", ConceptType::Event)
            .with_description("Failure to exercise reasonable care");
        let intentional_tort =
            LegalConcept::new("intentional_tort", "Intentional Tort", ConceptType::Event)
                .with_description("Deliberate harmful act");
        let strict_liability =
            LegalConcept::new("strict_liability", "Strict Liability", ConceptType::Norm)
                .with_description("Liability without fault");

        ontology.add_concept(tort.clone());
        ontology.add_concept(negligence);
        ontology.add_concept(intentional_tort);
        ontology.add_concept(strict_liability);

        // Relations
        ontology
            .add_relation(LegalRelation::new("negligence", "tort", RelationType::IsA))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new(
                "intentional_tort",
                "tort",
                RelationType::IsA,
            ))
            .unwrap();

        ontology
    }

    /// Builds a criminal law ontology.
    pub fn build_criminal_ontology() -> LegalOntology {
        let mut ontology = LegalOntology::new("criminal_law");

        // Core concepts
        let crime = LegalConcept::new("crime", "Crime", ConceptType::Event)
            .with_description("An act punishable by law");
        let felony = LegalConcept::new("felony", "Felony", ConceptType::Event)
            .with_description("Serious crime punishable by imprisonment");
        let misdemeanor = LegalConcept::new("misdemeanor", "Misdemeanor", ConceptType::Event)
            .with_description("Less serious crime");
        let mens_rea = LegalConcept::new("mens_rea", "Mens Rea", ConceptType::Object)
            .with_description("Criminal intent");
        let actus_reus = LegalConcept::new("actus_reus", "Actus Reus", ConceptType::Object)
            .with_description("Criminal act");

        ontology.add_concept(crime.clone());
        ontology.add_concept(felony);
        ontology.add_concept(misdemeanor);
        ontology.add_concept(mens_rea);
        ontology.add_concept(actus_reus);

        // Relations
        ontology
            .add_relation(LegalRelation::new("felony", "crime", RelationType::IsA))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new(
                "misdemeanor",
                "crime",
                RelationType::IsA,
            ))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new(
                "mens_rea",
                "crime",
                RelationType::PartOf,
            ))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new(
                "actus_reus",
                "crime",
                RelationType::PartOf,
            ))
            .unwrap();

        ontology
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_concept() {
        let concept = LegalConcept::new("contract", "Contract", ConceptType::Object)
            .with_description("A binding agreement")
            .with_property("jurisdiction", "US");

        assert_eq!(concept.id, "contract");
        assert_eq!(concept.label, "Contract");
        assert_eq!(concept.concept_type, ConceptType::Object);
        assert_eq!(concept.description, "A binding agreement");
        assert_eq!(
            concept.properties.get("jurisdiction"),
            Some(&"US".to_string())
        );
    }

    #[test]
    fn test_legal_relation() {
        let relation = LegalRelation::new("offer", "contract", RelationType::PartOf)
            .with_strength(0.9)
            .with_property("required", "true");

        assert_eq!(relation.from, "offer");
        assert_eq!(relation.to, "contract");
        assert_eq!(relation.relation_type, RelationType::PartOf);
        assert!((relation.strength - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ontology_basic() {
        let mut ontology = LegalOntology::new("test");

        let contract = LegalConcept::new("contract", "Contract", ConceptType::Object);
        let offer = LegalConcept::new("offer", "Offer", ConceptType::Event);

        ontology.add_concept(contract);
        ontology.add_concept(offer);

        assert_eq!(ontology.concepts.len(), 2);
        assert!(ontology.get_concept("contract").is_some());
    }

    #[test]
    fn test_ontology_relations() {
        let mut ontology = LegalOntology::new("test");

        let contract = LegalConcept::new("contract", "Contract", ConceptType::Object);
        let offer = LegalConcept::new("offer", "Offer", ConceptType::Event);

        ontology.add_concept(contract);
        ontology.add_concept(offer);

        let relation = LegalRelation::new("offer", "contract", RelationType::PartOf);
        ontology.add_relation(relation).unwrap();

        assert_eq!(ontology.relations.len(), 1);
        assert_eq!(ontology.get_relations_from("offer").len(), 1);
        assert_eq!(ontology.get_relations_to("contract").len(), 1);
    }

    #[test]
    fn test_is_subtype_of() {
        let mut ontology = LegalOntology::new("test");

        let crime = LegalConcept::new("crime", "Crime", ConceptType::Event);
        let felony = LegalConcept::new("felony", "Felony", ConceptType::Event);
        let murder = LegalConcept::new("murder", "Murder", ConceptType::Event);

        ontology.add_concept(crime);
        ontology.add_concept(felony);
        ontology.add_concept(murder);

        ontology
            .add_relation(LegalRelation::new("felony", "crime", RelationType::IsA))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new("murder", "felony", RelationType::IsA))
            .unwrap();

        assert!(ontology.is_subtype_of("felony", "crime"));
        assert!(ontology.is_subtype_of("murder", "felony"));
        assert!(ontology.is_subtype_of("murder", "crime")); // Transitive
        assert!(!ontology.is_subtype_of("crime", "felony"));
    }

    #[test]
    fn test_get_ancestors() {
        let mut ontology = LegalOntology::new("test");

        let crime = LegalConcept::new("crime", "Crime", ConceptType::Event);
        let felony = LegalConcept::new("felony", "Felony", ConceptType::Event);
        let murder = LegalConcept::new("murder", "Murder", ConceptType::Event);

        ontology.add_concept(crime);
        ontology.add_concept(felony);
        ontology.add_concept(murder);

        ontology
            .add_relation(LegalRelation::new("felony", "crime", RelationType::IsA))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new("murder", "felony", RelationType::IsA))
            .unwrap();

        let ancestors = ontology.get_ancestors("murder");
        assert_eq!(ancestors.len(), 2);
        assert!(ancestors.iter().any(|c| c.id == "felony"));
        assert!(ancestors.iter().any(|c| c.id == "crime"));
    }

    #[test]
    fn test_inference() {
        let mut ontology = LegalOntology::new("test");

        let crime = LegalConcept::new("crime", "Crime", ConceptType::Event);
        let felony = LegalConcept::new("felony", "Felony", ConceptType::Event);
        let murder = LegalConcept::new("murder", "Murder", ConceptType::Event);

        ontology.add_concept(crime);
        ontology.add_concept(felony);
        ontology.add_concept(murder);

        ontology
            .add_relation(LegalRelation::new("felony", "crime", RelationType::IsA))
            .unwrap();
        ontology
            .add_relation(LegalRelation::new("murder", "felony", RelationType::IsA))
            .unwrap();

        ontology.add_inference_rule(inference_rules::transitivity());

        let initial_count = ontology.relations.len();
        let inferred = ontology.infer();

        assert!(inferred > 0);
        assert!(ontology.relations.len() > initial_count);
    }

    #[test]
    fn test_contract_ontology_builder() {
        let ontology = LegalOntologyBuilder::build_contract_ontology();

        assert!(ontology.get_concept("contract").is_some());
        assert!(ontology.get_concept("offer").is_some());
        assert!(ontology.get_concept("acceptance").is_some());
        assert!(ontology.get_concept("consideration").is_some());
    }

    #[test]
    fn test_ontology_statistics() {
        let ontology = LegalOntologyBuilder::build_contract_ontology();
        let stats = ontology.statistics();

        assert_eq!(stats.total_concepts, 4);
        assert!(stats.total_relations > 0);
    }
}
