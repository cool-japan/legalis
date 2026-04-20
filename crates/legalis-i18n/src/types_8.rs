//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::{EURegulationType, TermIndex};
use super::types_3::{CourtParticipantRole, NameOrder, PartyRole, PluralCategory};
use super::types_6::{LegalSystem, PluralRules};
use super::types_7::CulturalParams;
use super::types_9::{EmphasisLevel, IdentifiedParty};
use super::types_10::Locale;

/// Translation dictionary for legal terms.
#[derive(Debug, Clone, Default)]
pub struct LegalDictionary {
    /// Locale this dictionary is for
    pub locale: Locale,
    /// Term translations: key -> translated term
    pub(super) translations: IndexMap<String, String>,
    /// Legal definitions: term -> definition
    pub(super) definitions: IndexMap<String, String>,
    /// Abbreviations: full term -> abbreviation
    pub(super) abbreviations: IndexMap<String, String>,
    /// Reverse abbreviation lookup: abbreviation -> full term
    pub(super) abbreviation_expansions: IndexMap<String, String>,
    /// Context-aware translations: (key, context) -> translation
    pub(super) contextual_translations: IndexMap<(String, String), String>,
}
impl LegalDictionary {
    /// Creates a new dictionary for a locale.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{LegalDictionary, Locale};
    ///
    /// let locale = Locale::new("ja").with_country("JP");
    /// let mut dict = LegalDictionary::new(locale);
    ///
    /// dict.add_translation("contract", "契約");
    /// dict.add_translation("statute", "法律");
    ///
    /// assert_eq!(dict.translate("contract"), Some("契約"));
    /// assert_eq!(dict.translate("statute"), Some("法律"));
    /// ```
    pub fn new(locale: Locale) -> Self {
        Self {
            locale,
            translations: IndexMap::new(),
            definitions: IndexMap::new(),
            abbreviations: IndexMap::new(),
            abbreviation_expansions: IndexMap::new(),
            contextual_translations: IndexMap::new(),
        }
    }
    /// Adds a translation.
    pub fn add_translation(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.translations.insert(key.into(), value.into());
    }
    /// Adds a definition.
    pub fn add_definition(&mut self, term: impl Into<String>, definition: impl Into<String>) {
        self.definitions.insert(term.into(), definition.into());
    }
    /// Gets a translation.
    pub fn translate(&self, key: &str) -> Option<&str> {
        self.translations.get(key).map(|s| s.as_str())
    }
    /// Gets a definition.
    pub fn define(&self, term: &str) -> Option<&str> {
        self.definitions.get(term).map(|s| s.as_str())
    }
    /// Adds an abbreviation for a term.
    pub fn add_abbreviation(&mut self, term: impl Into<String>, abbr: impl Into<String>) {
        let term_str = term.into();
        let abbr_str = abbr.into();
        self.abbreviation_expansions
            .insert(abbr_str.clone(), term_str.clone());
        self.abbreviations.insert(term_str, abbr_str);
    }
    /// Gets the abbreviation for a term.
    pub fn get_abbreviation(&self, term: &str) -> Option<&str> {
        self.abbreviations.get(term).map(|s| s.as_str())
    }
    /// Expands an abbreviation to its full term.
    pub fn expand_abbreviation(&self, abbr: &str) -> Option<&str> {
        self.abbreviation_expansions.get(abbr).map(|s| s.as_str())
    }
    /// Checks if a string is a known abbreviation.
    pub fn is_abbreviation(&self, text: &str) -> bool {
        self.abbreviation_expansions.contains_key(text)
    }
    /// Adds a context-aware translation.
    /// Context can be used to disambiguate terms with multiple meanings.
    /// Examples: "right" with context "legal" vs "direction", "party" with context "contract" vs "celebration"
    pub fn add_contextual_translation(
        &mut self,
        key: impl Into<String>,
        context: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.contextual_translations
            .insert((key.into(), context.into()), value.into());
    }
    /// Gets a translation with context.
    /// If no contextual translation is found, falls back to the default translation.
    pub fn translate_with_context(&self, key: &str, context: &str) -> Option<&str> {
        if let Some(translation) = self
            .contextual_translations
            .get(&(key.to_string(), context.to_string()))
        {
            return Some(translation.as_str());
        }
        self.translate(key)
    }
    /// Lists all available contexts for a given key.
    pub fn get_contexts_for_term(&self, key: &str) -> Vec<&str> {
        self.contextual_translations
            .keys()
            .filter_map(|(k, ctx)| if k == key { Some(ctx.as_str()) } else { None })
            .collect()
    }
    /// Exports the dictionary to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    /// Imports a dictionary from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    /// Gets the number of translations in this dictionary.
    pub fn translation_count(&self) -> usize {
        self.translations.len()
    }
    /// Gets the number of definitions in this dictionary.
    pub fn definition_count(&self) -> usize {
        self.definitions.len()
    }
    /// Gets the number of abbreviations in this dictionary.
    pub fn abbreviation_count(&self) -> usize {
        self.abbreviations.len()
    }
    /// Gets the number of contextual translations in this dictionary.
    pub fn contextual_translation_count(&self) -> usize {
        self.contextual_translations.len()
    }
    /// Merges another dictionary into this one.
    /// Existing entries are preserved; only new entries are added.
    pub fn merge(&mut self, other: &LegalDictionary) {
        for (key, value) in &other.translations {
            self.translations
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
        for (key, value) in &other.definitions {
            self.definitions
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
        for (key, value) in &other.abbreviations {
            self.abbreviations
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
        for (key, value) in &other.abbreviation_expansions {
            self.abbreviation_expansions
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
        for (key, value) in &other.contextual_translations {
            self.contextual_translations
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
    }
    /// Creates a standard English (US) legal dictionary.
    pub fn english_us() -> Self {
        let mut dict = Self::new(Locale::new("en").with_country("US"));
        dict.add_translation("statute", "statute");
        dict.add_translation("law", "law");
        dict.add_translation("regulation", "regulation");
        dict.add_translation("contract", "contract");
        dict.add_translation("agreement", "agreement");
        dict.add_translation("liability", "liability");
        dict.add_translation("obligation", "obligation");
        dict.add_translation("right", "right");
        dict.add_translation("duty", "duty");
        dict.add_translation("party", "party");
        dict.add_translation("plaintiff", "plaintiff");
        dict.add_translation("defendant", "defendant");
        dict.add_translation("court", "court");
        dict.add_translation("judge", "judge");
        dict.add_translation("jury", "jury");
        dict.add_translation("attorney", "attorney");
        dict.add_translation("lawyer", "lawyer");
        dict.add_translation("counsel", "counsel");
        dict.add_translation("witness", "witness");
        dict.add_translation("evidence", "evidence");
        dict.add_translation("testimony", "testimony");
        dict.add_translation("verdict", "verdict");
        dict.add_translation("judgment", "judgment");
        dict.add_translation("appeal", "appeal");
        dict.add_translation("damages", "damages");
        dict.add_translation("penalty", "penalty");
        dict.add_translation("fine", "fine");
        dict.add_translation("corporation", "corporation");
        dict.add_translation("shareholder", "shareholder");
        dict.add_translation("director", "director");
        dict.add_translation("officer", "officer");
        dict.add_translation("bylaws", "bylaws");
        dict.add_translation("merger", "merger");
        dict.add_translation("acquisition", "acquisition");
        dict.add_translation("dividend", "dividend");
        dict.add_translation("stock", "stock");
        dict.add_translation("securities", "securities");
        dict.add_translation("property", "property");
        dict.add_translation("real_estate", "real estate");
        dict.add_translation("ownership", "ownership");
        dict.add_translation("lease", "lease");
        dict.add_translation("tenant", "tenant");
        dict.add_translation("landlord", "landlord");
        dict.add_translation("mortgage", "mortgage");
        dict.add_translation("deed", "deed");
        dict.add_translation("title", "title");
        dict.add_translation("easement", "easement");
        dict.add_translation("crime", "crime");
        dict.add_translation("felony", "felony");
        dict.add_translation("misdemeanor", "misdemeanor");
        dict.add_translation("prosecution", "prosecution");
        dict.add_translation("indictment", "indictment");
        dict.add_translation("conviction", "conviction");
        dict.add_translation("sentence", "sentence");
        dict.add_translation("probation", "probation");
        dict.add_translation("parole", "parole");
        dict.add_translation("bail", "bail");
        dict.add_translation("jurisdiction", "jurisdiction");
        dict.add_translation("venue", "venue");
        dict.add_translation("standing", "standing");
        dict.add_translation("discovery", "discovery");
        dict.add_translation("deposition", "deposition");
        dict.add_translation("motion", "motion");
        dict.add_translation("injunction", "injunction");
        dict.add_translation("subpoena", "subpoena");
        dict.add_translation("hearing", "hearing");
        dict.add_translation("trial", "trial");
        dict.add_translation("patent", "patent");
        dict.add_translation("trademark", "trademark");
        dict.add_translation("copyright", "copyright");
        dict.add_translation("infringement", "infringement");
        dict.add_translation("royalty", "royalty");
        dict.add_translation("license", "license");
        dict.add_translation("marriage", "marriage");
        dict.add_translation("divorce", "divorce");
        dict.add_translation("custody", "custody");
        dict.add_translation("alimony", "alimony");
        dict.add_translation("adoption", "adoption");
        dict.add_translation("guardianship", "guardianship");
        dict.add_translation("arbitration", "arbitration");
        dict.add_translation("mediation", "mediation");
        dict.add_translation("settlement", "settlement");
        dict.add_translation("litigation", "litigation");
        dict.add_translation("precedent", "precedent");
        dict.add_translation("statute_of_limitations", "statute of limitations");
        dict.add_abbreviation("corporation", "Corp.");
        dict.add_abbreviation("incorporated", "Inc.");
        dict.add_abbreviation("limited_liability_company", "LLC");
        dict.add_abbreviation("attorney", "Atty.");
        dict.add_abbreviation("versus", "v.");
        dict.add_abbreviation("plaintiff", "Pl.");
        dict.add_abbreviation("defendant", "Def.");
        dict.add_abbreviation("contract", "K");
        dict.add_abbreviation("statute", "Stat.");
        dict.add_abbreviation("section", "§");
        dict.add_abbreviation("article", "Art.");
        dict.add_abbreviation("paragraph", "Para.");
        dict.add_abbreviation("supreme_court", "S.Ct.");
        dict.add_abbreviation("district_court", "D.C.");
        dict.add_abbreviation("court_of_appeals", "C.A.");
        dict.add_abbreviation("federal_register", "Fed. Reg.");
        dict.add_abbreviation("code_of_federal_regulations", "C.F.R.");
        dict.add_abbreviation("united_states_code", "U.S.C.");
        dict
    }
    /// Creates a standard Japanese legal dictionary.
    pub fn japanese() -> Self {
        let mut dict = Self::new(Locale::new("ja").with_country("JP"));
        dict.add_translation("statute", "法律");
        dict.add_translation("law", "法");
        dict.add_translation("regulation", "規則");
        dict.add_translation("contract", "契約");
        dict.add_translation("agreement", "合意");
        dict.add_translation("liability", "責任");
        dict.add_translation("obligation", "義務");
        dict.add_translation("right", "権利");
        dict.add_translation("duty", "義務");
        dict.add_translation("party", "当事者");
        dict.add_translation("plaintiff", "原告");
        dict.add_translation("defendant", "被告");
        dict.add_translation("court", "裁判所");
        dict.add_translation("judge", "裁判官");
        dict.add_translation("jury", "陪審");
        dict.add_translation("attorney", "弁護士");
        dict.add_translation("lawyer", "弁護士");
        dict.add_translation("counsel", "法律顧問");
        dict.add_translation("witness", "証人");
        dict.add_translation("evidence", "証拠");
        dict.add_translation("testimony", "証言");
        dict.add_translation("verdict", "評決");
        dict.add_translation("judgment", "判決");
        dict.add_translation("appeal", "控訴");
        dict.add_translation("damages", "損害賠償");
        dict.add_translation("penalty", "罰則");
        dict.add_translation("fine", "罰金");
        dict.add_translation("corporation", "法人");
        dict.add_translation("shareholder", "株主");
        dict.add_translation("director", "取締役");
        dict.add_translation("officer", "役員");
        dict.add_translation("bylaws", "定款");
        dict.add_translation("merger", "合併");
        dict.add_translation("acquisition", "買収");
        dict.add_translation("dividend", "配当");
        dict.add_translation("stock", "株式");
        dict.add_translation("securities", "有価証券");
        dict.add_translation("property", "財産");
        dict.add_translation("real_estate", "不動産");
        dict.add_translation("ownership", "所有権");
        dict.add_translation("lease", "賃貸借");
        dict.add_translation("tenant", "賃借人");
        dict.add_translation("landlord", "賃貸人");
        dict.add_translation("mortgage", "抵当権");
        dict.add_translation("deed", "証書");
        dict.add_translation("title", "権原");
        dict.add_translation("easement", "地役権");
        dict.add_translation("crime", "犯罪");
        dict.add_translation("felony", "重罪");
        dict.add_translation("misdemeanor", "軽罪");
        dict.add_translation("prosecution", "起訴");
        dict.add_translation("indictment", "起訴状");
        dict.add_translation("conviction", "有罪判決");
        dict.add_translation("sentence", "刑");
        dict.add_translation("probation", "執行猶予");
        dict.add_translation("parole", "仮釈放");
        dict.add_translation("bail", "保釈");
        dict.add_translation("jurisdiction", "管轄");
        dict.add_translation("venue", "裁判地");
        dict.add_translation("standing", "当事者適格");
        dict.add_translation("discovery", "証拠開示");
        dict.add_translation("deposition", "証言録取");
        dict.add_translation("motion", "申立て");
        dict.add_translation("injunction", "差止め");
        dict.add_translation("subpoena", "召喚状");
        dict.add_translation("hearing", "審理");
        dict.add_translation("trial", "裁判");
        dict.add_translation("patent", "特許");
        dict.add_translation("trademark", "商標");
        dict.add_translation("copyright", "著作権");
        dict.add_translation("infringement", "侵害");
        dict.add_translation("royalty", "使用料");
        dict.add_translation("license", "ライセンス");
        dict.add_translation("marriage", "婚姻");
        dict.add_translation("divorce", "離婚");
        dict.add_translation("custody", "親権");
        dict.add_translation("alimony", "扶養料");
        dict.add_translation("adoption", "養子縁組");
        dict.add_translation("guardianship", "後見");
        dict.add_translation("arbitration", "仲裁");
        dict.add_translation("mediation", "調停");
        dict.add_translation("settlement", "和解");
        dict.add_translation("litigation", "訴訟");
        dict.add_translation("precedent", "判例");
        dict.add_translation("statute_of_limitations", "時効");
        dict
    }
    /// Creates a standard German legal dictionary.
    pub fn german() -> Self {
        let mut dict = Self::new(Locale::new("de").with_country("DE"));
        dict.add_translation("statute", "Gesetz");
        dict.add_translation("law", "Recht");
        dict.add_translation("regulation", "Verordnung");
        dict.add_translation("contract", "Vertrag");
        dict.add_translation("agreement", "Vereinbarung");
        dict.add_translation("liability", "Haftung");
        dict.add_translation("obligation", "Verpflichtung");
        dict.add_translation("right", "Recht");
        dict.add_translation("duty", "Pflicht");
        dict.add_translation("party", "Partei");
        dict.add_translation("plaintiff", "Kläger");
        dict.add_translation("defendant", "Beklagter");
        dict.add_translation("court", "Gericht");
        dict.add_translation("judge", "Richter");
        dict.add_translation("jury", "Geschworene");
        dict.add_translation("attorney", "Rechtsanwalt");
        dict.add_translation("lawyer", "Anwalt");
        dict.add_translation("counsel", "Rechtsbeistand");
        dict.add_translation("witness", "Zeuge");
        dict.add_translation("evidence", "Beweis");
        dict.add_translation("testimony", "Zeugenaussage");
        dict.add_translation("verdict", "Urteil");
        dict.add_translation("judgment", "Urteil");
        dict.add_translation("appeal", "Berufung");
        dict.add_translation("damages", "Schadensersatz");
        dict.add_translation("penalty", "Strafe");
        dict.add_translation("fine", "Geldstrafe");
        dict.add_translation("corporation", "Gesellschaft");
        dict.add_translation("shareholder", "Aktionär");
        dict.add_translation("director", "Direktor");
        dict.add_translation("officer", "Vorstand");
        dict.add_translation("bylaws", "Satzung");
        dict.add_translation("merger", "Fusion");
        dict.add_translation("acquisition", "Übernahme");
        dict.add_translation("dividend", "Dividende");
        dict.add_translation("stock", "Aktie");
        dict.add_translation("securities", "Wertpapiere");
        dict.add_translation("property", "Eigentum");
        dict.add_translation("real_estate", "Immobilien");
        dict.add_translation("ownership", "Eigentum");
        dict.add_translation("lease", "Miete");
        dict.add_translation("tenant", "Mieter");
        dict.add_translation("landlord", "Vermieter");
        dict.add_translation("mortgage", "Hypothek");
        dict.add_translation("deed", "Urkunde");
        dict.add_translation("title", "Titel");
        dict.add_translation("easement", "Grunddienstbarkeit");
        dict.add_translation("crime", "Verbrechen");
        dict.add_translation("felony", "Verbrechen");
        dict.add_translation("misdemeanor", "Vergehen");
        dict.add_translation("prosecution", "Strafverfolgung");
        dict.add_translation("indictment", "Anklage");
        dict.add_translation("conviction", "Verurteilung");
        dict.add_translation("sentence", "Strafe");
        dict.add_translation("probation", "Bewährung");
        dict.add_translation("parole", "Bewährung");
        dict.add_translation("bail", "Kaution");
        dict.add_translation("jurisdiction", "Zuständigkeit");
        dict.add_translation("venue", "Gerichtsstand");
        dict.add_translation("standing", "Klagebefugnis");
        dict.add_translation("discovery", "Beweiserhebung");
        dict.add_translation("deposition", "Zeugenaussage");
        dict.add_translation("motion", "Antrag");
        dict.add_translation("injunction", "Einstweilige Verfügung");
        dict.add_translation("subpoena", "Vorladung");
        dict.add_translation("hearing", "Anhörung");
        dict.add_translation("trial", "Verhandlung");
        dict.add_translation("patent", "Patent");
        dict.add_translation("trademark", "Marke");
        dict.add_translation("copyright", "Urheberrecht");
        dict.add_translation("infringement", "Verletzung");
        dict.add_translation("royalty", "Lizenzgebühr");
        dict.add_translation("license", "Lizenz");
        dict.add_translation("marriage", "Ehe");
        dict.add_translation("divorce", "Scheidung");
        dict.add_translation("custody", "Sorgerecht");
        dict.add_translation("alimony", "Unterhalt");
        dict.add_translation("adoption", "Adoption");
        dict.add_translation("guardianship", "Vormundschaft");
        dict.add_translation("arbitration", "Schiedsverfahren");
        dict.add_translation("mediation", "Mediation");
        dict.add_translation("settlement", "Vergleich");
        dict.add_translation("litigation", "Rechtsstreit");
        dict.add_translation("precedent", "Präzedenzfall");
        dict.add_translation("statute_of_limitations", "Verjährung");
        dict
    }
    /// Creates a standard French legal dictionary.
    pub fn french() -> Self {
        let mut dict = Self::new(Locale::new("fr").with_country("FR"));
        dict.add_translation("statute", "loi");
        dict.add_translation("law", "droit");
        dict.add_translation("regulation", "règlement");
        dict.add_translation("contract", "contrat");
        dict.add_translation("agreement", "accord");
        dict.add_translation("liability", "responsabilité");
        dict.add_translation("obligation", "obligation");
        dict.add_translation("right", "droit");
        dict.add_translation("duty", "devoir");
        dict.add_translation("party", "partie");
        dict.add_translation("plaintiff", "demandeur");
        dict.add_translation("defendant", "défendeur");
        dict.add_translation("court", "tribunal");
        dict.add_translation("judge", "juge");
        dict.add_translation("jury", "jury");
        dict.add_translation("attorney", "avocat");
        dict.add_translation("lawyer", "avocat");
        dict.add_translation("counsel", "conseil");
        dict.add_translation("witness", "témoin");
        dict.add_translation("evidence", "preuve");
        dict.add_translation("testimony", "témoignage");
        dict.add_translation("verdict", "verdict");
        dict.add_translation("judgment", "jugement");
        dict.add_translation("appeal", "appel");
        dict.add_translation("damages", "dommages");
        dict.add_translation("penalty", "pénalité");
        dict.add_translation("fine", "amende");
        dict.add_translation("corporation", "société");
        dict.add_translation("shareholder", "actionnaire");
        dict.add_translation("director", "directeur");
        dict.add_translation("officer", "dirigeant");
        dict.add_translation("bylaws", "statuts");
        dict.add_translation("merger", "fusion");
        dict.add_translation("acquisition", "acquisition");
        dict.add_translation("dividend", "dividende");
        dict.add_translation("stock", "action");
        dict.add_translation("securities", "valeurs mobilières");
        dict.add_translation("property", "propriété");
        dict.add_translation("real_estate", "immobilier");
        dict.add_translation("ownership", "propriété");
        dict.add_translation("lease", "bail");
        dict.add_translation("tenant", "locataire");
        dict.add_translation("landlord", "bailleur");
        dict.add_translation("mortgage", "hypothèque");
        dict.add_translation("deed", "acte");
        dict.add_translation("title", "titre");
        dict.add_translation("easement", "servitude");
        dict.add_translation("crime", "crime");
        dict.add_translation("felony", "crime");
        dict.add_translation("misdemeanor", "délit");
        dict.add_translation("prosecution", "poursuite");
        dict.add_translation("indictment", "mise en accusation");
        dict.add_translation("conviction", "condamnation");
        dict.add_translation("sentence", "peine");
        dict.add_translation("probation", "sursis");
        dict.add_translation("parole", "libération conditionnelle");
        dict.add_translation("bail", "caution");
        dict.add_translation("jurisdiction", "compétence");
        dict.add_translation("venue", "lieu du procès");
        dict.add_translation("standing", "qualité pour agir");
        dict.add_translation("discovery", "communication de pièces");
        dict.add_translation("deposition", "déposition");
        dict.add_translation("motion", "requête");
        dict.add_translation("injunction", "injonction");
        dict.add_translation("subpoena", "assignation");
        dict.add_translation("hearing", "audience");
        dict.add_translation("trial", "procès");
        dict.add_translation("patent", "brevet");
        dict.add_translation("trademark", "marque");
        dict.add_translation("copyright", "droit d'auteur");
        dict.add_translation("infringement", "contrefaçon");
        dict.add_translation("royalty", "redevance");
        dict.add_translation("license", "licence");
        dict.add_translation("marriage", "mariage");
        dict.add_translation("divorce", "divorce");
        dict.add_translation("custody", "garde");
        dict.add_translation("alimony", "pension alimentaire");
        dict.add_translation("adoption", "adoption");
        dict.add_translation("guardianship", "tutelle");
        dict.add_translation("arbitration", "arbitrage");
        dict.add_translation("mediation", "médiation");
        dict.add_translation("settlement", "règlement");
        dict.add_translation("litigation", "litige");
        dict.add_translation("precedent", "précédent");
        dict.add_translation("statute_of_limitations", "prescription");
        dict
    }
    /// Creates a standard Spanish legal dictionary.
    pub fn spanish() -> Self {
        let mut dict = Self::new(Locale::new("es").with_country("ES"));
        dict.add_translation("statute", "estatuto");
        dict.add_translation("law", "ley");
        dict.add_translation("regulation", "reglamento");
        dict.add_translation("contract", "contrato");
        dict.add_translation("agreement", "acuerdo");
        dict.add_translation("liability", "responsabilidad");
        dict.add_translation("obligation", "obligación");
        dict.add_translation("right", "derecho");
        dict.add_translation("duty", "deber");
        dict.add_translation("party", "parte");
        dict.add_translation("plaintiff", "demandante");
        dict.add_translation("defendant", "demandado");
        dict.add_translation("court", "tribunal");
        dict.add_translation("judge", "juez");
        dict.add_translation("jury", "jurado");
        dict.add_translation("attorney", "abogado");
        dict.add_translation("lawyer", "abogado");
        dict.add_translation("counsel", "asesor");
        dict.add_translation("witness", "testigo");
        dict.add_translation("evidence", "prueba");
        dict.add_translation("testimony", "testimonio");
        dict.add_translation("verdict", "veredicto");
        dict.add_translation("judgment", "sentencia");
        dict.add_translation("appeal", "apelación");
        dict.add_translation("damages", "daños");
        dict.add_translation("penalty", "pena");
        dict.add_translation("fine", "multa");
        dict.add_translation("corporation", "corporación");
        dict.add_translation("shareholder", "accionista");
        dict.add_translation("director", "director");
        dict.add_translation("officer", "funcionario");
        dict.add_translation("bylaws", "estatutos");
        dict.add_translation("merger", "fusión");
        dict.add_translation("acquisition", "adquisición");
        dict.add_translation("dividend", "dividendo");
        dict.add_translation("stock", "acción");
        dict.add_translation("securities", "valores");
        dict.add_translation("property", "propiedad");
        dict.add_translation("real_estate", "bienes raíces");
        dict.add_translation("ownership", "propiedad");
        dict.add_translation("lease", "arrendamiento");
        dict.add_translation("tenant", "inquilino");
        dict.add_translation("landlord", "arrendador");
        dict.add_translation("mortgage", "hipoteca");
        dict.add_translation("deed", "escritura");
        dict.add_translation("title", "título");
        dict.add_translation("easement", "servidumbre");
        dict.add_translation("crime", "crimen");
        dict.add_translation("felony", "delito grave");
        dict.add_translation("misdemeanor", "delito menor");
        dict.add_translation("prosecution", "fiscalía");
        dict.add_translation("indictment", "acusación");
        dict.add_translation("conviction", "condena");
        dict.add_translation("sentence", "sentencia");
        dict.add_translation("probation", "libertad condicional");
        dict.add_translation("parole", "libertad condicional");
        dict.add_translation("bail", "fianza");
        dict.add_translation("jurisdiction", "jurisdicción");
        dict.add_translation("venue", "sede");
        dict.add_translation("standing", "legitimación");
        dict.add_translation("discovery", "descubrimiento");
        dict.add_translation("deposition", "declaración");
        dict.add_translation("motion", "moción");
        dict.add_translation("injunction", "mandamiento");
        dict.add_translation("subpoena", "citación");
        dict.add_translation("hearing", "audiencia");
        dict.add_translation("trial", "juicio");
        dict.add_translation("patent", "patente");
        dict.add_translation("trademark", "marca registrada");
        dict.add_translation("copyright", "derecho de autor");
        dict.add_translation("infringement", "infracción");
        dict.add_translation("royalty", "regalía");
        dict.add_translation("license", "licencia");
        dict.add_translation("marriage", "matrimonio");
        dict.add_translation("divorce", "divorcio");
        dict.add_translation("custody", "custodia");
        dict.add_translation("alimony", "pensión alimenticia");
        dict.add_translation("adoption", "adopción");
        dict.add_translation("guardianship", "tutela");
        dict.add_translation("arbitration", "arbitraje");
        dict.add_translation("mediation", "mediación");
        dict.add_translation("settlement", "acuerdo");
        dict.add_translation("litigation", "litigio");
        dict.add_translation("precedent", "precedente");
        dict.add_translation("statute_of_limitations", "prescripción");
        dict
    }
    /// Creates a standard Chinese (Simplified) legal dictionary.
    pub fn chinese_simplified() -> Self {
        let mut dict = Self::new(Locale::new("zh").with_country("CN"));
        dict.add_translation("statute", "法规");
        dict.add_translation("law", "法律");
        dict.add_translation("regulation", "规章");
        dict.add_translation("contract", "合同");
        dict.add_translation("agreement", "协议");
        dict.add_translation("liability", "责任");
        dict.add_translation("obligation", "义务");
        dict.add_translation("right", "权利");
        dict.add_translation("duty", "职责");
        dict.add_translation("party", "当事人");
        dict.add_translation("plaintiff", "原告");
        dict.add_translation("defendant", "被告");
        dict.add_translation("court", "法院");
        dict.add_translation("judge", "法官");
        dict.add_translation("jury", "陪审团");
        dict.add_translation("attorney", "律师");
        dict.add_translation("lawyer", "律师");
        dict.add_translation("counsel", "法律顾问");
        dict.add_translation("witness", "证人");
        dict.add_translation("evidence", "证据");
        dict.add_translation("testimony", "证词");
        dict.add_translation("verdict", "裁决");
        dict.add_translation("judgment", "判决");
        dict.add_translation("appeal", "上诉");
        dict.add_translation("damages", "损害赔偿");
        dict.add_translation("penalty", "处罚");
        dict.add_translation("fine", "罚款");
        dict.add_translation("corporation", "公司");
        dict.add_translation("shareholder", "股东");
        dict.add_translation("director", "董事");
        dict.add_translation("officer", "高管");
        dict.add_translation("bylaws", "章程");
        dict.add_translation("merger", "合并");
        dict.add_translation("acquisition", "收购");
        dict.add_translation("dividend", "股息");
        dict.add_translation("stock", "股票");
        dict.add_translation("securities", "证券");
        dict.add_translation("property", "财产");
        dict.add_translation("real_estate", "房地产");
        dict.add_translation("ownership", "所有权");
        dict.add_translation("lease", "租赁");
        dict.add_translation("tenant", "承租人");
        dict.add_translation("landlord", "出租人");
        dict.add_translation("mortgage", "抵押");
        dict.add_translation("deed", "契约");
        dict.add_translation("title", "产权");
        dict.add_translation("easement", "地役权");
        dict.add_translation("crime", "犯罪");
        dict.add_translation("felony", "重罪");
        dict.add_translation("misdemeanor", "轻罪");
        dict.add_translation("prosecution", "起诉");
        dict.add_translation("indictment", "起诉书");
        dict.add_translation("conviction", "定罪");
        dict.add_translation("sentence", "判刑");
        dict.add_translation("probation", "缓刑");
        dict.add_translation("parole", "假释");
        dict.add_translation("bail", "保释");
        dict.add_translation("jurisdiction", "管辖权");
        dict.add_translation("venue", "审判地");
        dict.add_translation("standing", "诉讼资格");
        dict.add_translation("discovery", "证据披露");
        dict.add_translation("deposition", "证词记录");
        dict.add_translation("motion", "动议");
        dict.add_translation("injunction", "禁令");
        dict.add_translation("subpoena", "传票");
        dict.add_translation("hearing", "听证");
        dict.add_translation("trial", "审判");
        dict.add_translation("patent", "专利");
        dict.add_translation("trademark", "商标");
        dict.add_translation("copyright", "版权");
        dict.add_translation("infringement", "侵权");
        dict.add_translation("royalty", "版税");
        dict.add_translation("license", "许可");
        dict.add_translation("marriage", "婚姻");
        dict.add_translation("divorce", "离婚");
        dict.add_translation("custody", "监护");
        dict.add_translation("alimony", "赡养费");
        dict.add_translation("adoption", "收养");
        dict.add_translation("guardianship", "监护权");
        dict.add_translation("arbitration", "仲裁");
        dict.add_translation("mediation", "调解");
        dict.add_translation("settlement", "和解");
        dict.add_translation("litigation", "诉讼");
        dict.add_translation("precedent", "判例");
        dict.add_translation("statute_of_limitations", "诉讼时效");
        dict
    }
    /// Creates a Latin legal terms dictionary.
    pub fn latin() -> Self {
        let mut dict = Self::new(Locale::new("la"));
        dict.add_translation("good_faith", "bona fide");
        dict.add_translation("by_the_fact_itself", "ipso facto");
        dict.add_translation("for_this_purpose", "ad hoc");
        dict.add_translation("in_good_faith", "bona fide");
        dict.add_translation("friend_of_the_court", "amicus curiae");
        dict.add_translation("body_of_the_crime", "corpus delicti");
        dict.add_translation("guilty_mind", "mens rea");
        dict.add_translation("guilty_act", "actus reus");
        dict.add_translation("you_have_the_body", "habeas corpus");
        dict.add_translation("let_the_buyer_beware", "caveat emptor");
        dict.add_translation("something_for_something", "quid pro quo");
        dict.add_translation("in_the_matter_of", "in re");
        dict.add_translation("by_operation_of_law", "ex lege");
        dict.add_translation("from_the_beginning", "ab initio");
        dict.add_translation("by_right", "de jure");
        dict.add_translation("in_fact", "de facto");
        dict.add_translation("according_to_law", "secundum legem");
        dict.add_translation("against_the_law", "contra legem");
        dict.add_translation("by_itself", "per se");
        dict.add_translation("burden_of_proof", "onus probandi");
        dict.add_translation("presumption_of_innocence", "praesumptio innocentiae");
        dict.add_translation("force_majeure", "vis major");
        dict.add_translation("highest_good_faith", "uberrima fides");
        dict.add_definition("bona fide", "In good faith; genuine");
        dict.add_definition("mens rea", "Guilty mind; criminal intent");
        dict.add_definition("actus reus", "Guilty act; physical element of a crime");
        dict.add_definition(
            "habeas corpus",
            "A writ requiring a person to be brought before a court",
        );
        dict.add_definition("caveat emptor", "Buyer beware; buyer assumes risk");
        dict.add_definition("quid pro quo", "Something for something; mutual exchange");
        dict
    }
    /// Creates a jurisdiction-specific glossary for Japan.
    pub fn glossary_japan() -> Self {
        let mut dict = Self::new(Locale::new("ja").with_country("JP"));
        dict.add_translation("civil_code", "民法");
        dict.add_translation("general_provisions", "総則");
        dict.add_translation("legal_person", "法人");
        dict.add_translation("juristic_act", "法律行為");
        dict.add_translation("prescription", "時効");
        dict.add_translation("acquisition_by_prescription", "取得時効");
        dict.add_translation("extinctive_prescription", "消滅時効");
        dict.add_translation("real_property", "不動産");
        dict.add_translation("movable_property", "動産");
        dict.add_translation("superficies", "地上権");
        dict.add_translation("emphyteusis", "永小作権");
        dict.add_translation("servitude", "地役権");
        dict.add_translation("family_register", "戸籍");
        dict.add_translation("koseki", "戸籍");
        dict.add_translation("parental_authority", "親権");
        dict.add_translation("kabushiki_kaisha", "株式会社");
        dict.add_translation("godo_kaisha", "合同会社");
        dict.add_translation("yugen_kaisha", "有限会社");
        dict.add_translation("penal_code", "刑法");
        dict.add_translation("suspended_sentence", "執行猶予");
        dict
    }
    /// Creates a jurisdiction-specific glossary for United States.
    pub fn glossary_united_states() -> Self {
        let mut dict = Self::new(Locale::new("en").with_country("US"));
        dict.add_translation("constitution", "Constitution");
        dict.add_translation("bill_of_rights", "Bill of Rights");
        dict.add_translation("due_process", "due process");
        dict.add_translation("equal_protection", "equal protection");
        dict.add_translation("commerce_clause", "Commerce Clause");
        dict.add_translation("federal", "federal");
        dict.add_translation("state", "state");
        dict.add_translation("supremacy_clause", "Supremacy Clause");
        dict.add_translation("supreme_court", "Supreme Court");
        dict.add_translation("circuit_court", "Circuit Court");
        dict.add_translation("district_court", "District Court");
        dict.add_translation("stare_decisis", "stare decisis");
        dict.add_translation("precedent", "precedent");
        dict.add_translation("case_law", "case law");
        dict.add_translation("punitive_damages", "punitive damages");
        dict.add_translation("treble_damages", "treble damages");
        dict.add_translation("strict_liability", "strict liability");
        dict.add_translation("discovery", "discovery");
        dict.add_translation("deposition", "deposition");
        dict.add_translation("summary_judgment", "summary judgment");
        dict.add_translation("class_action", "class action");
        dict
    }
    /// Creates a jurisdiction-specific glossary for United Kingdom.
    pub fn glossary_united_kingdom() -> Self {
        let mut dict = Self::new(Locale::new("en").with_country("GB"));
        dict.add_translation("high_court", "High Court");
        dict.add_translation("crown_court", "Crown Court");
        dict.add_translation("magistrates_court", "Magistrates' Court");
        dict.add_translation("supreme_court", "Supreme Court");
        dict.add_translation("barrister", "barrister");
        dict.add_translation("solicitor", "solicitor");
        dict.add_translation("queens_counsel", "Queen's Counsel");
        dict.add_translation("kings_counsel", "King's Counsel");
        dict.add_translation("freehold", "freehold");
        dict.add_translation("leasehold", "leasehold");
        dict.add_translation("commonhold", "commonhold");
        dict.add_translation("equity", "equity");
        dict.add_translation("trust", "trust");
        dict.add_translation("trustee", "trustee");
        dict.add_translation("beneficiary", "beneficiary");
        dict.add_translation("act_of_parliament", "Act of Parliament");
        dict.add_translation("statutory_instrument", "statutory instrument");
        dict
    }
    /// Creates a jurisdiction-specific glossary for Germany.
    pub fn glossary_germany() -> Self {
        let mut dict = Self::new(Locale::new("de").with_country("DE"));
        dict.add_translation("burgerliches_gesetzbuch", "Bürgerliches Gesetzbuch");
        dict.add_translation("bgb", "BGB");
        dict.add_translation("schuldrecht", "Schuldrecht");
        dict.add_translation("sachenrecht", "Sachenrecht");
        dict.add_translation("familienrecht", "Familienrecht");
        dict.add_translation("erbrecht", "Erbrecht");
        dict.add_translation("bundesverfassungsgericht", "Bundesverfassungsgericht");
        dict.add_translation("bundesgerichtshof", "Bundesgerichtshof");
        dict.add_translation("oberlandesgericht", "Oberlandesgericht");
        dict.add_translation("landgericht", "Landgericht");
        dict.add_translation("amtsgericht", "Amtsgericht");
        dict.add_translation("rechtsstaat", "Rechtsstaat");
        dict.add_translation("grundgesetz", "Grundgesetz");
        dict
    }
    /// Creates a jurisdiction-specific glossary for France.
    pub fn glossary_france() -> Self {
        let mut dict = Self::new(Locale::new("fr").with_country("FR"));
        dict.add_translation("code_civil", "Code civil");
        dict.add_translation("code_penal", "Code pénal");
        dict.add_translation("cour_de_cassation", "Cour de cassation");
        dict.add_translation("cour_dappel", "Cour d'appel");
        dict.add_translation("tribunal_de_grande_instance", "Tribunal de grande instance");
        dict.add_translation("droit_civil", "droit civil");
        dict.add_translation("droit_penal", "droit pénal");
        dict.add_translation("droit_administratif", "droit administratif");
        dict
    }
    /// Creates a jurisdiction-specific glossary for China.
    pub fn glossary_china() -> Self {
        let mut dict = Self::new(Locale::new("zh").with_country("CN"));
        dict.add_translation("civil_law", "民法");
        dict.add_translation("criminal_law", "刑法");
        dict.add_translation("administrative_law", "行政法");
        dict.add_translation("peoples_court", "人民法院");
        dict.add_translation("supreme_peoples_court", "最高人民法院");
        dict.add_translation("procuratorate", "检察院");
        dict
    }
    /// Creates a jurisdiction-specific glossary for a jurisdiction code.
    pub fn glossary_for_jurisdiction(code: &str) -> Self {
        match code {
            "JP" => Self::glossary_japan(),
            "US" => Self::glossary_united_states(),
            "GB" => Self::glossary_united_kingdom(),
            "DE" => Self::glossary_germany(),
            "FR" => Self::glossary_france(),
            "CN" => Self::glossary_china(),
            _ => {
                let locale = Locale::new("en");
                Self::new(locale)
            }
        }
    }
}
impl LegalDictionary {
    /// Builds a term index for fast prefix-based lookups.
    /// Useful for autocomplete and fuzzy search features.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{LegalDictionary, Locale};
    ///
    /// let mut dict = LegalDictionary::new(Locale::new("en"));
    /// dict.add_translation("contract", "contract");
    /// dict.add_translation("contractor", "contractor");
    /// dict.add_translation("copyright", "copyright");
    ///
    /// let index = dict.build_term_index();
    /// let matches = index.find_by_prefix("contr");
    /// assert!(matches.len() >= 2);
    /// ```
    pub fn build_term_index(&self) -> TermIndex {
        let mut index = TermIndex::new();
        for key in self.translations.keys() {
            index.index_term(key);
        }
        for abbr in self.abbreviations.keys() {
            index.index_term(abbr);
        }
        index
    }
}
/// Dialect variation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DialectType {
    /// Regional dialect.
    Regional,
    /// Social dialect (sociolect).
    Social,
    /// Occupational/professional dialect.
    Occupational,
    /// Historical/archaic form.
    Historical,
}
/// Sign language type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignLanguageType {
    /// American Sign Language
    ASL,
    /// British Sign Language
    BSL,
    /// Japanese Sign Language
    JSL,
    /// International Sign
    IS,
    /// Other sign language
    Other,
}
/// Court proceeding participant with language preference.
#[derive(Debug, Clone)]
pub struct CourtParticipant {
    /// Participant name.
    pub name: String,
    /// Role in the proceeding.
    pub role: CourtParticipantRole,
    /// Primary language locale.
    pub primary_language: Locale,
    /// Whether interpretation is required.
    pub requires_interpretation: bool,
}
impl CourtParticipant {
    /// Creates a new court participant.
    pub fn new(
        name: impl Into<String>,
        role: CourtParticipantRole,
        primary_language: Locale,
    ) -> Self {
        Self {
            name: name.into(),
            role,
            primary_language,
            requires_interpretation: false,
        }
    }
    /// Marks this participant as requiring interpretation.
    pub fn requires_interpretation(mut self) -> Self {
        self.requires_interpretation = true;
        self
    }
}
/// Name formatter for legal documents following cultural conventions.
#[derive(Debug, Clone)]
pub struct NameFormatter {
    locale: Locale,
    order: NameOrder,
}
impl NameFormatter {
    /// Creates a new name formatter for a locale.
    pub fn new(locale: Locale) -> Self {
        let order = Self::detect_name_order(&locale);
        Self { locale, order }
    }
    /// Detects the name order convention from locale.
    pub fn detect_name_order(locale: &Locale) -> NameOrder {
        match locale.language.as_str() {
            "ja" | "ko" | "zh" | "vi" | "hu" => NameOrder::FamilyFirst,
            _ => NameOrder::GivenFirst,
        }
    }
    /// Formats a full name according to cultural conventions.
    pub fn format_full_name(&self, name: &PersonName) -> String {
        match self.locale.language.as_str() {
            "ja" => self.format_japanese(name),
            "ko" => self.format_korean(name),
            "zh" => self.format_chinese(name),
            "ar" => self.format_arabic(name),
            "ru" => self.format_russian(name),
            _ => self.format_western(name),
        }
    }
    /// Formats a name in Western style (Given Middle Family).
    fn format_western(&self, name: &PersonName) -> String {
        let mut parts = Vec::new();
        if let Some(prefix) = &name.prefix {
            parts.push(prefix.clone());
        }
        parts.push(name.given_name.clone());
        if let Some(middle) = &name.middle_name {
            parts.push(middle.clone());
        }
        parts.push(name.family_name.clone());
        if let Some(suffix) = &name.suffix {
            parts.push(suffix.clone());
        }
        parts.join(" ")
    }
    /// Formats a name in Japanese style (Family Given).
    fn format_japanese(&self, name: &PersonName) -> String {
        let mut parts = Vec::new();
        if let Some(prefix) = &name.prefix {
            parts.push(prefix.clone());
        }
        parts.push(name.family_name.clone());
        parts.push(name.given_name.clone());
        parts.join(" ")
    }
    /// Formats a name in Korean style (Family Given).
    fn format_korean(&self, name: &PersonName) -> String {
        format!("{}{}", name.family_name, name.given_name)
    }
    /// Formats a name in Chinese style (Family Given).
    fn format_chinese(&self, name: &PersonName) -> String {
        match self.locale.country.as_deref() {
            Some("CN") | Some("SG") => format!("{}{}", name.family_name, name.given_name),
            Some("TW") | Some("HK") => {
                format!("{} {}", name.family_name, name.given_name)
            }
            _ => format!("{}{}", name.family_name, name.given_name),
        }
    }
    /// Formats a name in Arabic style (with patronymic).
    fn format_arabic(&self, name: &PersonName) -> String {
        let mut parts = Vec::new();
        parts.push(name.given_name.clone());
        if let Some(patronymic) = &name.patronymic {
            parts.push(patronymic.clone());
        }
        parts.push(name.family_name.clone());
        parts.join(" ")
    }
    /// Formats a name in Russian style (with patronymic).
    fn format_russian(&self, name: &PersonName) -> String {
        let mut parts = Vec::new();
        parts.push(name.family_name.clone());
        parts.push(name.given_name.clone());
        if let Some(patronymic) = &name.patronymic {
            parts.push(patronymic.clone());
        }
        parts.join(" ")
    }
    /// Formats a name for legal citations (typically Family, Given Middle).
    pub fn format_citation(&self, name: &PersonName) -> String {
        let mut result = name.family_name.clone();
        result.push_str(", ");
        result.push_str(&name.given_name);
        if let Some(middle) = &name.middle_name {
            result.push(' ');
            result.push_str(middle);
        }
        result
    }
    /// Formats initials (e.g., J. K. for John Kevin).
    pub fn format_initials(&self, name: &PersonName) -> String {
        let given_initial = name.given_name.chars().next().unwrap_or('X');
        let family_initial = name.family_name.chars().next().unwrap_or('X');
        match self.order {
            NameOrder::GivenFirst => {
                if let Some(middle) = &name.middle_name {
                    let middle_initial = middle.chars().next().unwrap_or('X');
                    format!("{}. {}. {}.", given_initial, middle_initial, family_initial)
                } else {
                    format!("{}. {}.", given_initial, family_initial)
                }
            }
            NameOrder::FamilyFirst => format!("{}. {}.", family_initial, given_initial),
        }
    }
    /// Formats a formal name with all components.
    pub fn format_formal(&self, name: &PersonName) -> String {
        let full_name = self.format_full_name(name);
        if let Some(prefix) = &name.prefix {
            format!("{} {}", prefix, full_name)
        } else {
            full_name
        }
    }
}
/// Audio description generator for legal documents.
/// Generates descriptive text for charts, diagrams, and complex structures.
#[derive(Debug)]
pub struct AudioDescriptionGenerator {
    #[allow(dead_code)]
    locale: Locale,
}
impl AudioDescriptionGenerator {
    /// Creates a new audio description generator.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }
    /// Generates alt text for a legal diagram.
    pub fn describe_diagram(&self, diagram_type: &str, elements: &[&str]) -> String {
        match diagram_type {
            "flowchart" => {
                format!(
                    "Flowchart showing legal process with {} steps: {}",
                    elements.len(),
                    elements.join(", then ")
                )
            }
            "hierarchy" => {
                format!(
                    "Organizational hierarchy showing {} levels: {}",
                    elements.len(),
                    elements.join(", reporting to ")
                )
            }
            "timeline" => {
                format!(
                    "Timeline of events with {} milestones: {}",
                    elements.len(),
                    elements.join(", followed by ")
                )
            }
            _ => {
                format!(
                    "Diagram of type {} with {} elements: {}",
                    diagram_type,
                    elements.len(),
                    elements.join(", ")
                )
            }
        }
    }
    /// Generates description for a statistical chart.
    pub fn describe_chart(&self, chart_type: &str, data_points: &[(String, f32)]) -> String {
        match chart_type {
            "bar" | "column" => {
                let mut desc = format!("Bar chart showing {} data points. ", data_points.len());
                for (label, value) in data_points {
                    desc.push_str(&format!("{}: {:.1}. ", label, value));
                }
                desc
            }
            "pie" => {
                let total: f32 = data_points.iter().map(|(_, v)| v).sum();
                let mut desc = format!("Pie chart with {} segments. ", data_points.len());
                for (label, value) in data_points {
                    let percentage = (value / total) * 100.0;
                    desc.push_str(&format!("{}: {:.1}%. ", label, percentage));
                }
                desc
            }
            "line" => {
                let mut desc = format!(
                    "Line chart with {} data points showing trend over time. ",
                    data_points.len()
                );
                if data_points.len() >= 2 {
                    let first = data_points
                        .first()
                        .expect("invariant: data_points.len() >= 2");
                    let last = data_points
                        .last()
                        .expect("invariant: data_points.len() >= 2");
                    desc.push_str(&format!(
                        "Starting at {} ({:.1}), ending at {} ({:.1}).",
                        first.0, first.1, last.0, last.1
                    ));
                }
                desc
            }
            _ => {
                format!(
                    "Chart of type {} with {} data points",
                    chart_type,
                    data_points.len()
                )
            }
        }
    }
    /// Generates description for a table.
    pub fn describe_table(&self, caption: &str, rows: usize, cols: usize) -> String {
        format!(
            "Table titled '{}' with {} rows and {} columns. Use table navigation commands to explore the data.",
            caption, rows, cols
        )
    }
}
/// Risk level for legal documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}
/// SSML (Speech Synthesis Markup Language) tag type.
#[derive(Debug, Clone, PartialEq)]
pub enum SSMLTag {
    /// Pause/break
    Break { duration_ms: u32 },
    /// Emphasis
    Emphasis { level: EmphasisLevel },
    /// Prosody (rate, pitch, volume)
    Prosody { rate: f32, pitch: f32, volume: f32 },
    /// Say-as (interpret as specific type)
    SayAs { interpret_as: String },
    /// Phoneme (pronunciation)
    Phoneme { ph: String, alphabet: String },
}
/// IETF BCP 47 language tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BCP47LanguageTag {
    /// Language subtag (e.g., "en", "ja").
    pub language: String,
    /// Script subtag (e.g., "Latn", "Jpan").
    pub script: Option<String>,
    /// Region subtag (e.g., "US", "JP").
    pub region: Option<String>,
    /// Variant subtags.
    pub variants: Vec<String>,
    /// Extension subtags (e.g., "u-ca-japanese").
    pub extensions: Vec<String>,
    /// Private use subtags.
    pub private_use: Vec<String>,
}
impl BCP47LanguageTag {
    /// Creates a new BCP 47 language tag.
    pub fn new(language: &str) -> Self {
        Self {
            language: language.to_lowercase(),
            script: None,
            region: None,
            variants: Vec::new(),
            extensions: Vec::new(),
            private_use: Vec::new(),
        }
    }
    /// Sets the script subtag.
    pub fn with_script(mut self, script: &str) -> Self {
        self.script = Some(
            script
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect(),
        );
        self
    }
    /// Sets the region subtag.
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_uppercase());
        self
    }
    /// Adds a variant subtag.
    pub fn add_variant(mut self, variant: &str) -> Self {
        self.variants.push(variant.to_lowercase());
        self
    }
    /// Adds an extension subtag.
    pub fn add_extension(mut self, extension: &str) -> Self {
        self.extensions.push(extension.to_lowercase());
        self
    }
    /// Adds a private use subtag.
    pub fn add_private_use(mut self, private: &str) -> Self {
        self.private_use.push(private.to_lowercase());
        self
    }
    /// Formats the tag as a BCP 47 string.
    pub(crate) fn format_tag(&self) -> String {
        let mut parts = vec![self.language.clone()];
        if let Some(ref script) = self.script {
            parts.push(script.clone());
        }
        if let Some(ref region) = self.region {
            parts.push(region.clone());
        }
        parts.extend(self.variants.clone());
        parts.extend(self.extensions.clone());
        if !self.private_use.is_empty() {
            parts.push("x".to_string());
            parts.extend(self.private_use.clone());
        }
        parts.join("-")
    }
    /// Parses a BCP 47 language tag from a string.
    pub fn parse(tag: &str) -> Result<Self, String> {
        let parts: Vec<&str> = tag.split('-').collect();
        if parts.is_empty() {
            return Err("Empty language tag".to_string());
        }
        let language = parts[0].to_lowercase();
        if language.len() < 2 || language.len() > 3 {
            return Err(format!("Invalid language subtag: {}", language));
        }
        let mut bcp47 = Self::new(&language);
        let mut i = 1;
        if i < parts.len() && parts[i].len() == 4 {
            bcp47 = bcp47.with_script(parts[i]);
            i += 1;
        }
        if i < parts.len() && (parts[i].len() == 2 || parts[i].len() == 3) {
            bcp47 = bcp47.with_region(parts[i]);
            i += 1;
        }
        while i < parts.len() {
            if parts[i] == "x" {
                i += 1;
                while i < parts.len() {
                    bcp47 = bcp47.add_private_use(parts[i]);
                    i += 1;
                }
                break;
            } else if parts[i].len() == 1 {
                let ext_type = parts[i];
                i += 1;
                while i < parts.len() && parts[i].len() > 1 && parts[i] != "x" {
                    bcp47 = bcp47.add_extension(&format!("{}-{}", ext_type, parts[i]));
                    i += 1;
                }
            } else {
                bcp47 = bcp47.add_variant(parts[i]);
                i += 1;
            }
        }
        Ok(bcp47)
    }
    /// Converts to a Locale.
    pub fn to_locale(&self) -> Locale {
        let mut locale = Locale::new(&self.language);
        if let Some(ref script) = self.script {
            locale = locale.with_script(script);
        }
        if let Some(ref region) = self.region {
            locale = locale.with_country(region);
        }
        locale
    }
    /// Creates a BCP 47 tag from a Locale.
    pub fn from_locale(locale: &Locale) -> Self {
        let mut tag = Self::new(&locale.language);
        if let Some(ref script) = locale.script {
            tag = tag.with_script(script);
        }
        if let Some(ref country) = locale.country {
            tag = tag.with_region(country);
        }
        tag
    }
    /// Validates the BCP 47 tag.
    pub fn is_valid(&self) -> bool {
        if self.language.len() < 2 || self.language.len() > 3 {
            return false;
        }
        if let Some(ref script) = self.script
            && script.len() != 4
        {
            return false;
        }
        if let Some(ref region) = self.region
            && (region.len() < 2 || region.len() > 3)
        {
            return false;
        }
        true
    }
}
/// Party identifier for legal documents.
#[derive(Debug, Default)]
pub struct PartyIdentifier {
    /// Patterns for identifying parties
    pub(super) patterns: Vec<String>,
}
impl PartyIdentifier {
    /// Creates a new party identifier.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a party identifier with default patterns.
    pub fn with_defaults() -> Self {
        let mut identifier = Self::new();
        identifier.add_pattern("party of the first part");
        identifier.add_pattern("party of the second part");
        identifier.add_pattern("hereinafter referred to as");
        identifier.add_pattern("plaintiff");
        identifier.add_pattern("defendant");
        identifier.add_pattern("between");
        identifier.add_pattern("and");
        identifier
    }
    /// Adds a pattern for identifying parties.
    pub fn add_pattern(&mut self, pattern: impl Into<String>) {
        self.patterns.push(pattern.into());
    }
    /// Identifies parties in document text.
    pub fn identify(&self, text: &str) -> Vec<IdentifiedParty> {
        let mut parties = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            if line_lower.contains("between") || line_lower.contains("party") {
                let words: Vec<&str> = line.split_whitespace().collect();
                for (j, word) in words.iter().enumerate() {
                    if word.len() > 2
                        && word
                            .chars()
                            .next()
                            .expect("invariant: word.len() > 2")
                            .is_uppercase()
                    {
                        let mut name_parts = vec![*word];
                        for next_word in words.iter().skip(j + 1) {
                            if next_word.len() > 1
                                && next_word
                                    .chars()
                                    .next()
                                    .expect("invariant: next_word.len() > 1")
                                    .is_uppercase()
                            {
                                name_parts.push(*next_word);
                            } else {
                                break;
                            }
                        }
                        if name_parts.len() >= 2
                            || (name_parts.len() == 1 && name_parts[0].len() > 3)
                        {
                            let name = name_parts.join(" ");
                            let role = if line_lower.contains("first part") {
                                PartyRole::FirstParty
                            } else if line_lower.contains("second part") {
                                PartyRole::SecondParty
                            } else if line_lower.contains("plaintiff") {
                                PartyRole::Plaintiff
                            } else if line_lower.contains("defendant") {
                                PartyRole::Defendant
                            } else {
                                PartyRole::Unknown
                            };
                            parties.push(IdentifiedParty {
                                name: name
                                    .trim_matches(|c: char| !c.is_alphanumeric() && c != ' ')
                                    .to_string(),
                                role,
                                position: i * 100,
                                confidence: 0.7,
                            });
                        }
                    }
                }
            }
        }
        parties
    }
}
/// Personal name components for legal documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonName {
    /// Given name (first name in Western cultures)
    pub given_name: String,
    /// Family name (last name in Western cultures, first in East Asian)
    pub family_name: String,
    /// Middle name(s) if applicable
    pub middle_name: Option<String>,
    /// Honorific prefix (Mr., Dr., etc.)
    pub prefix: Option<String>,
    /// Suffix (Jr., Sr., III, etc.)
    pub suffix: Option<String>,
    /// Patronymic or matronymic (e.g., Russian, Arabic)
    pub patronymic: Option<String>,
}
impl PersonName {
    /// Creates a new person name with given and family names.
    pub fn new(given_name: impl Into<String>, family_name: impl Into<String>) -> Self {
        Self {
            given_name: given_name.into(),
            family_name: family_name.into(),
            middle_name: None,
            prefix: None,
            suffix: None,
            patronymic: None,
        }
    }
    /// Sets the middle name.
    pub fn with_middle_name(mut self, middle_name: impl Into<String>) -> Self {
        self.middle_name = Some(middle_name.into());
        self
    }
    /// Sets the prefix.
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
    /// Sets the suffix.
    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
    /// Sets the patronymic.
    pub fn with_patronymic(mut self, patronymic: impl Into<String>) -> Self {
        self.patronymic = Some(patronymic.into());
        self
    }
}
/// ICU-style message formatter.
#[derive(Debug, Clone)]
pub struct MessageFormatter {
    #[allow(dead_code)]
    locale: Locale,
    plural_rules: PluralRules,
}
impl MessageFormatter {
    /// Creates a new message formatter.
    pub fn new(locale: Locale) -> Self {
        let plural_rules = PluralRules::new(locale.clone());
        Self {
            locale,
            plural_rules,
        }
    }
    /// Formats a message with variables.
    /// Simple implementation supporting {variable} placeholders.
    pub fn format(&self, pattern: &str, args: &HashMap<String, String>) -> String {
        let mut result = pattern.to_string();
        for (key, value) in args {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }
    /// Formats a plural message.
    /// Pattern format: "{count} {count, plural, one {item} other {items}}"
    pub fn format_plural(&self, count: i64, one: &str, other: &str) -> String {
        let category = self.plural_rules.category(count);
        match category {
            PluralCategory::One => one.to_string(),
            _ => other.to_string(),
        }
    }
    /// Formats a complex plural message with multiple categories.
    pub fn format_plural_complex(
        &self,
        count: i64,
        patterns: &HashMap<PluralCategory, String>,
    ) -> Option<String> {
        let category = self.plural_rules.category(count);
        patterns
            .get(&category)
            .or_else(|| patterns.get(&PluralCategory::Other))
            .cloned()
    }
}
/// EU regulation term with aligned translations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EURegulationTerm {
    /// Regulation type.
    pub regulation: EURegulationType,
    /// Canonical English term.
    pub canonical_term: String,
    /// Aligned translations in EU official languages.
    pub translations: HashMap<String, String>,
    /// Article or section reference.
    pub article_ref: Option<String>,
    /// Definition in English.
    pub definition: String,
}
impl EURegulationTerm {
    /// Creates a new EU regulation term.
    pub fn new(
        regulation: EURegulationType,
        canonical_term: impl Into<String>,
        definition: impl Into<String>,
    ) -> Self {
        Self {
            regulation,
            canonical_term: canonical_term.into(),
            translations: HashMap::new(),
            article_ref: None,
            definition: definition.into(),
        }
    }
    /// Adds a translation for a specific EU language.
    pub fn add_translation(
        mut self,
        language_code: impl Into<String>,
        term: impl Into<String>,
    ) -> Self {
        self.translations.insert(language_code.into(), term.into());
        self
    }
    /// Sets the article reference.
    pub fn with_article(mut self, article: impl Into<String>) -> Self {
        self.article_ref = Some(article.into());
        self
    }
    /// Gets the translation for a specific language.
    pub fn get_translation(&self, language_code: &str) -> Option<&String> {
        self.translations.get(language_code)
    }
}
/// Jurisdiction definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// Unique identifier (ISO 3166-1 alpha-2 or custom)
    pub id: String,
    /// Display name
    pub name: String,
    /// Primary locale
    pub locale: Locale,
    /// Legal system type
    pub legal_system: LegalSystem,
    /// Parent jurisdiction (for federated systems)
    pub parent: Option<String>,
    /// Cultural parameters affecting law interpretation
    pub cultural_params: CulturalParams,
}
impl Jurisdiction {
    /// Creates a new jurisdiction.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{Jurisdiction, Locale, LegalSystem};
    ///
    /// let locale = Locale::new("ja").with_country("JP");
    /// let jurisdiction = Jurisdiction::new("JP", "Japan", locale)
    ///     .with_legal_system(LegalSystem::CivilLaw);
    ///
    /// assert_eq!(jurisdiction.id, "JP");
    /// assert_eq!(jurisdiction.name, "Japan");
    /// assert_eq!(jurisdiction.legal_system, LegalSystem::CivilLaw);
    /// ```
    pub fn new(id: impl Into<String>, name: impl Into<String>, locale: Locale) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            locale,
            legal_system: LegalSystem::CivilLaw,
            parent: None,
            cultural_params: CulturalParams::default(),
        }
    }
    /// Sets the legal system.
    pub fn with_legal_system(mut self, system: LegalSystem) -> Self {
        self.legal_system = system;
        self
    }
    /// Sets cultural parameters.
    pub fn with_cultural_params(mut self, params: CulturalParams) -> Self {
        self.cultural_params = params;
        self
    }
}
