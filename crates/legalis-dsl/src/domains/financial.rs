//! Financial-services-regulation specialized syntax (roadmap v0.3.2).
//!
//! Recognized condition forms (percentages are written as plain numbers; the
//! `%` is optional):
//!
//! ```text
//! CAPITAL_RATIO [<op>] <pct>          ; total/CET1 capital adequacy ratio
//! LIQUIDITY_RATIO [<op>] <pct>        ; LCR-style liquidity ratio
//! LEVERAGE_RATIO [<op>] <pct>         ; leverage ratio
//! RATIO <name> [<op>] <pct>           ; a named prudential ratio
//! REPORTING <frequency>               ; supervisory reporting cadence
//! ```
//!
//! Validation range-checks the ratios (non-negative, plausibly ≤ 1000%) and
//! recognizes the common Basel ratio names.

use super::{
    DomainDiagnostic, DomainKeyword, DomainOperator, DomainSeverity, LegalDomain, TokenCursor,
    domain_tokens, percent_value, statute_atoms, value_as_f64,
};
use crate::ast::{ConditionNode, ConditionValue, StatuteNode};
use crate::{DslError, DslResult};

/// The financial-services-regulation domain.
pub struct FinancialServicesDomain;

impl FinancialServicesDomain {
    /// Prefix used for lowered ratio fields.
    pub const RATIO_PREFIX: &'static str = "ratio_";

    /// Recognized prudential ratio names (lowercase).
    pub const KNOWN_RATIOS: &'static [&'static str] = &[
        "capital",
        "cet1",
        "tier1",
        "tier2",
        "total",
        "liquidity",
        "lcr",
        "nsfr",
        "leverage",
    ];
}

impl LegalDomain for FinancialServicesDomain {
    fn name(&self) -> &str {
        "financial"
    }

    fn description(&self) -> &str {
        "Financial services regulation: capital/liquidity ratios and reporting obligations"
    }

    fn keywords(&self) -> Vec<DomainKeyword> {
        vec![
            DomainKeyword::new("CAPITAL_RATIO", "Capital adequacy ratio (percent)"),
            DomainKeyword::new("LIQUIDITY_RATIO", "Liquidity coverage ratio (percent)"),
            DomainKeyword::new("LEVERAGE_RATIO", "Leverage ratio (percent)"),
            DomainKeyword::new("RATIO", "A named prudential ratio (percent)"),
            DomainKeyword::new("REPORTING", "Supervisory reporting cadence"),
        ]
    }

    fn operators(&self) -> Vec<DomainOperator> {
        vec![DomainOperator::new(
            "%",
            "Percent modifier (optional, lexer-ignored)",
        )]
    }

    fn parse_condition(&self, input: &str) -> DslResult<ConditionNode> {
        let tokens = domain_tokens(input)?;
        let mut cur = TokenCursor::new(&tokens);
        let keyword = cur
            .peek_word()
            .ok_or_else(|| DslError::parse_error("Expected a financial keyword"))?;
        cur.advance();

        let node = match keyword.as_str() {
            "CAPITAL_RATIO" => ratio_condition(&mut cur, "capital")?,
            "LIQUIDITY_RATIO" => ratio_condition(&mut cur, "liquidity")?,
            "LEVERAGE_RATIO" => ratio_condition(&mut cur, "leverage")?,
            "RATIO" => {
                let name = cur.expect_field()?.to_lowercase();
                ratio_condition(&mut cur, &name)?
            }
            "REPORTING" | "REPORT" => {
                let frequency = cur.expect_string()?.to_lowercase();
                ConditionNode::Comparison {
                    field: "reporting_frequency".to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::String(frequency),
                }
            }
            other => {
                return Err(DslError::parse_error(format!(
                    "Unknown financial keyword: '{other}'"
                )));
            }
        };
        cur.expect_eof()?;
        Ok(node)
    }

    fn validate_statute(&self, statute: &StatuteNode) -> Vec<DomainDiagnostic> {
        let mut diags = Vec::new();
        for atom in statute_atoms(statute) {
            if let ConditionNode::Comparison { field, value, .. } = &atom
                && let Some(ratio_name) = field.strip_prefix(Self::RATIO_PREFIX)
                && let Some(pct) = value_as_f64(value)
            {
                if pct < 0.0 {
                    diags.push(DomainDiagnostic::new(
                        self.name(),
                        DomainSeverity::Error,
                        "financial.ratio-negative",
                        format!("{ratio_name} ratio {pct}% cannot be negative"),
                    ));
                } else if pct > 1000.0 {
                    diags.push(DomainDiagnostic::new(
                        self.name(),
                        DomainSeverity::Warning,
                        "financial.ratio-implausible",
                        format!("{ratio_name} ratio {pct}% is implausibly large"),
                    ));
                }
                if !Self::KNOWN_RATIOS.contains(&ratio_name) {
                    diags.push(DomainDiagnostic::new(
                        self.name(),
                        DomainSeverity::Info,
                        "financial.unknown-ratio",
                        format!("'{ratio_name}' is not a recognized prudential ratio name"),
                    ));
                }
            }
        }
        diags
    }
}

/// Builds a `ratio_<name> <op> <pct>` comparison from the cursor.
fn ratio_condition(cur: &mut TokenCursor, name: &str) -> DslResult<ConditionNode> {
    let op = cur.expect_comparison_op();
    let pct = cur.expect_number()?;
    Ok(ConditionNode::Comparison {
        field: format!("{}{}", FinancialServicesDomain::RATIO_PREFIX, name),
        operator: op,
        value: percent_value(pct),
    })
}
