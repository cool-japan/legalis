//! Commercial Law Types
//!
//! This module defines types for Lao commercial law, based on:
//! - **Enterprise Law 2013** (Law No. 46/NA)
//! - **Investment Promotion Law 2016** (Law No. 14/NA, amended 2017)
//!
//! ## Coverage
//! - Enterprise types and registration
//! - Capital requirements
//! - Corporate governance structures
//! - Foreign and domestic investment
//! - Investment incentives and privileges
//! - Restricted sectors
//! - Intellectual property registration

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Enterprise types recognized under Enterprise Law 2013
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnterpriseType {
    /// Individual enterprise (ວິສາຫະກິດສ່ວນບຸກຄົນ)
    /// No minimum capital requirement
    IndividualEnterprise,

    /// Partnership (ຫ້າງຫຸ້ນສ່ວນ)
    /// Ordinary partnership or limited partnership
    Partnership(PartnershipType),

    /// Limited company (ບໍລິສັດຈໍາກັດ)
    /// Private limited company with share capital
    LimitedCompany,

    /// Public company (ບໍລິສັດມະຫາຊົນ)
    /// Public limited company, can be listed on stock exchange
    PublicCompany,

    /// State-owned enterprise (ວິສາຫະກິດລັດ)
    StateOwnedEnterprise,
}

/// Partnership types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnershipType {
    /// Ordinary partnership (ຫ້າງຫຸ້ນສ່ວນສາມັນ)
    /// All partners have unlimited liability
    Ordinary,

    /// Limited partnership (ຫ້າງຫຸ້ນສ່ວນຈໍາກັດ)
    /// Has both general and limited partners
    Limited,
}

/// Individual enterprise structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualEnterprise {
    /// Business name (ຊື່ທຸລະກິດ)
    pub name_en: String,
    pub name_lo: String,

    /// Owner information
    pub owner_name: String,
    pub owner_id: String,

    /// Business address
    pub address: String,

    /// Business activities
    pub activities: Vec<String>,

    /// Registration date
    pub registered_at: DateTime<Utc>,

    /// Registration number
    pub registration_number: String,

    /// Initial capital (no minimum requirement)
    pub initial_capital: u64,
}

/// Partner in a partnership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partner {
    /// Partner name
    pub name: String,

    /// Partner ID/passport
    pub id: String,

    /// Partner type (general or limited)
    pub partner_type: PartnerType,

    /// Capital contribution
    pub capital_contribution: u64,

    /// Ownership percentage
    pub ownership_percentage: f64,

    /// Liability (unlimited for general, limited for limited partners)
    pub liability: LiabilityType,
}

/// Partner type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnerType {
    /// General partner (ຫຸ້ນສ່ວນທົ່ວໄປ) - unlimited liability
    General,

    /// Limited partner (ຫຸ້ນສ່ວນຈໍາກັດ) - liability limited to contribution
    Limited,
}

/// Liability type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiabilityType {
    /// Unlimited liability
    Unlimited,

    /// Limited to capital contribution
    Limited,
}

/// Partnership structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partnership {
    /// Partnership name
    pub name_en: String,
    pub name_lo: String,

    /// Partnership type
    pub partnership_type: PartnershipType,

    /// Partners
    pub partners: Vec<Partner>,

    /// Total capital
    pub total_capital: u64,

    /// Business activities
    pub activities: Vec<String>,

    /// Registration details
    pub registration_number: String,
    pub registered_at: DateTime<Utc>,

    /// Managing partner(s)
    pub managing_partners: Vec<String>,
}

/// Shareholder in a company
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shareholder {
    /// Shareholder name
    pub name: String,

    /// Shareholder ID
    pub id: String,

    /// Number of shares
    pub shares: u64,

    /// Ownership percentage
    pub ownership_percentage: f64,

    /// Is this a foreign shareholder?
    pub is_foreign: bool,

    /// Nationality (for foreign shareholders)
    pub nationality: Option<String>,
}

