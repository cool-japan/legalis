//! Error types for EU AI Act compliance validation

use crate::i18n::MultilingualText;
use thiserror::Error;

/// Errors for AI Act compliance validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AiRegulationError {
    /// Prohibited AI practice detected
    #[error("Prohibited AI practice: {practice}")]
    ProhibitedPractice { practice: String },

    /// High-risk AI system missing required conformity assessment
    #[error("High-risk AI system requires conformity assessment before market placement")]
    MissingConformityAssessment,

    /// Missing risk management system
    #[error("High-risk AI system must have risk management system (Article 9)")]
    MissingRiskManagement,

    /// Inadequate data governance
    #[error("Data governance requirements not met (Article 10): {reason}")]
    InadequateDataGovernance { reason: String },

    /// Missing technical documentation
    #[error("Technical documentation required (Article 11): {missing_elements}")]
    MissingTechnicalDocumentation { missing_elements: String },

    /// Record-keeping requirements not met
    #[error("Record-keeping requirements not met (Article 12): {reason}")]
    RecordKeepingViolation { reason: String },

    /// Insufficient transparency
    #[error("Transparency requirements not met (Article 13): {reason}")]
    InsufficientTransparency { reason: String },

    /// Missing human oversight
    #[error("Human oversight required for high-risk AI (Article 14)")]
    MissingHumanOversight,

    /// Accuracy or robustness requirements not met
    #[error("Accuracy/robustness requirements not met (Article 15): {reason}")]
    AccuracyRobustnessViolation { reason: String },

    /// Limited risk system transparency violation
    #[error("Limited risk system must inform users of AI interaction (Article 52)")]
    LimitedRiskTransparencyViolation,

    /// Deep fake not properly marked
    #[error("AI-generated content must be marked as artificially generated (Article 52)")]
    DeepFakeNotMarked,

    /// Bias not adequately addressed
    #[error("Identified biases not adequately mitigated: {biases}")]
    BiasNotMitigated { biases: String },

    /// Training data quality issues
    #[error("Training data quality issues: {issues}")]
    TrainingDataQualityIssues { issues: String },

    /// Missing field
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// Invalid value
    #[error("Invalid value for field '{field}': {reason}")]
    InvalidValue { field: String, reason: String },

    /// Conformity assessment failed
    #[error("Conformity assessment failed: {reasons}")]
    ConformityAssessmentFailed { reasons: String },

    /// General-purpose AI transparency violation
    #[error("General-purpose AI must provide transparency documentation (Article 51)")]
    GpaiTransparencyViolation,

    /// Systemic risk AI model violation
    #[error("AI model with systemic risk must meet additional requirements (Article 51)")]
    SystemicRiskViolation,

    /// Multiple violations
    #[error("Multiple AI Act violations: {count}")]
    MultipleViolations { count: usize },
}

impl AiRegulationError {
    /// Create error for prohibited practice
    pub fn prohibited_practice(practice: impl Into<String>) -> Self {
        Self::ProhibitedPractice {
            practice: practice.into(),
        }
    }

    /// Create error for inadequate data governance
    pub fn inadequate_data_governance(reason: impl Into<String>) -> Self {
        Self::InadequateDataGovernance {
            reason: reason.into(),
        }
    }

    /// Create error for missing technical documentation
    pub fn missing_technical_documentation(missing_elements: impl Into<String>) -> Self {
        Self::MissingTechnicalDocumentation {
            missing_elements: missing_elements.into(),
        }
    }

    /// Create error for record-keeping violation
    pub fn record_keeping_violation(reason: impl Into<String>) -> Self {
        Self::RecordKeepingViolation {
            reason: reason.into(),
        }
    }

    /// Create error for insufficient transparency
    pub fn insufficient_transparency(reason: impl Into<String>) -> Self {
        Self::InsufficientTransparency {
            reason: reason.into(),
        }
    }

    /// Create error for accuracy/robustness violation
    pub fn accuracy_robustness_violation(reason: impl Into<String>) -> Self {
        Self::AccuracyRobustnessViolation {
            reason: reason.into(),
        }
    }

    /// Create error for bias not mitigated
    pub fn bias_not_mitigated(biases: impl Into<String>) -> Self {
        Self::BiasNotMitigated {
            biases: biases.into(),
        }
    }

    /// Create error for training data quality issues
    pub fn training_data_quality_issues(issues: impl Into<String>) -> Self {
        Self::TrainingDataQualityIssues {
            issues: issues.into(),
        }
    }

