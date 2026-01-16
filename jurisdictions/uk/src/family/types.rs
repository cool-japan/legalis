//! UK Family Law - Core Types
//!
//! This module defines the core types for UK family law under:
//! - Marriage Act 1949
//! - Civil Partnership Act 2004
//! - Matrimonial Causes Act 1973
//! - Children Act 1989
//! - Family Law Act 1996
//! - Domestic Abuse Act 2021
//! - Divorce, Dissolution and Separation Act 2020

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

// ============================================================================
// Relationship Types
// ============================================================================

/// Type of legal relationship
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Marriage under Marriage Act 1949
    Marriage,
    /// Civil partnership under Civil Partnership Act 2004
    CivilPartnership,
    /// Cohabitation (no legal status but relevant for certain claims)
    Cohabitation,
}

/// Gender for marriage/civil partnership purposes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    /// Male
    Male,
    /// Female
    Female,
    /// Other/non-binary
    Other,
}

/// Marriage details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marriage {
    /// Date of marriage
    pub date: NaiveDate,
    /// Place of marriage
    pub place: String,
    /// Type of ceremony
    pub ceremony_type: CeremonyType,
    /// Spouse 1 details
    pub spouse1: PersonDetails,
    /// Spouse 2 details
    pub spouse2: PersonDetails,
    /// Is marriage valid?
    pub valid: bool,
    /// Grounds for invalidity (if any)
    pub invalidity_grounds: Option<MarriageInvalidityGround>,
}

/// Type of marriage ceremony
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CeremonyType {
    /// Civil ceremony at register office
    Civil,
    /// Religious ceremony (Church of England)
    ChurchOfEngland,
    /// Religious ceremony (other registered building)
    OtherReligious,
    /// Approved premises ceremony
    ApprovedPremises,
    /// Military chapel
    MilitaryChapel,
}

/// Grounds for marriage invalidity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarriageInvalidityGround {
    /// Void - parties within prohibited degrees (MA 1949 s.1)
    ProhibitedDegrees,
    /// Void - either party under 18 without consent (MA 1949 s.3)
    UnderageWithoutConsent,
    /// Void - either party already married (MA 1949 s.1)
    AlreadyMarried,
    /// Void - parties not male and female (pre-2014 marriages)
    SameSexPreLegalization,
    /// Void - non-compliance with formalities (MA 1949 s.49)
    FormalityDefect,
    /// Voidable - non-consummation (MCA 1973 s.12(a))
    NonConsummation,
    /// Voidable - lack of consent (MCA 1973 s.12(c))
    LackOfConsent,
    /// Voidable - mental disorder (MCA 1973 s.12(d))
    MentalDisorder,
    /// Voidable - pregnancy by another (MCA 1973 s.12(f))
    PregnancyByAnother,
    /// Voidable - gender recognition issue (MCA 1973 s.12(g)/(h))
    GenderRecognitionIssue,
}

/// Civil partnership details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CivilPartnership {
    /// Date of registration
    pub date: NaiveDate,
    /// Place of registration
    pub place: String,
    /// Partner 1 details
    pub partner1: PersonDetails,
    /// Partner 2 details
    pub partner2: PersonDetails,
    /// Is partnership valid?
    pub valid: bool,
    /// Grounds for invalidity (if any)
    pub invalidity_grounds: Option<CivilPartnershipInvalidityGround>,
}

/// Grounds for civil partnership invalidity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CivilPartnershipInvalidityGround {
    /// Void - within prohibited degrees (CPA 2004 Sch 1)
    ProhibitedDegrees,
    /// Void - either party under 18
    Underage,
    /// Void - either party already in civil partnership/marriage
    AlreadyInRelationship,
    /// Void - registration defects
    RegistrationDefect,
    /// Voidable - lack of consent
    LackOfConsent,
    /// Voidable - mental disorder
    MentalDisorder,
    /// Voidable - pregnancy by another
    PregnancyByAnother,
    /// Voidable - gender recognition issue
    GenderRecognitionIssue,
}

