//! Indonesian Company Law - UU No. 40/2007 on Limited Liability Companies (PT)
//!
//! ## Overview
//!
//! Law No. 40 of 2007 on Limited Liability Companies (Perseroan Terbatas - PT)
//! governs the formation, governance, and dissolution of Indonesian corporations.
//!
//! ## Key Features
//!
//! - **Minimum capital**: No minimum paid-up capital requirement
//! - **Minimum shareholders**: 2 (two) for establishment
//! - **Corporate organs**: GMS (Rapat Umum Pemegang Saham), Directors (Direksi), Commissioners (Dewan Komisaris)
//! - **Legal personality**: PT has full legal personality separate from shareholders
//! - **Limited liability**: Shareholders liable only to extent of shares
//!
//! ## Types of Companies
//!
//! - **PT Tertutup**: Closed company (shares not publicly traded)
//! - **PT Terbuka (Tbk)**: Public company (shares listed on stock exchange)
//! - **PT Perseorangan**: Single-shareholder company (since UU Cipta Kerja)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of limited liability company
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    /// Closed company (tertutup) - shares not publicly traded
    Closed,
    /// Public company (terbuka/Tbk) - shares listed on stock exchange
    Public,
    /// Single-shareholder company (perseorangan) - allowed under Omnibus Law
    SingleShareholder,
}

impl CompanyType {
    /// Get company type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Closed => "PT Tertutup",
            Self::Public => "PT Terbuka (Tbk)",
            Self::SingleShareholder => "PT Perseorangan",
        }
    }

    /// Get company type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Closed => "Closed Company",
            Self::Public => "Public Company",
            Self::SingleShareholder => "Single-Shareholder Company",
        }
    }

    /// Get minimum number of shareholders required
    pub fn minimum_shareholders(&self) -> u32 {
        match self {
            Self::Closed => 2,
            Self::Public => 2,
            Self::SingleShareholder => 1,
        }
    }
}

/// Company share capital information - Pasal 31-34
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareCapital {
    /// Authorized capital (modal dasar) - Pasal 32
    pub authorized_capital: i64,
    /// Issued capital (modal ditempatkan) - minimum 25% of authorized
    pub issued_capital: i64,
    /// Paid-up capital (modal disetor) - must equal issued capital
    pub paid_up_capital: i64,
    /// Nominal value per share
    pub par_value: i64,
    /// Total number of shares
    pub total_shares: u32,
    /// Number of issued shares
    pub issued_shares: u32,
}

impl ShareCapital {
    /// Check if capital structure is valid under Pasal 33
    pub fn is_valid(&self) -> bool {
        // Issued capital must be at least 25% of authorized capital
        if self.issued_capital < (self.authorized_capital / 4) {
            return false;
        }

        // Paid-up capital must equal issued capital
        if self.paid_up_capital != self.issued_capital {
            return false;
        }

        // Shares calculation must be consistent
        let calculated_issued = self.par_value * self.issued_shares as i64;
        if calculated_issued != self.issued_capital {
            return false;
        }

        true
    }

    /// Get minimum required issued capital
    pub fn minimum_issued_capital(&self) -> i64 {
        self.authorized_capital / 4
    }
}

/// Shareholder information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shareholder {
    /// Shareholder name
    pub name: String,
    /// ID number (NIK for individuals, registration number for entities)
    pub id_number: String,
    /// Shareholder type
    pub shareholder_type: ShareholderType,
    /// Number of shares owned
    pub shares: u32,
    /// Ownership percentage
    pub ownership_percentage: f64,
    /// Date of share acquisition
    pub acquisition_date: NaiveDate,
}

/// Type of shareholder
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShareholderType {
    /// Indonesian individual
    IndividualDomestic,
    /// Foreign individual
    IndividualForeign,
    /// Indonesian legal entity
    LegalEntityDomestic,
    /// Foreign legal entity
    LegalEntityForeign,
    /// Government
    Government,
}