/// Limited company structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitedCompany {
    /// Company name
    pub name_en: String,
    pub name_lo: String,

    /// Registered capital (minimum 50,000,000 LAK)
    pub registered_capital: u64,

    /// Paid-up capital (minimum 30% of registered capital)
    pub paid_up_capital: u64,

    /// Shareholders (1-30 shareholders)
    pub shareholders: Vec<Shareholder>,

    /// Board of directors
    pub board: BoardOfDirectors,

    /// Business activities
    pub activities: Vec<String>,

    /// Registration details
    pub registration_number: String,
    pub registered_at: DateTime<Utc>,

    /// Foreign ownership percentage
    pub foreign_ownership_percentage: f64,
}

/// Public company structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicCompany {
    /// Company name
    pub name_en: String,
    pub name_lo: String,

    /// Registered capital (minimum 1,000,000,000 LAK)
    pub registered_capital: u64,

    /// Paid-up capital (minimum 30% of registered capital)
    pub paid_up_capital: u64,

    /// Shareholders (minimum 15 shareholders)
    pub shareholders: Vec<Shareholder>,

    /// Board of directors
    pub board: BoardOfDirectors,

    /// Is listed on stock exchange?
    pub is_listed: bool,

    /// Stock exchange code (if listed)
    pub stock_code: Option<String>,

    /// Business activities
    pub activities: Vec<String>,

    /// Registration details
    pub registration_number: String,
    pub registered_at: DateTime<Utc>,

    /// Foreign ownership percentage
    pub foreign_ownership_percentage: f64,
}

/// Director in a company
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Director {
    /// Director name
    pub name: String,

    /// Director ID
    pub id: String,

    /// Position
    pub position: DirectorPosition,

    /// Nationality
    pub nationality: String,

    /// Is this a foreign director?
    pub is_foreign: bool,

    /// Appointed date
    pub appointed_at: DateTime<Utc>,
}

/// Director position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorPosition {
    /// Chairperson (ປະທານ)
    Chairperson,

    /// Managing Director / CEO (ຜູ້ຈັດການ)
    ManagingDirector,

    /// Director (ກໍາມະການ)
    Director,

    /// Independent Director (ກໍາມະການອິສະລະ)
    Independent,
}

/// Board of Directors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardOfDirectors {
    /// Directors (minimum 3)
    pub directors: Vec<Director>,

    /// Board meetings per year (minimum 1)
    pub meetings_per_year: u32,

    /// Last board meeting date
    pub last_meeting: Option<DateTime<Utc>>,
}

/// Shareholders' meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholdersMeeting {
    /// Meeting type
    pub meeting_type: MeetingType,

    /// Meeting date
    pub meeting_date: DateTime<Utc>,

    /// Agenda items
    pub agenda: Vec<String>,

    /// Resolutions passed
    pub resolutions: Vec<Resolution>,

    /// Attendance (percentage)
    pub attendance_percentage: f64,
}

/// Meeting type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeetingType {
    /// Annual General Meeting (ກອງປະຊຸມໃຫຍ່ປະຈໍາປີ)
    AnnualGeneral,

    /// Extraordinary General Meeting (ກອງປະຊຸມໃຫຍ່ສະໄໝວິສາມັນ)
    ExtraordinaryGeneral,

    /// Board meeting (ກອງປະຊຸມຄະນະກໍາມະການ)
    Board,
}

/// Resolution passed in a meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    /// Resolution description
    pub description: String,

    /// Votes in favor
    pub votes_for: u64,

    /// Votes against
    pub votes_against: u64,

    /// Abstentions
    pub abstentions: u64,

    /// Passed?
    pub passed: bool,
}

/// Investment type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InvestmentType {
    /// Domestic investment (ການລົງທຶນພາຍໃນ)
    Domestic(DomesticInvestment),

    /// Foreign investment (ການລົງທຶນຕ່າງປະເທດ)
    Foreign(ForeignInvestment),

    /// Joint venture (ການລົງທຶນຮ່ວມ)
    JointVenture {
        domestic_percentage: f64,
        foreign_percentage: f64,
    },
}

