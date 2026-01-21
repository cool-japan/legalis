//! Constitution of India Types
//!
//! Types for Constitutional law of India, the world's longest written constitution.
//! Constitution of India came into effect on 26th January 1950.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Parts of the Constitution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutionPart {
    /// Part I - The Union and its Territory (Articles 1-4)
    UnionTerritory,
    /// Part II - Citizenship (Articles 5-11)
    Citizenship,
    /// Part III - Fundamental Rights (Articles 12-35)
    FundamentalRights,
    /// Part IV - Directive Principles (Articles 36-51)
    DirectivePrinciples,
    /// Part IVA - Fundamental Duties (Article 51A)
    FundamentalDuties,
    /// Part V - The Union (Articles 52-151)
    TheUnion,
    /// Part VI - The States (Articles 152-237)
    TheStates,
    /// Part VII - States in Part B (Repealed)
    PartBStates,
    /// Part VIII - Union Territories (Articles 239-242)
    UnionTerritories,
    /// Part IX - Panchayats (Articles 243-243O)
    Panchayats,
    /// Part IXA - Municipalities (Articles 243P-243ZG)
    Municipalities,
    /// Part IXB - Co-operative Societies (Articles 243ZH-243ZT)
    CooperativeSocieties,
    /// Part X - Scheduled/Tribal Areas (Articles 244-244A)
    ScheduledTribalAreas,
    /// Part XI - Relations (Articles 245-263)
    CentreStateRelations,
    /// Part XII - Finance, Property (Articles 264-300A)
    FinanceProperty,
    /// Part XIII - Trade, Commerce (Articles 301-307)
    TradeCommerce,
    /// Part XIV - Services (Articles 308-323)
    PublicServices,
    /// Part XIVA - Tribunals (Articles 323A-323B)
    Tribunals,
    /// Part XV - Elections (Articles 324-329A)
    Elections,
    /// Part XVI - Special Provisions (Articles 330-342A)
    SpecialProvisions,
    /// Part XVII - Official Language (Articles 343-351)
    OfficialLanguage,
    /// Part XVIII - Emergency (Articles 352-360)
    Emergency,
    /// Part XIX - Miscellaneous (Articles 361-367)
    Miscellaneous,
    /// Part XX - Amendment (Article 368)
    Amendment,
    /// Part XXI - Temporary Provisions (Articles 369-392)
    TemporaryProvisions,
    /// Part XXII - Short Title (Articles 393-395)
    ShortTitle,
}

impl ConstitutionPart {
    /// Get article range for part
    pub fn article_range(&self) -> (u32, u32) {
        match self {
            Self::UnionTerritory => (1, 4),
            Self::Citizenship => (5, 11),
            Self::FundamentalRights => (12, 35),
            Self::DirectivePrinciples => (36, 51),
            Self::FundamentalDuties => (51, 51), // 51A
            Self::TheUnion => (52, 151),
            Self::TheStates => (152, 237),
            Self::PartBStates => (0, 0), // Repealed
            Self::UnionTerritories => (239, 242),
            Self::Panchayats => (243, 243),
            Self::Municipalities => (243, 243),
            Self::CooperativeSocieties => (243, 243),
            Self::ScheduledTribalAreas => (244, 244),
            Self::CentreStateRelations => (245, 263),
            Self::FinanceProperty => (264, 300),
            Self::TradeCommerce => (301, 307),
            Self::PublicServices => (308, 323),
            Self::Tribunals => (323, 323),
            Self::Elections => (324, 329),
            Self::SpecialProvisions => (330, 342),
            Self::OfficialLanguage => (343, 351),
            Self::Emergency => (352, 360),
            Self::Miscellaneous => (361, 367),
            Self::Amendment => (368, 368),
            Self::TemporaryProvisions => (369, 392),
            Self::ShortTitle => (393, 395),
        }
    }
}

