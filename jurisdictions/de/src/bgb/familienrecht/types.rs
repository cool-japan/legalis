//! Types for German Family Law (Familienrecht - BGB Book 4)
//!
//! This module provides type-safe representations of German family law concepts
//! including marriage, divorce, maintenance obligations, and parental custody.

use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::gmbhg::Capital;

/// Marriage status under German law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarriageStatus {
    /// Valid marriage (§1310 BGB)
    Valid,
    /// Invalid marriage (§1314 BGB)
    Invalid,
    /// Void marriage (nichtige Ehe)
    Void,
    /// Divorced (§1564 BGB)
    Divorced,
    /// Marriage ended by death
    EndedByDeath,
}

/// Gender for legal purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Diverse, // §22 Abs. 3 PStG (since 2018)
}

/// Person in family law context
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub place_of_birth: String,
    pub nationality: String,
    pub gender: Gender,
    pub address: String,
}

impl Person {
    /// Calculate age at a given date
    pub fn age_at(&self, date: NaiveDate) -> u32 {
        let years = date.year() - self.date_of_birth.year();
        if date.month() < self.date_of_birth.month()
            || (date.month() == self.date_of_birth.month() && date.day() < self.date_of_birth.day())
        {
            (years - 1) as u32
        } else {
            years as u32
        }
    }

    /// Check if person has reached age of majority (18 years) - §2 BGB
    pub fn is_adult(&self) -> bool {
        self.age_at(Utc::now().date_naive()) >= 18
    }

    /// Check if person meets minimum marriage age (18 years) - §1303 BGB
    pub fn meets_marriage_age(&self) -> bool {
        self.is_adult()
    }
}

/// Marriage impediments (Eheverbote) under §§1306-1311 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarriageImpediment {
    /// One party already married (§1306 BGB)
    ExistingMarriage,
    /// Parties are related (§1307 BGB - lineal relatives, siblings)
    Consanguinity,
    /// One party lacks legal capacity (§1304 BGB)
    LackOfCapacity,
    /// Minimum age not met (§1303 BGB)
    BelowMinimumAge,
}

/// Marriage (§§1303-1352 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marriage {
    pub spouse1: Person,
    pub spouse2: Person,
    pub marriage_date: NaiveDate,
    pub place_of_marriage: String,
    pub registrar_office: String,
    pub status: MarriageStatus,
    pub property_regime: MatrimonialPropertyRegime,
    pub impediments: Vec<MarriageImpediment>,
}

impl Marriage {
    /// Check if marriage is valid (no impediments)
    pub fn is_valid(&self) -> bool {
        self.impediments.is_empty() && self.status == MarriageStatus::Valid
    }

    /// Calculate marriage duration
    pub fn duration_years(&self) -> u32 {
        let today = Utc::now().date_naive();
        let years = today.year() - self.marriage_date.year();
        if today.month() < self.marriage_date.month()
            || (today.month() == self.marriage_date.month()
                && today.day() < self.marriage_date.day())
        {
            (years - 1) as u32
        } else {
            years as u32
        }
    }
}

/// Matrimonial property regimes (Güterrecht) - §§1363-1563 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatrimonialPropertyRegime {
    /// Community of accrued gains (Zugewinngemeinschaft) - §§1363-1390 BGB
    /// This is the DEFAULT regime if no agreement exists
    CommunityOfAccruedGains,
    /// Separation of property (Gütertrennung) - §§1414 BGB
    SeparationOfProperty,
    /// Community of property (Gütergemeinschaft) - §§1415-1518 BGB (rare)
    CommunityOfProperty,
}

/// Matrimonial property agreement (Ehevertrag) - §1408 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrimonialPropertyAgreement {
    pub spouses: (Person, Person),
    pub agreement_date: NaiveDate,
    pub notarized: bool, // REQUIRED per §1410 BGB
    pub chosen_regime: MatrimonialPropertyRegime,
    pub special_provisions: Vec<String>,
}

/// Assets for accrued gains calculation (§1374 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assets {
    pub real_estate_value: Capital,
    pub movable_property_value: Capital,
    pub bank_accounts: Capital,
    pub securities: Capital,
    pub business_interests: Capital,
    pub other_assets: Capital,
    pub liabilities: Capital,
}

