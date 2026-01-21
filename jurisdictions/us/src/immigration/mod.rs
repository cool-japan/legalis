//! Immigration Law Module (Immigration and Nationality Act)
//!
//! # Key Visa Categories
//!
//! ## Nonimmigrant Visas
//! - **H-1B**: Specialty occupation workers (65,000 annual cap + 20,000 advanced degree)
//! - **L-1**: Intracompany transferees (L-1A managers, L-1B specialized knowledge)
//! - **O-1**: Extraordinary ability (arts, sciences, business, athletics, education)
//! - **F-1**: Students (with Optional Practical Training)
//! - **B-1/B-2**: Business visitors and tourists
//! - **E-2**: Treaty investors ($100k+ investment)
//!
//! ## Employment-Based Immigrant Visas (Green Cards)
//! - **EB-1**: Priority workers (extraordinary ability, outstanding researchers, multinational executives)
//! - **EB-2**: Advanced degree professionals (requires PERM labor certification usually)
//! - **EB-3**: Skilled workers, professionals, unskilled workers (requires PERM)
//! - **EB-4**: Special immigrants (religious workers, etc.)
//! - **EB-5**: Investors ($1M or $500k in TEA)
//!
//! ## Family-Based Immigrant Visas
//! - **Immediate Relatives**: Spouses, parents, unmarried children under 21 of US citizens (no quota)
//! - **F-1**: Unmarried sons/daughters of US citizens
//! - **F-2A**: Spouses and children of LPRs
//! - **F-3**: Married sons/daughters of US citizens
//! - **F-4**: Siblings of US citizens (21+ years wait)
//!
//! # Green Card Process
//!
//! 1. **PERM Labor Certification** (EB-2/EB-3) - Employer must test US labor market
//! 2. **I-140 Petition** - Employer petitions for immigrant worker
//! 3. **Priority Date** - Date I-140 filed (or PERM filed)
//! 4. **Wait for visa availability** - Check Visa Bulletin monthly
//! 5. **I-485 Adjustment of Status** or consular processing
//! 6. **Green card issuance** - 10-year card (2-year conditional for marriage-based)
//!
//! # Naturalization (Citizenship)
//!
//! ## 5-Year Rule
//! - 5 years as LPR
//! - 30 months (2.5 years) physical presence in US
//! - Continuous residence (no trips > 6 months)
//! - Good moral character
//! - English proficiency
//! - Civics knowledge (US history and government)
//!
//! ## 3-Year Rule (spouse of US citizen)
//! - 3 years as LPR married to US citizen
//! - 18 months physical presence
//! - Same other requirements

pub mod types;
pub mod validator;

pub use types::*;
pub use validator::*;