/// Person details for family law purposes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonDetails {
    /// Full name
    pub name: String,
    /// Date of birth
    pub date_of_birth: Option<NaiveDate>,
    /// Gender
    pub gender: Option<Gender>,
    /// Address
    pub address: Option<String>,
    /// Occupation
    pub occupation: Option<String>,
    /// National Insurance number (if relevant)
    pub ni_number: Option<String>,
}

/// Cohabitation details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cohabitation {
    /// Start date of cohabitation
    pub start_date: NaiveDate,
    /// End date (if ended)
    pub end_date: Option<NaiveDate>,
    /// Person 1 details
    pub person1: PersonDetails,
    /// Person 2 details
    pub person2: PersonDetails,
    /// Joint property ownership?
    pub joint_property: bool,
    /// Children of the relationship
    pub children: Vec<ChildDetails>,
    /// Cohabitation agreement exists?
    pub cohabitation_agreement: bool,
}

// ============================================================================
// Children Types
// ============================================================================

/// Child details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildDetails {
    /// Child's name
    pub name: String,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// Is child a child of the family?
    pub child_of_family: bool,
    /// Biological parent 1
    pub biological_parent1: Option<String>,
    /// Biological parent 2
    pub biological_parent2: Option<String>,
    /// Current residence
    pub residence: Option<String>,
    /// Special needs or disabilities
    pub special_needs: Option<String>,
    /// Child's ascertainable wishes (if age appropriate)
    pub wishes: Option<String>,
}

impl ChildDetails {
    /// Calculate child's age at a given date
    pub fn age_at(&self, date: NaiveDate) -> u32 {
        let years = date.year() - self.date_of_birth.year();
        if date.ordinal() < self.date_of_birth.ordinal() {
            (years - 1).max(0) as u32
        } else {
            years.max(0) as u32
        }
    }

    /// Is child a minor (under 18)?
    pub fn is_minor(&self, date: NaiveDate) -> bool {
        self.age_at(date) < 18
    }

    /// Is child Gillick competent age (typically 12+)?
    pub fn is_gillick_competent_age(&self, date: NaiveDate) -> bool {
        self.age_at(date) >= 12
    }
}

/// Parental responsibility holder
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentalResponsibilityHolder {
    /// Mother (automatic - CA 1989 s.2(1))
    Mother,
    /// Married father (automatic - CA 1989 s.2(1))
    MarriedFather,
    /// Unmarried father (by agreement, court order, or birth registration post-Dec 2003)
    UnmarriedFather {
        /// How PR was acquired
        acquisition_method: PRAcquisitionMethod,
    },
    /// Step-parent (by agreement or court order - CA 1989 s.4A)
    StepParent {
        /// How PR was acquired
        acquisition_method: PRAcquisitionMethod,
    },
    /// Civil partner (by agreement or court order - CA 1989 s.4A)
    CivilPartner {
        /// How PR was acquired
        acquisition_method: PRAcquisitionMethod,
    },
    /// Second female parent (HFEA 2008 ss.42-44)
    SecondFemaleParent {
        /// How PR was acquired
        acquisition_method: PRAcquisitionMethod,
    },
    /// Guardian (appointed under CA 1989 s.5)
    Guardian,
    /// Special guardian (CA 1989 s.14A)
    SpecialGuardian,
    /// Local authority (care order - CA 1989 s.33)
    LocalAuthority,
    /// Person with child arrangements order (lives with)
    ChildArrangementsOrderHolder,
}

/// Method of acquiring parental responsibility
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PRAcquisitionMethod {
    /// Automatic (mother, married father)
    Automatic,
    /// Birth registration (unmarried father post-1 Dec 2003)
    BirthRegistration,
    /// Parental responsibility agreement (CA 1989 s.4(1)(b))
    Agreement,
    /// Court order (CA 1989 s.4(1)(a))
    CourtOrder,
    /// Appointment as guardian
    GuardianshipAppointment,
    /// Special guardianship order
    SpecialGuardianshipOrder,
    /// Child arrangements order (lives with)
    ChildArrangementsOrder,
    /// Adoption order
    AdoptionOrder,
}