/// Fundamental Rights (Part III)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FundamentalRight {
    /// Article 14 - Equality before law
    EqualityBeforeLaw,
    /// Article 15 - Prohibition of discrimination
    NonDiscrimination,
    /// Article 16 - Equality of opportunity
    EqualOpportunity,
    /// Article 17 - Abolition of untouchability
    AbolitionOfUntouchability,
    /// Article 18 - Abolition of titles
    AbolitionOfTitles,
    /// Article 19 - Freedom of speech, assembly, etc.
    SixFreedoms,
    /// Article 20 - Protection against conviction
    ProtectionAgainstConviction,
    /// Article 21 - Right to life and personal liberty
    RightToLife,
    /// Article 21A - Right to education
    RightToEducation,
    /// Article 22 - Protection against arrest
    ProtectionAgainstArrest,
    /// Article 23 - Prohibition of trafficking
    ProhibitionOfTrafficking,
    /// Article 24 - Prohibition of child labour
    ProhibitionOfChildLabour,
    /// Article 25 - Freedom of religion
    FreedomOfReligion,
    /// Article 26 - Freedom to manage religious affairs
    ReligiousAffairs,
    /// Article 27 - Freedom from religious tax
    ReligiousTax,
    /// Article 28 - Freedom in religious instruction
    ReligiousInstruction,
    /// Article 29 - Protection of cultural interests
    CulturalProtection,
    /// Article 30 - Right of minorities to establish institutions
    MinorityInstitutions,
    /// Article 32 - Right to Constitutional remedies
    ConstitutionalRemedies,
}

impl FundamentalRight {
    /// Get article number
    pub fn article(&self) -> &'static str {
        match self {
            Self::EqualityBeforeLaw => "14",
            Self::NonDiscrimination => "15",
            Self::EqualOpportunity => "16",
            Self::AbolitionOfUntouchability => "17",
            Self::AbolitionOfTitles => "18",
            Self::SixFreedoms => "19",
            Self::ProtectionAgainstConviction => "20",
            Self::RightToLife => "21",
            Self::RightToEducation => "21A",
            Self::ProtectionAgainstArrest => "22",
            Self::ProhibitionOfTrafficking => "23",
            Self::ProhibitionOfChildLabour => "24",
            Self::FreedomOfReligion => "25",
            Self::ReligiousAffairs => "26",
            Self::ReligiousTax => "27",
            Self::ReligiousInstruction => "28",
            Self::CulturalProtection => "29",
            Self::MinorityInstitutions => "30",
            Self::ConstitutionalRemedies => "32",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::EqualityBeforeLaw => "Equality before law and equal protection of laws",
            Self::NonDiscrimination => {
                "Prohibition of discrimination on grounds of religion, race, caste, sex, place of birth"
            }
            Self::EqualOpportunity => "Equality of opportunity in public employment",
            Self::AbolitionOfUntouchability => "Abolition of untouchability",
            Self::AbolitionOfTitles => "Abolition of titles",
            Self::SixFreedoms => {
                "Protection of six freedoms: speech, assembly, association, movement, residence, profession"
            }
            Self::ProtectionAgainstConviction => {
                "Protection against ex post facto laws, double jeopardy, self-incrimination"
            }
            Self::RightToLife => "Protection of life and personal liberty",
            Self::RightToEducation => {
                "Right to free and compulsory education for children 6-14 years"
            }
            Self::ProtectionAgainstArrest => "Protection against arrest and detention",
            Self::ProhibitionOfTrafficking => {
                "Prohibition of traffic in human beings and forced labour"
            }
            Self::ProhibitionOfChildLabour => {
                "Prohibition of employment of children in factories, mines, etc."
            }
            Self::FreedomOfReligion => {
                "Freedom of conscience and free profession, practice and propagation of religion"
            }
            Self::ReligiousAffairs => "Freedom to manage religious affairs",
            Self::ReligiousTax => "Freedom from payment of taxes for religious purposes",
            Self::ReligiousInstruction => {
                "Freedom from religious instruction in educational institutions"
            }
            Self::CulturalProtection => "Protection of interests of minorities",
            Self::MinorityInstitutions => {
                "Right of minorities to establish and administer educational institutions"
            }
            Self::ConstitutionalRemedies => "Right to move Supreme Court for enforcement of rights",
        }
    }

    /// Check if right can be suspended during emergency
    pub fn suspendable_during_emergency(&self) -> bool {
        // Articles 20 and 21 cannot be suspended even during emergency
        !matches!(self, Self::ProtectionAgainstConviction | Self::RightToLife)
    }
}