    /// Create error for missing field
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Create error for invalid value
    pub fn invalid_value(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Create error for conformity assessment failed
    pub fn conformity_assessment_failed(reasons: impl Into<String>) -> Self {
        Self::ConformityAssessmentFailed {
            reasons: reasons.into(),
        }
    }

    /// Get localized error message
    ///
    /// Returns the error message in the requested language with fallback to English.
    pub fn message(&self, lang: &str) -> String {
        let ml_text = self.to_multilingual();
        ml_text.in_language(lang).to_string()
    }

    /// Convert error to multilingual text
    fn to_multilingual(&self) -> MultilingualText {
        match self {
            Self::ProhibitedPractice { practice } => MultilingualText::new(format!(
                "Prohibited AI practice: {}",
                practice
            ))
            .with_de(format!("Verbotene KI-Praxis: {}", practice))
            .with_fr(format!("Pratique d'IA interdite: {}", practice))
            .with_es(format!("Práctica de IA prohibida: {}", practice))
            .with_it(format!("Pratica di IA vietata: {}", practice)),

            Self::MissingConformityAssessment => MultilingualText::new(
                "High-risk AI system requires conformity assessment before market placement"
            )
            .with_de("Hochrisiko-KI-System erfordert Konformitätsbewertung vor Markteinführung")
            .with_fr("Le système d'IA à haut risque nécessite une évaluation de conformité avant la mise sur le marché")
            .with_es("El sistema de IA de alto riesgo requiere evaluación de conformidad antes de la comercialización")
            .with_it("Il sistema di IA ad alto rischio richiede una valutazione di conformità prima dell'immissione sul mercato"),

            Self::MissingRiskManagement => MultilingualText::new(
                "High-risk AI system must have risk management system (Article 9)"
            )
            .with_de("Hochrisiko-KI-System muss Risikomanagementsystem haben (Artikel 9)")
            .with_fr("Le système d'IA à haut risque doit avoir un système de gestion des risques (Article 9)")
            .with_es("El sistema de IA de alto riesgo debe tener sistema de gestión de riesgos (Artículo 9)")
            .with_it("Il sistema di IA ad alto rischio deve avere un sistema di gestione dei rischi (Articolo 9)"),

            Self::InadequateDataGovernance { reason } => MultilingualText::new(format!(
                "Data governance requirements not met (Article 10): {}",
                reason
            ))
            .with_de(format!(
                "Anforderungen an Data Governance nicht erfüllt (Artikel 10): {}",
                reason
            ))
            .with_fr(format!(
                "Exigences de gouvernance des données non satisfaites (Article 10): {}",
                reason
            )),

            Self::MissingTechnicalDocumentation { missing_elements } => MultilingualText::new(format!(
                "Technical documentation required (Article 11): {}",
                missing_elements
            ))
            .with_de(format!(
                "Technische Dokumentation erforderlich (Artikel 11): {}",
                missing_elements
            ))
            .with_fr(format!(
                "Documentation technique requise (Article 11): {}",
                missing_elements
            )),

            Self::RecordKeepingViolation { reason } => MultilingualText::new(format!(
                "Record-keeping requirements not met (Article 12): {}",
                reason
            ))
            .with_de(format!(
                "Anforderungen an Aufzeichnungen nicht erfüllt (Artikel 12): {}",
                reason
            ))
            .with_fr(format!(
                "Exigences de tenue de registres non satisfaites (Article 12): {}",
                reason
            )),

            Self::InsufficientTransparency { reason } => MultilingualText::new(format!(
                "Transparency requirements not met (Article 13): {}",
                reason
            ))
            .with_de(format!(
                "Transparenzanforderungen nicht erfüllt (Artikel 13): {}",
                reason
            ))
            .with_fr(format!(
                "Exigences de transparence non satisfaites (Article 13): {}",
                reason
            )),

            Self::MissingHumanOversight => MultilingualText::new(
                "Human oversight required for high-risk AI (Article 14)"
            )
            .with_de("Menschliche Aufsicht für Hochrisiko-KI erforderlich (Artikel 14)")
            .with_fr("Surveillance humaine requise pour l'IA à haut risque (Article 14)")
            .with_es("Supervisión humana requerida para IA de alto riesgo (Artículo 14)")
            .with_it("Supervisione umana richiesta per IA ad alto rischio (Articolo 14)"),

            Self::AccuracyRobustnessViolation { reason } => MultilingualText::new(format!(
                "Accuracy/robustness requirements not met (Article 15): {}",
                reason
            ))
            .with_de(format!(
                "Anforderungen an Genauigkeit/Robustheit nicht erfüllt (Artikel 15): {}",
                reason
            ))
            .with_fr(format!(
                "Exigences de précision/robustesse non satisfaites (Article 15): {}",
                reason
            )),

            Self::LimitedRiskTransparencyViolation => MultilingualText::new(
                "Limited risk system must inform users of AI interaction (Article 52)"
            )
            .with_de("System mit begrenztem Risiko muss Nutzer über KI-Interaktion informieren (Artikel 52)")
            .with_fr("Le système à risque limité doit informer les utilisateurs de l'interaction avec l'IA (Article 52)")
            .with_es("El sistema de riesgo limitado debe informar a los usuarios de la interacción con IA (Artículo 52)")
            .with_it("Il sistema a rischio limitato deve informare gli utenti dell'interazione con l'IA (Articolo 52)"),

            Self::DeepFakeNotMarked => MultilingualText::new(
                "AI-generated content must be marked as artificially generated (Article 52)"
            )
            .with_de("KI-generierte Inhalte müssen als künstlich erzeugt gekennzeichnet werden (Artikel 52)")
            .with_fr("Le contenu généré par l'IA doit être marqué comme généré artificiellement (Article 52)")
            .with_es("El contenido generado por IA debe marcarse como generado artificialmente (Artículo 52)")
            .with_it("I contenuti generati dall'IA devono essere contrassegnati come generati artificialmente (Articolo 52)"),

            Self::BiasNotMitigated { biases } => MultilingualText::new(format!(
                "Identified biases not adequately mitigated: {}",
                biases
            ))
            .with_de(format!(
                "Identifizierte Verzerrungen nicht angemessen gemildert: {}",
                biases
            ))
            .with_fr(format!(
                "Biais identifiés non suffisamment atténués: {}",
                biases
            )),

            Self::TrainingDataQualityIssues { issues } => MultilingualText::new(format!(
                "Training data quality issues: {}",
                issues
            ))
            .with_de(format!("Probleme mit der Qualität der Trainingsdaten: {}", issues))
            .with_fr(format!("Problèmes de qualité des données d'entraînement: {}", issues))
            .with_es(format!("Problemas de calidad de los datos de entrenamiento: {}", issues))
            .with_it(format!("Problemi di qualità dei dati di addestramento: {}", issues)),

            Self::MissingField { field } => MultilingualText::new(format!(
                "Missing required field: {}",
                field
            ))
            .with_de(format!("Fehlendes Pflichtfeld: {}", field))
            .with_fr(format!("Champ obligatoire manquant: {}", field))
            .with_es(format!("Falta el campo obligatorio: {}", field))
            .with_it(format!("Campo obbligatorio mancante: {}", field)),

            Self::InvalidValue { field, reason } => MultilingualText::new(format!(
                "Invalid value for field '{}': {}",
                field, reason
            ))
            .with_de(format!("Ungültiger Wert für Feld '{}': {}", field, reason))
            .with_fr(format!("Valeur invalide pour le champ '{}': {}", field, reason)),

            Self::ConformityAssessmentFailed { reasons } => MultilingualText::new(format!(
                "Conformity assessment failed: {}",
                reasons
            ))
            .with_de(format!("Konformitätsbewertung fehlgeschlagen: {}", reasons))
            .with_fr(format!("Évaluation de conformité échouée: {}", reasons))
            .with_es(format!("Evaluación de conformidad fallida: {}", reasons))
            .with_it(format!("Valutazione di conformità fallita: {}", reasons)),

            Self::GpaiTransparencyViolation => MultilingualText::new(
                "General-purpose AI must provide transparency documentation (Article 51)"
            )
            .with_de("Allzweck-KI muss Transparenzdokumentation bereitstellen (Artikel 51)")
            .with_fr("L'IA à usage général doit fournir une documentation de transparence (Article 51)")
            .with_es("La IA de propósito general debe proporcionar documentación de transparencia (Artículo 51)")
            .with_it("L'IA per scopi generali deve fornire documentazione sulla trasparenza (Articolo 51)"),

            Self::SystemicRiskViolation => MultilingualText::new(
                "AI model with systemic risk must meet additional requirements (Article 51)"
            )
            .with_de("KI-Modell mit systemischem Risiko muss zusätzliche Anforderungen erfüllen (Artikel 51)")
            .with_fr("Le modèle d'IA à risque systémique doit répondre à des exigences supplémentaires (Article 51)")
            .with_es("El modelo de IA con riesgo sistémico debe cumplir requisitos adicionales (Artículo 51)")
            .with_it("Il modello di IA con rischio sistemico deve soddisfare requisiti aggiuntivi (Articolo 51)"),

            Self::MultipleViolations { count } => MultilingualText::new(format!(
                "Multiple AI Act violations: {}",
                count
            ))
            .with_de(format!("Mehrere KI-Gesetz-Verstöße: {}", count))
            .with_fr(format!("Violations multiples de la loi sur l'IA: {}", count))
            .with_es(format!("Múltiples violaciones de la Ley de IA: {}", count))
            .with_it(format!("Violazioni multiple della legge sull'IA: {}", count)),
        }
    }
}