/// Parental responsibility details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentalResponsibility {
    /// Person holding PR
    pub holder: String,
    /// Type of PR holder
    pub holder_type: ParentalResponsibilityHolder,
    /// Date PR acquired
    pub acquired_date: Option<NaiveDate>,
    /// Is PR still in force?
    pub in_force: bool,
    /// Date PR ended (if applicable)
    pub ended_date: Option<NaiveDate>,
    /// Reason PR ended (if applicable)
    pub ended_reason: Option<PREndReason>,
}

/// Reason parental responsibility ended
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PREndReason {
    /// Child reached 18
    ChildReachedAdulthood,
    /// Court order discharged PR
    CourtOrderDischarged,
    /// Adoption order made
    AdoptionOrder,
    /// Death of holder
    DeathOfHolder,
    /// Death of child
    DeathOfChild,
}

// ============================================================================
// Court Order Types
// ============================================================================

/// Type of family court order
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyOrderType {
    /// Child arrangements order (CA 1989 s.8) - replaces residence/contact
    ChildArrangements,
    /// Specific issue order (CA 1989 s.8)
    SpecificIssue,
    /// Prohibited steps order (CA 1989 s.8)
    ProhibitedSteps,
    /// Special guardianship order (CA 1989 s.14A)
    SpecialGuardianship,
    /// Parental responsibility order (CA 1989 s.4)
    ParentalResponsibility,
    /// Non-molestation order (FLA 1996 s.42)
    NonMolestation,
    /// Occupation order (FLA 1996 s.33-41)
    Occupation,
    /// Forced marriage protection order (FLA 1996 Part 4A)
    ForcedMarriageProtection,
    /// Female genital mutilation protection order (FGM Act 2003)
    FGMProtection,
    /// Financial remedy order (MCA 1973)
    FinancialRemedy,
    /// Maintenance pending suit (MCA 1973 s.22)
    MaintenancePendingSuit,
    /// Periodical payments order (MCA 1973 s.23)
    PeriodicalPayments,
    /// Lump sum order (MCA 1973 s.23)
    LumpSum,
    /// Property adjustment order (MCA 1973 s.24)
    PropertyAdjustment,
    /// Pension sharing order (MCA 1973 s.24B)
    PensionSharing,
    /// Pension attachment order (MCA 1973 s.25B)
    PensionAttachment,
}

/// Child arrangements order details (CA 1989 s.8)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildArrangementsOrder {
    /// Child the order relates to
    pub child: String,
    /// Date order made
    pub order_date: NaiveDate,
    /// Person(s) child lives with
    pub lives_with: Vec<String>,
    /// Person(s) child spends time with
    pub spends_time_with: Vec<SpendingTimeArrangement>,
    /// Other contact arrangements
    pub other_contact: Vec<OtherContactArrangement>,
    /// Conditions attached to order
    pub conditions: Vec<String>,
    /// Duration (normally until 16, can extend to 18)
    pub end_date: Option<NaiveDate>,
    /// Activity directions (CAFCASS)
    pub activity_directions: Vec<String>,
}

/// Arrangement for spending time with child
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpendingTimeArrangement {
    /// Person child spends time with
    pub person: String,
    /// Frequency (e.g., "Every other weekend")
    pub frequency: String,
    /// Duration of each visit
    pub duration: Option<String>,
    /// Location
    pub location: Option<String>,
    /// Handover arrangements
    pub handover: Option<String>,
    /// Supervised contact required?
    pub supervised: bool,
    /// Supervision provider (if supervised)
    pub supervisor: Option<String>,
}

/// Other contact arrangement (indirect)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherContactArrangement {
    /// Person having contact
    pub person: String,
    /// Type of contact
    pub contact_type: ContactType,
    /// Frequency
    pub frequency: String,
}

/// Type of indirect contact
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactType {
    /// Telephone calls
    Telephone,
    /// Video calls
    Video,
    /// Letters/cards
    Letters,
    /// Email
    Email,
    /// Text messages
    Text,
    /// Social media
    SocialMedia,
    /// Other
    Other(String),
}

/// Specific issue order (CA 1989 s.8)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecificIssueOrder {
    /// Child the order relates to
    pub child: String,
    /// Date order made
    pub order_date: NaiveDate,
    /// Issue being determined
    pub issue: SpecificIssue,
    /// Decision made
    pub decision: String,
    /// Reasons
    pub reasons: String,
}

