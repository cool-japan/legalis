# 民法415条ビルダーAPI - 法律専門家向けガイド

## 📋 概要：何を作ったのか

**民法第415条（債務不履行による損害賠償）の要件判定を、コンピュータで構造化・自動化するツール**を開発しました。

契約当事者間の債務不履行（契約違反）による損害賠償請求を構造的に処理できます。

このシステムでは：
- **債務の存在の確認**
- **不履行（履行遅滞・不完全履行・履行不能）の判定**
- **帰責事由（債務者の責任）の評価**
- **因果関係と損害の確認**
- **予見可能性による損害範囲の限定**

が可能になります。

---

## ⚖️ 法的背景：民法415条とは

民法第415条は契約違反（債務不履行）による損害賠償を定める規定です：

> **民法第415条（債務不履行による損害賠償）**
>
> 債務者がその債務の本旨に従った履行をしないときは、債権者は、これによって生じた損害の賠償を請求することができる。ただし、その債務の不履行が契約その他の債務の発生原因及び取引上の社会通念に照らして債務者の責めに帰することができない事由によるものであるときは、この限りでない。

**English Translation:**
If an obligor fails to perform the obligation in accordance with the purpose thereof, the obligee may request damages arising therefrom; provided, however, that this does not apply if the non-performance is due to grounds not attributable to the obligor in light of the contract or other sources of obligation and common sense in transactions.

### 不法行為（Article 709）との違い

Article 415とArticle 709の主な違い：

| 項目 | Article 415（契約責任） | Article 709（不法行為） |
|------|------------------------|----------------------|
| **前提** | 契約関係の存在 | 契約不要（一般的義務違反） |
| **要件** | 5要件 | 4要件 |
| **帰責事由** | 「帰責事由」（債務者の責任） | 「故意・過失」（主観的要素） |
| **立証責任** | 債権者が不履行を立証、債務者が免責を立証 | 債権者が故意・過失を立証 |
| **損害範囲** | 予見可能な損害 | 相当因果関係の範囲 |
| **消滅時効** | 5年または10年 | 3年または20年 |

### 成立要件（5要件）

1. **債務の存在**
   - 有効な契約に基づく債務があること

2. **債務の不履行**
   - 履行遅滞（遅れ）
   - 履行不能（不可能）
   - 不完全履行（不十分・欠陥）

3. **帰責事由**
   - 債務者の責めに帰すべき事由があること
   - 故意・過失は不要（Article 709との違い）

4. **因果関係**
   - 不履行と損害の因果関係

5. **損害の発生**
   - 実際の損害が存在すること

### ただし書（免責事由）

債務者は以下を立証すれば免責されます：

- 契約や取引上の社会通念に照らして
- 債務者の責めに帰することができない事由による不履行

**例：**
- 天災地変（不可抗力）
- 第三者の行為
- 債権者の協力不足

---

## 🔍 従来の問題点

### 1. 不法行為との混同

契約当事者間でも、Article 709（不法行為）とArticle 415（債務不履行）のどちらを適用すべきか混乱します。

**例：** 医療ミス
- 契約責任（415条）: 診療契約の債務不履行
- 不法行為（709条）: 医師の過失による身体侵害

### 2. 損害範囲の不明確性

「予見可能性」による損害範囲の限定が不明確です。

**例：** 配送遅延で工場停止
- 直接損害（予見可能）: 追加配送費 → 認容
- 間接損害（予見困難？）: 工場停止損害 → ？

### 3. 損害軽減義務の適用

債権者にも「損害を拡大しない義務」があるが、その範囲が不明確です。

**例：** 賃貸契約の中途解約
- 家主は新賃借人を探す義務があるか？
- 何もせずに全期間分を請求できるか？

---

## 💡 このシステムが提供するもの

### 1. **5要件の構造的チェック**

