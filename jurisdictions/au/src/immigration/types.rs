//! Core immigration and citizenship types
//!
//! Types for the Migration Act 1958 (Cth), Migration Regulations 1994,
//! and Australian Citizenship Act 2007 (Cth).

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Visa category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisaCategory {
    /// Skilled migration (subclasses 189, 190, 491, etc.)
    Skilled,
    /// Family stream (partner, parent, child, etc.)
    Family,
    /// Employer sponsored (subclasses 482, 494, etc.)
    EmployerSponsored,
    /// Business and investment (subclasses 188, 888, etc.)
    BusinessInvestment,
    /// Student (subclass 500)
    Student,
    /// Visitor (subclasses 600, 601, 651)
    Visitor,
    /// Working holiday (subclasses 417, 462)
    WorkingHoliday,
    /// Humanitarian/refugee (subclasses 200-204, 866)
    Humanitarian,
    /// Bridging (subclasses A-E)
    Bridging,
    /// Special (various)
    Special,
}

/// Common visa subclasses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisaSubclass {
    // Skilled
    /// Skilled Independent (subclass 189)
    SkilledIndependent189,
    /// Skilled Nominated (subclass 190)
    SkilledNominated190,
    /// Skilled Work Regional (subclass 491)
    SkilledWorkRegional491,
    /// Permanent Residence (Skilled Regional) (subclass 191)
    PermanentResidenceSkilledRegional191,

    // Employer Sponsored
    /// Temporary Skill Shortage (subclass 482)
    TemporarySkillShortage482,
    /// Skilled Employer Sponsored Regional (subclass 494)
    SkilledEmployerSponsoredRegional494,
    /// Employer Nomination Scheme (subclass 186)
    EmployerNominationScheme186,

    // Family
    /// Partner (subclass 820/801)
    Partner820801,
    /// Partner Offshore (subclass 309/100)
    PartnerOffshore309100,
    /// Parent (subclass 103)
    Parent103,
    /// Contributory Parent (subclass 143)
    ContributoryParent143,
    /// Child (subclass 101)
    Child101,

    // Business/Investment
    /// Business Innovation and Investment (Provisional) (subclass 188)
    BusinessInnovation188,
    /// Business Innovation and Investment (Permanent) (subclass 888)
    BusinessInnovation888,
    /// Global Talent (subclass 858)
    GlobalTalent858,

    // Student
    /// Student (subclass 500)
    Student500,
    /// Student Guardian (subclass 590)
    StudentGuardian590,

    // Visitor
    /// Visitor (subclass 600)
    Visitor600,
    /// Electronic Travel Authority (subclass 601)
    Eta601,
    /// eVisitor (subclass 651)
    EVisitor651,

    // Working Holiday
    /// Working Holiday (subclass 417)
    WorkingHoliday417,
    /// Work and Holiday (subclass 462)
    WorkAndHoliday462,

    // Humanitarian
    /// Refugee (subclass 200)
    Refugee200,
    /// Protection (subclass 866)
    Protection866,

    // Bridging
    /// Bridging A
    BridgingA010,
    /// Bridging B
    BridgingB020,
    /// Bridging C
    BridgingC030,
    /// Bridging D
    BridgingD040,
    /// Bridging E
    BridgingE050,

    // Special
    /// Other subclass
    Other(u16),
}

impl VisaSubclass {
    /// Get visa subclass number
    pub fn subclass_number(&self) -> u16 {
        match self {
            VisaSubclass::SkilledIndependent189 => 189,
            VisaSubclass::SkilledNominated190 => 190,
            VisaSubclass::SkilledWorkRegional491 => 491,
            VisaSubclass::PermanentResidenceSkilledRegional191 => 191,
            VisaSubclass::TemporarySkillShortage482 => 482,
            VisaSubclass::SkilledEmployerSponsoredRegional494 => 494,
            VisaSubclass::EmployerNominationScheme186 => 186,
            VisaSubclass::Partner820801 => 820,
            VisaSubclass::PartnerOffshore309100 => 309,
            VisaSubclass::Parent103 => 103,
            VisaSubclass::ContributoryParent143 => 143,
            VisaSubclass::Child101 => 101,
            VisaSubclass::BusinessInnovation188 => 188,
            VisaSubclass::BusinessInnovation888 => 888,
            VisaSubclass::GlobalTalent858 => 858,
            VisaSubclass::Student500 => 500,
            VisaSubclass::StudentGuardian590 => 590,
            VisaSubclass::Visitor600 => 600,
            VisaSubclass::Eta601 => 601,
            VisaSubclass::EVisitor651 => 651,
            VisaSubclass::WorkingHoliday417 => 417,
            VisaSubclass::WorkAndHoliday462 => 462,
            VisaSubclass::Refugee200 => 200,
            VisaSubclass::Protection866 => 866,
            VisaSubclass::BridgingA010 => 10,
            VisaSubclass::BridgingB020 => 20,
            VisaSubclass::BridgingC030 => 30,
            VisaSubclass::BridgingD040 => 40,
            VisaSubclass::BridgingE050 => 50,
            VisaSubclass::Other(n) => *n,
        }
    }

