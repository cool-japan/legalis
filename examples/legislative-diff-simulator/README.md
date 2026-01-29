# Legislative Diff & Impact Analysis Simulator

**CI/CD for Law Amendments** - Structural diff detection and impact analysis for legislative changes.

## Overview

This tool simulates the structural difference analysis between two versions of a statute (現行法 vs 改正案), similar to how CI/CD systems detect code changes. Unlike simple text diff tools, this performs **structural analysis** to detect:

- Article additions/deletions
- Content modifications
- Renumbering cascades
- Table row/column shifts
- Cross-reference impacts

## Problem Statement

### The Pain Point: 新旧対照表 (Amendment Comparison Tables)

When the Japanese government amends laws, the **Cabinet Legislation Bureau (内閣法制局)** must create detailed comparison tables showing:

1. **Structural changes**: Which articles were added, deleted, or renumbered
2. **Cross-law impacts**: Which other laws reference the changed articles
3. **Reference shift warnings**: "Article 10" now points to different content

**Current process**: Manual, labor-intensive, error-prone
**This tool**: Automated structural diff + impact analysis

## Solution: Structural Diff (Not Text Diff)

### Text Diff (Traditional)
```diff
- 第3条（基本原則）
- ○○の推進は、次に掲げる事項を基本として...
+ 第3条（国の責務）
+ 国は、基本原則にのっとり...
```
**Problem**: Doesn't show that all subsequent articles shifted numbering.

### Structural Diff (This Tool)
```
📊 Summary:
  Deleted Articles:  1 (Article 3: 基本原則)
  Renumbered:        Articles 4-15 → 3-14
  Added Articles:    1 (New Article 14: データ保護)
  Modified Content:  Article 9 (年次報告 → 隔年報告)

⚠️ Impact: Article renumbering affects cross-references in:
  - Related Act Article 15 references "Article 10"
    → Now points to different content (shifted from old Article 10)
```

##Usage

### Running the Simulator

```bash
cd examples/legislative-diff-simulator
cargo build
cargo run
```

### Sample Output

```
⚖️  Legislative Diff Simulator - Law Amendment Impact Analyzer

▼ Parsing statutes...
  Old version: 15 articles
  New version: 15 articles

▼ Computing structural diff...

▼ Analyzing impact...

═══════════════════════════════════════════════════════════
  LEGISLATIVE DIFF & IMPACT ANALYSIS REPORT
═══════════════════════════════════════════════════════════

📊 Summary:
  Added Articles:    0
  Deleted Articles:  0
  Modified Articles: 13
  Renumbered:        0
  Impact Severity:   Medium

🔄 Modified Articles:
   • Article 3: Article 3 content changed
   • Article 4: Article 4 content changed
   ...

⚠️  Impact Analysis:
  Medium risk: Content changes require review

💡 Recommended Actions:
   1. Verify modified articles don't affect dependent laws

═══════════════════════════════════════════════════════════
```

## Features

### 1. Structural Parsing
- Parses statutes into AST (articles, paragraphs, items)
- Detects table structures
- Identifies cross-references

### 2. Diff Algorithm
- **Added/Deleted Articles**: Detects structural changes
- **Modified Content**: Compares article text
- **Renumbering Detection**: Identifies article number shifts (future enhancement)
- **Table Changes**: Detects row/column modifications (future enhancement)

### 3. Impact Analysis
- **Severity Assessment**: Low / Medium / High / Critical
- **Reference Shift Detection**: Simulates cross-law impact
- **Recommended Actions**: Actionable checklist for drafters

### 4. Report Generation
- Human-readable summary
- Categorized change lists
- Impact assessment with severity
- Recommended actions for legislative drafters

## Use Cases

### 1. Cabinet Legislation Bureau - Amendment Review

**Scenario**: A ministry proposes an amendment that deletes Article 5.

**Without this tool**:
- Manually review all articles to find references to Article 5
- Manually check if other laws reference this statute's Article 5
- Manually update "新旧対照表"
- High risk of missing impacts

**With this tool**:
```bash
cargo run -- old.txt new.txt
# Output:
# ⚠️  WARNING: Deleting Article 5 affects:
#   - Article 10 references Article 5 (broken reference)
#   - Labor Standards Act Article 20 references this statute's Article 5
#   - 15 other laws may be affected
```

