//! Legacy [`legalis_core::Statute`]-producing parse path.
//!
//! `parse_tokens` builds a flat [`Statute`] (as opposed to the richer
//! [`crate::ast::StatuteNode`] produced by [`super::document`]) and owns the
//! `legalis_core::Condition` recursive-descent grammar plus the scalar/value
//! helpers (`parse_comparison_op`, `parse_number`, `parse_date`, region /
//! relationship / duration parsing, …). `parse_comparison_op` is `pub(crate)`
//! because the condition grammar in [`super::conditions`] also calls it. Split
//! out of the original `parser_impl.rs` to keep every file under 2000 lines.

use chrono::NaiveDate;
use legalis_core::{
    Condition, DurationUnit, Effect, EffectType, RegionType, RelationshipType, Statute,
    TemporalValidity,
};

use super::LegalDslParser;
use crate::ast::Token;
use crate::{DslError, DslResult};

impl LegalDslParser {
    pub(crate) fn parse_tokens(&self, tokens: &[Token]) -> DslResult<Statute> {
        let mut iter = tokens.iter().peekable();

        // Expect STATUTE
        match iter.next() {
            Some(Token::Statute) => {}
            _ => {
                return Err(DslError::parse_error("Expected 'STATUTE' keyword"));
            }
        }

        // Get ID
        let id = match iter.next() {
            Some(Token::Ident(s)) => s.clone(),
            _ => {
                return Err(DslError::parse_error("Expected statute identifier"));
            }
        };

        // Expect colon
        match iter.next() {
            Some(Token::Colon) => {}
            _ => {
                return Err(DslError::parse_error("Expected ':'"));
            }
        }

        // Get title
        let title = match iter.next() {
            Some(Token::StringLit(s)) => s.clone(),
            Some(Token::Ident(s)) => s.clone(),
            _ => {
                return Err(DslError::parse_error("Expected statute title"));
            }
        };

        // Expect LBrace
        match iter.next() {
            Some(Token::LBrace) => {}
            _ => {
                return Err(DslError::parse_error("Expected '{'"));
            }
        }

        let mut conditions = Vec::new();
        let mut effect = None;
        let mut discretion = None;
        let mut effective_date = None;
        let mut expiry_date = None;
        let mut jurisdiction = None;
        let mut version = None;

        // Parse body
        while let Some(token) = iter.next() {
            match token {
                Token::When => {
                    if let Some(cond) = self.parse_condition(&mut iter)? {
                        conditions.push(cond);
                    }
                }
                Token::Then => {
                    effect = Some(self.parse_effect(&mut iter)?);
                }
                Token::Discretion => {
                    if let Some(Token::StringLit(s)) = iter.next() {
                        discretion = Some(s.clone());
                    }
                }
                Token::EffectiveDate => {
                    effective_date = self.parse_date(&mut iter);
                }
                Token::ExpiryDate => {
                    expiry_date = self.parse_date(&mut iter);
                }
                Token::Jurisdiction => {
                    if let Some(Token::StringLit(s)) = iter.next() {
                        jurisdiction = Some(s.clone());
                    } else if let Some(Token::Ident(s)) = iter.peek() {
                        jurisdiction = Some(s.clone());
                        iter.next();
                    }
                }
                Token::Version => {
                    if let Some(Token::Number(n)) = iter.next() {
                        version = Some(*n as u32);
                    }
                }
                Token::RBrace => break,
                _ => {}
            }
        }

        let effect =
            effect.unwrap_or_else(|| Effect::new(EffectType::Custom, "No effect specified"));

        let mut statute = Statute::new(id, title, effect);
        statute.preconditions = conditions;
        statute.discretion_logic = discretion;

        // Set temporal validity if any dates were specified
        if effective_date.is_some() || expiry_date.is_some() {
            statute.temporal_validity = TemporalValidity {
                effective_date,
                expiry_date,
                enacted_at: None,
                amended_at: None,
            };
        }

        // Set jurisdiction and version if specified
        if let Some(jur) = jurisdiction {
            statute.jurisdiction = Some(jur);
        }
        if let Some(ver) = version {
            statute.version = ver;
        }

        Ok(statute)
    }