    /// Whether this visa grants permanent residence
    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            VisaSubclass::SkilledIndependent189
                | VisaSubclass::SkilledNominated190
                | VisaSubclass::PermanentResidenceSkilledRegional191
                | VisaSubclass::EmployerNominationScheme186
                | VisaSubclass::Partner820801
                | VisaSubclass::PartnerOffshore309100
                | VisaSubclass::Parent103
                | VisaSubclass::ContributoryParent143
                | VisaSubclass::Child101
                | VisaSubclass::BusinessInnovation888
                | VisaSubclass::GlobalTalent858
                | VisaSubclass::Refugee200
                | VisaSubclass::Protection866
        )
    }

    /// Whether this visa allows work
    pub fn allows_work(&self) -> bool {
        matches!(
            self,
            VisaSubclass::SkilledIndependent189
                | VisaSubclass::SkilledNominated190
                | VisaSubclass::SkilledWorkRegional491
                | VisaSubclass::PermanentResidenceSkilledRegional191
                | VisaSubclass::TemporarySkillShortage482
                | VisaSubclass::SkilledEmployerSponsoredRegional494
                | VisaSubclass::EmployerNominationScheme186
                | VisaSubclass::Partner820801
                | VisaSubclass::PartnerOffshore309100
                | VisaSubclass::BusinessInnovation188
                | VisaSubclass::BusinessInnovation888
                | VisaSubclass::GlobalTalent858
                | VisaSubclass::WorkingHoliday417
                | VisaSubclass::WorkAndHoliday462
                | VisaSubclass::Refugee200
                | VisaSubclass::Protection866
        )
    }

    /// Whether this visa is employer sponsored
    pub fn is_employer_sponsored(&self) -> bool {
        matches!(
            self,
            VisaSubclass::TemporarySkillShortage482
                | VisaSubclass::SkilledEmployerSponsoredRegional494
                | VisaSubclass::EmployerNominationScheme186
        )
    }

    /// Category of visa
    pub fn category(&self) -> VisaCategory {
        match self {
            VisaSubclass::SkilledIndependent189
            | VisaSubclass::SkilledNominated190
            | VisaSubclass::SkilledWorkRegional491
            | VisaSubclass::PermanentResidenceSkilledRegional191 => VisaCategory::Skilled,

            VisaSubclass::TemporarySkillShortage482
            | VisaSubclass::SkilledEmployerSponsoredRegional494
            | VisaSubclass::EmployerNominationScheme186 => VisaCategory::EmployerSponsored,

            VisaSubclass::Partner820801
            | VisaSubclass::PartnerOffshore309100
            | VisaSubclass::Parent103
            | VisaSubclass::ContributoryParent143
            | VisaSubclass::Child101 => VisaCategory::Family,

            VisaSubclass::BusinessInnovation188
            | VisaSubclass::BusinessInnovation888
            | VisaSubclass::GlobalTalent858 => VisaCategory::BusinessInvestment,

            VisaSubclass::Student500 | VisaSubclass::StudentGuardian590 => VisaCategory::Student,

            VisaSubclass::Visitor600 | VisaSubclass::Eta601 | VisaSubclass::EVisitor651 => {
                VisaCategory::Visitor
            }

            VisaSubclass::WorkingHoliday417 | VisaSubclass::WorkAndHoliday462 => {
                VisaCategory::WorkingHoliday
            }

            VisaSubclass::Refugee200 | VisaSubclass::Protection866 => VisaCategory::Humanitarian,

            VisaSubclass::BridgingA010
            | VisaSubclass::BridgingB020
            | VisaSubclass::BridgingC030
            | VisaSubclass::BridgingD040
            | VisaSubclass::BridgingE050 => VisaCategory::Bridging,

            VisaSubclass::Other(_) => VisaCategory::Special,
        }
    }
}

