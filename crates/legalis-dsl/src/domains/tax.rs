//! Tax-law specialized syntax (roadmap v0.3.2).
//!
//! Recognized condition forms (the leading keyword is case-insensitive; a
//! trailing `%`/`$` is optional and ignored by the lexer):
//!
//! ```text
//! BRACKET <base> FROM <lo> TO <hi> [RATE <pct>]   ; income/base in a bracket
//! RATE [<op>] <pct>                               ; applicable marginal rate
//! THRESHOLD <base> [<op>] <amount>                ; a filing/relief threshold
//! TAXABLE_BASE <base> [<op>] <amount>             ; the taxable base value
//! DEDUCTION <name> [<op>] <amount>                ; a deduction amount
//! EXEMPTION <base> <amount>                       ; exempt below an amount
//! ```
//!
//! Brackets lower to `<base> BETWEEN <lo> AND <hi>` (conjoined with `rate ==
//! <pct>` when a rate is given), so the result is an ordinary
//! [`ConditionNode`].

use super::{
    DomainDiagnostic, DomainKeyword, DomainOperator, DomainSeverity, LegalDomain, TokenCursor,
    domain_tokens, percent_value, statute_atoms, value_as_f64,
};
use crate::ast::{ConditionNode, ConditionValue, StatuteNode};
use crate::{DslError, DslResult};

/// The tax-law domain.
pub struct TaxLawDomain;

impl TaxLawDomain {
    /// The field name used for the applicable rate in lowered conditions.
    pub const RATE_FIELD: &'static str = "rate";
}

impl LegalDomain for TaxLawDomain {
    fn name(&self) -> &str {
        "tax"
    }

    fn description(&self) -> &str {
        "Tax law: brackets, marginal rates, thresholds and taxable-base constructs"
    }

    fn keywords(&self) -> Vec<DomainKeyword> {
        vec![
            DomainKeyword::new("BRACKET", "Taxable base falling within a rate band"),
            DomainKeyword::new("RATE", "Applicable marginal tax rate (percent)"),
            DomainKeyword::new("THRESHOLD", "A filing or relief threshold amount"),
            DomainKeyword::new("TAXABLE_BASE", "The taxable base amount"),
            DomainKeyword::new("DEDUCTION", "A deduction amount"),
            DomainKeyword::new("EXEMPTION", "Amount below which the base is exempt"),
        ]
    }

    fn operators(&self) -> Vec<DomainOperator> {
        vec![
            DomainOperator::new("FROM..TO", "Inclusive bracket bounds"),
            DomainOperator::new("RATE", "Associates a percentage rate with a bracket"),
            DomainOperator::new("%", "Percent modifier (optional, lexer-ignored)"),
        ]
    }

    fn parse_condition(&self, input: &str) -> DslResult<ConditionNode> {
        let tokens = domain_tokens(input)?;
        let mut cur = TokenCursor::new(&tokens);
        let keyword = cur
            .peek_word()
            .ok_or_else(|| DslError::parse_error("Expected a tax keyword"))?;
        cur.advance();

        let node = match keyword.as_str() {
            "BRACKET" => parse_bracket(&mut cur)?,
            "RATE" => {
                let op = cur.expect_comparison_op();
                let pct = cur.expect_number()?;
                ConditionNode::Comparison {
                    field: Self::RATE_FIELD.to_string(),
                    operator: op,
                    value: percent_value(pct),
                }
            }
            "THRESHOLD" | "TAXABLE_BASE" | "DEDUCTION" => {
                let field = cur.expect_field()?;
                let op = cur.expect_comparison_op();
                let amount = cur.expect_number()?;
                ConditionNode::Comparison {
                    field: normalize_field(&keyword, &field),
                    operator: op,
                    value: ConditionValue::Number(amount as i64),
                }
            }
            "EXEMPTION" | "EXEMPT" => {
                let field = cur.expect_field()?;
                // `EXEMPTION <base> BELOW <amount>` or `EXEMPTION <base> <amount>`.
                cur.eat_keyword("BELOW");
                let amount = cur.expect_number()?;
                ConditionNode::Comparison {
                    field,
                    operator: "<".to_string(),
                    value: ConditionValue::Number(amount as i64),
                }
            }
            other => {
                return Err(DslError::parse_error(format!(
                    "Unknown tax keyword: '{other}'"
                )));
            }
        };
        cur.expect_eof()?;
        Ok(node)
    }

