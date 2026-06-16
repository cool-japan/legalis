//! Parser for the contract / compliance / inline-test grammar.
//!
//! These rules live in their own module (rather than `parser_impl.rs`, which is
//! already at the 2000-line ceiling) but extend the very same
//! [`crate::LegalDslParser`] via an additional `impl` block. They operate on
//! [`SpannedToken`]s so every clause-level error carries an accurate
//! line/column, and they reuse [`LegalDslParser::parse_condition_node`] so the
//! `WHEN` grammar stays identical to statutes.
//!
//! See [`crate::contract`] for the AST these rules build and the round-tripping
//! printer in [`crate::printer`].

use crate::LegalDslParser;
use crate::ast::{ConditionNode, SpannedToken, Token};
use crate::contract::{
    ClauseNode, ComplianceRequirementNode, ContractDocument, ContractNode, DeadlineNode,
    ExpectedEffect, InspectionNode, ObligationNode, PartyNode, PartyRole, PenaltyNode,
    PerformanceBlock, ReportFrequency, ReportNode, RightKind, RightNode, TestBinding, TestCaseNode,
    TestExpectation, TestRunReport, TestValue, TimelineNode, run_test_cases,
};
use crate::{DslError, DslResult};

/// Returns the token at `pos`, if any.
pub(crate) fn peek_tok(toks: &[SpannedToken], pos: usize) -> Option<&Token> {
    toks.get(pos).map(|st| &st.token)
}

/// Returns `true` when the token at `pos` is an identifier equal (ignoring
/// case) to `keyword`.
pub(crate) fn is_ident_kw(toks: &[SpannedToken], pos: usize, keyword: &str) -> bool {
    matches!(peek_tok(toks, pos), Some(Token::Ident(s)) if s.eq_ignore_ascii_case(keyword))
}

/// Builds a located parse error pointing at `pos` (or the end of input).
pub(crate) fn err_at(toks: &[SpannedToken], pos: usize, message: impl Into<String>) -> DslError {
    match toks.get(pos).or_else(|| toks.last()) {
        Some(st) => DslError::parse_error_at(st.location.line, st.location.column, message),
        None => DslError::parse_error(message),
    }
}

/// Reads an identifier / string / number scalar as a `String`, advancing past
/// it. Used for ids and cross-references.
pub(crate) fn read_word(toks: &[SpannedToken], pos: &mut usize, what: &str) -> DslResult<String> {
    match peek_tok(toks, *pos) {
        Some(Token::StringLit(s)) | Some(Token::Ident(s)) => {
            let value = s.clone();
            *pos += 1;
            Ok(value)
        }
        Some(Token::Number(n)) => {
            let value = n.to_string();
            *pos += 1;
            Ok(value)
        }
        _ => Err(err_at(toks, *pos, format!("expected {what}"))),
    }
}

/// Reads a quoted date string (`"YYYY-MM-DD"`), advancing past it.
fn read_quoted_date(toks: &[SpannedToken], pos: &mut usize, what: &str) -> DslResult<String> {
    match peek_tok(toks, *pos) {
        Some(Token::StringLit(s)) => {
            let value = s.clone();
            *pos += 1;
            Ok(value)
        }
        _ => Err(err_at(
            toks,
            *pos,
            format!("expected a quoted date (\"YYYY-MM-DD\") for {what}"),
        )),
    }
}

/// Reads an integer literal as `i64`, advancing past it.
fn read_number(toks: &[SpannedToken], pos: &mut usize, what: &str) -> DslResult<i64> {
    match peek_tok(toks, *pos) {
        Some(Token::Number(n)) => {
            let value = *n as i64;
            *pos += 1;
            Ok(value)
        }
        _ => Err(err_at(toks, *pos, format!("expected a number for {what}"))),
    }
}

/// Consumes a single expected punctuation token, or reports an error.
pub(crate) fn expect_simple(
    toks: &[SpannedToken],
    pos: &mut usize,
    matcher: impl Fn(&Token) -> bool,
    what: &str,
) -> DslResult<()> {
    match peek_tok(toks, *pos) {
        Some(tok) if matcher(tok) => {
            *pos += 1;
            Ok(())
        }
        _ => Err(err_at(toks, *pos, format!("expected {what}"))),
    }
}