/// Visa status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisaStatus {
    /// Visa application lodged
    Applied,
    /// Visa granted
    Granted,
    /// Visa refused
    Refused,
    /// Visa cancelled
    Cancelled,
    /// Visa expired
    Expired,
    /// Visa ceased
    Ceased,
}

/// Visa condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisaCondition {
    /// 8101 - No work
    NoWork8101,
    /// 8104 - Work limitation (max 48 hours per fortnight during session)
    WorkLimitation8104,
    /// 8105 - Work limitation for students
    WorkLimitationStudent8105,
    /// 8106 - No change of course
    NoChangeCourse8106,
    /// 8107 - Employer restriction
    EmployerRestriction8107,
    /// 8201 - Maximum 3 months study
    MaxStudyThreeMonths8201,
    /// 8303 - No criminal conduct
    NoCriminalConduct8303,
    /// 8501 - Maintain health insurance
    MaintainHealthInsurance8501,
    /// 8503 - No further stay
    NoFurtherStay8503,
    /// 8504 - No further visa except protection
    NoFurtherVisaExceptProtection8504,
    /// 8533 - Notify of address changes
    NotifyAddressChanges8533,
    /// 8534 - No further stay (student must leave)
    NoFurtherStayStudent8534,
    /// 8535 - Employer must be sponsoring
    EmployerMustBeSponsoring8535,
    /// 8564 - Must work in occupation/location sponsored for
    MustWorkInSponsoredOccupation8564,
    /// 8578 - Must notify sponsorship end
    MustNotifySponsorshipEnd8578,
    /// 8607 - Regional work requirement
    RegionalWorkRequirement8607,
    /// Other condition
    Other(u16),
}

impl VisaCondition {
    /// Get condition number
    pub fn condition_number(&self) -> u16 {
        match self {
            VisaCondition::NoWork8101 => 8101,
            VisaCondition::WorkLimitation8104 => 8104,
            VisaCondition::WorkLimitationStudent8105 => 8105,
            VisaCondition::NoChangeCourse8106 => 8106,
            VisaCondition::EmployerRestriction8107 => 8107,
            VisaCondition::MaxStudyThreeMonths8201 => 8201,
            VisaCondition::NoCriminalConduct8303 => 8303,
            VisaCondition::MaintainHealthInsurance8501 => 8501,
            VisaCondition::NoFurtherStay8503 => 8503,
            VisaCondition::NoFurtherVisaExceptProtection8504 => 8504,
            VisaCondition::NotifyAddressChanges8533 => 8533,
            VisaCondition::NoFurtherStayStudent8534 => 8534,
            VisaCondition::EmployerMustBeSponsoring8535 => 8535,
            VisaCondition::MustWorkInSponsoredOccupation8564 => 8564,
            VisaCondition::MustNotifySponsorshipEnd8578 => 8578,
            VisaCondition::RegionalWorkRequirement8607 => 8607,
            VisaCondition::Other(n) => *n,
        }
    }
}

/// Visa application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisaApplication {
    /// Application ID
    pub application_id: String,
    /// Applicant name
    pub applicant_name: String,
    /// Visa subclass applied for
    pub visa_subclass: VisaSubclass,
    /// Application date
    pub application_date: NaiveDate,
    /// Status
    pub status: VisaStatus,
    /// Decision date (if decided)
    pub decision_date: Option<NaiveDate>,
    /// Stream (if applicable)
    pub stream: Option<String>,
    /// Points (for skilled visas)
    pub points_claimed: Option<u32>,
    /// Sponsor (if sponsored visa)
    pub sponsor: Option<String>,
}

/// Visa holder
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisaHolder {
    /// Name
    pub name: String,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// Passport number
    pub passport_number: String,
    /// Passport country
    pub passport_country: String,
    /// Current visa subclass
    pub visa_subclass: VisaSubclass,
    /// Visa grant date
    pub visa_grant_date: NaiveDate,
    /// Visa expiry date (None if permanent)
    pub visa_expiry_date: Option<NaiveDate>,
    /// Visa conditions
    pub conditions: Vec<VisaCondition>,
    /// Immigration status
    pub status: ImmigrationStatus,
}

/// Immigration status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImmigrationStatus {
    /// Australian citizen
    Citizen,
    /// Permanent resident
    PermanentResident,
    /// Lawful non-citizen with valid temporary visa
    LawfulNonCitizenTemporary,
    /// Unlawful non-citizen (no valid visa)
    UnlawfulNonCitizen,
    /// Bridging visa holder
    BridgingVisaHolder,
}

