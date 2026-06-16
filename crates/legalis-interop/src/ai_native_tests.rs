//! Converter-level integration tests for the AI-native formats (v0.3.2):
//! LLM-native, embedding, neural-document, attention-markup, and semantic-chunk.

#[cfg(test)]
mod tests {
    use crate::{LegalConverter, LegalFormat};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    #[test]
    fn test_ai_native_formats_registered() {
        let converter = LegalConverter::new();
        let imports = converter.supported_imports();
        let exports = converter.supported_exports();

        for format in [
            LegalFormat::LlmNative,
            LegalFormat::Embedding,
            LegalFormat::NeuralDocument,
            LegalFormat::AttentionMarkup,
            LegalFormat::SemanticChunk,
        ] {
            assert!(imports.contains(&format), "import missing {format:?}");
            assert!(exports.contains(&format), "export missing {format:?}");
        }
    }

    #[test]
    fn test_ai_native_format_extensions() {
        assert_eq!(LegalFormat::LlmNative.extension(), "llm.json");
        assert_eq!(LegalFormat::Embedding.extension(), "emb.json");
        assert_eq!(LegalFormat::NeuralDocument.extension(), "neural.json");
        assert_eq!(LegalFormat::AttentionMarkup.extension(), "attn.json");
        assert_eq!(LegalFormat::SemanticChunk.extension(), "chunks.json");
    }

    #[test]
    fn test_ai_native_format_from_extension() {
        assert_eq!(
            LegalFormat::from_extension("llm.json"),
            Some(LegalFormat::LlmNative)
        );
        assert_eq!(
            LegalFormat::from_extension("emb"),
            Some(LegalFormat::Embedding)
        );
        assert_eq!(
            LegalFormat::from_extension("neural"),
            Some(LegalFormat::NeuralDocument)
        );
        assert_eq!(
            LegalFormat::from_extension("attn"),
            Some(LegalFormat::AttentionMarkup)
        );
        assert_eq!(
            LegalFormat::from_extension("chunks"),
            Some(LegalFormat::SemanticChunk)
        );
    }

    fn sample_statute() -> Statute {
        Statute::new(
            "voting-rights",
            "Voting Rights",
            Effect::new(EffectType::Grant, "Grant the right to vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_jurisdiction("US")
    }

    #[test]
    fn test_converter_llm_native_roundtrip() {
        let mut converter = LegalConverter::new();
        let statute = sample_statute();

        let (output, export_report) = converter
            .export(&[statute], LegalFormat::LlmNative)
            .unwrap();
        assert_eq!(export_report.statutes_converted, 1);
        assert!(output.contains("legalis.llm-native/v1"));

        let (imported, import_report) = converter.import(&output, LegalFormat::LlmNative).unwrap();
        assert_eq!(import_report.statutes_converted, 1);
        assert_eq!(imported[0].id, "voting-rights");
    }

    #[test]
    fn test_converter_ai_native_roundtrips_all() {
        let mut converter = LegalConverter::new();
        let statute = sample_statute();

        for format in [
            LegalFormat::Embedding,
            LegalFormat::NeuralDocument,
            LegalFormat::AttentionMarkup,
            LegalFormat::SemanticChunk,
        ] {
            let (output, export_report) = converter
                .export(std::slice::from_ref(&statute), format)
                .unwrap();
            assert_eq!(export_report.statutes_converted, 1, "export {format:?}");

            let (imported, import_report) = converter.import(&output, format).unwrap();
            assert_eq!(import_report.statutes_converted, 1, "import {format:?}");
            assert_eq!(imported.len(), 1, "count {format:?}");
            assert_eq!(imported[0].id, "voting-rights", "id {format:?}");
            assert_eq!(
                imported[0].preconditions.len(),
                1,
                "preconditions {format:?}"
            );
        }
    }

    #[test]
    fn test_auto_detect_ai_native_formats() {
        let mut converter = LegalConverter::new();
        let statute = sample_statute();

        for format in [
            LegalFormat::LlmNative,
            LegalFormat::Embedding,
            LegalFormat::NeuralDocument,
            LegalFormat::AttentionMarkup,
            LegalFormat::SemanticChunk,
        ] {
            let (output, _) = converter
                .export(std::slice::from_ref(&statute), format)
                .unwrap();
            let (statutes, report) = converter.auto_import(&output).unwrap();
            assert_eq!(report.source_format, Some(format), "auto-detect {format:?}");
            assert!(!statutes.is_empty(), "auto-detect empty {format:?}");
        }
    }

    #[test]
    fn test_convert_catala_to_semantic_chunk() {
        let mut converter = LegalConverter::new();
        let catala_source = r#"
declaration scope AdultRights:
  context input content integer
"#;
        let (output, report) = converter
            .convert(
                catala_source,
                LegalFormat::Catala,
                LegalFormat::SemanticChunk,
            )
            .unwrap();
        assert!(report.statutes_converted >= 1);
        assert!(output.contains("legalis.semantic-chunk/v1"));
    }
}