    /// Parses a condition expression (handles OR at lowest precedence).
    fn parse_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        self.parse_or_condition(iter)
    }

    /// Parses OR expressions (lowest precedence).
    fn parse_or_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let Some(mut result) = self.parse_and_condition(iter)? else {
            return Ok(None);
        };

        while matches!(iter.peek(), Some(Token::Or)) {
            iter.next(); // consume OR
            let right = self.parse_and_condition(iter)?;
            if let Some(right_cond) = right {
                result = Condition::Or(Box::new(result), Box::new(right_cond));
            }
        }

        Ok(Some(result))
    }

    /// Parses AND expressions (higher precedence than OR).
    fn parse_and_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let Some(mut result) = self.parse_unary_condition(iter)? else {
            return Ok(None);
        };

        while matches!(iter.peek(), Some(Token::And)) {
            iter.next(); // consume AND
            let right = self.parse_unary_condition(iter)?;
            if let Some(right_cond) = right {
                result = Condition::And(Box::new(result), Box::new(right_cond));
            }
        }

        Ok(Some(result))
    }

    /// Parses unary expressions (NOT) and primary conditions.
    fn parse_unary_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Not) => {
                iter.next(); // consume NOT
                let inner = self.parse_unary_condition(iter)?;
                Ok(inner.map(|c| Condition::Not(Box::new(c))))
            }
            Some(Token::LParen) => {
                iter.next(); // consume (
                let inner = self.parse_or_condition(iter)?;
                // Expect closing paren
                match iter.peek() {
                    Some(Token::RParen) => {
                        iter.next(); // consume )
                    }
                    _ => return Err(DslError::UnmatchedParen(None)),
                }
                Ok(inner)
            }
            _ => self.parse_primary_condition(iter),
        }
    }

    /// Parses primary (atomic) conditions.
    fn parse_primary_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek().cloned() {
            Some(Token::Age) => {
                iter.next();
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_number(iter)?;
                Ok(Some(Condition::Age {
                    operator: op,
                    value: value as u32,
                }))
            }
            Some(Token::Income) => {
                iter.next();
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_number(iter)?;
                Ok(Some(Condition::Income {
                    operator: op,
                    value,
                }))
            }
            Some(Token::Has) => {
                iter.next();
                // Expect an identifier after HAS
                if let Some(Token::Ident(key)) = iter.peek() {
                    let key = key.clone();
                    iter.next();
                    Ok(Some(Condition::HasAttribute { key }))
                } else if let Some(Token::StringLit(key)) = iter.peek() {
                    let key = key.clone();
                    iter.next();
                    Ok(Some(Condition::HasAttribute { key }))
                } else {
                    Ok(None)
                }
            }
            Some(Token::Ident(key)) => {
                let key = key.clone();
                iter.next();
                // The printer emits `Duration`/`ResidencyDuration` as bare uppercase
                // keyword identifiers (`DURATION op N unit`, `RESIDENCY op N months`).
                // Recognise them in the Ident branch only — quoted attribute keys
                // (e.g. an `AttributeEquals` on "residency") arrive as StringLit and
                // must not be mistaken for the keyword.
                if key.eq_ignore_ascii_case("DURATION")
                    && matches!(iter.peek(), Some(Token::Operator(_)))
                {
                    let operator = self.parse_comparison_op(iter)?;
                    let value = self.parse_number(iter)? as u32;
                    let unit = self.parse_duration_unit(iter)?;
                    return Ok(Some(Condition::Duration {
                        operator,
                        value,
                        unit,
                    }));
                }
                if key.eq_ignore_ascii_case("RESIDENCY")
                    && matches!(iter.peek(), Some(Token::Operator(_)))
                {
                    let operator = self.parse_comparison_op(iter)?;
                    let months = self.parse_number(iter)? as u32;
                    // Consume the trailing unit word ("months") the printer emits.
                    if matches!(iter.peek(), Some(Token::Ident(_))) {
                        iter.next();
                    }
                    return Ok(Some(Condition::ResidencyDuration { operator, months }));
                }
                // `REGION <RegionType> "id"` and `RELATIONSHIP <RelationshipType> target`
                // (the printer prints the fieldless enum variant via `{:?}`, which is a
                // bare identifier — so they round-trip without a printer change).
                if key.eq_ignore_ascii_case("REGION")
                    && matches!(iter.peek(), Some(Token::Ident(_)))
                {
                    let region_type = self.parse_region_type(iter)?;
                    let region_id = self.parse_attribute_value(iter)?;
                    return Ok(Some(Condition::Geographic {
                        region_type,
                        region_id,
                    }));
                }
                if key.eq_ignore_ascii_case("RELATIONSHIP")
                    && matches!(iter.peek(), Some(Token::Ident(_)))
                {
                    let relationship_type = self.parse_relationship_type(iter)?;
                    let target_entity_id = match iter.peek() {
                        Some(Token::Star) => {
                            iter.next();
                            None
                        }
                        Some(Token::StringLit(_)) | Some(Token::Ident(_)) => {
                            Some(self.parse_attribute_value(iter)?)
                        }
                        _ => None,
                    };
                    return Ok(Some(Condition::EntityRelationship {
                        relationship_type,
                        target_entity_id,
                    }));
                }
                // `PERCENTAGE op N% (context)` → Percentage. The `%` glyph is dropped
                // by the tokenizer; we read op, value, then the parenthesised context.
                if key.eq_ignore_ascii_case("PERCENTAGE")
                    && matches!(iter.peek(), Some(Token::Operator(_)))
                {
                    let operator = self.parse_comparison_op(iter)?;
                    let value = self.parse_number(iter)? as u32;
                    // Skip a stray operator (a tokenized '%') if one survived.
                    if matches!(iter.peek(), Some(Token::Operator(_))) {
                        iter.next();
                    }
                    let context = self.parse_percentage_context(iter)?;
                    return Ok(Some(Condition::Percentage {
                        operator,
                        value,
                        context,
                    }));
                }
                // `CALC "formula" op value` → Calculation (formula is always quoted;
                // the value may be an integer or a float literal).
                if key.eq_ignore_ascii_case("CALC")
                    && matches!(
                        iter.peek(),
                        Some(Token::StringLit(_)) | Some(Token::Ident(_))
                    )
                {
                    let formula = self.parse_attribute_value(iter)?;
                    let operator = self.parse_comparison_op(iter)?;
                    let value = self.parse_float_value(iter)?;
                    return Ok(Some(Condition::Calculation {
                        formula,
                        operator,
                        value,
                    }));
                }
                // `CUSTOM "description"` → Custom.
                if key.eq_ignore_ascii_case("CUSTOM")
                    && matches!(
                        iter.peek(),
                        Some(Token::StringLit(_)) | Some(Token::Ident(_))
                    )
                {
                    let description = self.parse_attribute_value(iter)?;
                    return Ok(Some(Condition::Custom { description }));
                }
                // `DATE <start|*> TO <end|*>` → DateRange (dates as `YYYY-MM-DD`).
                if key.eq_ignore_ascii_case("DATE")
                    && matches!(iter.peek(), Some(Token::Number(_)) | Some(Token::Star))
                {
                    let start = if matches!(iter.peek(), Some(Token::Star)) {
                        iter.next();
                        None
                    } else {
                        self.parse_date(iter)
                    };
                    if matches!(iter.peek(), Some(Token::Ident(s)) if s.eq_ignore_ascii_case("TO"))
                    {
                        iter.next();
                    }
                    let end = if matches!(iter.peek(), Some(Token::Star)) {
                        iter.next();
                        None
                    } else {
                        self.parse_date(iter)
                    };
                    return Ok(Some(Condition::DateRange { start, end }));
                }
                self.parse_attribute_tail(iter, key)
            }
            Some(Token::StringLit(key)) => {
                let key = key.clone();
                iter.next();
                self.parse_attribute_tail(iter, key)
            }
            _ => Ok(None),
        }
    }

    /// Parses the tail of an attribute-keyed condition once the key has been
    /// consumed. Recognises the printer's `key = "value"` (→ `AttributeEquals`)
    /// and `key MATCHES/LIKE "pattern"` (→ `Pattern`) forms, falling back to a
    /// bare `HasAttribute` when no recognised operator follows. This keeps those
    /// condition kinds lossless across the `format_statute` → `parse_statute`
    /// round-trip.
    fn parse_attribute_tail<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
        key: String,
    ) -> DslResult<Option<Condition>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Operator(op)) if op.as_str() == "=" || op.as_str() == "==" => {
                iter.next();
                let value = self.parse_attribute_value(iter)?;
                Ok(Some(Condition::AttributeEquals { key, value }))
            }
            Some(Token::Matches) | Some(Token::Like) => {
                iter.next();
                let pattern = self.parse_attribute_value(iter)?;
                Ok(Some(Condition::Pattern {
                    attribute: key,
                    pattern,
                    negated: false,
                }))
            }
            // `attr IN {a, b, c}` → SetMembership (the printer uses `{..}`).
            Some(Token::In) => {
                iter.next();
                let values = self.parse_set_literal(iter)?;
                Ok(Some(Condition::SetMembership {
                    attribute: key,
                    values,
                    negated: false,
                }))
            }
            // `attr NOT IN {..}` / `attr NOT MATCHES "regex"` → negated variants.
            Some(Token::Not) => {
                iter.next();
                match iter.peek() {
                    Some(Token::In) => {
                        iter.next();
                        let values = self.parse_set_literal(iter)?;
                        Ok(Some(Condition::SetMembership {
                            attribute: key,
                            values,
                            negated: true,
                        }))
                    }
                    Some(Token::Matches) | Some(Token::Like) => {
                        iter.next();
                        let pattern = self.parse_attribute_value(iter)?;
                        Ok(Some(Condition::Pattern {
                            attribute: key,
                            pattern,
                            negated: true,
                        }))
                    }
                    _ => Ok(Some(Condition::HasAttribute { key })),
                }
            }
            _ => Ok(Some(Condition::HasAttribute { key })),
        }
    }

    /// Parses a brace/paren-delimited set literal (`{a, b, c}`) into the list of
    /// string members used by [`legalis_core::Condition::SetMembership`].
    fn parse_set_literal<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<Vec<String>>
    where
        I: Iterator<Item = &'a Token>,
    {
        if matches!(iter.peek(), Some(Token::LBrace) | Some(Token::LParen)) {
            iter.next();
        }
        let mut values = Vec::new();
        loop {
            let value: Option<String> = match iter.peek() {
                Some(Token::RBrace) | Some(Token::RParen) => {
                    iter.next();
                    break;
                }
                Some(Token::Then) | Some(Token::And) | Some(Token::Or) | None => break,
                Some(Token::Comma) => None,
                Some(Token::StringLit(s)) | Some(Token::Ident(s)) => Some(s.clone()),
                Some(Token::Number(n)) => Some(n.to_string()),
                _ => None,
            };
            iter.next();
            if let Some(v) = value {
                values.push(v);
            }
        }
        Ok(values)
    }

    /// Parses a [`RegionType`] variant name (as emitted by the printer's `{:?}`).
    fn parse_region_type<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<RegionType>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Ident(s)) => Ok(match s.as_str() {
                "Country" => RegionType::Country,
                "State" => RegionType::State,
                "City" => RegionType::City,
                "District" => RegionType::District,
                "PostalCode" => RegionType::PostalCode,
                _ => RegionType::Custom,
            }),
            other => Err(DslError::InvalidCondition(format!(
                "expected a region type, found {other:?}"
            ))),
        }
    }

    /// Parses a [`RelationshipType`] variant name (as emitted by the printer's `{:?}`).
    fn parse_relationship_type<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<RelationshipType>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Ident(s)) => match s.as_str() {
                "ParentChild" => Ok(RelationshipType::ParentChild),
                "Spouse" => Ok(RelationshipType::Spouse),
                "Employment" => Ok(RelationshipType::Employment),
                "Guardian" => Ok(RelationshipType::Guardian),
                "BusinessOwner" => Ok(RelationshipType::BusinessOwner),
                "Contractual" => Ok(RelationshipType::Contractual),
                other => Err(DslError::InvalidCondition(format!(
                    "unknown relationship type '{other}'"
                ))),
            },
            other => Err(DslError::InvalidCondition(format!(
                "expected a relationship type, found {other:?}"
            ))),
        }
    }

    /// Reads the parenthesised context of a `PERCENTAGE … (context)` condition.
    /// Tolerant of missing parentheses; returns an empty string if absent.
    fn parse_percentage_context<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<String>
    where
        I: Iterator<Item = &'a Token>,
    {
        if matches!(iter.peek(), Some(Token::LParen)) {
            iter.next();
        }
        let context = match iter.peek() {
            Some(Token::Ident(s)) | Some(Token::StringLit(s)) => Some(s.clone()),
            _ => None,
        };
        let context = match context {
            Some(c) => {
                iter.next();
                c
            }
            None => String::new(),
        };
        if matches!(iter.peek(), Some(Token::RParen)) {
            iter.next();
        }
        Ok(context)
    }

    /// Parses a duration unit word (`days`/`weeks`/`months`/`years`, as emitted
    /// by [`legalis_core::DurationUnit`]'s `Display`) into the enum.
    fn parse_duration_unit<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<DurationUnit>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Ident(u)) => match u.to_lowercase().as_str() {
                "day" | "days" => Ok(DurationUnit::Days),
                "week" | "weeks" => Ok(DurationUnit::Weeks),
                "month" | "months" => Ok(DurationUnit::Months),
                "year" | "years" => Ok(DurationUnit::Years),
                other => Err(DslError::InvalidCondition(format!(
                    "unknown duration unit '{other}'"
                ))),
            },
            other => Err(DslError::InvalidCondition(format!(
                "expected a duration unit, found {other:?}"
            ))),
        }
    }

    /// Reads a single scalar value (string literal, identifier, or number) as a
    /// `String`, used for attribute-equality values and pattern literals.
    fn parse_attribute_value<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<String>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::StringLit(s)) | Some(Token::Ident(s)) => Ok(s.clone()),
            Some(Token::Number(n)) => Ok(n.to_string()),
            Some(Token::Float(f)) => Ok(f.to_string()),
            other => Err(DslError::InvalidCondition(format!(
                "expected a value after attribute operator, found {other:?}"
            ))),
        }
    }

    /// Reads a numeric value as `f64`, accepting both integer and float literals
    /// (used for `Calculation` values).
    fn parse_float_value<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<f64>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Float(f)) => Ok(*f),
            Some(Token::Number(n)) => Ok(*n as f64),
            other => Err(DslError::InvalidCondition(format!(
                "expected a numeric value, found {other:?}"
            ))),
        }
    }

    pub(crate) fn parse_comparison_op<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<legalis_core::ComparisonOp>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Operator(op)) => match op.as_str() {
                ">=" => Ok(legalis_core::ComparisonOp::GreaterOrEqual),
                "<=" => Ok(legalis_core::ComparisonOp::LessOrEqual),
                ">" => Ok(legalis_core::ComparisonOp::GreaterThan),
                "<" => Ok(legalis_core::ComparisonOp::LessThan),
                "==" | "=" => Ok(legalis_core::ComparisonOp::Equal),
                "!=" => Ok(legalis_core::ComparisonOp::NotEqual),
                _ => Err(DslError::InvalidCondition(format!(
                    "Unknown operator: {op}"
                ))),
            },
            _ => Err(DslError::InvalidCondition(
                "Expected comparison operator".to_string(),
            )),
        }
    }

    fn parse_number<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<u64>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.next() {
            Some(Token::Number(n)) => Ok(*n),
            _ => Err(DslError::InvalidCondition("Expected number".to_string())),
        }
    }

    fn parse_effect<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> DslResult<Effect>
    where
        I: Iterator<Item = &'a Token>,
    {
        let effect_type = match iter.next() {
            Some(Token::Grant) => EffectType::Grant,
            Some(Token::Revoke) => EffectType::Revoke,
            Some(Token::Obligation) => EffectType::Obligation,
            Some(Token::Prohibition) => EffectType::Prohibition,
            Some(Token::Ident(_)) => EffectType::Custom,
            _ => EffectType::Custom,
        };

        let description = match iter.peek() {
            Some(Token::StringLit(s)) => {
                let s = s.clone();
                iter.next();
                s
            }
            _ => "No description".to_string(),
        };

        Ok(Effect::new(effect_type, description))
    }

    /// Parses a date in YYYY-MM-DD format.
    fn parse_date<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> Option<NaiveDate>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Try to parse date as YYYY-MM-DD (Number-Dash-Number-Dash-Number)
        // or as a quoted string "YYYY-MM-DD"
        match iter.peek() {
            Some(Token::StringLit(s)) => {
                let date_str = s.clone();
                iter.next();
                NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()
            }
            Some(Token::Number(year)) => {
                let year = *year as i32;
                iter.next();

                // Expect dash
                if !matches!(iter.next(), Some(Token::Dash)) {
                    return None;
                }

                // Month
                let month = match iter.next() {
                    Some(Token::Number(m)) => *m as u32,
                    _ => return None,
                };

                // Expect dash
                if !matches!(iter.next(), Some(Token::Dash)) {
                    return None;
                }

                // Day
                let day = match iter.next() {
                    Some(Token::Number(d)) => *d as u32,
                    _ => return None,
                };

                NaiveDate::from_ymd_opt(year, month, day)
            }
            _ => None,
        }
    }
}
