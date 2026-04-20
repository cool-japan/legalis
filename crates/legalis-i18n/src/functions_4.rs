//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[cfg(test)]
mod real_time_interpretation_tests {
    use crate::*;
    #[test]
    fn test_audio_quality_display() {
        assert_eq!(AudioQuality::Low.to_string(), "Low (8kHz)");
        assert_eq!(AudioQuality::Medium.to_string(), "Medium (16kHz)");
        assert_eq!(AudioQuality::High.to_string(), "High (44.1kHz)");
        assert_eq!(AudioQuality::Studio.to_string(), "Studio (48kHz+)");
    }
    #[test]
    fn test_transcription_segment_creation() {
        let locale = Locale::new("en").with_country("US");
        let segment = TranscriptionSegment::new("This is a test.", 0, 3000, locale.clone());
        assert_eq!(segment.text, "This is a test.");
        assert_eq!(segment.start_ms, 0);
        assert_eq!(segment.end_ms, 3000);
        assert_eq!(segment.confidence, 1.0);
        assert!(segment.speaker.is_none());
    }
    #[test]
    fn test_transcription_segment_with_speaker() {
        let locale = Locale::new("en").with_country("US");
        let segment = TranscriptionSegment::new("Test", 0, 1000, locale)
            .with_speaker("Judge")
            .with_confidence(0.95);
        assert_eq!(segment.speaker, Some("Judge".to_string()));
        assert_eq!(segment.confidence, 0.95);
    }
    #[test]
    fn test_transcription_segment_duration() {
        let locale = Locale::new("en").with_country("US");
        let segment = TranscriptionSegment::new("Test", 1000, 5000, locale);
        assert_eq!(segment.duration_ms(), 4000);
    }
    #[test]
    fn test_transcription_segment_format() {
        let locale = Locale::new("en").with_country("US");
        let segment =
            TranscriptionSegment::new("Test speech", 0, 3000, locale).with_speaker("Attorney");
        let formatted = segment.format_with_timestamp();
        assert!(formatted.contains("00:00 - 00:03"));
        assert!(formatted.contains("Attorney"));
        assert!(formatted.contains("Test speech"));
    }
    #[test]
    fn test_legal_speech_domain_display() {
        assert_eq!(
            LegalSpeechDomain::CourtProceedings.to_string(),
            "Court Proceedings"
        );
        assert_eq!(LegalSpeechDomain::Depositions.to_string(), "Depositions");
        assert_eq!(
            LegalSpeechDomain::Consultations.to_string(),
            "Legal Consultations"
        );
        assert_eq!(
            LegalSpeechDomain::ContractNegotiations.to_string(),
            "Contract Negotiations"
        );
        assert_eq!(
            LegalSpeechDomain::ArbitrationMediation.to_string(),
            "Arbitration/Mediation"
        );
        assert_eq!(LegalSpeechDomain::General.to_string(), "General Legal");
    }
    #[test]
    fn test_legal_speech_transcriber_creation() {
        let locale = Locale::new("en").with_country("US");
        let transcriber =
            LegalSpeechTranscriber::new(locale.clone(), LegalSpeechDomain::CourtProceedings);
        assert_eq!(transcriber.locale, locale);
        assert_eq!(transcriber.audio_quality, AudioQuality::Medium);
        assert_eq!(transcriber.domain, LegalSpeechDomain::CourtProceedings);
        assert!(!transcriber.speaker_diarization);
        assert!(!transcriber.legal_vocabulary_boost);
    }
    #[test]
    fn test_legal_speech_transcriber_for_court() {
        let locale = Locale::new("en").with_country("US");
        let transcriber = LegalSpeechTranscriber::for_court_proceedings(locale);
        assert_eq!(transcriber.audio_quality, AudioQuality::Studio);
        assert!(transcriber.speaker_diarization);
        assert!(transcriber.legal_vocabulary_boost);
    }
    #[test]
    fn test_legal_speech_transcriber_for_depositions() {
        let locale = Locale::new("en").with_country("US");
        let transcriber = LegalSpeechTranscriber::for_depositions(locale);
        assert_eq!(transcriber.audio_quality, AudioQuality::High);
        assert!(transcriber.speaker_diarization);
        assert!(transcriber.legal_vocabulary_boost);
    }
    #[test]
    fn test_legal_speech_transcriber_vocabulary_hints() {
        let locale = Locale::new("en").with_country("US");
        let transcriber = LegalSpeechTranscriber::for_court_proceedings(locale);
        let hints = transcriber.get_vocabulary_hints();
        assert!(hints.contains(&"objection".to_string()));
        assert!(hints.contains(&"sustained".to_string()));
        assert!(hints.contains(&"Your Honor".to_string()));
    }
    #[test]
    fn test_legal_speech_transcriber_with_dictionary() {
        let locale = Locale::new("en").with_country("US");
        let mut dict = LegalDictionary::new(locale.clone());
        dict.add_translation(
            "plaintiff",
            "A person who brings a case against another in a court of law",
        );
        let transcriber =
            LegalSpeechTranscriber::for_court_proceedings(locale).with_dictionary(dict);
        assert!(transcriber.dictionary.is_some());
        let hints = transcriber.get_vocabulary_hints();
        assert!(hints.contains(&"plaintiff".to_string()));
    }
    #[test]
    fn test_interpretation_mode_display() {
        assert_eq!(InterpretationMode::Consecutive.to_string(), "Consecutive");
        assert_eq!(InterpretationMode::Simultaneous.to_string(), "Simultaneous");
        assert_eq!(
            InterpretationMode::Whispered.to_string(),
            "Whispered (Chuchotage)"
        );
    }
    #[test]
    fn test_interpreted_segment_creation() {
        let source_locale = Locale::new("en").with_country("US");
        let target_locale = Locale::new("es").with_country("ES");
        let source_segment =
            TranscriptionSegment::new("The defendant is guilty.", 0, 2000, source_locale);
        let interpreted = InterpretedSegment::new(
            source_segment,
            "El acusado es culpable.",
            target_locale.clone(),
        );
        assert_eq!(interpreted.target_text, "El acusado es culpable.");
        assert_eq!(interpreted.target_locale, target_locale);
        assert_eq!(interpreted.interpretation_confidence, 1.0);
        assert_eq!(interpreted.delay_ms, 0);
    }
    #[test]
    fn test_interpreted_segment_with_confidence() {
        let source_locale = Locale::new("en").with_country("US");
        let target_locale = Locale::new("fr").with_country("FR");
        let source_segment = TranscriptionSegment::new("Test", 0, 1000, source_locale);
        let interpreted = InterpretedSegment::new(source_segment, "Test", target_locale)
            .with_confidence(0.85)
            .with_delay_ms(250);
        assert_eq!(interpreted.interpretation_confidence, 0.85);
        assert_eq!(interpreted.delay_ms, 250);
    }
    #[test]
    fn test_interpreted_segment_format_bilingual() {
        let source_locale = Locale::new("en").with_country("US");
        let target_locale = Locale::new("ja").with_country("JP");
        let source_segment = TranscriptionSegment::new("Court", 0, 1000, source_locale);
        let interpreted = InterpretedSegment::new(source_segment, "裁判所", target_locale);
        let formatted = interpreted.format_bilingual();
        assert!(formatted.contains("[en-US]"));
        assert!(formatted.contains("Court"));
        assert!(formatted.contains("[ja-JP]"));
        assert!(formatted.contains("裁判所"));
    }
    #[test]
    fn test_simultaneous_interpreter_creation() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("es").with_country("ES");
        let interpreter = SimultaneousInterpreter::new(
            source.clone(),
            target.clone(),
            LegalSpeechDomain::CourtProceedings,
        );
        assert_eq!(interpreter.source_locale, source);
        assert_eq!(interpreter.target_locale, target);
        assert_eq!(interpreter.mode, InterpretationMode::Simultaneous);
        assert_eq!(interpreter.max_delay_ms, 3000);
    }
    #[test]
    fn test_simultaneous_interpreter_for_court() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("fr").with_country("FR");
        let interpreter = SimultaneousInterpreter::for_court_proceedings(source, target);
        assert_eq!(interpreter.domain, LegalSpeechDomain::CourtProceedings);
        assert_eq!(interpreter.transcriber.audio_quality, AudioQuality::Studio);
    }
    #[test]
    fn test_simultaneous_interpreter_with_mode() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("de").with_country("DE");
        let interpreter = SimultaneousInterpreter::new(source, target, LegalSpeechDomain::General)
            .with_mode(InterpretationMode::Consecutive);
        assert_eq!(interpreter.mode, InterpretationMode::Consecutive);
    }
    #[test]
    fn test_simultaneous_interpreter_segment() {
        let source = Locale::new("en").with_country("US");
        let target = Locale::new("ja").with_country("JP");
        let interpreter = SimultaneousInterpreter::new(
            source.clone(),
            target,
            LegalSpeechDomain::CourtProceedings,
        );
        let segment =
            TranscriptionSegment::new("Objection!", 0, 1000, source).with_confidence(0.98);
        let interpreted = interpreter.interpret_segment(segment);
        assert_eq!(interpreted.source_segment.text, "Objection!");
        assert_eq!(interpreted.delay_ms, 200);
        assert!(interpreted.interpretation_confidence > 0.0);
    }
    #[test]
    fn test_court_participant_role_display() {
        assert_eq!(CourtParticipantRole::Judge.to_string(), "Judge");
        assert_eq!(CourtParticipantRole::Prosecutor.to_string(), "Prosecutor");
        assert_eq!(
            CourtParticipantRole::DefenseAttorney.to_string(),
            "Defense Attorney"
        );
        assert_eq!(CourtParticipantRole::Witness.to_string(), "Witness");
        assert_eq!(CourtParticipantRole::Juror.to_string(), "Juror");
    }
    #[test]
    fn test_court_participant_creation() {
        let locale = Locale::new("en").with_country("US");
        let participant =
            CourtParticipant::new("John Doe", CourtParticipantRole::Defendant, locale.clone());
        assert_eq!(participant.name, "John Doe");
        assert_eq!(participant.role, CourtParticipantRole::Defendant);
        assert_eq!(participant.primary_language, locale);
        assert!(!participant.requires_interpretation);
    }
    #[test]
    fn test_court_participant_requires_interpretation() {
        let locale = Locale::new("es").with_country("ES");
        let participant =
            CourtParticipant::new("Maria Garcia", CourtParticipantRole::Witness, locale)
                .requires_interpretation();
        assert!(participant.requires_interpretation);
    }
    #[test]
    fn test_court_proceeding_translator_creation() {
        let court_language = Locale::new("en").with_country("US");
        let translator = CourtProceedingTranslator::new(court_language.clone());
        assert_eq!(translator.court_language, court_language);
        assert_eq!(translator.participants.len(), 0);
        assert_eq!(translator.interpreters.len(), 0);
        assert!(translator.record_audio);
        assert!(translator.real_time_transcripts);
    }
    #[test]
    fn test_court_proceeding_add_participant() {
        let court_language = Locale::new("en").with_country("US");
        let mut translator = CourtProceedingTranslator::new(court_language);
        let spanish_locale = Locale::new("es").with_country("ES");
        let participant = CourtParticipant::new(
            "Juan Lopez",
            CourtParticipantRole::Defendant,
            spanish_locale,
        )
        .requires_interpretation();
        translator.add_participant(participant);
        assert_eq!(translator.participants.len(), 1);
        assert_eq!(translator.interpreters.len(), 1);
        assert_eq!(translator.language_count(), 2);
    }
    #[test]
    fn test_court_proceeding_multiple_languages() {
        let court_language = Locale::new("en").with_country("US");
        let mut translator = CourtProceedingTranslator::new(court_language.clone());
        let spanish = Locale::new("es").with_country("ES");
        let french = Locale::new("fr").with_country("FR");
        translator.add_participant(
            CourtParticipant::new("Juan", CourtParticipantRole::Defendant, spanish)
                .requires_interpretation(),
        );
        translator.add_participant(
            CourtParticipant::new("Pierre", CourtParticipantRole::Witness, french)
                .requires_interpretation(),
        );
        assert_eq!(translator.language_count(), 3);
    }
    #[test]
    #[allow(dead_code)]
    fn test_court_proceeding_process_utterance() {
        let court_language = Locale::new("en").with_country("US");
        let mut translator = CourtProceedingTranslator::new(court_language.clone());
        let spanish = Locale::new("es").with_country("ES");
        translator.add_participant(
            CourtParticipant::new("Defendant", CourtParticipantRole::Defendant, spanish)
                .requires_interpretation(),
        );
        let segment = TranscriptionSegment::new("Please state your name.", 0, 2000, court_language);
        let translations = translator.process_utterance("Judge", segment);
        assert_eq!(translations.len(), 1);
    }
    #[test]
    fn test_multilingual_hearing_creation() {
        let primary = Locale::new("en").with_country("US");
        let hearing = MultilingualHearing::new("Case 123", primary.clone());
        assert_eq!(hearing.title, "Case 123");
        assert_eq!(hearing.primary_language, primary);
        assert_eq!(hearing.active_languages.len(), 1);
        assert!(hearing.closed_captions);
    }
    #[test]
    fn test_multilingual_hearing_add_language() {
        let primary = Locale::new("en").with_country("US");
        let mut hearing = MultilingualHearing::new("Hearing", primary);
        let spanish = Locale::new("es").with_country("ES");
        hearing.add_language(spanish.clone());
        assert_eq!(hearing.active_languages.len(), 2);
        assert_eq!(hearing.channel_count(), 1);
        assert!(hearing.active_languages.contains(&spanish));
    }
    #[test]
    fn test_multilingual_hearing_add_participant() {
        let primary = Locale::new("en").with_country("US");
        let mut hearing = MultilingualHearing::new("Trial", primary);
        let japanese = Locale::new("ja").with_country("JP");
        let participant =
            CourtParticipant::new("Tanaka", CourtParticipantRole::Witness, japanese.clone())
                .requires_interpretation();
        hearing.add_participant(participant);
        assert_eq!(hearing.active_languages.len(), 2);
        assert!(hearing.active_languages.contains(&japanese));
    }
    #[test]
    fn test_multilingual_hearing_process_utterance() {
        let primary = Locale::new("en").with_country("US");
        let mut hearing = MultilingualHearing::new("Case", primary.clone());
        let french = Locale::new("fr").with_country("FR");
        hearing.add_language(french);
        let segment = TranscriptionSegment::new("The court is now in session.", 0, 3000, primary);
        let interpretations = hearing.process_multilingual_utterance(segment);
        assert_eq!(interpretations.len(), 1);
    }
    #[test]
    fn test_subtitle_cue_creation() {
        let locale = Locale::new("en").with_country("US");
        let cue = SubtitleCue::new("Hello", 0, 2000, locale.clone());
        assert_eq!(cue.text, "Hello");
        assert_eq!(cue.start_ms, 0);
        assert_eq!(cue.end_ms, 2000);
        assert_eq!(cue.locale, locale);
        assert!(cue.speaker.is_none());
        assert!(cue.position.is_none());
    }
    #[test]
    fn test_subtitle_cue_with_speaker() {
        let locale = Locale::new("en").with_country("US");
        let cue = SubtitleCue::new("Text", 0, 1000, locale)
            .with_speaker("Judge")
            .with_position(SubtitlePosition::BottomCenter);
        assert_eq!(cue.speaker, Some("Judge".to_string()));
        assert_eq!(cue.position, Some(SubtitlePosition::BottomCenter));
    }
    #[test]
    fn test_subtitle_cue_to_webvtt() {
        let locale = Locale::new("en").with_country("US");
        let cue = SubtitleCue::new("Test subtitle", 1000, 3000, locale).with_speaker("Attorney");
        let vtt = cue.to_webvtt();
        assert!(vtt.contains("00:00:01.000 --> 00:00:03.000"));
        assert!(vtt.contains("<v Attorney>Test subtitle</v>"));
    }
    #[test]
    fn test_subtitle_cue_to_srt() {
        let locale = Locale::new("en").with_country("US");
        let cue = SubtitleCue::new("Test", 0, 2000, locale).with_speaker("Judge");
        let srt = cue.to_srt(1);
        assert!(srt.contains("1\n"));
        assert!(srt.contains("00:00:00,000 --> 00:00:02,000"));
        assert!(srt.contains("Judge: Test"));
    }
    #[test]
    fn test_subtitle_position_display() {
        assert_eq!(SubtitlePosition::BottomCenter.to_string(), "Bottom Center");
        assert_eq!(SubtitlePosition::TopCenter.to_string(), "Top Center");
        assert_eq!(SubtitlePosition::BottomLeft.to_string(), "Bottom Left");
        assert_eq!(SubtitlePosition::TopRight.to_string(), "Top Right");
    }
    #[test]
    fn test_accessibility_subtitle_generator_creation() {
        let locale = Locale::new("en").with_country("US");
        let generator = AccessibilitySubtitleGenerator::new(locale.clone());
        assert_eq!(generator.primary_locale, locale);
        assert!(generator.include_speakers);
        assert!(generator.include_sound_descriptions);
        assert_eq!(generator.max_chars_per_line, 42);
        assert!(!generator.multilingual);
    }
    #[test]
    fn test_accessibility_subtitle_generator_for_multilingual() {
        let primary = Locale::new("en").with_country("US");
        let spanish = Locale::new("es").with_country("ES");
        let secondary = vec![spanish];
        let generator = AccessibilitySubtitleGenerator::for_multilingual_court(primary, secondary);
        assert!(generator.multilingual);
        assert_eq!(generator.secondary_locales.len(), 1);
    }
    #[test]
    fn test_accessibility_subtitle_generator_cues() {
        let locale = Locale::new("en").with_country("US");
        let generator = AccessibilitySubtitleGenerator::new(locale.clone());
        let segments = vec![
            TranscriptionSegment::new("First segment", 0, 2000, locale.clone())
                .with_speaker("Judge"),
            TranscriptionSegment::new("Second segment", 2000, 4000, locale)
                .with_speaker("Attorney"),
        ];
        let cues = generator.generate_cues(&segments);
        assert!(cues.len() >= 2);
        assert_eq!(cues[0].speaker, Some("Judge".to_string()));
        assert_eq!(cues[1].speaker, Some("Attorney".to_string()));
    }
    #[test]
    fn test_accessibility_subtitle_generator_webvtt() {
        let locale = Locale::new("en").with_country("US");
        let generator = AccessibilitySubtitleGenerator::new(locale.clone());
        let segments = vec![TranscriptionSegment::new("Test", 0, 1000, locale)];
        let webvtt = generator.generate_webvtt(&segments);
        assert!(webvtt.starts_with("WEBVTT"));
        assert!(webvtt.contains("00:00:00.000 --> 00:00:01.000"));
    }
    #[test]
    fn test_accessibility_subtitle_generator_srt() {
        let locale = Locale::new("en").with_country("US");
        let generator = AccessibilitySubtitleGenerator::new(locale.clone());
        let segments = vec![TranscriptionSegment::new("Test", 0, 1000, locale)];
        let srt = generator.generate_srt(&segments);
        assert!(srt.contains("1\n"));
        assert!(srt.contains("00:00:00,000 --> 00:00:01,000"));
    }
    #[test]
    fn test_accessibility_subtitle_generator_long_text() {
        let locale = Locale::new("en").with_country("US");
        let generator = AccessibilitySubtitleGenerator::new(locale.clone()).with_max_chars(20);
        let long_text = "This is a very long sentence that should be split into multiple lines.";
        let segments = vec![TranscriptionSegment::new(long_text, 0, 5000, locale)];
        let cues = generator.generate_cues(&segments);
        assert!(cues.len() > 1);
    }
    #[test]
    fn test_accessibility_subtitle_generator_sound_description() {
        let locale = Locale::new("en").with_country("US");
        let generator = AccessibilitySubtitleGenerator::new(locale.clone());
        let mut cues = Vec::new();
        generator.add_sound_description(&mut cues, "gavel banging", 0, 500);
        assert_eq!(cues.len(), 1);
        assert_eq!(cues[0].text, "[gavel banging]");
    }
    #[test]
    fn test_accessibility_subtitle_generator_without_speakers() {
        let locale = Locale::new("en").with_country("US");
        let generator =
            AccessibilitySubtitleGenerator::new(locale.clone()).with_speaker_labels(false);
        let segments =
            vec![TranscriptionSegment::new("Test", 0, 1000, locale).with_speaker("Judge")];
        let cues = generator.generate_cues(&segments);
        assert_eq!(cues[0].speaker, None);
    }
}
#[cfg(test)]
mod semantic_search_tests {
    use crate::*;
    #[test]
    fn test_embedding_model_display() {
        assert_eq!(
            EmbeddingModel::MultilinguralBERT.to_string(),
            "Multilingual BERT"
        );
        assert_eq!(EmbeddingModel::XLMRoBERTa.to_string(), "XLM-RoBERTa");
        assert_eq!(EmbeddingModel::LaBSE.to_string(), "LaBSE");
        assert_eq!(
            EmbeddingModel::LegalMultilingual.to_string(),
            "Legal Multilingual"
        );
    }
    #[test]
    fn test_semantic_embedding_creation() {
        let locale = Locale::new("en").with_country("US");
        let vector = vec![0.1, 0.2, 0.3, 0.4];
        let embedding =
            SemanticEmbedding::new("test", locale.clone(), vector.clone(), "test-model");
        assert_eq!(embedding.text, "test");
        assert_eq!(embedding.locale, locale);
        assert_eq!(embedding.vector, vector);
        assert_eq!(embedding.dimensions(), 4);
    }
    #[test]
    fn test_semantic_embedding_cosine_similarity() {
        let locale = Locale::new("en").with_country("US");
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![1.0, 0.0, 0.0];
        let vec3 = vec![0.0, 1.0, 0.0];
        let emb1 = SemanticEmbedding::new("test1", locale.clone(), vec1, "model");
        let emb2 = SemanticEmbedding::new("test2", locale.clone(), vec2, "model");
        let emb3 = SemanticEmbedding::new("test3", locale, vec3, "model");
        assert!((emb1.cosine_similarity(&emb2) - 1.0).abs() < 0.001);
        assert!((emb1.cosine_similarity(&emb3)).abs() < 0.001);
    }
    #[test]
    fn test_semantic_embedding_with_domain() {
        let locale = Locale::new("en").with_country("US");
        let embedding = SemanticEmbedding::new("test", locale, vec![0.1], "model")
            .with_domain(LegalSpeechDomain::CourtProceedings);
        assert_eq!(embedding.domain, Some(LegalSpeechDomain::CourtProceedings));
    }
    #[test]
    fn test_multilingual_embedder_creation() {
        let embedder = MultilingualEmbedder::new(EmbeddingModel::LaBSE, 768);
        assert_eq!(embedder.model, EmbeddingModel::LaBSE);
        assert_eq!(embedder.dimension, 768);
        assert!(embedder.normalize);
    }
    #[test]
    fn test_multilingual_embedder_labse() {
        let embedder = MultilingualEmbedder::labse();
        assert_eq!(embedder.model, EmbeddingModel::LaBSE);
        assert_eq!(embedder.dimension, 768);
    }
    #[test]
    fn test_multilingual_embedder_xlm_roberta() {
        let embedder = MultilingualEmbedder::xlm_roberta();
        assert_eq!(embedder.model, EmbeddingModel::XLMRoBERTa);
        assert_eq!(embedder.dimension, 1024);
    }
    #[test]
    fn test_multilingual_embedder_legal_domain() {
        let embedder = MultilingualEmbedder::legal_domain();
        assert_eq!(embedder.model, EmbeddingModel::LegalMultilingual);
        assert_eq!(embedder.dimension, 768);
    }
    #[test]
    fn test_multilingual_embedder_embed() {
        let embedder = MultilingualEmbedder::labse();
        let locale = Locale::new("en").with_country("US");
        let embedding = embedder.embed("test text", locale.clone());
        assert_eq!(embedding.text, "test text");
        assert_eq!(embedding.locale, locale);
        assert_eq!(embedding.dimensions(), 768);
    }
    #[test]
    fn test_multilingual_embedder_embed_batch() {
        let embedder = MultilingualEmbedder::labse();
        let en_us = Locale::new("en").with_country("US");
        let es_es = Locale::new("es").with_country("ES");
        let texts = vec![
            ("text1".to_string(), en_us.clone()),
            ("text2".to_string(), es_es.clone()),
        ];
        let embeddings = embedder.embed_batch(&texts);
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].text, "text1");
        assert_eq!(embeddings[1].text, "text2");
    }
    #[test]
    fn test_legal_case_creation() {
        let locale = Locale::new("en").with_country("US");
        let case = LegalCase::new(
            "123",
            "Smith v. Jones",
            "US Federal",
            "Summary",
            locale.clone(),
            2023,
        );
        assert_eq!(case.case_id, "123");
        assert_eq!(case.title, "Smith v. Jones");
        assert_eq!(case.jurisdiction, "US Federal");
        assert_eq!(case.year, 2023);
        assert!(case.embedding.is_none());
    }
    #[test]
    fn test_legal_case_with_domain() {
        let locale = Locale::new("en").with_country("US");
        let case = LegalCase::new("123", "Title", "Jurisdiction", "Summary", locale, 2023)
            .with_domain(LegalSpeechDomain::ContractNegotiations);
        assert_eq!(case.domain, Some(LegalSpeechDomain::ContractNegotiations));
    }
    #[test]
    fn test_search_result_creation() {
        let locale = Locale::new("en").with_country("US");
        let case = LegalCase::new("1", "Title", "Jurisdiction", "Summary", locale, 2023);
        let result = SearchResult::new(case.clone(), 0.95, 1);
        assert_eq!(result.case.case_id, "1");
        assert_eq!(result.similarity, 0.95);
        assert_eq!(result.rank, 1);
    }
    #[test]
    fn test_cross_lingual_case_search_creation() {
        let embedder = MultilingualEmbedder::labse();
        let search = CrossLingualCaseSearch::new(embedder);
        assert_eq!(search.min_similarity, 0.5);
        assert_eq!(search.case_count(), 0);
    }
    #[test]
    fn test_cross_lingual_case_search_add_case() {
        let embedder = MultilingualEmbedder::labse();
        let mut search = CrossLingualCaseSearch::new(embedder);
        let locale = Locale::new("en").with_country("US");
        let case = LegalCase::new("1", "Title", "US", "Summary", locale, 2023);
        search.add_case(case);
        assert_eq!(search.case_count(), 1);
    }
    #[test]
    fn test_cross_lingual_case_search_search() {
        let embedder = MultilingualEmbedder::labse();
        let mut search = CrossLingualCaseSearch::new(embedder).with_min_similarity(0.0);
        let en = Locale::new("en").with_country("US");
        let case1 = LegalCase::new(
            "1",
            "Contract Case",
            "US",
            "contract dispute",
            en.clone(),
            2023,
        );
        let case2 = LegalCase::new("2", "Tort Case", "US", "negligence claim", en.clone(), 2023);
        search.add_case(case1);
        search.add_case(case2);
        let results = search.search("contract", en, 10);
        assert!(results.len() <= 2);
    }
    #[test]
    fn test_cross_lingual_case_search_by_jurisdiction() {
        let embedder = MultilingualEmbedder::labse();
        let mut search = CrossLingualCaseSearch::new(embedder).with_min_similarity(0.0);
        let en = Locale::new("en").with_country("US");
        let case1 = LegalCase::new("1", "Title", "US", "Summary", en.clone(), 2023);
        let case2 = LegalCase::new("2", "Title", "UK", "Summary", en.clone(), 2023);
        search.add_case(case1);
        search.add_case(case2);
        let results = search.search_by_jurisdiction("test", en, "US", 10);
        assert!(results.iter().all(|r| r.case.jurisdiction == "US"));
    }
    #[test]
    fn test_legal_concept_creation() {
        let concept = LegalConcept::new("contract", "Contract", "A legally binding agreement");
        assert_eq!(concept.concept_id, "contract");
        assert_eq!(concept.canonical_name, "Contract");
        assert_eq!(concept.definition, "A legally binding agreement");
    }
    #[test]
    fn test_legal_concept_add_localized_name() {
        let es = Locale::new("es").with_country("ES");
        let concept = LegalConcept::new("contract", "Contract", "Definition")
            .add_localized_name(es.clone(), "Contrato");
        assert_eq!(concept.get_name(&es), "Contrato");
    }
    #[test]
    fn test_legal_concept_get_name_fallback() {
        let concept = LegalConcept::new("contract", "Contract", "Definition");
        let de = Locale::new("de").with_country("DE");
        assert_eq!(concept.get_name(&de), "Contract");
    }
    #[test]
    fn test_concept_mapper_creation() {
        let embedder = MultilingualEmbedder::labse();
        let mapper = ConceptMapper::new(embedder);
        assert_eq!(mapper.concept_count(), 0);
    }
    #[test]
    fn test_concept_mapper_with_defaults() {
        let embedder = MultilingualEmbedder::labse();
        let mapper = ConceptMapper::with_defaults(embedder);
        assert!(mapper.concept_count() > 0);
        assert!(mapper.concepts.contains_key("contract"));
        assert!(mapper.concepts.contains_key("tort"));
        assert!(mapper.concepts.contains_key("jurisdiction"));
    }
    #[test]
    fn test_concept_mapper_add_concept() {
        let embedder = MultilingualEmbedder::labse();
        let mut mapper = ConceptMapper::new(embedder);
        let concept = LegalConcept::new("test", "Test", "Test definition");
        mapper.add_concept(concept);
        assert_eq!(mapper.concept_count(), 1);
    }
    #[test]
    fn test_concept_mapper_find_concept() {
        let embedder = MultilingualEmbedder::labse();
        let mapper = ConceptMapper::with_defaults(embedder);
        let en = Locale::new("en").with_country("US");
        let result = mapper.find_concept("contract", en);
        assert!(result.is_some());
    }
    #[test]
    fn test_concept_mapper_map_term_across_languages() {
        let embedder = MultilingualEmbedder::labse();
        let mapper = ConceptMapper::with_defaults(embedder);
        let en = Locale::new("en").with_country("US");
        let mappings = mapper.map_term_across_languages("contract", en);
        assert!(!mappings.is_empty());
    }
    #[test]
    fn test_knowledge_graph_node_creation() {
        let locale = Locale::new("en").with_country("US");
        let node = KnowledgeGraphNode::new("node1", "concept", "Contract", locale.clone());
        assert_eq!(node.node_id, "node1");
        assert_eq!(node.node_type, "concept");
        assert_eq!(node.label, "Contract");
        assert_eq!(node.locale, locale);
    }
    #[test]
    fn test_knowledge_graph_node_with_property() {
        let locale = Locale::new("en").with_country("US");
        let node = KnowledgeGraphNode::new("node1", "concept", "Label", locale)
            .with_property("key", "value");
        assert_eq!(node.properties.get("key"), Some(&"value".to_string()));
    }
    #[test]
    fn test_knowledge_graph_edge_creation() {
        let edge = KnowledgeGraphEdge::new("node1", "node2", "relates_to");
        assert_eq!(edge.from_node, "node1");
        assert_eq!(edge.to_node, "node2");
        assert_eq!(edge.relationship, "relates_to");
    }
    #[test]
    fn test_knowledge_graph_edge_with_property() {
        let edge =
            KnowledgeGraphEdge::new("node1", "node2", "relates_to").with_property("weight", "0.5");
        assert_eq!(edge.properties.get("weight"), Some(&"0.5".to_string()));
    }
    #[test]
    fn test_multilingual_knowledge_graph_creation() {
        let embedder = MultilingualEmbedder::labse();
        let graph = MultilingualKnowledgeGraph::new(embedder);
        let (nodes, edges) = graph.stats();
        assert_eq!(nodes, 0);
        assert_eq!(edges, 0);
    }
    #[test]
    fn test_multilingual_knowledge_graph_add_node() {
        let embedder = MultilingualEmbedder::labse();
        let mut graph = MultilingualKnowledgeGraph::new(embedder);
        let locale = Locale::new("en").with_country("US");
        let node = KnowledgeGraphNode::new("node1", "concept", "Contract", locale);
        graph.add_node(node);
        let (nodes, _) = graph.stats();
        assert_eq!(nodes, 1);
    }
    #[test]
    fn test_multilingual_knowledge_graph_add_edge() {
        let embedder = MultilingualEmbedder::labse();
        let mut graph = MultilingualKnowledgeGraph::new(embedder);
        let edge = KnowledgeGraphEdge::new("node1", "node2", "relates_to");
        graph.add_edge(edge);
        let (_, edges) = graph.stats();
        assert_eq!(edges, 1);
    }
    #[test]
    fn test_multilingual_knowledge_graph_get_node() {
        let embedder = MultilingualEmbedder::labse();
        let mut graph = MultilingualKnowledgeGraph::new(embedder);
        let locale = Locale::new("en").with_country("US");
        let node = KnowledgeGraphNode::new("node1", "concept", "Contract", locale);
        graph.add_node(node);
        let retrieved = graph.get_node("node1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().node_id, "node1");
    }
    #[test]
    fn test_multilingual_knowledge_graph_find_nodes_by_type() {
        let embedder = MultilingualEmbedder::labse();
        let mut graph = MultilingualKnowledgeGraph::new(embedder);
        let locale = Locale::new("en").with_country("US");
        let node1 = KnowledgeGraphNode::new("node1", "concept", "Contract", locale.clone());
        let node2 = KnowledgeGraphNode::new("node2", "case", "Smith v Jones", locale);
        graph.add_node(node1);
        graph.add_node(node2);
        let concepts = graph.find_nodes_by_type("concept");
        assert_eq!(concepts.len(), 1);
        assert_eq!(concepts[0].node_id, "node1");
    }
    #[test]
    fn test_multilingual_knowledge_graph_find_edges() {
        let embedder = MultilingualEmbedder::labse();
        let mut graph = MultilingualKnowledgeGraph::new(embedder);
        let edge1 = KnowledgeGraphEdge::new("node1", "node2", "relates_to");
        let edge2 = KnowledgeGraphEdge::new("node2", "node3", "relates_to");
        graph.add_edge(edge1);
        graph.add_edge(edge2);
        let outgoing = graph.find_outgoing_edges("node1");
        assert_eq!(outgoing.len(), 1);
        let incoming = graph.find_incoming_edges("node2");
        assert_eq!(incoming.len(), 1);
    }
    #[test]
    fn test_multilingual_knowledge_graph_find_similar_nodes() {
        let embedder = MultilingualEmbedder::labse();
        let mut graph = MultilingualKnowledgeGraph::new(embedder);
        let locale = Locale::new("en").with_country("US");
        let node =
            KnowledgeGraphNode::new("node1", "concept", "contract agreement", locale.clone());
        graph.add_node(node);
        let similar = graph.find_similar_nodes("contract", locale, 5);
        assert!(!similar.is_empty());
    }
    #[test]
    fn test_legal_reasoning_engine_creation() {
        let embedder = MultilingualEmbedder::labse();
        let engine = LegalReasoningEngine::new(embedder);
        assert!(engine.concept_mapper.concept_count() > 0);
        assert_eq!(engine.case_search.case_count(), 0);
    }
    #[test]
    fn test_legal_reasoning_engine_analyze_query() {
        let embedder = MultilingualEmbedder::labse();
        let engine = LegalReasoningEngine::new(embedder);
        let locale = Locale::new("en").with_country("US");
        let result = engine.analyze_query("contract", locale);
        assert_eq!(result.query, "contract");
        assert!(result.has_results());
    }
    #[test]
    fn test_legal_reasoning_engine_cross_jurisdictional() {
        let embedder = MultilingualEmbedder::labse();
        let engine = LegalReasoningEngine::new(embedder);
        let en = Locale::new("en").with_country("US");
        let equivalents = engine.find_cross_jurisdictional_equivalents("contract", en);
        assert!(!equivalents.is_empty());
    }
    #[test]
    fn test_analysis_result_has_results() {
        let locale = Locale::new("en").with_country("US");
        let result = AnalysisResult {
            query: "test".to_string(),
            locale,
            matched_concept: None,
            similar_cases: vec![],
            related_nodes: vec![],
        };
        assert!(!result.has_results());
        assert_eq!(result.case_count(), 0);
        assert_eq!(result.node_count(), 0);
    }
    #[test]
    fn test_analysis_result_with_concept() {
        let locale = Locale::new("en").with_country("US");
        let concept = LegalConcept::new("contract", "Contract", "Definition");
        let result = AnalysisResult {
            query: "test".to_string(),
            locale,
            matched_concept: Some(concept),
            similar_cases: vec![],
            related_nodes: vec![],
        };
        assert!(result.has_results());
    }
}
#[cfg(test)]
mod regulatory_harmonization_tests {
    use crate::*;
    #[test]
    fn test_eu_regulation_type_display() {
        assert_eq!(EURegulationType::GDPR.to_string(), "GDPR");
        assert_eq!(EURegulationType::MiFIDII.to_string(), "MiFID II");
        assert_eq!(EURegulationType::REACH.to_string(), "REACH");
        assert_eq!(EURegulationType::AIAct.to_string(), "AI Act");
    }
    #[test]
    fn test_eu_regulation_term_creation() {
        let term = EURegulationTerm::new(
            EURegulationType::GDPR,
            "personal data",
            "Information relating to an identified person",
        )
        .add_translation("de", "personenbezogene Daten")
        .add_translation("fr", "données à caractère personnel")
        .with_article("Article 4(1)");
        assert_eq!(term.regulation, EURegulationType::GDPR);
        assert_eq!(term.canonical_term, "personal data");
        assert_eq!(
            term.get_translation("de"),
            Some(&"personenbezogene Daten".to_string())
        );
        assert_eq!(term.article_ref, Some("Article 4(1)".to_string()));
    }
    #[test]
    fn test_eu_regulation_aligner_defaults() {
        let aligner = EURegulationAligner::with_gdpr_defaults();
        assert!(aligner.term_count() > 0);
        assert!(
            aligner
                .supported_regulations()
                .contains(&EURegulationType::GDPR)
        );
    }
    #[test]
    fn test_eu_regulation_aligner_translate() {
        let aligner = EURegulationAligner::with_gdpr_defaults();
        let translation = aligner.translate_term("personal data", "de");
        assert_eq!(translation, Some("personenbezogene Daten".to_string()));
        let translation_fr = aligner.translate_term("data controller", "fr");
        assert_eq!(
            translation_fr,
            Some("responsable du traitement".to_string())
        );
    }
    #[test]
    fn test_eu_regulation_aligner_get_terms() {
        let aligner = EURegulationAligner::with_gdpr_defaults();
        let gdpr_terms = aligner.get_terms(EURegulationType::GDPR);
        assert!(!gdpr_terms.is_empty());
        assert!(gdpr_terms.len() >= 4);
    }
    #[test]
    fn test_eu_regulation_aligner_add_custom() {
        let mut aligner = EURegulationAligner::new();
        aligner.add_term(
            EURegulationTerm::new(
                EURegulationType::AIAct,
                "high-risk AI system",
                "AI system that poses significant risks",
            )
            .add_translation("de", "KI-System mit hohem Risiko"),
        );
        assert_eq!(aligner.term_count(), 1);
        let translation = aligner.translate_term("high-risk AI system", "de");
        assert_eq!(translation, Some("KI-System mit hohem Risiko".to_string()));
    }
    #[test]
    fn test_treaty_type_display() {
        assert_eq!(TreatyType::Bilateral.to_string(), "Bilateral Treaty");
        assert_eq!(TreatyType::UNTreaty.to_string(), "UN Treaty");
        assert_eq!(
            TreatyType::Environmental.to_string(),
            "Environmental Treaty"
        );
    }
    #[test]
    fn test_treaty_term_creation() {
        let term = TreatyTerm::new("UNCLOS", TreatyType::UNTreaty, "territorial sea")
            .add_translation("fr", "mer territoriale")
            .add_translation("es", "mar territorial")
            .with_article("Article 2")
            .add_country("US")
            .add_country("FR");
        assert_eq!(term.treaty_name, "UNCLOS");
        assert_eq!(term.treaty_type, TreatyType::UNTreaty);
        assert_eq!(term.canonical_term, "territorial sea");
        assert_eq!(
            term.translations.get("fr"),
            Some(&"mer territoriale".to_string())
        );
        assert_eq!(term.ratifying_countries.len(), 2);
    }
    #[test]
    fn test_treaty_standardizer_defaults() {
        let standardizer = TreatyStandardizer::with_un_defaults();
        assert!(standardizer.treaty_count() > 0);
        assert!(standardizer.term_count() > 0);
    }
    #[test]
    fn test_treaty_standardizer_translate() {
        let standardizer = TreatyStandardizer::with_un_defaults();
        let translations = standardizer.translate_term("territorial sea", "fr");
        assert!(!translations.is_empty());
        assert!(translations.contains(&"mer territoriale".to_string()));
    }
    #[test]
    fn test_treaty_standardizer_get_treaty_terms() {
        let standardizer = TreatyStandardizer::with_un_defaults();
        let unclos_terms = standardizer.get_treaty_terms("UNCLOS");
        assert!(!unclos_terms.is_empty());
    }
    #[test]
    fn test_treaty_standardizer_add_custom() {
        let mut standardizer = TreatyStandardizer::new();
        standardizer.add_term(
            TreatyTerm::new(
                "Vienna Convention",
                TreatyType::Multilateral,
                "consular relations",
            )
            .add_translation("es", "relaciones consulares"),
        );
        assert_eq!(standardizer.treaty_count(), 1);
        assert_eq!(standardizer.term_count(), 1);
    }
    #[test]
    fn test_standard_type_display() {
        assert_eq!(StandardType::ISO.to_string(), "ISO");
        assert_eq!(StandardType::IETF.to_string(), "IETF");
        assert_eq!(StandardType::UNCITRAL.to_string(), "UNCITRAL");
        assert_eq!(
            StandardType::HagueConference.to_string(),
            "Hague Conference"
        );
    }
    #[test]
    fn test_adoption_status_display() {
        assert_eq!(AdoptionStatus::FullyAdopted.to_string(), "Fully Adopted");
        assert_eq!(
            AdoptionStatus::PartiallyAdopted.to_string(),
            "Partially Adopted"
        );
        assert_eq!(AdoptionStatus::InProgress.to_string(), "In Progress");
        assert_eq!(AdoptionStatus::NotAdopted.to_string(), "Not Adopted");
    }
    #[test]
    fn test_standard_adoption_creation() {
        let adoption = StandardAdoption::new(
            "ISO 27001",
            StandardType::ISO,
            "US",
            AdoptionStatus::FullyAdopted,
        )
        .with_date("2013-10-01")
        .with_law("NIST SP 800-53")
        .add_deviation("Some deviations allowed");
        assert_eq!(adoption.standard_id, "ISO 27001");
        assert_eq!(adoption.standard_type, StandardType::ISO);
        assert_eq!(adoption.jurisdiction, "US");
        assert_eq!(adoption.status, AdoptionStatus::FullyAdopted);
        assert_eq!(adoption.adoption_date, Some("2013-10-01".to_string()));
        assert_eq!(
            adoption.implementing_law,
            Some("NIST SP 800-53".to_string())
        );
        assert_eq!(adoption.deviations.len(), 1);
    }
    #[test]
    fn test_standard_adoption_tracker_defaults() {
        let tracker = StandardAdoptionTracker::with_defaults();
        assert!(tracker.standard_count() > 0);
        assert!(tracker.adoption_count() > 0);
    }
    #[test]
    fn test_standard_adoption_tracker_get_standard() {
        let tracker = StandardAdoptionTracker::with_defaults();
        let adoptions = tracker.get_standard_adoptions("ISO 27001");
        assert!(!adoptions.is_empty());
    }
    #[test]
    fn test_standard_adoption_tracker_get_jurisdiction() {
        let tracker = StandardAdoptionTracker::with_defaults();
        let us_adoptions = tracker.get_jurisdiction_adoptions("US");
        assert!(!us_adoptions.is_empty());
    }
    #[test]
    fn test_standard_adoption_tracker_is_fully_adopted() {
        let tracker = StandardAdoptionTracker::with_defaults();
        assert!(tracker.is_fully_adopted("ISO 27001", "US"));
        assert!(tracker.is_fully_adopted("ISO 27001", "GB"));
        assert!(!tracker.is_fully_adopted("ISO 27001", "JP"));
    }
    #[test]
    fn test_standard_adoption_tracker_add_custom() {
        let mut tracker = StandardAdoptionTracker::new();
        tracker.add_adoption(
            StandardAdoption::new(
                "ISO 9001",
                StandardType::ISO,
                "DE",
                AdoptionStatus::FullyAdopted,
            )
            .with_date("2015-09-15"),
        );
        assert_eq!(tracker.standard_count(), 1);
        assert_eq!(tracker.adoption_count(), 1);
    }
    #[test]
    fn test_regulatory_equivalence_level_display() {
        assert_eq!(
            RegulatoryEquivalenceLevel::Full.to_string(),
            "Full Equivalence"
        );
        assert_eq!(
            RegulatoryEquivalenceLevel::Conditional.to_string(),
            "Conditional Equivalence"
        );
        assert_eq!(
            RegulatoryEquivalenceLevel::Partial.to_string(),
            "Partial Equivalence"
        );
        assert_eq!(
            RegulatoryEquivalenceLevel::NoEquivalence.to_string(),
            "No Equivalence"
        );
    }
    #[test]
    fn test_regulatory_domain_display() {
        assert_eq!(
            RegulatoryDomain::DataProtection.to_string(),
            "Data Protection"
        );
        assert_eq!(
            RegulatoryDomain::FinancialServices.to_string(),
            "Financial Services"
        );
        assert_eq!(RegulatoryDomain::Environmental.to_string(), "Environmental");
        assert_eq!(
            RegulatoryDomain::ProfessionalQualifications.to_string(),
            "Professional Qualifications"
        );
    }
    #[test]
    fn test_regulatory_equivalence_creation() {
        let equivalence = RegulatoryEquivalence::new(
            "EU",
            "US",
            RegulatoryDomain::DataProtection,
            RegulatoryEquivalenceLevel::Conditional,
        )
        .with_basis("EU-US Privacy Shield")
        .add_condition("Must use SCCs")
        .with_review_date("2023-07-10");
        assert_eq!(equivalence.source_jurisdiction, "EU");
        assert_eq!(equivalence.target_jurisdiction, "US");
        assert_eq!(equivalence.domain, RegulatoryDomain::DataProtection);
        assert_eq!(equivalence.level, RegulatoryEquivalenceLevel::Conditional);
        assert_eq!(equivalence.basis, Some("EU-US Privacy Shield".to_string()));
        assert_eq!(equivalence.conditions.len(), 1);
        assert!(!equivalence.is_mutual());
    }
    #[test]
    fn test_regulatory_equivalence_is_mutual() {
        let full_equiv = RegulatoryEquivalence::new(
            "AU",
            "NZ",
            RegulatoryDomain::ProfessionalQualifications,
            RegulatoryEquivalenceLevel::Full,
        );
        assert!(full_equiv.is_mutual());
    }
    #[test]
    fn test_regulatory_equivalence_mapper_defaults() {
        let mapper = RegulatoryEquivalenceMapper::with_defaults();
        assert!(mapper.equivalence_count() > 0);
        assert!(mapper.jurisdiction_count() > 0);
    }
    #[test]
    fn test_regulatory_equivalence_mapper_get_equivalences() {
        let mapper = RegulatoryEquivalenceMapper::with_defaults();
        let eu_equivalences = mapper.get_equivalences("EU");
        assert!(!eu_equivalences.is_empty());
    }
    #[test]
    fn test_regulatory_equivalence_mapper_get_by_domain() {
        let mapper = RegulatoryEquivalenceMapper::with_defaults();
        let data_protection = mapper.get_by_domain(RegulatoryDomain::DataProtection);
        assert!(!data_protection.is_empty());
    }
    #[test]
    fn test_regulatory_equivalence_mapper_has_equivalence() {
        let mapper = RegulatoryEquivalenceMapper::with_defaults();
        let level = mapper.has_equivalence("EU", "GB", RegulatoryDomain::DataProtection);
        assert_eq!(level, Some(RegulatoryEquivalenceLevel::Full));
        let no_equiv = mapper.has_equivalence("XX", "YY", RegulatoryDomain::DataProtection);
        assert_eq!(no_equiv, None);
    }
    #[test]
    fn test_regulatory_equivalence_mapper_add_custom() {
        let mut mapper = RegulatoryEquivalenceMapper::new();
        mapper.add_equivalence(RegulatoryEquivalence::new(
            "JP",
            "KR",
            RegulatoryDomain::FinancialServices,
            RegulatoryEquivalenceLevel::Partial,
        ));
        assert_eq!(mapper.equivalence_count(), 1);
        assert_eq!(mapper.jurisdiction_count(), 1);
    }
    #[test]
    fn test_normalization_level_display() {
        assert_eq!(NormalizationLevel::Strict.to_string(), "Strict");
        assert_eq!(NormalizationLevel::Standard.to_string(), "Standard");
        assert_eq!(NormalizationLevel::Flexible.to_string(), "Flexible");
    }
    #[test]
    fn test_compliance_term_creation() {
        let term = ComplianceTerm::new(
            "data controller",
            RegulatoryDomain::DataProtection,
            "Entity determining purposes",
            NormalizationLevel::Strict,
        )
        .add_variant("controller")
        .add_variant("data owner");
        assert_eq!(term.canonical, "data controller");
        assert_eq!(term.domain, RegulatoryDomain::DataProtection);
        assert_eq!(term.normalization_level, NormalizationLevel::Strict);
        assert_eq!(term.variants.len(), 2);
    }
    #[test]
    fn test_compliance_term_matches() {
        let term = ComplianceTerm::new(
            "personal data",
            RegulatoryDomain::DataProtection,
            "Information about a person",
            NormalizationLevel::Strict,
        )
        .add_variant("PII")
        .add_variant("personally identifiable information");
        assert!(term.matches("personal data"));
        assert!(term.matches("Personal Data"));
        assert!(term.matches("PII"));
        assert!(term.matches("pii"));
        assert!(!term.matches("random text"));
    }
    #[test]
    fn test_compliance_normalizer_defaults() {
        let normalizer = ComplianceNormalizer::with_defaults();
        assert!(normalizer.term_count() > 0);
    }
    #[test]
    fn test_compliance_normalizer_normalize() {
        let normalizer = ComplianceNormalizer::with_defaults();
        let normalized = normalizer.normalize("PII");
        assert_eq!(normalized, Some("personal data".to_string()));
        let normalized2 = normalizer.normalize("controller");
        assert_eq!(normalized2, Some("data controller".to_string()));
    }
    #[test]
    fn test_compliance_normalizer_normalize_in_domain() {
        let normalizer = ComplianceNormalizer::with_defaults();
        let normalized = normalizer.normalize_in_domain("PII", RegulatoryDomain::DataProtection);
        assert_eq!(normalized, Some("personal data".to_string()));
        let not_found = normalizer.normalize_in_domain("PII", RegulatoryDomain::Environmental);
        assert_eq!(not_found, None);
    }
    #[test]
    fn test_compliance_normalizer_is_normalized() {
        let normalizer = ComplianceNormalizer::with_defaults();
        assert!(normalizer.is_normalized("data controller"));
        assert!(normalizer.is_normalized("personal data"));
        assert!(!normalizer.is_normalized("PII"));
    }
    #[test]
    fn test_compliance_normalizer_get_variants() {
        let normalizer = ComplianceNormalizer::with_defaults();
        let variants = normalizer.get_variants("personal data");
        assert!(!variants.is_empty());
        assert!(variants.contains(&"PII".to_string()));
    }
    #[test]
    fn test_compliance_normalizer_get_by_domain() {
        let normalizer = ComplianceNormalizer::with_defaults();
        let data_protection_terms = normalizer.get_by_domain(RegulatoryDomain::DataProtection);
        assert!(!data_protection_terms.is_empty());
        let financial_terms = normalizer.get_by_domain(RegulatoryDomain::FinancialServices);
        assert!(!financial_terms.is_empty());
    }
    #[test]
    fn test_compliance_normalizer_add_custom() {
        let mut normalizer = ComplianceNormalizer::new(NormalizationLevel::Standard);
        normalizer.add_term(
            ComplianceTerm::new(
                "cybersecurity incident",
                RegulatoryDomain::DataProtection,
                "Security breach affecting data",
                NormalizationLevel::Standard,
            )
            .add_variant("data breach")
            .add_variant("security incident"),
        );
        assert_eq!(normalizer.term_count(), 1);
        let normalized = normalizer.normalize("data breach");
        assert_eq!(normalized, Some("cybersecurity incident".to_string()));
    }
    #[test]
    fn test_regulatory_harmonization_integration() {
        let aligner = EURegulationAligner::with_gdpr_defaults();
        let normalizer = ComplianceNormalizer::with_defaults();
        let de_translation = aligner.translate_term("personal data", "de");
        assert!(de_translation.is_some());
        let normalized = normalizer.normalize("PII");
        assert_eq!(normalized, Some("personal data".to_string()));
        assert_eq!(normalized.unwrap(), "personal data");
    }
    #[test]
    fn test_treaty_and_standard_integration() {
        let standardizer = TreatyStandardizer::with_un_defaults();
        let tracker = StandardAdoptionTracker::with_defaults();
        let unclos_terms = standardizer.get_treaty_terms("UNCLOS");
        assert!(!unclos_terms.is_empty());
        let iso_adoptions = tracker.get_standard_adoptions("ISO 27001");
        assert!(!iso_adoptions.is_empty());
    }
    #[test]
    fn test_equivalence_mapper_integration() {
        let mapper = RegulatoryEquivalenceMapper::with_defaults();
        let normalizer = ComplianceNormalizer::with_defaults();
        let eu_us_equiv = mapper.has_equivalence("EU", "US", RegulatoryDomain::DataProtection);
        assert!(eu_us_equiv.is_some());
        let normalized = normalizer.normalize_in_domain("PII", RegulatoryDomain::DataProtection);
        assert!(normalized.is_some());
    }
}
