# 民法710条ビルダーAPI - 法律専門家向けガイド

## 📋 概要：何を作ったのか

**民法第710条（財産以外の損害の賠償＝慰謝料）の要件判定を、コンピュータで構造化・自動化するツール**を開発しました。

Article 709（不法行為の成立）を前提として、非財産的損害（精神的苦痛、名誉感情、身体的苦痛など）の賠償請求を構造的に処理できます。

従来、慰謝料の算定は経験則に依存していましたが、このシステムでは：
- **Article 709の成立を前提条件として確認**
- **被侵害利益の種類を明示**
- **被害程度に応じた慰謝料額の推奨**
- **過失相殺の自動計算**

が可能になります。

---

## ⚖️ 法的背景：民法710条とは

民法第710条は非財産的損害（慰謝料）の賠償を定める規定です：

> **民法第710条（財産以外の損害の賠償）**
>
> 他人の身体、自由若しくは名誉を侵害した場合又は他人の財産権を侵害した場合のいずれであるかを問わず、前条の規定により損害賠償の責任を負う者は、財産以外の損害に対しても、その賠償をしなければならない。

**English Translation:**
A person who is liable to compensate for damage under the provisions of the preceding Article (Article 709) shall compensate for non-pecuniary damage as well, regardless of whether the victim's body, liberty, reputation, or property rights were infringed.

### Article 709との関係（前提条件）

Article 710は**Article 709の成立を前提**とする規定です：

1. **まずArticle 709で不法行為責任を確立**
   - 故意・過失
   - 権利侵害
   - 因果関係
   - 損害発生

2. **その上でArticle 710により非財産的損害を追加請求**
   - 精神的苦痛（慰謝料）
   - 身体的苦痛
   - 名誉感情の侵害
   - プライバシー侵害

### 対象となる非財産的損害

1. **身体・健康侵害** (BodyAndHealth)
   - 交通事故による身体的苦痛
   - 医療ミスによる後遺症への不安
   - 暴行による精神的ショック

2. **名誉毀損** (ReputationDamage)
   - 誹謗中傷による社会的信用の低下
   - 虚偽の風評による精神的苦痛

3. **自由の侵害** (LibertyInfringement)
   - 不当な拘束による精神的苦痛
   - ストーカー行為による恐怖

4. **財産権侵害に伴う精神的苦痛** (PropertyRelatedDistress)
   - 思い出の品の破損による精神的苦痛
   - ペットの死亡による悲しみ

---

## 🔍 従来の問題点

### 1. 慰謝料額の算定基準の不透明性

慰謝料は「被害者の精神的苦痛を金銭的に評価する」という性質上、明確な基準がありません。

**例：** 同じ骨折でも、弁護士によって50万円～200万円と幅がある

### 2. Article 709との連動の見落とし

Article 710は単独では成立しません。Article 709の成立が前提です。

**リスク：** Article 709の立証が不十分なまま、慰謝料のみを主張して敗訴

### 3. 被害程度の評価の属人性

「重傷」「軽傷」などの評価が担当者の感覚に依存し、統一的な基準がありません。

---

## 💡 このシステムが提供するもの

### 1. **Article 709との自動連携**

```rust
use legalis_jp::tort::{Article709, Article710, Intent, Damage, CausalLink, ProtectedInterest};
use legalis_jp::tort::{NonPecuniaryDamageType, HarmSeverity};

// まずArticle 709を確立
let article_709_claim = Article709::new()
    .with_act("交通事故で歩行者に衝突")
    .with_intent(Intent::Negligence)
    .with_victim_interest(ProtectedInterest::BodyAndHealth)
    .with_damage(Damage::new(200_000, "治療費"))
    .with_causal_link(CausalLink::Direct);

// Article 710で慰謝料を追加請求
let article_710_claim = Article710::new()
    .with_article_709(article_709_claim)  // 前提条件として連結
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Moderate)
    .emotional_distress("継続的な痛みと精神的苦痛");
```

**Article 709の成立を前提条件として自動チェック**します。

### 2. **被害程度に応じた慰謝料額の自動推奨**

```rust
let recommended = article_710_claim.recommended_consolation_money();
// HarmSeverity::Moderate → ¥500,000
```

システムが被害の程度に応じて推奨額を算出：
- Minor（軽微） → ¥100,000
- Moderate（中程度） → ¥500,000
- Severe（重度） → ¥1,500,000
- Catastrophic（壊滅的） → ¥5,000,000

### 3. **過失相殺の自動計算**

```rust
let claim = Article710::new()
    // ...
    .victim_comparative_fault(30);  // 被害者の過失30%
```