impl Assets {
    /// Calculate net assets (§1374 BGB)
    pub fn net_value(&self) -> Capital {
        let total_assets = Capital::from_cents(
            self.real_estate_value.amount_cents
                + self.movable_property_value.amount_cents
                + self.bank_accounts.amount_cents
                + self.securities.amount_cents
                + self.business_interests.amount_cents
                + self.other_assets.amount_cents,
        );

        if total_assets.amount_cents > self.liabilities.amount_cents {
            Capital::from_cents(total_assets.amount_cents - self.liabilities.amount_cents)
        } else {
            Capital::from_cents(0)
        }
    }
}

/// Accrued gains calculation (Zugewinnausgleich) - §§1372-1390 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccruedGainsCalculation {
    pub spouse1_initial_assets: Assets,
    pub spouse1_final_assets: Assets,
    pub spouse2_initial_assets: Assets,
    pub spouse2_final_assets: Assets,
    pub marriage_start_date: NaiveDate,
    pub marriage_end_date: NaiveDate,
}

impl AccruedGainsCalculation {
    /// Calculate accrued gain for spouse 1 (§1373 BGB)
    pub fn spouse1_accrued_gain(&self) -> Capital {
        let initial = self.spouse1_initial_assets.net_value();
        let final_value = self.spouse1_final_assets.net_value();

        if final_value.amount_cents > initial.amount_cents {
            Capital::from_cents(final_value.amount_cents - initial.amount_cents)
        } else {
            Capital::from_cents(0)
        }
    }

    /// Calculate accrued gain for spouse 2 (§1373 BGB)
    pub fn spouse2_accrued_gain(&self) -> Capital {
        let initial = self.spouse2_initial_assets.net_value();
        let final_value = self.spouse2_final_assets.net_value();

        if final_value.amount_cents > initial.amount_cents {
            Capital::from_cents(final_value.amount_cents - initial.amount_cents)
        } else {
            Capital::from_cents(0)
        }
    }

    /// Calculate equalization claim (§1378 BGB)
    /// The spouse with lower accrued gain has a claim for half the difference
    pub fn equalization_claim(&self) -> (EqualizationClaimant, Capital) {
        let gain1 = self.spouse1_accrued_gain();
        let gain2 = self.spouse2_accrued_gain();

        if gain1.amount_cents > gain2.amount_cents {
            let difference = gain1.amount_cents - gain2.amount_cents;
            (
                EqualizationClaimant::Spouse2,
                Capital::from_cents(difference / 2),
            )
        } else if gain2.amount_cents > gain1.amount_cents {
            let difference = gain2.amount_cents - gain1.amount_cents;
            (
                EqualizationClaimant::Spouse1,
                Capital::from_cents(difference / 2),
            )
        } else {
            (EqualizationClaimant::Neither, Capital::from_cents(0))
        }
    }
}

/// Who has the equalization claim
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EqualizationClaimant {
    Spouse1,
    Spouse2,
    Neither,
}

/// Grounds for divorce (§1565 BGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DivorceGround {
    /// Marriage breakdown (Scheitern der Ehe) - §1565 Abs. 1 BGB
    /// Presumed after 1 year separation with mutual consent (§1566 Abs. 1)
    /// Presumed after 3 years separation without consent (§1566 Abs. 2)
    MarriageBreakdown,
}

/// Divorce proceedings (Scheidung) - §§1564-1587 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Divorce {
    pub marriage: Marriage,
    pub filing_date: NaiveDate,
    pub separation_date: NaiveDate,
    pub ground: DivorceGround,
    pub mutual_consent: bool,
    pub divorce_decree_date: Option<NaiveDate>,
    pub accrued_gains_equalization: Option<AccruedGainsCalculation>,
    pub pension_equalization: Option<PensionEqualization>,
}

impl Divorce {
    /// Calculate separation period in months
    pub fn separation_period_months(&self) -> u32 {
        let end_date = self.divorce_decree_date.unwrap_or(Utc::now().date_naive());
        let years = end_date.year() - self.separation_date.year();
        let months = end_date.month() as i32 - self.separation_date.month() as i32;
        ((years * 12) + months) as u32
    }

