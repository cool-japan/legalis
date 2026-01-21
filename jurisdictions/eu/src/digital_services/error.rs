//! Error types for Digital Services Act (DSA) and Digital Markets Act (DMA) compliance

use crate::i18n::MultilingualText;
use thiserror::Error;

/// Errors for DSA/DMA compliance validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DigitalServicesError {
    /// Platform does not meet VLOP threshold
    #[error("Platform does not meet VLOP threshold: {recipients}M recipients (requires >= 45M)")]
    BelowVlopThreshold { recipients: u64 },

    /// Platform does not meet VLOSE threshold
    #[error("Platform does not meet VLOSE threshold: {recipients}M recipients (requires >= 45M)")]
    BelowVloseThreshold { recipients: u64 },

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// Invalid illegal content notice
    #[error("Invalid illegal content notice: {reason}")]
    InvalidNotice { reason: String },

    /// Notice response deadline exceeded
    #[error("Notice response deadline exceeded: platform must respond promptly")]
    NoticeResponseDelayed,

    /// Missing statement of reasons
    #[error("Missing statement of reasons for content moderation decision")]
    MissingStatementOfReasons,

    /// Insufficient transparency reporting
    #[error("Insufficient transparency reporting: {missing_elements}")]
    InsufficientTransparency { missing_elements: String },

    /// Systemic risk assessment not performed
    #[error("VLOP/VLOSE must conduct annual systemic risk assessment (Article 34)")]
    MissingSystemicRiskAssessment,

    /// Risk mitigation measures inadequate
    #[error("Risk mitigation measures inadequate: {reason}")]
    InadequateRiskMitigation { reason: String },

    /// Missing redress mechanism
    #[error("Platform must provide effective redress mechanisms (Article 20-21)")]
    MissingRedressMechanism,

    /// Algorithmic transparency violation
    #[error("VLOP/VLOSE must provide algorithmic transparency (Article 27): {violation}")]
    AlgorithmicTransparencyViolation { violation: String },

    /// Trusted flagger framework violation
    #[error("Violation of trusted flagger framework (Article 22): {reason}")]
    TrustedFlaggerViolation { reason: String },

    /// Gatekeeper designation threshold not met
    #[error("Does not meet gatekeeper designation thresholds: {missing_criteria}")]
    NotGatekeeper { missing_criteria: String },

    /// Gatekeeper obligation violation
    #[error("Gatekeeper obligation violated: {obligation}")]
    GatekeeperObligationViolation { obligation: String },

    /// Interoperability requirement not met
    #[error("Interoperability requirement not met: {requirement}")]
    InteroperabilityViolation { requirement: String },

    /// Self-preferencing violation (Article 6(f))
    #[error("Self-preferencing violation: gatekeeper gave preferential treatment")]
    SelfPreferencing,

    /// Data combination without consent (Article 5(a))
    #[error("Illegal data combination without consent")]
    IllegalDataCombination,

    /// App store fairness violation (Article 6(k))
    #[error("App store terms not fair, reasonable, or non-discriminatory")]
    UnfairAppStoreTerms,

    /// Missing data portability tools (Article 6(b))
    #[error("Gatekeeper must provide effective data portability tools")]
    MissingDataPortabilityTools,

    /// Invalid compliance report
    #[error("Invalid DMA compliance report: {reason}")]
    InvalidComplianceReport { reason: String },

    /// Multiple violations
    #[error("Multiple DSA/DMA violations: {count}")]
    MultipleViolations { count: usize },

    /// Invalid value for field
    #[error("Invalid value for field '{field}': {reason}")]
    InvalidValue { field: String, reason: String },
}

impl DigitalServicesError {
    /// Create error for missing field
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Create error for invalid notice
    pub fn invalid_notice(reason: impl Into<String>) -> Self {
        Self::InvalidNotice {
            reason: reason.into(),
        }
    }

    /// Create error for insufficient transparency
    pub fn insufficient_transparency(missing_elements: impl Into<String>) -> Self {
        Self::InsufficientTransparency {
            missing_elements: missing_elements.into(),
        }
    }

    /// Create error for inadequate risk mitigation
    pub fn inadequate_risk_mitigation(reason: impl Into<String>) -> Self {
        Self::InadequateRiskMitigation {
            reason: reason.into(),
        }
    }