/// Article 19(1) Six Freedoms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Article19Freedom {
    /// Article 19(1)(a) - Freedom of speech and expression
    SpeechExpression,
    /// Article 19(1)(b) - Freedom to assemble peacefully
    Assembly,
    /// Article 19(1)(c) - Freedom to form associations/unions
    Association,
    /// Article 19(1)(d) - Freedom to move freely
    Movement,
    /// Article 19(1)(e) - Freedom to reside and settle
    Residence,
    /// Article 19(1)(g) - Freedom to practice profession/trade
    Profession,
}

impl Article19Freedom {
    /// Get reasonable restrictions (Article 19(2)-(6))
    pub fn restrictions(&self) -> Vec<&'static str> {
        match self {
            Self::SpeechExpression => vec![
                "Sovereignty and integrity of India",
                "Security of the State",
                "Friendly relations with foreign states",
                "Public order",
                "Decency or morality",
                "Contempt of court",
                "Defamation",
                "Incitement to an offence",
            ],
            Self::Assembly => vec!["Sovereignty and integrity of India", "Public order"],
            Self::Association => vec![
                "Sovereignty and integrity of India",
                "Public order",
                "Morality",
            ],
            Self::Movement => vec![
                "Interests of the general public",
                "Protection of scheduled tribes",
            ],
            Self::Residence => vec![
                "Interests of the general public",
                "Protection of scheduled tribes",
            ],
            Self::Profession => vec![
                "Interests of the general public",
                "Professional/technical qualifications",
                "State monopoly",
            ],
        }
    }
}

/// Directive Principles of State Policy (Part IV)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectivePrinciple {
    /// Article 38 - Social welfare
    SocialWelfare,
    /// Article 39 - Adequate means of livelihood
    MeansOfLivelihood,
    /// Article 39A - Equal justice and free legal aid
    FreeJustice,
    /// Article 40 - Village Panchayats
    Panchayats,
    /// Article 41 - Right to work, education, public assistance
    RightToWork,
    /// Article 42 - Just and humane work conditions
    WorkConditions,
    /// Article 43 - Living wage
    LivingWage,
    /// Article 43A - Worker participation in management
    WorkerParticipation,
    /// Article 44 - Uniform Civil Code
    UniformCivilCode,
    /// Article 45 - Early childhood care
    ChildCare,
    /// Article 46 - Promotion of weaker sections
    WeakerSections,
    /// Article 47 - Nutrition, public health
    PublicHealth,
    /// Article 48 - Agriculture and animal husbandry
    Agriculture,
    /// Article 48A - Environment and forests
    Environment,
    /// Article 49 - Protection of monuments
    Monuments,
    /// Article 50 - Separation of judiciary
    JudiciarySeparation,
    /// Article 51 - International peace
    InternationalPeace,
}

impl DirectivePrinciple {
    /// Get article number
    pub fn article(&self) -> u32 {
        match self {
            Self::SocialWelfare => 38,
            Self::MeansOfLivelihood => 39,
            Self::FreeJustice => 39, // 39A
            Self::Panchayats => 40,
            Self::RightToWork => 41,
            Self::WorkConditions => 42,
            Self::LivingWage => 43,
            Self::WorkerParticipation => 43, // 43A
            Self::UniformCivilCode => 44,
            Self::ChildCare => 45,
            Self::WeakerSections => 46,
            Self::PublicHealth => 47,
            Self::Agriculture => 48,
            Self::Environment => 48, // 48A
            Self::Monuments => 49,
            Self::JudiciarySeparation => 50,
            Self::InternationalPeace => 51,
        }
    }

    /// Check if principle is justiciable
    pub fn is_justiciable(&self) -> bool {
        // DPSPs are generally not justiciable, but some have been made enforceable
        matches!(
            self,
            Self::FreeJustice | Self::ChildCare | Self::Environment
        )
    }
}