/// Types of specific issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecificIssue {
    /// Education (school choice)
    Education,
    /// Medical treatment
    MedicalTreatment,
    /// Religion
    Religion,
    /// Change of name
    NameChange,
    /// Relocation within UK
    InternalRelocation,
    /// Relocation outside UK
    InternationalRelocation,
    /// Passport application
    Passport,
    /// Vaccination
    Vaccination,
    /// Circumcision
    Circumcision,
    /// Other
    Other(String),
}

/// Prohibited steps order (CA 1989 s.8)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProhibitedStepsOrder {
    /// Child the order relates to
    pub child: String,
    /// Date order made
    pub order_date: NaiveDate,
    /// Person prohibited
    pub prohibited_person: String,
    /// Steps prohibited
    pub prohibited_steps: Vec<ProhibitedStep>,
    /// Duration
    pub end_date: Option<NaiveDate>,
}

/// Types of prohibited steps
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProhibitedStep {
    /// Removing child from UK
    RemovalFromUK,
    /// Removing child from school
    RemovalFromSchool,
    /// Changing child's name
    ChangingName,
    /// Medical treatment without consent
    MedicalTreatment,
    /// Contacting child
    Contact,
    /// Coming within specified distance
    ExclusionZone,
    /// Other
    Other(String),
}

// ============================================================================
// Financial Types
// ============================================================================

/// Financial position of a party
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialPosition {
    /// Party name
    pub party: String,
    /// Income (annual)
    pub income: f64,
    /// Earning capacity
    pub earning_capacity: Option<f64>,
    /// Capital assets
    pub capital: f64,
    /// Pension value (CETV)
    pub pension_cetv: f64,
    /// Debts
    pub debts: f64,
    /// Housing needs
    pub housing_needs: HousingNeeds,
    /// Standard of living
    pub standard_of_living: Option<String>,
    /// Age
    pub age: u32,
    /// Health
    pub health: Option<String>,
    /// Contributions (financial and non-financial)
    pub contributions: Vec<Contribution>,
    /// Conduct (if relevant - rare)
    pub conduct: Option<String>,
    /// Benefits lost (e.g., pension widow's benefits)
    pub benefits_lost: Vec<String>,
}

/// Housing needs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HousingNeeds {
    /// Current housing
    pub current_housing: String,
    /// Minimum housing requirement
    pub minimum_requirement: String,
    /// Housing cost estimate
    pub cost_estimate: f64,
    /// Children's housing needs
    pub children_needs: Option<String>,
}

/// Type of contribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contribution {
    /// Type of contribution
    pub contribution_type: ContributionType,
    /// Description
    pub description: String,
    /// Value (if quantifiable)
    pub value: Option<f64>,
}

/// Types of contribution under MCA 1973 s.25(2)(f)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributionType {
    /// Direct financial contribution
    FinancialDirect,
    /// Indirect financial contribution (e.g., paying bills)
    FinancialIndirect,
    /// Homemaking
    Homemaking,
    /// Childcare
    Childcare,
    /// Career sacrifice
    CareerSacrifice,
    /// Business contribution
    BusinessContribution,
    /// Inheritance brought to marriage
    Inheritance,
    /// Pre-marital assets
    PreMaritalAssets,
    /// Post-separation contributions
    PostSeparation,
}

/// Asset for financial proceedings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    /// Asset description
    pub description: String,
    /// Asset type
    pub asset_type: AssetType,
    /// Current value
    pub value: f64,
    /// Owner(s)
    pub owner: AssetOwnership,
    /// Is it matrimonial property?
    pub matrimonial: bool,
    /// Is it non-matrimonial (inherited, pre-marital)?
    pub non_matrimonial: bool,
    /// Date acquired
    pub acquired_date: Option<NaiveDate>,
    /// Encumbrances (mortgages, charges)
    pub encumbrances: f64,
}

