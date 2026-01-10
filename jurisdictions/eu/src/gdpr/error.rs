//! Error types for GDPR compliance validation

use crate::i18n::MultilingualText;
use thiserror::Error;

/// Errors for GDPR compliance validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum GdprError {
    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// No lawful basis provided for processing
    #[error("No lawful basis for processing under Article 6")]
    MissingLawfulBasis,

    /// Invalid lawful basis
    #[error("Invalid lawful basis: {reason}")]
    InvalidLawfulBasis { reason: String },

    /// Invalid consent (doesn't meet Article 7 requirements)
    #[error("Invalid consent: {reason}")]
    InvalidConsent { reason: String },

    /// No data categories specified
    #[error("No personal data categories specified")]
    NoDataCategories,

    /// Special category data without Article 9 exception
    #[error("Processing special categories requires Article 9 exception")]
    SpecialCategoryWithoutException,

    /// Invalid cross-border transfer
    #[error("Invalid international transfer: {reason}")]
    InvalidTransfer { reason: String },

    /// Invalid data subject request
    #[error("Invalid data subject request: {reason}")]
    InvalidRequest { reason: String },

    /// Breach notification deadline exceeded
    #[error("Breach notification deadline exceeded: {hours} hours late")]
    BreachNotificationLate { hours: i64 },

    /// Multiple GDPR violations
    #[error("Multiple GDPR violations: {0:?}")]
    MultipleViolations(Vec<String>),

    /// Processing operation not permitted
    #[error("Processing operation not permitted: {operation}")]
    OperationNotPermitted { operation: String },

    /// Data subject is a child without parental consent (Article 8)
    #[error("Child data subject requires parental consent (Article 8)")]
    ChildConsentRequired,

    /// Invalid value for field
    #[error("Invalid value for field '{field}': {reason}")]
    InvalidValue { field: String, reason: String },
}

impl GdprError {
    /// Create error for missing field
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    /// Create error for invalid lawful basis
    pub fn invalid_lawful_basis(reason: impl Into<String>) -> Self {
        Self::InvalidLawfulBasis {
            reason: reason.into(),
        }
    }

    /// Create error for invalid consent
    pub fn invalid_consent(reason: impl Into<String>) -> Self {
        Self::InvalidConsent {
            reason: reason.into(),
        }
    }

    /// Create error for invalid transfer
    pub fn invalid_transfer(reason: impl Into<String>) -> Self {
        Self::InvalidTransfer {
            reason: reason.into(),
        }
    }

    /// Create error for invalid request
    pub fn invalid_request(reason: impl Into<String>) -> Self {
        Self::InvalidRequest {
            reason: reason.into(),
        }
    }

