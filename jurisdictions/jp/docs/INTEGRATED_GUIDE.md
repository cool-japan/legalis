# 統合ガイド：4条文の総合的活用 - Version 0.1.1

## 📋 概要

このガイドでは、**Articles 709, 710, 715, 415** の4つの条文を総合的に活用する方法を解説します。

実務では、一つの事案に複数の条文が適用されることが多く、それぞれの条文の関係性を理解することが重要です。

### 本ガイドで学べること

1. **4条文の関係性と使い分け**
2. **実務的な条文選択戦略**
3. **統合事例の詳細分析**
4. **請求先の選択と求償関係**
5. **契約法と不法行為法の交錯**

---

## ⚖️ 4条文の関係マップ

```
民法の体系
├── 不法行為法（Tort Law）
│   ├── Article 709: 一般不法行為（基本規定）★
│   ├── Article 710: 非財産的損害（慰謝料）★
│   └── Article 715: 使用者責任（代位責任）★
└── 債権法（Obligations Law）
    └── 契約（Contract）
        └── Article 415: 債務不履行（契約違反）★
```

### 条文間の依存関係

```
Article 709 ──前提──→ Article 710
    ↓              (慰謝料は709成立が前提)
    前提
    ↓
Article 715
(使用者責任は従業員の709成立が前提)

Article 415 ←── 選択 ──→ Article 709
(契約当事者間では請求権競合)
```

---

## 🔍 条文の使い分け

### Article 709 vs Article 415（不法行為 vs 契約違反）

| 判断基準 | Article 709（不法行為） | Article 415（契約違反） |
|---------|---------------------|---------------------|
| **契約関係** | 不要 | 必要 |
| **適用対象** | すべての人 | 契約当事者のみ |
| **帰責要件** | 故意・過失 | 帰責事由（より広い） |
| **立証責任** | 原告が故意・過失を立証 | 被告が免責を立証 |
| **損害範囲** | 相当因果関係 | 予見可能性 |
| **消滅時効** | 3年/20年 | 5年/10年 |

**選択の基本原則：**
- **契約当事者間** → まずArticle 415を検討
- **第三者** → Article 709のみ適用可能
- **併用可能** → 有利な方を選択

### Article 709 + Article 710（財産的 + 非財産的）

Article 709で不法行為が成立すれば、Article 710で慰謝料を**追加**請求できます。

```
損害の内訳
├── 財産的損害（Article 709）
│   ├── 治療費
│   ├── 休業損害
│   └── 物損
└── 非財産的損害（Article 710）
    ├── 精神的苦痛
    ├── 身体的苦痛
    └── 名誉感情の侵害
```

### Article 709 + Article 715（従業員 + 使用者）

従業員の不法行為について、被害者は**選択的に**請求できます：

```
被害者の選択
├── 従業員個人に請求（Article 709）
│   → 資力不足のリスク
└── 使用者に請求（Article 715）
    → 資力あり（深いポケット）
```

**実務の鉄則：** 被害者は資力のある使用者に請求するのが一般的

---

## 📚 統合事例：レストラン配達事故

### 事案の概要

**シチュエーション：**
レストラン「和食亭」が配送業務を外部の配送会社「クイック配送」に委託。配達員が配達中に交通事故を起こし、歩行者に重傷を負わせた。また、注文客への配達も2時間遅延した。

**登場人物：**
1. **配達員（田中）** - クイック配送の契約社員
2. **配送会社（クイック配送）** - 田中の使用者
3. **レストラン（和食亭）** - 配送を委託
4. **被害者A（歩行者）** - 交通事故で負傷
5. **被害者B（注文客）** - 配達遅延でパーティー中止

---

## 🔎 Step-by-Step 法的分析

### STEP 1: Article 709（配達員の不法行為責任）

**要件確認：**

```rust
use legalis_jp::tort::{Article709, Intent, Damage, CausalLink, ProtectedInterest};

let driver_tort = Article709::new()
    .with_act("信号無視により横断歩道の歩行者をはねた")
    .with_intent(Intent::NegligenceWithDuty {
        duty_of_care: "信号遵守義務・前方注視義務違反".to_string()
    })
    .with_victim_interest(ProtectedInterest::BodyAndHealth)
    .with_damage(Damage::new(3_000_000, "治療費 + 入院費"))
    .with_causal_link(CausalLink::Direct);
```

