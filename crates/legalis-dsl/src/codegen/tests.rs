//! Tests for the codegen module.

#[cfg(test)]
mod codegen_tests {
    use crate::ast::{ConditionNode, ConditionValue, EffectNode, LegalDocument, StatuteNode};
    use crate::codegen::{
        CSharpGenerator, CodeGenerator, GoGenerator, JavaGenerator, PrologGenerator,
        PythonGenerator, RustGenerator, SqlGenerator, TypeScriptGenerator,
    };

    fn sample_statute() -> StatuteNode {
        StatuteNode {
            id: "voting-rights".to_string(),
            visibility: crate::module_system::Visibility::Private,
            title: "Voting Rights Statute".to_string(),
            conditions: vec![
                ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(18),
                },
                ConditionNode::HasAttribute {
                    key: "citizen".to_string(),
                },
            ],
            effects: vec![EffectNode {
                effect_type: "grant".to_string(),
                description: "Right to vote".to_string(),
                parameters: vec![],
            }],
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec![],
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        }
    }

    #[test]
    fn test_sql_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = SqlGenerator::new();
        let sql = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(sql.contains("CREATE TABLE voting_rights"));
        assert!(sql.contains("age"));
        assert!(sql.contains("citizen"));
    }

    #[test]
    fn test_python_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PythonGenerator::new();
        let py = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(py.contains("def voting_rights"));
        assert!(py.contains("obj.age >= 18"));
        assert!(py.contains("hasattr(obj, 'citizen')"));
    }

    #[test]
    fn test_sql_generator_metadata() {
        let generator = SqlGenerator::new();
        assert_eq!(generator.target_language(), "SQL");
        assert_eq!(generator.file_extension(), "sql");
    }

    #[test]
    fn test_python_generator_metadata() {
        let generator = PythonGenerator::new();
        assert_eq!(generator.target_language(), "Python");
        assert_eq!(generator.file_extension(), "py");
    }

    #[test]
    fn test_prolog_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PrologGenerator::new();
        let pl = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(pl.contains("voting_rights(Entity)"));
        assert!(pl.contains("Entity_age >= 18"));
        assert!(pl.contains("nonvar(Entity_citizen)"));
    }

    #[test]
    fn test_prolog_generator_metadata() {
        let generator = PrologGenerator::new();
        assert_eq!(generator.target_language(), "Prolog");
        assert_eq!(generator.file_extension(), "pl");
    }

    #[test]
    fn test_prolog_module_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PrologGenerator {
            generate_module: true,
            use_dynamic: true,
        };
        let pl = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(pl.contains(":- module(legal_statutes, [])"));
        assert!(pl.contains(":- dynamic voting_rights/1"));
    }

    #[test]
    fn test_prolog_effect_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PrologGenerator::new();
        let pl = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(pl.contains("voting_rights_effect_1"));
        assert!(pl.contains("Right to vote"));
    }

    #[test]
    fn test_sql_roundtrip_validation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = SqlGenerator::new();
        let sql = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        // Verify SQL contains expected keywords
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("PRIMARY KEY"));
        assert!(sql.contains("CHECK"));

        // Verify no syntax errors in basic structure
        assert!(!sql.contains(";;")); // No double semicolons
        assert!(sql.matches('(').count() == sql.matches(')').count()); // Balanced parentheses
    }

    #[test]
    fn test_python_roundtrip_validation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PythonGenerator::new();
        let py = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        // Verify Python contains expected structures
        assert!(py.contains("def "));
        assert!(py.contains("return "));
        assert!(py.contains("from typing import Any"));

        // Verify basic Python syntax
        assert!(py.matches("def ").count() == py.matches("return ").count());
    }

    #[test]
    fn test_prolog_roundtrip_validation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = PrologGenerator::new();
        let pl = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        // Verify Prolog contains expected structures
        assert!(pl.contains("(Entity) :- "));
        assert!(pl.ends_with("\n") || pl.ends_with("."));

        // Verify balanced predicates (all :- have corresponding .)
        assert!(pl.matches(":-").count() <= pl.matches('.').count());
    }

    #[test]
    fn test_complex_document_all_generators() {
        let complex_statute = StatuteNode {
            id: "complex-law".to_string(),
            visibility: crate::module_system::Visibility::Private,
            title: "Complex Law Test".to_string(),
            conditions: vec![ConditionNode::And(
                Box::new(ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">=".to_string(),
                    value: ConditionValue::Number(18),
                }),
                Box::new(ConditionNode::In {
                    field: "status".to_string(),
                    values: vec![
                        ConditionValue::String("citizen".to_string()),
                        ConditionValue::String("resident".to_string()),
                    ],
                }),
            )],
            effects: vec![
                EffectNode {
                    effect_type: "GRANT".to_string(),
                    description: "Voting rights".to_string(),
                    parameters: vec![],
                },
                EffectNode {
                    effect_type: "OBLIGATION".to_string(),
                    description: "Register to vote".to_string(),
                    parameters: vec![],
                },
            ],
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec![],
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        };

        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![complex_statute],
        };

        // Test all generators can handle complex documents
        let sql_gen = SqlGenerator::new();
        let sql = sql_gen
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(sql.len() > 100);

        let py_gen = PythonGenerator::new();
        let py = py_gen
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(py.len() > 100);

        let pl_gen = PrologGenerator::new();
        let pl = pl_gen
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(pl.len() > 100);

        // Test new generators
        let ts_gen = TypeScriptGenerator::new();
        let ts = ts_gen
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(ts.len() > 100);

        let rust_gen = RustGenerator::new();
        let rust = rust_gen
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(rust.len() > 100);

        let go_gen = GoGenerator::new();
        let go = go_gen
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(go.len() > 100);

        let java_gen = JavaGenerator::new();
        let java = java_gen
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(java.len() > 100);

        let cs_gen = CSharpGenerator::new();
        let cs = cs_gen
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(cs.len() > 100);
    }

    #[test]
    fn test_typescript_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = TypeScriptGenerator::new();
        let ts = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(ts.contains("export function"));
        assert!(ts.contains("voting_rights"));
        assert!(ts.contains("entity"));
        assert!(ts.contains(": boolean"));
    }

    #[test]
    fn test_rust_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = RustGenerator::new();
        let rs = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(rs.contains("pub fn voting_rights"));
        assert!(rs.contains("-> bool"));
        assert!(rs.contains("use regex::Regex"));
    }

    #[test]
    fn test_go_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = GoGenerator::new();
        let go = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(go.contains("package statutes"));
        assert!(go.contains("func Voting_rights"));
        assert!(go.contains("bool"));
        assert!(go.contains("import ("));
    }

    #[test]
    fn test_java_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = JavaGenerator::new();
        let java = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(java.contains("public class StatuteValidator"));
        assert!(java.contains("public static boolean votingRights"));
        assert!(java.contains("package com.legal.statutes"));
        assert!(java.contains("import java.util"));
    }

    #[test]
    fn test_typescript_javascript_mode() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let mut generator = TypeScriptGenerator::new();
        generator.use_typescript = false;

        let js = generator
            .generate(&doc)
            .expect("writing to String is infallible");
        assert!(js.contains("export function"));
        assert!(!js.contains(": boolean"));
        assert_eq!(generator.file_extension(), "js");
        assert_eq!(generator.target_language(), "JavaScript");
    }

    #[test]
    fn test_csharp_generation() {
        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![sample_statute()],
        };

        let generator = CSharpGenerator::new();
        let cs = generator
            .generate(&doc)
            .expect("writing to String is infallible");

        assert!(cs.contains("namespace Legal.Statutes"));
        assert!(cs.contains("public static class StatuteValidator"));
        assert!(cs.contains("public static bool VotingRights"));
        assert!(cs.contains("using System"));
    }

    #[test]
    fn test_all_generators_file_extensions() {
        assert_eq!(SqlGenerator::new().file_extension(), "sql");
        assert_eq!(PythonGenerator::new().file_extension(), "py");
        assert_eq!(PrologGenerator::new().file_extension(), "pl");
        assert_eq!(TypeScriptGenerator::new().file_extension(), "ts");
        assert_eq!(RustGenerator::new().file_extension(), "rs");
        assert_eq!(GoGenerator::new().file_extension(), "go");
        assert_eq!(JavaGenerator::new().file_extension(), "java");
        assert_eq!(CSharpGenerator::new().file_extension(), "cs");
    }

    #[test]
    fn test_all_generators_target_languages() {
        assert_eq!(SqlGenerator::new().target_language(), "SQL");
        assert_eq!(PythonGenerator::new().target_language(), "Python");
        assert_eq!(PrologGenerator::new().target_language(), "Prolog");
        assert_eq!(TypeScriptGenerator::new().target_language(), "TypeScript");
        assert_eq!(RustGenerator::new().target_language(), "Rust");
        assert_eq!(GoGenerator::new().target_language(), "Go");
        assert_eq!(JavaGenerator::new().target_language(), "Java");
        assert_eq!(CSharpGenerator::new().target_language(), "C#");
    }
}
