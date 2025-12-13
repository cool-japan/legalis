//! Pretty-printer for Legal DSL AST.
//!
//! Converts `Statute` structures back to human-readable DSL format.

use legalis_core::{ComparisonOp, Condition, EffectType, Statute};

/// Configuration for the pretty-printer.
#[derive(Debug, Clone)]
pub struct PrinterConfig {
    /// Indentation string (default: 4 spaces)
    pub indent: String,
    /// Include comments with metadata
    pub include_comments: bool,
    /// Line width for wrapping (0 = no wrapping)
    pub line_width: usize,
    /// Uppercase keywords
    pub uppercase_keywords: bool,
}

impl Default for PrinterConfig {
    fn default() -> Self {
        Self {
            indent: "    ".to_string(),
            include_comments: false,
            line_width: 80,
            uppercase_keywords: true,
        }
    }
}

impl PrinterConfig {
    /// Creates a compact configuration with minimal formatting.
    pub fn compact() -> Self {
        Self {
            indent: "  ".to_string(),
            include_comments: false,
            line_width: 0,
            uppercase_keywords: true,
        }
    }

    /// Creates a verbose configuration with comments and wide lines.
    pub fn verbose() -> Self {
        Self {
            indent: "    ".to_string(),
            include_comments: true,
            line_width: 120,
            uppercase_keywords: true,
        }
    }
}

/// Pretty-printer for Legal DSL.
#[derive(Debug, Default)]
pub struct DslPrinter {
    config: PrinterConfig,
}

impl DslPrinter {
    /// Creates a new printer with default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a printer with custom configuration.
    pub fn with_config(config: PrinterConfig) -> Self {
        Self { config }
    }

    /// Formats a statute as DSL text.
    pub fn format(&self, statute: &Statute) -> String {
        let mut output = String::new();

        // Optional header comment
        if self.config.include_comments {
            output.push_str(&format!("// Statute: {}\n", statute.title));
            if let Some(ref jur) = statute.jurisdiction {
                output.push_str(&format!("// Jurisdiction: {}\n", jur));
            }
            output.push('\n');
        }

        // STATUTE declaration
        output.push_str(&self.kw("STATUTE"));
        output.push(' ');
        output.push_str(&statute.id);
        output.push_str(": ");
        output.push_str(&self.quote(&statute.title));
        output.push_str(" {\n");

        // Metadata
        if let Some(ref jur) = statute.jurisdiction {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("JURISDICTION"));
            output.push(' ');
            output.push_str(&self.quote(jur));
            output.push('\n');
        }