/// Fundamental Duties (Article 51A)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FundamentalDuty {
    /// 51A(a) - Abide by Constitution, respect flag and anthem
    RespectConstitution,
    /// 51A(b) - Cherish ideals of freedom struggle
    CherishIdeals,
    /// 51A(c) - Uphold sovereignty and integrity
    UpholdSovereignty,
    /// 51A(d) - Defend the country
    DefendCountry,
    /// 51A(e) - Promote harmony
    PromoteHarmony,
    /// 51A(f) - Preserve composite culture
    PreserveCulture,
    /// 51A(g) - Protect environment
    ProtectEnvironment,
    /// 51A(h) - Develop scientific temper
    ScientificTemper,
    /// 51A(i) - Safeguard public property
    SafeguardProperty,
    /// 51A(j) - Strive for excellence
    StriveForExcellence,
    /// 51A(k) - Provide education to children (6-14 years)
    ProvideEducation,
}

impl FundamentalDuty {
    /// Get clause reference
    pub fn clause(&self) -> char {
        match self {
            Self::RespectConstitution => 'a',
            Self::CherishIdeals => 'b',
            Self::UpholdSovereignty => 'c',
            Self::DefendCountry => 'd',
            Self::PromoteHarmony => 'e',
            Self::PreserveCulture => 'f',
            Self::ProtectEnvironment => 'g',
            Self::ScientificTemper => 'h',
            Self::SafeguardProperty => 'i',
            Self::StriveForExcellence => 'j',
            Self::ProvideEducation => 'k',
        }
    }
}

/// Writ types under Article 32 and 226
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WritType {
    /// Habeas Corpus - produce the body
    HabeasCorpus,
    /// Mandamus - command to perform duty
    Mandamus,
    /// Prohibition - to inferior court to stop
    Prohibition,
    /// Certiorari - to quash order
    Certiorari,
    /// Quo Warranto - by what authority
    QuoWarranto,
}

impl WritType {
    /// Get Latin meaning
    pub fn meaning(&self) -> &'static str {
        match self {
            Self::HabeasCorpus => "produce the body",
            Self::Mandamus => "we command",
            Self::Prohibition => "to forbid",
            Self::Certiorari => "to be informed",
            Self::QuoWarranto => "by what authority",
        }
    }

    /// Get purpose
    pub fn purpose(&self) -> &'static str {
        match self {
            Self::HabeasCorpus => "Protect personal liberty from illegal detention",
            Self::Mandamus => "Command public authority to perform mandatory duty",
            Self::Prohibition => "Prevent inferior court/tribunal from exceeding jurisdiction",
            Self::Certiorari => "Quash orders of inferior courts/tribunals",
            Self::QuoWarranto => "Challenge unauthorized holding of public office",
        }
    }

    /// Who can be respondent
    pub fn against_whom(&self) -> &'static str {
        match self {
            Self::HabeasCorpus => "Any person (public or private) detaining someone",
            Self::Mandamus => "Public authorities, officials, tribunals, corporations",
            Self::Prohibition => "Inferior courts and tribunals",
            Self::Certiorari => "Inferior courts, tribunals, public authorities",
            Self::QuoWarranto => "Person holding public office",
        }
    }
}

/// Writ petition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WritPetition {
    /// Writ type
    pub writ_type: WritType,
    /// Article invoked (32 or 226)
    pub article: u32,
    /// Court filed in
    pub court: ConstitutionalCourt,
    /// Fundamental right violated
    pub right_violated: Option<FundamentalRight>,
    /// Petitioner name
    pub petitioner: String,
    /// Respondent name
    pub respondent: String,
    /// Prayer/relief sought
    pub prayer: String,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Case number
    pub case_number: Option<String>,
}

/// Constitutional courts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutionalCourt {
    /// Supreme Court (Article 32)
    SupremeCourt,
    /// High Court (Article 226)
    HighCourt,
}

impl ConstitutionalCourt {
    /// Get writ jurisdiction article
    pub fn writ_article(&self) -> u32 {
        match self {
            Self::SupremeCourt => 32,
            Self::HighCourt => 226,
        }
    }

    /// Get territorial jurisdiction
    pub fn jurisdiction(&self) -> &'static str {
        match self {
            Self::SupremeCourt => "All India",
            Self::HighCourt => "State/Union Territory",
        }
    }
}

