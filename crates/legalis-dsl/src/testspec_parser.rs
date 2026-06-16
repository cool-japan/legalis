//! Parser for the test-specification grammar (`@mock`, `@property`,
//! `@coverage`, `@snapshot`).
//!
//! Like [`crate::contract_parser`], these rules live in their own file (rather
//! than the over-long `parser_impl.rs`) but extend the same
//! [`crate::LegalDslParser`] through an additional `impl` block. They operate on
//! [`SpannedToken`]s for accurate line/column errors and reuse the
//! `pub(crate)` lexing helpers and the `@test`/`GIVEN`/`EXPECT` sub-parsers from
//! [`crate::contract_parser`] so the shared vocabulary stays identical.
//!
//! See [`crate::testspec`] for the AST these rules build, the runners that
//! execute the resulting [`TestSpecDocument`], and the round-tripping printer in
//! [`crate::printer`].

use crate::DslResult;
use crate::LegalDslParser;
use crate::ast::{SpannedToken, Token};
use crate::contract::{TestBinding, TestCaseNode};
use crate::contract_parser::{
    err_at, expect_simple, is_ident_kw, peek_tok, read_binding_key, read_test_value, read_word,
    skip_block,
};
use crate::testspec::{
    CoverageComparator, CoverageMetric, CoverageRequirementNode, MockEntityNode, PropertyDomain,
    PropertySpecNode, PropertyVar, SnapshotAssertionNode, SnapshotMode, TestSpecDocument,
    TestSpecReport,
};

/// A top-level `@`-directive. The large payloads are boxed so the variants are
/// uniformly small.
pub(crate) enum Directive {
    /// An inline `@test` case.
    Test(Box<TestCaseNode>),
    /// A `@mock` entity definition.
    Mock(MockEntityNode),
    /// A `@property` specification.
    Property(Box<PropertySpecNode>),
    /// A `@coverage` requirement.
    Coverage(CoverageRequirementNode),
    /// A `@snapshot` assertion.
    Snapshot(SnapshotAssertionNode),
}

/// Reads an optionally-signed integer bound (`-5`, `120`), advancing past it.
fn read_int_bound(toks: &[SpannedToken], pos: &mut usize, what: &str) -> DslResult<i64> {
    let negative = matches!(peek_tok(toks, *pos), Some(Token::Dash));
    if negative {
        *pos += 1;
    }
    match peek_tok(toks, *pos) {
        Some(Token::Number(n)) => {
            let value = *n as i64;
            *pos += 1;
            Ok(if negative { -value } else { value })
        }
        _ => Err(err_at(
            toks,
            *pos,
            format!("expected an integer for {what}"),
        )),
    }
}

/// Reads a non-negative case count (after `CASES`), advancing past it.
fn read_count(toks: &[SpannedToken], pos: &mut usize) -> DslResult<usize> {
    match peek_tok(toks, *pos) {
        Some(Token::Number(n)) => {
            let value = *n as usize;
            *pos += 1;
            Ok(value)
        }
        _ => Err(err_at(toks, *pos, "expected a case count after CASES")),
    }
}

/// Reads a percentage threshold (`80`, `75.5`); a trailing `%` is ignored by the
/// lexer and so needs no explicit consumption.
fn read_percent(toks: &[SpannedToken], pos: &mut usize) -> DslResult<f64> {
    let value = match peek_tok(toks, *pos) {
        Some(Token::Number(n)) => *n as f64,
        Some(Token::Float(f)) => *f,
        _ => return Err(err_at(toks, *pos, "expected a percentage threshold")),
    };
    *pos += 1;
    Ok(value)
}

