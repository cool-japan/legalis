//! Trust Creation and Constitution
//!
//! This module implements the requirements for valid trust creation under English law,
//! including the three certainties (Knight v Knight [1840]) and constitution requirements.
//!
//! ## Three Certainties (Knight v Knight [1840])
//!
//! For a trust to be valid, there must be:
//! 1. **Certainty of Intention** - Clear intent to create a trust, not a gift or loan
//! 2. **Certainty of Subject Matter** - Clear identification of trust property and beneficial shares
//! 3. **Certainty of Objects** - Clear identification of beneficiaries (or charitable purpose)
//!
//! ## Constitution of Trust (Milroy v Lord [1862])
//!
//! A trust must be properly constituted through one of:
//! - **Transfer to trustees** - Legal title must be properly transferred
//! - **Declaration of self as trustee** - Settlor declares themselves trustee
//! - **Disposition of equitable interest** - s.53(1)(c) LPA 1925 (must be in writing)
//!
//! "Equity will not perfect an imperfect gift" - but exceptions exist:
//! - **Re Rose [1952]** - Settlor has done everything in their power
//! - **Strong v Bird [1874]** - Imperfect gift perfected by vesting as executor
//! - **Unconscionability** (Pennington v Waine [2002])
//!
//! ## Formality Requirements
//!
//! - **s.53(1)(b) LPA 1925**: Trusts of land must be evidenced in writing
//! - **s.53(1)(c) LPA 1925**: Disposition of equitable interest must be in writing (Grey v IRC)
//! - **s.53(2)**: Resulting and constructive trusts exempt from formalities

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::error::{TrustError, TrustResult};
use super::types::{Beneficiary, BeneficiaryType, PropertyType, TrustType};

// ============================================================================
// Certainty of Intention
// ============================================================================

/// Words used to express trust intention
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntentionWords {
    /// Imperative words clearly creating trust (e.g., "on trust", "as trustee")
    Imperative(String),
    /// Precatory words (wishes/hopes) - generally insufficient (Re Adams and Kensington Vestry)
    Precatory(String),
    /// Mixed or ambiguous words requiring interpretation
    Ambiguous(String),
}

impl IntentionWords {
    /// Analyze whether words are sufficient to create trust
    ///
    /// # Returns
    /// - `true` for imperative words
    /// - `false` for precatory words (unless context shows trust intention)
    pub fn indicates_trust(&self) -> bool {
        match self {
            IntentionWords::Imperative(_) => true,
            IntentionWords::Precatory(_) => false,
            IntentionWords::Ambiguous(_) => false, // Conservative approach
        }
    }

    /// Get the actual words used
    pub fn words(&self) -> &str {
        match self {
            IntentionWords::Imperative(w)
            | IntentionWords::Precatory(w)
            | IntentionWords::Ambiguous(w) => w,
        }
    }
}

/// Certainty of intention analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertaintyOfIntention {
    /// Words used by settlor
    pub words: IntentionWords,
    /// Whether intention is certain
    pub is_certain: bool,
    /// Supporting evidence for trust intention
    pub supporting_evidence: Vec<String>,
    /// Relevant case law
    pub case_law: Vec<CaseLawReference>,
    /// Analysis notes
    pub analysis: String,
}

impl CertaintyOfIntention {
    /// Create new certainty of intention analysis
    pub fn new(words: IntentionWords) -> Self {
        let is_certain = words.indicates_trust();
        let analysis = match &words {
            IntentionWords::Imperative(w) => {
                format!(
                    "The words '{}' are imperative and clearly indicate trust intention.",
                    w
                )
            }
            IntentionWords::Precatory(w) => {
                format!(
                    "The words '{}' are precatory (expressing wish/hope). Following Re Adams and \
                     Kensington Vestry [1884], precatory words do not create a trust unless \
                     context clearly shows trust intention.",
                    w
                )
            }
            IntentionWords::Ambiguous(w) => {
                format!(
                    "The words '{}' are ambiguous. The court must examine the full context \
                     to determine whether a trust was intended (Paul v Constance [1977]).",
                    w
                )
            }
        };

        Self {
            words,
            is_certain,
            supporting_evidence: Vec::new(),
            case_law: Vec::new(),
            analysis,
        }
    }

