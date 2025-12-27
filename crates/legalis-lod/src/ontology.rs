//! Legal ontology support for Legalis-LOD.
//!
//! This module provides support for various legal and bibliographic ontologies:
//! - FaBiO (FRBR-aligned Bibliographic Ontology)
//! - LKIF-Core (Legal Knowledge Interchange Format)
//! - LegalRuleML
//! - Akoma Ntoso

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
    pub fn condition_to_lkif_triples(
        condition_uri: &str,
        _condition: &Condition,
    ) -> Vec<Triple> {
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
    pub fn condition_to_atom(
        atom_uri: &str,
        condition: &Condition,
    ) -> Vec<Triple> {
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
    pub fn create_provision(
        provision_uri: &str,
        content: &str,
    ) -> Vec<Triple> {
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
    triples.extend(lkif::statute_to_lkif_triples(subject_uri, statute, base_uri));

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
        assert!(triples.iter().any(|t| t.predicate.contains("hasConclusion")));
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
        assert!(fabio::FabioType::LegislativeAct.uri().contains("LegislativeAct"));
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
}
