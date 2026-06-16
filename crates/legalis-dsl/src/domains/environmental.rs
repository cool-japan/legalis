//! Environmental-regulation specialized syntax (roadmap v0.3.2).
//!
//! Recognized condition forms:
//!
//! ```text
//! EMISSION_LIMIT <pollutant> [<op>] <value> [<unit>]   ; an emission ceiling
//! THRESHOLD <parameter> [<op>] <value> [<unit>]        ; a regulatory threshold
//! REPORTING_PERIOD <n> [<unit>]                         ; reporting cadence
//! MONITORING <parameter>                                ; a monitored parameter
//! ```
//!
//! Units are recognized for diagnostics (`mg/m3`, `kg`, `tonnes`, `ppm`, `ug/l`,
//! …) but, since the lexer drops `/`, multi-part units are written as a single
//! identifier (e.g. `mgm3`, `ugl`).

use super::{
    DomainDiagnostic, DomainKeyword, DomainOperator, DomainSeverity, LegalDomain, TokenCursor,
    domain_tokens, statute_atoms, value_as_f64,
};
use crate::ast::{ConditionNode, ConditionValue, StatuteNode};
use crate::{DslError, DslResult};

/// The environmental-regulation domain.
pub struct EnvironmentalDomain;

impl EnvironmentalDomain {
    /// Prefix used for lowered emission-limit fields.
    pub const EMISSION_PREFIX: &'static str = "emission_";
    /// Field used for the (normalized, in days) reporting period.
    pub const REPORTING_PERIOD_FIELD: &'static str = "reporting_period_days";

    /// Recognized measurement units (lexer-friendly spellings).
    pub const UNITS: &'static [&'static str] = &[
        "mgm3", "ugm3", "ppm", "ppb", "kg", "tonnes", "tons", "ugl", "mgl", "g", "percent", "db",
    ];
}

impl LegalDomain for EnvironmentalDomain {
    fn name(&self) -> &str {
        "environmental"
    }

    fn description(&self) -> &str {
        "Environmental regulation: emission limits, thresholds and reporting periods"
    }

    fn keywords(&self) -> Vec<DomainKeyword> {
        vec![
            DomainKeyword::new(
                "EMISSION_LIMIT",
                "Maximum permitted emission of a pollutant",
            ),
            DomainKeyword::new("THRESHOLD", "A regulatory threshold for a parameter"),
            DomainKeyword::new("REPORTING_PERIOD", "How often compliance must be reported"),
            DomainKeyword::new("MONITORING", "A parameter that must be monitored"),
        ]
    }

    fn operators(&self) -> Vec<DomainOperator> {
        vec![DomainOperator::new(
            "<unit>",
            "Optional measurement unit suffix",
        )]
    }

    fn parse_condition(&self, input: &str) -> DslResult<ConditionNode> {
        let tokens = domain_tokens(input)?;
        let mut cur = TokenCursor::new(&tokens);
        let keyword = cur
            .peek_word()
            .ok_or_else(|| DslError::parse_error("Expected an environmental keyword"))?;
        cur.advance();

        let node = match keyword.as_str() {
            "EMISSION_LIMIT" => {
                let pollutant = cur.expect_field()?;
                // Default operator for a *limit* is "<=" (must not exceed).
                let op = if matches!(cur.peek(), Some(crate::ast::Token::Operator(_))) {
                    cur.expect_comparison_op()
                } else {
                    "<=".to_string()
                };
                let value = cur.expect_number()?;
                let _unit = optional_unit(&mut cur);
                ConditionNode::Comparison {
                    field: format!("{}{}", Self::EMISSION_PREFIX, pollutant.to_lowercase()),
                    operator: op,
                    value: ConditionValue::Number(value as i64),
                }
            }
            "THRESHOLD" => {
                let parameter = cur.expect_field()?;
                let op = cur.expect_comparison_op();
                let value = cur.expect_number()?;
                let _unit = optional_unit(&mut cur);
                ConditionNode::Comparison {
                    field: format!("threshold_{}", parameter.to_lowercase()),
                    operator: op,
                    value: ConditionValue::Number(value as i64),
                }
            }
            "REPORTING_PERIOD" => {
                let n = cur.expect_number()?;
                let unit = if cur.is_eof() {
                    "days".to_string()
                } else {
                    cur.expect_string()?.to_lowercase()
                };
                let days = period_to_days(n, &unit);
                ConditionNode::Comparison {
                    field: Self::REPORTING_PERIOD_FIELD.to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::Number(days),
                }
            }
            "MONITORING" => {
                let parameter = cur.expect_field()?;
                ConditionNode::HasAttribute {
                    key: format!("monitoring_{}", parameter.to_lowercase()),
                }
            }
            other => {
                return Err(DslError::parse_error(format!(
                    "Unknown environmental keyword: '{other}'"
                )));
            }
        };
        cur.expect_eof()?;
        Ok(node)
    }

    fn validate_statute(&self, statute: &StatuteNode) -> Vec<DomainDiagnostic> {
        let mut diags = Vec::new();
        for atom in statute_atoms(statute) {
            match &atom {
                ConditionNode::Comparison { field, value, .. }
                    if field.starts_with(Self::EMISSION_PREFIX)
                        || field.starts_with("threshold_") =>
                {
                    if let Some(v) = value_as_f64(value)
                        && v < 0.0
                    {
                        diags.push(DomainDiagnostic::new(
                            self.name(),
                            DomainSeverity::Error,
                            "environmental.negative-limit",
                            format!("limit/threshold '{field}' cannot be negative ({v})"),
                        ));
                    }
                }
                ConditionNode::Comparison { field, value, .. }
                    if field == Self::REPORTING_PERIOD_FIELD =>
                {
                    if let Some(days) = value_as_f64(value)
                        && days <= 0.0
                    {
                        diags.push(DomainDiagnostic::new(
                            self.name(),
                            DomainSeverity::Error,
                            "environmental.invalid-reporting-period",
                            format!("reporting period must be positive ({days} days)"),
                        ));
                    }
                }
                _ => {}
            }
        }
        diags
    }
}

/// Consumes an optional unit identifier, emitting nothing if absent. Returns the
/// lowercased unit (or `None`).
fn optional_unit(cur: &mut TokenCursor) -> Option<String> {
    if matches!(cur.peek(), Some(crate::ast::Token::Ident(_)))
        || matches!(cur.peek(), Some(crate::ast::Token::StringLit(_)))
    {
        cur.expect_string().ok().map(|s| s.to_lowercase())
    } else {
        None
    }
}

/// Converts a reporting period in `unit`s to whole days.
fn period_to_days(n: f64, unit: &str) -> i64 {
    let factor = match unit {
        "day" | "days" | "daily" => 1.0,
        "week" | "weeks" | "weekly" => 7.0,
        "month" | "months" | "monthly" => 30.0,
        "quarter" | "quarters" | "quarterly" => 90.0,
        "year" | "years" | "yearly" | "annual" | "annually" => 365.0,
        _ => 1.0,
    };
    (n * factor).round() as i64
}