    /// Create error for operation not permitted
    pub fn operation_not_permitted(operation: impl Into<String>) -> Self {
        Self::OperationNotPermitted {
            operation: operation.into(),
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
    /// Returns the error message in the requested language (en, de, fr) with fallback to English.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use legalis_eu::gdpr::error::GdprError;
    ///
    /// let error = GdprError::MissingLawfulBasis;
    /// assert_eq!(error.message("en"), "No lawful basis for processing under Article 6");
    /// assert_eq!(error.message("de"), "Keine Rechtsgrundlage für die Verarbeitung gemäß Artikel 6");
    /// assert_eq!(error.message("fr"), "Aucune base juridique pour le traitement en vertu de l'article 6");
    /// ```
    pub fn message(&self, lang: &str) -> String {
        let ml_text = self.to_multilingual();
        ml_text.in_language(lang).to_string()
    }

    /// Convert error to multilingual text
    fn to_multilingual(&self) -> MultilingualText {
        match self {
            Self::MissingField(field) => MultilingualText::new(format!("Missing required field: {}", field))
                .with_de(format!("Fehlendes Pflichtfeld: {}", field))
                .with_fr(format!("Champ obligatoire manquant: {}", field))
                .with_es(format!("Falta el campo obligatorio: {}", field))
                .with_it(format!("Campo obbligatorio mancante: {}", field))
                .with_pl(format!("Brak wymaganego pola: {}", field))
                .with_nl(format!("Verplicht veld ontbreekt: {}", field))
                .with_pt(format!("Campo obrigatório em falta: {}", field))
                .with_sv(format!("Obligatoriskt fält saknas: {}", field))
                .with_cs(format!("Chybí povinné pole: {}", field))
                .with_el(format!("Λείπει υποχρεωτικό πεδίο: {}", field)),

            Self::MissingLawfulBasis => MultilingualText::new("No lawful basis for processing under Article 6")
                .with_de("Keine Rechtsgrundlage für die Verarbeitung gemäß Artikel 6")
                .with_fr("Aucune base juridique pour le traitement en vertu de l'article 6")
                .with_es("No hay base legal para el tratamiento según el Artículo 6")
                .with_it("Nessuna base giuridica per il trattamento ai sensi dell'articolo 6")
                .with_pl("Brak podstawy prawnej przetwarzania zgodnie z art. 6")
                .with_nl("Geen rechtmatige grondslag voor verwerking onder artikel 6")
                .with_pt("Sem base legal para o tratamento nos termos do Artigo 6")
                .with_sv("Ingen rättslig grund för behandling enligt artikel 6")
                .with_cs("Žádný právní základ pro zpracování podle článku 6")
                .with_el("Καμία νόμιμη βάση για επεξεργασία σύμφωνα με το Άρθρο 6"),

            Self::InvalidLawfulBasis { reason } => MultilingualText::new(format!("Invalid lawful basis: {}", reason))
                .with_de(format!("Ungültige Rechtsgrundlage: {}", reason))
                .with_fr(format!("Base juridique invalide: {}", reason))
                .with_es(format!("Base legal inválida: {}", reason))
                .with_it(format!("Base giuridica non valida: {}", reason))
                .with_pl(format!("Nieprawidłowa podstawa prawna: {}", reason))
                .with_nl(format!("Ongeldige rechtmatige grondslag: {}", reason))
                .with_pt(format!("Base legal inválida: {}", reason))
                .with_sv(format!("Ogiltig rättslig grund: {}", reason))
                .with_cs(format!("Neplatný právní základ: {}", reason))
                .with_el(format!("Μη έγκυρη νόμιμη βάση: {}", reason)),

            Self::InvalidConsent { reason } => MultilingualText::new(format!("Invalid consent: {}", reason))
                .with_de(format!("Ungültige Einwilligung: {}", reason))
                .with_fr(format!("Consentement invalide: {}", reason))
                .with_es(format!("Consentimiento inválido: {}", reason))
                .with_it(format!("Consenso non valido: {}", reason))
                .with_pl(format!("Nieprawidłowa zgoda: {}", reason))
                .with_nl(format!("Ongeldige toestemming: {}", reason))
                .with_pt(format!("Consentimento inválido: {}", reason))
                .with_sv(format!("Ogiltigt samtycke: {}", reason))
                .with_cs(format!("Neplatný souhlas: {}", reason))
                .with_el(format!("Μη έγκυρη συγκατάθεση: {}", reason)),

            Self::NoDataCategories => MultilingualText::new("No personal data categories specified")
                .with_de("Keine personenbezogenen Datenkategorien angegeben")
                .with_fr("Aucune catégorie de données personnelles spécifiée")
                .with_es("No se especificaron categorías de datos personales")
                .with_it("Nessuna categoria di dati personali specificata")
                .with_pl("Nie określono kategorii danych osobowych")
                .with_nl("Geen categorieën persoonsgegevens opgegeven")
                .with_pt("Nenhuma categoria de dados pessoais especificada")
                .with_sv("Inga kategorier av personuppgifter angivna")
                .with_cs("Nebyly uvedeny žádné kategorie osobních údajů")
                .with_el("Δεν καθορίστηκαν κατηγορίες προσωπικών δεδομένων"),

            Self::SpecialCategoryWithoutException => MultilingualText::new("Processing special categories requires Article 9 exception")
                .with_de("Die Verarbeitung besonderer Kategorien erfordert eine Ausnahme nach Artikel 9")
                .with_fr("Le traitement de catégories particulières nécessite une exception de l'article 9")
                .with_es("El tratamiento de categorías especiales requiere excepción del Artículo 9")
                .with_it("Il trattamento di categorie particolari richiede un'eccezione dell'articolo 9")
                .with_pl("Przetwarzanie szczególnych kategorii wymaga wyjątku z art. 9")
                .with_nl("Verwerking van bijzondere categorieën vereist uitzondering van artikel 9")
                .with_pt("O tratamento de categorias especiais requer exceção do Artigo 9")
                .with_sv("Behandling av särskilda kategorier kräver undantag enligt artikel 9")
                .with_cs("Zpracování zvláštních kategorií vyžaduje výjimku podle článku 9")
                .with_el("Η επεξεργασία ειδικών κατηγοριών απαιτεί εξαίρεση του Άρθρου 9"),

            Self::InvalidTransfer { reason } => MultilingualText::new(format!("Invalid international transfer: {}", reason))
                .with_de(format!("Ungültige internationale Übermittlung: {}", reason))
                .with_fr(format!("Transfert international invalide: {}", reason))
                .with_es(format!("Transferencia internacional inválida: {}", reason))
                .with_it(format!("Trasferimento internazionale non valido: {}", reason))
                .with_pl(format!("Nieprawidłowe przekazanie międzynarodowe: {}", reason))
                .with_nl(format!("Ongeldige internationale doorgifte: {}", reason))
                .with_pt(format!("Transferência internacional inválida: {}", reason))
                .with_sv(format!("Ogiltig internationell överföring: {}", reason))
                .with_cs(format!("Neplatný mezinárodní přenos: {}", reason))
                .with_el(format!("Μη έγκυρη διεθνής μεταφορά: {}", reason)),

            Self::InvalidRequest { reason } => MultilingualText::new(format!("Invalid data subject request: {}", reason))
                .with_de(format!("Ungültiger Antrag der betroffenen Person: {}", reason))
                .with_fr(format!("Demande de la personne concernée invalide: {}", reason))
                .with_es(format!("Solicitud del interesado inválida: {}", reason))
                .with_it(format!("Richiesta dell'interessato non valida: {}", reason))
                .with_pl(format!("Nieprawidłowe żądanie osoby, której dotyczą dane: {}", reason))
                .with_nl(format!("Ongeldig verzoek van betrokkene: {}", reason))
                .with_pt(format!("Pedido do titular dos dados inválido: {}", reason))
                .with_sv(format!("Ogiltig begäran från registrerad: {}", reason))
                .with_cs(format!("Neplatná žádost subjektu údajů: {}", reason))
                .with_el(format!("Μη έγκυρο αίτημα υποκειμένου δεδομένων: {}", reason)),

            Self::BreachNotificationLate { hours } => MultilingualText::new(format!("Breach notification deadline exceeded: {} hours late", hours))
                .with_de(format!("Frist für Meldung von Datenschutzverletzungen überschritten: {} Stunden zu spät", hours))
                .with_fr(format!("Délai de notification de violation dépassé: {} heures de retard", hours))
                .with_es(format!("Plazo de notificación de violación excedido: {} horas de retraso", hours))
                .with_it(format!("Termine di notifica della violazione superato: {} ore di ritardo", hours))
                .with_pl(format!("Przekroczono termin zgłoszenia naruszenia: {} godzin opóźnienia", hours))
                .with_nl(format!("Melding inbreuk te laat: {} uur te laat", hours))
                .with_pt(format!("Prazo de notificação de violação excedido: {} horas de atraso", hours))
                .with_sv(format!("Tidsfristen för anmälan av intrång överskriden: {} timmar för sent", hours))
                .with_cs(format!("Lhůta pro oznámení porušení překročena: {} hodin zpoždění", hours))
                .with_el(format!("Η προθεσμία κοινοποίησης παραβίασης υπερβλήθηκε: {} ώρες καθυστέρηση", hours)),

            Self::MultipleViolations(violations) => MultilingualText::new(format!("Multiple GDPR violations: {:?}", violations))
                .with_de(format!("Mehrere DSGVO-Verstöße: {:?}", violations))
                .with_fr(format!("Violations multiples du RGPD: {:?}", violations))
                .with_es(format!("Múltiples violaciones del RGPD: {:?}", violations))
                .with_it(format!("Violazioni multiple del GDPR: {:?}", violations))
                .with_pl(format!("Wielokrotne naruszenia RODO: {:?}", violations))
                .with_nl(format!("Meerdere AVG-overtredingen: {:?}", violations))
                .with_pt(format!("Violações múltiplas do RGPD: {:?}", violations))
                .with_sv(format!("Flera GDPR-överträdelser: {:?}", violations))
                .with_cs(format!("Vícenásobná porušení GDPR: {:?}", violations))
                .with_el(format!("Πολλαπλές παραβιάσεις GDPR: {:?}", violations)),

            Self::OperationNotPermitted { operation } => MultilingualText::new(format!("Processing operation not permitted: {}", operation))
                .with_de(format!("Verarbeitungsvorgang nicht zulässig: {}", operation))
                .with_fr(format!("Opération de traitement non autorisée: {}", operation))
                .with_es(format!("Operación de tratamiento no permitida: {}", operation))
                .with_it(format!("Operazione di trattamento non consentita: {}", operation))
                .with_pl(format!("Operacja przetwarzania niedozwolona: {}", operation))
                .with_nl(format!("Verwerkingsactiviteit niet toegestaan: {}", operation))
                .with_pt(format!("Operação de tratamento não permitida: {}", operation))
                .with_sv(format!("Behandlingsåtgärd inte tillåten: {}", operation))
                .with_cs(format!("Operace zpracování není povolena: {}", operation))
                .with_el(format!("Λειτουργία επεξεργασίας δεν επιτρέπεται: {}", operation)),

            Self::ChildConsentRequired => MultilingualText::new("Child data subject requires parental consent (Article 8)")
                .with_de("Für minderjährige betroffene Person ist Einwilligung der Eltern erforderlich (Artikel 8)")
                .with_fr("Les données d'enfant nécessitent le consentement parental (Article 8)")
                .with_es("Los datos de menores requieren consentimiento parental (Artículo 8)")
                .with_it("I dati dei minori richiedono il consenso genitoriale (Articolo 8)")
                .with_pl("Dane dziecka wymagają zgody rodzicielskiej (Artykuł 8)")
                .with_nl("Gegevens van kinderen vereisen ouderlijke toestemming (Artikel 8)")
                .with_pt("Dados de crianças requerem consentimento parental (Artigo 8)")
                .with_sv("Barnuppgifter kräver föräldrasamtycke (Artikel 8)")
                .with_cs("Údaje dětí vyžadují souhlas rodičů (Článek 8)")
                .with_el("Τα δεδομένα παιδιών απαιτούν γονική συγκατάθεση (Άρθρο 8)"),

            Self::InvalidValue { field, reason } => MultilingualText::new(format!("Invalid value for field '{}': {}", field, reason))
                .with_de(format!("Ungültiger Wert für Feld '{}': {}", field, reason))
                .with_fr(format!("Valeur invalide pour le champ '{}': {}", field, reason))
                .with_es(format!("Valor inválido para el campo '{}': {}", field, reason))
                .with_it(format!("Valore non valido per il campo '{}': {}", field, reason))
                .with_pl(format!("Nieprawidłowa wartość dla pola '{}': {}", field, reason))
                .with_nl(format!("Ongeldige waarde voor veld '{}': {}", field, reason))
                .with_pt(format!("Valor inválido para o campo '{}': {}", field, reason))
                .with_sv(format!("Ogiltigt värde för fält '{}': {}", field, reason))
                .with_cs(format!("Neplatná hodnota pro pole '{}': {}", field, reason))
                .with_el(format!("Μη έγκυρη τιμή για πεδίο '{}': {}", field, reason)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = GdprError::missing_field("controller");
        assert_eq!(err.to_string(), "Missing required field: controller");

        let err2 = GdprError::MissingLawfulBasis;
        assert_eq!(
            err2.to_string(),
            "No lawful basis for processing under Article 6"
        );
    }

    #[test]
    fn test_error_construction() {
        let err = GdprError::invalid_consent("Not freely given");
        assert!(err.to_string().contains("Invalid consent"));

        let err2 = GdprError::operation_not_permitted("CrossBorderTransfer");
        assert!(err2.to_string().contains("not permitted"));
    }

    #[test]
    fn test_i18n_error_messages_english() {
        let err = GdprError::MissingLawfulBasis;
        assert_eq!(
            err.message("en"),
            "No lawful basis for processing under Article 6"
        );

        let err2 = GdprError::missing_field("controller");
        assert_eq!(err2.message("en"), "Missing required field: controller");
    }

    #[test]
    fn test_i18n_error_messages_german() {
        let err = GdprError::MissingLawfulBasis;
        assert_eq!(
            err.message("de"),
            "Keine Rechtsgrundlage für die Verarbeitung gemäß Artikel 6"
        );

        let err2 = GdprError::NoDataCategories;
        assert_eq!(
            err2.message("de"),
            "Keine personenbezogenen Datenkategorien angegeben"
        );
    }

    #[test]
    fn test_i18n_error_messages_french() {
        let err = GdprError::MissingLawfulBasis;
        assert_eq!(
            err.message("fr"),
            "Aucune base juridique pour le traitement en vertu de l'article 6"
        );

        let err2 = GdprError::SpecialCategoryWithoutException;
        assert_eq!(
            err2.message("fr"),
            "Le traitement de catégories particulières nécessite une exception de l'article 9"
        );
    }

    #[test]
    fn test_i18n_error_messages_with_parameters() {
        let err = GdprError::BreachNotificationLate { hours: 96 };
        assert!(err.message("en").contains("96 hours late"));
        assert!(err.message("de").contains("96 Stunden zu spät"));
        assert!(err.message("fr").contains("96 heures de retard"));
    }

    #[test]
    fn test_i18n_fallback_to_english() {
        let err = GdprError::MissingLawfulBasis;
        // Unsupported languages (non-EU) should fall back to English
        assert_eq!(
            err.message("ja"),
            "No lawful basis for processing under Article 6"
        );
        assert_eq!(
            err.message("zh"),
            "No lawful basis for processing under Article 6"
        );
        assert_eq!(
            err.message("ko"),
            "No lawful basis for processing under Article 6"
        );
    }

    #[test]
    fn test_i18n_all_11_languages() {
        let err = GdprError::MissingLawfulBasis;

        // Test all 11 supported languages return non-empty messages
        let languages = vec![
            "en", "de", "fr", "es", "it", "pl", "nl", "pt", "sv", "cs", "el",
        ];

        for lang in languages {
            let message = err.message(lang);
            assert!(
                !message.is_empty(),
                "Language {} should have a message",
                lang
            );
            assert!(
                message.len() > 10,
                "Language {} message should be substantial",
                lang
            );
        }

        // Verify language-specific content
        assert!(err.message("en").contains("lawful basis"));
        assert!(err.message("de").contains("Rechtsgrundlage"));
        assert!(err.message("fr").contains("base juridique"));
        assert!(err.message("es").contains("base legal"));
        assert!(err.message("it").contains("base giuridica"));
        assert!(err.message("pl").contains("podstawy prawnej"));
        assert!(err.message("nl").contains("rechtmatige grondslag"));
        assert!(err.message("pt").contains("base legal"));
        assert!(err.message("sv").contains("rättslig grund"));
        assert!(err.message("cs").contains("právní základ"));
        assert!(err.message("el").contains("νόμιμη βάση"));
    }

    #[test]
    fn test_i18n_parametric_messages_all_languages() {
        let err = GdprError::BreachNotificationLate { hours: 96 };

        // Test that all languages properly format the parameter
        assert!(err.message("en").contains("96"));
        assert!(err.message("de").contains("96"));
        assert!(err.message("fr").contains("96"));
        assert!(err.message("es").contains("96"));
        assert!(err.message("it").contains("96"));
        assert!(err.message("pl").contains("96"));
        assert!(err.message("nl").contains("96"));
        assert!(err.message("pt").contains("96"));
        assert!(err.message("sv").contains("96"));
        assert!(err.message("cs").contains("96"));
        assert!(err.message("el").contains("96"));

        // Verify language-specific time words
        assert!(err.message("en").contains("hours"));
        assert!(err.message("de").contains("Stunden"));
        assert!(err.message("fr").contains("heures"));
        assert!(err.message("es").contains("horas"));
        assert!(err.message("it").contains("ore"));
        assert!(err.message("pl").contains("godzin"));
        assert!(err.message("nl").contains("uur"));
        assert!(err.message("pt").contains("horas"));
        assert!(err.message("sv").contains("timmar"));
        assert!(err.message("cs").contains("hodin"));
        assert!(err.message("el").contains("ώρες"));
    }

    #[test]
    fn test_i18n_child_consent_all_languages() {
        let err = GdprError::ChildConsentRequired;

        // Verify Article 8 reference in all languages
        assert!(err.message("en").contains("Article 8"));
        assert!(err.message("de").contains("Artikel 8"));
        assert!(err.message("fr").contains("Article 8"));
        assert!(err.message("es").contains("Artículo 8"));
        assert!(err.message("it").contains("Articolo 8"));
        assert!(err.message("pl").contains("Artykuł 8"));
        assert!(err.message("nl").contains("Artikel 8"));
        assert!(err.message("pt").contains("Artigo 8"));
        assert!(err.message("sv").contains("Artikel 8"));
        assert!(err.message("cs").contains("Článek 8"));
        assert!(err.message("el").contains("Άρθρο 8"));

        // Verify parental consent concept in all languages
        assert!(err.message("en").contains("parental"));
        assert!(err.message("de").contains("Eltern"));
        assert!(err.message("fr").contains("parental"));
        assert!(err.message("es").contains("parental"));
        assert!(err.message("it").contains("genitoriale"));
        assert!(err.message("pl").contains("rodzicielskiej"));
        assert!(err.message("nl").contains("ouderlijke"));
        assert!(err.message("pt").contains("parental"));
        assert!(err.message("sv").contains("föräldrasamtycke"));
        assert!(err.message("cs").contains("rodičů"));
        assert!(err.message("el").contains("γονική"));
    }
}
