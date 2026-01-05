//! Legal ontology support for Legalis-LOD.
//!
//! This module provides support for various legal and bibliographic ontologies:
//! - FaBiO (FRBR-aligned Bibliographic Ontology)
//! - LKIF-Core (Legal Knowledge Interchange Format)
//! - LegalRuleML
//! - Akoma Ntoso
//! - Custom Legalis Extensions (condition/effect relationships, discretion zones, simulation results)

use crate::{RdfValue, Triple};
use legalis_core::{Condition, EffectType, Statute};

/// FaBiO (FRBR-aligned Bibliographic Ontology) support.
///
/// FaBiO is an ontology for describing bibliographic entities.
/// It implements the FRBR (Functional Requirements for Bibliographic Records) model.
pub mod fabio {
    use super::*;

    /// FaBiO namespace URI.
    pub const NAMESPACE: &str = "http://purl.org/spar/fabio/";

    /// FaBiO entity types for legal documents.
    #[derive(Debug, Clone, Copy)]
    pub enum FabioType {
        /// A legislative act
        LegislativeAct,
        /// A regulation
        Regulation,
        /// A statute
        Statute,
        /// A legal case
        LegalCase,
        /// A legal decision
        LegalDecision,
        /// A legal opinion
        LegalOpinion,
    }

    impl FabioType {
        /// Returns the FaBiO type URI.
        pub fn uri(&self) -> String {
            let type_name = match self {
                Self::LegislativeAct => "LegislativeAct",
                Self::Regulation => "Regulation",
                Self::Statute => "Statute",
                Self::LegalCase => "LegalCase",
                Self::LegalDecision => "LegalDecision",
                Self::LegalOpinion => "LegalOpinion",
            };
            format!("{}{}", NAMESPACE, type_name)
        }
    }

    /// Generates FaBiO triples for a statute.
    pub fn statute_to_fabio_triples(subject_uri: &str, statute: &Statute) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type declaration
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(FabioType::Statute.uri()),
        });

        // Has title
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: format!("{}hasTitle", NAMESPACE),
            object: RdfValue::string(&statute.title),
        });

        // Jurisdiction as publication place
        if let Some(ref jurisdiction) = statute.jurisdiction {
            triples.push(Triple {
                subject: subject_uri.to_string(),
                predicate: format!("{}hasPublicationPlace", NAMESPACE),
                object: RdfValue::string(jurisdiction),
            });
        }

        // Effective date as publication date
        if let Some(effective_date) = statute.temporal_validity.effective_date {
            triples.push(Triple {
                subject: subject_uri.to_string(),
                predicate: format!("{}hasPublicationDate", NAMESPACE),
                object: RdfValue::date(effective_date),
            });
        }

        triples
    }
}

/// LKIF-Core (Legal Knowledge Interchange Format) support.
///
/// LKIF-Core is an ontology for representing legal knowledge and reasoning.
pub mod lkif {
    use super::*;

    /// LKIF-Core namespace URI.
    pub const NAMESPACE: &str = "http://www.estrellaproject.org/lkif-core/";

    /// LKIF norm types.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum NormType {
        /// A right
        Right,
        /// An obligation
        Obligation,
        /// A prohibition
        Prohibition,
        /// A permission
        Permission,
        /// A power
        Power,
    }

    impl NormType {
        /// Returns the LKIF norm type URI.
        pub fn uri(&self) -> String {
            let type_name = match self {
                Self::Right => "Right",
                Self::Obligation => "Obligation",
                Self::Prohibition => "Prohibition",
                Self::Permission => "Permission",
                Self::Power => "Power",
            };
            format!("{}norm#{}", NAMESPACE, type_name)
        }

        /// Maps an effect type to LKIF norm type.
        pub fn from_effect_type(effect_type: &EffectType) -> Option<Self> {
            match effect_type {
                EffectType::Grant => Some(Self::Right),
                EffectType::Obligation => Some(Self::Obligation),
                EffectType::Prohibition => Some(Self::Prohibition),
                _ => None,
            }
        }
    }

    /// Generates LKIF triples for a statute.
    pub fn statute_to_lkif_triples(
        subject_uri: &str,
        statute: &Statute,
        base_uri: &str,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Norm type
        if let Some(norm_type) = NormType::from_effect_type(&statute.effect.effect_type) {
            let norm_uri = format!("{}norm/{}", base_uri, &statute.id);
            triples.push(Triple {
                subject: norm_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri(norm_type.uri()),
            });

            // Link statute to norm
            triples.push(Triple {
                subject: subject_uri.to_string(),
                predicate: format!("{}norm#qualified_by", NAMESPACE),
                object: RdfValue::Uri(norm_uri.clone()),
            });

            // Norm description
            triples.push(Triple {
                subject: norm_uri,
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(&statute.effect.description),
            });
        }

        // Legal source
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}legal-action#Legal_Source", NAMESPACE)),
        });

        triples
    }

    /// Generates LKIF triples for a condition as a qualification.
    #[allow(dead_code)]
    pub fn condition_to_lkif_triples(condition_uri: &str, _condition: &Condition) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Mark as qualification
        triples.push(Triple {
            subject: condition_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}expression#Qualification", NAMESPACE)),
        });

        triples
    }
}