/// Points test category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointsCategory {
    /// Age (25-32 = 30 points, etc.)
    Age,
    /// English language ability
    EnglishLanguage,
    /// Overseas skilled employment
    OverseasEmployment,
    /// Australian skilled employment
    AustralianEmployment,
    /// Educational qualifications
    Education,
    /// Australian study requirement
    AustralianStudy,
    /// Specialist education
    SpecialistEducation,
    /// Credentialled community language
    CommunityLanguage,
    /// Professional Year
    ProfessionalYear,
    /// Partner skills
    PartnerSkills,
    /// State/territory nomination
    StateNomination,
    /// Regional nomination
    RegionalNomination,
}

/// Points test result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointsTestResult {
    /// Age points
    pub age_points: u32,
    /// English language points
    pub english_points: u32,
    /// Overseas employment points
    pub overseas_employment_points: u32,
    /// Australian employment points
    pub australian_employment_points: u32,
    /// Education points
    pub education_points: u32,
    /// Australian study points
    pub australian_study_points: u32,
    /// Specialist education points
    pub specialist_education_points: u32,
    /// Community language points
    pub community_language_points: u32,
    /// Professional year points
    pub professional_year_points: u32,
    /// Partner points (skills or single applicant)
    pub partner_points: u32,
    /// Nomination points (state/regional)
    pub nomination_points: u32,
    /// Total points
    pub total_points: u32,
    /// Pass mark (currently 65)
    pub pass_mark: u32,
    /// Whether passed
    pub passed: bool,
}

impl PointsTestResult {
    /// Calculate total points
    pub fn calculate_total(&self) -> u32 {
        self.age_points
            + self.english_points
            + self.overseas_employment_points
            + self.australian_employment_points
            + self.education_points
            + self.australian_study_points
            + self.specialist_education_points
            + self.community_language_points
            + self.professional_year_points
            + self.partner_points
            + self.nomination_points
    }
}

/// Citizenship stream
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitizenshipStream {
    /// By conferral (general eligibility)
    ByConferral,
    /// By descent (parent is citizen)
    ByDescent,
    /// By birth (born in Australia)
    ByBirth,
    /// Resumption (former citizen)
    Resumption,
}

/// Citizenship application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CitizenshipApplication {
    /// Application ID
    pub application_id: String,
    /// Applicant name
    pub applicant_name: String,
    /// Stream
    pub stream: CitizenshipStream,
    /// Application date
    pub application_date: NaiveDate,
    /// Status
    pub status: CitizenshipApplicationStatus,
    /// Permanent resident since
    pub permanent_resident_since: Option<NaiveDate>,
    /// Days in Australia during qualifying period
    pub days_in_australia: Option<u32>,
    /// Days absent during qualifying period
    pub days_absent: Option<u32>,
    /// Test passed
    pub test_passed: Option<bool>,
    /// Decision date
    pub decision_date: Option<NaiveDate>,
}

/// Citizenship application status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitizenshipApplicationStatus {
    /// Application lodged
    Lodged,
    /// Awaiting interview
    AwaitingInterview,
    /// Awaiting test
    AwaitingTest,
    /// Approved pending ceremony
    ApprovedPendingCeremony,
    /// Approved
    Approved,
    /// Refused
    Refused,
    /// Withdrawn
    Withdrawn,
}

/// Residence requirement for citizenship
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResidenceRequirement {
    /// Start of qualifying period (4 years before application)
    pub qualifying_period_start: NaiveDate,
    /// End of qualifying period (application date)
    pub qualifying_period_end: NaiveDate,
    /// Total days in qualifying period
    pub total_days_in_period: u32,
    /// Days physically present in Australia
    pub days_present: u32,
    /// Days absent from Australia
    pub days_absent: u32,
    /// Days in 12 months before application
    pub days_in_last_twelve_months: u32,
    /// Days absent in 12 months before application
    pub days_absent_in_last_twelve_months: u32,
    /// Whether held permanent visa throughout
    pub permanent_visa_throughout: bool,
    /// Whether requirement satisfied
    pub requirement_met: bool,
}