    /// Add supporting evidence that strengthens trust intention
    pub fn add_supporting_evidence(&mut self, evidence: &str) {
        self.supporting_evidence.push(evidence.to_string());
        // Re-evaluate if we have ambiguous words with supporting evidence
        if matches!(self.words, IntentionWords::Ambiguous(_))
            && !self.supporting_evidence.is_empty()
        {
            self.is_certain = true;
            self.analysis
                .push_str(" However, supporting evidence indicates trust intention was present.");
        }
    }
}

// ============================================================================
// Certainty of Subject Matter
// ============================================================================

/// Subject matter certainty issues
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubjectMatterIssue {
    /// Property not identified (Palmer v Simmonds - "bulk of my residue")
    PropertyNotIdentified {
        /// Uncertain words used
        uncertain_words: String,
    },
    /// Beneficial shares not certain (Boyce v Boyce)
    BeneficialSharesUncertain {
        /// Description of uncertain shares
        description: String,
    },
    /// Property adequately certain
    Certain,
}

/// Certainty of subject matter analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertaintyOfSubjectMatter {
    /// Description of trust property
    pub property_description: String,
    /// Type of property
    pub property_type: PropertyType,
    /// Whether property is identified
    pub property_identified: bool,
    /// Description of beneficial interests
    pub beneficial_interests: String,
    /// Whether beneficial shares are certain
    pub shares_certain: bool,
    /// Any issues found
    pub issues: Vec<SubjectMatterIssue>,
    /// Whether tangible vs intangible (Re London Wine - tangible must be segregated)
    pub is_tangible: bool,
    /// Analysis notes
    pub analysis: String,
}

impl CertaintyOfSubjectMatter {
    /// Analyze subject matter certainty
    pub fn analyze(
        property_description: &str,
        property_type: PropertyType,
        beneficial_interests: &str,
    ) -> Self {
        let is_tangible = matches!(
            property_type,
            PropertyType::RealProperty | PropertyType::PersonalProperty
        );

        let (property_identified, property_issue) =
            Self::check_property_identification(property_description, is_tangible);
        let (shares_certain, shares_issue) = Self::check_beneficial_shares(beneficial_interests);

        let mut issues = Vec::new();
        if let Some(issue) = property_issue {
            issues.push(issue);
        }
        if let Some(issue) = shares_issue {
            issues.push(issue);
        }

        let analysis = Self::generate_analysis(property_identified, shares_certain, &issues);

        Self {
            property_description: property_description.to_string(),
            property_type,
            property_identified,
            beneficial_interests: beneficial_interests.to_string(),
            shares_certain,
            issues,
            is_tangible,
            analysis,
        }
    }

    fn check_property_identification(
        description: &str,
        is_tangible: bool,
    ) -> (bool, Option<SubjectMatterIssue>) {
        let desc_lower = description.to_lowercase();

        // Check for inherently uncertain words (Palmer v Simmonds)
        let uncertain_indicators = [
            "bulk of",
            "most of",
            "some of",
            "such part as",
            "remainder",
            "residue", // unless entire residue
        ];

        for indicator in &uncertain_indicators {
            if desc_lower.contains(indicator)
                && !desc_lower.contains("entire")
                && !desc_lower.contains("all of")
            {
                return (
                    false,
                    Some(SubjectMatterIssue::PropertyNotIdentified {
                        uncertain_words: description.to_string(),
                    }),
                );
            }
        }

        // For tangible property, must be segregated (Re London Wine)
        if is_tangible && desc_lower.contains("from my collection") {
            return (
                false,
                Some(SubjectMatterIssue::PropertyNotIdentified {
                    uncertain_words: "Tangible property from larger collection not segregated"
                        .to_string(),
                }),
            );
        }

        (true, None)
    }

    fn check_beneficial_shares(interests: &str) -> (bool, Option<SubjectMatterIssue>) {
        let interests_lower = interests.to_lowercase();

        // Check for uncertain share descriptions (Boyce v Boyce)
        let uncertain_share_indicators = [
            "reasonable",
            "fair",
            "appropriate",
            "such amount as",
            "as they see fit",
        ];

        for indicator in &uncertain_share_indicators {
            if interests_lower.contains(indicator) {
                return (
                    false,
                    Some(SubjectMatterIssue::BeneficialSharesUncertain {
                        description: interests.to_string(),
                    }),
                );
            }
        }

        (true, None)
    }