/// Asset types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    /// Family home
    FamilyHome,
    /// Other property
    Property,
    /// Bank accounts
    BankAccount,
    /// Investments
    Investments,
    /// Pension
    Pension,
    /// Business interest
    Business,
    /// Vehicle
    Vehicle,
    /// Personal possessions
    PersonalPossessions,
    /// Crypto assets
    CryptoAssets,
    /// Other
    Other(String),
}

/// Asset ownership
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetOwnership {
    /// Sole ownership
    Sole(String),
    /// Joint ownership
    Joint,
    /// Owned by third party (e.g., trust, company)
    ThirdParty(String),
}

// ============================================================================
// Domestic Abuse Types
// ============================================================================

/// Type of domestic abuse (Domestic Abuse Act 2021 s.1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbuseType {
    /// Physical abuse
    Physical,
    /// Emotional abuse
    Emotional,
    /// Psychological abuse
    Psychological,
    /// Sexual abuse
    Sexual,
    /// Economic abuse (DAA 2021 s.1(4))
    Economic,
    /// Coercive control (Serious Crime Act 2015 s.76)
    CoerciveControl,
    /// Controlling behaviour
    Controlling,
    /// Threats
    Threats,
    /// Harassment and stalking
    HarassmentStalking,
}

/// Relationship between victim and perpetrator for domestic abuse purposes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssociatedPersonRelationship {
    /// Married/civil partners
    SpouseOrCivilPartner,
    /// Former spouse/civil partner
    FormerSpouseOrCivilPartner,
    /// Cohabitants
    Cohabitant,
    /// Former cohabitants
    FormerCohabitant,
    /// Same household (not tenant/lodger)
    SameHousehold,
    /// Relatives
    Relative,
    /// Engaged/agreed to civil partnership
    EngagedOrAgreed,
    /// Intimate personal relationship of significant duration
    IntimateRelationship,
    /// Parents of same child
    ParentsOfChild,
    /// Party to same family proceedings
    FamilyProceedings,
}

/// Non-molestation order details (FLA 1996 s.42)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonMolestationOrder {
    /// Applicant
    pub applicant: String,
    /// Respondent
    pub respondent: String,
    /// Date order made
    pub order_date: NaiveDate,
    /// Relationship to respondent
    pub relationship: AssociatedPersonRelationship,
    /// Prohibited conduct
    pub prohibited_conduct: Vec<String>,
    /// Persons protected (applicant and any relevant child)
    pub protected_persons: Vec<String>,
    /// Duration
    pub end_date: Option<NaiveDate>,
    /// Power of arrest attached?
    pub power_of_arrest: bool,
    /// Made without notice (ex parte)?
    pub without_notice: bool,
    /// Undertaking accepted instead?
    pub undertaking: bool,
}

/// Occupation order applicant category (FLA 1996 ss.33-38)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupationOrderCategory {
    /// S.33 - Entitled applicant (has property rights)
    Section33Entitled,
    /// S.35 - Former spouse/civil partner, no existing right
    Section35FormerSpouse,
    /// S.36 - Cohabitant/former cohabitant, no existing right
    Section36Cohabitant,
    /// S.37 - Neither spouse entitled
    Section37NeitherEntitled,
    /// S.38 - Neither cohabitant entitled
    Section38NeitherEntitled,
}

/// Occupation order details (FLA 1996 Part IV)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OccupationOrder {
    /// Applicant
    pub applicant: String,
    /// Respondent
    pub respondent: String,
    /// Property address
    pub property: String,
    /// Date order made
    pub order_date: NaiveDate,
    /// Applicant category
    pub category: OccupationOrderCategory,
    /// Orders made
    pub orders: Vec<OccupationOrderProvision>,
    /// Duration
    pub end_date: Option<NaiveDate>,
    /// Power of arrest attached?
    pub power_of_arrest: bool,
    /// Made without notice?
    pub without_notice: bool,
}

/// Provisions in occupation order
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupationOrderProvision {
    /// Enforce applicant's entitlement to remain
    EnforceEntitlement,
    /// Require respondent to permit applicant to enter/remain
    RequirePermitEntry,
    /// Regulate occupation by either/both parties
    RegulateOccupation,
    /// Prohibit respondent from occupying
    ProhibitOccupation,
    /// Require respondent to leave
    RequireLeave,
    /// Exclude respondent from defined area
    ExclusionZone,
    /// Order respondent to maintain property
    MaintenanceOfProperty,
    /// Order payment of rent/mortgage
    PaymentOfRent,
    /// Grant applicant possession/use of furniture
    FurnitureUse,
}