/// Character test ground (s.501(6))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterTestGround {
    /// s.501(6)(a) - Substantial criminal record
    SubstantialCriminalRecord,
    /// s.501(6)(b) - Association with criminal group/conduct
    CriminalAssociation,
    /// s.501(6)(ba) - Been/are a member of a group involved in criminal conduct
    CriminalGroupMembership,
    /// s.501(6)(c) - Past and present criminal/general conduct
    PastCriminalConduct,
    /// s.501(6)(d) - Risk of conduct specified in s.501(6)(c)
    RiskOfCriminalConduct,
    /// s.501(6)(e) - Harassment, molestation, intimidation, stalking
    HarassmentStalking,
    /// s.501(6)(f) - Vilification of segment of Australian community
    Vilification,
    /// s.501(6)(g) - Risk of vilification
    RiskOfVilification,
    /// s.501(6)(h) - Adverse security assessment
    AdverseSecurityAssessment,
    /// s.501(6)(ha) - Qualified security assessment
    QualifiedSecurityAssessment,
    /// ASIO assessment
    AsioAssessment,
}

/// Sponsor type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SponsorType {
    /// Standard Business Sponsor
    StandardBusinessSponsor,
    /// Accredited Sponsor
    AccreditedSponsor,
    /// Labour Agreement sponsor
    LabourAgreementSponsor,
    /// Family sponsor
    FamilySponsor,
    /// Community Support Program sponsor
    CommunitySupport,
}

/// Sponsor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sponsor {
    /// Sponsor name/business name
    pub name: String,
    /// ABN (if business)
    pub abn: Option<String>,
    /// Sponsor type
    pub sponsor_type: SponsorType,
    /// Approval date
    pub approval_date: NaiveDate,
    /// Expiry date
    pub expiry_date: Option<NaiveDate>,
    /// Number of sponsored positions approved
    pub approved_positions: Option<u32>,
    /// Compliance history
    pub compliance_history: SponsorComplianceHistory,
}

/// Sponsor compliance history
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SponsorComplianceHistory {
    /// Good compliance
    Good,
    /// Minor concerns
    MinorConcerns,
    /// Significant concerns
    SignificantConcerns,
    /// Under monitoring
    UnderMonitoring,
    /// Barred
    Barred,
}

/// Skilled occupation list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkilledOccupationList {
    /// Medium and Long-term Strategic Skills List (MLTSSL)
    Mltssl,
    /// Short-term Skilled Occupation List (STSOL)
    Stsol,
    /// Regional Occupation List (ROL)
    Rol,
    /// Priority Migration Skilled Occupation List (PMSOL)
    Pmsol,
}

/// Occupation assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OccupationAssessment {
    /// ANZSCO code
    pub anzsco_code: String,
    /// Occupation title
    pub occupation_title: String,
    /// Assessing authority
    pub assessing_authority: String,
    /// Assessment date
    pub assessment_date: NaiveDate,
    /// Valid until
    pub valid_until: NaiveDate,
    /// Outcome
    pub outcome: AssessmentOutcome,
    /// Skilled employment claimed
    pub skilled_employment_years: Option<f32>,
    /// Which occupation list
    pub occupation_list: SkilledOccupationList,
}

/// Assessment outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssessmentOutcome {
    /// Suitable
    Suitable,
    /// Suitable with conditions
    SuitableWithConditions,
    /// Not suitable
    NotSuitable,
    /// Pending
    Pending,
}

/// English language test
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnglishLanguageTest {
    /// IELTS
    Ielts,
    /// PTE Academic
    PteAcademic,
    /// TOEFL iBT
    ToeflIbt,
    /// Cambridge C1 Advanced
    CambridgeC1,
    /// OET
    Oet,
}

/// English language level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnglishLanguageLevel {
    /// Competent English (IELTS 6)
    Competent,
    /// Proficient English (IELTS 7)
    Proficient,
    /// Superior English (IELTS 8)
    Superior,
}

/// English test result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnglishTestResult {
    /// Test type
    pub test_type: EnglishLanguageTest,
    /// Test date
    pub test_date: NaiveDate,
    /// Listening score
    pub listening: f32,
    /// Reading score
    pub reading: f32,
    /// Writing score
    pub writing: f32,
    /// Speaking score
    pub speaking: f32,
    /// Overall score
    pub overall: f32,
    /// Level achieved
    pub level: EnglishLanguageLevel,
}

impl EnglishTestResult {
    /// Check if result meets competent English (minimum for most skilled visas)
    pub fn meets_competent(&self) -> bool {
        match self.test_type {
            EnglishLanguageTest::Ielts => {
                self.listening >= 6.0
                    && self.reading >= 6.0
                    && self.writing >= 6.0
                    && self.speaking >= 6.0
            }
            EnglishLanguageTest::PteAcademic => {
                self.listening >= 50.0
                    && self.reading >= 50.0
                    && self.writing >= 50.0
                    && self.speaking >= 50.0
            }
            _ => false, // Simplified - would need full mapping
        }
    }