/// Reads a recurrence frequency value following an `EVERY` keyword.
fn read_frequency(toks: &[SpannedToken], pos: &mut usize) -> DslResult<ReportFrequency> {
    match peek_tok(toks, *pos) {
        Some(Token::StringLit(s)) => {
            let freq = ReportFrequency::Custom(s.clone());
            *pos += 1;
            Ok(freq)
        }
        Some(Token::Ident(s)) => {
            let freq = ReportFrequency::from_keyword(s);
            *pos += 1;
            Ok(freq)
        }
        _ => Err(err_at(
            toks,
            *pos,
            "expected a frequency keyword or quoted cadence after EVERY",
        )),
    }
}

/// Reads a `GIVEN` binding name. Accepts ordinary identifiers and the reserved
/// field keywords (`AGE`, `INCOME`) that double as attribute names in the
/// evaluation context, so test bindings can target them.
pub(crate) fn read_binding_key(toks: &[SpannedToken], pos: &mut usize) -> DslResult<String> {
    let key = match peek_tok(toks, *pos) {
        Some(Token::StringLit(s)) | Some(Token::Ident(s)) => s.clone(),
        Some(Token::Number(n)) => n.to_string(),
        Some(Token::Age) => "age".to_string(),
        Some(Token::Income) => "income".to_string(),
        _ => return Err(err_at(toks, *pos, "expected binding name")),
    };
    *pos += 1;
    Ok(key)
}

/// Reads a `GIVEN` binding value.
pub(crate) fn read_test_value(toks: &[SpannedToken], pos: &mut usize) -> DslResult<TestValue> {
    let value = match peek_tok(toks, *pos) {
        Some(Token::Number(n)) => TestValue::Number(*n as i64),
        Some(Token::Float(f)) => TestValue::String(f.to_string()),
        Some(Token::StringLit(s)) => TestValue::String(s.clone()),
        Some(Token::Ident(s)) => {
            if s.eq_ignore_ascii_case("true") {
                TestValue::Boolean(true)
            } else if s.eq_ignore_ascii_case("false") {
                TestValue::Boolean(false)
            } else {
                TestValue::String(s.clone())
            }
        }
        _ => return Err(err_at(toks, *pos, "expected a value after '='")),
    };
    *pos += 1;
    Ok(value)
}

/// Advances `pos` past a brace-delimited block (e.g. a `STATUTE` body) so the
/// contract parser can ignore constructs handled elsewhere.
pub(crate) fn skip_block(toks: &[SpannedToken], pos: &mut usize) {
    while *pos < toks.len() && !matches!(peek_tok(toks, *pos), Some(Token::LBrace)) {
        *pos += 1;
    }
    let mut depth = 0usize;
    while *pos < toks.len() {
        let token = &toks[*pos].token;
        *pos += 1;
        match token {
            Token::LBrace => depth += 1,
            Token::RBrace => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return;
                }
            }
            _ => {}
        }
    }
}

impl LegalDslParser {
    /// Parses a contract document: every `CONTRACT` block and `@test` case in
    /// the source. `STATUTE` blocks (and module declarations) are skipped — use
    /// [`LegalDslParser::parse_document`] for those.
    pub fn parse_contract_document(&self, input: &str) -> DslResult<ContractDocument> {
        let toks = self.tokenize(input)?;
        let mut pos = 0;
        let mut contracts = Vec::new();
        let mut test_cases = Vec::new();

        while pos < toks.len() {
            match &toks[pos].token {
                Token::Contract => contracts.push(self.parse_contract(&toks, &mut pos)?),
                // `@`-directives: a `ContractDocument` only retains `@test` cases;
                // `@mock`/`@property`/`@coverage`/`@snapshot` are parsed (so `pos`
                // advances) but belong to the richer `TestSpecDocument`.
                Token::At => {
                    if let crate::testspec_parser::Directive::Test(case) =
                        self.parse_directive(&toks, &mut pos)?
                    {
                        test_cases.push(*case);
                    }
                }
                Token::Statute | Token::Public | Token::Private => skip_block(&toks, &mut pos),
                _ => pos += 1,
            }
        }

        Ok(ContractDocument {
            contracts,
            test_cases,
        })
    }

