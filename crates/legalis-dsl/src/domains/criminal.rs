//! Criminal-law specialized syntax (roadmap v0.3.2).
//!
//! Recognized condition forms:
//!
//! ```text
//! MENS_REA <level>                 ; intent|knowledge|recklessness|negligence|strict
//! ACTUS_REUS <desc>                ; the prohibited act
//! ELEMENT <name>                   ; a named offence element
//! PENALTY_RANGE <min> TO <max> [<unit>]   ; sentencing band (months by default)
//! OFFENSE <desc>                   ; the offence label
//! ```
//!
//! Validation enforces the classic completeness principle *actus non facit reum
//! nisi mens sit rea* — a tagged offence statute should specify both an actus
//! reus and a (non-strict) mens rea — alongside recognized mens-rea levels and
//! coherent penalty ranges.

use super::{
    DomainDiagnostic, DomainKeyword, DomainOperator, DomainSeverity, LegalDomain, TokenCursor,
    domain_tokens, statute_atoms, value_as_f64,
};
use crate::ast::{ConditionNode, ConditionValue, StatuteNode};
use crate::{DslError, DslResult};

/// The criminal-law domain.
pub struct CriminalLawDomain;

impl CriminalLawDomain {
    /// Field name used for the mens rea (mental element).
    pub const MENS_REA_FIELD: &'static str = "mens_rea";
    /// Field name used for the actus reus (physical element).
    pub const ACTUS_REUS_FIELD: &'static str = "actus_reus";

    /// Recognized mens-rea levels (canonical, lowercase).
    pub const MENS_REA_LEVELS: &'static [&'static str] = &[
        "intent",
        "intentional",
        "knowledge",
        "knowing",
        "recklessness",
        "reckless",
        "negligence",
        "negligent",
        "strict",
    ];
}

impl LegalDomain for CriminalLawDomain {
    fn name(&self) -> &str {
        "criminal"
    }

    fn description(&self) -> &str {
        "Criminal law: offence elements, mens rea / actus reus and penalty ranges"
    }

    fn keywords(&self) -> Vec<DomainKeyword> {
        vec![
            DomainKeyword::new("MENS_REA", "The required mental element"),
            DomainKeyword::new("ACTUS_REUS", "The prohibited physical act"),
            DomainKeyword::new("ELEMENT", "A named offence element"),
            DomainKeyword::new("PENALTY_RANGE", "Sentencing band (min TO max)"),
            DomainKeyword::new("OFFENSE", "The offence label"),
        ]
    }

    fn operators(&self) -> Vec<DomainOperator> {
        vec![DomainOperator::new(
            "TO",
            "Separates the lower and upper penalty bounds",
        )]
    }

    fn parse_condition(&self, input: &str) -> DslResult<ConditionNode> {
        let tokens = domain_tokens(input)?;
        let mut cur = TokenCursor::new(&tokens);
        let keyword = cur
            .peek_word()
            .ok_or_else(|| DslError::parse_error("Expected a criminal-law keyword"))?;
        cur.advance();

        let node = match keyword.as_str() {
            "MENS_REA" => {
                let level = cur.expect_string()?.to_lowercase();
                ConditionNode::Comparison {
                    field: Self::MENS_REA_FIELD.to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::String(level),
                }
            }
            "ACTUS_REUS" => {
                let desc = cur.expect_string()?;
                ConditionNode::Comparison {
                    field: Self::ACTUS_REUS_FIELD.to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::String(desc),
                }
            }
            "ELEMENT" => {
                let name = cur.expect_field()?;
                ConditionNode::HasAttribute { key: name }
            }
            "PENALTY_RANGE" => parse_penalty_range(&mut cur)?,
            "OFFENSE" | "OFFENCE" => {
                let desc = cur.expect_string()?;
                ConditionNode::Comparison {
                    field: "offense".to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::String(desc),
                }
            }
            other => {
                return Err(DslError::parse_error(format!(
                    "Unknown criminal-law keyword: '{other}'"
                )));
            }
        };
        cur.expect_eof()?;
        Ok(node)
    }

    fn validate_statute(&self, statute: &StatuteNode) -> Vec<DomainDiagnostic> {
        let mut diags = Vec::new();
        let atoms = statute_atoms(statute);

        let mut has_actus_reus = false;
        let mut mens_rea_level: Option<String> = None;

        for atom in &atoms {
            match atom {
                ConditionNode::Comparison { field, .. }
                    if field.eq_ignore_ascii_case(Self::ACTUS_REUS_FIELD) =>
                {
                    has_actus_reus = true;
                }
                ConditionNode::Comparison {
                    field,
                    value: ConditionValue::String(level),
                    ..
                } if field.eq_ignore_ascii_case(Self::MENS_REA_FIELD) => {
                    let level = level.to_lowercase();
                    if !Self::MENS_REA_LEVELS.contains(&level.as_str()) {
                        diags.push(DomainDiagnostic::new(
                            self.name(),
                            DomainSeverity::Error,
                            "criminal.unknown-mens-rea",
                            format!(
                                "unrecognized mens rea level '{level}' (expected one of: {})",
                                Self::MENS_REA_LEVELS.join(", ")
                            ),
                        ));
                    }
                    mens_rea_level = Some(level);
                }
                ConditionNode::Between { field, min, max }
                    if field.to_lowercase().starts_with("penalty") =>
                {
                    if let (Some(lo), Some(hi)) = (value_as_f64(min), value_as_f64(max)) {
                        if lo > hi {
                            diags.push(DomainDiagnostic::new(
                                self.name(),
                                DomainSeverity::Error,
                                "criminal.penalty-range-inverted",
                                format!("penalty range is inverted (min {lo} > max {hi})"),
                            ));
                        }
                        if lo < 0.0 {
                            diags.push(DomainDiagnostic::new(
                                self.name(),
                                DomainSeverity::Error,
                                "criminal.penalty-negative",
                                format!("penalty range lower bound {lo} cannot be negative"),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        // Completeness: every offence needs both an act and a (non-strict)
        // mental element.
        if !has_actus_reus {
            diags.push(DomainDiagnostic::new(
                self.name(),
                DomainSeverity::Warning,
                "criminal.missing-actus-reus",
                "offence statute does not specify an actus reus (prohibited act)",
            ));
        }
        match &mens_rea_level {
            None => diags.push(DomainDiagnostic::new(
                self.name(),
                DomainSeverity::Warning,
                "criminal.missing-mens-rea",
                "offence statute does not specify a mens rea (mental element)",
            )),
            Some(level) if level == "strict" => diags.push(DomainDiagnostic::new(
                self.name(),
                DomainSeverity::Info,
                "criminal.strict-liability",
                "offence is strict liability (no mens rea required)",
            )),
            _ => {}
        }

        diags
    }
}

/// Parses `<min> TO <max> [<unit>]` after the `PENALTY_RANGE` head.
fn parse_penalty_range(cur: &mut TokenCursor) -> DslResult<ConditionNode> {
    let min = cur.expect_number()?;
    cur.expect_keyword("TO")?;
    let max = cur.expect_number()?;
    // Optional unit (months/years/days/fine). Defaults to months.
    let unit = if cur.is_eof() {
        "months".to_string()
    } else {
        cur.expect_string()?.to_lowercase()
    };
    Ok(ConditionNode::Between {
        field: format!("penalty_{unit}"),
        min: ConditionValue::Number(min as i64),
        max: ConditionValue::Number(max as i64),
    })
}