    fn generate_analysis(
        property_identified: bool,
        shares_certain: bool,
        issues: &[SubjectMatterIssue],
    ) -> String {
        let mut analysis = String::new();

        if property_identified && shares_certain {
            analysis.push_str(
                "Subject matter is sufficiently certain. Both the trust property and \
                 beneficial shares are identifiable.",
            );
        } else {
            if !property_identified {
                analysis.push_str(
                    "Trust property is not sufficiently certain. Following Palmer v Simmonds \
                     [1854], vague descriptions like 'bulk of' or 'most of' fail for uncertainty. ",
                );
            }
            if !shares_certain {
                analysis.push_str(
                    "Beneficial shares are not sufficiently certain. Following Boyce v Boyce \
                     [1849], shares dependent on uncertain factors fail. ",
                );
            }
            for issue in issues {
                match issue {
                    SubjectMatterIssue::PropertyNotIdentified { uncertain_words } => {
                        analysis
                            .push_str(&format!("Uncertain words used: '{}'. ", uncertain_words));
                    }
                    SubjectMatterIssue::BeneficialSharesUncertain { description } => {
                        analysis
                            .push_str(&format!("Uncertain share description: '{}'. ", description));
                    }
                    SubjectMatterIssue::Certain => {}
                }
            }
        }

        analysis
    }

    /// Check if subject matter is certain
    pub fn is_certain(&self) -> bool {
        self.property_identified && self.shares_certain
    }
}

// ============================================================================
// Certainty of Objects
// ============================================================================

/// Test for certainty of objects (depends on trust type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectsCertaintyTest {
    /// Fixed trust - complete list test (IRC v Broadway Cottages)
    /// Must be able to list ALL beneficiaries
    CompleteList,
    /// Discretionary trust - is/is not test (McPhail v Doulton [1971])
    /// Must be able to say whether ANY GIVEN PERSON is/is not a beneficiary
    IsOrIsNot,
    /// Fiduciary power - is/is not test (same as discretionary)
    FiduciaryPower,
    /// Mere power - "mere power" test (Re Gulbenkian [1970])
    /// Power valid if possible to say ONE person is within the class
    MerePower,
}

impl ObjectsCertaintyTest {
    /// Get test applicable to trust type
    pub fn for_trust_type(trust_type: TrustType) -> Self {
        match trust_type {
            TrustType::Fixed | TrustType::Bare => ObjectsCertaintyTest::CompleteList,
            TrustType::Discretionary => ObjectsCertaintyTest::IsOrIsNot,
            TrustType::Express | TrustType::Charitable => ObjectsCertaintyTest::IsOrIsNot,
            TrustType::Resulting | TrustType::Constructive => {
                // Resulting/constructive arise by operation of law
                ObjectsCertaintyTest::IsOrIsNot
            }
        }
    }
}

/// Issues with certainty of objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectsIssue {
    /// Conceptual uncertainty - class description unclear
    ConceptualUncertainty {
        /// Description of the uncertainty
        description: String,
    },
    /// Evidential uncertainty - hard to prove membership (generally not fatal)
    EvidentialUncertainty {
        /// Description of evidential difficulty
        description: String,
    },
    /// Administrative unworkability (McPhail v Doulton criterion)
    AdministrativeUnworkability {
        /// Description of unworkability
        description: String,
    },
    /// Class too wide (R v District Auditor, ex p West Yorkshire)
    ClassTooWide {
        /// Description of width issue
        description: String,
    },
}

/// Certainty of objects analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertaintyOfObjects {
    /// Description of beneficiary class
    pub class_description: String,
    /// Test applied
    pub test_applied: ObjectsCertaintyTest,
    /// Whether objects are certain under applicable test
    pub is_certain: bool,
    /// Issues found
    pub issues: Vec<ObjectsIssue>,
    /// Example of person clearly within class
    pub example_within: Option<String>,
    /// Example of person clearly outside class
    pub example_outside: Option<String>,
    /// Analysis notes
    pub analysis: String,
}

impl CertaintyOfObjects {
    /// Analyze certainty of objects for a given trust type
    pub fn analyze(
        class_description: &str,
        trust_type: TrustType,
        beneficiaries: &[Beneficiary],
    ) -> Self {
        let test_applied = ObjectsCertaintyTest::for_trust_type(trust_type);
        let (is_certain, issues) = Self::apply_test(class_description, test_applied, beneficiaries);
        let analysis = Self::generate_analysis(test_applied, is_certain, &issues);

        Self {
            class_description: class_description.to_string(),
            test_applied,
            is_certain,
            issues,
            example_within: None,
            example_outside: None,
            analysis,
        }
    }