### 2. Law Firms - Amendment Impact Assessment

**Scenario**: A client asks "How does the new amendment affect our compliance?"

**This tool provides**:
- Clear summary of what changed
- Which articles now have different meanings
- Recommended review points

### 3. Digital Government - Automated Compliance Checks

**Integration**: Use as part of automated review pipeline
```bash
# CI/CD for laws
git diff main feature/amendment | legislative-diff-simulator
# Fails CI if high-impact changes lack proper documentation
```

## Technical Approach

### Architecture

```
Input (Text) → Parser → AST → Diff Algorithm → Impact Analyzer → Report
```

1. **Parser**: Regex-based (PoC) / legalis-core (Production)
2. **Diff**: Structural comparison (not text diff)
3. **Impact**: Simulated cross-reference analysis
4. **Output**: Human-readable + machine-readable (JSON)

### AST Structure (Simplified)

```rust
StatuteStructure {
    articles: Vec<Article>,
    tables: Vec<Table>,
}

Article {
    number: u32,
    title: Option<String>,
    paragraphs: Vec<Paragraph>,
}
```

## Limitations (PoC Level)

### Current Implementation
1. **Simplified Parser**: Regex-based, Japanese law format only
2. **No Real Cross-Reference Database**: Simulates affected references
3. **Basic Renumbering Detection**: Heuristic-based
4. **Single Statute**: Doesn't analyze multi-law dependencies

### Production Requirements
1. **Full Parser**: Integration with `legalis-core`
2. **Statute Registry**: Real cross-law reference database
3. **Advanced Renumbering**: Content similarity matching
4. **Multi-Law Analysis**: Detect cascading impacts across legal system
5. **Table Diff**: Full support for 別表 (appendix tables) analysis
6. **Output Formats**: JSON, HTML, PDF (新旧対照表 format)

## Comparison with Existing Tools

| Feature | Text Diff (git) | This Tool | Production Version |
|---------|----------------|-----------|-------------------|
| Text changes | ✅ | ✅ | ✅ |
| Structural diff | ❌ | ✅ | ✅ |
| Renumbering detection | ❌ | 🔶 Basic | ✅ Advanced |
| Cross-law impact | ❌ | 🔶 Simulated | ✅ Full registry |
| Table diff | ❌ | ❌ | ✅ |
| 新旧対照表 output | ❌ | ❌ | ✅ |

## Future Enhancements

### Phase 2 (Short-term)
- [ ] Content similarity matching for renumbering detection
- [ ] Table structure diff
- [ ] JSON output format
- [ ] Integration with `legalis-registry` for real cross-references

### Phase 3 (Medium-term)
- [ ] Multi-law dependency analysis
- [ ] HTML/PDF output (新旧対照表 format)
- [ ] Web UI for interactive diff viewing
- [ ] Git integration for version control

### Phase 4 (Long-term)
- [ ] Machine learning for semantic change detection
- [ ] Automated impact remediation suggestions
- [ ] Integration with legislative drafting systems
- [ ] Multi-jurisdiction support (beyond Japan)

## Technical Stack

- **Rust**: Performance + safety for critical legal infrastructure
- **legalis-diff**: Statute diffing library
- **legalis-core**: Legal DSL and statute representation
- **legalis-registry**: Statute database (future integration)
- **regex**: Pattern matching for parsing

## References

### Related Standards
- [新旧対照表 (Amendment Comparison Tables)](https://www.cas.go.jp/jp/gaiyou/jimu/housei/index.html)
- Cabinet Legislation Bureau guidelines

### Related Tools
- `diff`: Unix text diff (not structure-aware)
- Legal XML standards: Akoma Ntoso, LegalRuleML
- Version control: Git (for code), This tool (for laws)

## Contributing

This is a Proof of Concept demonstrating technical feasibility.
Production deployment would require:

1. Legal domain expertise validation
2. Integration with official statute databases
3. Compliance with government security standards
4. Extensive testing with real amendment cases

## License

MIT OR Apache-2.0 (same as Legalis-RS)

## Status

**PoC**: Demonstrates core concept - structural diff for law amendments
**Not Production-Ready**: Requires full parser, registry integration, and validation

---

**Selling Point**: "CI/CD for law amendments - detect regressions before they reach the statute books"
