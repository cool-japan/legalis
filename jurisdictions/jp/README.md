# legalis-jp

日本法サポートライブラリ for Legalis-RS

**Version 0.2.0** - 5 Major Law Domains with e-Gov Electronic Filing Support

## 概要

`legalis-jp`は、Legalis-RSフレームワークにおける日本の法体系サポートを提供するクレートです。日本特有の法制度、和暦、e-Gov電子申請システム、主要法令の実装を含みます。

### v0.2.0 新機能 🎉

**5つの主要法域とe-Gov電子申請サポートを追加実装：**

#### e-Gov電子申請システム（Electronic Filing System）
- ✅ XML/JSON形式サポート - レガシーXMLと最新JSON形式の両対応
- ✅ 申請状態管理 - Draft/Submitted/UnderReview/Approved等の状態遷移
- ✅ 書類添付管理 - MIME type検証、10MB制限チェック
- ✅ バリデーション - 申請前の必須項目確認と業務ルール検証

#### 行政手続法・電子署名法（Administrative Procedure Act）
- ✅ Article 2: 手続類型（申請・届出・行政指導・処分・聴聞）
- ✅ Article 5: 理由の提示要件の検証
- ✅ Article 7: 標準処理期間の管理
- ✅ 電子署名: RSA-2048/4096、ECDSA-P256/P384対応
- ✅ 電子証明書の有効期限検証

#### 建設業法・宅建業法（Construction & Real Estate Acts）
- ✅ Article 3: 建設業許可（一般・特定）の資本要件検証
- ✅ Article 7-8: 最低資本金・技術者要件の自動判定
- ✅ Article 35: 宅建業の重要事項説明義務の確認
- ✅ Article 46: 仲介手数料上限の自動計算（3-5%段階制）
- ✅ 5年有効期限と更新管理

#### 環境法（Environmental Law）
- ✅ 大気汚染防止法: SOx/NOx/ばいじん等の排出基準検証
- ✅ Article 6: 工場設置の60日前届出要件の確認
- ✅ 水質汚濁防止法: BOD/COD基準の管理
- ✅ 廃棄物処理法: 収集運搬業・処分業の許可検証
- ✅ Article 12-3: 産業廃棄物マニフェスト制度

#### 個人情報保護法（Personal Information Protection Act）
- ✅ Article 15: 利用目的の特定・明示義務
- ✅ Article 17: 要配慮個人情報の同意取得
- ✅ Article 20: 安全管理措置（アクセス制御・暗号化等）
- ✅ Article 23-24: 第三者提供・越境移転の制限
- ✅ Article 28-30: 開示・訂正・利用停止請求への対応
- ✅ AIリスク評価: データ量・機械学習・プロファイリング考慮

#### 消費者保護法の拡張（Consumer Protection Enhancement）
- ✅ ECプラットフォーム対応（自社EC・マーケットプレイス等）
- ✅ デジタルコンテンツの特別規定
- ✅ Article 11: 特定商取引法に基づく表記の検証
- ✅ 返品ポリシーの消費者フレンドリー度判定
- ✅ サブスクリプションサービスの解約条件検証

**398個のテスト、27,600行以上のコード！**

---

### v0.1.0 既存機能

**型安全なビルダーパターンAPIで主要法域を実装：**

#### 民法（Civil Code）
- ✅ Article 709（一般不法行為）- 完全な要件判定システム
- ✅ Article 710（非財産的損害・慰謝料）- 慰謝料額自動推奨
- ✅ Article 715（使用者責任）- 雇用主の代位責任判定
- ✅ Article 415（債務不履行）- 契約違反の5要件検証

#### 商法・会社法（Commercial Law）
- ✅ Companies Act（会社法）- 株式会社・合同会社の設立と運営
- ✅ Articles of Incorporation（定款）- 設立登記要件の検証
- ✅ Shareholders Meetings（株主総会）- 定足数・決議要件の自動判定

#### 労働法（Labor Law）
- ✅ Labor Standards Act（労働基準法）- 労働時間・残業代の計算
- ✅ Employment Contracts（雇用契約）- 契約形態と法令遵守の検証
- ✅ Termination Notice（解雇予告）- 解雇予告期間と手当の自動計算

#### 知的財産法（Intellectual Property Law）
- ✅ Patent Act（特許法）- 特許出願要件の検証システム
- ✅ Copyright Act（著作権法）- 著作物の保護期間と権利判定
- ✅ Trademark Act（商標法）- 商標登録要件の検証
- ✅ Design Act（意匠法）- 意匠登録の適格性判定

