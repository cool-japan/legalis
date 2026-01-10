# 第36条協定ガイド (36協定実装ガイド)

## 概要

労働基準法第36条は、日本における時間外労働の枠組みを規定しています。このガイドでは、Legalis-JPにおける第36条協定の検証実装について説明します。

## 第36条協定とは

**正式名称**: 時間外・休日労働に関する協定

**通称**: 36協定 (サブロク協定、Saburoku Kyōtei)

**目的**: 法定労働時間を超える時間外労働を使用者が命じることを可能にするが、厳格な規制条件下でのみ許可される。

## 法的枠組み

### 法定上限 (第36条協定なしの場合)

労働基準法第32条により、デフォルトの上限は:

- **1日**: 最大8時間
- **週**: 最大40時間

**第36条協定なし**: 時間外労働は**違法**

### 第36条協定がある場合

使用者は以下の条件を満たせば時間外労働を命じることができる:

1. ✅ 労働基準監督署に書面による協定を届出
2. ✅ 労働者代表による署名
3. ✅ 時間外労働の上限を明示し遵守
4. ✅ 時間外労働の理由を文書化

## Legalis-JPでの実装

### データ構造

```rust
pub struct Article36Agreement {
    /// 事業主名
    pub employer_name: String,

    /// 労働者代表
    pub labor_representative: String,

    /// 有効期間開始日
    pub effective_date: NaiveDate,

    /// 有効期間終了日
    pub expiration_date: NaiveDate,

    /// 1日の時間外労働上限
    pub max_overtime_per_day: u32,

    /// 1ヶ月の時間外労働上限
    /// 標準: 45時間
    pub max_overtime_per_month: u32,

    /// 1年の時間外労働上限
    /// 標準: 360時間
    pub max_overtime_per_year: u32,

    /// 特別条項の有無
    pub has_special_circumstances: bool,

    /// 特別条項の月上限
    /// 最大: 100時間 (休日労働含む)
    pub special_max_per_month: Option<u32>,

    /// 特別条項の年間適用回数
    /// 最大: 6ヶ月
    pub special_months_per_year: Option<u32>,

    /// 時間外労働の理由
    pub permitted_reasons: Vec<String>,
}
```

### 標準上限

**特別条項なしの場合**:

| 期間 | 時間外労働上限 |
|------|---------------|
| 1日あたり | 厳密な上限なし (ただし「合理的」でなければならない) |
| 1ヶ月あたり | 45時間 |
| 1年あたり | 360時間 |

**例 - 標準的な協定**:

```rust
use legalis_jp::labor_law::Article36Agreement;
use chrono::{Utc, Duration};

let agreement = Article36Agreement {
    employer_name: "株式会社テクノロジー".to_string(),
    labor_representative: "山田太郎（労働者代表）".to_string(),
    effective_date: Utc::now().date_naive(),
    expiration_date: (Utc::now() + Duration::days(365)).date_naive(),
    max_overtime_per_day: 3,      // 1日3時間
    max_overtime_per_month: 45,   // 標準上限
    max_overtime_per_year: 360,   // 標準上限
    has_special_circumstances: false,
    special_max_per_month: None,
    special_months_per_year: None,
    permitted_reasons: vec![
        "納期対応のため".to_string(),
        "業務繁忙期のため".to_string(),
    ],
};

// 検証
assert!(agreement.is_within_standard_limits());
assert!(agreement.validate().is_ok());
```

### 特別条項 (特別条項)

**一時的、例外的**な事情が発生した場合、使用者は標準上限を超えることができる:

**特別上限** (告示第316号):

| 要件 | 上限 |
|------|------|
| 月あたりの最大 | 100時間 (休日労働含む) |
| 年間適用可能月数 | 最大6ヶ月 |
| 2-6ヶ月平均 | 80時間 |
| 具体的理由 | 文書化必須 |

**例 - 特別条項付き協定**:

```rust
let agreement = Article36Agreement {
    employer_name: "株式会社製造業".to_string(),
    labor_representative: "佐藤花子（労働者代表）".to_string(),
    effective_date: Utc::now().date_naive(),
    expiration_date: (Utc::now() + Duration::days(365)).date_naive(),
    max_overtime_per_day: 5,
    max_overtime_per_month: 45,      // 標準ベース
    max_overtime_per_year: 360,      // 標準ベース
    has_special_circumstances: true, // 特別条項有効
    special_max_per_month: Some(80), // 月最大80時間まで可
    special_months_per_year: Some(6), // 年6ヶ月まで
    permitted_reasons: vec![
        "突発的な設備トラブル対応".to_string(),
        "納期の急な短縮要請".to_string(),
        "大規模プロジェクトの納期対応".to_string(),
    ],
};

// 検証
assert!(agreement.is_within_standard_limits()); // ベース上限OK
assert!(agreement.is_special_circumstances_valid()); // 特別条項OK
assert!(agreement.validate().is_ok());
```

