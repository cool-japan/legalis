//! Intellectual Property Law Module (知的財産法モジュール)
//!
//! This module provides comprehensive support for Japanese intellectual property law,
//! including the Patent Act (特許法), Copyright Act (著作権法), Trademark Act (商標法),
//! and Design Act (意匠法).
//!
//! # Features
//!
//! - Patent application and grant validation (特許出願・査定検証)
//! - Copyright work validation and fair use analysis (著作権・権利制限分析)
//! - Trademark registration and similarity assessment (商標登録・類似性評価)
//! - Design registration and protection validation (意匠登録・保護検証)
//! - Infringement detection and analysis (侵害検出・分析)
//! - Type-safe validation and error handling
//!
//! # Legal Framework
//!
//! ## Patent Act (特許法 - Act No. 121 of 1959)
//!
//! The Patent Act protects inventions that are:
//! - Industrially applicable (産業上の利用可能性) - Article 29 preamble
//! - Novel (新規性) - Article 29-1
//! - Inventive (進歩性) - Article 29-2
//!
//! Protection period: 20 years from filing date (Article 67)
//!
//! ## Copyright Act (著作権法 - Act No. 48 of 1970)
//!
//! The Copyright Act protects original works of authorship:
//! - Literary, musical, artistic works (Article 10)
//! - Economic rights (Articles 21-28): reproduction, performance, public transmission, etc.
//! - Moral rights (Articles 18-20): right to make public, name, integrity
//! - Fair use limitations (Articles 30-47-8): private use, quotation, education
//!
//! Protection period: Author's life + 70 years (Article 51)
//!
//! ## Trademark Act (商標法 - Act No. 127 of 1959)
//!
//! The Trademark Act protects distinctive signs:
//! - Word, logo, combined, 3D, color, sound marks (Article 2)
//! - Must be distinctive (Article 3)
//! - Nice Classification system (Classes 1-45)
//!
//! Protection period: 10 years, renewable indefinitely (Article 19)
//!
//! ## Design Act (意匠法 - Act No. 125 of 1959)
//!
//! The Design Act protects aesthetic designs:
//! - Product, partial, related designs (Article 2)
//! - Must be novel and non-obvious (Article 3)
//!
//! Protection period: 25 years from registration (Article 21)
//!
//! # Examples
//!
//! ## Validating a Patent Application
//!
//! ```rust
//! use legalis_jp::intellectual_property::*;
//! use chrono::Utc;
//!
//! let application = PatentApplication {
//!     application_number: "2020-123456".to_string(),
//!     filing_date: Utc::now(),
//!     title: "Novel Device for Efficient Processing".to_string(),
//!     inventors: vec!["山田太郎".to_string()],
//!     applicants: vec!["テック株式会社".to_string()],
//!     category: InventionCategory::Product,
//!     claims: vec!["A device comprising...".to_string()],
//!     abstract_text: "This invention relates to...".to_string(),
//!     priority_date: None,
//!     examination_requested: false,
//! };
//!
//! assert!(validate_patent_application(&application).is_ok());
//! ```
//!
//! ## Assessing Trademark Similarity
//!
//! ```rust
//! use legalis_jp::intellectual_property::*;
//!
//! let similarity = assess_trademark_similarity("ACME", "ACNE");
//! assert_eq!(similarity, SimilarityLevel::Similar);
//! ```
//!
//! ## Validating Copyright Work
//!
//! ```rust
//! use legalis_jp::intellectual_property::*;
//! use chrono::Utc;
//!
//! let work = CopyrightedWork {
//!     title: "小説・春の物語".to_string(),
//!     authors: vec!["作家名".to_string()],
//!     category: WorkCategory::Literary,
//!     creation_date: Utc::now(),
//!     first_publication_date: Some(Utc::now()),
//!     copyright_holder: "出版社".to_string(),
//!     is_work_for_hire: false,
//!     derivative_source: None,
//! };
//!
//! assert!(validate_copyrighted_work(&work).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{IntellectualPropertyError, Result};
pub use types::*;
pub use validator::*;