impl ShareholderType {
    /// Check if shareholder is foreign
    pub fn is_foreign(&self) -> bool {
        matches!(self, Self::IndividualForeign | Self::LegalEntityForeign)
    }

    /// Check if shareholder is legal entity
    pub fn is_legal_entity(&self) -> bool {
        matches!(self, Self::LegalEntityDomestic | Self::LegalEntityForeign)
    }
}

/// Corporate organ type - Pasal 1(2)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorporateOrgan {
    /// General Meeting of Shareholders (RUPS - Rapat Umum Pemegang Saham)
    GeneralMeetingShareholders,
    /// Board of Directors (Direksi) - Pasal 92-107
    BoardOfDirectors,
    /// Board of Commissioners (Dewan Komisaris) - Pasal 108-121
    BoardOfCommissioners,
}

impl CorporateOrgan {
    /// Get organ name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::GeneralMeetingShareholders => "Rapat Umum Pemegang Saham (RUPS)",
            Self::BoardOfDirectors => "Direksi",
            Self::BoardOfCommissioners => "Dewan Komisaris",
        }
    }

    /// Get organ name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::GeneralMeetingShareholders => "General Meeting of Shareholders",
            Self::BoardOfDirectors => "Board of Directors",
            Self::BoardOfCommissioners => "Board of Commissioners",
        }
    }
}

/// Director or commissioner record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Officer {
    /// Name
    pub name: String,
    /// ID number (NIK)
    pub id_number: String,
    /// Position
    pub position: OfficerPosition,
    /// Appointment date
    pub appointment_date: NaiveDate,
    /// Term end date (typically 5 years - Pasal 94)
    pub term_end_date: Option<NaiveDate>,
    /// Whether officer is Indonesian citizen (required - Pasal 93)
    pub is_indonesian_citizen: bool,
    /// Whether officer resides in Indonesia (required - Pasal 93)
    pub resides_in_indonesia: bool,
}

/// Officer position type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OfficerPosition {
    /// President Director (Direktur Utama)
    PresidentDirector,
    /// Director (Direktur)
    Director,
    /// President Commissioner (Komisaris Utama)
    PresidentCommissioner,
    /// Commissioner (Komisaris)
    Commissioner,
    /// Independent Commissioner (Komisaris Independen) - for public companies
    IndependentCommissioner,
}

impl OfficerPosition {
    /// Get position name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::PresidentDirector => "Direktur Utama",
            Self::Director => "Direktur",
            Self::PresidentCommissioner => "Komisaris Utama",
            Self::Commissioner => "Komisaris",
            Self::IndependentCommissioner => "Komisaris Independen",
        }
    }

    /// Get position name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::PresidentDirector => "President Director",
            Self::Director => "Director",
            Self::PresidentCommissioner => "President Commissioner",
            Self::Commissioner => "Commissioner",
            Self::IndependentCommissioner => "Independent Commissioner",
        }
    }

    /// Check if position is director
    pub fn is_director(&self) -> bool {
        matches!(self, Self::PresidentDirector | Self::Director)
    }

    /// Check if position is commissioner
    pub fn is_commissioner(&self) -> bool {
        matches!(
            self,
            Self::PresidentCommissioner | Self::Commissioner | Self::IndependentCommissioner
        )
    }
}

/// Type of General Meeting of Shareholders - Pasal 78
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GmsType {
    /// Annual GMS (RUPS Tahunan) - must be held within 6 months after fiscal year end
    Annual,
    /// Extraordinary GMS (RUPS Luar Biasa)
    Extraordinary,
}

impl GmsType {
    /// Get GMS type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Annual => "RUPS Tahunan",
            Self::Extraordinary => "RUPS Luar Biasa",
        }
    }

    /// Get GMS type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Annual => "Annual General Meeting",
            Self::Extraordinary => "Extraordinary General Meeting",
        }
    }
}

