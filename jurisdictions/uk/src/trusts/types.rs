//! Core trust types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Trust under English law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trust {
    /// Trust name/identifier
    pub name: String,
    /// Type of trust
    pub trust_type: TrustType,
    /// Settlor (creator of trust)
    pub settlor: String,
    /// Trustees
    pub trustees: Vec<Trustee>,
    /// Beneficiaries
    pub beneficiaries: Vec<Beneficiary>,
    /// Trust property
    pub property: Vec<TrustProperty>,
    /// Creation date
    pub creation_date: NaiveDate,
    /// Trust deed (if written)
    pub trust_deed: Option<String>,
    /// Is trust validly created?
    pub is_valid: bool,
}

/// Type of trust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustType {
    /// Express trust (intentionally created)
    Express,
    /// Resulting trust (presumed intention)
    Resulting,
    /// Constructive trust (imposed by law)
    Constructive,
    /// Charitable trust (Charities Act 2011)
    Charitable,
    /// Bare trust (trustee holds legal title only)
    Bare,
    /// Discretionary trust (trustee has discretion in distribution)
    Discretionary,
    /// Fixed trust (beneficial interests fixed)
    Fixed,
}

/// Trustee
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trustee {
    /// Name
    pub name: String,
    /// Type (individual, corporate trustee)
    pub trustee_type: TrusteeType,
    /// Appointment date
    pub appointment_date: NaiveDate,
    /// Removal date (if removed)
    pub removal_date: Option<NaiveDate>,
    /// Is professional trustee?
    pub is_professional: bool,
}

/// Type of trustee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrusteeType {
    /// Individual person
    Individual,
    /// Corporate trustee (trust company)
    Corporate,
    /// Public trustee
    Public,
    /// Judicial trustee
    Judicial,
}

/// Beneficiary
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Beneficiary {
    /// Name (or description for class of beneficiaries)
    pub name: String,
    /// Type
    pub beneficiary_type: BeneficiaryType,
    /// Share/interest (if fixed trust)
    pub share: Option<String>,
    /// Is interest vested or contingent?
    pub vested: bool,
}

/// Type of beneficiary
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeneficiaryType {
    /// Named individual
    Individual,
    /// Class of persons (e.g., "my children")
    Class,
    /// Discretionary (trustee has discretion)
    Discretionary,
}

/// Trust property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustProperty {
    /// Description of property
    pub description: String,
    /// Type of property
    pub property_type: PropertyType,
    /// Estimated value
    pub value_gbp: Option<f64>,
    /// Date added to trust
    pub date_added: NaiveDate,
}

/// Type of property
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    /// Real property (land)
    RealProperty,
    /// Money
    Money,
    /// Shares/securities
    Securities,
    /// Personal property (chattels)
    PersonalProperty,
    /// Intellectual property
    IntellectualProperty,
    /// Business interest
    BusinessInterest,
}