**結論：**
✅ 配達員（田中）の Article 709 不法行為責任成立
- 損害額：¥3,000,000（治療費等）

---

### STEP 2: Article 710（非財産的損害・慰謝料）

Article 709が成立したので、Article 710で慰謝料を追加請求：

```rust
use legalis_jp::tort::{Article710, NonPecuniaryDamageType, HarmSeverity};

let consolation_claim = Article710::new()
    .with_article_709(driver_tort.clone())
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Severe)
    .emotional_distress("骨折による2ヶ月入院、痛みと精神的苦痛、後遺症への不安");
```

**結論：**
✅ Article 710 に基づく慰謝料請求成立
- 推奨慰謝料額：¥1,500,000

**被害者Aが請求できる合計額：**
```
財産的損害（Article 709）: ¥3,000,000
慰謝料（Article 710）:      ¥1,500,000
────────────────────────────────
合計：                       ¥4,500,000
```

---

### STEP 3: Article 715（配送会社の使用者責任）

配達員の不法行為について、配送会社の責任を検討：

```rust
use legalis_jp::tort::{Article715, EmploymentType};

let employer_liability = Article715::new()
    .employee_tort(driver_tort.clone())
    .employer("配送業者「クイック配送」")
    .employee("配達員 田中一郎")
    .employment_type(EmploymentType::Contract)
    .during_business_execution(true)
    .business_context("レストランからの委託配達業務中")
    .reasonable_care_appointment(false)
    .reasonable_care_supervision(false);
```

**判断要素：**
- ✅ 使用関係：契約社員（実質的な指揮監督あり）
- ✅ 事業執行性：配達業務中の事故
- ✅ 免責の抗弁：選任・監督の注意義務違反

**結論：**
✅ 配送会社の Article 715 使用者責任成立

**被害者Aの請求先選択：**
```
被害者A（歩行者）
├→ 配達員個人（709条+710条）¥4,500,000
└→ 配送会社（715条）        ¥4,500,000（連帯責任）

   💡 実務：資力のある配送会社に請求
```

---

### STEP 4: Article 415（レストランと注文客の契約違反）

事故により配達が2時間遅延し、注文客のパーティーに間に合わなかった：

```rust
use legalis_jp::contract::{Article415, Attribution, AttributionType, BreachType, ObligationType};

let contract_breach = Article415::new()
    .with_obligation(ObligationType::Service {
        description: "30分以内の配達サービス".to_string(),
        duration: Some("30分".to_string())
    })
    .with_breach(BreachType::DelayedPerformance { days_late: 0 })
    .with_attribution(Attribution::new(
        AttributionType::Negligence,
        "配達員の交通事故により遅延"
    ))
    .with_damage(Damage::new(50_000, "代替飲食費用 + パーティー中止損害"))
    .with_causal_link(CausalLink::Direct)
    .creditor("注文客 山田花子")
    .debtor("レストラン和食亭");
```

**5要件確認：**
1. ✅ 債務：30分配達サービス
2. ✅ 不履行：2時間遅延
3. ✅ 帰責事由：配達員事故（レストラン側リスク）
4. ✅ 因果関係：直接
5. ✅ 損害：¥50,000

**結論：**
✅ レストランの Article 415 債務不履行責任成立
- 損害額：¥50,000

**被害者Bの請求先：**
```
被害者B（注文客）
└→ レストラン和食亭（415条） ¥50,000
```

---

## 📊 総合的損害額と責任関係の整理

### 被害者別の請求先

#### 被害者A：歩行者（交通事故）

**請求先候補：**

| 請求先 | 法的根拠 | 請求額 | 備考 |
|-------|---------|--------|------|
| 配達員個人 | 709条+710条 | ¥4,500,000 | 資力不足の可能性 |
| 配送会社 | 715条 | ¥4,500,000 | 連帯責任、資力あり ✅ |

