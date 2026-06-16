//! Custom literal syntax (roadmap v0.3.4).
//!
//! A [`CustomLiteral`] recognizes and parses a bespoke literal form (money,
//! percentages, durations, …) from a raw lexeme, producing a typed
//! [`LiteralValue`] that lowers to a core [`ConditionValue`]. Implementations are
//! collected in a [`LiteralRegistry`] which tries them in registration order.
//!
//! This layer is opt-in: the core tokenizer never sees these forms (it drops
//! `$`/`%` etc.), so callers feed raw lexemes to the registry explicitly.

use crate::ast::ConditionValue;
use crate::{DslError, DslResult};
use regex::Regex;

/// A typed value produced by a [`CustomLiteral`].
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    /// A monetary amount in minor units (e.g. cents) plus an ISO-ish currency.
    Money {
        /// Amount in minor units (cents).
        minor_units: i64,
        /// Currency code (e.g. `USD`).
        currency: String,
    },
    /// A percentage value.
    Percent(f64),
    /// A duration in whole seconds.
    Duration {
        /// Length in seconds.
        seconds: i64,
    },
    /// A plain integer.
    Integer(i64),
    /// Free text.
    Text(String),
}

impl LiteralValue {
    /// Lowers the typed value to a core [`ConditionValue`] for embedding in
    /// conditions. Money becomes its minor-unit count, durations their seconds,
    /// percentages an integer when whole (else a decimal string).
    pub fn to_condition_value(&self) -> ConditionValue {
        match self {
            LiteralValue::Money { minor_units, .. } => ConditionValue::Number(*minor_units),
            LiteralValue::Percent(p) => {
                if p.fract() == 0.0 && p.abs() < i64::MAX as f64 {
                    ConditionValue::Number(*p as i64)
                } else {
                    ConditionValue::String(format!("{p}"))
                }
            }
            LiteralValue::Duration { seconds } => ConditionValue::Number(*seconds),
            LiteralValue::Integer(n) => ConditionValue::Number(*n),
            LiteralValue::Text(s) => ConditionValue::String(s.clone()),
        }
    }
}

/// A pluggable custom literal form.
pub trait CustomLiteral: Send + Sync {
    /// The literal's name (e.g. `money`).
    fn name(&self) -> &str;

    /// Returns true if `lexeme` looks like this literal form.
    fn matches(&self, lexeme: &str) -> bool;

    /// Parses and validates `lexeme`, returning the typed value or an error
    /// message.
    fn parse(&self, lexeme: &str) -> Result<LiteralValue, String>;
}

/// A registry of custom literal forms.
#[derive(Default)]
pub struct LiteralRegistry {
    literals: Vec<Box<dyn CustomLiteral>>,
}

impl LiteralRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a registry with the built-in money, percent and duration forms.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(MoneyLiteral::new()));
        registry.register(Box::new(PercentLiteral::new()));
        registry.register(Box::new(DurationLiteral::new()));
        registry
    }

    /// Registers a literal form.
    pub fn register(&mut self, literal: Box<dyn CustomLiteral>) {
        self.literals.push(literal);
    }

    /// Returns the names of the registered literal forms.
    pub fn names(&self) -> Vec<String> {
        self.literals.iter().map(|l| l.name().to_string()).collect()
    }

    /// Tries each registered form in order, returning the name and parsed value
    /// of the first that both matches and parses successfully.
    pub fn try_parse(&self, lexeme: &str) -> Option<(String, LiteralValue)> {
        let trimmed = lexeme.trim();
        for literal in &self.literals {
            if literal.matches(trimmed)
                && let Ok(value) = literal.parse(trimmed)
            {
                return Some((literal.name().to_string(), value));
            }
        }
        None
    }

    /// Like [`try_parse`](Self::try_parse) but returns an error when no form
    /// applies.
    pub fn parse(&self, lexeme: &str) -> DslResult<LiteralValue> {
        self.try_parse(lexeme)
            .map(|(_, v)| v)
            .ok_or_else(|| DslError::parse_error(format!("No custom literal matches '{lexeme}'")))
    }
}

/// Compiles a regex, returning `None` rather than panicking on a bad pattern
/// (keeps the no-unwrap policy; the built-in patterns are always valid).
fn compile(pattern: &str) -> Option<Regex> {
    Regex::new(pattern).ok()
}

/// A money literal: optional `$`, grouped digits, up to two decimal places and an
/// optional three-letter currency code (e.g. `$1,234.56`, `100 USD`). A bare
/// integer is *not* money (it needs a `$` or a currency).
pub struct MoneyLiteral {
    re: Option<Regex>,
}

impl Default for MoneyLiteral {
    fn default() -> Self {
        Self::new()
    }
}

impl MoneyLiteral {
    /// Creates the money literal recognizer.
    pub fn new() -> Self {
        Self {
            re: compile(
                r"(?i)^(?P<sym>\$)?\s*(?P<int>\d{1,3}(?:,\d{3})*|\d+)(?:\.(?P<frac>\d{1,2}))?\s*(?P<ccy>[a-z]{3})?$",
            ),
        }
    }
}

