//! Healthcare-compliance specialized syntax (roadmap v0.3.2).
//!
//! Recognized condition forms:
//!
//! ```text
//! CONSENT <level>           ; explicit|implied|required|none|optout
//! DATA_CATEGORY <category>  ; phi|pii|sensitive|public|anonymized
//! RETENTION <n> [<unit>]    ; data retention period
//! PURPOSE <desc>            ; declared processing purpose
//! ```
//!
//! Validation recognizes the consent levels and data categories, requires a
//! positive retention period, and flags the privacy-sensitive combination of
//! protected/sensitive data with weak (implied/absent) consent.

use super::{
    DomainDiagnostic, DomainKeyword, DomainOperator, DomainSeverity, LegalDomain, TokenCursor,
    domain_tokens, statute_atoms, value_as_f64,
};
use crate::ast::{ConditionNode, ConditionValue, StatuteNode};
use crate::{DslError, DslResult};

/// The healthcare-compliance domain.
pub struct HealthcareDomain;

impl HealthcareDomain {
    /// Field used for the consent level.
    pub const CONSENT_FIELD: &'static str = "consent";
    /// Field used for the data category.
    pub const DATA_CATEGORY_FIELD: &'static str = "data_category";
    /// Field used for the (normalized, in days) retention period.
    pub const RETENTION_FIELD: &'static str = "retention_days";

    /// Recognized consent levels (lowercase).
    pub const CONSENT_LEVELS: &'static [&'static str] =
        &["explicit", "implied", "required", "none", "optout", "optin"];
    /// Recognized data categories (lowercase).
    pub const DATA_CATEGORIES: &'static [&'static str] = &[
        "phi",
        "pii",
        "sensitive",
        "public",
        "anonymized",
        "pseudonymized",
    ];
    /// Categories considered protected/sensitive for consent checks.
    pub const PROTECTED_CATEGORIES: &'static [&'static str] = &["phi", "sensitive", "pii"];
    /// Consent levels considered weak for protected data.
    pub const WEAK_CONSENT: &'static [&'static str] = &["none", "implied"];
}

impl LegalDomain for HealthcareDomain {
    fn name(&self) -> &str {
        "healthcare"
    }

    fn description(&self) -> &str {
        "Healthcare compliance: consent, data-category and retention constructs"
    }

    fn keywords(&self) -> Vec<DomainKeyword> {
        vec![
            DomainKeyword::new("CONSENT", "The required consent level"),
            DomainKeyword::new("DATA_CATEGORY", "The category of data processed"),
            DomainKeyword::new("RETENTION", "Data retention period"),
            DomainKeyword::new("PURPOSE", "Declared processing purpose"),
        ]
    }

    fn operators(&self) -> Vec<DomainOperator> {
        vec![DomainOperator::new(
            "<unit>",
            "Optional retention unit (days/months/years)",
        )]
    }

    fn parse_condition(&self, input: &str) -> DslResult<ConditionNode> {
        let tokens = domain_tokens(input)?;
        let mut cur = TokenCursor::new(&tokens);
        let keyword = cur
            .peek_word()
            .ok_or_else(|| DslError::parse_error("Expected a healthcare keyword"))?;
        cur.advance();

        let node = match keyword.as_str() {
            "CONSENT" => {
                let level = cur.expect_string()?.to_lowercase();
                ConditionNode::Comparison {
                    field: Self::CONSENT_FIELD.to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::String(level),
                }
            }
            "DATA_CATEGORY" => {
                let category = cur.expect_string()?.to_lowercase();
                ConditionNode::Comparison {
                    field: Self::DATA_CATEGORY_FIELD.to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::String(category),
                }
            }
            "RETENTION" => {
                let n = cur.expect_number()?;
                let unit = if cur.is_eof() {
                    "days".to_string()
                } else {
                    cur.expect_string()?.to_lowercase()
                };
                ConditionNode::Comparison {
                    field: Self::RETENTION_FIELD.to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::Number(retention_to_days(n, &unit)),
                }
            }
            "PURPOSE" => {
                let desc = cur.expect_string()?;
                ConditionNode::Comparison {
                    field: "purpose".to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::String(desc),
                }
            }
            other => {
                return Err(DslError::parse_error(format!(
                    "Unknown healthcare keyword: '{other}'"
                )));
            }
        };
        cur.expect_eof()?;
        Ok(node)
    }

    fn validate_statute(&self, statute: &StatuteNode) -> Vec<DomainDiagnostic> {
        let mut diags = Vec::new();
        let atoms = statute_atoms(statute);

        let mut consent: Option<String> = None;
        let mut category: Option<String> = None;

        for atom in &atoms {
            match atom {
                ConditionNode::Comparison {
                    field,
                    value: ConditionValue::String(level),
                    ..
                } if field.eq_ignore_ascii_case(Self::CONSENT_FIELD) => {
                    let level = level.to_lowercase();
                    if !Self::CONSENT_LEVELS.contains(&level.as_str()) {
                        diags.push(DomainDiagnostic::new(
                            self.name(),
                            DomainSeverity::Error,
                            "healthcare.unknown-consent",
                            format!("unrecognized consent level '{level}'"),
                        ));
                    }
                    consent = Some(level);
                }
                ConditionNode::Comparison {
                    field,
                    value: ConditionValue::String(cat),
                    ..
                } if field.eq_ignore_ascii_case(Self::DATA_CATEGORY_FIELD) => {
                    let cat = cat.to_lowercase();
                    if !Self::DATA_CATEGORIES.contains(&cat.as_str()) {
                        diags.push(DomainDiagnostic::new(
                            self.name(),
                            DomainSeverity::Error,
                            "healthcare.unknown-data-category",
                            format!("unrecognized data category '{cat}'"),
                        ));
                    }
                    category = Some(cat);
                }
                ConditionNode::Comparison { field, value, .. }
                    if field.eq_ignore_ascii_case(Self::RETENTION_FIELD) =>
                {
                    if let Some(days) = value_as_f64(value)
                        && days <= 0.0
                    {
                        diags.push(DomainDiagnostic::new(
                            self.name(),
                            DomainSeverity::Error,
                            "healthcare.invalid-retention",
                            format!("retention period must be positive ({days} days)"),
                        ));
                    }
                }
                _ => {}
            }
        }

        // Privacy invariant: protected/sensitive data with weak consent.
        if let (Some(cat), Some(level)) = (&category, &consent)
            && Self::PROTECTED_CATEGORIES.contains(&cat.as_str())
            && Self::WEAK_CONSENT.contains(&level.as_str())
        {
            diags.push(DomainDiagnostic::new(
                self.name(),
                DomainSeverity::Warning,
                "healthcare.weak-consent-for-protected-data",
                format!(
                    "data category '{cat}' should require explicit consent, but consent is '{level}'"
                ),
            ));
        }

        diags
    }
}

/// Converts a retention period in `unit`s to whole days.
fn retention_to_days(n: f64, unit: &str) -> i64 {
    let factor = match unit {
        "day" | "days" => 1.0,
        "week" | "weeks" => 7.0,
        "month" | "months" => 30.0,
        "year" | "years" => 365.0,
        _ => 1.0,
    };
    (n * factor).round() as i64
}
