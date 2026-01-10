//! German Stock Corporation Act (Aktiengesetz - AktG) types
//!
//! Type-safe representations of stock corporations (Aktiengesellschaft - AG)
//! under German law.
//!
//! # Legal Context
//!
//! The AktG regulates stock corporations in Germany:
//! - Formation and organization (§1-53 AktG)
//! - Management Board (Vorstand) (§76-94 AktG)
//! - Supervisory Board (Aufsichtsrat) (§95-116 AktG)
//! - Share capital and shares (§6-12 AktG)
//! - General meetings (Hauptversammlung) (§118-149 AktG)
//!
//! # AG Characteristics
//!
//! - **Minimum capital**: €50,000 (§7 AktG)
//! - **Two-tier board system**: Management Board + Supervisory Board
//! - **Limited liability**: Shareholders liable only for share capital
//! - **Shares**: Freely transferable (except restrictions in articles)
//! - **Public disclosure**: Annual reports, commercial register entries
//! - **Suitable for**: Large companies, public listings, international business

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::gmbhg::Capital;

// =============================================================================
// Stock Corporation (Aktiengesellschaft - AG)
// =============================================================================

/// Stock Corporation per §1-53 AktG
///
/// The AG is Germany's stock corporation form with a two-tier board system
/// and minimum capital of €50,000.
///
/// # Legal Requirements
///
/// - **Minimum capital**: €50,000 (§7 AktG)
/// - **Initial payment**: At least 25% of par value + full premium (§36a AktG)
/// - **Management Board**: At least 1 member (§76 AktG)
/// - **Supervisory Board**: At least 3 members (§95 AktG)
/// - **Company name**: Must include "AG" or "Aktiengesellschaft" (§4 AktG)
/// - **Articles of incorporation**: Notarization required (§23 AktG)
/// - **Commercial register**: Registration creates legal entity (§41 AktG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AG {
    /// Company name (Firma) - must include "AG" suffix (§4 AktG)
    pub company_name: String,

    /// Registered office (Sitz) - German city
    pub registered_office: String,

    /// Business purpose (Unternehmensgegenstand)
    pub business_purpose: String,

    /// Share capital (Grundkapital) - minimum €50,000 (§7 AktG)
    pub share_capital: Capital,

    /// Shares issued (Aktien)
    pub shares: Vec<Share>,

    /// Management Board (Vorstand) - minimum 1 member (§76 AktG)
    pub management_board: ManagementBoard,

    /// Supervisory Board (Aufsichtsrat) - minimum 3 members (§95 AktG)
    pub supervisory_board: SupervisoryBoard,

    /// Formation date (Gründungsdatum)
    pub formation_date: Option<DateTime<Utc>>,

    /// Fiscal year end (Geschäftsjahresende)
    pub fiscal_year_end: Option<FiscalYearEnd>,

    /// Duration (Dauer)
    pub duration: Option<Duration>,
}

// =============================================================================
// Shares (Aktien)
// =============================================================================

/// Share (Aktie) in an AG
///
/// Represents ownership in the corporation. Shares can be:
/// - **Par value shares (Nennbetragsaktien)**: Fixed nominal value (§8 Abs. 2 AktG)
/// - **No-par shares (Stückaktien)**: Proportional share of capital (§8 Abs. 3 AktG)
///
/// # Legal Requirements
///
/// - Par value shares: Minimum €1 (§8 Abs. 2 AktG)
/// - No-par shares: Notional value minimum €1 (§8 Abs. 3 AktG)
/// - Share capital = Sum of all par values or notional values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Share {
    /// Share type (par value or no-par)
    pub share_type: ShareType,

    /// Number of shares
    pub quantity: u64,

    /// Shareholder
    pub shareholder: Shareholder,

    /// Issue price (Ausgabepreis) - may exceed par value
    ///
    /// Issue price = Par value + Premium (Agio)
    /// Premium goes to capital reserves (§272 Abs. 2 Nr. 1 HGB)
    pub issue_price: Capital,

    /// Amount paid (Eingezahlter Betrag)
    ///
    /// Must be at least 25% of par value + full premium (§36a AktG)
    pub amount_paid: Capital,

    /// Share certificate type (Aktiengattung)
    pub certificate_type: CertificateType,
}