        if statute.version > 1 {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("VERSION"));
            output.push(' ');
            output.push_str(&statute.version.to_string());
            output.push('\n');
        }

        if let Some(eff) = statute.temporal_validity.effective_date {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("EFFECTIVE_DATE"));
            output.push(' ');
            output.push_str(&eff.format("%Y-%m-%d").to_string());
            output.push('\n');
        }

        if let Some(exp) = statute.temporal_validity.expiry_date {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("EXPIRY_DATE"));
            output.push(' ');
            output.push_str(&exp.format("%Y-%m-%d").to_string());
            output.push('\n');
        }

        // Conditions
        if !statute.preconditions.is_empty() {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("WHEN"));
            output.push(' ');

            let conditions: Vec<String> = statute
                .preconditions
                .iter()
                .map(|c| self.format_condition(c))
                .collect();

            if conditions.len() == 1 {
                output.push_str(&conditions[0]);
            } else {
                output.push_str(&conditions.join(&format!(" {} ", self.kw("AND"))));
            }
            output.push('\n');
        }

        // Effect
        output.push_str(&self.config.indent);
        output.push_str(&self.kw("THEN"));
        output.push(' ');
        output.push_str(&self.format_effect_type(&statute.effect.effect_type));
        output.push(' ');
        output.push_str(&self.quote(&statute.effect.description));
        output.push('\n');

        // Discretion
        if let Some(ref discretion) = statute.discretion_logic {
            output.push_str(&self.config.indent);
            output.push_str(&self.kw("DISCRETION"));
            output.push(' ');
            output.push_str(&self.quote(discretion));
            output.push('\n');
        }

        output.push('}');
        output.push('\n');

        output
    }

    /// Formats multiple statutes as DSL text.
    pub fn format_batch(&self, statutes: &[Statute]) -> String {
        statutes
            .iter()
            .map(|s| self.format(s))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Formats a condition expression.
    fn format_condition(&self, condition: &Condition) -> String {
        match condition {
            Condition::Age { operator, value } => {
                format!("{} {} {}", self.kw("AGE"), self.format_op(*operator), value)
            }
            Condition::Income { operator, value } => {
                format!(
                    "{} {} {}",
                    self.kw("INCOME"),
                    self.format_op(*operator),
                    value
                )
            }
            Condition::HasAttribute { key } => {
                if key.contains('-') || key.contains(' ') {
                    format!("{} {}", self.kw("HAS"), self.quote(key))
                } else {
                    format!("{} {}", self.kw("HAS"), key)
                }
            }
            Condition::AttributeEquals { key, value } => {
                format!("{} = {}", self.quote(key), self.quote(value))
            }
            Condition::And(left, right) => {
                let left_str = self.format_condition(left);
                let right_str = self.format_condition(right);
                format!("{} {} {}", left_str, self.kw("AND"), right_str)
            }
            Condition::Or(left, right) => {
                let left_str = self.format_condition_with_parens(left, true);
                let right_str = self.format_condition_with_parens(right, true);
                format!("{} {} {}", left_str, self.kw("OR"), right_str)
            }
            Condition::Not(inner) => {
                let inner_str = self.format_condition_with_parens(inner, false);
                format!("{} {}", self.kw("NOT"), inner_str)
            }
            Condition::ResidencyDuration { operator, months } => {
                format!(
                    "{} {} {} months",
                    self.kw("RESIDENCY"),
                    self.format_op(*operator),
                    months
                )
            }
            Condition::Geographic {
                region_type,
                region_id,
            } => {
                format!(
                    "{} {:?} {}",
                    self.kw("REGION"),
                    region_type,
                    self.quote(region_id)
                )
            }
            Condition::DateRange { start, end } => {
                let start_str = start
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "*".to_string());
                let end_str = end
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "*".to_string());
                format!("{} {} TO {}", self.kw("DATE"), start_str, end_str)
            }
            Condition::EntityRelationship {
                relationship_type,
                target_entity_id,
            } => {
                let target = target_entity_id
                    .as_ref()
                    .map(|t| self.quote(t))
                    .unwrap_or_else(|| "*".to_string());
                format!(
                    "{} {:?} {}",
                    self.kw("RELATIONSHIP"),
                    relationship_type,
                    target
                )
            }
            Condition::Custom { description } => {
                format!("{} {}", self.kw("CUSTOM"), self.quote(description))
            }
        }
    }

    /// Formats a condition, adding parentheses if needed for clarity.
    fn format_condition_with_parens(&self, condition: &Condition, is_or_context: bool) -> String {
        let needs_parens = match condition {
            Condition::And(_, _) if is_or_context => true,
            Condition::Or(_, _) => true,
            _ => false,
        };

        let inner = self.format_condition(condition);
        if needs_parens {
            format!("({})", inner)
        } else {
            inner
        }
    }

    /// Formats a comparison operator.
    fn format_op(&self, op: ComparisonOp) -> &'static str {
        match op {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterOrEqual => ">=",
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessOrEqual => "<=",
        }
    }

    /// Formats an effect type.
    fn format_effect_type(&self, effect_type: &EffectType) -> String {
        let name = match effect_type {
            EffectType::Grant => "GRANT",
            EffectType::Revoke => "REVOKE",
            EffectType::Obligation => "OBLIGATION",
            EffectType::Prohibition => "PROHIBITION",
            EffectType::MonetaryTransfer => "MONETARY_TRANSFER",
            EffectType::StatusChange => "STATUS_CHANGE",
            EffectType::Custom => "CUSTOM",
        };
        self.kw(name)
    }

    /// Applies keyword casing based on configuration.
    fn kw(&self, keyword: &str) -> String {
        if self.config.uppercase_keywords {
            keyword.to_uppercase()
        } else {
            keyword.to_lowercase()
        }
    }

    /// Quotes a string value.
    fn quote(&self, s: &str) -> String {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    }
}

