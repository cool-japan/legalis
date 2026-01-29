//! UAE Real Estate Law
//!
//! Comprehensive real estate regulations including:
//! - **RERA** (Real Estate Regulatory Agency) - Dubai
//! - **Escrow Law** - Federal Decree-Law No. 9/2009
//! - **Strata Law** - Federal Law No. 27/2007
//! - Property registration and transfer
//!
//! ## Key Laws
//!
//! - Federal Law No. 27/2007 on Jointly Owned Property (Strata Law)
//! - Dubai Law No. 7/2006 on Real Estate Registration
//! - Federal Decree-Law No. 9/2009 on Escrow Accounts
//! - Abu Dhabi Law No. 3/2015 on Real Estate Registration

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for real estate operations
pub type RealEstateResult<T> = Result<T, RealEstateError>;

/// Property types in UAE
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    /// Residential villa (فيلا سكنية)
    ResidentialVilla,
    /// Residential apartment (شقة سكنية)
    ResidentialApartment,
    /// Commercial office (مكتب تجاري)
    CommercialOffice,
    /// Commercial retail (محل تجاري)
    CommercialRetail,
    /// Industrial property (عقار صناعي)
    Industrial,
    /// Land plot (أرض)
    Land,
    /// Warehouse (مستودع)
    Warehouse,
    /// Hotel apartment (شقة فندقية)
    HotelApartment,
}

impl PropertyType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::ResidentialVilla => "فيلا سكنية",
            Self::ResidentialApartment => "شقة سكنية",
            Self::CommercialOffice => "مكتب تجاري",
            Self::CommercialRetail => "محل تجاري",
            Self::Industrial => "عقار صناعي",
            Self::Land => "أرض",
            Self::Warehouse => "مستودع",
            Self::HotelApartment => "شقة فندقية",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::ResidentialVilla => "Residential Villa",
            Self::ResidentialApartment => "Residential Apartment",
            Self::CommercialOffice => "Commercial Office",
            Self::CommercialRetail => "Commercial Retail",
            Self::Industrial => "Industrial",
            Self::Land => "Land",
            Self::Warehouse => "Warehouse",
            Self::HotelApartment => "Hotel Apartment",
        }
    }

    /// Check if property is residential
    pub fn is_residential(&self) -> bool {
        matches!(self, Self::ResidentialVilla | Self::ResidentialApartment)
    }

    /// Typical Dubai Land Department (DLD) registration fee percentage
    pub fn dld_fee_percentage(&self) -> f64 {
        4.0 // 4% of property value
    }
}

/// Property ownership types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OwnershipType {
    /// Freehold (تملك حر) - Full ownership
    Freehold,
    /// Leasehold (إيجار طويل الأجل) - Long-term lease (typically 99 years)
    Leasehold { years: u32 },
    /// Usufruct (حق الانتفاع) - Right to use
    Usufruct { years: u32 },
    /// Musataha (مساطحة) - Right to build on land
    Musataha { years: u32 },
}

impl OwnershipType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Freehold => "تملك حر",
            Self::Leasehold { .. } => "إيجار طويل الأجل",
            Self::Usufruct { .. } => "حق الانتفاع",
            Self::Musataha { .. } => "مساطحة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Freehold => "Freehold",
            Self::Leasehold { .. } => "Leasehold",
            Self::Usufruct { .. } => "Usufruct",
            Self::Musataha { .. } => "Musataha",
        }
    }
}

/// Designated freehold areas in Dubai
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FreeholdArea {
    /// Dubai Marina
    DubaiMarina,
    /// Downtown Dubai
    DowntownDubai,
    /// Palm Jumeirah
    PalmJumeirah,
    /// Jumeirah Beach Residence (JBR)
    Jbr,
    /// Business Bay
    BusinessBay,
    /// Dubai Hills Estate
    DubaiHillsEstate,
    /// Other designated area
    Other(String),
}

/// RERA rental regulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalContract {
    /// Annual rent (AED)
    pub annual_rent: Aed,
    /// Contract duration (years, typically 1)
    pub duration_years: u32,
    /// Security deposit (typically 5% of annual rent)
    pub security_deposit: Aed,
    /// Number of rent payments per year (1, 2, or 4)
    pub payment_frequency: u32,
    /// Is registered with Ejari (mandatory in Dubai)
    pub ejari_registered: bool,
}

