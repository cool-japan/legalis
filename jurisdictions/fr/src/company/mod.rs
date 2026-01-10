//! Company law module (Module de droit des sociétés)
//!
//! This module provides comprehensive support for French company law under the Code de commerce.
//!
//! # Introduction to French Company Law
//!
//! French company law (droit des sociétés) is codified primarily in Book II of the Code de commerce
//! (Articles L210-1 to L242-33). It governs the formation, governance, and dissolution of commercial
//! companies in France.
//!
//! ## Historical Foundation: The Napoleonic Code de Commerce (1807)
//!
//! The modern French company law system traces its roots to the Napoleonic Code de commerce of 1807,
//! which established basic principles of commercial law during Napoleon I's reign. This foundational
//! code introduced:
//!
//! - **Commercial court system** (tribunaux de commerce) - specialized merchant courts
//! - **Partnership forms** (société en nom collectif, société en commandite)
//! - **Legal personality** concept (personne morale distincte from owners)
//! - **Bankruptcy procedures** (faillite)
//!
//! However, the 1807 Code did not yet recognize true limited liability corporations. The société anonyme
//! (SA) existed in embryonic form but required case-by-case government authorization, limiting its use
//! to major infrastructure projects (canals, bridges, railways).
//!
//! ## The 1867 Revolution: Birth of Modern SA
//!
//! The **Law of July 24, 1867** transformed French capitalism by:
//!
//! 1. **Abolishing authorization requirement**: SAs could form by simple registration (like partnerships)
//! 2. **Generalizing limited liability**: All shareholders protected (previously only silent partners)
//! 3. **Minimum capital rules**: Introduced capital requirements for creditor protection
//! 4. **Mandatory disclosure**: Required publication of statuts and annual accounts
//!
//! This liberalization unleashed industrial development in the Second Empire and Third Republic. Major
//! companies founded in this era include Crédit Lyonnais (1863), Société Générale (1864), and countless
//! industrial concerns (mines, steel, textiles).
//!
//! **Comparative context**: France's 1867 reform paralleled UK's 1855-1862 limited liability acts and
//! Germany's 1870 corporation law. This reflected Europe-wide shift toward corporate capitalism.
//!
//! ## 20th Century Reforms: Democratization and Specialization
//!
//! ### 1925: SARL Creation
//!
//! The **Law of March 7, 1925** introduced the SARL (Société à Responsabilité Limitée), inspired by
//! Germany's GmbH (1892). Rationale:
//!
//! - SA too complex/expensive for small merchants and artisans
//! - Need for "middle-class" limited liability vehicle
//! - Combine partnership intimacy (close ownership) with corporate protection (limited liability)
//!
//! Initial features: 20,000 francs minimum capital, max 50 partners, gérant management system.
//!
//! ### 1966: Comprehensive Company Law Reform
//!
//! The **Law of July 24, 1966** (Loi sur les sociétés commerciales) constituted the first systematic
//! codification of company law since 1807. Major innovations:
//!
//! - **SA modernization**: Fixed board size (3-12), director terms (6 years), voting rules
//! - **Dualist option**: Allowed two-tier structure (directoire + conseil de surveillance)
//! - **SARL refinement**: Increased partner cap to 50 (later 100), simplified formalities
//! - **Minority protections**: Enhanced shareholder litigation rights (action sociale/individuelle)
//! - **Auditor requirements**: Mandatory commissaire aux comptes for transparency
//!
//! The 1966 Law's provisions were later integrated into the Code de commerce (2000 consolidation).
//!
//! ### 1994: SAS Revolution
//!
//! The **Law of January 3, 1994** created the SAS (Société par Actions Simplifiée), driven by:
//!
//! - **Private equity demands**: Investors wanted flexible governance (board observers, veto rights)
//! - **Family business needs**: Wanted share structure (actions) without SA rigidity
//! - **Competitiveness**: France losing incorporations to Delaware, Luxembourg due to SA constraints
//!
//! SAS features: No board requirement (only président), total governance freedom in statuts, initially
//! €250,000 minimum capital (reduced to €1 in 1999).
//!
//! **Impact**: By 2020, SAS accounts for 65% of new formations. Displaced SARL as preferred startup form.
//!
//! ### 2001: NRE Law (Corporate Governance)
//!
//! The **Loi sur les Nouvelles Régulations Économiques (2001)** enhanced governance standards:
//!
//! - **Board size**: Increased SA maximum from 15 to 18 directors
//! - **Transparency**: Stricter disclosure of executive compensation (rémunération des dirigeants)
//! - **Employee representation**: Enhanced works council (comité d'entreprise) consultation rights
//! - **Consolidation**: Required group-level accounts for subsidiaries
//!
//! Driven by corporate scandals (Vivendi, Enron globally) and investor protection demands.
//!
//! ### 2011: Copé-Zimmermann Law (Gender Quotas)
//!
//! The **Law of January 27, 2011** imposed binding gender diversity requirements:
//!
//! - **40% quota**: Boards of 8+ members must have 40% of each sex (phased: 20% by 2014, 40% by 2017)
//! - **Sanctions**: Appointments violating quota are void; director fees suspended
//! - **Scope**: Applies to SA, SCA, SAS with >500 employees and >€50M revenue
//!
//! **Results**: France went from 10% women directors (2009) to 45% (2022), highest in OECD.
//!
//! ### 2019: PACTE Law (Stakeholder Capitalism)
//!
//! The **Loi relative à la croissance et la transformation des entreprises (2019)** redefined corporate purpose:
//!
//! - **Article 1833 Code civil amended**: Companies must consider social/environmental impacts, not just profit
//! - **Raison d'être**: SAs can voluntarily adopt mission/purpose in statuts (Entreprise à Mission)
//! - **Employee profit-sharing**: Expanded participation and épargne salariale requirements
//! - **Simplification**: Reduced capital requirements for transformations (SA → SAS)
//!
//! Reflects shift toward ESG (environmental, social, governance) capitalism globally.
//!
//! ## Company Types: SA, SARL, SAS
//!
//! French commercial law recognizes three dominant company types:
//!
//! - **SA (Société Anonyme)**: Stock company, €37,000 minimum capital, mandatory board (3-18 directors)
//!   - Flagship form for large corporations and public offerings
//!   - CAC 40 presence: 38/40 companies (including SCA variants)
//!   - ~2% of new incorporations (2,500/year) but dominates large enterprise
//!
//! - **SARL (Société à Responsabilité Limitée)**: LLC, €1 minimum capital, max 100 partners
//!   - Traditional SME form, family-friendly (parts sociales less liquid than actions)
//!   - Gérant management system (no board requirement)
//!   - ~30% of new incorporations (32,000/year)
//!
//! - **SAS (Société par Actions Simplifiée)**: Simplified stock company, €1 minimum capital
//!   - Fastest-growing form, dominant in startups/tech
//!   - Total governance freedom (defined in statuts)
//!   - ~65% of new incorporations (70,000/year)
//!
//! ## Module Structure
//!
//! This module provides:
//!
//! - **Types** (`types.rs`): Core data structures (ArticlesOfIncorporation, BoardOfDirectors, etc.)
//! - **SA articles** (`sa.rs`): Statute implementations for Articles L225-1, L225-17, L225-18
//! - **Validation** (`validator.rs`): Compliance checking functions
//! - **Errors** (`error.rs`): Comprehensive error types for violations
//!
//! ## Usage Examples
//!
//! ### Validate SA formation
//!
//! ```
//! use legalis_fr::company::{
//!     ArticlesOfIncorporation, Capital, CompanyType, Shareholder,
//!     validate_articles_of_incorporation,
//! };
//!
//! let articles = ArticlesOfIncorporation::new(
//!     "TechCorp SA".to_string(),
//!     CompanyType::SA,
//!     Capital::new(100_000),
//! )
//! .with_business_purpose("Software development".to_string())
//! .with_head_office("Paris, France".to_string())
//! .with_shareholder(Shareholder::new("Founder".to_string(), 1000, 100_000));
//!
//! assert!(validate_articles_of_incorporation(&articles).is_ok());
//! ```
//!
//! ### Validate board of directors
//!
//! ```
//! use legalis_fr::company::{BoardOfDirectors, Director, validate_sa_board};
//! use chrono::Utc;
//!
//! let board = BoardOfDirectors::new()
//!     .with_director(Director::new(
//!         "Director 1".to_string(),
//!         Utc::now().naive_utc().date(),
//!         6,
//!     ))
//!     .with_director(Director::new(
//!         "Director 2".to_string(),
//!         Utc::now().naive_utc().date(),
//!         6,
//!     ))
//!     .with_director(Director::new(
//!         "Director 3".to_string(),
//!         Utc::now().naive_utc().date(),
//!         6,
//!     ));
//!
//! assert!(validate_sa_board(&board).is_ok());
//! ```
//!
//! ## Further Reading
//!
//! - **Code de commerce**: Official text at Légifrance (legifrance.gouv.fr)
//! - **AMF (Autorité des Marchés Financiers)**: Listed company regulations (amf-france.org)
//! - **AFEP-MEDEF Code**: Corporate governance soft law (afep.com)
//! - **Memento Sociétés Commerciales (Francis Lefebvre)**: Practitioner reference
//! - **Anonim**: Historical resource on French company law evolution

pub mod error;
pub mod sa;
pub mod types;
pub mod validator;

// Re-export key types
pub use error::{CompanyLawError, ValidationResult};
pub use types::{
    ArticlesOfIncorporation, BoardOfDirectors, Capital, CompanyType, Director, DirectorPosition,
    MeetingType, ResolutionType, Shareholder, ShareholdersMeeting,
};
pub use validator::{
    validate_articles_of_incorporation, validate_sa_board, validate_shareholders_meeting,
};

// Re-export SA articles
pub use sa::{article_l225_1, article_l225_17, article_l225_18};