    fn apply_test(
        class_description: &str,
        test: ObjectsCertaintyTest,
        beneficiaries: &[Beneficiary],
    ) -> (bool, Vec<ObjectsIssue>) {
        let mut issues = Vec::new();
        let desc_lower = class_description.to_lowercase();

        // Check for conceptually uncertain descriptions
        let conceptually_uncertain = [
            "friends", // Re Barlow's WT - but can be saved
            "those who merit it",
            "deserving persons",
            "people I like",
        ];

        for uncertain in &conceptually_uncertain {
            if desc_lower.contains(uncertain) {
                issues.push(ObjectsIssue::ConceptualUncertainty {
                    description: format!(
                        "'{}' is conceptually uncertain - Re Gulbenkian",
                        uncertain
                    ),
                });
            }
        }

        // Check for administratively unworkable classes
        let admin_unworkable = [
            "all the residents of",
            "all inhabitants of",
            "everyone in",
            "the public",
        ];

        for unworkable in &admin_unworkable {
            if desc_lower.contains(unworkable) {
                issues.push(ObjectsIssue::AdministrativeUnworkability {
                    description: format!(
                        "'{}' may be administratively unworkable - R v District Auditor",
                        class_description
                    ),
                });
                issues.push(ObjectsIssue::ClassTooWide {
                    description: "Class may be so wide as to be meaningless".to_string(),
                });
            }
        }

        // For fixed trusts (complete list test), check if we can enumerate
        if test == ObjectsCertaintyTest::CompleteList {
            let has_class_beneficiaries = beneficiaries
                .iter()
                .any(|b| b.beneficiary_type == BeneficiaryType::Class);
            if has_class_beneficiaries {
                // Class beneficiaries in fixed trust - can we list them all?
                issues.push(ObjectsIssue::ConceptualUncertainty {
                    description: "Fixed trust with class beneficiary - must be able to enumerate \
                                  complete list (IRC v Broadway Cottages)"
                        .to_string(),
                });
            }
        }

        let is_certain = issues.is_empty();
        (is_certain, issues)
    }

    fn generate_analysis(
        test: ObjectsCertaintyTest,
        is_certain: bool,
        issues: &[ObjectsIssue],
    ) -> String {
        let mut analysis = String::new();

        let test_name = match test {
            ObjectsCertaintyTest::CompleteList => {
                "complete list test (IRC v Broadway Cottages [1955])"
            }
            ObjectsCertaintyTest::IsOrIsNot => "is/is not test (McPhail v Doulton [1971])",
            ObjectsCertaintyTest::FiduciaryPower => "is/is not test for fiduciary power",
            ObjectsCertaintyTest::MerePower => "mere power test (Re Gulbenkian [1970])",
        };

        analysis.push_str(&format!("Applying {}. ", test_name));

        if is_certain {
            analysis.push_str("Objects are sufficiently certain under this test.");
        } else {
            analysis.push_str("Objects fail certainty test. Issues: ");
            for issue in issues {
                match issue {
                    ObjectsIssue::ConceptualUncertainty { description } => {
                        analysis.push_str(&format!("Conceptual uncertainty: {}. ", description));
                    }
                    ObjectsIssue::EvidentialUncertainty { description } => {
                        analysis.push_str(&format!(
                            "Evidential uncertainty (usually not fatal): {}. ",
                            description
                        ));
                    }
                    ObjectsIssue::AdministrativeUnworkability { description } => {
                        analysis
                            .push_str(&format!("Administratively unworkable: {}. ", description));
                    }
                    ObjectsIssue::ClassTooWide { description } => {
                        analysis.push_str(&format!("Class too wide: {}. ", description));
                    }
                }
            }
        }

        analysis
    }

    /// Set example of person within class
    pub fn with_example_within(mut self, example: &str) -> Self {
        self.example_within = Some(example.to_string());
        self
    }

    /// Set example of person outside class
    pub fn with_example_outside(mut self, example: &str) -> Self {
        self.example_outside = Some(example.to_string());
        self
    }
}