    /// Check if minimum separation period met (§1566 BGB)
    pub fn meets_separation_requirement(&self) -> bool {
        let months = self.separation_period_months();
        if self.mutual_consent {
            months >= 12 // §1566 Abs. 1 - 1 year with consent
        } else {
            months >= 36 // §1566 Abs. 2 - 3 years without consent
        }
    }
}

/// Post-marital maintenance (nachehelicher Unterhalt) - §§1569-1586 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMaritalMaintenance {
    pub claimant: Person,
    pub obligor: Person,
    pub ground: MaintenanceGround,
    pub monthly_amount: Capital,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub limited_duration: bool, // §1578b BGB - temporal limitation
}

/// Grounds for post-marital maintenance (§§1570-1576 BGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceGround {
    /// Child care (§1570 BGB)
    ChildCare,
    /// Age (§1571 BGB)
    Age,
    /// Illness (§1572 BGB)
    Illness,
    /// Unemployment (§1573 BGB)
    Unemployment,
    /// Additional training (§1575 BGB)
    AdditionalTraining,
    /// Equity reasons (§1576 BGB)
    Equity,
}

/// Pension equalization (Versorgungsausgleich) - §§1587-1587p BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PensionEqualization {
    pub spouse1_pension_rights: Capital,
    pub spouse2_pension_rights: Capital,
    pub marriage_duration_years: u32,
    pub equalization_performed: bool,
}

impl PensionEqualization {
    /// Calculate equalization amount
    pub fn equalization_amount(&self) -> (EqualizationClaimant, Capital) {
        if self.spouse1_pension_rights.amount_cents > self.spouse2_pension_rights.amount_cents {
            let difference =
                self.spouse1_pension_rights.amount_cents - self.spouse2_pension_rights.amount_cents;
            (
                EqualizationClaimant::Spouse2,
                Capital::from_cents(difference / 2),
            )
        } else if self.spouse2_pension_rights.amount_cents
            > self.spouse1_pension_rights.amount_cents
        {
            let difference =
                self.spouse2_pension_rights.amount_cents - self.spouse1_pension_rights.amount_cents;
            (
                EqualizationClaimant::Spouse1,
                Capital::from_cents(difference / 2),
            )
        } else {
            (EqualizationClaimant::Neither, Capital::from_cents(0))
        }
    }
}

/// Parentage (Abstammung) - §§1591-1600 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentageStatus {
    /// Mother by birth (§1591 BGB)
    MotherByBirth,
    /// Father by marriage (§1592 Nr. 1 BGB)
    FatherByMarriage,
    /// Father by acknowledgment (§1592 Nr. 2 BGB)
    FatherByAcknowledgment,
    /// Father by court determination (§1592 Nr. 3 BGB)
    FatherByCourtDetermination,
}

/// Parent-child relationship
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentChildRelationship {
    pub parent: Person,
    pub child: Person,
    pub parentage_status: ParentageStatus,
    pub established_date: NaiveDate,
}

/// Maintenance obligation (Unterhaltspflicht) - §§1601-1615 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceObligation {
    pub obligor: Person,
    pub beneficiary: Person,
    pub relationship: MaintenanceRelationship,
    pub monthly_amount: Capital,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

/// Types of maintenance relationships
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceRelationship {
    /// Parent to child (§1601 BGB)
    ParentToChild,
    /// Child to parent (§1601 BGB)
    ChildToParent,
    /// Between spouses during marriage (§1360 BGB)
    BetweenSpouses,
    /// Post-marital (§§1569-1586 BGB)
    PostMarital,
}

/// Parental custody (elterliche Sorge) - §§1626-1698 BGB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentalCustody {
    pub child: Person,
    pub custody_holders: Vec<Person>,
    pub custody_type: CustodyType,
    pub established_date: NaiveDate,
}

/// Types of parental custody
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustodyType {
    /// Joint custody (gemeinsame Sorge) - §1626 BGB (default for married parents)
    Joint,
    /// Sole custody (Alleinsorge) - §1626a BGB
    Sole,
}

impl ParentalCustody {
    /// Check if custody is joint
    pub fn is_joint(&self) -> bool {
        matches!(self.custody_type, CustodyType::Joint)
    }

    /// Check if child is minor (under 18)
    pub fn child_is_minor(&self) -> bool {
        !self.child.is_adult()
    }
}