impl RentalContract {
    /// Create standard Dubai rental contract
    pub fn standard(annual_rent: Aed) -> Self {
        let security_deposit = Aed::from_fils(annual_rent.fils() * 5 / 100);
        Self {
            annual_rent,
            duration_years: 1,
            security_deposit,
            payment_frequency: 1, // Annual payment
            ejari_registered: false,
        }
    }

    /// Validate rental contract
    pub fn is_valid(&self) -> RealEstateResult<()> {
        if !self.ejari_registered {
            return Err(RealEstateError::EjariNotRegistered);
        }

        if !matches!(self.payment_frequency, 1 | 2 | 4) {
            return Err(RealEstateError::InvalidPaymentFrequency {
                frequency: self.payment_frequency,
            });
        }

        Ok(())
    }

    /// Calculate Ejari registration fee
    pub fn ejari_fee(&self) -> Aed {
        Aed::from_dirhams(220) // Standard Ejari fee
    }
}

/// Escrow account requirements (Federal Decree-Law No. 9/2009)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowAccount {
    /// Project name
    pub project_name: String,
    /// Developer name
    pub developer: String,
    /// Total project value
    pub project_value: Aed,
    /// Is escrow account opened
    pub escrow_opened: bool,
    /// Bank name
    pub bank: Option<String>,
    /// Percentage of payments to escrow
    pub escrow_percentage: u32,
}

impl EscrowAccount {
    /// Check if escrow is required
    pub fn is_required(&self) -> bool {
        self.project_value.dirhams() > 500_000 // Threshold
    }

    /// Validate escrow compliance
    pub fn is_compliant(&self) -> RealEstateResult<()> {
        if self.is_required() && !self.escrow_opened {
            return Err(RealEstateError::EscrowRequired);
        }

        if self.escrow_opened && self.bank.is_none() {
            return Err(RealEstateError::EscrowBankNotSpecified);
        }

        Ok(())
    }
}

/// Strata title (jointly owned property) - Federal Law No. 27/2007
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrataProperty {
    /// Building/complex name
    pub building_name: String,
    /// Unit number
    pub unit_number: String,
    /// Unit area (sqm)
    pub unit_area_sqm: f64,
    /// Common area percentage
    pub common_area_percentage: f64,
    /// Monthly service charge (AED)
    pub monthly_service_charge: Aed,
    /// Has owners association
    pub has_owners_association: bool,
}

impl StrataProperty {
    /// Calculate annual service charges
    pub fn annual_service_charge(&self) -> Aed {
        Aed::from_fils(self.monthly_service_charge.fils() * 12)
    }

    /// Validate strata compliance
    pub fn is_compliant(&self) -> RealEstateResult<()> {
        if self.common_area_percentage < 0.0 || self.common_area_percentage > 100.0 {
            return Err(RealEstateError::InvalidCommonAreaPercentage {
                percentage: self.common_area_percentage,
            });
        }

        Ok(())
    }
}

/// Transfer fees and costs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCosts {
    /// Property value
    pub property_value: Aed,
    /// Dubai Land Department (DLD) fee (4%)
    pub dld_fee: Aed,
    /// Trustee fee (0.25%)
    pub trustee_fee: Aed,
    /// Mortgage registration (if applicable)
    pub mortgage_registration: Option<Aed>,
    /// Real estate agent fee (typically 2%)
    pub agent_fee: Option<Aed>,
    /// Total transfer costs
    pub total: Aed,
}

impl TransferCosts {
    /// Calculate Dubai property transfer costs
    pub fn calculate_dubai(property_value: Aed, has_mortgage: bool, has_agent: bool) -> Self {
        let dld_fee = Aed::from_fils(property_value.fils() * 4 / 100);
        let trustee_fee = Aed::from_fils(property_value.fils() * 25 / 10000); // 0.25%

        let mortgage_registration = if has_mortgage {
            Some(Aed::from_fils(property_value.fils() * 25 / 10000))
        } else {
            None
        };

        let agent_fee = if has_agent {
            Some(Aed::from_fils(property_value.fils() * 2 / 100))
        } else {
            None
        };

        let mut total = dld_fee + trustee_fee;
        if let Some(mortgage) = mortgage_registration {
            total = total + mortgage;
        }
        if let Some(agent) = agent_fee {
            total = total + agent;
        }

        Self {
            property_value,
            dld_fee,
            trustee_fee,
            mortgage_registration,
            agent_fee,
            total,
        }
    }
}

/// Real estate errors
#[derive(Debug, Error)]
pub enum RealEstateError {
    /// Ejari not registered
    #[error("عقد الإيجار غير مسجل في نظام إيجاري")]
    EjariNotRegistered,