/// Emergency types (Part XVIII)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmergencyType {
    /// Article 352 - National Emergency (war, external aggression, armed rebellion)
    NationalEmergency,
    /// Article 356 - President's Rule (state emergency)
    PresidentsRule,
    /// Article 360 - Financial Emergency
    FinancialEmergency,
}

impl EmergencyType {
    /// Get article number
    pub fn article(&self) -> u32 {
        match self {
            Self::NationalEmergency => 352,
            Self::PresidentsRule => 356,
            Self::FinancialEmergency => 360,
        }
    }

    /// Get proclamation duration
    pub fn initial_duration_months(&self) -> u32 {
        match self {
            Self::NationalEmergency => 6,  // Can be extended indefinitely
            Self::PresidentsRule => 6,     // Max 3 years (6 months × 6)
            Self::FinancialEmergency => 0, // Continues until revoked
        }
    }

    /// Get maximum duration (months)
    pub fn max_duration_months(&self) -> Option<u32> {
        match self {
            Self::NationalEmergency => None,  // No limit
            Self::PresidentsRule => Some(36), // 3 years
            Self::FinancialEmergency => None, // No limit
        }
    }
}

/// Constitutional amendment (Article 368)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstitutionalAmendment {
    /// Amendment number
    pub number: u32,
    /// Date of enactment
    pub date: NaiveDate,
    /// Subject matter
    pub subject: String,
    /// Key provisions
    pub key_provisions: Vec<String>,
    /// State ratification required
    pub state_ratification_required: bool,
    /// Number of states ratified (if applicable)
    pub states_ratified: Option<u32>,
}

/// Amendment procedure type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmendmentProcedure {
    /// Simple majority (not amendment under 368)
    SimpleMajority,
    /// Special majority (2/3 of members present + voting, majority of total membership)
    SpecialMajority,
    /// Special majority + ratification by half the states
    SpecialMajorityWithRatification,
}

impl AmendmentProcedure {
    /// Get requirements
    pub fn requirements(&self) -> &'static str {
        match self {
            Self::SimpleMajority => "Simple majority of members present and voting",
            Self::SpecialMajority => {
                "2/3 of members present and voting + majority of total membership"
            }
            Self::SpecialMajorityWithRatification => {
                "Special majority + ratification by legislatures of not less than half the states"
            }
        }
    }

    /// Get examples of provisions requiring this procedure
    pub fn examples(&self) -> Vec<&'static str> {
        match self {
            Self::SimpleMajority => vec!["Admission of new states", "Formation of new states"],
            Self::SpecialMajority => {
                vec![
                    "Fundamental rights",
                    "Directive principles",
                    "Most amendments",
                ]
            }
            Self::SpecialMajorityWithRatification => vec![
                "Election of President",
                "Extent of executive power",
                "Supreme Court and High Courts",
                "Union-State relations",
                "Representation of states in Parliament",
                "Amendment procedure itself",
            ],
        }
    }
}

/// Basic structure doctrine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicStructure {
    /// Feature name
    pub feature: String,
    /// Case establishing the feature
    pub establishing_case: String,
    /// Year established
    pub year: u32,
}

impl BasicStructure {
    /// Get recognized basic structure features
    pub fn recognized_features() -> Vec<Self> {
        vec![
            Self {
                feature: "Supremacy of the Constitution".to_string(),
                establishing_case: "Kesavananda Bharati v. State of Kerala".to_string(),
                year: 1973,
            },
            Self {
                feature: "Republican and democratic form of government".to_string(),
                establishing_case: "Kesavananda Bharati v. State of Kerala".to_string(),
                year: 1973,
            },
            Self {
                feature: "Secular character".to_string(),
                establishing_case: "S.R. Bommai v. Union of India".to_string(),
                year: 1994,
            },
            Self {
                feature: "Federal character".to_string(),
                establishing_case: "Kesavananda Bharati v. State of Kerala".to_string(),
                year: 1973,
            },
            Self {
                feature: "Separation of powers".to_string(),
                establishing_case: "Kesavananda Bharati v. State of Kerala".to_string(),
                year: 1973,
            },
            Self {
                feature: "Judicial review".to_string(),
                establishing_case: "L. Chandra Kumar v. Union of India".to_string(),
                year: 1997,
            },
            Self {
                feature: "Rule of law".to_string(),
                establishing_case: "Indira Gandhi v. Raj Narain".to_string(),
                year: 1975,
            },
            Self {
                feature: "Free and fair elections".to_string(),
                establishing_case: "Indira Gandhi v. Raj Narain".to_string(),
                year: 1975,
            },
            Self {
                feature: "Independence of judiciary".to_string(),
                establishing_case: "S.P. Gupta v. Union of India".to_string(),
                year: 1982,
            },
            Self {
                feature: "Limited amending power".to_string(),
                establishing_case: "Kesavananda Bharati v. State of Kerala".to_string(),
                year: 1973,
            },
        ]
    }
}