#### 消費者保護法（Consumer Protection Law）
- ✅ Consumer Contract Act（消費者契約法）- 不当条項の検出
- ✅ Cooling-off Period（クーリングオフ）- 解約権の要件判定
- ✅ Specified Commercial Transactions（特定商取引法）- 取引規制の検証

#### 判例データベース（Case Law Database）
- ✅ Court Decisions（裁判例検索）- 最高裁・高裁・地裁の判例検索
- ✅ Citation Formatting（判例引用）- 標準形式での判例引用生成
- ✅ Leading Precedents（重要判例）- リーディングケースの管理

#### リスク分析システム（Risk Analysis System）
- ✅ Contract Risk Detection（契約リスク検出）- 不当条項の自動検出
- ✅ Compliance Checking（法令遵守チェック）- 消費者保護法・労働法違反の検出
- ✅ Risk Scoring（リスク評価）- 4段階の深刻度分類（Critical/High/Medium/Low）
- ✅ Automated Recommendations（改善提案）- 具体的な修正案の自動生成

#### 契約テンプレート（Contract Templates）
- ✅ Template Engine（テンプレートエンジン）- 変数置換と条件分岐
- ✅ Clause Library（条項ライブラリ）- 18種類の標準条項
- ✅ Contract Generation（契約書生成）- 雇用契約・NDA・業務委託契約
- ✅ Type-Safe Variables（型安全な変数）- 文字列・数値・日付・リスト対応

**176個のテスト、10個の動作例、13,400行以上のコード！**

## 機能

### 和暦（元号）サポート

日本の元号システムに対応した日付処理機能を提供します。

- 明治、大正、昭和、平成、令和の各元号に対応
- 西暦と和暦の相互変換
- 法令における正式な日付表記のサポート

```rust
use legalis_jp::{Era, JapaneseDate};

let date = JapaneseDate::new(Era::Reiwa, 5, 1, 1)?;
assert_eq!(date.to_gregorian()?, (2023, 1, 1));
```

### e-Gov XML法令パーサー

e-Gov法令検索のXML形式に対応したパーサーを提供します。

- e-Gov法令データの読み込み
- 条文構造の解析
- バイリンガル（日本語/英語）対応

```rust
use legalis_jp::EGovLawParser;

let parser = EGovLawParser::new();
let law = parser.parse_xml(xml_content)?;
```

### 日本国憲法

日本国憲法の条文を構造化されたデータとして提供します。

- 全103条の憲法条文
- 章・条・項の階層構造
- バイリンガル表記

```rust
use legalis_jp::Constitution;

let constitution = Constitution::new();
let article_9 = constitution.get_article(9)?;
```

### 民法（Minpo）

民法の主要条文を実装しています。不法行為法（Articles 709, 710, 715）と契約法（Article 415）をカバー。

#### 不法行為法（Tort Law）

- **第709条（一般不法行為）**: 故意・過失、権利侵害、因果関係、損害の4要件を判定
- **第710条（非財産的損害・慰謝料）**: Article 709を前提とし、精神的苦痛への慰謝料を算定
- **第715条（使用者責任）**: 雇用主の代位責任。従業員の不法行為に対する使用者の責任を判定

```rust
use legalis_jp::tort::{Article709, Article710, Article715};

// Article 709: 基本的な不法行為
let tort = Article709::new()
    .perpetrator("加害者")
    .victim("被害者")
    .intent_or_negligence(true)
    .rights_violation(true)
    .causal_link_direct()
    .damage(500_000, "治療費")
    .build()?;

// Article 710: 慰謝料請求
let damages = Article710::new()
    .with_article_709(tort)
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Moderate)
    .build()?;

// Article 715: 使用者責任
let employer_liability = Article715::new()
    .employee_tort(employee_tort)
    .employer("会社名")
    .employee("従業員名")
    .employment_type(EmploymentType::FullTime)
    .during_business_execution(true)
    .build()?;
```

#### 契約法（Contract Law）

- **第415条（債務不履行）**: 契約違反の5要件（債務の存在、不履行、帰責事由、因果関係、損害）を検証

```rust
use legalis_jp::contract::{Article415, ObligationType, BreachType, AttributionType};

let breach = Article415::new()
    .with_obligation(ObligationType::Monetary {
        amount: 1_000_000,
        currency: "JPY".into()
    })
    .with_breach(BreachType::NonPerformance)
    .with_attribution(Attribution::new(
        AttributionType::Negligence,
        "正当な理由なく履行拒否"
    ))
    .creditor("会社A")
    .debtor("供給業者B")
    .build()?;
```

