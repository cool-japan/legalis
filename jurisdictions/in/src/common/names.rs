//! Indian Name Formatting and Conventions
//!
//! India has diverse naming conventions across different communities, religions, and regions.
//! This module provides utilities for handling Indian names appropriately.
//!
//! ## Naming Patterns
//!
//! - **North Indian**: Given name + Surname (e.g., Rajesh Kumar)
//! - **South Indian**: Given name + Father's name + Surname (e.g., Ramesh Venkataraman Iyer)
//! - **Sikh**: Given name + Singh/Kaur (religious surname)
//! - **Single Name**: Some individuals use only one name
//!
//! ## Usage
//!
//! ```rust
//! use legalis_in::common::names::*;
//!
//! let name = IndianName::new("Rajesh Kumar Sharma")
//!     .with_title(Title::Mr);
//!
//! assert_eq!(name.full_name(), "Mr. Rajesh Kumar Sharma");
//! ```

use serde::{Deserialize, Serialize};

/// Title or honorific
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Title {
    /// Mr. (male)
    Mr,
    /// Mrs. (married female)
    Mrs,
    /// Ms. (female, marital status unspecified)
    Ms,
    /// Dr. (doctorate holder)
    Dr,
    /// Prof. (professor)
    Prof,
    /// Shri (respectful form for males)
    Shri,
    /// Smt. (respectful form for married females)
    Smt,
    /// Kumari (respectful form for unmarried females)
    Kumari,
    /// Hon. (honorable, for judges, ministers)
    Hon,
    /// Justice (for judges)
    Justice,
}

impl Title {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Title::Mr => "Mr.",
            Title::Mrs => "Mrs.",
            Title::Ms => "Ms.",
            Title::Dr => "Dr.",
            Title::Prof => "Prof.",
            Title::Shri => "Shri",
            Title::Smt => "Smt.",
            Title::Kumari => "Kumari",
            Title::Hon => "Hon.",
            Title::Justice => "Justice",
        }
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Indian person name
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IndianName {
    /// Title or honorific
    pub title: Option<Title>,
    /// Full name as provided
    pub full_name: String,
    /// Given name (first name)
    pub given_name: Option<String>,
    /// Middle name or father's name
    pub middle_name: Option<String>,
    /// Surname or family name
    pub surname: Option<String>,
    /// Initials (common in South India)
    pub initials: Option<String>,
}

impl IndianName {
    /// Create a new Indian name with just the full name
    pub fn new(full_name: impl Into<String>) -> Self {
        Self {
            title: None,
            full_name: full_name.into(),
            given_name: None,
            middle_name: None,
            surname: None,
            initials: None,
        }
    }

    /// Create a name with detailed components
    pub fn from_components(
        given_name: impl Into<String>,
        middle_name: Option<String>,
        surname: Option<String>,
    ) -> Self {
        let given = given_name.into();
        let full = Self::construct_full_name(&given, middle_name.as_deref(), surname.as_deref());

        Self {
            title: None,
            full_name: full,
            given_name: Some(given),
            middle_name,
            surname,
            initials: None,
        }
    }

    /// Set the title
    pub fn with_title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    /// Set initials (common in South India, e.g., "A. R." in "A. R. Rahman")
    pub fn with_initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    /// Get the formatted full name with title
    pub fn formatted_with_title(&self) -> String {
        match self.title {
            Some(title) => format!("{} {}", title, self.full_name),
            None => self.full_name.clone(),
        }
    }

    /// Get the display name (full name without title)
    pub fn display_name(&self) -> &str {
        &self.full_name
    }

    /// Get the formal name (with title if available)
    pub fn formal_name(&self) -> String {
        self.formatted_with_title()
    }

    /// Get initials and surname format (e.g., "A. R. Rahman" -> "A. R. Rahman")
    pub fn initials_format(&self) -> String {
        match (&self.initials, &self.surname) {
            (Some(init), Some(sur)) => format!("{} {}", init, sur),
            _ => self.full_name.clone(),
        }
    }

    /// Construct full name from components
    fn construct_full_name(given: &str, middle: Option<&str>, surname: Option<&str>) -> String {
        let mut parts = vec![given];
        if let Some(m) = middle {
            parts.push(m);
        }
        if let Some(s) = surname {
            parts.push(s);
        }
        parts.join(" ")
    }