    /// Parses just the `CONTRACT` blocks in the source.
    pub fn parse_contracts(&self, input: &str) -> DslResult<Vec<ContractNode>> {
        Ok(self.parse_contract_document(input)?.contracts)
    }

    /// Parses just the inline `@test` cases in the source.
    pub fn parse_test_cases(&self, input: &str) -> DslResult<Vec<TestCaseNode>> {
        Ok(self.parse_contract_document(input)?.test_cases)
    }

    /// Parses the statutes and inline `@test` cases from `input` and runs the
    /// cases against the statutes. This is the executable entry point for the
    /// `@test` syntax.
    pub fn run_embedded_tests(&self, input: &str) -> DslResult<TestRunReport> {
        let statutes = self.parse_statutes(input)?;
        let cases = self.parse_test_cases(input)?;
        Ok(run_test_cases(&statutes, &cases))
    }

    /// Parses a condition starting at `pos`, advancing `pos` past whatever the
    /// shared condition grammar consumed.
    fn parse_condition_at(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<Option<ConditionNode>> {
        let remaining = &toks[*pos..];
        let total = remaining.len();
        let mut iter = remaining.iter().map(|st| &st.token).peekable();
        let condition = self.parse_condition_node(&mut iter)?;
        let left = iter.count();
        *pos += total - left;
        Ok(condition)
    }

    /// Parses a `CONTRACT <id>: "<title>" { ... }` block.
    fn parse_contract(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<ContractNode> {
        *pos += 1; // consume CONTRACT
        let id = read_word(toks, pos, "contract identifier after CONTRACT")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after contract id",
        )?;
        let title = read_word(toks, pos, "contract title")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::LBrace),
            "'{' to open contract body",
        )?;

        let mut contract = ContractNode::new(id, title);