/// Company record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    /// Company name (must end with "PT" or "Perseroan Terbatas")
    pub name: String,
    /// Registration number (NIB or previous TDP)
    pub registration_number: Option<String>,
    /// Tax ID (NPWP)
    pub tax_id: Option<String>,
    /// Company type
    pub company_type: CompanyType,
    /// Establishment date
    pub establishment_date: NaiveDate,
    /// Date of deed of establishment (akta pendirian)
    pub deed_date: NaiveDate,
    /// Notary deed number
    pub deed_number: String,
    /// Notary name
    pub notary_name: String,
    /// Ministry of Law approval number (Pasal 7)
    pub ministry_approval_number: Option<String>,
    /// Ministry approval date
    pub ministry_approval_date: Option<NaiveDate>,
    /// Share capital
    pub share_capital: ShareCapital,
    /// Shareholders
    pub shareholders: Vec<Shareholder>,
    /// Directors
    pub directors: Vec<Officer>,
    /// Commissioners
    pub commissioners: Vec<Officer>,
    /// Business purposes (maksud dan tujuan)
    pub business_purposes: Vec<String>,
    /// Registered address (domisili)
    pub registered_address: String,
    /// Fiscal year end (month)
    pub fiscal_year_end_month: u32,
}

impl Company {
    /// Check if company structure is valid under Law 40/2007
    pub fn is_valid_structure(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check minimum shareholders
        let min_shareholders = self.company_type.minimum_shareholders();
        if (self.shareholders.len() as u32) < min_shareholders {
            errors.push(format!(
                "Minimum {} shareholders required for {}",
                min_shareholders,
                self.company_type.name_en()
            ));
        }

        // Check capital structure
        if !self.share_capital.is_valid() {
            errors.push("Invalid share capital structure (Pasal 33)".to_string());
        }

        // Check minimum directors (at least 1)
        if self.directors.is_empty() {
            errors.push("At least one director required (Pasal 92)".to_string());
        }

        // Check minimum commissioners (at least 1)
        if self.commissioners.is_empty() {
            errors.push("At least one commissioner required (Pasal 108)".to_string());
        }

        // Check directors are Indonesian citizens and residents
        for director in &self.directors {
            if !director.is_indonesian_citizen || !director.resides_in_indonesia {
                errors.push(format!(
                    "Director {} must be Indonesian citizen and resident (Pasal 93)",
                    director.name
                ));
            }
        }

        // Check for public companies
        if matches!(self.company_type, CompanyType::Public) {
            // Public companies must have at least 2 commissioners (Pasal 108(2))
            if self.commissioners.len() < 2 {
                errors.push(
                    "Public company must have at least 2 commissioners (Pasal 108)".to_string(),
                );
            }

            // Must have independent commissioner
            let has_independent = self
                .commissioners
                .iter()
                .any(|c| matches!(c.position, OfficerPosition::IndependentCommissioner));
            if !has_independent {
                errors.push("Public company must have independent commissioner".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Calculate total foreign ownership percentage
    pub fn foreign_ownership_percentage(&self) -> f64 {
        self.shareholders
            .iter()
            .filter(|s| s.shareholder_type.is_foreign())
            .map(|s| s.ownership_percentage)
            .sum()
    }

    /// Check if company requires annual GMS (within 6 months of fiscal year end)
    pub fn annual_gms_deadline(&self, current_year: i32) -> NaiveDate {
        // 6 months after fiscal year end
        let mut year = current_year;
        let mut month = self.fiscal_year_end_month + 6;
        if month > 12 {
            month -= 12;
            year += 1;
        }
        NaiveDate::from_ymd_opt(year, month, 1).unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(year, month - 1, 28).expect("Invalid date calculation")
        })
    }
}

/// Grounds for company dissolution - Pasal 142
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DissolutionGround {
    /// GMS decision
    GmsDecision,
    /// Expiry of period stated in articles of association
    ExpiryOfPeriod,
    /// Court order
    CourtOrder,
    /// Bankruptcy (kepailitan)
    Bankruptcy,
    /// Revocation of business license by authority
    LicenseRevocation,
    /// Only one shareholder remains for 6 months (non-single-shareholder PT)
    SingleShareholderFor6Months,
}

impl DissolutionGround {
    /// Get dissolution ground description in Indonesian
    pub fn description_id(&self) -> &str {
        match self {
            Self::GmsDecision => "Keputusan RUPS",
            Self::ExpiryOfPeriod => "Berakhirnya jangka waktu berdirinya",
            Self::CourtOrder => "Penetapan pengadilan",
            Self::Bankruptcy => "Kepailitan",
            Self::LicenseRevocation => "Pencabutan izin usaha",
            Self::SingleShareholderFor6Months => "Hanya memiliki 1 pemegang saham selama 6 bulan",
        }
    }

    /// Get dissolution ground description in English
    pub fn description_en(&self) -> &str {
        match self {
            Self::GmsDecision => "GMS decision",
            Self::ExpiryOfPeriod => "Expiry of period",
            Self::CourtOrder => "Court order",
            Self::Bankruptcy => "Bankruptcy",
            Self::LicenseRevocation => "License revocation",
            Self::SingleShareholderFor6Months => "Single shareholder for 6 months",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_type() {
        let closed = CompanyType::Closed;
        assert_eq!(closed.name_id(), "PT Tertutup");
        assert_eq!(closed.minimum_shareholders(), 2);

        let single = CompanyType::SingleShareholder;
        assert_eq!(single.minimum_shareholders(), 1);
    }

    #[test]
    fn test_share_capital_valid() {
        let capital = ShareCapital {
            authorized_capital: 1_000_000_000,
            issued_capital: 250_000_000,  // 25% of authorized
            paid_up_capital: 250_000_000, // equals issued
            par_value: 1_000_000,
            total_shares: 1_000,
            issued_shares: 250,
        };
        assert!(capital.is_valid());
    }

    #[test]
    fn test_share_capital_invalid_issued() {
        let capital = ShareCapital {
            authorized_capital: 1_000_000_000,
            issued_capital: 200_000_000, // less than 25%
            paid_up_capital: 200_000_000,
            par_value: 1_000_000,
            total_shares: 1_000,
            issued_shares: 200,
        };
        assert!(!capital.is_valid());
    }

    #[test]
    fn test_share_capital_invalid_paid_up() {
        let capital = ShareCapital {
            authorized_capital: 1_000_000_000,
            issued_capital: 250_000_000,
            paid_up_capital: 200_000_000, // less than issued
            par_value: 1_000_000,
            total_shares: 1_000,
            issued_shares: 250,
        };
        assert!(!capital.is_valid());
    }

    #[test]
    fn test_shareholder_type() {
        let foreign_individual = ShareholderType::IndividualForeign;
        assert!(foreign_individual.is_foreign());
        assert!(!foreign_individual.is_legal_entity());

        let domestic_entity = ShareholderType::LegalEntityDomestic;
        assert!(!domestic_entity.is_foreign());
        assert!(domestic_entity.is_legal_entity());
    }

    #[test]
    fn test_officer_position() {
        let director = OfficerPosition::PresidentDirector;
        assert!(director.is_director());
        assert!(!director.is_commissioner());

        let commissioner = OfficerPosition::IndependentCommissioner;
        assert!(!commissioner.is_director());
        assert!(commissioner.is_commissioner());
    }

    #[test]
    fn test_gms_type() {
        let annual = GmsType::Annual;
        assert_eq!(annual.name_id(), "RUPS Tahunan");
        assert_eq!(annual.name_en(), "Annual General Meeting");
    }

    #[test]
    fn test_dissolution_ground() {
        let bankruptcy = DissolutionGround::Bankruptcy;
        assert_eq!(bankruptcy.description_id(), "Kepailitan");
        assert_eq!(bankruptcy.description_en(), "Bankruptcy");
    }
}