/// Domestic investment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomesticInvestment {
    /// Investor name
    pub investor_name: String,

    /// Investment amount (LAK)
    pub investment_amount: u64,

    /// Sector
    pub sector: BusinessSector,

    /// Location (province)
    pub location: String,

    /// Incentives applicable
    pub incentives: Vec<InvestmentIncentive>,
}

/// Foreign investment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignInvestment {
    /// Investor name
    pub investor_name: String,

    /// Investor nationality
    pub investor_nationality: String,

    /// Investment amount (LAK)
    pub investment_amount: u64,

    /// Sector
    pub sector: BusinessSector,

    /// Location (province)
    pub location: String,

    /// Foreign ownership percentage
    pub foreign_ownership_percentage: f64,

    /// Requires approval?
    pub requires_approval: bool,

    /// Approval status
    pub approval_status: Option<ApprovalStatus>,

    /// Incentives applicable
    pub incentives: Vec<InvestmentIncentive>,

    /// Concession agreement (if applicable)
    pub concession: Option<Concession>,
}

/// Approval status for foreign investment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    /// Pending approval
    Pending,

    /// Approved
    Approved,

    /// Rejected
    Rejected,

    /// Conditional approval
    Conditional,
}

/// Business sectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessSector {
    /// Agriculture (ກະສິກໍາ)
    Agriculture,

    /// Manufacturing (ການຜະລິດ)
    Manufacturing,

    /// Services (ການບໍລິການ)
    Services,

    /// Tourism (ການທ່ອງທ່ຽວ)
    Tourism,

    /// Mining (ການຂຸດຄົ້ນບໍ່ແຮ່)
    Mining,

    /// Energy (ພະລັງງານ)
    Energy,

    /// Infrastructure (ໂຄງລ່າງພື້ນຖານ)
    Infrastructure,

    /// Finance (ການເງິນ)
    Finance,

    /// Telecommunications (ໂທລະຄົມມະນາຄົມ)
    Telecommunications,

    /// Education (ການສຶກສາ)
    Education,

    /// Healthcare (ສາທາລະນະສຸກ)
    Healthcare,

    /// Real Estate (ອະສັງຫາລິມະຊັບ)
    RealEstate,

    /// Technology (ເຕັກໂນໂລຊີ)
    Technology,
}

/// Investment incentives under Investment Promotion Law 2016
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestmentIncentive {
    /// Profit tax exemption (years)
    ProfitTaxExemption(u32),

    /// Import duty exemption
    ImportDutyExemption,

    /// Export duty exemption
    ExportDutyExemption,

    /// Land rental fee reduction (percentage)
    LandRentalReduction(u32),

    /// Fast-track approval
    FastTrackApproval,

    /// Priority in license issuance
    PriorityLicensing,

    /// Special economic zone benefits
    SEZBenefits,
}

/// Concession agreement for large-scale investment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Concession {
    /// Concession type
    pub concession_type: ConcessionType,

    /// Duration (years)
    pub duration_years: u32,

    /// Area (hectares for land concession)
    pub area_hectares: Option<f64>,

    /// Royalty rate (percentage)
    pub royalty_rate: f64,

    /// Start date
    pub start_date: DateTime<Utc>,

    /// End date
    pub end_date: DateTime<Utc>,

    /// Conditions and obligations
    pub conditions: Vec<String>,
}

/// Concession types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcessionType {
    /// Mining concession
    Mining,

    /// Land concession for agriculture
    LandAgriculture,

    /// Hydropower concession
    Hydropower,

    /// Infrastructure concession (BOT/PPP)
    Infrastructure,
}

/// Restricted and prohibited sectors for foreign investment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestrictedSector {
    /// Prohibited for foreign investment
    Prohibited(ProhibitedSector),

    /// Conditional approval required
    Conditional(ConditionalSector),
}