/// LegalRuleML ontology support.
///
/// LegalRuleML is an OASIS standard for representing legal rules.
pub mod legalruleml {
    use super::*;
    use legalis_core::ComparisonOp;

    /// LegalRuleML namespace URI.
    pub const NAMESPACE: &str = "http://docs.oasis-open.org/legalruleml/ns/v1.0/";

    /// Generates LegalRuleML triples for a statute.
    pub fn statute_to_legalruleml_triples(
        subject_uri: &str,
        statute: &Statute,
        base_uri: &str,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type as legal rule
        let rule_uri = format!("{}rule/{}", base_uri, &statute.id);
        triples.push(Triple {
            subject: rule_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}LegalRule", NAMESPACE)),
        });

        // Link statute to rule
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: format!("{}definesRule", NAMESPACE),
            object: RdfValue::Uri(rule_uri.clone()),
        });

        // Rule has conditions (preconditions)
        if !statute.preconditions.is_empty() {
            let conditions_uri = format!("{}conditions/{}", base_uri, &statute.id);
            triples.push(Triple {
                subject: rule_uri.clone(),
                predicate: format!("{}hasPrecondition", NAMESPACE),
                object: RdfValue::Uri(conditions_uri.clone()),
            });

            // Condition set
            triples.push(Triple {
                subject: conditions_uri,
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri(format!("{}ConditionSet", NAMESPACE)),
            });
        }

        // Rule has conclusion (effect)
        let conclusion_uri = format!("{}conclusion/{}", base_uri, &statute.id);
        triples.push(Triple {
            subject: rule_uri.clone(),
            predicate: format!("{}hasConclusion", NAMESPACE),
            object: RdfValue::Uri(conclusion_uri.clone()),
        });

        triples.push(Triple {
            subject: conclusion_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}Conclusion", NAMESPACE)),
        });

        triples.push(Triple {
            subject: conclusion_uri,
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(&statute.effect.description),
        });

        // Rule strength (normative vs defeasible)
        let strength = if statute.discretion_logic.is_some() {
            "defeasible"
        } else {
            "strict"
        };
        triples.push(Triple {
            subject: rule_uri,
            predicate: format!("{}hasStrength", NAMESPACE),
            object: RdfValue::Uri(format!("{}Strength/{}", NAMESPACE, strength)),
        });

        triples
    }

    /// Maps condition to LegalRuleML atom.
    #[allow(dead_code)]
    pub fn condition_to_atom(atom_uri: &str, condition: &Condition) -> Vec<Triple> {
        let mut triples = Vec::new();

        triples.push(Triple {
            subject: atom_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}Atom", NAMESPACE)),
        });

        match condition {
            Condition::Age { operator, value } => {
                triples.push(Triple {
                    subject: atom_uri.to_string(),
                    predicate: format!("{}hasPredicate", NAMESPACE),
                    object: RdfValue::string("age"),
                });
                triples.push(Triple {
                    subject: atom_uri.to_string(),
                    predicate: format!("{}hasOperator", NAMESPACE),
                    object: RdfValue::string(operator_to_string(*operator)),
                });
                triples.push(Triple {
                    subject: atom_uri.to_string(),
                    predicate: format!("{}hasValue", NAMESPACE),
                    object: RdfValue::integer(*value as i64),
                });
            }
            Condition::Income { operator, value } => {
                triples.push(Triple {
                    subject: atom_uri.to_string(),
                    predicate: format!("{}hasPredicate", NAMESPACE),
                    object: RdfValue::string("income"),
                });
                triples.push(Triple {
                    subject: atom_uri.to_string(),
                    predicate: format!("{}hasOperator", NAMESPACE),
                    object: RdfValue::string(operator_to_string(*operator)),
                });
                triples.push(Triple {
                    subject: atom_uri.to_string(),
                    predicate: format!("{}hasValue", NAMESPACE),
                    object: RdfValue::integer(*value as i64),
                });
            }
            _ => {}
        }

        triples
    }

    fn operator_to_string(op: ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "equal",
            ComparisonOp::NotEqual => "notEqual",
            ComparisonOp::GreaterThan => "greaterThan",
            ComparisonOp::GreaterOrEqual => "greaterOrEqual",
            ComparisonOp::LessThan => "lessThan",
            ComparisonOp::LessOrEqual => "lessOrEqual",
        }
    }
}

