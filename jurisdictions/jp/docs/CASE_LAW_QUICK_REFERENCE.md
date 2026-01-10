# 判例データベース クイックリファレンス

## 5分で始める判例検索

### 1. データベースの作成と判例追加 (30秒)

```rust
use legalis_jp::case_law::*;
use chrono::NaiveDate;

// データベース作成
let mut db = InMemoryCaseDatabase::new();

// 判例追加
let metadata = CaseMetadata::new(
    "令和2年(受)第1234号",                        // 事件番号
    NaiveDate::from_ymd_opt(2020, 7, 15).unwrap(), // 判決日
    Court::new(CourtLevel::Supreme),                // 最高裁判所
    LegalArea::Civil,                               // 民事
    CaseOutcome::PlaintiffWins,                     // 原告勝訴
);

let decision = CourtDecision::new(
    "case-001",
    metadata,
    "使用者は従業員の不法行為について責任を負う",
);

db.add_case(decision)?;
```

### 2. 判例検索 (1分)

```rust
// 検索エンジン作成
let engine = CaseLawSearchEngine::new(db);

// 検索実行
let query = CaseSearchQuery::new()
    .with_keyword("不法行為")
    .with_court_level(CourtLevel::Supreme)
    .with_limit(10);

let results = engine.search(&query)?;

// 結果表示
for result in results {
    println!("{}: {:.1}点",
        result.decision.metadata.case_number,
        result.relevance_score
    );
}
```

### 3. 引用形式の生成 (30秒)

```rust
// 標準形式
let citation = CitationFormatter::format(&decision, CitationStyle::Standard)?;
// => "最高裁判所令和2年(受)第1234号判決（令和2年7月15日）"

// 短縮形式
let short = CitationFormatter::format(&decision, CitationStyle::Short)?;
// => "最高裁令2・7・15判決"
```

---

## よく使うコードスニペット

### 判例データの完全な作成

```rust
let holding = Holding {
    issue: "論点".to_string(),
    reasoning: "理由".to_string(),
    conclusion: "結論".to_string(),
};

let parties = Parties {
    plaintiff: "原告名".to_string(),
    defendant: "被告名".to_string(),
};

let statute = StatuteCitation {
    statute_name: "民法".to_string(),
    article: "第709条".to_string(),
    paragraph: None,
};

let decision = CourtDecision::new("case-id", metadata, "要旨")
    .with_holding(holding)
    .with_parties(parties)
    .with_statute_citation(statute);
```

### 複数キーワード検索

```rust
let query = CaseSearchQuery::new()
    .with_keyword("使用者責任")
    .with_keyword("損害賠償")
    .with_legal_area(LegalArea::Civil);
```

### 関連性スコアでフィルタリング

```rust
let results = engine.search(&query)?;
let high_relevance: Vec<_> = results
    .into_iter()
    .filter(|r| r.relevance_score >= 70.0)
    .collect();
```

### 判例の詳細表示

```rust
let case = db.get_case("case-id")?;

println!("事件番号: {}", case.metadata.case_number);
println!("判決日: {}", case.metadata.decision_date);
println!("裁判所: {:?}", case.metadata.court.level);
println!("要旨: {}", case.summary);

for (i, holding) in case.holdings.iter().enumerate() {
    println!("{}. {}", i + 1, holding.issue);
    println!("   {}", holding.conclusion);
}
```

---

## チートシート

### 裁判所レベル

| コード | 日本語 | 英語 |
|--------|--------|------|
| `CourtLevel::Supreme` | 最高裁判所 | Supreme Court |
| `CourtLevel::High` | 高等裁判所 | High Court |
| `CourtLevel::District` | 地方裁判所 | District Court |
| `CourtLevel::Family` | 家庭裁判所 | Family Court |
| `CourtLevel::Summary` | 簡易裁判所 | Summary Court |

### 法分野

| コード | 日本語 | 英語 |
|--------|--------|------|
| `LegalArea::Civil` | 民事 | Civil |
| `LegalArea::Criminal` | 刑事 | Criminal |
| `LegalArea::Constitutional` | 憲法 | Constitutional |
| `LegalArea::Administrative` | 行政 | Administrative |
| `LegalArea::Family` | 家事 | Family |
| `LegalArea::Labor` | 労働 | Labor |
| `LegalArea::Intellectual` | 知的財産 | Intellectual Property |
| `LegalArea::Tax` | 税務 | Tax |

### 判決結果

| コード | 意味 |
|--------|------|
| `CaseOutcome::PlaintiffWins` | 原告勝訴 |
| `CaseOutcome::DefendantWins` | 被告勝訴 |
| `CaseOutcome::PartialVictory` | 一部勝訴 |
| `CaseOutcome::Settled` | 和解 |
| `CaseOutcome::Dismissed` | 却下 |

### 引用スタイル

| スタイル | 用途 | 例 |
|---------|------|-----|
| `CitationStyle::Standard` | 法律文書 | 最高裁判所令和2年(受)第1234号判決（令和2年7月15日） |
| `CitationStyle::Short` | 脚注 | 最高裁令2・7・15判決 |
| `CitationStyle::Full` | 詳細分析 | 最高裁判所令和2年(受)第1234号判決（令和2年7月15日）「要旨」 |
| `CitationStyle::BlueBook` | 英語論文 | Supreme Court of Japan, Case No. 令和2年(受)第1234号 (July 15, 2020) |

---

## エラーハンドリング

### よくあるエラーと対処法

```rust
match db.get_case(case_id) {
    Ok(case) => { /* 処理 */ },
    Err(CaseLawError::CaseNotFound(id)) => {
        eprintln!("判例が見つかりません: {}", id);
    },
    Err(e) => {
        eprintln!("エラー: {}", e);
    },
}
```