### 商法・会社法（Commercial Law）

株式会社と合同会社の設立、運営、株主総会の要件を完全実装。

```rust
use legalis_jp::commercial_law::{ArticlesOfIncorporation, CompanyType, Capital};

// 株式会社の定款作成
let articles = ArticlesOfIncorporation::new()
    .company_name("テクノロジーソリューションズ株式会社")
    .company_type(CompanyType::StockCompany)
    .capital(Capital::new(10_000_000))
    .authorized_shares(10_000)
    .business_purpose("ソフトウェア開発及び販売")
    .build()?;

// 株主総会の定足数チェック
use legalis_jp::commercial_law::ShareholdersMeeting;

let meeting = ShareholdersMeeting::new()
    .total_voting_rights(1_500)
    .present_voting_rights(1_000)
    .agenda_item("決算承認")
    .build()?;

assert!(meeting.has_quorum()); // 定足数を満たしているか判定
```

### 労働法（Labor Law）

労働基準法に基づく労働時間管理、残業代計算、解雇予告の要件を実装。

```rust
use legalis_jp::labor_law::{EmploymentContract, EmploymentType, WorkingTimeRecord};

// 雇用契約の作成と検証
let contract = EmploymentContract::builder()
    .employee_name("山田太郎")
    .employer_name("株式会社ABC")
    .employment_type(EmploymentType::IndefiniteTerm)
    .base_wage(350_000)
    .daily_working_hours(8.0)
    .build()?;

// 残業代の自動計算
let record = WorkingTimeRecord::new()
    .date("2024-01-15")
    .working_hours(10.0) // 実働10時間
    .rest_period_minutes(60)
    .build()?;

let overtime_pay = record.calculate_overtime_premium(contract.base_wage())?;
```

### 知的財産法（Intellectual Property Law）

特許法、著作権法、商標法、意匠法の出願・登録要件を完全カバー。

```rust
use legalis_jp::intellectual_property::{PatentApplication, CopyrightedWork, TrademarkApplication};

// 特許出願の要件検証
let patent = PatentApplication::new()
    .title("エネルギー効率改善装置")
    .application_number("2020-123456")
    .inventor("山田太郎")
    .claim("特定の構造により消費電力を50%削減")
    .build()?;

// 著作権保護期間の判定
let work = CopyrightedWork::new()
    .title("春の物語")
    .author("作家名")
    .publication_date("2020-04-01")
    .work_type(WorkType::Literary)
    .build()?;

let protection_expires = work.protection_expiration_date()?; // 著作権満了日を自動計算
```

### 消費者保護法（Consumer Protection Law）

消費者契約法と特定商取引法に基づく不当条項の検出とクーリングオフ。

```rust
use legalis_jp::consumer_protection::{ConsumerContract, CoolingOffExercise};

// 消費者契約の検証
let contract = ConsumerContract::new()
    .contract_title("オンライン通販サービス")
    .clause("当社は一切責任を負いません") // 全部免責条項（無効）
    .build()?;

let validation = contract.validate()?;
assert!(validation.has_unfair_terms()); // 不当条項を検出

// クーリングオフ期間の判定
let cooling_off = CoolingOffExercise::new()
    .transaction_type(TransactionType::DoorToDoorSales)
    .contract_date("2024-01-10")
    .build()?;

assert!(cooling_off.is_within_period("2024-01-15")); // 8日以内
```

### 判例データベース（Case Law Database）

裁判例の検索、引用、重要判例の管理機能。

```rust
use legalis_jp::case_law::{CaseLawDatabase, InMemoryCaseDatabase, CourtLevel, CitationFormatter};

// 判例データベースの構築
let mut database = InMemoryCaseDatabase::new();

database.add_decision(CourtDecision::new()
    .case_number("令和2年(受)第1234号")
    .court_level(CourtLevel::SupremeCourt)
    .date("2022-06-15")
    .summary("不法行為における因果関係の立証基準")
    .is_leading_precedent(true)
    .build()?);

// 判例の検索
let results = database.search_by_keyword("因果関係")?;

// 標準形式での引用
let formatter = CitationFormatter::new();
let citation = formatter.format_citation(&results[0]);
// "最高裁令和2年(受)第1234号判決（令和4年6月15日）"
```

### リスク分析システム（Risk Analysis System）