// ============================================================================
// Combined Three Certainties
// ============================================================================

/// Combined three certainties analysis (Knight v Knight [1840])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreeCertainties {
    /// Certainty of intention
    pub intention: CertaintyOfIntention,
    /// Certainty of subject matter
    pub subject_matter: CertaintyOfSubjectMatter,
    /// Certainty of objects
    pub objects: CertaintyOfObjects,
}

impl ThreeCertainties {
    /// Create combined analysis
    pub fn new(
        intention: CertaintyOfIntention,
        subject_matter: CertaintyOfSubjectMatter,
        objects: CertaintyOfObjects,
    ) -> Self {
        Self {
            intention,
            subject_matter,
            objects,
        }
    }

    /// Check if all three certainties are satisfied
    pub fn all_satisfied(&self) -> bool {
        self.intention.is_certain && self.subject_matter.is_certain() && self.objects.is_certain
    }

    /// Get list of failed certainties
    pub fn failed_certainties(&self) -> Vec<&'static str> {
        let mut failed = Vec::new();
        if !self.intention.is_certain {
            failed.push("intention");
        }
        if !self.subject_matter.is_certain() {
            failed.push("subject matter");
        }
        if !self.objects.is_certain {
            failed.push("objects");
        }
        failed
    }
}

// ============================================================================
// Trust Declaration
// ============================================================================

/// A trust declaration to be validated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustDeclaration {
    /// Settlor name
    pub settlor: String,
    /// Property description
    pub property: String,
    /// Property type
    pub property_type: PropertyType,
    /// Beneficiaries (or description of class)
    pub beneficiaries: Vec<String>,
    /// Words expressing intention
    pub intention_words: String,
    /// Trust type (if express, or inferred)
    pub trust_type: TrustType,
    /// Date of declaration
    pub declaration_date: NaiveDate,
    /// Is declaration in writing?
    pub in_writing: bool,
    /// Witnesses (if applicable)
    pub witnesses: Vec<String>,
}

impl TrustDeclaration {
    /// Create a new trust declaration builder
    pub fn builder() -> TrustDeclarationBuilder {
        TrustDeclarationBuilder::new()
    }
}

/// Builder for trust declarations
#[derive(Debug, Default)]
pub struct TrustDeclarationBuilder {
    settlor: Option<String>,
    property: Option<String>,
    property_type: Option<PropertyType>,
    beneficiaries: Vec<String>,
    intention_words: Option<String>,
    trust_type: Option<TrustType>,
    declaration_date: Option<NaiveDate>,
    in_writing: bool,
    witnesses: Vec<String>,
}

impl TrustDeclarationBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set settlor
    pub fn settlor(mut self, settlor: &str) -> Self {
        self.settlor = Some(settlor.to_string());
        self
    }

    /// Set property
    pub fn property(mut self, property: &str) -> Self {
        self.property = Some(property.to_string());
        self
    }

    /// Set property type
    pub fn property_type(mut self, property_type: PropertyType) -> Self {
        self.property_type = Some(property_type);
        self
    }

    /// Add beneficiary
    pub fn add_beneficiary(mut self, beneficiary: &str) -> Self {
        self.beneficiaries.push(beneficiary.to_string());
        self
    }

    /// Set intention words
    pub fn intention_words(mut self, words: &str) -> Self {
        self.intention_words = Some(words.to_string());
        self
    }

    /// Set trust type
    pub fn trust_type(mut self, trust_type: TrustType) -> Self {
        self.trust_type = Some(trust_type);
        self
    }

    /// Set declaration date
    pub fn declaration_date(mut self, date: NaiveDate) -> Self {
        self.declaration_date = Some(date);
        self
    }

    /// Mark as written
    pub fn in_writing(mut self, written: bool) -> Self {
        self.in_writing = written;
        self
    }

    /// Add witness
    pub fn add_witness(mut self, witness: &str) -> Self {
        self.witnesses.push(witness.to_string());
        self
    }

    /// Build the declaration
    pub fn build(self) -> Result<TrustDeclaration, TrustError> {
        let settlor = self.settlor.ok_or(TrustError::NotProperlyConstituted)?;
        let property = self
            .property
            .ok_or(TrustError::LacksCertaintySubjectMatter {
                words: "No property specified".to_string(),
            })?;
        let intention_words = self
            .intention_words
            .ok_or(TrustError::LacksCertaintyIntention)?;

        if self.beneficiaries.is_empty() {
            return Err(TrustError::LacksCertaintyObjects);
        }

        Ok(TrustDeclaration {
            settlor,
            property,
            property_type: self.property_type.unwrap_or(PropertyType::Money),
            beneficiaries: self.beneficiaries,
            intention_words,
            trust_type: self.trust_type.unwrap_or(TrustType::Express),
            declaration_date: self
                .declaration_date
                .unwrap_or_else(|| chrono::Local::now().date_naive()),
            in_writing: self.in_writing,
            witnesses: self.witnesses,
        })
    }
}