/// Share type (Aktienart)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareType {
    /// Par value share (Nennbetragsaktie) - §8 Abs. 2 AktG
    ///
    /// Share with fixed nominal value (minimum €1).
    /// Example: 1,000 shares × €10 par value = €10,000 capital
    ParValue {
        /// Par value per share in Euro cents (minimum 100 = €1)
        par_value_cents: u64,
    },

    /// No-par share (Stückaktie) - §8 Abs. 3 AktG
    ///
    /// Share representing proportional ownership without nominal value.
    /// Notional value = Total capital ÷ Number of shares (minimum €1)
    /// Example: €50,000 capital ÷ 1,000 shares = €50 notional value per share
    NoPar,
}

/// Share certificate type (Aktienurkunde)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificateType {
    /// Bearer share (Inhaberaktie) - §10 Abs. 1 AktG
    ///
    /// Ownership transfers by physical delivery of certificate.
    /// Shareholder is whoever possesses the certificate.
    Bearer,

    /// Registered share (Namensaktie) - §10 Abs. 1 AktG
    ///
    /// Shareholder recorded in share register (Aktienbuch).
    /// Transfer requires endorsement and registration.
    Registered,

    /// Registered share with restricted transferability (vinkulierte Namensaktie)
    ///
    /// Transfer requires company approval (§68 Abs. 2 AktG).
    /// Articles of incorporation must authorize restrictions.
    RegisteredRestricted,
}

/// Shareholder in AG
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shareholder {
    /// Name (natural person or legal entity)
    pub name: String,

    /// Address
    pub address: String,

    /// Shareholder type
    pub shareholder_type: ShareholderType,
}

/// Shareholder type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareholderType {
    /// Natural person (natürliche Person)
    NaturalPerson,

    /// Legal entity (juristische Person) - e.g., GmbH, AG
    LegalEntity,
}

// =============================================================================
// Management Board (Vorstand)
// =============================================================================

/// Management Board (Vorstand) per §76-94 AktG
///
/// The Vorstand manages the corporation under its own responsibility.
///
/// # Legal Requirements
///
/// - **Minimum members**: 1 (§76 Abs. 2 AktG)
/// - **Appointment**: By supervisory board (§84 AktG)
/// - **Term**: Maximum 5 years (§84 Abs. 1 AktG)
/// - **Duties**: Manage company, represent corporation (§76, §78 AktG)
/// - **Liability**: Personal liability for breaches (§93 AktG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementBoard {
    /// Board members (Vorstandsmitglieder)
    pub members: Vec<BoardMember>,

    /// Representation rules (Vertretungsregelung)
    pub representation: RepresentationRule,
}

/// Management board member (Vorstandsmitglied)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardMember {
    /// Name
    pub name: String,

    /// Date of birth
    pub date_of_birth: Option<NaiveDate>,

    /// Address
    pub address: String,

    /// Appointment date (Bestellung)
    pub appointment_date: DateTime<Utc>,

    /// Term end date (maximum 5 years - §84 Abs. 1 AktG)
    pub term_end_date: Option<DateTime<Utc>>,

    /// Position (e.g., CEO, CFO, CTO)
    pub position: Option<String>,

    /// Has legal capacity (Geschäftsfähigkeit)
    ///
    /// Must be natural person with full legal capacity (§76 Abs. 3 AktG)
    pub has_capacity: bool,
}

/// Representation rule (Vertretungsregelung) for Management Board
///
/// Defines how board members can represent the corporation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepresentationRule {
    /// Each board member can represent alone (Einzelvertretung)
    ///
    /// Default unless articles specify otherwise.
    Individual,

    /// All board members must act jointly (Gesamtvertretung)
    ///
    /// Requires signatures of all members.
    Joint,

    /// Two board members or one member with authorized signatory (Prokura)
    ///
    /// Common for larger boards.
    TwoMembersOrProkura,
}

// =============================================================================
// Supervisory Board (Aufsichtsrat)
// =============================================================================