契約書の自動リスク検出、法令遵守チェック、改善提案の生成。

```rust
use legalis_jp::risk_analysis::{quick_analyze, ContractType, RiskSeverity};

// 契約書の包括的リスク分析
let report = quick_analyze(
    "オンライン通販サービス利用規約",
    ContractType::Consumer,
    "第5条 当社は一切責任を負いません。
     第10条 解約時には全額を違約金として支払うものとします。"
)?;

// リスク評価結果
println!("Overall Risk Score: {}/100", report.overall_risk_score);
println!("Critical Risks: {}", report.critical_count());

// 検出された問題の詳細
for finding in &report.findings {
    if finding.severity == RiskSeverity::Critical {
        println!("❌ {}", finding.issue_description);
        println!("   Legal Reference: {}", finding.legal_reference.as_ref().unwrap());
        println!("   Recommendation: {}", finding.recommendation);
    }
}
```

### 契約テンプレート（Contract Templates）

テンプレートエンジンを使用して、動的な契約書を自動生成。

```rust
use legalis_jp::contract_templates::*;

// テンプレートエンジンの作成
let mut engine = TemplateEngine::new();

// 雇用契約テンプレートの定義
let template = ContractTemplate::new(
    "employment_contract",
    "雇用契約書",
    TemplateType::Employment,
    r#"
{{employer_name}}（甲）と{{employee_name}}（乙）は、以下のとおり雇用契約を締結する。

第1条（雇用）
甲は乙を{{position}}として雇用する。

第2条（賃金）
基本給: 月額{{base_salary}}円

{{#if has_probation}}
第3条（試用期間）
雇用開始日から{{probation_months}}ヶ月間を試用期間とする。
{{/if}}
    "#,
);

engine.register_template(template);

// 変数の設定
let mut context = TemplateContext::new();
context.set_string("employer_name", "株式会社ABC");
context.set_string("employee_name", "山田太郎");
context.set_string("position", "ソフトウェアエンジニア");
context.set_integer("base_salary", 400_000);
context.set_boolean("has_probation", true);
context.set_integer("probation_months", 3);

// 契約書の生成
let contract = engine.render("employment_contract", &context)?;
println!("{}", contract.content_ja);

// 標準条項ライブラリの使用
let library = ClauseLibrary::new();
let confidentiality = library.get_clause("confidentiality_obligation").unwrap();
println!("{}", confidentiality.title_ja); // "第13条（秘密保持義務）"
```

## ドキュメント / Documentation

### 民法条文ガイド（Civil Code Article Guides）

各条文の詳細な解説ガイド：

- **[ARTICLE709_GUIDE.md](docs/ARTICLE709_GUIDE.md)** - 第709条（一般不法行為）完全ガイド (551行)
- **[ARTICLE710_GUIDE.md](docs/ARTICLE710_GUIDE.md)** - 第710条（非財産的損害）完全ガイド (540行)
- **[ARTICLE715_GUIDE.md](docs/ARTICLE715_GUIDE.md)** - 第715条（使用者責任）完全ガイド (660行)
- **[ARTICLE415_GUIDE.md](docs/ARTICLE415_GUIDE.md)** - 第415条（債務不履行）完全ガイド (637行)
- **[INTEGRATED_GUIDE.md](docs/INTEGRATED_GUIDE.md)** - 統合ガイド：全条文の連携 (544行)

各ガイドには以下を含みます：
- 📖 法的背景と条文（日英バイリンガル）
- ⚖️ 要件事実と判定ロジック
- 💼 実務での使用例（3つ以上のシナリオ）
- 🔧 完全なAPIリファレンス
- ❓ FAQ・よくある質問

### 労働法ガイド（Labor Law Guides）

労働契約と36協定の実務ガイド：

- **[CONTRACT_VALIDATION_GUIDE.md](docs/CONTRACT_VALIDATION_GUIDE.md)** - 雇用契約検証システム完全ガイド (~900行)
  - 雇用契約の要件検証（必須項目・推奨項目）
  - 契約形態別の検証（正社員・契約社員・パート・派遣）
  - 実務での使用例と検証フロー
  - トラブルシューティングとベストプラクティス

- **[ARTICLE_36_AGREEMENT_GUIDE.md](docs/ARTICLE_36_AGREEMENT_GUIDE.md)** - 36協定（時間外労働協定）完全ガイド (~850行)
  - 36協定の基本要件と特別条項
  - 時間外労働の上限規制（月45時間・年360時間）
  - 特別条項の適用要件（月100時間未満・年720時間以内）
  - インタラクティブな協定作成ツール

