//! Case Law Database System (判例データベースシステム)
//!
//! This module provides a comprehensive system for managing and searching Japanese
//! court decisions (判例 - Hanrei).
//!
//! # Features
//!
//! - **Court Decision Data Model** (裁判例データモデル)
//!   - Comprehensive metadata (case numbers, dates, parties)
//!   - Holdings and legal principles (判旨・法理)
//!   - Citation tracking (引用追跡)
//!
//! - **Search Engine** (検索エンジン)
//!   - Keyword search with relevance scoring
//!   - Multi-dimensional filtering (court level, legal area, date range)
//!   - Cited statute filtering
//!   - Similar case discovery
//!
//! - **Citation Formatting** (引用形式)
//!   - Japanese legal citation format (標準引用形式)
//!   - Short citation format (短縮引用)
//!   - Blue Book style (American legal citation)
//!   - Full citation with URLs
//!
//! - **Database Abstraction** (データベース抽象化)
//!   - Trait-based design for multiple backends
//!   - In-memory implementation for testing
//!   - Extensible for SQL/NoSQL backends
//!
//! # Japanese Court System
//!
//! The Japanese judicial system consists of five levels of courts:
//!
//! 1. **Supreme Court** (最高裁判所 - Saikō-saibansho)
//!    - Highest court, final appellate authority
//!    - Decisions have binding precedent value
//!
//! 2. **High Courts** (高等裁判所 - Kōtō-saibansho)
//!    - Appellate courts (8 locations nationwide)
//!    - Strong persuasive authority
//!
//! 3. **District Courts** (地方裁判所 - Chihō-saibansho)
//!    - Trial courts of general jurisdiction (50 locations)
//!    - Handle civil and criminal first instance cases
//!
//! 4. **Family Courts** (家庭裁判所 - Katei-saibansho)
//!    - Specialized family law jurisdiction
//!    - Same level as District Courts
//!
//! 5. **Summary Courts** (簡易裁判所 - Kan'i-saibansho)
//!    - Limited jurisdiction for minor civil/criminal matters
//!
//! # Citation Formats
//!
//! ## Standard Japanese Format (標準形式)
//!
//! ```text
//! 最高裁判所令和2年1月10日判決 令和元年(受)第1234号
//! (Supreme Court, January 10, 2020, Reiwa 1 (Ju) No. 1234)
//! ```
//!
//! ## Short Format (短縮形式)
//!
//! ```text
//! 最判令和2年1月10日
//! (Supreme Court Decision, January 10, 2020)
//! ```
//!
//! # Examples
//!
//! ## Creating a Court Decision
//!
//! ```rust
//! use legalis_jp::case_law::*;
//! use chrono::Utc;
//!
//! let metadata = CaseMetadata::new(
//!     "令和2年(受)第1234号",
//!     Utc::now(),
//!     Court::new(CourtLevel::Supreme).with_location("Tokyo"),
//!     LegalArea::Civil,
//!     CaseOutcome::PlaintiffWins,
//! );
//!
//! let mut decision = CourtDecision::new(
//!     "case-001",
//!     metadata,
//!     "事案の概要: 不法行為に基づく損害賠償請求"
//! );
//!
//! // Add holding
//! decision.add_holding(Holding {
//!     principle: "民法709条の不法行為の成立要件".to_string(),
//!     reasoning: "故意または過失により...".to_string(),
//!     related_statutes: vec!["民法第709条".to_string()],
//!     is_leading_case: true,
//! });
//! ```
//!
//! ## Searching for Cases
//!
//! ```rust
//! use legalis_jp::case_law::*;
//!
//! let mut db = InMemoryCaseDatabase::new();
//! let engine = CaseLawSearchEngine::new(db);
//!
//! let query = CaseSearchQuery::new()
//!     .with_keyword("不法行為")
//!     .with_court_level(CourtLevel::Supreme)
//!     .with_legal_area(LegalArea::Civil)
//!     .with_limit(10);
//!
//! // Search returns results sorted by relevance
//! match engine.search(&query) {
//!     Ok(results) => {
//!         for result in results {
//!             println!("Case: {} (Relevance: {:.2})",
//!                 result.decision.id,
//!                 result.relevance_score
//!             );
//!         }
//!     }
//!     Err(e) => eprintln!("Search error: {}", e),
//! }
//! ```
//!
//! ## Formatting Citations
//!
//! ```rust
//! use legalis_jp::case_law::*;
//!
//! // Assuming you have a CourtDecision
//! # use chrono::Utc;
//! # let metadata = CaseMetadata::new(
//! #     "令和2年(受)第1234号",
//! #     Utc::now(),
//! #     Court::new(CourtLevel::Supreme),
//! #     LegalArea::Civil,
//! #     CaseOutcome::PlaintiffWins,
//! # );
//! # let decision = CourtDecision::new("case-001", metadata, "Test");
//!
//! // Standard format
//! let citation = CitationFormatter::format(&decision, CitationStyle::Standard).unwrap();
//! println!("{}", citation);
//!
//! // Short format
//! let short = CitationFormatter::format(&decision, CitationStyle::Short).unwrap();
//! println!("{}", short);
//! ```
//!
//! # Legal References
//!
//! - Court Act (裁判所法 - Act No. 59 of 1947)
//! - Code of Civil Procedure (民事訴訟法 - Act No. 109 of 1996)
//! - Code of Criminal Procedure (刑事訴訟法 - Act No. 131 of 1948)
//!
//! # Database Integration
//!
//! The `CaseLawDatabase` trait allows integration with various backends:
//!
//! - **In-memory**: `InMemoryCaseDatabase` - for testing and small datasets
//! - **SQL**: Implement trait for PostgreSQL, MySQL, SQLite
//! - **NoSQL**: Implement trait for MongoDB, Elasticsearch
//! - **External APIs**: Implement trait for courts.go.jp scraper

pub mod citation;
pub mod error;
pub mod search;
pub mod types;

// Re-export commonly used types and functions
pub use citation::{CitationFormatter, CitationStyle};
pub use error::{CaseLawError, Result};
pub use search::{CaseLawDatabase, CaseLawSearchEngine, InMemoryCaseDatabase};
pub use types::*;