/// Akoma Ntoso ontology support.
///
/// Akoma Ntoso is an XML schema and ontology for legal documents.
pub mod akoma_ntoso {
    use super::*;

    /// Akoma Ntoso namespace URI.
    pub const NAMESPACE: &str = "http://docs.oasis-open.org/legaldocml/ns/akn/3.0/";

    /// Akoma Ntoso document types.
    #[derive(Debug, Clone, Copy)]
    pub enum DocumentType {
        /// An act
        Act,
        /// A bill
        Bill,
        /// A regulation
        Regulation,
        /// A judgment
        Judgment,
    }

    impl DocumentType {
        /// Returns the Akoma Ntoso document type URI.
        pub fn uri(&self) -> String {
            let type_name = match self {
                Self::Act => "act",
                Self::Bill => "bill",
                Self::Regulation => "regulation",
                Self::Judgment => "judgment",
            };
            format!("{}{}", NAMESPACE, type_name)
        }
    }

    /// Generates Akoma Ntoso triples for a statute.
    pub fn statute_to_akoma_ntoso_triples(
        subject_uri: &str,
        statute: &Statute,
        base_uri: &str,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Document type
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(DocumentType::Act.uri()),
        });

        // FRBRWork level (abstract work)
        let work_uri = format!("{}work/{}", base_uri, &statute.id);
        triples.push(Triple {
            subject: work_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}FRBRWork", NAMESPACE)),
        });

        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: format!("{}hasFRBRWork", NAMESPACE),
            object: RdfValue::Uri(work_uri.clone()),
        });

        // FRBRExpression level (specific language version)
        let expression_uri = format!("{}expression/{}", base_uri, &statute.id);
        triples.push(Triple {
            subject: expression_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}FRBRExpression", NAMESPACE)),
        });

        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: format!("{}hasFRBRExpression", NAMESPACE),
            object: RdfValue::Uri(expression_uri),
        });

        // Document title
        triples.push(Triple {
            subject: work_uri.clone(),
            predicate: format!("{}FRBRname", NAMESPACE),
            object: RdfValue::string(&statute.title),
        });

        // Document date (effective date)
        if let Some(effective_date) = statute.temporal_validity.effective_date {
            triples.push(Triple {
                subject: work_uri,
                predicate: format!("{}FRBRdate", NAMESPACE),
                object: RdfValue::date(effective_date),
            });
        }

        triples
    }

    /// Generates Akoma Ntoso provision triples.
    #[allow(dead_code)]
    pub fn create_provision(provision_uri: &str, content: &str) -> Vec<Triple> {
        let mut triples = Vec::new();

        triples.push(Triple {
            subject: provision_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}provision", NAMESPACE)),
        });

        triples.push(Triple {
            subject: provision_uri.to_string(),
            predicate: format!("{}content", NAMESPACE),
            object: RdfValue::string(content),
        });

        triples
    }
}

/// Custom Legalis ontology extensions.
///
/// This module provides custom vocabulary extensions for:
/// - Condition/effect relationships
/// - Discretion zone modeling
/// - Simulation result representation
pub mod custom {
    use super::*;

    /// Custom Legalis namespace (already defined in standard prefixes).
    pub const NAMESPACE: &str = "https://legalis.dev/ontology#";

