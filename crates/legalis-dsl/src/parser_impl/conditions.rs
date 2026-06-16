//! Condition parsing producing [`crate::ast::ConditionNode`] values.
//!
//! Implements the recursive-descent grammar for conditions used by both statute
//! bodies and (via the `pub(crate)` `parse_condition_node` entry point) the
//! contract grammar in [`crate::contract_parser`]: OR/AND/unary precedence,
//! primary atoms, temporal/range/field comparisons, and `IN` set helpers. Split
//! out of the original `parser_impl.rs` to keep every file under 2000 lines.

use super::LegalDslParser;
use crate::ast::{self, Token};
use crate::{DslError, DslResult};

impl LegalDslParser {
    /// Parses a condition into an AST ConditionNode. `pub(crate)` so the
    /// contract grammar in [`crate::contract_parser`] reuses the same rules.
    pub(crate) fn parse_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        self.parse_or_condition_node(iter)
    }

    fn parse_or_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let Some(mut result) = self.parse_and_condition_node(iter)? else {
            return Ok(None);
        };

        while matches!(iter.peek(), Some(Token::Or)) {
            iter.next();
            let right = self.parse_and_condition_node(iter)?;
            if let Some(right_cond) = right {
                result = ast::ConditionNode::Or(Box::new(result), Box::new(right_cond));
            }
        }

        Ok(Some(result))
    }

    fn parse_and_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let Some(mut result) = self.parse_unary_condition_node(iter)? else {
            return Ok(None);
        };

        while matches!(iter.peek(), Some(Token::And)) {
            iter.next();
            let right = self.parse_unary_condition_node(iter)?;
            if let Some(right_cond) = right {
                result = ast::ConditionNode::And(Box::new(result), Box::new(right_cond));
            }
        }

        Ok(Some(result))
    }

    fn parse_unary_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Not) => {
                iter.next();
                let inner = self.parse_unary_condition_node(iter)?;
                Ok(inner.map(|c| ast::ConditionNode::Not(Box::new(c))))
            }
            Some(Token::LParen) => {
                iter.next();
                let inner = self.parse_or_condition_node(iter)?;
                match iter.peek() {
                    Some(Token::RParen) => {
                        iter.next();
                    }
                    _ => return Err(DslError::UnmatchedParen(None)),
                }
                Ok(inner)
            }
            _ => self.parse_primary_condition_node(iter),
        }
    }

    fn parse_primary_condition_node<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek().cloned() {
            Some(Token::Age) => {
                iter.next();
                self.parse_field_condition(iter, "age")
            }
            Some(Token::Income) => {
                iter.next();
                self.parse_field_condition(iter, "income")
            }
            Some(Token::CurrentDate) => {
                iter.next();
                self.parse_temporal_condition(iter, ast::TemporalField::CurrentDate)
            }
            Some(Token::DateField) => {
                iter.next();
                // Expect field name
                let field_name = match iter.next() {
                    Some(Token::Ident(s)) => s.clone(),
                    Some(Token::StringLit(s)) => s.clone(),
                    _ => {
                        return Err(DslError::InvalidCondition(
                            "Expected field name after DATE_FIELD".to_string(),
                        ));
                    }
                };
                self.parse_temporal_condition(iter, ast::TemporalField::DateField(field_name))
            }
            Some(Token::Has) => {
                iter.next();
                if let Some(Token::Ident(key)) = iter.peek() {
                    let key = key.clone();
                    iter.next();
                    Ok(Some(ast::ConditionNode::HasAttribute { key }))
                } else if let Some(Token::StringLit(key)) = iter.peek() {
                    let key = key.clone();
                    iter.next();
                    Ok(Some(ast::ConditionNode::HasAttribute { key }))
                } else {
                    Err(DslError::InvalidCondition(
                        "Expected attribute key after HAS".to_string(),
                    ))
                }
            }
            Some(Token::Ident(name)) => {
                let name = name.clone();
                iter.next();
                // Check for qualified reference (alias.statute_id)
                if matches!(iter.peek(), Some(Token::Dot)) {
                    iter.next(); // consume dot
                    if let Some(Token::Ident(member)) = iter.next() {
                        // This is a qualified reference like "other.adult_rights"
                        Ok(Some(ast::ConditionNode::HasAttribute {
                            key: format!("{}.{}", name, member),
                        }))
                    } else {
                        Err(DslError::parse_error("Expected identifier after '.'"))
                    }
                } else if matches!(
                    iter.peek(),
                    Some(Token::Operator(_))
                        | Some(Token::Between)
                        | Some(Token::In)
                        | Some(Token::Like)
                        | Some(Token::Matches)
                        | Some(Token::InRange)
                        | Some(Token::NotInRange)
                ) {
                    // This is a field condition (e.g., "email MATCHES pattern")
                    self.parse_field_condition(iter, &name)
                } else {
                    Ok(Some(ast::ConditionNode::HasAttribute { key: name }))
                }
            }
            Some(Token::Then) | Some(Token::RBrace) | Some(Token::Discretion) => Ok(None),
            _ => Ok(None),
        }
    }

    /// Parses temporal field conditions (date comparisons).
    fn parse_temporal_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
        field: ast::TemporalField,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let op = self.parse_comparison_op(iter)?;
        let value = self.parse_condition_value(iter)?;
        Ok(Some(ast::ConditionNode::TemporalComparison {
            field,
            operator: op.to_string(),
            value,
        }))
    }

    /// Parses numeric range conditions with inclusive/exclusive bounds.
    /// Syntax: IN_RANGE min..max or IN_RANGE (min..max) or IN_RANGE [min..max]
    fn parse_range_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
        field: &str,
        negated: bool,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Check for opening bracket/paren to determine inclusivity
        let mut inclusive_min = true;
        let mut inclusive_max = true;

        // Look for optional opening bracket
        if matches!(iter.peek(), Some(Token::LParen)) {
            iter.next();
            inclusive_min = false;
        }

        // Parse min value
        let min = self.parse_condition_value(iter)?;

        // Expect .. or ...
        match iter.peek() {
            Some(Token::Dot) => {
                iter.next(); // first dot
                if matches!(iter.peek(), Some(Token::Dot)) {
                    iter.next(); // second dot
                    // Check for third dot (exclusive max)
                    if matches!(iter.peek(), Some(Token::Dot)) {
                        iter.next();
                        inclusive_max = false;
                    }
                } else {
                    return Err(DslError::InvalidCondition(
                        "Expected '..' or '...' in range".to_string(),
                    ));
                }
            }
            _ => {
                return Err(DslError::InvalidCondition(
                    "Expected '..' in range expression".to_string(),
                ));
            }
        }

        // Parse max value
        let max = self.parse_condition_value(iter)?;

        // Look for closing bracket/paren
        if matches!(iter.peek(), Some(Token::RParen)) {
            iter.next();
            if !inclusive_min {
                inclusive_max = false; // (min..max) - both exclusive
            }
        }

        if negated {
            Ok(Some(ast::ConditionNode::NotInRange {
                field: field.to_string(),
                min,
                max,
                inclusive_min,
                inclusive_max,
            }))
        } else {
            Ok(Some(ast::ConditionNode::InRange {
                field: field.to_string(),
                min,
                max,
                inclusive_min,
                inclusive_max,
            }))
        }
    }

    /// Parses field conditions including BETWEEN, IN, LIKE, and comparison operators.
    fn parse_field_condition<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
        field: &str,
    ) -> DslResult<Option<ast::ConditionNode>>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Between) => {
                iter.next(); // consume BETWEEN
                let min = self.parse_condition_value(iter)?;
                // Expect AND
                if !matches!(iter.next(), Some(Token::And)) {
                    return Err(DslError::InvalidCondition(
                        "Expected AND in BETWEEN expression".to_string(),
                    ));
                }
                let max = self.parse_condition_value(iter)?;
                Ok(Some(ast::ConditionNode::Between {
                    field: field.to_string(),
                    min,
                    max,
                }))
            }
            Some(Token::In) => {
                iter.next(); // consume IN
                // Expect opening paren or bracket
                let _has_paren = if matches!(iter.peek(), Some(Token::LParen)) {
                    iter.next();
                    true
                } else {
                    false
                };

                let mut values = Vec::new();
                loop {
                    if matches!(iter.peek(), Some(Token::RParen) | Some(Token::Comma)) {
                        if matches!(iter.peek(), Some(Token::RParen)) {
                            iter.next(); // consume closing paren
                            break;
                        }
                        if matches!(iter.peek(), Some(Token::Comma)) {
                            iter.next(); // consume comma
                            continue;
                        }
                    }

                    if matches!(
                        iter.peek(),
                        Some(Token::Then)
                            | Some(Token::And)
                            | Some(Token::Or)
                            | Some(Token::RBrace)
                    ) {
                        break;
                    }

                    let value = self.parse_condition_value(iter)?;
                    values.push(value);

                    if matches!(iter.peek(), Some(Token::Comma)) {
                        iter.next(); // consume comma
                    } else if matches!(iter.peek(), Some(Token::RParen)) {
                        iter.next(); // consume closing paren
                        break;
                    } else {
                        break;
                    }
                }

                Ok(Some(ast::ConditionNode::In {
                    field: field.to_string(),
                    values,
                }))
            }
            Some(Token::Like) => {
                iter.next(); // consume LIKE
                let pattern = match iter.next() {
                    Some(Token::StringLit(s)) => s.clone(),
                    Some(Token::Ident(s)) => s.clone(),
                    _ => {
                        return Err(DslError::InvalidCondition(
                            "Expected pattern after LIKE".to_string(),
                        ));
                    }
                };
                Ok(Some(ast::ConditionNode::Like {
                    field: field.to_string(),
                    pattern,
                }))
            }
            Some(Token::Matches) => {
                iter.next(); // consume MATCHES
                let regex_pattern = match iter.next() {
                    Some(Token::StringLit(s)) => s.clone(),
                    Some(Token::Ident(s)) => s.clone(),
                    _ => {
                        return Err(DslError::InvalidCondition(
                            "Expected regex pattern after MATCHES".to_string(),
                        ));
                    }
                };
                // Validate regex pattern
                if let Err(e) = regex::Regex::new(&regex_pattern) {
                    return Err(DslError::InvalidCondition(format!(
                        "Invalid regex pattern: {}",
                        e
                    )));
                }
                Ok(Some(ast::ConditionNode::Matches {
                    field: field.to_string(),
                    regex_pattern,
                }))
            }
            Some(Token::InRange) => {
                iter.next(); // consume IN_RANGE
                self.parse_range_condition(iter, field, false)
            }
            Some(Token::NotInRange) => {
                iter.next(); // consume NOT_IN_RANGE
                self.parse_range_condition(iter, field, true)
            }
            Some(Token::Operator(_)) => {
                let op = self.parse_comparison_op(iter)?;
                let value = self.parse_condition_value(iter)?;
                Ok(Some(ast::ConditionNode::Comparison {
                    field: field.to_string(),
                    operator: op.to_string(),
                    value,
                }))
            }
            _ => Err(DslError::InvalidCondition(format!(
                "Expected comparison operator, BETWEEN, IN, or LIKE after {}",
                field
            ))),
        }
    }

    /// Parses a condition value (number, string, date, or boolean).
    fn parse_condition_value<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::ConditionValue>
    where
        I: Iterator<Item = &'a Token>,
    {
        match iter.peek() {
            Some(Token::Number(n)) => {
                let val = ast::ConditionValue::Number(*n as i64);
                iter.next();
                Ok(val)
            }
            Some(Token::StringLit(s)) => {
                let s = s.clone();
                iter.next();
                // Check if it looks like a date (YYYY-MM-DD)
                if s.contains('-')
                    && s.split('-').count() == 3
                    && s.split('-').all(|part| part.parse::<u32>().is_ok())
                {
                    Ok(ast::ConditionValue::Date(s))
                } else {
                    Ok(ast::ConditionValue::String(s))
                }
            }
            Some(Token::Ident(s)) => {
                let s_upper = s.to_uppercase();
                let val = if s_upper == "TRUE" {
                    ast::ConditionValue::Boolean(true)
                } else if s_upper == "FALSE" {
                    ast::ConditionValue::Boolean(false)
                } else {
                    ast::ConditionValue::String(s.clone())
                };
                iter.next();
                Ok(val)
            }
            _ => Err(DslError::InvalidCondition(
                "Expected value (number, string, date, or boolean)".to_string(),
            )),
        }
    }

    /// Parses a set expression for set operations.
    /// Supports UNION, INTERSECT, and DIFFERENCE operations.
    /// Example: (1, 2, 3) UNION (4, 5, 6)
    #[allow(dead_code)]
    fn parse_set_expression<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::SetExpression>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Parse the initial set (values in parentheses or a single value list)
        let left = self.parse_simple_set(iter)?;

        // Check for set operations
        match iter.peek() {
            Some(Token::Union) => {
                iter.next(); // consume UNION
                let right = self.parse_set_expression(iter)?;
                Ok(ast::SetExpression::Union(Box::new(left), Box::new(right)))
            }
            Some(Token::Intersect) => {
                iter.next(); // consume INTERSECT
                let right = self.parse_set_expression(iter)?;
                Ok(ast::SetExpression::Intersect(
                    Box::new(left),
                    Box::new(right),
                ))
            }
            Some(Token::Difference) => {
                iter.next(); // consume DIFFERENCE
                let right = self.parse_set_expression(iter)?;
                Ok(ast::SetExpression::Difference(
                    Box::new(left),
                    Box::new(right),
                ))
            }
            _ => Ok(left),
        }
    }

    /// Parses a simple set of values (without operations).
    #[allow(dead_code)]
    fn parse_simple_set<'a, I>(
        &self,
        iter: &mut std::iter::Peekable<I>,
    ) -> DslResult<ast::SetExpression>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut values = Vec::new();

        // Expect opening paren (optional if already consumed)
        if matches!(iter.peek(), Some(Token::LParen)) {
            iter.next(); // consume opening paren
        }

        // Parse values until we hit a closing paren or set operator
        loop {
            if matches!(
                iter.peek(),
                Some(Token::RParen)
                    | Some(Token::Union)
                    | Some(Token::Intersect)
                    | Some(Token::Difference)
            ) {
                break;
            }

            // Skip commas
            if matches!(iter.peek(), Some(Token::Comma)) {
                iter.next();
                continue;
            }

            // Stop at logical operators or statement terminators
            if matches!(
                iter.peek(),
                Some(Token::And) | Some(Token::Or) | Some(Token::Then) | Some(Token::RBrace)
            ) {
                break;
            }

            let value = self.parse_condition_value(iter)?;
            values.push(value);
        }

        // Consume closing paren if present
        if matches!(iter.peek(), Some(Token::RParen)) {
            iter.next();
        }

        Ok(ast::SetExpression::Values(values))
    }
}