/// Supervisory Board (Aufsichtsrat) per §95-116 AktG
///
/// The Aufsichtsrat supervises management and appoints/dismisses board members.
///
/// # Legal Requirements
///
/// - **Minimum members**: 3 (§95 AktG)
/// - **Must be divisible by 3**: For co-determination (§96, §101 AktG)
/// - **Term**: Maximum 4 years (§102 AktG)
/// - **Duties**: Supervise management, appoint/dismiss Vorstand (§111, §84 AktG)
/// - **Cannot serve on both boards**: Vorstand and Aufsichtsrat separate (§105 AktG)
///
/// # Co-Determination (Mitbestimmung)
///
/// For companies with >500 employees, employee representatives required:
/// - 500-2,000 employees: 1/3 employee representatives (§76 BetrVG 1952)
/// - >2,000 employees: 1/2 employee representatives (§7 MitbestG)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisoryBoard {
    /// Board members (Aufsichtsratsmitglieder)
    pub members: Vec<SupervisoryBoardMember>,

    /// Chairman (Vorsitzender) - has casting vote in deadlock (§108 AktG)
    pub chairman_name: String,

    /// Deputy chairman (Stellvertretender Vorsitzender)
    pub deputy_chairman_name: Option<String>,
}

/// Supervisory board member (Aufsichtsratsmitglied)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisoryBoardMember {
    /// Name
    pub name: String,

    /// Address
    pub address: String,

    /// Appointment date
    pub appointment_date: DateTime<Utc>,

    /// Term end date (maximum 4 years - §102 AktG)
    pub term_end_date: Option<DateTime<Utc>>,

    /// Member type (shareholder or employee representative)
    pub member_type: SupervisoryBoardMemberType,

    /// Position (e.g., Chairman, Deputy Chairman, Member)
    pub position: Option<String>,
}

/// Supervisory board member type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupervisoryBoardMemberType {
    /// Shareholder representative (Anteilseignervertreter)
    ///
    /// Elected by shareholders at general meeting (§101 AktG)
    ShareholderRepresentative,

    /// Employee representative (Arbeitnehmervertreter)
    ///
    /// Required for companies with >500 employees under co-determination laws.
    EmployeeRepresentative,
}

// =============================================================================
// Supporting Types
// =============================================================================

/// Fiscal year end (Geschäftsjahresende)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiscalYearEnd {
    /// Month (1-12)
    pub month: u8,

    /// Day (1-31, depending on month)
    pub day: u8,
}

