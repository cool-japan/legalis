# 判例データベース使い方ガイド (Case Law Database Usage Guide)

## 目次

1. [概要](#概要)
2. [日本の裁判所システム](#日本の裁判所システム)
3. [主要コンポーネント](#主要コンポーネント)
4. [基本的な使い方](#基本的な使い方)
5. [検索機能](#検索機能)
6. [引用形式](#引用形式)
7. [実用例](#実用例)
8. [他モジュールとの統合](#他モジュールとの統合)

---

## 概要

判例データベースシステムは、日本の裁判所判例を効率的に管理・検索するための機能を提供します。

### 主な機能

- **判例の保存・管理**: メタデータ、判決要旨、当事者情報を含む完全な判例データ
- **高度な検索**: キーワード検索、裁判所レベルフィルタ、法分野フィルタ
- **関連性スコアリング**: 検索結果の関連性を自動計算
- **引用形式**: 標準形式、短縮形式、完全形式、ブルーブック形式に対応
- **データベース抽象化**: メモリ内実装とデータベースバックエンドの切り替えが可能

### アーキテクチャ

```
CaseLawSearchEngine
    ↓
CaseLawDatabase (trait)
    ↓
InMemoryCaseDatabase (実装)
```

---

## 日本の裁判所システム

判例データベースは日本の裁判所階層構造を反映しています。

### 裁判所レベル (CourtLevel)

```rust
pub enum CourtLevel {
    Supreme,      // 最高裁判所
    High,         // 高等裁判所
    District,     // 地方裁判所
    Family,       // 家庭裁判所
    Summary,      // 簡易裁判所
}
```

### 主要裁判所

| レベル | 名称 | 役割 |
|--------|------|------|
| Supreme | 最高裁判所 | 最終審、憲法判断 |
| High | 高等裁判所 | 控訴審 (全国8箇所) |
| District | 地方裁判所 | 第一審 (民事・刑事) |
| Family | 家庭裁判所 | 家事事件・少年事件 |
| Summary | 簡易裁判所 | 少額訴訟 (140万円以下) |

### 法分野 (LegalArea)

```rust
pub enum LegalArea {
    Civil,           // 民事
    Criminal,        // 刑事
    Constitutional,  // 憲法
    Administrative,  // 行政
    Family,          // 家事
    Labor,           // 労働
    Intellectual,    // 知的財産
    Tax,             // 税務
    Other(String),   // その他
}
```

---

## 主要コンポーネント

### 1. CourtDecision (判例データ)

判例を表す中心的なデータ構造。

```rust
pub struct CourtDecision {
    pub id: String,
    pub metadata: CaseMetadata,
    pub summary: String,
    pub holdings: Vec<Holding>,
    pub parties: Parties,
    pub cited_statutes: Vec<StatuteCitation>,
}
```

**フィールド説明**:
- `id`: 判例の一意識別子 (例: "minpo-article709-001")
- `metadata`: 事件番号、判決日、裁判所、法分野、結果
- `summary`: 判決の要旨
- `holdings`: 判示事項のリスト (複数の論点を含む)
- `parties`: 当事者情報 (原告・被告)
- `cited_statutes`: 引用された法令のリスト

### 2. CaseMetadata (メタデータ)

判例の基本情報。

```rust
pub struct CaseMetadata {
    pub case_number: String,      // 事件番号: "令和2年(受)第1234号"
    pub decision_date: NaiveDate, // 判決日
    pub court: Court,             // 裁判所
    pub legal_area: LegalArea,    // 法分野
    pub outcome: CaseOutcome,     // 判決結果
}
```

**事件番号の形式**:
- 令和2年(受)第1234号: 最高裁判所上告事件
- 令和3年(ネ)第5678号: 高等裁判所控訴事件
- 令和4年(ワ)第9012号: 地方裁判所民事第一審事件

### 3. Holding (判示事項)

判例の論点と判断。

```rust
pub struct Holding {
    pub issue: String,        // 論点
    pub reasoning: String,    // 理由
    pub conclusion: String,   // 結論
}
```

**例**:
```rust
Holding {
    issue: "使用者責任が成立するか".to_string(),
    reasoning: "従業員が業務中に第三者に損害を与えた場合、使用者は選任監督上の注意義務を尽くしたことを立証しない限り責任を負う".to_string(),
    conclusion: "民法第715条により使用者責任が成立する".to_string(),
}
```

### 4. InMemoryCaseDatabase

メモリ内判例データベース実装。

```rust
pub struct InMemoryCaseDatabase {
    cases: HashMap<String, CourtDecision>,
}

impl CaseLawDatabase for InMemoryCaseDatabase {
    fn add_case(&mut self, decision: CourtDecision) -> Result<(), CaseLawError>;
    fn get_case(&self, id: &str) -> Result<CourtDecision, CaseLawError>;
    fn search(&self, query: &CaseSearchQuery) -> Result<Vec<CourtDecision>, CaseLawError>;
}
```

### 5. CaseLawSearchEngine

検索エンジンと関連性スコアリング。

```rust
pub struct CaseLawSearchEngine<D: CaseLawDatabase> {
    database: D,
}

impl<D: CaseLawDatabase> CaseLawSearchEngine<D> {
    pub fn search(&self, query: &CaseSearchQuery) -> Result<Vec<SearchResult>, CaseLawError>;
}

pub struct SearchResult {
    pub decision: CourtDecision,
    pub relevance_score: f64,  // 0.0-100.0
}
```

---

## 基本的な使い方

### ステップ1: データベースの作成

```rust
use legalis_jp::case_law::{InMemoryCaseDatabase, CaseLawDatabase};

// 空のデータベースを作成
let mut database = InMemoryCaseDatabase::new();
```

### ステップ2: 判例の追加

```rust
use legalis_jp::case_law::{
    CourtDecision, CaseMetadata, Court, CourtLevel,
    LegalArea, CaseOutcome, Holding, Parties, StatuteCitation
};
use chrono::NaiveDate;

// メタデータの作成
let metadata = CaseMetadata::new(
    "令和2年(受)第1234号",                        // 事件番号
    NaiveDate::from_ymd_opt(2020, 7, 15).unwrap(), // 判決日
    Court::new(CourtLevel::Supreme),                // 最高裁判所
    LegalArea::Civil,                               // 民事事件
    CaseOutcome::PlaintiffWins,                     // 原告勝訴
);

// 判示事項の作成
let holding = Holding {
    issue: "使用者責任の成立要件".to_string(),
    reasoning: "従業員の不法行為が業務執行に関連する場合、使用者は民法第715条により責任を負う".to_string(),
    conclusion: "使用者責任が成立し、被告は損害賠償義務を負う".to_string(),
};

// 当事者情報
let parties = Parties {
    plaintiff: "山田太郎".to_string(),
    defendant: "株式会社ABC".to_string(),
};

// 引用法令
let statute = StatuteCitation {
    statute_name: "民法".to_string(),
    article: "第715条".to_string(),
    paragraph: Some("第1項".to_string()),
};

// 判例の作成
let decision = CourtDecision::new(
    "case-minpo715-001",           // ID
    metadata,
    "使用者は従業員の不法行為について責任を負う", // 要旨
)
.with_holding(holding)
.with_parties(parties)
.with_statute_citation(statute);

// データベースに追加
database.add_case(decision)?;
```

### ステップ3: 判例の取得

```rust
// IDで判例を取得
let case = database.get_case("case-minpo715-001")?;

println!("事件番号: {}", case.metadata.case_number);
println!("判決日: {}", case.metadata.decision_date);
println!("要旨: {}", case.summary);
```

---

## 検索機能

### CaseSearchQuery ビルダー

```rust
use legalis_jp::case_law::{CaseSearchQuery, CourtLevel, LegalArea};

let query = CaseSearchQuery::new()
    .with_keyword("不法行為")                    // キーワード
    .with_court_level(CourtLevel::Supreme)       // 裁判所レベル
    .with_legal_area(LegalArea::Civil)           // 法分野
    .with_limit(10);                             // 結果数上限
```

### 検索の実行

```rust
use legalis_jp::case_law::CaseLawSearchEngine;

let engine = CaseLawSearchEngine::new(database);
let results = engine.search(&query)?;

// 結果の処理
for result in results {
    println!("判例ID: {}", result.decision.id);
    println!("関連性スコア: {:.2}", result.relevance_score);
    println!("事件番号: {}", result.decision.metadata.case_number);
    println!("要旨: {}", result.decision.summary);
    println!("---");
}
```

### 検索フィルタの組み合わせ

#### 例1: 最高裁の民事判例のみ検索

```rust
let query = CaseSearchQuery::new()
    .with_keyword("損害賠償")
    .with_court_level(CourtLevel::Supreme)
    .with_legal_area(LegalArea::Civil);
```

#### 例2: 労働事件で「解雇」を含む判例

```rust
let query = CaseSearchQuery::new()
    .with_keyword("解雇")
    .with_legal_area(LegalArea::Labor);
```

#### 例3: 特定の裁判所の全判例

```rust
let query = CaseSearchQuery::new()
    .with_court_level(CourtLevel::District)
    .with_limit(50);
```

### 関連性スコアリング

検索エンジンは以下の要素で関連性を計算します:

1. **キーワード一致** (最大50点)
   - 要旨に含まれる: +30点
   - 判示事項に含まれる: +20点

2. **裁判所レベル** (最大30点)
   - 最高裁判所: +30点
   - 高等裁判所: +20点
   - 地方裁判所以下: +10点

3. **法分野一致** (最大20点)
   - 完全一致: +20点

**スコア範囲**: 0.0 ～ 100.0

---

## 引用形式

### CitationFormatter

判例の引用を複数の形式で生成します。

```rust
use legalis_jp::case_law::{CitationFormatter, CitationStyle};

let citation = CitationFormatter::format(&decision, CitationStyle::Standard)?;
println!("{}", citation);
```

### 引用スタイル

#### 1. Standard (標準形式)

日本の法律文書で一般的に使用される形式。

**出力例**:
```
最高裁判所令和2年(受)第1234号判決（令和2年7月15日）
```

**使用場面**:
- 法律文書
- 学術論文
- 判例評釈

#### 2. Short (短縮形式)

簡潔な引用形式。

**出力例**:
```
最高裁令2・7・15判決
```

**使用場面**:
- 脚注
- 判例リスト
- 簡易参照

#### 3. Full (完全形式)

判示事項を含む詳細な引用。

**出力例**:
```
最高裁判所令和2年(受)第1234号判決（令和2年7月15日）
「使用者は従業員の不法行為について責任を負う」
```

**使用場面**:
- 判例データベース
- 詳細な法律分析
- 研究論文

#### 4. BlueBook (ブルーブック形式)

国際的な法律文献引用形式。

**出力例**:
```
Supreme Court of Japan, Case No. 令和2年(受)第1234号 (July 15, 2020)
```

**使用場面**:
- 英語論文
- 国際比較法研究
- 外国法曹向け文書

### 引用形式の選択ガイド

| 状況 | 推奨形式 | 理由 |
|------|---------|------|
| 判決書の引用 | Standard | 公式文書で標準的 |
| 脚注での言及 | Short | スペース節約 |
| 判例紹介 | Full | 内容が一目でわかる |
| 英語論文 | BlueBook | 国際標準 |

---

## 実用例

### 例1: 民法第709条関連判例の検索

```rust
use legalis_jp::case_law::*;
use chrono::NaiveDate;

fn search_tort_cases() -> Result<(), CaseLawError> {
    // データベースとエンジンの初期化
    let mut db = InMemoryCaseDatabase::new();
    let engine = CaseLawSearchEngine::new(db);

    // 検索クエリ
    let query = CaseSearchQuery::new()
        .with_keyword("不法行為")
        .with_keyword("損害賠償")
        .with_legal_area(LegalArea::Civil)
        .with_limit(20);

    // 検索実行
    let results = engine.search(&query)?;

    println!("検索結果: {}件", results.len());
    println!();

    // 結果の表示
    for (i, result) in results.iter().enumerate() {
        println!("{}. {}", i + 1, result.decision.metadata.case_number);
        println!("   裁判所: {:?}", result.decision.metadata.court.level);
        println!("   判決日: {}", result.decision.metadata.decision_date);
        println!("   関連性: {:.1}%", result.relevance_score);
        println!("   要旨: {}", result.decision.summary);

        // 標準形式で引用
        let citation = CitationFormatter::format(
            &result.decision,
            CitationStyle::Standard
        )?;
        println!("   引用: {}", citation);
        println!();
    }

    Ok(())
}
```

### 例2: 判例データベースの構築

```rust
fn build_case_database() -> Result<InMemoryCaseDatabase, CaseLawError> {
    let mut db = InMemoryCaseDatabase::new();

    // 判例1: 使用者責任 (民法715条)
    let case1 = create_employer_liability_case()?;
    db.add_case(case1)?;

    // 判例2: 不法行為 (民法709条)
    let case2 = create_tort_case()?;
    db.add_case(case2)?;

    // 判例3: 契約不履行 (民法415条)
    let case3 = create_breach_of_contract_case()?;
    db.add_case(case3)?;

    Ok(db)
}

fn create_tort_case() -> Result<CourtDecision, CaseLawError> {
    let metadata = CaseMetadata::new(
        "令和3年(ワ)第5678号",
        NaiveDate::from_ymd_opt(2021, 3, 10).unwrap(),
        Court::new(CourtLevel::District),
        LegalArea::Civil,
        CaseOutcome::PlaintiffWins,
    );

    let holding = Holding {
        issue: "過失による不法行為の成立要件".to_string(),
        reasoning: "被告は注意義務に違反し、原告に損害を与えた。因果関係も認められる。".to_string(),
        conclusion: "民法第709条により損害賠償請求が認容される".to_string(),
    };

    let parties = Parties {
        plaintiff: "佐藤花子".to_string(),
        defendant: "鈴木一郎".to_string(),
    };

    let statute = StatuteCitation {
        statute_name: "民法".to_string(),
        article: "第709条".to_string(),
        paragraph: None,
    };

    Ok(CourtDecision::new(
        "case-minpo709-district-001",
        metadata,
        "過失による不法行為が成立し、損害賠償請求が認容された事例",
    )
    .with_holding(holding)
    .with_parties(parties)
    .with_statute_citation(statute))
}
```

### 例3: 判例の詳細分析

```rust
fn analyze_case(case_id: &str, db: &InMemoryCaseDatabase) -> Result<(), CaseLawError> {
    // 判例を取得
    let case = db.get_case(case_id)?;

    println!("=" .repeat(80));
    println!("判例分析: {}", case.id);
    println!("=".repeat(80));
    println!();

    // 基本情報
    println!("【基本情報】");
    println!("事件番号: {}", case.metadata.case_number);
    println!("判決日: {}", case.metadata.decision_date);
    println!("裁判所: {:?}", case.metadata.court.level);
    println!("法分野: {:?}", case.metadata.legal_area);
    println!("判決結果: {:?}", case.metadata.outcome);
    println!();

    // 当事者
    println!("【当事者】");
    println!("原告: {}", case.parties.plaintiff);
    println!("被告: {}", case.parties.defendant);
    println!();

    // 要旨
    println!("【判決要旨】");
    println!("{}", case.summary);
    println!();

    // 判示事項
    println!("【判示事項】");
    for (i, holding) in case.holdings.iter().enumerate() {
        println!("{}. 論点: {}", i + 1, holding.issue);
        println!("   理由: {}", holding.reasoning);
        println!("   結論: {}", holding.conclusion);
        println!();
    }

    // 引用法令
    println!("【引用法令】");
    for statute in &case.cited_statutes {
        print!("{} {}", statute.statute_name, statute.article);
        if let Some(para) = &statute.paragraph {
            print!(" {}", para);
        }
        println!();
    }
    println!();

    // 引用形式
    println!("【引用例】");
    let standard = CitationFormatter::format(&case, CitationStyle::Standard)?;
    println!("標準形式: {}", standard);

    let short = CitationFormatter::format(&case, CitationStyle::Short)?;
    println!("短縮形式: {}", short);

    let bluebook = CitationFormatter::format(&case, CitationStyle::BlueBook)?;
    println!("ブルーブック: {}", bluebook);
    println!();

    Ok(())
}
```

### 例4: 複数キーワードでの精密検索

```rust
fn advanced_search(db: &InMemoryCaseDatabase) -> Result<(), CaseLawError> {
    let engine = CaseLawSearchEngine::new(db);

    // 「使用者責任」と「損害賠償」の両方を含む判例
    let query = CaseSearchQuery::new()
        .with_keyword("使用者責任")
        .with_keyword("損害賠償")
        .with_court_level(CourtLevel::Supreme)
        .with_limit(5);

    let results = engine.search(&query)?;

    println!("高度検索結果: {}件", results.len());

    for result in results {
        println!();
        println!("事件番号: {}", result.decision.metadata.case_number);
        println!("関連性: {:.1}点", result.relevance_score);

        // 判示事項の表示
        if let Some(holding) = result.decision.holdings.first() {
            println!("主要論点: {}", holding.issue);
        }

        // 短縮引用
        let citation = CitationFormatter::format(
            &result.decision,
            CitationStyle::Short
        )?;
        println!("引用: {}", citation);
    }

    Ok(())
}
```

---

## 他モジュールとの統合

### 1. 不法行為モジュールとの統合

```rust
use legalis_jp::tort::article709::*;
use legalis_jp::case_law::*;

fn tort_claim_with_precedents(
    claim: &TortClaim,
    db: &InMemoryCaseDatabase,
) -> Result<(), CaseLawError> {
    // 不法行為の検証
    let result = validate_tort_claim(claim)?;

    if result.is_valid() {
        // 関連判例の検索
        let engine = CaseLawSearchEngine::new(db);
        let query = CaseSearchQuery::new()
            .with_keyword("不法行為")
            .with_keyword(&claim.harm_description)
            .with_legal_area(LegalArea::Civil)
            .with_limit(5);

        let precedents = engine.search(&query)?;

        println!("【請求の有効性】");
        println!("✓ この不法行為請求は有効です");
        println!();

        println!("【関連判例】");
        for (i, result) in precedents.iter().enumerate() {
            println!("{}. {}", i + 1, result.decision.metadata.case_number);
            println!("   {}", result.decision.summary);

            let citation = CitationFormatter::format(
                &result.decision,
                CitationStyle::Standard
            )?;
            println!("   引用: {}", citation);
            println!();
        }
    }

    Ok(())
}
```

### 2. 労働法モジュールとの統合

```rust
use legalis_jp::labor_law::*;
use legalis_jp::case_law::*;

fn check_dismissal_with_precedents(
    contract: &EmploymentContract,
    dismissal_reason: &str,
    db: &InMemoryCaseDatabase,
) -> Result<(), CaseLawError> {
    // 解雇の妥当性チェック (労働法モジュール)
    // ...

    // 解雇関連の判例検索
    let engine = CaseLawSearchEngine::new(db);
    let query = CaseSearchQuery::new()
        .with_keyword("解雇")
        .with_keyword("整理解雇")
        .with_legal_area(LegalArea::Labor)
        .with_court_level(CourtLevel::Supreme);

    let precedents = engine.search(&query)?;

    println!("【解雇の4要件に関する判例】");
    for result in precedents {
        println!("• {}", result.decision.metadata.case_number);

        // 判示事項から解雇要件を抽出
        for holding in &result.decision.holdings {
            if holding.issue.contains("解雇") {
                println!("  論点: {}", holding.issue);
                println!("  判断: {}", holding.conclusion);
            }
        }
        println!();
    }

    Ok(())
}
```

### 3. 契約テンプレートとの統合

```rust
use legalis_jp::contract_templates::*;
use legalis_jp::case_law::*;

fn generate_contract_with_legal_notes(
    template: &EmploymentContractTemplate,
    db: &InMemoryCaseDatabase,
) -> Result<String, TemplateError> {
    // 契約書を生成
    let contract = template.render()?;

    // 関連判例を検索
    let engine = CaseLawSearchEngine::new(db);
    let query = CaseSearchQuery::new()
        .with_keyword("雇用契約")
        .with_keyword("労働条件")
        .with_legal_area(LegalArea::Labor)
        .with_limit(3);

    let precedents = engine.search(&query)?;

    // 契約書に判例注釈を追加
    let mut annotated_contract = contract.clone();
    annotated_contract.push_str("\n\n【参考判例】\n");

    for (i, result) in precedents.iter().enumerate() {
        let citation = CitationFormatter::format(
            &result.decision,
            CitationStyle::Standard
        )?;
        annotated_contract.push_str(&format!("{}. {}\n", i + 1, citation));
        annotated_contract.push_str(&format!("   要旨: {}\n", result.decision.summary));
    }

    Ok(annotated_contract)
}
```

---

## ベストプラクティス

### 1. データベースの初期化

```rust
// アプリケーション起動時に一度だけ初期化
lazy_static! {
    static ref CASE_DATABASE: InMemoryCaseDatabase = {
        let mut db = InMemoryCaseDatabase::new();
        // 判例データの読み込み
        load_cases_from_file(&mut db).expect("Failed to load cases");
        db
    };
}
```

### 2. エラーハンドリング

```rust
fn safe_case_search(query: &CaseSearchQuery) -> Result<Vec<SearchResult>, String> {
    let engine = CaseLawSearchEngine::new(&*CASE_DATABASE);

    engine.search(query).map_err(|e| match e {
        CaseLawError::CaseNotFound(id) => format!("判例が見つかりません: {}", id),
        CaseLawError::InvalidQuery(msg) => format!("無効な検索条件: {}", msg),
        _ => format!("検索エラー: {}", e),
    })
}
```

### 3. 検索結果のキャッシング

```rust
use std::collections::HashMap;

struct CachedSearchEngine {
    engine: CaseLawSearchEngine<InMemoryCaseDatabase>,
    cache: HashMap<String, Vec<SearchResult>>,
}

impl CachedSearchEngine {
    fn search_with_cache(&mut self, query: &CaseSearchQuery) -> Result<Vec<SearchResult>, CaseLawError> {
        let cache_key = format!("{:?}", query);

        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        let results = self.engine.search(query)?;
        self.cache.insert(cache_key, results.clone());
        Ok(results)
    }
}
```

### 4. 判例IDの命名規則

```rust
// 推奨される命名規則:
// {法令}-{条文}-{裁判所レベル}-{連番}

let case_ids = vec![
    "minpo-article709-supreme-001",      // 民法709条最高裁判例1
    "minpo-article715-high-002",         // 民法715条高裁判例2
    "rodokeiyakuho-article18-district-003", // 労働契約法18条地裁判例3
];
```

---

## トラブルシューティング

### 問題1: 検索結果が0件

**原因**: キーワードが厳密すぎる、またはフィルタが過度に制限的

**解決策**:
```rust
// 段階的に条件を緩和
let query1 = CaseSearchQuery::new()
    .with_keyword("非常に具体的なキーワード")
    .with_court_level(CourtLevel::Supreme);

let results1 = engine.search(&query1)?;

if results1.is_empty() {
    // 裁判所レベルフィルタを削除
    let query2 = CaseSearchQuery::new()
        .with_keyword("非常に具体的なキーワード");

    let results2 = engine.search(&query2)?;

    if results2.is_empty() {
        // キーワードを一般化
        let query3 = CaseSearchQuery::new()
            .with_keyword("一般的なキーワード");

        let results3 = engine.search(&query3)?;
    }
}
```

### 問題2: 関連性スコアが低い

**原因**: 検索キーワードが判例内容と一致していない

**解決策**:
```rust
// 複数の関連キーワードを試す
let keywords = vec!["損害賠償", "不法行為", "過失"];

for keyword in keywords {
    let query = CaseSearchQuery::new().with_keyword(keyword);
    let results = engine.search(&query)?;

    if !results.is_empty() && results[0].relevance_score > 50.0 {
        println!("有効なキーワード: {}", keyword);
        break;
    }
}
```

### 問題3: メモリ使用量が多い

**原因**: InMemoryCaseDatabase に大量の判例を保存

**解決策**:
1. データベースバックエンドの実装を検討 (将来)
2. 判例を分割してロード
3. 必要な判例のみを保持

```rust
fn load_relevant_cases_only(
    db: &mut InMemoryCaseDatabase,
    legal_area: LegalArea,
) -> Result<(), CaseLawError> {
    // 特定の法分野の判例のみロード
    let all_cases = load_all_cases_metadata()?;

    for case_meta in all_cases {
        if case_meta.legal_area == legal_area {
            let case = load_full_case(&case_meta.id)?;
            db.add_case(case)?;
        }
    }

    Ok(())
}
```

---

## パフォーマンス最適化

### 1. インデックス作成 (将来の拡張)

```rust
// 現在はフルスキャン、将来的にはインデックスを追加
struct IndexedCaseDatabase {
    cases: HashMap<String, CourtDecision>,
    keyword_index: HashMap<String, Vec<String>>,  // keyword -> case_ids
    court_index: HashMap<CourtLevel, Vec<String>>, // court -> case_ids
}
```

### 2. 並列検索

```rust
use rayon::prelude::*;

fn parallel_search(
    cases: &[CourtDecision],
    keyword: &str,
) -> Vec<SearchResult> {
    cases
        .par_iter()
        .filter_map(|case| {
            let score = calculate_relevance(case, keyword);
            if score > 0.0 {
                Some(SearchResult {
                    decision: case.clone(),
                    relevance_score: score,
                })
            } else {
                None
            }
        })
        .collect()
}
```

---

## まとめ

判例データベースシステムは以下を提供します:

1. **完全な判例管理**: メタデータ、判示事項、当事者、引用法令
2. **高度な検索**: キーワード、裁判所レベル、法分野でフィルタリング
3. **関連性スコアリング**: 検索結果の自動ランキング
4. **柔軟な引用形式**: 4種類の引用スタイルをサポート
5. **モジュール統合**: 不法行為、労働法、契約テンプレートと連携

### 次のステップ

1. `examples/case-law-search-demo.rs` を実行して動作を確認
2. 自分の判例データを追加
3. 検索クエリをカスタマイズ
4. 他のモジュールと統合

### 参考リソース

- **例**: `/mnt/fd/legalis/jurisdictions/jp/examples/case-law-search-demo.rs`
- **モジュール**: `/mnt/fd/legalis/jurisdictions/jp/src/case_law/`
- **テスト**: `/mnt/fd/legalis/jurisdictions/jp/src/case_law/tests.rs`

---

**最終更新**: 2025-01-09
**バージョン**: 0.1.1
**ライセンス**: MIT