```rust
use legalis_jp::contract::{Article415, Attribution, AttributionType, BreachType, ObligationType};
use legalis_jp::tort::{Damage, CausalLink};

let claim = Article415::new()
    .with_obligation(ObligationType::Delivery {
        description: "コンピュータ機器10台の引渡".to_string()
    })
    .with_breach(BreachType::NonPerformance)
    .with_attribution(Attribution::new(
        AttributionType::Negligence,
        "正当な理由なく引渡しを拒否"
    ))
    .with_damage(Damage::new(5_000_000, "代替品購入費用"))
    .with_causal_link(CausalLink::Direct)
    .creditor("株式会社ABC")
    .debtor("供給業者XYZ");
```

各要件を一つずつ明示的に設定し、漏れを防止します。

### 2. **債務の種類の明確化**

```rust
pub enum ObligationType {
    Monetary { amount: u64, currency: String },  // 金銭債務
    Delivery { description: String },            // 引渡債務
    Service { description: String, duration: Option<String> },  // 役務提供債務
    Other(String),                               // その他
}
```

様々な債務の種類に対応します。

### 3. **不履行の類型化**

```rust
pub enum BreachType {
    NonPerformance,                             // 履行拒絶
    DelayedPerformance { days_late: u32 },     // 履行遅滞
    DefectivePerformance { description: String },  // 不完全履行
}
```

不履行の種類を明確に分類します。

---

## 📚 具体的な使用例

### ケース1：商品未引渡（基本的な債務不履行）

**事案：** 売主が契約した商品10台を引き渡さず、買主は代替品を500万円で購入した。

```rust
use legalis_jp::contract::{
    Article415, Attribution, AttributionType, BreachType, ObligationType
};
use legalis_jp::tort::{Damage, CausalLink};

let claim = Article415::new()
    .with_obligation(ObligationType::Delivery {
        description: "コンピュータ機器10台の引渡".to_string()
    })
    .with_breach(BreachType::NonPerformance)
    .with_attribution(Attribution::new(
        AttributionType::Negligence,
        "正当な理由なく引渡しを拒否"
    ))
    .with_damage(Damage::new(5_000_000, "代替品購入費用"))
    .with_causal_link(CausalLink::Direct)
    .creditor("株式会社ABC")
    .debtor("供給業者XYZ");

match claim.build() {
    Ok(breach_claim) => {
        println!("✅ Article 415の5要件をすべて充足");
        println!();
        println!("【5要件の確認】:");
        println!("1. 債務の存在: ✅ 商品引渡債務");
        println!("2. 不履行: ✅ 履行拒絶");
        println!("3. 帰責事由: ✅ 過失（正当理由なく拒否）");
        println!("4. 因果関係: ✅ 直接因果関係");
        println!("5. 損害: ✅ 代替品購入費用 ¥5,000,000");
        println!();

        match breach_claim.validate() {
            Ok(_) => {
                println!("✅ 債務不履行責任成立");
                println!("推定損害額: ¥{}", breach_claim.estimated_damages());
            }
            Err(e) => println!("検証失敗: {:?}", e),
        }
    }
    Err(e) => println!("ビルドエラー: {:?}", e),
}
```

**結果：**
- 売主に債務不履行責任あり
- 損害額：¥5,000,000

---

### ケース2：配送遅延と予見可能性（Hadley原則）

**事案：** 配送業者が重要部品の配送を7日遅延。製造会社の工場が停止し、1000万円の損失。