/// Public Interest Litigation (PIL)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicInterestLitigation {
    /// Case title
    pub title: String,
    /// Petitioner type
    pub petitioner_type: PilPetitionerType,
    /// Issue addressed
    pub issue: String,
    /// Court
    pub court: ConstitutionalCourt,
    /// Filing date
    pub filing_date: NaiveDate,
    /// Fundamental right involved
    pub right_involved: Option<FundamentalRight>,
}

/// PIL petitioner type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PilPetitionerType {
    /// Individual citizen
    Individual,
    /// NGO
    Ngo,
    /// Journalist
    Journalist,
    /// Social activist
    SocialActivist,
    /// Lawyer
    Lawyer,
    /// Suo motu (court's own motion)
    SuoMotu,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fundamental_right_articles() {
        assert_eq!(FundamentalRight::EqualityBeforeLaw.article(), "14");
        assert_eq!(FundamentalRight::RightToLife.article(), "21");
        assert_eq!(FundamentalRight::RightToEducation.article(), "21A");
    }

    #[test]
    fn test_emergency_suspension() {
        assert!(FundamentalRight::SixFreedoms.suspendable_during_emergency());
        assert!(!FundamentalRight::RightToLife.suspendable_during_emergency());
        assert!(!FundamentalRight::ProtectionAgainstConviction.suspendable_during_emergency());
    }

    #[test]
    fn test_article19_restrictions() {
        let speech_restrictions = Article19Freedom::SpeechExpression.restrictions();
        assert!(speech_restrictions.contains(&"Public order"));
        assert!(speech_restrictions.contains(&"Defamation"));
    }

    #[test]
    fn test_writ_types() {
        assert_eq!(WritType::HabeasCorpus.meaning(), "produce the body");
        assert_eq!(WritType::Mandamus.meaning(), "we command");
    }

    #[test]
    fn test_emergency_types() {
        assert_eq!(EmergencyType::NationalEmergency.article(), 352);
        assert_eq!(
            EmergencyType::PresidentsRule.max_duration_months(),
            Some(36)
        );
    }

    #[test]
    fn test_amendment_procedures() {
        let special = AmendmentProcedure::SpecialMajorityWithRatification;
        let examples = special.examples();
        assert!(examples.contains(&"Election of President"));
    }

    #[test]
    fn test_basic_structure() {
        let features = BasicStructure::recognized_features();
        assert!(!features.is_empty());
        assert!(
            features
                .iter()
                .any(|f| f.feature.contains("Judicial review"))
        );
    }

    #[test]
    fn test_constitutional_court() {
        assert_eq!(ConstitutionalCourt::SupremeCourt.writ_article(), 32);
        assert_eq!(ConstitutionalCourt::HighCourt.writ_article(), 226);
    }

    #[test]
    fn test_fundamental_duties() {
        assert_eq!(FundamentalDuty::RespectConstitution.clause(), 'a');
        assert_eq!(FundamentalDuty::ProvideEducation.clause(), 'k');
    }

    #[test]
    fn test_directive_principles() {
        assert_eq!(DirectivePrinciple::UniformCivilCode.article(), 44);
        assert!(DirectivePrinciple::Environment.is_justiciable());
        assert!(!DirectivePrinciple::UniformCivilCode.is_justiciable());
    }
}