    /// Create error for algorithmic transparency violation
    pub fn algorithmic_transparency_violation(violation: impl Into<String>) -> Self {
        Self::AlgorithmicTransparencyViolation {
            violation: violation.into(),
        }
    }

    /// Create error for trusted flagger violation
    pub fn trusted_flagger_violation(reason: impl Into<String>) -> Self {
        Self::TrustedFlaggerViolation {
            reason: reason.into(),
        }
    }

    /// Create error for not being a gatekeeper
    pub fn not_gatekeeper(missing_criteria: impl Into<String>) -> Self {
        Self::NotGatekeeper {
            missing_criteria: missing_criteria.into(),
        }
    }

    /// Create error for gatekeeper obligation violation
    pub fn gatekeeper_obligation_violation(obligation: impl Into<String>) -> Self {
        Self::GatekeeperObligationViolation {
            obligation: obligation.into(),
        }
    }

    /// Create error for interoperability violation
    pub fn interoperability_violation(requirement: impl Into<String>) -> Self {
        Self::InteroperabilityViolation {
            requirement: requirement.into(),
        }
    }

    /// Create error for invalid compliance report
    pub fn invalid_compliance_report(reason: impl Into<String>) -> Self {
        Self::InvalidComplianceReport {
            reason: reason.into(),
        }
    }