被害者にも過失がある場合、慰謝料額を自動調整します。

---

## 📚 具体的な使用例

### ケース1：交通事故（基本的な慰謝料請求）

**事案：** 運転手Aが信号無視で歩行者Bをはね、Bは骨折で2週間入院。治療費20万円、慰謝料を請求したい。

```rust
use legalis_jp::tort::{
    Article709, Article710, Intent, Damage, CausalLink, ProtectedInterest,
    NonPecuniaryDamageType, HarmSeverity, validate_tort_claim
};

// Step 1: Article 709の確立
let article_709_claim = Article709::new()
    .with_act("信号無視で横断歩道の歩行者をはねた")
    .with_intent(Intent::NegligenceWithDuty {
        duty_of_care: "信号遵守義務違反".to_string()
    })
    .with_victim_interest(ProtectedInterest::BodyAndHealth)
    .with_damage(Damage::new(200_000, "治療費"))
    .with_causal_link(CausalLink::Direct);

// Step 2: Article 709の検証
match validate_tort_claim(&article_709_claim) {
    Ok(_) => {
        println!("Article 709成立");

        // Step 3: Article 710で慰謝料を追加請求
        let article_710_claim = Article710::new()
            .with_article_709(article_709_claim)
            .damage_type(NonPecuniaryDamageType::BodyAndHealth)
            .harm_severity(HarmSeverity::Moderate)
            .emotional_distress("2週間の入院生活による精神的苦痛");

        match article_710_claim.validate() {
            Ok(_) => {
                let consolation = article_710_claim.recommended_consolation_money();
                println!("推奨慰謝料額: ¥{}", consolation);
                println!("合計請求額: ¥{}", 200_000 + consolation);
                // Output: 推奨慰謝料額: ¥500,000
                //         合計請求額: ¥700,000
            }
            Err(e) => println!("Article 710不成立: {:?}", e),
        }
    }
    Err(e) => println!("Article 709不成立: {:?}", e),
}
```

**結果：**
- 財産的損害（Article 709）: ¥200,000
- 慰謝料（Article 710）: ¥500,000
- **合計: ¥700,000**

---

### ケース2：名誉毀損（SNS誹謗中傷）

**事案：** AがSNSでBを誹謗中傷。Bは社会的信用を失い、精神的ショックで不眠症に。弁護士費用10万円、慰謝料を請求。

```rust
use legalis_jp::tort::{
    Article709, Article710, Intent, Damage, CausalLink, ProtectedInterest,
    NonPecuniaryDamageType, HarmSeverity
};

// Article 709: 名誉毀損の不法行為
let defamation_tort = Article709::new()
    .with_act("SNSで虚偽の情報を拡散し被害者の名誉を毀損")
    .with_intent(Intent::Intentional { age: 25 })
    .with_victim_interest(ProtectedInterest::Reputation)
    .with_damage(Damage::new(100_000, "弁護士費用"))
    .with_causal_link(CausalLink::Direct);

// Article 710: 非財産的損害（名誉感情の侵害）
let consolation_claim = Article710::new()
    .with_article_709(defamation_tort)
    .damage_type(NonPecuniaryDamageType::ReputationDamage)
    .harm_severity(HarmSeverity::Moderate)
    .emotional_distress("社会的信用の低下、精神的ショック、不眠");

match consolation_claim.validate() {
    Ok(_) => {
        let amount = consolation_claim.recommended_consolation_money();
        println!("財産的損害: ¥100,000");
        println!("慰謝料: ¥{}", amount);
        println!("合計: ¥{}", 100_000 + amount);
        // Output: 財産的損害: ¥100,000
        //         慰謝料: ¥500,000
        //         合計: ¥600,000
    }
    Err(e) => println!("検証失敗: {:?}", e),
}
```

**ポイント：**
- 名誉毀損の場合、財産的損害は少額（弁護士費用など）
- しかし、Article 710により精神的苦痛に対する慰謝料を別途請求可能
- 合計額は財産的損害の6倍に

---

### ケース3：重傷事故（詳細な損害計算）

**事案：** 車両事故で被害者が骨折、3ヶ月入院。治療費200万円、入院費150万円、休業損害150万円。後遺症への不安が強い。

