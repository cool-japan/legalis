//! Unicode identifier tests for the Legalis DSL.

use super::*;

mod unicode_identifier_tests {
    use super::*;

    #[test]
    fn test_japanese_identifiers() {
        let input = r#"
        STATUTE 成人投票権: "Adult Voting Rights" {
            WHEN 年齢 >= 18
            THEN GRANT "投票権"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "成人投票権");
        assert_eq!(doc.statutes[0].conditions.len(), 1);
    }

    #[test]
    fn test_chinese_identifiers() {
        let input = r#"
        STATUTE 成年投票权: "Adult Voting Rights" {
            WHEN 年龄 >= 18
            THEN GRANT "投票权"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "成年投票权");
        assert_eq!(doc.statutes[0].conditions.len(), 1);
    }

    #[test]
    fn test_mixed_language_identifiers() {
        let input = r#"
        STATUTE 法律_article_123: "Mixed Language Statute" {
            WHEN 年齢_age >= 18 AND status_状態 = "active"
            THEN GRANT "資格_eligibility"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "法律_article_123");
        assert_eq!(doc.statutes[0].conditions.len(), 1);
    }

    #[test]
    fn test_cyrillic_identifiers() {
        let input = r#"
        STATUTE законопроект_123: "Legal Statute" {
            WHEN возраст >= 18
            THEN GRANT "права"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "законопроект_123");
        assert_eq!(doc.statutes[0].conditions.len(), 1);
    }

    #[test]
    fn test_arabic_identifiers() {
        let input = r#"
        STATUTE قانون_123: "Legal Statute" {
            WHEN العمر >= 18
            THEN GRANT "الحقوق"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "قانون_123");
        assert_eq!(doc.statutes[0].conditions.len(), 1);
    }

    #[test]
    fn test_unicode_with_complex_conditions() {
        let input = r#"
        STATUTE 社会保障: "Social Security" {
            WHEN 年齢 BETWEEN 18 AND 65
            WHEN 収入 <= 5000000
            THEN GRANT "給付金"
            THEN OBLIGATION "税金の支払い"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "社会保障");
        assert_eq!(doc.statutes[0].conditions.len(), 2);
        assert_eq!(doc.statutes[0].effects.len(), 2);
    }

    #[test]
    fn test_unicode_with_imports_and_requires() {
        let input = r#"
        IMPORT "基本法.ldsl"

        STATUTE 派生法規: "Derived Regulation" {
            REQUIRES 基本法
            WHEN 条件 = "満たす"
            THEN GRANT "権利"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.imports.len(), 1);
        assert_eq!(doc.imports[0].path, "基本法.ldsl");
        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "派生法規");
        assert_eq!(doc.statutes[0].requires.len(), 1);
        assert_eq!(doc.statutes[0].requires[0], "基本法");
    }

    #[test]
    fn test_greek_identifiers() {
        let input = r#"
        STATUTE νόμος_123: "Greek Law" {
            WHEN ηλικία >= 18
            THEN GRANT "δικαιώματα"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "νόμος_123");
    }

    #[test]
    fn test_hebrew_identifiers() {
        let input = r#"
        STATUTE חוק_123: "Hebrew Law" {
            WHEN גיל >= 18
            THEN GRANT "זכויות"
        }
        "#;

        let parser = LegalDslParser::new();
        let doc = parser.parse_document(input).unwrap();

        assert_eq!(doc.statutes.len(), 1);
        assert_eq!(doc.statutes[0].id, "חוק_123");
    }
}