    /// Create error for invalid value
    pub fn invalid_value(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Get localized error message
    ///
    /// Returns the error message in the requested language with fallback to English.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use legalis_eu::digital_services::DigitalServicesError;
    ///
    /// let error = DigitalServicesError::MissingStatementOfReasons;
    /// assert_eq!(error.message("en"), "Missing statement of reasons for content moderation decision");
    /// ```
    pub fn message(&self, lang: &str) -> String {
        let ml_text = self.to_multilingual();
        ml_text.in_language(lang).to_string()
    }

    /// Convert error to multilingual text
    fn to_multilingual(&self) -> MultilingualText {
        match self {
            Self::BelowVlopThreshold { recipients } => MultilingualText::new(format!(
                "Platform does not meet VLOP threshold: {}M recipients (requires >= 45M)",
                recipients
            ))
            .with_de(format!(
                "Plattform erreicht nicht die VLOP-Schwelle: {}M Nutzer (erforderlich >= 45M)",
                recipients
            ))
            .with_fr(format!(
                "La plateforme n'atteint pas le seuil VLOP: {}M utilisateurs (requis >= 45M)",
                recipients
            ))
            .with_es(format!(
                "La plataforma no alcanza el umbral VLOP: {}M usuarios (se requiere >= 45M)",
                recipients
            ))
            .with_it(format!(
                "La piattaforma non raggiunge la soglia VLOP: {}M utenti (richiesti >= 45M)",
                recipients
            )),

            Self::BelowVloseThreshold { recipients } => MultilingualText::new(format!(
                "Platform does not meet VLOSE threshold: {}M recipients (requires >= 45M)",
                recipients
            ))
            .with_de(format!(
                "Plattform erreicht nicht die VLOSE-Schwelle: {}M Nutzer (erforderlich >= 45M)",
                recipients
            ))
            .with_fr(format!(
                "La plateforme n'atteint pas le seuil VLOSE: {}M utilisateurs (requis >= 45M)",
                recipients
            )),

            Self::MissingField { field } => MultilingualText::new(format!("Missing required field: {}", field))
                .with_de(format!("Fehlendes Pflichtfeld: {}", field))
                .with_fr(format!("Champ obligatoire manquant: {}", field))
                .with_es(format!("Falta el campo obligatorio: {}", field))
                .with_it(format!("Campo obbligatorio mancante: {}", field)),

            Self::InvalidNotice { reason } => MultilingualText::new(format!("Invalid illegal content notice: {}", reason))
                .with_de(format!("Ungültige Meldung rechtswidriger Inhalte: {}", reason))
                .with_fr(format!("Notification de contenu illégal invalide: {}", reason))
                .with_es(format!("Notificación de contenido ilegal inválida: {}", reason))
                .with_it(format!("Notifica di contenuto illegale non valida: {}", reason)),

            Self::NoticeResponseDelayed => MultilingualText::new(
                "Notice response deadline exceeded: platform must respond promptly"
            )
            .with_de("Frist für Antwort auf Meldung überschritten: Plattform muss unverzüglich antworten")
            .with_fr("Délai de réponse à la notification dépassé: la plateforme doit répondre rapidement")
            .with_es("Plazo de respuesta a la notificación excedido: la plataforma debe responder con prontitud")
            .with_it("Scadenza della risposta alla notifica superata: la piattaforma deve rispondere tempestivamente"),

            Self::MissingStatementOfReasons => MultilingualText::new(
                "Missing statement of reasons for content moderation decision"
            )
            .with_de("Fehlende Begründung für Entscheidung zur Inhaltsmoderation")
            .with_fr("Déclaration de motifs manquante pour la décision de modération de contenu")
            .with_es("Falta declaración de motivos para la decisión de moderación de contenido")
            .with_it("Manca la dichiarazione dei motivi per la decisione di moderazione dei contenuti"),

            Self::InsufficientTransparency { missing_elements } => MultilingualText::new(format!(
                "Insufficient transparency reporting: {}",
                missing_elements
            ))
            .with_de(format!("Unzureichende Transparenzberichterstattung: {}", missing_elements))
            .with_fr(format!("Rapport de transparence insuffisant: {}", missing_elements))
            .with_es(format!("Informe de transparencia insuficiente: {}", missing_elements))
            .with_it(format!("Rapporto di trasparenza insufficiente: {}", missing_elements)),

            Self::MissingSystemicRiskAssessment => MultilingualText::new(
                "VLOP/VLOSE must conduct annual systemic risk assessment (Article 34)"
            )
            .with_de("VLOP/VLOSE muss jährliche systemische Risikobewertung durchführen (Artikel 34)")
            .with_fr("VLOP/VLOSE doit effectuer une évaluation annuelle des risques systémiques (Article 34)")
            .with_es("VLOP/VLOSE debe realizar evaluación anual de riesgos sistémicos (Artículo 34)")
            .with_it("VLOP/VLOSE deve condurre valutazione annuale dei rischi sistemici (Articolo 34)"),

            Self::InadequateRiskMitigation { reason } => MultilingualText::new(format!(
                "Risk mitigation measures inadequate: {}",
                reason
            ))
            .with_de(format!("Risikominderungsmaßnahmen unzureichend: {}", reason))
            .with_fr(format!("Mesures d'atténuation des risques inadéquates: {}", reason))
            .with_es(format!("Medidas de mitigación de riesgos inadecuadas: {}", reason))
            .with_it(format!("Misure di mitigazione dei rischi inadeguate: {}", reason)),

            Self::MissingRedressMechanism => MultilingualText::new(
                "Platform must provide effective redress mechanisms (Article 20-21)"
            )
            .with_de("Plattform muss wirksame Rechtsbehelfsverfahren bereitstellen (Artikel 20-21)")
            .with_fr("La plateforme doit fournir des mécanismes de recours efficaces (Article 20-21)")
            .with_es("La plataforma debe proporcionar mecanismos de recurso efectivos (Artículo 20-21)")
            .with_it("La piattaforma deve fornire meccanismi di ricorso efficaci (Articolo 20-21)"),

            Self::AlgorithmicTransparencyViolation { violation } => MultilingualText::new(format!(
                "VLOP/VLOSE must provide algorithmic transparency (Article 27): {}",
                violation
            ))
            .with_de(format!(
                "VLOP/VLOSE muss algorithmische Transparenz bieten (Artikel 27): {}",
                violation
            ))
            .with_fr(format!(
                "VLOP/VLOSE doit fournir la transparence algorithmique (Article 27): {}",
                violation
            )),

            Self::TrustedFlaggerViolation { reason } => MultilingualText::new(format!(
                "Violation of trusted flagger framework (Article 22): {}",
                reason
            ))
            .with_de(format!("Verstoß gegen Rahmen für vertrauenswürdige Hinweisgeber (Artikel 22): {}", reason))
            .with_fr(format!("Violation du cadre des signaleurs de confiance (Article 22): {}", reason)),

            Self::NotGatekeeper { missing_criteria } => MultilingualText::new(format!(
                "Does not meet gatekeeper designation thresholds: {}",
                missing_criteria
            ))
            .with_de(format!(
                "Erfüllt nicht die Schwellenwerte für Torwächter-Benennung: {}",
                missing_criteria
            ))
            .with_fr(format!(
                "Ne remplit pas les seuils de désignation de contrôleur d'accès: {}",
                missing_criteria
            )),

            Self::GatekeeperObligationViolation { obligation } => MultilingualText::new(format!(
                "Gatekeeper obligation violated: {}",
                obligation
            ))
            .with_de(format!("Torwächter-Verpflichtung verletzt: {}", obligation))
            .with_fr(format!("Obligation de contrôleur d'accès violée: {}", obligation))
            .with_es(format!("Obligación de controlador de acceso violada: {}", obligation))
            .with_it(format!("Obbligo di gatekeeper violato: {}", obligation)),

            Self::InteroperabilityViolation { requirement } => MultilingualText::new(format!(
                "Interoperability requirement not met: {}",
                requirement
            ))
            .with_de(format!("Interoperabilitätsanforderung nicht erfüllt: {}", requirement))
            .with_fr(format!("Exigence d'interopérabilité non satisfaite: {}", requirement))
            .with_es(format!("Requisito de interoperabilidad no cumplido: {}", requirement))
            .with_it(format!("Requisito di interoperabilità non soddisfatto: {}", requirement)),

            Self::SelfPreferencing => MultilingualText::new(
                "Self-preferencing violation: gatekeeper gave preferential treatment"
            )
            .with_de("Selbstbevorzugung: Torwächter gewährte Vorzugsbehandlung")
            .with_fr("Auto-préférence: le contrôleur d'accès a accordé un traitement préférentiel")
            .with_es("Autopreferencia: el controlador de acceso otorgó trato preferencial")
            .with_it("Auto-preferenza: il gatekeeper ha concesso trattamento preferenziale"),

            Self::IllegalDataCombination => MultilingualText::new(
                "Illegal data combination without consent"
            )
            .with_de("Illegale Datenkombination ohne Einwilligung")
            .with_fr("Combinaison de données illégale sans consentement")
            .with_es("Combinación ilegal de datos sin consentimiento")
            .with_it("Combinazione illegale di dati senza consenso"),

            Self::UnfairAppStoreTerms => MultilingualText::new(
                "App store terms not fair, reasonable, or non-discriminatory"
            )
            .with_de("App-Store-Bedingungen nicht fair, angemessen oder nicht-diskriminierend")
            .with_fr("Conditions de l'app store pas équitables, raisonnables ou non discriminatoires")
            .with_es("Términos de la tienda de aplicaciones no justos, razonables o no discriminatorios")
            .with_it("Termini dell'app store non equi, ragionevoli o non discriminatori"),

            Self::MissingDataPortabilityTools => MultilingualText::new(
                "Gatekeeper must provide effective data portability tools"
            )
            .with_de("Torwächter muss wirksame Datenübertragbarkeits-Tools bereitstellen")
            .with_fr("Le contrôleur d'accès doit fournir des outils de portabilité des données efficaces")
            .with_es("El controlador de acceso debe proporcionar herramientas efectivas de portabilidad de datos")
            .with_it("Il gatekeeper deve fornire strumenti efficaci di portabilità dei dati"),

            Self::InvalidComplianceReport { reason } => MultilingualText::new(format!(
                "Invalid DMA compliance report: {}",
                reason
            ))
            .with_de(format!("Ungültiger DMA-Compliance-Bericht: {}", reason))
            .with_fr(format!("Rapport de conformité DMA invalide: {}", reason))
            .with_es(format!("Informe de cumplimiento DMA inválido: {}", reason))
            .with_it(format!("Rapporto di conformità DMA non valido: {}", reason)),

            Self::MultipleViolations { count } => MultilingualText::new(format!(
                "Multiple DSA/DMA violations: {}",
                count
            ))
            .with_de(format!("Mehrere DSA/DMA-Verstöße: {}", count))
            .with_fr(format!("Violations multiples DSA/DMA: {}", count))
            .with_es(format!("Múltiples violaciones DSA/DMA: {}", count))
            .with_it(format!("Violazioni multiple DSA/DMA: {}", count)),

            Self::InvalidValue { field, reason } => MultilingualText::new(format!(
                "Invalid value for field '{}': {}",
                field, reason
            ))
            .with_de(format!("Ungültiger Wert für Feld '{}': {}", field, reason))
            .with_fr(format!("Valeur invalide pour le champ '{}': {}", field, reason))
            .with_es(format!("Valor inválido para el campo '{}': {}", field, reason))
            .with_it(format!("Valore non valido per il campo '{}': {}", field, reason)),
        }
    }
}