```rust
use legalis_jp::tort::{
    Article709, Article710, Intent, Damage, CausalLink, ProtectedInterest,
    NonPecuniaryDamageType, HarmSeverity
};

// Article 709: 財産的損害
let serious_accident = Article709::new()
    .with_act("赤信号無視で横断中の歩行者をはねた")
    .with_intent(Intent::NegligenceWithDuty {
        duty_of_care: "信号遵守義務・前方注視義務違反".to_string()
    })
    .with_victim_interest(ProtectedInterest::BodyAndHealth)
    .with_damage(Damage::new(5_000_000, "治療費 + 入院費 + 休業損害"))
    .with_causal_link(CausalLink::Direct);

// Article 710: 重度の非財産的損害
let severe_consolation = Article710::new()
    .with_article_709(serious_accident)
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Severe)  // 重度
    .emotional_distress("3ヶ月間の入院生活による精神的苦痛、後遺症への不安")
    .consolation_money(1_500_000);  // 明示的に指定

println!("財産的損害の内訳:");
println!("  治療費: ¥2,000,000");
println!("  入院費: ¥1,500,000");
println!("  休業損害: ¥1,500,000");
println!("  小計: ¥5,000,000");
println!();
println!("非財産的損害:");
println!("  慰謝料: ¥1,500,000");
println!();
println!("合計請求額: ¥6,500,000");
```

**実務的ポイント：**
- 重傷事故では財産的損害（治療費等）と非財産的損害（慰謝料）を明確に分けて算定
- 入院慰謝料は日数×基準額で計算するのが実務だが、システムが自動推奨
- 後遺障害が残る場合は別途後遺障害慰謝料も検討

---

## 🔧 API リファレンス

### Article710 Builder Methods

#### `Article710::new() -> Self`
新しいArticle 710請求を作成します。

```rust
let claim = Article710::new();
```

#### `.with_article_709(claim: Article709<'a>) -> Self`
**必須**：Article 709の不法行為成立を前提条件として設定します。

```rust
.with_article_709(article_709_claim)
```

#### `.damage_type(dtype: NonPecuniaryDamageType) -> Self`
**必須**：非財産的損害の種類を設定します。

```rust
.damage_type(NonPecuniaryDamageType::BodyAndHealth)
```

**NonPecuniaryDamageType の種類：**
- `BodyAndHealth` - 身体・健康侵害
- `ReputationDamage` - 名誉毀損
- `LibertyInfringement` - 自由の侵害
- `PropertyRelatedDistress` - 財産権侵害に伴う精神的苦痛

#### `.harm_severity(severity: HarmSeverity) -> Self`
**必須**：被害の程度を設定します。

```rust
.harm_severity(HarmSeverity::Moderate)
```

**HarmSeverity の種類：**
- `Minor` - 軽微（推奨額: ¥100,000）
- `Moderate` - 中程度（推奨額: ¥500,000）
- `Severe` - 重度（推奨額: ¥1,500,000）
- `Catastrophic` - 壊滅的（推奨額: ¥5,000,000）

#### `.emotional_distress(description: impl Into<String>) -> Self`
精神的苦痛の内容を説明します。

```rust
.emotional_distress("継続的な痛みと精神的苦痛")
```

#### `.consolation_money(amount: u64) -> Self`
慰謝料額を明示的に指定します（省略可、省略時は自動推奨）。

```rust
.consolation_money(1_500_000)
```

#### `.victim_comparative_fault(percentage: u8) -> Self`
被害者の過失割合（0-100%）を設定します。

```rust
.victim_comparative_fault(30)  // 被害者30%の過失
```

#### `.build() -> Result<Article710<'a>, TortClaimError>`
必須フィールドがすべて設定されているか確認してビルドします。

```rust
let claim = Article710::new()
    .with_article_709(tort)
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Moderate)
    .build()?;
```

#### `.validate() -> Result<(), ValidationError>`
Article 710の要件をすべて満たしているか検証します。

```rust
match claim.validate() {
    Ok(_) => println!("慰謝料請求成立"),
    Err(e) => println!("不成立: {:?}", e),
}
```

#### `.recommended_consolation_money() -> u64`
被害の程度に基づいて推奨慰謝料額を返します。

```rust
let amount = claim.recommended_consolation_money();
println!("推奨額: ¥{}", amount);
```

---

## 📊 慰謝料額の算定ガイドライン

### 交通事故の入院慰謝料（裁判所基準）

| 入院期間 | 推奨慰謝料額 | HarmSeverity |
|---------|-------------|--------------|
| 1ヶ月 | ¥530,000 | Moderate |
| 2ヶ月 | ¥1,010,000 | Moderate |
| 3ヶ月 | ¥1,450,000 | Severe |
| 6ヶ月 | ¥2,440,000 | Severe |
| 1年 | ¥3,770,000 | Catastrophic |

### 後遺障害慰謝料

| 後遺障害等級 | 推奨慰謝料額 | HarmSeverity |
|-------------|-------------|--------------|
| 14級 | ¥1,100,000 | Moderate |
| 12級 | ¥2,900,000 | Severe |
| 9級 | ¥6,900,000 | Severe |
| 7級 | ¥10,000,000 | Catastrophic |
| 1級 | ¥28,000,000 | Catastrophic |