```rust
use legalis_jp::contract::{
    Article415, Attribution, AttributionType, BreachType, ObligationType
};
use legalis_jp::tort::{Damage, CausalLink};

// 間接損害（予見可能性が問題）
let indirect_claim = Article415::new()
    .with_obligation(ObligationType::Delivery {
        description: "重要部品の納期までの配送".to_string()
    })
    .with_breach(BreachType::DelayedPerformance { days_late: 7 })
    .with_attribution(Attribution::new(
        AttributionType::Negligence,
        "配送手配を怠った"
    ))
    .with_damage(Damage::new(10_000_000, "工場操業停止による逸失利益"))
    .with_causal_link(CausalLink::Adequate(
        "部品なしで工場が停止、契約時に通知済み"
    ))
    .creditor("製造会社A")
    .debtor("配送業者B");

match indirect_claim.build() {
    Ok(claim) => {
        println!("📊 予見可能性の判断:");
        println!();
        println!("判断要素:");
        println!("• 契約時に部品の重要性を通知していたか？");
        println!("• 遅延で工場停止することは当然予見可能か？");
        println!();
        println!("Hadley v. Baxendale原則:");
        println!("「契約時に通常予見できた損害」または");
        println!("「当事者が特別の事情を知っていた場合の損害」のみ賠償");
        println!();

        match claim.validate() {
            Ok(_) => {
                println!("✅ 債務不履行責任成立");
                println!();
                println!("結論:");
                println!("直接損害（追加配送費）: 確実に認容");
                println!("間接損害（工場停止損害）: 通知の有無で判断が分かれる");
                println!();
                println!("💡 実務的アドバイス:");
                println!("契約時に「遅延の場合の損害」を明示しておくべき");
            }
            Err(e) => println!("検証失敗: {:?}", e),
        }
    }
    Err(e) => println!("ビルドエラー: {:?}", e),
}
```

**Hadley v. Baxendale原則（予見可能性ルール）：**

契約違反で賠償すべき損害は以下に限定：
1. **通常損害**: 契約時に通常予見できた損害
2. **特別損害**: 当事者が特別の事情を知っていた場合の損害

**実務上の対策：**
契約書に「遅延の場合、1日あたり○○円の損害が発生する」と明記する。

---

### ケース3：損害軽減義務（賃貸契約の中途解約）

**事案：** 賃借人が2年契約の賃貸借を1年で一方的に解約。家主は残り1年分（120万円）を請求したい。

```rust
use legalis_jp::contract::{
    Article415, Attribution, AttributionType, BreachType, ObligationType
};
use legalis_jp::tort::{Damage, CausalLink};

let breach_claim = Article415::new()
    .with_obligation(ObligationType::Monetary {
        amount: 100_000,
        currency: "JPY".to_string()
    })
    .with_breach(BreachType::NonPerformance)
    .with_attribution(Attribution::new(
        AttributionType::Intentional,
        "一方的に契約解除を通告"
    ))
    .with_damage(Damage::new(1_200_000, "残期間12ヶ月分の家賃"))
    .with_causal_link(CausalLink::Direct)
    .creditor("家主")
    .debtor("賃借人")
    .contract_date("2025-01-01");

match breach_claim.build() {
    Ok(claim) => {
        println!("✅ Article 415債務不履行成立");
        println!();
        println!("しかし...");
        println!();
        println!("🔍 損害軽減義務の検討:");
        println!();
        println!("債権者（家主）の義務:");
        println!("• 新たな賃借人を探す努力をすべき");
        println!("• 空室期間を合理的に短縮する義務");
        println!();
        println!("シナリオA: 家主が何もしない場合");
        println!("→ 全額 ¥1,200,000 の請求は認められない可能性");
        println!();
        println!("シナリオB: 家主が募集し3ヶ月後に新賃借人");
        println!("→ 認容額: ¥300,000（3ヶ月分のみ）");
        println!();
        println!("💡 Legal Principle:");
        println!("判例法理: 「債権者も損害の拡大を防ぐ義務を負う」");
        println!();
        println!("比較法:");
        println!("• 英米法: Duty to mitigate（明文化）");
        println!("• 日本法: 信義則（民法1条2項）から導出");
    }
    Err(e) => println!("ビルドエラー: {:?}", e),
}
```

**損害軽減義務の実務：**

債権者（被害者）には、損害を不当に拡大させない義務があります。

**具体例：**
- 賃貸物件: 新賃借人を探す
- 雇用契約: 新しい職を探す
- 売買契約: 代替取引を探す