    /// Check if result meets proficient English (20 points)
    pub fn meets_proficient(&self) -> bool {
        match self.test_type {
            EnglishLanguageTest::Ielts => {
                self.listening >= 7.0
                    && self.reading >= 7.0
                    && self.writing >= 7.0
                    && self.speaking >= 7.0
            }
            EnglishLanguageTest::PteAcademic => {
                self.listening >= 65.0
                    && self.reading >= 65.0
                    && self.writing >= 65.0
                    && self.speaking >= 65.0
            }
            _ => false,
        }
    }

    /// Check if result meets superior English (20 points)
    pub fn meets_superior(&self) -> bool {
        match self.test_type {
            EnglishLanguageTest::Ielts => {
                self.listening >= 8.0
                    && self.reading >= 8.0
                    && self.writing >= 8.0
                    && self.speaking >= 8.0
            }
            EnglishLanguageTest::PteAcademic => {
                self.listening >= 79.0
                    && self.reading >= 79.0
                    && self.writing >= 79.0
                    && self.speaking >= 79.0
            }
            _ => false,
        }
    }
}

/// Migration zone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MigrationZone {
    /// Migration zone (mainland Australia, Tasmania)
    MigrationZone,
    /// Excised offshore place (Christmas Island, Cocos Islands, etc.)
    ExcisedOffshorePlace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visa_subclass_number() {
        assert_eq!(VisaSubclass::SkilledIndependent189.subclass_number(), 189);
        assert_eq!(VisaSubclass::Student500.subclass_number(), 500);
        assert_eq!(VisaSubclass::Other(999).subclass_number(), 999);
    }

    #[test]
    fn test_visa_is_permanent() {
        assert!(VisaSubclass::SkilledIndependent189.is_permanent());
        assert!(!VisaSubclass::Student500.is_permanent());
        assert!(!VisaSubclass::TemporarySkillShortage482.is_permanent());
    }

    #[test]
    fn test_visa_allows_work() {
        assert!(VisaSubclass::SkilledIndependent189.allows_work());
        assert!(!VisaSubclass::Student500.allows_work()); // Limited work rights
        assert!(!VisaSubclass::Visitor600.allows_work());
    }

    #[test]
    fn test_visa_category() {
        assert_eq!(
            VisaSubclass::SkilledIndependent189.category(),
            VisaCategory::Skilled
        );
        assert_eq!(VisaSubclass::Partner820801.category(), VisaCategory::Family);
        assert_eq!(VisaSubclass::Student500.category(), VisaCategory::Student);
    }

    #[test]
    fn test_visa_condition_number() {
        assert_eq!(VisaCondition::NoWork8101.condition_number(), 8101);
        assert_eq!(
            VisaCondition::MaintainHealthInsurance8501.condition_number(),
            8501
        );
    }

    #[test]
    fn test_points_test_calculation() {
        let result = PointsTestResult {
            age_points: 30,
            english_points: 20,
            overseas_employment_points: 10,
            australian_employment_points: 5,
            education_points: 15,
            australian_study_points: 5,
            specialist_education_points: 0,
            community_language_points: 5,
            professional_year_points: 5,
            partner_points: 10,
            nomination_points: 0,
            total_points: 105,
            pass_mark: 65,
            passed: true,
        };
        assert_eq!(result.calculate_total(), 105);
    }

    #[test]
    fn test_english_level_competent() {
        let result = EnglishTestResult {
            test_type: EnglishLanguageTest::Ielts,
            test_date: NaiveDate::from_ymd_opt(2025, 1, 15).expect("valid date"),
            listening: 6.0,
            reading: 6.0,
            writing: 6.0,
            speaking: 6.0,
            overall: 6.0,
            level: EnglishLanguageLevel::Competent,
        };
        assert!(result.meets_competent());
        assert!(!result.meets_proficient());
    }

    #[test]
    fn test_english_level_superior() {
        let result = EnglishTestResult {
            test_type: EnglishLanguageTest::Ielts,
            test_date: NaiveDate::from_ymd_opt(2025, 1, 15).expect("valid date"),
            listening: 8.0,
            reading: 8.0,
            writing: 8.0,
            speaking: 8.0,
            overall: 8.0,
            level: EnglishLanguageLevel::Superior,
        };
        assert!(result.meets_competent());
        assert!(result.meets_proficient());
        assert!(result.meets_superior());
    }
}