## 検証ルール

### 1. 標準上限チェック

```rust
pub fn is_within_standard_limits(&self) -> bool {
    self.max_overtime_per_month <= 45 &&
    self.max_overtime_per_year <= 360
}
```

**合格**: 月45時間**かつ**年360時間以下
**不合格**: いずれかの上限を超過

### 2. 特別条項チェック

```rust
pub fn is_special_circumstances_valid(&self) -> bool {
    if !self.has_special_circumstances {
        return true; // 特別条項を使用しない場合、常に有効
    }

    match (self.special_max_per_month, self.special_months_per_year) {
        (Some(monthly), Some(months)) => {
            monthly <= 100 &&  // 月最大100時間
            months <= 6        // 年最大6ヶ月
        }
        _ => false, // 設定不足
    }
}
```

**要件**:
- ✅ `special_max_per_month` ≤ 100時間
- ✅ `special_months_per_year` ≤ 6ヶ月
- ✅ 有効にした場合、両フィールド必須

### 3. 有効期間チェック

```rust
pub fn is_currently_valid(&self) -> bool {
    let now = Utc::now().date_naive();
    now >= self.effective_date && now <= self.expiration_date
}
```

**有効**: 現在の日付が有効期間内
**無効**: 協定が期限切れまたはまだ有効でない

### 4. 設定チェック

```rust
pub fn validate(&self) -> Result<(), String> {
    // 標準上限チェック
    if !self.is_within_standard_limits() {
        return Err("標準上限を超過 (月45時間、年360時間)");
    }

    // 特別条項が有効な場合チェック
    if !self.is_special_circumstances_valid() {
        return Err("無効な特別条項設定");
    }

    // 理由が提供されているかチェック
    if self.permitted_reasons.is_empty() {
        return Err("時間外労働の理由を指定する必要があります");
    }

    // 有効期間チェック
    if self.expiration_date <= self.effective_date {
        return Err("終了日は開始日より後でなければなりません");
    }

    Ok(())
}
```

## 実例

### 例1: ソフトウェア企業 (標準)

```rust
let agreement = Article36Agreement {
    employer_name: "テクノロジー株式会社".to_string(),
    labor_representative: "エンジニア代表：鈴木一郎".to_string(),
    effective_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
    expiration_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
    max_overtime_per_day: 3,
    max_overtime_per_month: 40,  // 標準45時間以下
    max_overtime_per_year: 300,  // 標準360時間以下
    has_special_circumstances: false,
    special_max_per_month: None,
    special_months_per_year: None,
    permitted_reasons: vec![
        "プロダクトリリース対応".to_string(),
        "顧客サポート対応".to_string(),
        "システム障害対応".to_string(),
    ],
};

println!("ステータス: {:?}", agreement.validate()); // Ok(())
```

### 例2: 製造業 (特別条項あり)

```rust
let agreement = Article36Agreement {
    employer_name: "製造業株式会社".to_string(),
    labor_representative: "工場労働者代表：田中次郎".to_string(),
    effective_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
    expiration_date: NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
    max_overtime_per_day: 5,
    max_overtime_per_month: 45,
    max_overtime_per_year: 360,
    has_special_circumstances: true,
    special_max_per_month: Some(70),  // 特別月は70時間まで可
    special_months_per_year: Some(4), // 4ヶ月間 (繁忙期)
    permitted_reasons: vec![
        "年末年始の繁忙期対応".to_string(),
        "大型受注への対応".to_string(),
        "設備トラブルの緊急対応".to_string(),
    ],
};

println!("標準OK: {}", agreement.is_within_standard_limits());
println!("特別OK: {}", agreement.is_special_circumstances_valid());
```

### 例3: 無効な設定