// ============================================================================
// Trust Constitution
// ============================================================================

/// Method of trust constitution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutionMethod {
    /// Transfer legal title to trustees
    TransferToTrustees,
    /// Settlor declares self as trustee (no transfer needed)
    SelfDeclaration,
    /// Disposition of existing equitable interest (s.53(1)(c) LPA 1925)
    DispositionOfEquitableInterest,
    /// Direction to trustee (Grey v IRC vs Vandervell)
    DirectionToTrustee,
}

/// Status of trust constitution
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutionStatus {
    /// Fully constituted - trust is valid
    FullyConstituted,
    /// Incompletely constituted - may be saved by exception
    Incompletely {
        /// Reason for incomplete constitution
        reason: String,
    },
    /// Saved by Re Rose principle (settlor has done everything in their power)
    SavedByReRose,
    /// Saved by Strong v Bird (donee becomes executor)
    SavedByStrongVBird,
    /// Saved by unconscionability (Pennington v Waine)
    SavedByUnconscionability,
    /// Covenant under seal - can be enforced by covenantee
    CovenantUnderSeal,
}

/// Trust constitution analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustConstitution {
    /// Method of constitution
    pub method: ConstitutionMethod,
    /// Status
    pub status: ConstitutionStatus,
    /// Is transfer complete for given asset type?
    pub transfer_complete: bool,
    /// Formality requirements satisfied?
    pub formalities_satisfied: bool,
    /// Analysis notes
    pub analysis: String,
}

impl TrustConstitution {
    /// Analyze constitution for self-declaration
    pub fn self_declaration() -> Self {
        Self {
            method: ConstitutionMethod::SelfDeclaration,
            status: ConstitutionStatus::FullyConstituted,
            transfer_complete: true, // No transfer needed
            formalities_satisfied: true,
            analysis: "Settlor has declared themselves trustee. No transfer of legal title is \
                      required - the trust is fully constituted by the declaration itself."
                .to_string(),
        }
    }

    /// Analyze constitution for transfer to trustees
    pub fn transfer_to_trustees(
        property_type: PropertyType,
        transfer_executed: bool,
        all_steps_taken: bool,
    ) -> Self {
        let formalities = Self::check_transfer_formalities(property_type);

        let (status, analysis) = if transfer_executed {
            (
                ConstitutionStatus::FullyConstituted,
                "Legal title has been properly transferred to the trustees. \
                 Trust is fully constituted."
                    .to_string(),
            )
        } else if all_steps_taken {
            // Re Rose exception
            (
                ConstitutionStatus::SavedByReRose,
                format!(
                    "Although legal title has not passed, the settlor has done everything \
                     in their power to effect the transfer. Following Re Rose [1952], \
                     the transfer will be treated as complete in equity. Formalities for \
                     {:?}: {}",
                    property_type, formalities
                ),
            )
        } else {
            (
                ConstitutionStatus::Incompletely {
                    reason: "Transfer not complete and Re Rose does not apply".to_string(),
                },
                format!(
                    "Transfer is incomplete. 'Equity will not perfect an imperfect gift' \
                     (Milroy v Lord [1862]). Required transfer formalities for {:?}: {}",
                    property_type, formalities
                ),
            )
        };

        Self {
            method: ConstitutionMethod::TransferToTrustees,
            status,
            transfer_complete: transfer_executed || all_steps_taken,
            formalities_satisfied: true,
            analysis,
        }
    }