### 安全な検索

```rust
fn safe_search(keyword: &str) -> Vec<SearchResult> {
    let engine = CaseLawSearchEngine::new(db);
    let query = CaseSearchQuery::new().with_keyword(keyword);

    engine.search(&query).unwrap_or_else(|e| {
        eprintln!("検索エラー: {}", e);
        Vec::new()
    })
}
```

---

## パフォーマンスTips

### 1. 検索結果を制限

```rust
// 悪い例: 全件取得
let query = CaseSearchQuery::new().with_keyword("不法行為");

// 良い例: 必要な件数のみ
let query = CaseSearchQuery::new()
    .with_keyword("不法行為")
    .with_limit(10);
```

### 2. フィルタを最初に適用

```rust
// 悪い例: 検索後にフィルタリング
let results = engine.search(&query)?;
let filtered: Vec<_> = results
    .into_iter()
    .filter(|r| r.decision.metadata.court.level == CourtLevel::Supreme)
    .collect();

// 良い例: クエリでフィルタリング
let query = CaseSearchQuery::new()
    .with_keyword("不法行為")
    .with_court_level(CourtLevel::Supreme);
let results = engine.search(&query)?;
```

### 3. 早期リターン

```rust
// 最初の1件で十分な場合
let query = CaseSearchQuery::new()
    .with_keyword("不法行為")
    .with_limit(1);

let results = engine.search(&query)?;
if let Some(first) = results.first() {
    // 処理
}
```

---

## 実践テンプレート

### テンプレート1: 判例検索アプリ

```rust
fn main() -> Result<(), CaseLawError> {
    // 初期化
    let mut db = InMemoryCaseDatabase::new();
    load_cases(&mut db)?;
    let engine = CaseLawSearchEngine::new(db);

    // ユーザー入力
    println!("検索キーワード: ");
    let mut keyword = String::new();
    std::io::stdin().read_line(&mut keyword)?;

    // 検索
    let query = CaseSearchQuery::new()
        .with_keyword(keyword.trim())
        .with_limit(10);

    let results = engine.search(&query)?;

    // 表示
    for (i, result) in results.iter().enumerate() {
        println!("{}. {}", i + 1, result.decision.metadata.case_number);
        println!("   {}", result.decision.summary);
        println!("   関連性: {:.1}%", result.relevance_score);
        println!();
    }

    Ok(())
}
```

### テンプレート2: 判例データベースビルダー

```rust
fn build_database() -> Result<InMemoryCaseDatabase, CaseLawError> {
    let mut db = InMemoryCaseDatabase::new();

    // JSON/CSVからロード
    let cases_data = load_cases_from_json("cases.json")?;

    for case_data in cases_data {
        let metadata = CaseMetadata::new(
            case_data.case_number,
            case_data.decision_date,
            case_data.court,
            case_data.legal_area,
            case_data.outcome,
        );

        let decision = CourtDecision::new(
            &case_data.id,
            metadata,
            &case_data.summary,
        );

        db.add_case(decision)?;
    }

    Ok(db)
}
```

### テンプレート3: 判例分析レポート生成

```rust
fn generate_case_report(case_id: &str, db: &InMemoryCaseDatabase) -> String {
    let case = db.get_case(case_id).expect("Case not found");

    let mut report = String::new();

    report.push_str(&format!("# 判例分析レポート\n\n"));
    report.push_str(&format!("## 基本情報\n"));
    report.push_str(&format!("- 事件番号: {}\n", case.metadata.case_number));
    report.push_str(&format!("- 判決日: {}\n", case.metadata.decision_date));
    report.push_str(&format!("- 裁判所: {:?}\n\n", case.metadata.court.level));

    report.push_str(&format!("## 判決要旨\n{}\n\n", case.summary));

    report.push_str(&format!("## 判示事項\n"));
    for (i, holding) in case.holdings.iter().enumerate() {
        report.push_str(&format!("### {}. {}\n", i + 1, holding.issue));
        report.push_str(&format!("**理由**: {}\n\n", holding.reasoning));
        report.push_str(&format!("**結論**: {}\n\n", holding.conclusion));
    }

    report.push_str(&format!("## 引用\n"));
    let citation = CitationFormatter::format(&case, CitationStyle::Standard)
        .unwrap_or_default();
    report.push_str(&format!("{}\n", citation));

    report
}
```

---

## トラブルシューティング早見表

| 問題 | 原因 | 解決策 |
|------|------|--------|
| 検索結果0件 | キーワードが厳密すぎる | キーワードを一般化、フィルタを削除 |
| 関連性スコアが低い | キーワードミスマッチ | 複数のキーワードを試す |
| メモリ使用量が多い | 判例数が多すぎる | 必要な判例のみロード |
| 検索が遅い | フルスキャン | フィルタをクエリで指定 |
| 判例が重複 | 同じIDで複数回追加 | ID重複チェック |

---

## コマンド集

### テスト実行

```bash
# 判例モジュールのテストのみ
cargo nextest run --package legalis-jp --lib case_law

# デモ実行
cargo run --example case-law-search-demo
```

### ドキュメント生成

```bash
# 判例モジュールのドキュメント
cargo doc --package legalis-jp --open
```

---

## リンク

- **完全ガイド**: `/tmp/CASE_LAW_DATABASE_GUIDE.md`
- **デモ**: `/mnt/fd/legalis/jurisdictions/jp/examples/case-law-search-demo.rs`
- **ソースコード**: `/mnt/fd/legalis/jurisdictions/jp/src/case_law/`

---

**最終更新**: 2025-01-09
**バージョン**: 0.1.1