```rust
// ❌ 無効: 特別条項が上限を超過
let invalid_agreement = Article36Agreement {
    employer_name: "ブラック企業株式会社".to_string(),
    labor_representative: "代表".to_string(),
    effective_date: Utc::now().date_naive(),
    expiration_date: (Utc::now() + Duration::days(365)).date_naive(),
    max_overtime_per_day: 5,
    max_overtime_per_month: 45,
    max_overtime_per_year: 360,
    has_special_circumstances: true,
    special_max_per_month: Some(120), // ❌ 100時間上限を超過!
    special_months_per_year: Some(8),  // ❌ 6ヶ月上限を超過!
    permitted_reasons: vec!["常時繁忙".to_string()],
};

match invalid_agreement.validate() {
    Ok(_) => println!("有効"),
    Err(e) => println!("無効: {}", e),
    // 出力: "無効: 無効な特別条項設定"
}
```

## 契約検証との統合

### 時間外労働要件の検出

```rust
use legalis_jp::contract_templates::employment_helper::validate_employment_data;

let report = validate_employment_data(
    "山田太郎",
    "株式会社ABC",
    450_000,
    9,  // 1日9時間 - 8時間の法定上限を超過
    5,
    Prefecture::Tokyo,
)?;

// 警告をチェック
if !report.warnings.is_empty() {
    for warning in &report.warnings {
        if warning.check_name.contains("Working Hours") {
            println!("⚠️ {}", warning.description);
            println!("📋 アクション: 第36条協定の届出");
        }
    }
}

// 出力:
// ⚠️ 1日の労働時間9時間が法定8時間を超過。第36条協定が必要です。
// 📋 アクション: 第36条協定の届出
```

## コンプライアンスチェックリスト

### 第36条協定届出前

- [ ] 労働者代表の選出 (従業員の過半数代表)
- [ ] 時間外労働の上限を明記
- [ ] 時間外労働の理由を記載
- [ ] 有効期間を設定
- [ ] 特別条項の正当性を確認
- [ ] 健康確保措置を含める

### 届出後

- [ ] 協定を職場に掲示 (社内掲示)
- [ ] 従業員への周知
- [ ] 勤怠管理システムを更新
- [ ] 月次チェック体制の確立
- [ ] 年次見直しのスケジュール設定

## 法的罰則

**第36条違反**:

| 違反 | 罰則 |
|------|------|
| 協定の未届出 | 6ヶ月以下の懲役または30万円以下の罰金 |
| 上限超過 | 6ヶ月以下の懲役または30万円以下の罰金 |
| 強制的な時間外労働 | 労働基準法違反 |

## ベストプラクティス

### 1. 保守的な上限設定

```rust
// ✓ 良い: 標準上限より低く設定
max_overtime_per_month: 40,  // 5時間のバッファ
max_overtime_per_year: 300,  // 60時間のバッファ

// ⚠️ リスク: 正確な上限
max_overtime_per_month: 45,  // エラーの余地なし
max_overtime_per_year: 360,
```

### 2. 具体的な理由

```rust
// ✓ 良い: 具体的、文書化された理由
permitted_reasons: vec![
    "年度末決算業務対応（3月）".to_string(),
    "システム更改作業（土日）".to_string(),
]

// ❌ 悪い: 曖昧、継続的な理由
permitted_reasons: vec![
    "業務多忙のため".to_string(),  // 曖昧すぎる
    "常時必要".to_string(),        // 体系的な問題を示唆
]
```

### 3. 定期的な見直し

```rust
// 年次見直しを強制するため有効期限を設定
expiration_date: one_year_from_now,

// 無期限はNG
expiration_date: far_future, // ❌ 悪い慣行
```

## テスト

```rust
#[test]
fn test_standard_agreement() {
    let agreement = Article36Agreement {
        // ... 標準設定
        max_overtime_per_month: 45,
        max_overtime_per_year: 360,
        has_special_circumstances: false,
        // ...
    };

    assert!(agreement.is_within_standard_limits());
    assert!(agreement.validate().is_ok());
}

#[test]
fn test_special_circumstances() {
    let agreement = Article36Agreement {
        // ... 特別条項あり
        has_special_circumstances: true,
        special_max_per_month: Some(80),
        special_months_per_year: Some(6),
        // ...
    };

    assert!(agreement.is_special_circumstances_valid());
}
```

## まとめ

Legalis-JPにおける第36条協定:

✅ **適用** 標準上限 (月45時間、年360時間)
✅ **検証** 特別条項 (月100時間、年6ヶ月)
✅ **要求** 理由の文書化
✅ **確認** 有効期間
✅ **検出** 協定が必要なタイミング

本システムは、労働者保護を維持しながら、時間外労働が日本の労働法に準拠することを保証します。

## 参考文献

- 労働基準法第36条 (労働基準法第36条)
- 厚生労働省告示第316号 (告示第316号)
- 働き方改革関連法 2019年 (働き方改革関連法)