    fn check_transfer_formalities(property_type: PropertyType) -> String {
        match property_type {
            PropertyType::RealProperty => {
                "Deed required (s.52 LPA 1925), registered (Land Registration Act 2002)".to_string()
            }
            PropertyType::Securities => {
                "Stock transfer form and registration with company".to_string()
            }
            PropertyType::PersonalProperty => "Delivery with intention to pass title".to_string(),
            PropertyType::Money => "Transfer of funds or delivery of cash".to_string(),
            PropertyType::IntellectualProperty => {
                "Written assignment (Patents Act 1977 s.30, Copyright Act s.90)".to_string()
            }
            PropertyType::BusinessInterest => "Assignment and registration if required".to_string(),
        }
    }

    /// Analyze disposition of equitable interest (s.53(1)(c) LPA 1925)
    pub fn disposition_of_equitable(in_writing: bool, signed: bool) -> Self {
        let (status, analysis) = if in_writing && signed {
            (
                ConstitutionStatus::FullyConstituted,
                "Disposition of equitable interest complies with s.53(1)(c) LPA 1925 - \
                 in writing and signed."
                    .to_string(),
            )
        } else {
            (
                ConstitutionStatus::Incompletely {
                    reason: "s.53(1)(c) LPA 1925 not satisfied".to_string(),
                },
                format!(
                    "Disposition of equitable interest fails s.53(1)(c) LPA 1925. \
                     In writing: {}, Signed: {}. Following Grey v IRC [1960], \
                     disposition must be in writing and signed by the disponor.",
                    in_writing, signed
                ),
            )
        };

        Self {
            method: ConstitutionMethod::DispositionOfEquitableInterest,
            status,
            transfer_complete: in_writing && signed,
            formalities_satisfied: in_writing && signed,
            analysis,
        }
    }
}

// ============================================================================
// Case Law Reference
// ============================================================================

/// Reference to case law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CaseLawReference {
    /// Case name
    pub name: String,
    /// Year
    pub year: u16,
    /// Citation (if available)
    pub citation: Option<String>,
    /// Relevance to analysis
    pub relevance: String,
}

impl CaseLawReference {
    /// Create new case reference
    pub fn new(name: &str, year: u16, relevance: &str) -> Self {
        Self {
            name: name.to_string(),
            year,
            citation: None,
            relevance: relevance.to_string(),
        }
    }