/// Sectors prohibited for foreign investment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProhibitedSector {
    /// Media and broadcasting
    MediaBroadcasting,

    /// Domestic postal services
    DomesticPostal,

    /// Certain cultural activities
    CulturalActivities,
}

/// Sectors requiring conditional approval
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionalSector {
    /// Banking (foreign ownership max 49%)
    Banking,

    /// Insurance (foreign ownership max 49%)
    Insurance,

    /// Telecommunications (foreign ownership max 49%)
    Telecommunications,

    /// Education (special approval)
    Education,

    /// Healthcare (special approval)
    Healthcare,

    /// Legal services (partnership only)
    LegalServices,

    /// Accounting services (partnership only)
    AccountingServices,
}

/// Intellectual property types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntellectualProperty {
    /// Patent (ສິດທິບັດ)
    Patent(Patent),

    /// Trademark (ເຄື່ອງໝາຍການຄ້າ)
    Trademark(Trademark),

    /// Copyright (ລິຂະສິດ)
    Copyright(Copyright),

    /// Industrial design (ແບບອຸດສາຫະກໍາ)
    IndustrialDesign(IndustrialDesign),
}

/// Patent registration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patent {
    /// Patent title
    pub title_en: String,
    pub title_lo: String,

    /// Inventor(s)
    pub inventors: Vec<String>,

    /// Applicant
    pub applicant: String,

    /// Application number
    pub application_number: String,

    /// Application date
    pub application_date: DateTime<Utc>,

    /// Grant date (if granted)
    pub grant_date: Option<DateTime<Utc>>,

    /// Patent number (if granted)
    pub patent_number: Option<String>,

    /// Protection period (20 years from application)
    pub expiry_date: DateTime<Utc>,

    /// Patent status
    pub status: IPStatus,

    /// Description/abstract
    pub description: String,
}

/// Trademark registration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trademark {
    /// Trademark name/text
    pub trademark_text: Option<String>,

    /// Trademark logo (description or file reference)
    pub trademark_logo: Option<String>,

    /// Owner
    pub owner: String,

    /// Application number
    pub application_number: String,

    /// Application date
    pub application_date: DateTime<Utc>,

    /// Registration date (if registered)
    pub registration_date: Option<DateTime<Utc>>,

    /// Registration number (if registered)
    pub registration_number: Option<String>,

    /// Trademark classes (Nice Classification)
    pub classes: Vec<u32>,

    /// Protection period (10 years, renewable)
    pub expiry_date: DateTime<Utc>,

    /// Status
    pub status: IPStatus,
}

/// Copyright registration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Copyright {
    /// Work title
    pub title_en: String,
    pub title_lo: String,

    /// Author(s)
    pub authors: Vec<String>,

    /// Copyright holder
    pub holder: String,

    /// Type of work
    pub work_type: CopyrightWorkType,

    /// Registration number
    pub registration_number: Option<String>,

    /// Registration date
    pub registration_date: Option<DateTime<Utc>>,

    /// Publication date
    pub publication_date: DateTime<Utc>,

    /// Protection period (life + 50 years)
    pub expiry_date: DateTime<Utc>,

    /// Status
    pub status: IPStatus,
}

/// Industrial design registration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndustrialDesign {
    /// Design title
    pub title_en: String,
    pub title_lo: String,

    /// Designer(s)
    pub designers: Vec<String>,

    /// Applicant
    pub applicant: String,

    /// Application number
    pub application_number: String,

    /// Application date
    pub application_date: DateTime<Utc>,

    /// Registration date (if registered)
    pub registration_date: Option<DateTime<Utc>>,

    /// Registration number (if registered)
    pub registration_number: Option<String>,

    /// Protection period (5 years, renewable up to 15 years)
    pub expiry_date: DateTime<Utc>,

    /// Status
    pub status: IPStatus,

    /// Description
    pub description: String,
}

/// Copyright work types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CopyrightWorkType {
    /// Literary work
    Literary,

    /// Musical work
    Musical,

    /// Artistic work
    Artistic,

    /// Dramatic work
    Dramatic,

    /// Audiovisual work
    Audiovisual,

    /// Computer software
    Software,

    /// Database
    Database,
}