    /// Parse a full name into components (simple heuristic)
    pub fn parse(full_name: impl Into<String>) -> Self {
        let full = full_name.into();
        let parts: Vec<&str> = full.split_whitespace().collect();

        let (given_name, middle_name, surname) = match parts.len() {
            0 => (None, None, None),
            1 => (Some(parts[0].to_string()), None, None),
            2 => (Some(parts[0].to_string()), None, Some(parts[1].to_string())),
            3 => (
                Some(parts[0].to_string()),
                Some(parts[1].to_string()),
                Some(parts[2].to_string()),
            ),
            _ => {
                // For more than 3 parts, take first as given, last as surname, rest as middle
                let given = Some(parts[0].to_string());
                let surname = Some(parts[parts.len() - 1].to_string());
                let middle = Some(parts[1..parts.len() - 1].join(" "));
                (given, middle, surname)
            }
        };

        Self {
            title: None,
            full_name: full,
            given_name,
            middle_name,
            surname,
            initials: None,
        }
    }
}

impl std::fmt::Display for IndianName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_with_title())
    }
}

impl Default for IndianName {
    fn default() -> Self {
        Self::new("")
    }
}

/// Name formatter for Indian names
pub struct IndianNameFormatter;

impl IndianNameFormatter {
    /// Format a name for formal documents
    pub fn format_formal(name: &IndianName) -> String {
        name.formal_name()
    }

    /// Format a name for informal use
    pub fn format_informal(name: &IndianName) -> String {
        match &name.given_name {
            Some(given) => given.clone(),
            None => name.full_name.clone(),
        }
    }

    /// Format a name with last name first (e.g., "Sharma, Rajesh Kumar")
    pub fn format_last_name_first(name: &IndianName) -> String {
        match (&name.surname, &name.given_name, &name.middle_name) {
            (Some(sur), Some(given), Some(middle)) => {
                format!("{}, {} {}", sur, given, middle)
            }
            (Some(sur), Some(given), None) => {
                format!("{}, {}", sur, given)
            }
            _ => name.full_name.clone(),
        }
    }

    /// Format initials from a full name
    pub fn extract_initials(name: &str) -> String {
        name.split_whitespace()
            .filter_map(|part| part.chars().next())
            .map(|c| format!("{}.", c.to_uppercase()))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_display() {
        assert_eq!(Title::Mr.as_str(), "Mr.");
        assert_eq!(Title::Shri.as_str(), "Shri");
        assert_eq!(Title::Justice.as_str(), "Justice");
    }

    #[test]
    fn test_indian_name_new() {
        let name = IndianName::new("Rajesh Kumar Sharma");
        assert_eq!(name.display_name(), "Rajesh Kumar Sharma");
        assert_eq!(name.formal_name(), "Rajesh Kumar Sharma");
    }

    #[test]
    fn test_indian_name_with_title() {
        let name = IndianName::new("Rajesh Kumar Sharma").with_title(Title::Mr);
        assert_eq!(name.formal_name(), "Mr. Rajesh Kumar Sharma");
    }

    #[test]
    fn test_indian_name_from_components() {
        let name = IndianName::from_components(
            "Rajesh",
            Some("Kumar".to_string()),
            Some("Sharma".to_string()),
        );
        assert_eq!(name.given_name, Some("Rajesh".to_string()));
        assert_eq!(name.middle_name, Some("Kumar".to_string()));
        assert_eq!(name.surname, Some("Sharma".to_string()));
        assert_eq!(name.display_name(), "Rajesh Kumar Sharma");
    }

    #[test]
    fn test_indian_name_parse() {
        let name = IndianName::parse("Rajesh Kumar Sharma");
        assert_eq!(name.given_name, Some("Rajesh".to_string()));
        assert_eq!(name.middle_name, Some("Kumar".to_string()));
        assert_eq!(name.surname, Some("Sharma".to_string()));

        let single = IndianName::parse("Ramesh");
        assert_eq!(single.given_name, Some("Ramesh".to_string()));
        assert_eq!(single.middle_name, None);
        assert_eq!(single.surname, None);
    }

    #[test]
    fn test_initials_format() {
        let name = IndianName::from_components(
            "Abdul",
            Some("Rahman".to_string()),
            Some("Khan".to_string()),
        )
        .with_initials("A. R.");
        assert_eq!(name.initials_format(), "A. R. Khan");
    }

    #[test]
    fn test_name_formatter() {
        let name = IndianName::from_components(
            "Rajesh",
            Some("Kumar".to_string()),
            Some("Sharma".to_string()),
        );

        assert_eq!(
            IndianNameFormatter::format_formal(&name),
            "Rajesh Kumar Sharma"
        );
        assert_eq!(IndianNameFormatter::format_informal(&name), "Rajesh");
        assert_eq!(
            IndianNameFormatter::format_last_name_first(&name),
            "Sharma, Rajesh Kumar"
        );
    }

    #[test]
    fn test_extract_initials() {
        let initials = IndianNameFormatter::extract_initials("Rajesh Kumar Sharma");
        assert_eq!(initials, "R. K. S.");
    }
}