**実務的選択：** 配送会社に請求

#### 被害者B：注文客（配達遅延）

| 請求先 | 法的根拠 | 請求額 | 備考 |
|-------|---------|--------|------|
| レストラン | 415条 | ¥50,000 | 契約相手方 |

**ポイント：** 配送会社には直接請求できない（契約関係がない）

### 求償関係のフロー

```
【支払いの流れ】

1️⃣  配送会社が被害者Aに ¥4,500,000 支払い
     ↓
2️⃣  配送会社が配達員（田中）に求償
     （雇用契約・就業規則に基づく）
     ↓
3️⃣  レストランが被害者Bに ¥50,000 支払い
     ↓
4️⃣  レストランが配送会社に求償
     （委託契約に基づく）
     ↓
5️⃣  配送会社が配達員（田中）に求償
     （すべての損害の根本原因）
```

**最終的な負担：**
- 配達員（田中）: ¥4,550,000（全額）
- 配送会社: 一時的に支払うが、田中に求償
- レストラン: 一時的に支払うが、配送会社→田中に求償

---

## 💡 契約法と不法行為法の交錯

### 同じ事故が異なる法律関係を生む

```
         同じ配達事故
              ↓
    ┌─────────┴─────────┐
    │                         │
被害者A（歩行者）        被害者B（注文客）
    ↓                         ↓
不法行為責任              契約責任
(709+710+715条)           (415条)
    ↓                         ↓
配送会社に請求            レストランに請求
¥4,500,000                ¥50,000
```

**重要な教訓：**
- 契約関係の**ない**第三者 → 不法行為法
- 契約関係の**ある**当事者 → 契約法
- 同じ事故でも、被害者によって適用法が異なる

---

## 🎯 実務的な条文選択戦略

### ケース1: 医療ミス

**状況：** 医師の手術ミスで患者が後遺症

**選択肢：**
```
A. Article 415（診療契約の債務不履行）
   ✅ 帰責事由の立証が容易
   ✅ 医師側が免責を立証する必要

B. Article 709（医師の過失による不法行為）
   ⚠️  過失の立証が必要（原告側）

C. 併用（請求権競合）
   💡 実務では415条を主張、予備的に709条
```

### ケース2: 従業員の横領

**状況：** 従業員が会社の金を横領

**選択肢：**
```
A. Article 709（不法行為）
   ○ 適用可能

B. Article 415（雇用契約の債務不履行）
   ○ 適用可能（誠実義務違反）

C. 併用
   ✅ 実務ではどちらか有利な方
   💡 通常は刑事告訴も併用
```

### ケース3: 配送業務中の事故

**状況：** 配達員が配達中に交通事故

**選択肢：**
```
被害者（歩行者）の選択：
A. 配達員個人（Article 709）
   ⚠️  資力不足の可能性

B. 配送会社（Article 715）
   ✅ 資力あり、実務的に推奨

C. 併用（連帯債務）
   ○ 可能だが、通常はBのみ請求
```

---

## 📋 条文選択のデシジョンツリー

```
事案発生
  ↓
┌─ 契約関係あり？
│   YES → Article 415を検討
│          └→ 不法行為にも該当？
│              YES → 請求権競合、有利な方を選択
│              NO  → Article 415のみ
│   NO  → Article 709を検討
│          └→ 身体・名誉侵害？
│              YES → Article 710も検討（慰謝料）
│              NO  → Article 709のみ
└→ 加害者が従業員？
    YES → Article 715も検討（使用者責任）
           └→ 被害者は使用者に請求（深いポケット）
    NO  → 個人に請求
```

---

## 🔧 統合的なコード例

### 完全な事案処理