/// Intellectual property status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IPStatus {
    /// Application pending
    Pending,

    /// Granted/Registered
    Granted,

    /// Rejected
    Rejected,

    /// Expired
    Expired,

    /// Abandoned
    Abandoned,

    /// Under examination
    UnderExamination,
}

/// Capital requirements for different enterprise types
pub struct CapitalRequirements;

impl CapitalRequirements {
    /// Minimum registered capital for limited company (LAK)
    pub const LIMITED_COMPANY_MIN: u64 = 50_000_000; // 50 million LAK

    /// Minimum registered capital for public company (LAK)
    pub const PUBLIC_COMPANY_MIN: u64 = 1_000_000_000; // 1 billion LAK

    /// Minimum paid-up capital ratio (percentage of registered capital)
    pub const MINIMUM_PAID_UP_RATIO: f64 = 0.3; // 30%

    /// Get minimum capital for enterprise type
    pub fn get_minimum_capital(enterprise_type: &EnterpriseType) -> Option<u64> {
        match enterprise_type {
            EnterpriseType::IndividualEnterprise => None,
            EnterpriseType::Partnership(_) => None,
            EnterpriseType::LimitedCompany => Some(Self::LIMITED_COMPANY_MIN),
            EnterpriseType::PublicCompany => Some(Self::PUBLIC_COMPANY_MIN),
            EnterpriseType::StateOwnedEnterprise => None,
        }
    }

    /// Calculate minimum paid-up capital
    pub fn calculate_minimum_paid_up(registered_capital: u64) -> u64 {
        ((registered_capital as f64) * Self::MINIMUM_PAID_UP_RATIO) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capital_requirements_limited_company() {
        assert_eq!(
            CapitalRequirements::get_minimum_capital(&EnterpriseType::LimitedCompany),
            Some(50_000_000)
        );
    }

    #[test]
    fn test_capital_requirements_public_company() {
        assert_eq!(
            CapitalRequirements::get_minimum_capital(&EnterpriseType::PublicCompany),
            Some(1_000_000_000)
        );
    }

    #[test]
    fn test_capital_requirements_individual() {
        assert_eq!(
            CapitalRequirements::get_minimum_capital(&EnterpriseType::IndividualEnterprise),
            None
        );
    }

    #[test]
    fn test_minimum_paid_up_calculation() {
        assert_eq!(
            CapitalRequirements::calculate_minimum_paid_up(100_000_000),
            30_000_000
        );
    }

    #[test]
    fn test_shareholder_creation() {
        let shareholder = Shareholder {
            name: "John Doe".to_string(),
            id: "P1234567".to_string(),
            shares: 1000,
            ownership_percentage: 25.0,
            is_foreign: true,
            nationality: Some("USA".to_string()),
        };
        assert_eq!(shareholder.ownership_percentage, 25.0);
        assert!(shareholder.is_foreign);
    }

    #[test]
    fn test_partnership_types() {
        let ordinary = PartnershipType::Ordinary;
        let limited = PartnershipType::Limited;
        assert_ne!(ordinary, limited);
    }

    #[test]
    fn test_director_positions() {
        let chair = DirectorPosition::Chairperson;
        let md = DirectorPosition::ManagingDirector;
        assert_ne!(chair, md);
    }

    #[test]
    fn test_investment_incentives() {
        let tax_exempt = InvestmentIncentive::ProfitTaxExemption(7);
        let import_exempt = InvestmentIncentive::ImportDutyExemption;
        assert_ne!(tax_exempt, import_exempt);
    }

    #[test]
    fn test_business_sectors() {
        let agri = BusinessSector::Agriculture;
        let tech = BusinessSector::Technology;
        assert_ne!(agri, tech);
    }

    #[test]
    fn test_ip_status() {
        let pending = IPStatus::Pending;
        let granted = IPStatus::Granted;
        assert_ne!(pending, granted);
    }
}