/// Reads a coverage comparator operator (`>=`, `>`, `=`/`==`).
fn read_comparator(toks: &[SpannedToken], pos: &mut usize) -> DslResult<CoverageComparator> {
    match peek_tok(toks, *pos) {
        Some(Token::Operator(op)) => match CoverageComparator::from_operator(op) {
            Some(comparator) => {
                *pos += 1;
                Ok(comparator)
            }
            None => Err(err_at(
                toks,
                *pos,
                format!("unsupported comparator '{op}' (use >=, > or ==)"),
            )),
        },
        _ => Err(err_at(
            toks,
            *pos,
            "expected a comparator (>=, > or ==) after the coverage metric",
        )),
    }
}

impl LegalDslParser {
    /// Parses every `@`-directive and statute-test construct in `input` into a
    /// [`TestSpecDocument`]. `STATUTE`/`CONTRACT` blocks (and module
    /// declarations) are skipped — use the dedicated parsers for those.
    pub fn parse_test_spec_document(&self, input: &str) -> DslResult<TestSpecDocument> {
        let toks = self.tokenize(input)?;
        let mut pos = 0;
        let mut doc = TestSpecDocument::default();

        while pos < toks.len() {
            match &toks[pos].token {
                Token::At => match self.parse_directive(&toks, &mut pos)? {
                    Directive::Test(case) => doc.tests.push(*case),
                    Directive::Mock(mock) => doc.mocks.push(mock),
                    Directive::Property(prop) => doc.properties.push(*prop),
                    Directive::Coverage(req) => doc.coverage.push(req),
                    Directive::Snapshot(snap) => doc.snapshots.push(snap),
                },
                Token::Statute | Token::Contract | Token::Public | Token::Private => {
                    skip_block(&toks, &mut pos)
                }
                _ => pos += 1,
            }
        }

        Ok(doc)
    }

    /// Parses the statutes and the full test specification from `input` and runs
    /// every directive against the statutes. This is the executable entry point
    /// for the complete Test DSL.
    pub fn run_test_spec(&self, input: &str) -> DslResult<TestSpecReport> {
        let statutes = self.parse_statutes(input)?;
        let spec = self.parse_test_spec_document(input)?;
        Ok(spec.run(&statutes))
    }