### 名誉毀損

| 被害内容 | 推奨慰謝料額 | HarmSeverity |
|---------|-------------|--------------|
| 個人間の口頭誹謗 | ¥100,000 | Minor |
| SNS誹謗中傷 | ¥500,000 | Moderate |
| マスコミ報道 | ¥1,500,000 | Severe |
| 社会的地位の喪失 | ¥5,000,000+ | Catastrophic |

---

## 🎯 実務上のTips

### 1. Article 709との一体的立証

Article 710は単独では成立しません。必ずArticle 709の成立を先に確認してください。

```rust
// ❌ 間違い：Article 709なしでArticle 710のみ
let claim = Article710::new()
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Moderate);
// → build()時にエラー

// ✅ 正しい：Article 709を前提条件として設定
let claim = Article710::new()
    .with_article_709(article_709_claim)  // 必須
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Moderate);
```

### 2. 慰謝料額の明示 vs 自動推奨

システムは被害程度に応じて推奨額を算出しますが、実際の事案に応じて調整が必要です。

```rust
// 自動推奨を使う
let claim = Article710::new()
    .with_article_709(tort)
    .harm_severity(HarmSeverity::Moderate);
let amount = claim.recommended_consolation_money();  // ¥500,000

// 明示的に指定する
let claim = Article710::new()
    .with_article_709(tort)
    .harm_severity(HarmSeverity::Moderate)
    .consolation_money(800_000);  // 実情に応じて調整
```

### 3. 過失相殺の適用

被害者にも過失がある場合、慰謝料額を減額する必要があります。

```rust
let claim = Article710::new()
    .with_article_709(tort)
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Moderate)
    .victim_comparative_fault(30);  // 被害者30%過失

// 慰謝料 ¥500,000 → 30%減額 → ¥350,000
```

---

## 🔗 関連条文との関係

### Article 709（不法行為の一般規定）
Article 710の**前提条件**です。必ずArticle 709の成立を先に確認してください。

→ [ARTICLE709_GUIDE.md](./ARTICLE709_GUIDE.md) 参照

### Article 715（使用者責任）
従業員の不法行為について、使用者（雇用主）が連帯責任を負います。Article 710の慰謝料も使用者に請求可能です。

→ [ARTICLE715_GUIDE.md](./ARTICLE715_GUIDE.md) 参照

### Article 415（債務不履行）
契約違反の場合、不法行為（Article 709+710）ではなく、契約責任（Article 415）を検討することもあります。

→ [ARTICLE415_GUIDE.md](./ARTICLE415_GUIDE.md) 参照

---

## 📁 サンプルコード

完全な動作例は以下のexampleプロジェクトを参照してください：

```bash
cd examples/minpo-710-damages-builder
cargo run
```

**実行内容：**
1. 基本的な慰謝料請求（交通事故）
2. 詳細な損害計算（重傷事故）
3. 比較分析（709単独 vs 709+710）

---

## ❓ FAQ

### Q1: Article 709が成立しない場合、Article 710も請求できませんか？

**A:** はい。Article 710は「前条（Article 709）の規定により損害賠償の責任を負う者」が対象です。Article 709が成立しなければ、Article 710も成立しません。

### Q2: 財産権侵害でも慰謝料を請求できますか？

**A:** はい。Article 710は「財産権を侵害した場合のいずれであるかを問わず」と明記しています。ただし、財産権侵害の場合、非財産的損害（精神的苦痛）の立証が困難なケースが多いです。

**例：** ペットの死亡（財産権侵害）→ 飼い主の悲しみ（精神的苦痛）→ 慰謝料請求可能

### Q3: 推奨慰謝料額は裁判所の基準ですか？

**A:** 推奨額は一般的な裁判例の傾向を参考にしていますが、個別の事案によって異なります。あくまで**目安**としてご利用ください。

### Q4: 後遺障害がある場合の慰謝料は？

**A:** 入院慰謝料と後遺障害慰謝料は別途計算します。本システムでは両方を合算して`consolation_money`として指定できます。

```rust
// 入院慰謝料 ¥1,000,000 + 後遺障害慰謝料（14級）¥1,100,000
.consolation_money(2_100_000)
```

---

## 📞 お問い合わせ・フィードバック

本ツールに関するご質問・ご要望は、GitHubリポジトリのIssueからお寄せください。

---

**Document Version:** 1.0
**Last Updated:** 2026-01-09
**Author:** Legalis-RS Project Team