    /// Condition-effect relationship types.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ConditionEffectRelation {
        /// Condition is necessary for the effect
        Necessary,
        /// Condition is sufficient for the effect
        Sufficient,
        /// Condition is necessary and sufficient
        NecessaryAndSufficient,
        /// Condition modulates the effect (e.g., determines magnitude)
        Modulates,
        /// Condition blocks the effect
        Blocks,
        /// Condition triggers the effect
        Triggers,
    }

    impl ConditionEffectRelation {
        /// Returns the URI for this relationship type.
        pub fn uri(&self) -> String {
            let relation_name = match self {
                Self::Necessary => "necessaryFor",
                Self::Sufficient => "sufficientFor",
                Self::NecessaryAndSufficient => "necessaryAndSufficientFor",
                Self::Modulates => "modulates",
                Self::Blocks => "blocks",
                Self::Triggers => "triggers",
            };
            format!("{}{}", NAMESPACE, relation_name)
        }

        /// Returns the label for this relationship.
        pub fn label(&self) -> &'static str {
            match self {
                Self::Necessary => "Necessary Condition",
                Self::Sufficient => "Sufficient Condition",
                Self::NecessaryAndSufficient => "Necessary and Sufficient Condition",
                Self::Modulates => "Modulating Condition",
                Self::Blocks => "Blocking Condition",
                Self::Triggers => "Triggering Condition",
            }
        }
    }

    /// Discretion zone modeling vocabulary.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DiscretionZoneType {
        /// Mandatory zone - no discretion
        Mandatory,
        /// Advisory zone - full discretion
        Advisory,
        /// Constrained zone - limited discretion with boundaries
        Constrained,
        /// Presumptive zone - default action with override
        Presumptive,
    }

    impl DiscretionZoneType {
        /// Returns the URI for this discretion zone type.
        pub fn uri(&self) -> String {
            let zone_name = match self {
                Self::Mandatory => "MandatoryZone",
                Self::Advisory => "AdvisoryZone",
                Self::Constrained => "ConstrainedZone",
                Self::Presumptive => "PresumptiveZone",
            };
            format!("{}{}", NAMESPACE, zone_name)
        }

        /// Returns the label.
        pub fn label(&self) -> &'static str {
            match self {
                Self::Mandatory => "Mandatory Discretion Zone",
                Self::Advisory => "Advisory Discretion Zone",
                Self::Constrained => "Constrained Discretion Zone",
                Self::Presumptive => "Presumptive Discretion Zone",
            }
        }

        /// Returns the definition.
        pub fn definition(&self) -> &'static str {
            match self {
                Self::Mandatory => "A zone where the action is mandatory with no discretion",
                Self::Advisory => "A zone where the decision-maker has full discretion",
                Self::Constrained => "A zone with limited discretion within specified boundaries",
                Self::Presumptive => "A zone with a default action that can be overridden",
            }
        }
    }

    /// Simulation result types.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SimulationResultType {
        /// Success - effect was applied
        Success,
        /// Failure - effect was not applied
        Failure,
        /// Partial - effect was partially applied
        Partial,
        /// Deferred - decision was postponed
        Deferred,
        /// Alternative - alternative action was taken
        Alternative,
    }

    impl SimulationResultType {
        /// Returns the URI for this result type.
        pub fn uri(&self) -> String {
            let result_name = match self {
                Self::Success => "SuccessResult",
                Self::Failure => "FailureResult",
                Self::Partial => "PartialResult",
                Self::Deferred => "DeferredResult",
                Self::Alternative => "AlternativeResult",
            };
            format!("{}{}", NAMESPACE, result_name)
        }

        /// Returns the label.
        pub fn label(&self) -> &'static str {
            match self {
                Self::Success => "Success Result",
                Self::Failure => "Failure Result",
                Self::Partial => "Partial Result",
                Self::Deferred => "Deferred Result",
                Self::Alternative => "Alternative Result",
            }
        }
    }

    /// Generates condition-effect relationship triples.
    pub fn create_condition_effect_relationship(
        condition_uri: &str,
        effect_uri: &str,
        relation: ConditionEffectRelation,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        triples.push(Triple {
            subject: condition_uri.to_string(),
            predicate: relation.uri(),
            object: RdfValue::Uri(effect_uri.to_string()),
        });

        // Add inverse relationship
        let inverse = match relation {
            ConditionEffectRelation::Necessary => "hasNecessaryCondition",
            ConditionEffectRelation::Sufficient => "hasSufficientCondition",
            ConditionEffectRelation::NecessaryAndSufficient => "hasNecessaryAndSufficientCondition",
            ConditionEffectRelation::Modulates => "isModulatedBy",
            ConditionEffectRelation::Blocks => "isBlockedBy",
            ConditionEffectRelation::Triggers => "isTriggeredBy",
        };

        triples.push(Triple {
            subject: effect_uri.to_string(),
            predicate: format!("{}{}", NAMESPACE, inverse),
            object: RdfValue::Uri(condition_uri.to_string()),
        });

        triples
    }

    /// Generates discretion zone triples.
    pub fn create_discretion_zone(
        zone_uri: &str,
        zone_type: DiscretionZoneType,
        statute_uri: Option<&str>,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type
        triples.push(Triple {
            subject: zone_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}DiscretionZone", NAMESPACE)),
        });

        triples.push(Triple {
            subject: zone_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(zone_type.uri()),
        });

        // Label
        triples.push(Triple {
            subject: zone_uri.to_string(),
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(zone_type.label()),
        });

        // Definition
        triples.push(Triple {
            subject: zone_uri.to_string(),
            predicate: "skos:definition".to_string(),
            object: RdfValue::string(zone_type.definition()),
        });

        // Link to statute if provided
        if let Some(statute) = statute_uri {
            triples.push(Triple {
                subject: statute.to_string(),
                predicate: format!("{}hasDiscretionZone", NAMESPACE),
                object: RdfValue::Uri(zone_uri.to_string()),
            });
        }

        triples
    }

    /// Generates simulation result triples.
    pub fn create_simulation_result(
        result_uri: &str,
        result_type: SimulationResultType,
        statute_uri: &str,
        explanation: Option<&str>,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type
        triples.push(Triple {
            subject: result_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("{}SimulationResult", NAMESPACE)),
        });

        triples.push(Triple {
            subject: result_uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(result_type.uri()),
        });

        // Label
        triples.push(Triple {
            subject: result_uri.to_string(),
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(result_type.label()),
        });

        // Link to statute
        triples.push(Triple {
            subject: result_uri.to_string(),
            predicate: format!("{}appliesTo", NAMESPACE),
            object: RdfValue::Uri(statute_uri.to_string()),
        });

        // Explanation if provided
        if let Some(exp) = explanation {
            triples.push(Triple {
                subject: result_uri.to_string(),
                predicate: format!("{}explanation", NAMESPACE),
                object: RdfValue::string(exp),
            });
        }

        triples
    }

    /// Adds discretion zone modeling to statute with discretion logic.
    pub fn add_discretion_zone_to_statute(
        statute: &Statute,
        subject_uri: &str,
        base_uri: &str,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        if statute.discretion_logic.is_some() {
            let zone_uri = format!("{}discretion-zone/{}", base_uri, &statute.id);

            // Create a presumptive zone (default for discretion logic)
            triples.extend(create_discretion_zone(
                &zone_uri,
                DiscretionZoneType::Presumptive,
                Some(subject_uri),
            ));
        }

        triples
    }
}