        loop {
            match peek_tok(toks, *pos) {
                Some(Token::RBrace) => {
                    *pos += 1;
                    break;
                }
                Some(Token::Party) => contract.parties.push(self.parse_party(toks, pos)?),
                Some(Token::Clause) => contract.clauses.push(self.parse_clause(toks, pos)?),
                Some(Token::Obligation) => {
                    contract.obligations.push(self.parse_obligation(toks, pos)?)
                }
                Some(Token::Right) => contract.rights.push(self.parse_right(toks, pos)?),
                Some(Token::Performance) => contract
                    .performances
                    .push(self.parse_performance(toks, pos)?),
                Some(Token::Compliance) => {
                    contract.compliance.push(self.parse_compliance(toks, pos)?)
                }
                Some(Token::Penalty) => contract.penalties.push(self.parse_penalty(toks, pos)?),
                Some(Token::Report) => contract.reports.push(self.parse_report(toks, pos)?),
                Some(Token::Inspect) => {
                    contract.inspections.push(self.parse_inspection(toks, pos)?)
                }
                Some(Token::Deadline) => contract.deadlines.push(self.parse_deadline(toks, pos)?),
                Some(Token::Timeline) => contract.timelines.push(self.parse_timeline(toks, pos)?),
                None => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "unterminated CONTRACT block (expected '}')",
                    ));
                }
                Some(_) => {
                    return Err(err_at(toks, *pos, "unexpected token in CONTRACT body"));
                }
            }
        }

        Ok(contract)
    }

    /// Parses `PARTY <id>: "<name>" [ROLE <role>]`.
    fn parse_party(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<PartyNode> {
        *pos += 1; // consume PARTY
        let id = read_word(toks, pos, "party identifier")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after party id",
        )?;
        let name = read_word(toks, pos, "party name")?;

        let role = if is_ident_kw(toks, *pos, "ROLE") {
            *pos += 1;
            let word = read_word(toks, pos, "role keyword after ROLE")?;
            Some(PartyRole::from_keyword(&word))
        } else {
            None
        };

        Ok(PartyNode { id, name, role })
    }

    /// Parses `CLAUSE <id> [FROM <template>]: "<text>"`.
    fn parse_clause(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<ClauseNode> {
        *pos += 1; // consume CLAUSE
        let id = read_word(toks, pos, "clause identifier")?;

        let from_template = if matches!(peek_tok(toks, *pos), Some(Token::From)) {
            *pos += 1;
            Some(read_word(toks, pos, "template identifier after FROM")?)
        } else {
            None
        };

        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after clause id",
        )?;
        let text = read_word(toks, pos, "clause text")?;

        Ok(ClauseNode {
            id,
            from_template,
            text,
        })
    }

    /// Parses `OBLIGATION <id> [BY <party>] [TO <party>]: "<desc>" [WHEN ..] [DUE ..]`.
    fn parse_obligation(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<ObligationNode> {
        *pos += 1; // consume OBLIGATION
        let id = read_word(toks, pos, "obligation identifier")?;

        let mut obligor = None;
        let mut obligee = None;
        loop {
            if is_ident_kw(toks, *pos, "BY") {
                *pos += 1;
                obligor = Some(read_word(toks, pos, "obligor party after BY")?);
            } else if is_ident_kw(toks, *pos, "TO") {
                *pos += 1;
                obligee = Some(read_word(toks, pos, "obligee party after TO")?);
            } else {
                break;
            }
        }

        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after obligation header",
        )?;
        let description = read_word(toks, pos, "obligation description")?;

        let mut conditions = Vec::new();
        let mut due = None;
        loop {
            if matches!(peek_tok(toks, *pos), Some(Token::When)) {
                *pos += 1;
                if let Some(cond) = self.parse_condition_at(toks, pos)? {
                    conditions.push(cond);
                }
            } else if is_ident_kw(toks, *pos, "DUE") {
                *pos += 1;
                due = Some(read_quoted_date(toks, pos, "obligation due date")?);
            } else {
                break;
            }
        }

        Ok(ObligationNode {
            id,
            description,
            obligor,
            obligee,
            conditions,
            due,
        })
    }

    /// Parses `RIGHT <id> [OF <party>] [<kind>]: "<desc>" [WHEN ..] [CORRELATIVE <oblig>]`.
    fn parse_right(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<RightNode> {
        *pos += 1; // consume RIGHT
        let id = read_word(toks, pos, "right identifier")?;

        let mut holder = None;
        let mut kind = None;
        loop {
            if is_ident_kw(toks, *pos, "OF") {
                *pos += 1;
                holder = Some(read_word(toks, pos, "holder party after OF")?);
            } else if let Some(Token::Ident(word)) = peek_tok(toks, *pos)
                && let Some(parsed) = RightKind::from_keyword(word)
            {
                kind = Some(parsed);
                *pos += 1;
            } else {
                break;
            }
        }

        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after right header",
        )?;
        let description = read_word(toks, pos, "right description")?;

        let mut conditions = Vec::new();
        let mut correlative_obligation = None;
        loop {
            if matches!(peek_tok(toks, *pos), Some(Token::When)) {
                *pos += 1;
                if let Some(cond) = self.parse_condition_at(toks, pos)? {
                    conditions.push(cond);
                }
            } else if is_ident_kw(toks, *pos, "CORRELATIVE") {
                *pos += 1;
                correlative_obligation =
                    Some(read_word(toks, pos, "obligation id after CORRELATIVE")?);
            } else {
                break;
            }
        }

        Ok(RightNode {
            id,
            description,
            holder,
            kind,
            conditions,
            correlative_obligation,
        })
    }

    /// Parses `PERFORMANCE <id> { [DESC ..] [WHEN ..]* [DUE ..] }`.
    fn parse_performance(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<PerformanceBlock> {
        *pos += 1; // consume PERFORMANCE
        let id = read_word(toks, pos, "performance identifier")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::LBrace),
            "'{' to open PERFORMANCE block",
        )?;

        let mut block = PerformanceBlock {
            id,
            ..PerformanceBlock::default()
        };

        loop {
            match peek_tok(toks, *pos) {
                Some(Token::RBrace) => {
                    *pos += 1;
                    break;
                }
                Some(Token::When) => {
                    *pos += 1;
                    if let Some(cond) = self.parse_condition_at(toks, pos)? {
                        block.conditions.push(cond);
                    }
                }
                _ if is_ident_kw(toks, *pos, "DESC") => {
                    *pos += 1;
                    block.description = Some(read_word(toks, pos, "performance description")?);
                }
                _ if is_ident_kw(toks, *pos, "DUE") => {
                    *pos += 1;
                    block.due = Some(read_quoted_date(toks, pos, "performance due date")?);
                }
                None => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "unterminated PERFORMANCE block (expected '}')",
                    ));
                }
                Some(_) => {
                    return Err(err_at(toks, *pos, "unexpected token in PERFORMANCE block"));
                }
            }
        }

        Ok(block)
    }

    /// Parses `COMPLIANCE <id>: "<desc>" [STANDARD "<std>"] [WHEN ..]`.
    fn parse_compliance(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<ComplianceRequirementNode> {
        *pos += 1; // consume COMPLIANCE
        let id = read_word(toks, pos, "compliance identifier")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after compliance id",
        )?;
        let description = read_word(toks, pos, "compliance description")?;

        let mut standard = None;
        let mut conditions = Vec::new();
        loop {
            if is_ident_kw(toks, *pos, "STANDARD") {
                *pos += 1;
                standard = Some(read_word(toks, pos, "standard after STANDARD")?);
            } else if matches!(peek_tok(toks, *pos), Some(Token::When)) {
                *pos += 1;
                if let Some(cond) = self.parse_condition_at(toks, pos)? {
                    conditions.push(cond);
                }
            } else {
                break;
            }
        }

        Ok(ComplianceRequirementNode {
            id,
            description,
            standard,
            conditions,
        })
    }

    /// Parses `PENALTY <id>: "<desc>" [AMOUNT n [cur]] [PER unit] [FOR oblig] [WHEN ..]`.
    fn parse_penalty(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<PenaltyNode> {
        *pos += 1; // consume PENALTY
        let id = read_word(toks, pos, "penalty identifier")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after penalty id",
        )?;
        let description = read_word(toks, pos, "penalty description")?;

        let mut amount = None;
        let mut currency = None;
        let mut per_unit = None;
        let mut for_obligation = None;
        let mut conditions = Vec::new();
        loop {
            if is_ident_kw(toks, *pos, "AMOUNT") {
                *pos += 1;
                amount = Some(read_number(toks, pos, "penalty amount after AMOUNT")?);
                // Optional currency: a plain word that is not the next modifier.
                match peek_tok(toks, *pos) {
                    Some(Token::StringLit(s)) => {
                        currency = Some(s.clone());
                        *pos += 1;
                    }
                    Some(Token::Ident(s))
                        if !s.eq_ignore_ascii_case("PER") && !s.eq_ignore_ascii_case("FOR") =>
                    {
                        currency = Some(s.clone());
                        *pos += 1;
                    }
                    _ => {}
                }
            } else if is_ident_kw(toks, *pos, "PER") {
                *pos += 1;
                per_unit = Some(read_word(toks, pos, "unit after PER")?);
            } else if is_ident_kw(toks, *pos, "FOR") {
                *pos += 1;
                for_obligation = Some(read_word(toks, pos, "obligation id after FOR")?);
            } else if matches!(peek_tok(toks, *pos), Some(Token::When)) {
                *pos += 1;
                if let Some(cond) = self.parse_condition_at(toks, pos)? {
                    conditions.push(cond);
                }
            } else {
                break;
            }
        }

        Ok(PenaltyNode {
            id,
            description,
            amount,
            currency,
            per_unit,
            for_obligation,
            conditions,
        })
    }

    /// Parses `REPORT <id>: "<desc>" [EVERY <freq>] [TO <recipient>] [DUE ..]`.
    fn parse_report(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<ReportNode> {
        *pos += 1; // consume REPORT
        let id = read_word(toks, pos, "report identifier")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after report id",
        )?;
        let description = read_word(toks, pos, "report description")?;

        let mut frequency = None;
        let mut recipient = None;
        let mut due = None;
        loop {
            if is_ident_kw(toks, *pos, "EVERY") {
                *pos += 1;
                frequency = Some(read_frequency(toks, pos)?);
            } else if is_ident_kw(toks, *pos, "TO") {
                *pos += 1;
                recipient = Some(read_word(toks, pos, "recipient after TO")?);
            } else if is_ident_kw(toks, *pos, "DUE") {
                *pos += 1;
                due = Some(read_quoted_date(toks, pos, "report due date")?);
            } else {
                break;
            }
        }

        Ok(ReportNode {
            id,
            description,
            frequency,
            recipient,
            due,
        })
    }

    /// Parses `INSPECT <id>: "<desc>" [BY <authority>] [EVERY <freq>] [WHEN ..]`.
    fn parse_inspection(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<InspectionNode> {
        *pos += 1; // consume INSPECT
        let id = read_word(toks, pos, "inspection identifier")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after inspection id",
        )?;
        let description = read_word(toks, pos, "inspection description")?;

        let mut authority = None;
        let mut frequency = None;
        let mut conditions = Vec::new();
        loop {
            if is_ident_kw(toks, *pos, "BY") {
                *pos += 1;
                authority = Some(read_word(toks, pos, "authority after BY")?);
            } else if is_ident_kw(toks, *pos, "EVERY") {
                *pos += 1;
                frequency = Some(read_frequency(toks, pos)?);
            } else if matches!(peek_tok(toks, *pos), Some(Token::When)) {
                *pos += 1;
                if let Some(cond) = self.parse_condition_at(toks, pos)? {
                    conditions.push(cond);
                }
            } else {
                break;
            }
        }

        Ok(InspectionNode {
            id,
            description,
            authority,
            frequency,
            conditions,
        })
    }

    /// Parses `DEADLINE <id>: "<date>" ["<desc>"]`.
    fn parse_deadline(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<DeadlineNode> {
        *pos += 1; // consume DEADLINE
        let id = read_word(toks, pos, "deadline identifier")?;
        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::Colon),
            "':' after deadline id",
        )?;
        let date = read_quoted_date(toks, pos, "deadline date")?;

        let description = match peek_tok(toks, *pos) {
            Some(Token::StringLit(s)) => {
                let value = s.clone();
                *pos += 1;
                Some(value)
            }
            _ => None,
        };

        Ok(DeadlineNode {
            id,
            date,
            description,
        })
    }

    /// Parses `TIMELINE <id>[: "<desc>"] { DEADLINE .. }`.
    fn parse_timeline(&self, toks: &[SpannedToken], pos: &mut usize) -> DslResult<TimelineNode> {
        *pos += 1; // consume TIMELINE
        let id = read_word(toks, pos, "timeline identifier")?;

        let description = if matches!(peek_tok(toks, *pos), Some(Token::Colon)) {
            *pos += 1;
            Some(read_word(toks, pos, "timeline description")?)
        } else {
            None
        };

        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::LBrace),
            "'{' to open TIMELINE block",
        )?;

        let mut deadlines = Vec::new();
        loop {
            match peek_tok(toks, *pos) {
                Some(Token::RBrace) => {
                    *pos += 1;
                    break;
                }
                Some(Token::Deadline) => deadlines.push(self.parse_deadline(toks, pos)?),
                None => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "unterminated TIMELINE block (expected '}')",
                    ));
                }
                Some(_) => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "expected DEADLINE or '}' in TIMELINE block",
                    ));
                }
            }
        }

        Ok(TimelineNode {
            id,
            description,
            deadlines,
        })
    }

    /// Parses `@test "<name>" FOR <statute> { [USING ..] [GIVEN ..] EXPECT .. }`.
    ///
    /// `pub(crate)` so the [`crate::testspec`] directive dispatcher can collect
    /// `@test` cases alongside `@mock`/`@property`/`@coverage`/`@snapshot`.
    pub(crate) fn parse_test_case(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<TestCaseNode> {
        *pos += 1; // consume '@'
        if !is_ident_kw(toks, *pos, "test") {
            return Err(err_at(toks, *pos, "expected 'test' after '@'"));
        }
        *pos += 1; // consume 'test'

        let name = read_word(toks, pos, "test case name")?;

        if !is_ident_kw(toks, *pos, "FOR") {
            return Err(err_at(toks, *pos, "expected FOR after the test name"));
        }
        *pos += 1; // consume FOR
        let target_statute = read_word(toks, pos, "statute id after FOR")?;

        expect_simple(
            toks,
            pos,
            |t| matches!(t, Token::LBrace),
            "'{' to open @test block",
        )?;

        let mut uses = Vec::new();
        let mut bindings = Vec::new();
        let mut expectation = None;
        loop {
            match peek_tok(toks, *pos) {
                Some(Token::RBrace) => {
                    *pos += 1;
                    break;
                }
                _ if is_ident_kw(toks, *pos, "USING") => {
                    *pos += 1;
                    self.parse_mock_refs(toks, pos, &mut uses)?;
                }
                _ if is_ident_kw(toks, *pos, "GIVEN") => {
                    *pos += 1;
                    self.parse_test_bindings(toks, pos, &mut bindings)?;
                }
                _ if is_ident_kw(toks, *pos, "EXPECT") => {
                    *pos += 1;
                    expectation = Some(self.parse_expectation(toks, pos)?);
                }
                None => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "unterminated @test block (expected '}')",
                    ));
                }
                Some(_) => {
                    return Err(err_at(
                        toks,
                        *pos,
                        "expected USING, GIVEN, EXPECT or '}' in @test block",
                    ));
                }
            }
        }

        let expectation = expectation
            .ok_or_else(|| err_at(toks, *pos, "@test block is missing an EXPECT clause"))?;

        Ok(TestCaseNode {
            name,
            target_statute,
            uses,
            bindings,
            expectation,
        })
    }

    /// Reads one or more comma-separated mock identifiers after `USING`.
    pub(crate) fn parse_mock_refs(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
        uses: &mut Vec<String>,
    ) -> DslResult<()> {
        loop {
            uses.push(read_word(toks, pos, "mock entity id after USING")?);
            if matches!(peek_tok(toks, *pos), Some(Token::Comma)) {
                *pos += 1;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Reads one or more comma-separated `key = value` bindings after `GIVEN`.
    pub(crate) fn parse_test_bindings(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
        bindings: &mut Vec<TestBinding>,
    ) -> DslResult<()> {
        loop {
            let key = read_binding_key(toks, pos)?;
            expect_simple(
                toks,
                pos,
                |t| {
                    matches!(t, Token::Operator(op) if op == "=" || op == "==")
                        || matches!(t, Token::Colon)
                },
                "'=' in binding",
            )?;
            let value = read_test_value(toks, pos)?;
            bindings.push(TestBinding { key, value });

            if matches!(peek_tok(toks, *pos), Some(Token::Comma)) {
                *pos += 1;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Parses the value of an `EXPECT` clause.
    pub(crate) fn parse_expectation(
        &self,
        toks: &[SpannedToken],
        pos: &mut usize,
    ) -> DslResult<TestExpectation> {
        match peek_tok(toks, *pos) {
            Some(Token::Not) => {
                *pos += 1;
                if !is_ident_kw(toks, *pos, "SATISFIED") {
                    return Err(err_at(toks, *pos, "expected SATISFIED after NOT"));
                }
                *pos += 1;
                Ok(TestExpectation::Unsatisfied)
            }
            Some(Token::Grant) => {
                *pos += 1;
                Ok(TestExpectation::Effect(ExpectedEffect::Grant))
            }
            Some(Token::Revoke) => {
                *pos += 1;
                Ok(TestExpectation::Effect(ExpectedEffect::Revoke))
            }
            Some(Token::Obligation) => {
                *pos += 1;
                Ok(TestExpectation::Effect(ExpectedEffect::Obligation))
            }
            Some(Token::Prohibition) => {
                *pos += 1;
                Ok(TestExpectation::Effect(ExpectedEffect::Prohibition))
            }
            Some(Token::Ident(word)) => {
                if word.eq_ignore_ascii_case("SATISFIED") {
                    *pos += 1;
                    Ok(TestExpectation::Satisfied)
                } else if word.eq_ignore_ascii_case("UNSATISFIED") {
                    *pos += 1;
                    Ok(TestExpectation::Unsatisfied)
                } else if let Some(effect) = ExpectedEffect::from_keyword(word) {
                    *pos += 1;
                    Ok(TestExpectation::Effect(effect))
                } else {
                    Err(err_at(
                        toks,
                        *pos,
                        "expected SATISFIED, NOT SATISFIED, or an effect kind after EXPECT",
                    ))
                }
            }
            _ => Err(err_at(
                toks,
                *pos,
                "expected SATISFIED, NOT SATISFIED, or an effect kind after EXPECT",
            )),
        }
    }
}