/// Balance of harm test result (FLA 1996 s.33(7))
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceOfHarmTest {
    /// Harm to applicant/child if order not made
    pub harm_if_not_made: Vec<HarmFactor>,
    /// Harm to respondent/child if order made
    pub harm_if_made: Vec<HarmFactor>,
    /// Does applicant/child face significant harm?
    pub significant_harm_to_applicant: bool,
    /// Would harm to applicant exceed harm to respondent?
    pub balance_favours_applicant: bool,
    /// Test outcome
    pub outcome: BalanceOfHarmOutcome,
}

/// Harm factor in balance of harm test
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmFactor {
    /// Person affected
    pub person: String,
    /// Type of harm
    pub harm_type: HarmType,
    /// Description
    pub description: String,
    /// Severity
    pub severity: HarmSeverity,
}

/// Type of harm
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmType {
    /// Physical harm
    Physical,
    /// Psychological harm
    Psychological,
    /// Emotional harm
    Emotional,
    /// Financial harm
    Financial,
    /// Harm to health
    Health,
    /// Impairment of development (child)
    ImpairmentOfDevelopment,
}

/// Severity of harm
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmSeverity {
    /// Minor
    Minor,
    /// Moderate
    Moderate,
    /// Serious
    Serious,
    /// Severe
    Severe,
}

/// Balance of harm test outcome
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceOfHarmOutcome {
    /// Order must be made (significant harm, balance favours)
    MustMakeOrder,
    /// Order may be made (discretionary)
    MayMakeOrder,
    /// Order should not be made
    ShouldNotMakeOrder,
}

// ============================================================================
// Court and Procedure Types
// ============================================================================

/// Family court level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyCourtLevel {
    /// Family Court (default)
    FamilyCourt,
    /// High Court Family Division
    HighCourtFamilyDivision,
    /// Court of Appeal
    CourtOfAppeal,
    /// Supreme Court
    SupremeCourt,
}

/// Family proceedings type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyProceedingsType {
    /// Divorce/dissolution
    DivorceOrDissolution,
    /// Children Act private law
    ChildrenPrivateLaw,
    /// Children Act public law
    ChildrenPublicLaw,
    /// Financial remedies
    FinancialRemedies,
    /// Domestic abuse (injunctions)
    DomesticAbuse,
    /// Adoption
    Adoption,
    /// International (Hague Convention)
    International,
    /// Forced marriage protection
    ForcedMarriageProtection,
    /// FGM protection
    FGMProtection,
}

/// Party to family proceedings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FamilyProceedingsParty {
    /// Party name
    pub name: String,
    /// Party role
    pub role: PartyRole,
    /// Legal representation
    pub representation: Representation,
    /// Vulnerable party?
    pub vulnerable: bool,
    /// Special measures required?
    pub special_measures: Vec<SpecialMeasure>,
}

/// Role in family proceedings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyRole {
    /// Applicant/petitioner
    Applicant,
    /// Respondent
    Respondent,
    /// Intervenor
    Intervenor,
    /// Child (joined as party)
    Child,
    /// Children's guardian (CAFCASS)
    ChildrensGuardian,
    /// Local authority
    LocalAuthority,
}

/// Legal representation status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Representation {
    /// Legally represented
    Represented,
    /// Litigant in person
    LitigantInPerson,
    /// McKenzie friend
    McKenzieFriend,
    /// Direct access barrister
    DirectAccess,
}

/// Special measures for vulnerable parties
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialMeasure {
    /// Screens
    Screens,
    /// Separate waiting areas
    SeparateWaiting,
    /// Video link
    VideoLink,
    /// Separate entrance/exit
    SeparateEntrance,
    /// Intermediary
    Intermediary,
    /// Ground rules hearing
    GroundRulesHearing,
    /// Breaks during evidence
    RegularBreaks,
}