impl CustomLiteral for MoneyLiteral {
    fn name(&self) -> &str {
        "money"
    }

    fn matches(&self, lexeme: &str) -> bool {
        let Some(re) = &self.re else {
            return false;
        };
        match re.captures(lexeme) {
            Some(caps) => caps.name("sym").is_some() || caps.name("ccy").is_some(),
            None => false,
        }
    }

    fn parse(&self, lexeme: &str) -> Result<LiteralValue, String> {
        let re = self
            .re
            .as_ref()
            .ok_or_else(|| "money literal recognizer unavailable".to_string())?;
        let caps = re
            .captures(lexeme)
            .ok_or_else(|| format!("'{lexeme}' is not a money literal"))?;
        if caps.name("sym").is_none() && caps.name("ccy").is_none() {
            return Err(format!("'{lexeme}' lacks a currency symbol or code"));
        }
        let int_part = caps
            .name("int")
            .map(|m| m.as_str().replace(',', ""))
            .unwrap_or_default();
        let units: i64 = int_part
            .parse()
            .map_err(|_| format!("invalid amount in '{lexeme}'"))?;
        let cents: i64 = match caps.name("frac") {
            Some(m) => {
                let padded = format!("{:0<2}", m.as_str());
                padded
                    .parse()
                    .map_err(|_| format!("invalid fractional part in '{lexeme}'"))?
            }
            None => 0,
        };
        let currency = caps
            .name("ccy")
            .map(|m| m.as_str().to_uppercase())
            .unwrap_or_else(|| "USD".to_string());
        Ok(LiteralValue::Money {
            minor_units: units * 100 + cents,
            currency,
        })
    }
}

/// A percentage literal: a number followed by `%` (e.g. `12.5%`).
pub struct PercentLiteral {
    re: Option<Regex>,
}

impl Default for PercentLiteral {
    fn default() -> Self {
        Self::new()
    }
}

impl PercentLiteral {
    /// Creates the percent literal recognizer.
    pub fn new() -> Self {
        Self {
            re: compile(r"^(?P<num>\d+(?:\.\d+)?)\s*%$"),
        }
    }
}

impl CustomLiteral for PercentLiteral {
    fn name(&self) -> &str {
        "percent"
    }

    fn matches(&self, lexeme: &str) -> bool {
        self.re.as_ref().is_some_and(|re| re.is_match(lexeme))
    }

    fn parse(&self, lexeme: &str) -> Result<LiteralValue, String> {
        let re = self
            .re
            .as_ref()
            .ok_or_else(|| "percent literal recognizer unavailable".to_string())?;
        let caps = re
            .captures(lexeme)
            .ok_or_else(|| format!("'{lexeme}' is not a percent literal"))?;
        let num: f64 = caps
            .name("num")
            .and_then(|m| m.as_str().parse().ok())
            .ok_or_else(|| format!("invalid percent in '{lexeme}'"))?;
        Ok(LiteralValue::Percent(num))
    }
}

/// A duration literal: an integer with a time unit suffix (`30d`, `6mo`, `2y`).
pub struct DurationLiteral {
    re: Option<Regex>,
}

impl Default for DurationLiteral {
    fn default() -> Self {
        Self::new()
    }
}

impl DurationLiteral {
    /// Creates the duration literal recognizer.
    pub fn new() -> Self {
        Self {
            re: compile(
                r"(?i)^(?P<num>\d+)\s*(?P<unit>s|sec|secs|seconds|min|mins|minutes|h|hr|hrs|hours|d|day|days|w|wk|weeks|mo|month|months|y|yr|yrs|years)$",
            ),
        }
    }
}

impl CustomLiteral for DurationLiteral {
    fn name(&self) -> &str {
        "duration"
    }

    fn matches(&self, lexeme: &str) -> bool {
        self.re.as_ref().is_some_and(|re| re.is_match(lexeme))
    }

    fn parse(&self, lexeme: &str) -> Result<LiteralValue, String> {
        let re = self
            .re
            .as_ref()
            .ok_or_else(|| "duration literal recognizer unavailable".to_string())?;
        let caps = re
            .captures(lexeme)
            .ok_or_else(|| format!("'{lexeme}' is not a duration literal"))?;
        let num: i64 = caps
            .name("num")
            .and_then(|m| m.as_str().parse().ok())
            .ok_or_else(|| format!("invalid duration in '{lexeme}'"))?;
        let unit = caps
            .name("unit")
            .map(|m| m.as_str().to_lowercase())
            .unwrap_or_default();
        let factor: i64 = match unit.as_str() {
            "s" | "sec" | "secs" | "seconds" => 1,
            "min" | "mins" | "minutes" => 60,
            "h" | "hr" | "hrs" | "hours" => 3_600,
            "d" | "day" | "days" => 86_400,
            "w" | "wk" | "weeks" => 604_800,
            "mo" | "month" | "months" => 2_592_000,
            "y" | "yr" | "yrs" | "years" => 31_536_000,
            _ => return Err(format!("unknown duration unit '{unit}'")),
        };
        Ok(LiteralValue::Duration {
            seconds: num * factor,
        })
    }
}