    fn validate_statute(&self, statute: &StatuteNode) -> Vec<DomainDiagnostic> {
        let mut diags = Vec::new();
        let atoms = statute_atoms(statute);
        let mut brackets: Vec<(String, f64, f64)> = Vec::new();

        for atom in &atoms {
            match atom {
                ConditionNode::Comparison {
                    field,
                    operator: _,
                    value,
                } if field.eq_ignore_ascii_case(Self::RATE_FIELD)
                    || field.to_lowercase().contains("rate") =>
                {
                    if let Some(rate) = value_as_f64(value)
                        && !(0.0..=100.0).contains(&rate)
                    {
                        diags.push(DomainDiagnostic::new(
                            self.name(),
                            DomainSeverity::Error,
                            "tax.rate-out-of-range",
                            format!("tax rate {rate}% must be between 0% and 100%"),
                        ));
                    }
                }
                ConditionNode::Between { field, min, max } => {
                    let (lo, hi) = (value_as_f64(min), value_as_f64(max));
                    if let (Some(lo), Some(hi)) = (lo, hi) {
                        if lo > hi {
                            diags.push(DomainDiagnostic::new(
                                self.name(),
                                DomainSeverity::Error,
                                "tax.bracket-inverted",
                                format!(
                                    "tax bracket on '{field}' is inverted (lower {lo} > upper {hi})"
                                ),
                            ));
                        }
                        brackets.push((field.to_lowercase(), lo, hi));
                    }
                }
                _ => {}
            }
        }

        // Flag overlapping brackets on the same base.
        for i in 0..brackets.len() {
            for j in (i + 1)..brackets.len() {
                let (fa, la, ha) = &brackets[i];
                let (fb, lb, hb) = &brackets[j];
                if fa == fb && la.max(*lb) < ha.min(*hb) {
                    diags.push(DomainDiagnostic::new(
                        self.name(),
                        DomainSeverity::Warning,
                        "tax.brackets-overlap",
                        format!("tax brackets on '{fa}' overlap: [{la}, {ha}] and [{lb}, {hb}]"),
                    ));
                }
            }
        }

        diags
    }
}

/// Parses `FROM <lo> TO <hi> [RATE <pct>]` following the `BRACKET <base>` head.
fn parse_bracket(cur: &mut TokenCursor) -> DslResult<ConditionNode> {
    let base = cur.expect_field()?;
    cur.expect_keyword("FROM")?;
    let lo = cur.expect_number()?;
    cur.expect_keyword("TO")?;
    let hi = cur.expect_number()?;

    let between = ConditionNode::Between {
        field: base,
        min: ConditionValue::Number(lo as i64),
        max: ConditionValue::Number(hi as i64),
    };

    if cur.eat_keyword("RATE") {
        let pct = cur.expect_number()?;
        let rate = ConditionNode::Comparison {
            field: TaxLawDomain::RATE_FIELD.to_string(),
            operator: "==".to_string(),
            value: percent_value(pct),
        };
        Ok(ConditionNode::And(Box::new(between), Box::new(rate)))
    } else {
        Ok(between)
    }
}

/// Maps a keyword + raw field name to the canonical lowered field name.
fn normalize_field(keyword: &str, field: &str) -> String {
    match keyword {
        "DEDUCTION" => format!("deduction_{}", field.to_lowercase()),
        _ => field.to_string(),
    }
}
