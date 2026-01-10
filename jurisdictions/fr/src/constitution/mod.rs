//! French Constitution of 1958 (Constitution de la Cinquième République)
//!
//! This module provides the complete structure of the French Constitution,
//! including all 16 titles and 89 articles.
//!
//! ## Key Features
//!
//! - **16 Titles**: Complete title structure with bilingual text
//! - **89 Articles**: Full metadata for all constitutional articles
//! - **Semi-presidential system**: President + Prime Minister
//! - **Bicameral parliament**: National Assembly + Senate
//! - **Constitutional Council**: Judicial review
//!
//! ## Example
//!
//! ```
//! use legalis_fr::constitution::{all_titles, get_title};
//!
//! // Get all 16 titles
//! let titles = all_titles();
//! assert_eq!(titles.len(), 16);
//!
//! // Get Title II - President
//! let president_title = get_title(2).unwrap();
//! assert_eq!(president_title.article_count(), 15);
//! ```

pub mod titles;
pub mod types;

// Re-export key types
pub use titles::{all_titles, get_title, total_article_count};
pub use types::{ConstitutionArticle, ConstitutionTitle, FundamentalRight, Institution};