/// Generates triples for all supported ontologies.
pub fn generate_all_ontology_triples(
    subject_uri: &str,
    statute: &Statute,
    base_uri: &str,
) -> Vec<Triple> {
    let mut triples = Vec::new();

    // FaBiO triples
    triples.extend(fabio::statute_to_fabio_triples(subject_uri, statute));

    // LKIF triples
    triples.extend(lkif::statute_to_lkif_triples(
        subject_uri,
        statute,
        base_uri,
    ));

    // LegalRuleML triples
    triples.extend(legalruleml::statute_to_legalruleml_triples(
        subject_uri,
        statute,
        base_uri,
    ));

    // Akoma Ntoso triples
    triples.extend(akoma_ntoso::statute_to_akoma_ntoso_triples(
        subject_uri,
        statute,
        base_uri,
    ));

    // Custom Legalis extensions - discretion zone modeling
    triples.extend(custom::add_discretion_zone_to_statute(
        statute,
        subject_uri,
        base_uri,
    ));

    triples
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Effect, EffectType};

    fn sample_statute() -> Statute {
        Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grant test rights"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_fabio_statute_triples() {
        let statute = sample_statute();
        let triples = fabio::statute_to_fabio_triples("http://example.org/statute/test", &statute);

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"));
        assert!(triples.iter().any(|t| t.predicate.contains("hasTitle")));
    }

    #[test]
    fn test_lkif_statute_triples() {
        let statute = sample_statute();
        let triples = lkif::statute_to_lkif_triples(
            "http://example.org/statute/test",
            &statute,
            "http://example.org/",
        );

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"));
    }

    #[test]
    fn test_legalruleml_statute_triples() {
        let statute = sample_statute();
        let triples = legalruleml::statute_to_legalruleml_triples(
            "http://example.org/statute/test",
            &statute,
            "http://example.org/",
        );

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate.contains("definesRule")));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate.contains("hasConclusion"))
        );
    }

    #[test]
    fn test_akoma_ntoso_statute_triples() {
        let statute = sample_statute();
        let triples = akoma_ntoso::statute_to_akoma_ntoso_triples(
            "http://example.org/statute/test",
            &statute,
            "http://example.org/",
        );

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate.contains("FRBR")));
    }

    #[test]
    fn test_generate_all_ontology_triples() {
        let statute = sample_statute();
        let triples = generate_all_ontology_triples(
            "http://example.org/statute/test",
            &statute,
            "http://example.org/",
        );

        // Should have triples from all ontologies
        assert!(triples.len() > 10);
        assert!(triples.iter().any(|t| t.predicate.contains("hasTitle"))); // FaBiO
        assert!(triples.iter().any(|t| t.predicate.contains("qualified_by"))); // LKIF
        assert!(triples.iter().any(|t| t.predicate.contains("definesRule"))); // LegalRuleML
        assert!(triples.iter().any(|t| t.predicate.contains("FRBR"))); // Akoma Ntoso
    }

    #[test]
    fn test_fabio_types() {
        assert!(fabio::FabioType::Statute.uri().contains("Statute"));
        assert!(
            fabio::FabioType::LegislativeAct
                .uri()
                .contains("LegislativeAct")
        );
    }

    #[test]
    fn test_lkif_norm_types() {
        assert_eq!(
            lkif::NormType::from_effect_type(&EffectType::Grant),
            Some(lkif::NormType::Right)
        );
        assert_eq!(
            lkif::NormType::from_effect_type(&EffectType::Obligation),
            Some(lkif::NormType::Obligation)
        );
    }

    #[test]
    fn test_custom_condition_effect_relationship() {
        let triples = custom::create_condition_effect_relationship(
            "http://example.org/condition/1",
            "http://example.org/effect/1",
            custom::ConditionEffectRelation::Necessary,
        );

        assert_eq!(triples.len(), 2);
        assert!(triples.iter().any(|t| t.predicate.contains("necessaryFor")));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate.contains("hasNecessaryCondition"))
        );
    }

    #[test]
    fn test_custom_discretion_zone() {
        let triples = custom::create_discretion_zone(
            "http://example.org/zone/1",
            custom::DiscretionZoneType::Presumptive,
            Some("http://example.org/statute/1"),
        );

        assert!(!triples.is_empty());
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("DiscretionZone")))
        );
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("PresumptiveZone")))
        );
        assert!(
            triples
                .iter()
                .any(|t| t.predicate.contains("hasDiscretionZone"))
        );
    }

    #[test]
    fn test_custom_simulation_result() {
        let triples = custom::create_simulation_result(
            "http://example.org/result/1",
            custom::SimulationResultType::Success,
            "http://example.org/statute/1",
            Some("All conditions met"),
        );

        assert!(!triples.is_empty());
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("SimulationResult")))
        );
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("SuccessResult")))
        );
        assert!(triples.iter().any(|t| t.predicate.contains("appliesTo")));
        assert!(triples.iter().any(|t| t.predicate.contains("explanation")));
    }

    #[test]
    fn test_custom_discretion_zone_types() {
        assert_eq!(
            custom::DiscretionZoneType::Mandatory.label(),
            "Mandatory Discretion Zone"
        );
        assert!(
            custom::DiscretionZoneType::Constrained
                .uri()
                .contains("ConstrainedZone")
        );
        assert!(!custom::DiscretionZoneType::Advisory.definition().is_empty());
    }

    #[test]
    fn test_custom_simulation_result_types() {
        assert_eq!(
            custom::SimulationResultType::Partial.label(),
            "Partial Result"
        );
        assert!(
            custom::SimulationResultType::Deferred
                .uri()
                .contains("DeferredResult")
        );
    }

    #[test]
    fn test_custom_condition_effect_relation_types() {
        assert_eq!(
            custom::ConditionEffectRelation::Sufficient.label(),
            "Sufficient Condition"
        );
        assert!(
            custom::ConditionEffectRelation::Modulates
                .uri()
                .contains("modulates")
        );
    }
}