    /// Add citation
    pub fn with_citation(mut self, citation: &str) -> Self {
        self.citation = Some(citation.to_string());
        self
    }
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Check certainty of intention
pub fn check_certainty_intention(words: &str) -> CertaintyOfIntention {
    let intention_words = classify_intention_words(words);
    CertaintyOfIntention::new(intention_words)
}

/// Classify intention words as imperative, precatory, or ambiguous
fn classify_intention_words(words: &str) -> IntentionWords {
    let words_lower = words.to_lowercase();

    // Imperative words
    let imperative_indicators = [
        "on trust",
        "as trustee",
        "upon trust",
        "in trust",
        "hold on trust",
        "to be held",
        "shall hold",
        "i declare myself trustee",
        "i hold as trustee",
    ];

    for indicator in &imperative_indicators {
        if words_lower.contains(indicator) {
            return IntentionWords::Imperative(words.to_string());
        }
    }

    // Precatory words (Re Adams and Kensington Vestry)
    let precatory_indicators = [
        "i hope",
        "i wish",
        "i desire",
        "in full confidence",
        "feeling confident",
        "i request",
        "it is my wish",
        "i would like",
    ];

    for indicator in &precatory_indicators {
        if words_lower.contains(indicator) {
            return IntentionWords::Precatory(words.to_string());
        }
    }

    // Otherwise ambiguous
    IntentionWords::Ambiguous(words.to_string())
}

/// Check certainty of subject matter
pub fn check_certainty_subject_matter(
    property: &str,
    property_type: PropertyType,
    beneficial_interests: &str,
) -> CertaintyOfSubjectMatter {
    CertaintyOfSubjectMatter::analyze(property, property_type, beneficial_interests)
}

/// Check certainty of objects
pub fn check_certainty_objects(
    class_description: &str,
    trust_type: TrustType,
    beneficiaries: &[Beneficiary],
) -> CertaintyOfObjects {
    CertaintyOfObjects::analyze(class_description, trust_type, beneficiaries)
}

/// Check all three certainties for a trust declaration
pub fn check_three_certainties(declaration: &TrustDeclaration) -> TrustResult<ThreeCertainties> {
    let intention = check_certainty_intention(&declaration.intention_words);
    let subject_matter = check_certainty_subject_matter(
        &declaration.property,
        declaration.property_type,
        "equal shares", // Default - could be enhanced
    );

    // Convert string beneficiaries to Beneficiary structs for analysis
    let beneficiaries: Vec<Beneficiary> = declaration
        .beneficiaries
        .iter()
        .map(|b| Beneficiary {
            name: b.clone(),
            beneficiary_type: if b.contains("children") || b.contains("descendants") {
                BeneficiaryType::Class
            } else {
                BeneficiaryType::Individual
            },
            share: None,
            vested: true,
        })
        .collect();

    let class_description = declaration.beneficiaries.join(", ");
    let objects =
        check_certainty_objects(&class_description, declaration.trust_type, &beneficiaries);

    let three_certainties = ThreeCertainties::new(intention, subject_matter, objects);

    if three_certainties.all_satisfied() {
        Ok(three_certainties)
    } else {
        let failed = three_certainties.failed_certainties();
        if failed.contains(&"intention") {
            Err(TrustError::LacksCertaintyIntention)
        } else if failed.contains(&"subject matter") {
            Err(TrustError::LacksCertaintySubjectMatter {
                words: declaration.property.clone(),
            })
        } else {
            Err(TrustError::LacksCertaintyObjects)
        }
    }
}

/// Validate trust constitution
pub fn validate_trust_constitution(
    method: ConstitutionMethod,
    property_type: PropertyType,
    transfer_executed: bool,
    all_steps_taken: bool,
    in_writing: bool,
) -> TrustConstitution {
    match method {
        ConstitutionMethod::SelfDeclaration => TrustConstitution::self_declaration(),
        ConstitutionMethod::TransferToTrustees => TrustConstitution::transfer_to_trustees(
            property_type,
            transfer_executed,
            all_steps_taken,
        ),
        ConstitutionMethod::DispositionOfEquitableInterest => {
            TrustConstitution::disposition_of_equitable(in_writing, true)
        }
        ConstitutionMethod::DirectionToTrustee => {
            // Similar to disposition - Grey v IRC vs Vandervell analysis
            TrustConstitution::disposition_of_equitable(in_writing, true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imperative_words_indicate_trust() {
        let intention = check_certainty_intention("I hold these shares on trust for my children");
        assert!(intention.is_certain);
        assert!(matches!(intention.words, IntentionWords::Imperative(_)));
    }

    #[test]
    fn test_precatory_words_fail() {
        let intention = check_certainty_intention("I hope you will look after my family");
        assert!(!intention.is_certain);
        assert!(matches!(intention.words, IntentionWords::Precatory(_)));
    }

    #[test]
    fn test_subject_matter_bulk_fails() {
        let sm = check_certainty_subject_matter(
            "bulk of my estate",
            PropertyType::Money,
            "equal shares",
        );
        assert!(!sm.is_certain());
    }

    #[test]
    fn test_subject_matter_specific_passes() {
        let sm = check_certainty_subject_matter(
            "100,000 ordinary shares in XYZ Ltd",
            PropertyType::Securities,
            "50% each to A and B",
        );
        assert!(sm.is_certain());
    }

    #[test]
    fn test_self_declaration_constitution() {
        let constitution = TrustConstitution::self_declaration();
        assert!(matches!(
            constitution.status,
            ConstitutionStatus::FullyConstituted
        ));
    }

    #[test]
    fn test_re_rose_saves_incomplete_transfer() {
        let constitution = TrustConstitution::transfer_to_trustees(
            PropertyType::Securities,
            false, // Not registered yet
            true,  // But all steps taken
        );
        assert!(matches!(
            constitution.status,
            ConstitutionStatus::SavedByReRose
        ));
    }

    #[test]
    fn test_three_certainties_all_satisfied() {
        let declaration = TrustDeclaration::builder()
            .settlor("John Smith")
            .property("100,000 ordinary shares in ABC Ltd")
            .property_type(PropertyType::Securities)
            .intention_words("I declare that I hold these shares on trust")
            .add_beneficiary("Mary Smith")
            .add_beneficiary("James Smith")
            .trust_type(TrustType::Fixed)
            .build()
            .expect("Valid declaration");

        let result = check_three_certainties(&declaration);
        assert!(result.is_ok());
        assert!(result.expect("Result").all_satisfied());
    }
}