    /// Dispatches a leading `@` to the matching directive parser. Shared by
    /// [`LegalDslParser::parse_contract_document`] (which keeps only `Test`) and
    /// [`LegalDslParser::parse_test_spec_document`].
    pub(crate) fn parse_directive(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<Directive> {
        let unknown = |toks: &[SpannedToken], at: usize| {
            err_at(
                toks,
                at,
                "expected 'test', 'mock', 'property', 'coverage' or 'snapshot' after '@'",
            )
        };
        let directive = match peek_tok(toks, *pos + 1) {
            Some(Token::Ident(word)) => word.to_ascii_lowercase(),
            _ => return Err(unknown(toks, *pos + 1)),
        };
        match directive.as_str() {
            "test" => Ok(Directive::Test(Box::new(self.parse_test_case(toks, pos)?))),
            "mock" => Ok(Directive::Mock(self.parse_mock(toks, pos)?)),
            "property" => Ok(Directive::Property(Box::new(
                self.parse_property(toks, pos)?,
            ))),
            "coverage" => Ok(Directive::Coverage(self.parse_coverage(toks, pos)?)),
            "snapshot" => Ok(Directive::Snapshot(self.parse_snapshot(toks, pos)?)),
            _ => Err(unknown(toks, *pos + 1)),
        }
    }

    /// Parses `@mock <id> { [<key> = <value>[, ...]] }`.
    fn parse_mock(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<MockEntityNode> {
        *pos += 1; // consume '@'
        if !is_ident_kw(toks, *pos, "mock") {
            return Err(err_at(toks, *pos, "expected 'mock' after '@'"));
        }
        *pos += 1; // consume 'mock'

        let id = read_word(toks, pos, "mock entity identifier")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::LBrace),
            "'{' to open @mock block",
        )?;

        let mut bindings = Vec::new();
        loop {
            match peek_tok(toks, *pos) {
                Some(Token::RBrace) => {
                    *pos += 1;
                    break;
                }
                None => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "unterminated @mock block (expected '}')",
                    ));
                }
                _ => {
                    let key = read_binding_key(toks, pos)?;
                    expect_simple(
                        toks,
                        pos,
                        |t| {
                            matches!(t, Token::Operator(op) if op == "=" || op == "==")
                                || matches!(t, Token::Colon)
                        },
                        "'=' in mock binding",
                    )?;
                    let value = read_test_value(toks, pos)?;
                    bindings.push(TestBinding { key, value });
                    if matches!(peek_tok(toks, *pos), Some(Token::Comma)) {
                        *pos += 1;
                    }
                }
            }
        }

        Ok(MockEntityNode { id, bindings })
    }

    /// Parses `@property "<name>" FOR <statute> { (FORALL ..)+ [GIVEN ..]
    /// [USING ..] EXPECT .. [CASES n] }`.
    fn parse_property(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<PropertySpecNode> {
        *pos += 1; // consume '@'
        if !is_ident_kw(toks, *pos, "property") {
            return Err(err_at(toks, *pos, "expected 'property' after '@'"));
        }
        *pos += 1; // consume 'property'

        let name = read_word(toks, pos, "property name")?;
        if !is_ident_kw(toks, *pos, "FOR") {
            return Err(err_at(toks, *pos, "expected FOR after the property name"));
        }
        *pos += 1; // consume FOR
        let target_statute = read_word(toks, pos, "statute id after FOR")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::LBrace),
            "'{' to open @property block",
        )?;

        let mut vars = Vec::new();
        let mut fixed_bindings = Vec::new();
        let mut uses = Vec::new();
        let mut expectation = None;
        let mut max_cases = None;
        loop {
            match peek_tok(toks, *pos) {
                Some(Token::RBrace) => {
                    *pos += 1;
                    break;
                }
                None => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "unterminated @property block (expected '}')",
                    ));
                }
                _ if is_ident_kw(toks, *pos, "FORALL") => {
                    *pos += 1;
                    vars.push(self.parse_forall(toks, pos)?);
                }
                _ if is_ident_kw(toks, *pos, "GIVEN") => {
                    *pos += 1;
                    self.parse_test_bindings(toks, pos, &mut fixed_bindings)?;
                }
                _ if is_ident_kw(toks, *pos, "USING") => {
                    *pos += 1;
                    self.parse_mock_refs(toks, pos, &mut uses)?;
                }
                _ if is_ident_kw(toks, *pos, "EXPECT") => {
                    *pos += 1;
                    expectation = Some(self.parse_expectation(toks, pos)?);
                }
                _ if is_ident_kw(toks, *pos, "CASES") => {
                    *pos += 1;
                    max_cases = Some(read_count(toks, pos)?);
                }
                Some(_) => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "expected FORALL, GIVEN, USING, EXPECT, CASES or '}' in @property block",
                    ));
                }
            }
        }

        if vars.is_empty() {
            return Err(err_at(
                toks,
                *pos,
                "@property block needs at least one FORALL clause",
            ));
        }
        let expectation = expectation
            .ok_or_else(|| err_at(toks, *pos, "@property block is missing an EXPECT clause"))?;

        Ok(PropertySpecNode {
            name,
            target_statute,
            vars,
            fixed_bindings,
            uses,
            expectation,
            max_cases,
        })
    }

    /// Parses a single `FORALL <var> IN <domain>` clause (the `FORALL` keyword is
    /// already consumed).
    fn parse_forall(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<PropertyVar> {
        let name = read_binding_key(toks, pos)?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::In),
            "IN after the FORALL variable",
        )?;
        let domain = self.parse_domain(toks, pos)?;
        Ok(PropertyVar { name, domain })
    }

    /// Parses a property domain: either a value list `( v1, v2, ... )` or an
    /// inclusive integer range `<lo> TO <hi>`.
    fn parse_domain(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<PropertyDomain> {
        if matches!(peek_tok(toks, *pos), Some(Token::LParen)) {
            *pos += 1; // consume '('
            let mut values = Vec::new();
            loop {
                match peek_tok(toks, *pos) {
                    Some(Token::RParen) => {
                        *pos += 1;
                        break;
                    }
                    None => {
                        return Err(err_at(toks, *pos, "unterminated value list (expected ')')"));
                    }
                    _ => {
                        values.push(read_test_value(toks, pos)?);
                        if matches!(peek_tok(toks, *pos), Some(Token::Comma)) {
                            *pos += 1;
                        }
                    }
                }
            }
            if values.is_empty() {
                return Err(err_at(
                    toks,
                    *pos,
                    "value list domain must list at least one value",
                ));
            }
            Ok(PropertyDomain::Values(values))
        } else {
            let lo = read_int_bound(toks, pos, "range lower bound")?;
            if !is_ident_kw(toks, *pos, "TO") {
                return Err(err_at(toks, *pos, "expected TO between range bounds"));
            }
            *pos += 1; // consume TO
            let hi = read_int_bound(toks, pos, "range upper bound")?;
            if hi < lo {
                return Err(err_at(
                    toks,
                    *pos,
                    format!("range upper bound {hi} is below lower bound {lo}"),
                ));
            }
            Ok(PropertyDomain::IntRange { lo, hi })
        }
    }

    /// Parses `@coverage REQUIRE <metric> <op> <pct>[%] [FOR <statute>]`.
    fn parse_coverage(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<CoverageRequirementNode> {
        *pos += 1; // consume '@'
        if !is_ident_kw(toks, *pos, "coverage") {
            return Err(err_at(toks, *pos, "expected 'coverage' after '@'"));
        }
        *pos += 1; // consume 'coverage'
        if !is_ident_kw(toks, *pos, "REQUIRE") {
            return Err(err_at(toks, *pos, "expected REQUIRE after @coverage"));
        }
        *pos += 1; // consume REQUIRE

        let metric_pos = *pos;
        let metric_word = read_word(toks, pos, "coverage metric (statutes|outcomes)")?;
        let metric = CoverageMetric::from_keyword(&metric_word).ok_or_else(|| {
            err_at(
                toks,
                metric_pos,
                format!("unknown coverage metric '{metric_word}' (use statutes or outcomes)"),
            )
        })?;
        let comparator = read_comparator(toks, pos)?;
        let threshold = read_percent(toks, pos)?;
        let target = if is_ident_kw(toks, *pos, "FOR") {
            *pos += 1;
            Some(read_word(toks, pos, "statute id after FOR")?)
        } else {
            None
        };

        Ok(CoverageRequirementNode {
            metric,
            comparator,
            threshold,
            target,
        })
    }

    /// Parses `@snapshot "<name>" FOR <statute> (EXPECT "<sig>" | RECORD)`.
    fn parse_snapshot(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<SnapshotAssertionNode> {
        *pos += 1; // consume '@'
        if !is_ident_kw(toks, *pos, "snapshot") {
            return Err(err_at(toks, *pos, "expected 'snapshot' after '@'"));
        }
        *pos += 1; // consume 'snapshot'

        let name = read_word(toks, pos, "snapshot name")?;
        if !is_ident_kw(toks, *pos, "FOR") {
            return Err(err_at(toks, *pos, "expected FOR after the snapshot name"));
        }
        *pos += 1; // consume FOR
        let target_statute = read_word(toks, pos, "statute id after FOR")?;

        let mode = if is_ident_kw(toks, *pos, "EXPECT") {
            *pos += 1;
            let signature = read_word(toks, pos, "expected signature after EXPECT")?;
            SnapshotMode::Match(signature)
        } else if is_ident_kw(toks, *pos, "RECORD") {
            *pos += 1;
            SnapshotMode::Record
        } else {
            return Err(err_at(
                toks,
                *pos,
                "expected EXPECT \"<signature>\" or RECORD in @snapshot",
            ));
        };

        Ok(SnapshotAssertionNode {
            name,
            target_statute,
            mode,
        })
    }
}