### 判例データベースガイド（Case Law Database Guides）

判例検索と引用のための完全ガイド：

- **[CASE_LAW_DATABASE_GUIDE.md](docs/CASE_LAW_DATABASE_GUIDE.md)** - 判例データベース完全ガイド (~800行)
  - 日本の裁判所システム（最高裁・高裁・地裁・家裁・簡裁）
  - 判例の保存・管理と検索機能
  - 関連性スコアリングアルゴリズム
  - 4種類の引用形式（標準・短縮・完全・ブルーブック）
  - 不法行為・労働法・契約テンプレートとの統合例
  - ベストプラクティスとトラブルシューティング

- **[CASE_LAW_QUICK_REFERENCE.md](docs/CASE_LAW_QUICK_REFERENCE.md)** - 判例データベースクイックリファレンス (~400行)
  - 5分で始める判例検索
  - よく使うコードスニペット
  - チートシート（裁判所レベル・法分野・判決結果・引用スタイル）
  - 実践テンプレート（判例検索アプリ・データベースビルダー・分析レポート生成）
  - トラブルシューティング早見表

## サンプルコード / Examples

10個の実働サンプルで各法域の使い方を学べます：

```bash
# 商法・会社法
cargo run --example company-formation-kaisha           # 株式会社の設立
cargo run --example shareholders-meeting-validation    # 株主総会の要件検証

# 労働法
cargo run --example employment-contract-validator      # 雇用契約の検証
cargo run --example overtime-calculator                # 残業代の自動計算

# 知的財産法
cargo run --example patent-application-validator       # 特許出願の要件チェック
cargo run --example copyright-trademark-validator      # 著作権・商標の保護期間判定

# 消費者保護法
cargo run --example consumer-contract-risk-analyzer    # 消費者契約の不当条項検出

# 判例データベース
cargo run --example case-law-search-demo              # 判例検索と引用

# リスク分析システム
cargo run --example contract-risk-analyzer            # 契約書の包括的リスク分析

# 契約テンプレート
cargo run --example contract-template-generator        # 契約書の自動生成（雇用・NDA・業務委託）
```

各サンプルは完全に動作し、実務に即した3-4つのシナリオを含みます。

## テスト・品質保証 / Testing & Quality

```bash
# 全テストを実行（176個のテストすべてがパス）
cargo nextest run --all-features

# 警告ゼロポリシー準拠
cargo clippy --all-targets --all-features

# ドキュメントテスト
cargo test --doc

# ドキュメント生成
cargo doc --no-deps --open
```

**品質指標:**
- ✅ **176個のユニットテスト** - 全モジュールの要件判定をカバー
- ✅ **30個のドキュメントテスト** - APIの使用例を実行可能な形で提供
- ✅ **ゼロ警告ポリシー** - コンパイラ警告・Clippy警告なし
- ✅ **型安全性** - ビルダーパターンで不正な状態を排除
- ✅ **13,400行以上のコード** - 本番環境対応の完全実装
- ✅ **10個の動作例** - すべてコンパイル・実行可能
- ✅ **包括的カバレッジ** - 8つの主要法域を完全実装

## 法体系の特徴

日本法は**大陸法系（Civil Law）**に属し、以下の特徴を持ちます：

- 成文法主義：法典（民法、刑法など）が主要な法源
- 演繹的推論：法典の条文から個別事案へ適用
- 制定法優位：判例は参考だが拘束力なし

### 英米法（Common Law）との比較

| 特徴 | 大陸法（日本） | 英米法（米国） |
|------|--------------|--------------|
| 主要法源 | 法典・制定法 | 判例・先例 |
| 裁判所の役割 | 法の適用 | 法の創造 |
| 推論方法 | 演繹的（法典→事案） | 類推的（判例→判例） |
| 拘束力 | 制定法の文言 | 先例（stare decisis） |

## 依存関係

- `legalis-core` - コア型とトレイト
- `legalis-i18n` - 国際化サポート
- `chrono` - 日時処理
- `quick-xml` - XMLパーサー

## ライセンス

MIT OR Apache-2.0

## 関連リンク

- [e-Gov法令検索](https://elaws.e-gov.go.jp/)
- [日本法令外国語訳データベース](https://www.japaneselawtranslation.go.jp/)
- [GitHub: cool-japan/legalis](https://github.com/cool-japan/legalis)