**違反の効果：**
軽減できたはずの損害部分は賠償請求できません。

---

## 🔧 API リファレンス

### Article415 Builder Methods

#### `Article415::new() -> Self`
新しいArticle 415請求を作成します。

```rust
let claim = Article415::new();
```

#### `.with_obligation(obligation: ObligationType) -> Self`
**必須**：債務の内容を設定します。

```rust
.with_obligation(ObligationType::Delivery {
    description: "商品10台の引渡".to_string()
})
```

**ObligationType の種類：**
- `Monetary { amount, currency }` - 金銭債務
- `Delivery { description }` - 引渡債務
- `Service { description, duration }` - 役務提供債務
- `Other(String)` - その他

#### `.with_breach(breach: BreachType) -> Self`
**必須**：不履行の種類を設定します。

```rust
.with_breach(BreachType::NonPerformance)
```

**BreachType の種類：**
- `NonPerformance` - 履行拒絶
- `DelayedPerformance { days_late }` - 履行遅滞
- `DefectivePerformance { description }` - 不完全履行

#### `.with_attribution(attribution: Attribution) -> Self`
**必須**：帰責事由を設定します。

```rust
.with_attribution(Attribution::new(
    AttributionType::Negligence,
    "正当な理由なく履行を拒否"
))
```

**AttributionType の種類：**
- `Intentional` - 故意
- `Negligence` - 過失
- `StrictLiability` - 無過失責任

#### `.with_causal_link(link: CausalLink<'a>) -> Self`
**必須**：因果関係を設定します。

```rust
.with_causal_link(CausalLink::Direct)
```

#### `.with_damage(damage: Damage) -> Self`
**必須**：損害を設定します。

```rust
.with_damage(Damage::new(5_000_000, "代替品購入費用"))
```

#### `.creditor(name: impl Into<String>) -> Self`
債権者（権利者）を設定します。

```rust
.creditor("株式会社ABC")
```

#### `.debtor(name: impl Into<String>) -> Self`
債務者（義務者）を設定します。

```rust
.debtor("供給業者XYZ")
```

#### `.contract_date(date: impl Into<String>) -> Self`
契約日を設定します（オプション）。

```rust
.contract_date("2025-01-01")
```

#### `.with_due_date(date: impl Into<String>) -> Self`
履行期日を設定します（オプション）。

```rust
.with_due_date("2026-01-10")
```

#### `.build() -> Result<Article415<'a>, ContractLiabilityError>`
必須フィールドがすべて設定されているか確認してビルドします。

```rust
let claim = Article415::new()
    .with_obligation(obligation)
    .with_breach(breach)
    .with_attribution(attribution)
    .with_damage(damage)
    .with_causal_link(link)
    .build()?;
```

#### `.validate() -> Result<(), ContractLiabilityError>`
Article 415の要件をすべて満たしているか検証します。

```rust
match claim.validate() {
    Ok(_) => println!("債務不履行責任成立"),
    Err(e) => println!("不成立: {:?}", e),
}
```

#### `.estimated_damages() -> u64`
推定損害額を返します。

```rust
let amount = claim.estimated_damages();
println!("推定損害額: ¥{}", amount);
```

---

## 📊 契約責任 vs 不法行為責任

### どちらを選択すべきか？

| 状況 | Article 415（契約） | Article 709（不法行為） |
|------|-------------------|---------------------|
| **契約当事者間** | ○ 原則こちら | △ 併用可能 |
| **第三者** | × 適用不可 | ○ こちら |
| **医療ミス** | ○ 診療契約違反 | ○ 過失による侵害（併用） |
| **交通事故** | × 契約なし | ○ こちら |
| **商品の欠陥** | ○ 契約違反（売主） | ○ 製造物責任（併用） |

### 併用が可能なケース（請求権競合）

契約関係があり、かつ不法行為にも該当する場合、**両方の請求が可能**です（請求権競合）。

**例：** 医療ミス
- 契約責任（415条）: 診療契約の債務不履行
- 不法行為（709条）: 医師の過失による身体侵害