    /// Invalid payment frequency
    #[error("تكرار الدفع غير صحيح: {frequency} (يجب أن يكون 1 أو 2 أو 4)")]
    InvalidPaymentFrequency { frequency: u32 },

    /// Escrow account required
    #[error("يجب فتح حساب ضمان (Escrow) للمشروع")]
    EscrowRequired,

    /// Escrow bank not specified
    #[error("لم يتم تحديد البنك لحساب الضمان")]
    EscrowBankNotSpecified,

    /// Invalid common area percentage
    #[error("نسبة المساحة المشتركة غير صحيحة: {percentage}%")]
    InvalidCommonAreaPercentage { percentage: f64 },

    /// Freehold restriction
    #[error("التملك الحر غير متاح في هذه المنطقة")]
    FreeholdRestriction,
}

/// Get real estate transaction checklist
pub fn get_transaction_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("شهادة ملكية", "Title deed verification"),
        ("رسوم دائرة الأراضي", "Land Department fees (4%)"),
        ("NOC من المطور", "No Objection Certificate from developer"),
        ("براءة ذمة", "Clearance certificate (utilities)"),
        ("تسجيل الرهن", "Mortgage registration (if applicable)"),
        ("عقد البيع", "Sale Purchase Agreement (SPA)"),
        ("نقل الملكية", "Transfer of ownership"),
        ("رسوم الوكيل", "Agent fees (if applicable)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_types() {
        let villa = PropertyType::ResidentialVilla;
        assert!(villa.is_residential());
        assert_eq!(villa.name_ar(), "فيلا سكنية");
        assert_eq!(villa.dld_fee_percentage(), 4.0);
    }

    #[test]
    fn test_ownership_types() {
        let freehold = OwnershipType::Freehold;
        assert_eq!(freehold.name_ar(), "تملك حر");

        let leasehold = OwnershipType::Leasehold { years: 99 };
        assert_eq!(leasehold.name_en(), "Leasehold");
    }

    #[test]
    fn test_rental_contract() {
        let mut contract = RentalContract::standard(Aed::from_dirhams(100_000));
        assert_eq!(contract.security_deposit.dirhams(), 5_000); // 5% of rent

        contract.ejari_registered = true;
        assert!(contract.is_valid().is_ok());
    }

    #[test]
    fn test_rental_contract_ejari_required() {
        let contract = RentalContract::standard(Aed::from_dirhams(80_000));
        assert!(contract.is_valid().is_err());
    }

    #[test]
    fn test_escrow_account() {
        let mut escrow = EscrowAccount {
            project_name: "Test Towers".to_string(),
            developer: "ABC Developers".to_string(),
            project_value: Aed::from_dirhams(50_000_000),
            escrow_opened: false,
            bank: None,
            escrow_percentage: 100,
        };

        assert!(escrow.is_required());
        assert!(escrow.is_compliant().is_err());

        escrow.escrow_opened = true;
        escrow.bank = Some("Emirates NBD".to_string());
        assert!(escrow.is_compliant().is_ok());
    }

    #[test]
    fn test_strata_property() {
        let strata = StrataProperty {
            building_name: "Marina Heights".to_string(),
            unit_number: "1502".to_string(),
            unit_area_sqm: 120.0,
            common_area_percentage: 15.0,
            monthly_service_charge: Aed::from_dirhams(1_200),
            has_owners_association: true,
        };

        assert_eq!(strata.annual_service_charge().dirhams(), 14_400);
        assert!(strata.is_compliant().is_ok());
    }

    #[test]
    fn test_transfer_costs() {
        let costs = TransferCosts::calculate_dubai(
            Aed::from_dirhams(2_000_000),
            true, // has mortgage
            true, // has agent
        );

        assert_eq!(costs.dld_fee.dirhams(), 80_000); // 4%
        assert_eq!(costs.trustee_fee.dirhams(), 5_000); // 0.25%
        assert!(costs.mortgage_registration.is_some());
        assert!(costs.agent_fee.is_some());
        assert!(costs.total.dirhams() > 80_000);
    }

    #[test]
    fn test_transaction_checklist() {
        let checklist = get_transaction_checklist();
        assert!(!checklist.is_empty());
    }

    #[test]
    fn test_ejari_fee() {
        let contract = RentalContract::standard(Aed::from_dirhams(100_000));
        assert_eq!(contract.ejari_fee().dirhams(), 220);
    }
}