```rust
use legalis_jp::tort::{
    Article709, Article710, Article715,
    Intent, Damage, CausalLink, ProtectedInterest,
    NonPecuniaryDamageType, HarmSeverity, EmploymentType
};
use legalis_jp::contract::{
    Article415, Attribution, AttributionType, BreachType, ObligationType
};

fn comprehensive_case_analysis() {
    // Step 1: 従業員の不法行為（Article 709）
    let employee_tort = Article709::new()
        .with_act("配達中の交通事故")
        .with_intent(Intent::Negligence)
        .with_victim_interest(ProtectedInterest::BodyAndHealth)
        .with_damage(Damage::new(3_000_000, "治療費"))
        .with_causal_link(CausalLink::Direct);

    // Step 2: 慰謝料（Article 710）
    let consolation = Article710::new()
        .with_article_709(employee_tort.clone())
        .damage_type(NonPecuniaryDamageType::BodyAndHealth)
        .harm_severity(HarmSeverity::Severe)
        .build()
        .unwrap();

    // Step 3: 使用者責任（Article 715）
    let employer_claim = Article715::new()
        .employee_tort(employee_tort)
        .employer("配送会社")
        .employee("配達員")
        .employment_type(EmploymentType::Contract)
        .during_business_execution(true)
        .build()
        .unwrap();

    // Step 4: 契約違反（Article 415）
    let contract_breach = Article415::new()
        .with_obligation(ObligationType::Service {
            description: "30分配達".to_string(),
            duration: Some("30分".to_string())
        })
        .with_breach(BreachType::DelayedPerformance { days_late: 0 })
        .with_attribution(Attribution::new(
            AttributionType::Negligence,
            "配達遅延"
        ))
        .with_damage(Damage::new(50_000, "パーティー中止"))
        .with_causal_link(CausalLink::Direct)
        .build()
        .unwrap();

    // 検証
    assert!(employee_tort.validate().is_ok());
    assert!(consolation.validate().is_ok());
    assert!(employer_claim.validate().is_ok());
    assert!(contract_breach.validate().is_ok());

    println!("すべての条文が成立しました！");
}
```

---

## ❓ FAQ

### Q1: 同じ事案で複数の条文を同時に適用できますか？

**A:** はい。以下のパターンがあります：

1. **Article 709 + 710**: 必ず併用（709成立なら710も検討）
2. **Article 709 + 715**: 従業員の不法行為なら併用
3. **Article 709 + 415**: 請求権競合（どちらか選択）

### Q2: Article 415と709、どちらが有利ですか？

**A:** ケースバイケースですが：

**Article 415が有利な場合：**
- 立証責任（被告が免責を立証）
- 契約内容が明確

**Article 709が有利な場合：**
- 消滅時効（事案による）
- 損害範囲（相当因果関係）

### Q3: 使用者責任（715条）で使用者が免責される可能性は？

**A:** 実務上、免責はほぼ認められません（5%未満）。

免責の立証責任は使用者側にあり、極めて高いハードルです。

### Q4: 被害者は複数の相手に重複して請求できますか？

**A:** できます（連帯債務）。ただし、二重取りはできません。

**例：** 配達員と配送会社の両方に¥450万円請求できるが、どちらか一方が全額支払えば、もう一方への請求権は消滅。

### Q5: 求償権はどのように行使しますか？

**A:** 内部関係（雇用契約、委託契約等）に基づいて求償します。

**例：** 配送会社が被害者に支払った後、配達員に対し雇用契約に基づいて求償。

---

## 📁 サンプルコード

完全な統合事例は以下のexampleプロジェクトを参照：

```bash
cd examples/minpo-integrated-tort-damages
cargo run
```

**実行内容：**
- レストラン配達事故の完全分析
- 4条文すべての連動
- 被害者別の請求先整理
- 求償関係のフロー

---

## 🔗 個別条文ガイドへのリンク

詳細は各条文のガイドを参照してください：

- [Article 709: 一般不法行為](./ARTICLE709_GUIDE.md)
- [Article 710: 非財産的損害（慰謝料）](./ARTICLE710_GUIDE.md)
- [Article 715: 使用者責任](./ARTICLE715_GUIDE.md)
- [Article 415: 債務不履行](./ARTICLE415_GUIDE.md)

---

## 📞 お問い合わせ・フィードバック

本ツールに関するご質問・ご要望は、GitHubリポジトリのIssueからお寄せください。

---

**Document Version:** 1.0 (for legalis-jp v0.1.1)
**Last Updated:** 2026-01-09
**Author:** Legalis-RS Project Team