被害者はどちらか有利な方を選択できます。

**選択のポイント：**
- 立証責任の違い
- 消滅時効の違い
- 損害範囲の違い

---

## 🎯 実務上のTips

### 1. 契約書への明記

損害の予見可能性を確保するため、契約書に明記すべき事項：

```
【契約書の条項例】
第○条（損害賠償）
乙が本契約に違反した場合、甲に生じた以下の損害を賠償する：
1. 直接損害（代替取引費用、追加費用等）
2. 間接損害（本契約締結時に乙が知っていた特別の事情による損害）
3. 1日あたり金○○円の遅延損害金
```

### 2. 損害軽減義務の実践

債権者として、損害を不当に拡大させないこと：

```rust
// ❌ 何もせずに全額請求
.with_damage(Damage::new(1_200_000, "残期間全額"))

// ✅ 軽減努力後の実損害を請求
.with_damage(Damage::new(300_000, "新賃借人確保までの3ヶ月分"))
```

### 3. 免責条項の限界

契約書に免責条項があっても、以下は無効：

- 故意・重過失の免責
- 消費者契約における不当な免責
- 公序良俗違反

```rust
// こういう条項は無効の可能性
"当社は一切の責任を負わない"
```

---

## 🔗 関連条文との関係

### Article 414（履行の強制）
債務不履行に対し、まず「履行を強制」することも可能です。Article 415は履行に代わる損害賠償です。

### Article 416（損害賠償の範囲）
予見可能性による損害範囲の限定を定めています。

### Article 419（金銭債務の特則）
金銭債務の不履行には、不可抗力でも免責されません。

### Article 420（賠償額の予定）
違約金条項がある場合の扱いを定めています。

### Article 709（不法行為）
契約当事者間でも、不法行為責任との競合があります。

→ [ARTICLE709_GUIDE.md](./ARTICLE709_GUIDE.md) 参照

---

## 📁 サンプルコード

完全な動作例は以下のexampleプロジェクトを参照してください：

```bash
cd examples/minpo-415-breach-damages
cargo run
```

**実行内容：**
1. 基本的な債務不履行（商品未引渡）
2. 予見可能性と損害範囲（Hadley原則）
3. 損害軽減義務（賃貸契約）

---

## ❓ FAQ

### Q1: Article 415とArticle 709の違いは何ですか？

**A:** 主な違いは以下です：

- **前提**: 415は契約関係が必要、709は不要
- **帰責要件**: 415は「帰責事由」、709は「故意・過失」
- **立証責任**: 415は債務者が免責を立証、709は債権者が故意・過失を立証
- **損害範囲**: 415は予見可能性、709は相当因果関係

### Q2: 予見可能性とは具体的に何ですか？

**A:** 「契約時に当事者が予見できた（または予見すべきだった）損害」です。

**Hadley v. Baxendale原則:**
- 通常生じる損害 → 常に予見可能
- 特別の事情による損害 → 契約時に知っていれば予見可能

### Q3: 損害軽減義務は法律に明記されていますか？

**A:** 明文規定はありませんが、判例法理として確立しています。

根拠は民法1条2項の「信義誠実の原則」です。

### Q4: 契約書に「一切の責任を負わない」と書いてあれば免責されますか？

**A:** いいえ。以下の場合は免責条項が無効です：

- 故意・重過失による損害
- 消費者契約法に違反する条項
- 公序良俗違反

### Q5: 同じ事案で415条と709条の両方を請求できますか？

**A:** はい。契約当事者間で不法行為にも該当する場合、両方請求可能です（請求権競合）。

被害者に有利な方を選択できます。

---

## 📞 お問い合わせ・フィードバック

本ツールに関するご質問・ご要望は、GitHubリポジトリのIssueからお寄せください。

---

**Document Version:** 1.0
**Last Updated:** 2026-01-09
**Author:** Legalis-RS Project Team