/// Formats a statute to DSL string using default configuration.
pub fn format_statute(statute: &Statute) -> String {
    DslPrinter::new().format(statute)
}

/// Formats multiple statutes to DSL string using default configuration.
pub fn format_statutes(statutes: &[Statute]) -> String {
    DslPrinter::new().format_batch(statutes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::Effect;

    fn sample_statute() -> Statute {
        Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_format_simple_statute() {
        let statute = sample_statute();
        let output = format_statute(&statute);

        assert!(output.contains("STATUTE adult-rights:"));
        assert!(output.contains("\"Adult Rights Act\""));
        assert!(output.contains("WHEN AGE >= 18"));
        assert!(output.contains("THEN GRANT \"Full legal capacity\""));
    }

    #[test]
    fn test_format_with_discretion() {
        let statute = sample_statute().with_discretion("Consider individual circumstances");
        let output = format_statute(&statute);

        assert!(output.contains("DISCRETION \"Consider individual circumstances\""));
    }

    #[test]
    fn test_format_and_condition() {
        let statute = Statute::new(
            "complex",
            "Complex Statute",
            Effect::new(EffectType::Grant, "Rights"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));

        let output = format_statute(&statute);
        assert!(output.contains("AGE >= 18 AND INCOME < 50000"));
    }

    #[test]
    fn test_format_or_condition() {
        let statute = Statute::new(
            "either",
            "Either Or",
            Effect::new(EffectType::Grant, "Benefits"),
        )
        .with_precondition(Condition::Or(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 65,
            }),
            Box::new(Condition::HasAttribute {
                key: "disabled".to_string(),
            }),
        ));

        let output = format_statute(&statute);
        assert!(output.contains("AGE >= 65 OR HAS disabled"));
    }

    #[test]
    fn test_format_not_condition() {
        let statute = Statute::new(
            "exclude",
            "Exclusion",
            Effect::new(EffectType::Grant, "Rights"),
        )
        .with_precondition(Condition::Not(Box::new(Condition::HasAttribute {
            key: "convicted".to_string(),
        })));

        let output = format_statute(&statute);
        assert!(output.contains("NOT HAS convicted"));
    }

    #[test]
    fn test_format_with_jurisdiction() {
        let mut statute = sample_statute();
        statute.jurisdiction = Some("JP".to_string());

        let output = format_statute(&statute);
        assert!(output.contains("JURISDICTION \"JP\""));
    }

    #[test]
    fn test_format_with_version() {
        let mut statute = sample_statute();
        statute.version = 3;

        let output = format_statute(&statute);
        assert!(output.contains("VERSION 3"));
    }

    #[test]
    fn test_format_compact() {
        let printer = DslPrinter::with_config(PrinterConfig::compact());
        let statute = sample_statute();
        let output = printer.format(&statute);

        // Compact uses 2-space indent
        assert!(output.contains("  WHEN"));
    }

    #[test]
    fn test_format_verbose() {
        let printer = DslPrinter::with_config(PrinterConfig::verbose());
        let statute = sample_statute();
        let output = printer.format(&statute);

        // Verbose includes comments
        assert!(output.contains("// Statute:"));
    }

    #[test]
    fn test_format_batch() {
        let statutes = vec![
            sample_statute(),
            Statute::new(
                "another",
                "Another Statute",
                Effect::new(EffectType::Revoke, "Something"),
            ),
        ];

        let output = format_statutes(&statutes);
        assert!(output.contains("adult-rights"));
        assert!(output.contains("another"));
    }

    #[test]
    fn test_roundtrip_simple() {
        let statute = sample_statute();
        let dsl = format_statute(&statute);

        // Parse it back
        let parser = crate::LegalDslParser::new();
        let parsed = parser.parse_statute(&dsl).unwrap();

        assert_eq!(parsed.id, statute.id);
        assert_eq!(parsed.title, statute.title);
    }

    #[test]
    fn test_format_has_with_hyphen() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"))
            .with_precondition(Condition::HasAttribute {
                key: "active-member".to_string(),
            });

        let output = format_statute(&statute);
        assert!(output.contains("HAS \"active-member\""));
    }
}
