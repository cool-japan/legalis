//! Common Utilities for Chinese Law
//!
//! Provides shared utilities including:
//! - Date handling (Chinese calendar considerations)
//! - Currency formatting (RMB/CNY)
//! - Name handling (Chinese name conventions)
//! - Public holidays
//!
//! # 通用工具 / Common Utilities

pub mod currency;
pub mod dates;
pub mod names;

pub use currency::*;
pub use dates::*;
pub use names::*;