/// Duration (Dauer) of corporation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Duration {
    /// Unlimited duration (unbegrenzt)
    Unlimited,

    /// Limited to specific date (befristet)
    LimitedTo(DateTime<Utc>),
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_share_type_par_value() {
        let share_type = ShareType::ParValue {
            par_value_cents: 1000, // €10
        };
        assert!(matches!(share_type, ShareType::ParValue { .. }));
    }

    #[test]
    fn test_share_type_no_par() {
        let share_type = ShareType::NoPar;
        assert_eq!(share_type, ShareType::NoPar);
    }

    #[test]
    fn test_certificate_type_enum() {
        let bearer = CertificateType::Bearer;
        let registered = CertificateType::Registered;
        let restricted = CertificateType::RegisteredRestricted;

        assert_ne!(bearer, registered);
        assert_ne!(registered, restricted);
    }

    #[test]
    fn test_shareholder_creation() {
        let shareholder = Shareholder {
            name: "Max Mustermann".to_string(),
            address: "Berlin".to_string(),
            shareholder_type: ShareholderType::NaturalPerson,
        };

        assert_eq!(shareholder.name, "Max Mustermann");
        assert_eq!(shareholder.shareholder_type, ShareholderType::NaturalPerson);
    }

    #[test]
    fn test_share_creation() {
        let share = Share {
            share_type: ShareType::ParValue {
                par_value_cents: 100, // €1
            },
            quantity: 50_000,
            shareholder: Shareholder {
                name: "Founder".to_string(),
                address: "München".to_string(),
                shareholder_type: ShareholderType::NaturalPerson,
            },
            issue_price: Capital::from_euros(1),
            amount_paid: Capital::from_euros(1),
            certificate_type: CertificateType::Registered,
        };

        assert_eq!(share.quantity, 50_000);
        assert_eq!(share.certificate_type, CertificateType::Registered);
    }

    #[test]
    fn test_board_member_creation() {
        let member = BoardMember {
            name: "Dr. Anna Schmidt".to_string(),
            date_of_birth: Some(NaiveDate::from_ymd_opt(1975, 3, 15).unwrap()),
            address: "Frankfurt am Main".to_string(),
            appointment_date: Utc::now(),
            term_end_date: None,
            position: Some("CEO".to_string()),
            has_capacity: true,
        };

        assert_eq!(member.name, "Dr. Anna Schmidt");
        assert!(member.has_capacity);
    }

    #[test]
    fn test_management_board_creation() {
        let board = ManagementBoard {
            members: vec![BoardMember {
                name: "CEO".to_string(),
                date_of_birth: None,
                address: "Berlin".to_string(),
                appointment_date: Utc::now(),
                term_end_date: None,
                position: Some("CEO".to_string()),
                has_capacity: true,
            }],
            representation: RepresentationRule::Individual,
        };

        assert_eq!(board.members.len(), 1);
        assert_eq!(board.representation, RepresentationRule::Individual);
    }

    #[test]
    fn test_supervisory_board_member_creation() {
        let member = SupervisoryBoardMember {
            name: "Prof. Dr. Weber".to_string(),
            address: "Hamburg".to_string(),
            appointment_date: Utc::now(),
            term_end_date: None,
            member_type: SupervisoryBoardMemberType::ShareholderRepresentative,
            position: Some("Chairman".to_string()),
        };

        assert_eq!(
            member.member_type,
            SupervisoryBoardMemberType::ShareholderRepresentative
        );
    }

    #[test]
    fn test_supervisory_board_creation() {
        let board = SupervisoryBoard {
            members: vec![
                SupervisoryBoardMember {
                    name: "Member 1".to_string(),
                    address: "Berlin".to_string(),
                    appointment_date: Utc::now(),
                    term_end_date: None,
                    member_type: SupervisoryBoardMemberType::ShareholderRepresentative,
                    position: Some("Chairman".to_string()),
                },
                SupervisoryBoardMember {
                    name: "Member 2".to_string(),
                    address: "München".to_string(),
                    appointment_date: Utc::now(),
                    term_end_date: None,
                    member_type: SupervisoryBoardMemberType::ShareholderRepresentative,
                    position: None,
                },
                SupervisoryBoardMember {
                    name: "Member 3".to_string(),
                    address: "Hamburg".to_string(),
                    appointment_date: Utc::now(),
                    term_end_date: None,
                    member_type: SupervisoryBoardMemberType::ShareholderRepresentative,
                    position: None,
                },
            ],
            chairman_name: "Member 1".to_string(),
            deputy_chairman_name: Some("Member 2".to_string()),
        };

        assert_eq!(board.members.len(), 3);
        assert_eq!(board.chairman_name, "Member 1");
    }

    #[test]
    fn test_ag_creation() {
        let ag = AG {
            company_name: "Tech Solutions AG".to_string(),
            registered_office: "Berlin".to_string(),
            business_purpose: "Softwareentwicklung und IT-Beratung".to_string(),
            share_capital: Capital::from_euros(50_000),
            shares: vec![],
            management_board: ManagementBoard {
                members: vec![],
                representation: RepresentationRule::Individual,
            },
            supervisory_board: SupervisoryBoard {
                members: vec![],
                chairman_name: "Chairman".to_string(),
                deputy_chairman_name: None,
            },
            formation_date: Some(Utc::now()),
            fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
            duration: Some(Duration::Unlimited),
        };

        assert_eq!(ag.company_name, "Tech Solutions AG");
        assert_eq!(ag.share_capital.to_euros(), 50_000.0);
    }

    #[test]
    fn test_representation_rule_enum() {
        let individual = RepresentationRule::Individual;
        let joint = RepresentationRule::Joint;
        let two_or_prokura = RepresentationRule::TwoMembersOrProkura;

        assert_ne!(individual, joint);
        assert_ne!(joint, two_or_prokura);
    }

    #[test]
    fn test_fiscal_year_end() {
        let fye = FiscalYearEnd { month: 12, day: 31 };
        assert_eq!(fye.month, 12);
        assert_eq!(fye.day, 31);
    }

    #[test]
    fn test_duration_enum() {
        let unlimited = Duration::Unlimited;
        let limited = Duration::LimitedTo(Utc::now());

        assert!(matches!(unlimited, Duration::Unlimited));
        assert!(matches!(limited, Duration::LimitedTo(_)));
    }
}
