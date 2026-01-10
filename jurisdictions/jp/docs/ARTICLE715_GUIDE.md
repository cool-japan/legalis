# 民法715条ビルダーAPI - 法律専門家向けガイド

## 📋 概要：何を作ったのか

**民法第715条（使用者責任・雇用者の代位責任）の要件判定を、コンピュータで構造化・自動化するツール**を開発しました。

従業員（被用者）が業務中に第三者に損害を与えた場合、使用者（雇用主）が代わりに責任を負う「使用者責任」を構造的に処理できます。

このシステムでは：
- **従業員のArticle 709不法行為の確認**
- **使用関係（雇用関係）の存在判定**
- **事業執行性（業務中か）の判定**
- **免責の抗弁（使用者の防御）の評価**
- **外形理論（見かけ上の権限）の適用**

が可能になります。

---

## ⚖️ 法的背景：民法715条とは

民法第715条第1項は使用者の代位責任（vicarious liability）を定める規定です：

> **民法第715条第1項（使用者等の責任）**
>
> ある事業のために他人を使用する者は、被用者がその事業の執行について第三者に加えた損害を賠償する責任を負う。ただし、使用者が被用者の選任及びその事業の監督について相当の注意をしたとき、又は相当の注意をしても損害が生ずべきであったときは、この限りでない。

**English Translation:**
A person who employs another to engage in an undertaking is liable for damage inflicted on a third party by the employee in the course of execution of that undertaking; provided, however, that this does not apply if the employer exercised reasonable care in appointing the employee and in supervising the undertaking, or if the damage would have occurred even if the employer had exercised reasonable care.

### 使用者責任の趣旨（立法趣旨）

1. **危険責任の法理**
   - 他人を使って事業を行い利益を得る者は、その事業から生じる危険も負担すべき

2. **被害者保護**
   - 従業員個人に資力がない場合でも、資力のある使用者から賠償を受けられる
   - 「深いポケット理論」（Deep Pocket Theory）

3. **事故予防のインセンティブ**
   - 使用者に責任を課すことで、適切な監督・教育を促進

### 成立要件（3要件）

1. **使用関係（雇用関係）の存在**
   - 使用者と被用者の間に指揮監督関係があること
   - 正社員、アルバイト、契約社員など雇用形態は問わない

2. **被用者の不法行為（Article 709）**
   - 従業員がArticle 709の不法行為を犯したこと
   - 故意・過失、権利侵害、因果関係、損害の4要件

3. **事業執行について（事業執行性）**
   - 不法行為が「事業の執行について」行われたこと
   - 業務時間中、業務の一環として、または業務に関連して

### 免責の抗弁（使用者の防御）

使用者は以下を立証すれば免責されます：

1. **選任の際の相当の注意**
   - 採用時に十分な調査・確認を行った

2. **監督の際の相当の注意**
   - 業務監督、教育・研修を適切に行った

3. **相当の注意をしても損害発生**
   - 上記の注意をしても避けられなかった

**実務上の重要ポイント：**
免責の立証責任は使用者側にあり、実際に免責が認められるケースは極めて稀です。

---

## 🔍 従来の問題点

### 1. 使用関係の判定の曖昧性

「使用関係」があるか否かの判断が困難です。

**例：** 業務委託契約だが実質的に指揮監督下にある場合
- 形式：独立請負人（使用者責任なし）
- 実質：従業員（使用者責任あり）

### 2. 事業執行性の範囲の不明確性

「事業の執行について」の範囲が不明確です。

**例：** 配達途中にコンビニに寄り道した際の事故
- 完全に私用 → 事業執行性なし
- 業務の延長 → 事業執行性あり（外形理論）

### 3. 免責の抗弁の立証困難性

使用者が免責を主張しても、その立証が極めて困難です。

**リスク：** 「相当の注意」の基準が不明確で、実務上ほぼ免責されない

---

## 💡 このシステムが提供するもの

### 1. **Article 709との自動連携**

```rust
use legalis_jp::tort::{Article709, Article715, Intent, Damage, CausalLink, ProtectedInterest};
use legalis_jp::tort::EmploymentType;

// Step 1: 従業員のArticle 709不法行為を確立
let employee_tort = Article709::new()
    .with_act("配達中に前方不注意で歩行者に衝突")
    .with_intent(Intent::NegligenceWithDuty {
        duty_of_care: "前方注視義務違反".to_string()
    })
    .with_victim_interest(ProtectedInterest::BodyAndHealth)
    .with_damage(Damage::new(800_000, "治療費 + 休業損害"))
    .with_causal_link(CausalLink::Direct);

// Step 2: 使用者責任の検討
let employer_liability = Article715::new()
    .employee_tort(employee_tort)
    .employer("株式会社ABC配送")
    .employee("配達員 山田太郎")
    .employment_type(EmploymentType::FullTime)
    .during_business_execution(true)
    .business_context("通常の配達業務中の事故");
```

従業員の709条不法行為を前提に、使用者責任を自動判定します。

### 2. **雇用形態の柔軟な判定**

```rust
pub enum EmploymentType {
    FullTime,        // 正社員
    PartTime,        // アルバイト・パート
    Contract,        // 契約社員
    Dispatch,        // 派遣社員
    Independent,     // 独立請負人
    Agent,           // 代理人
}
```

様々な雇用形態に対応し、実質的な使用関係を判定します。

### 3. **免責の抗弁の構造的評価**

```rust
let claim = Article715::new()
    .employee_tort(employee_tort)
    .employer("運送会社XYZ")
    .employee("ドライバー")
    .employment_type(EmploymentType::Contract)
    .during_business_execution(true)
    .reasonable_care_appointment(false)  // 選任の注意なし
    .reasonable_care_supervision(false)  // 監督の注意なし
    .care_evidence("免許確認を怠り、形式的な面接のみで採用");
```

使用者の防御（免責の抗弁）を構造的に評価します。

---

## 📚 具体的な使用例

### ケース1：配達業務中の交通事故（直接的な使用者責任）

**事案：** 配送会社のドライバーが配達中に前方不注意で歩行者に衝突。被害者は治療費80万円を請求したい。

```rust
use legalis_jp::tort::{
    Article709, Article715, Intent, Damage, CausalLink, ProtectedInterest,
    EmploymentType, validate_tort_claim
};

// Step 1: 従業員（ドライバー）のArticle 709不法行為
let employee_tort = Article709::new()
    .with_act("配達中に前方不注意で歩行者に衝突")
    .with_intent(Intent::NegligenceWithDuty {
        duty_of_care: "前方注視義務違反".to_string()
    })
    .with_victim_interest(ProtectedInterest::BodyAndHealth)
    .with_damage(Damage::new(800_000, "治療費 + 休業損害"))
    .with_causal_link(CausalLink::Direct);

// Step 2: Article 709の検証
match validate_tort_claim(&employee_tort) {
    Ok(_) => {
        println!("従業員の709条不法行為成立");

        // Step 3: 使用者責任の検討
        let employer_liability = Article715::new()
            .employee_tort(employee_tort)
            .employer("株式会社ABC配送")
            .employee("配達員 山田太郎")
            .employment_type(EmploymentType::FullTime)
            .during_business_execution(true)
            .business_context("通常の配達業務中の事故");

        match employer_liability.build() {
            Ok(claim) => {
                match claim.validate() {
                    Ok(_) => {
                        println!("✅ Article 715使用者責任成立");
                        println!("被害者は配送会社に請求可能");
                        // Output: ✅ Article 715使用者責任成立
                        //         被害者は配送会社に請求可能
                    }
                    Err(e) => println!("検証失敗: {:?}", e),
                }
            }
            Err(e) => println!("ビルドエラー: {:?}", e),
        }
    }
    Err(e) => println!("従業員の709条不成立: {:?}", e),
}
```

**結果：**
- 従業員個人（ドライバー）: Article 709により責任あり
- 使用者（配送会社）: Article 715により連帯責任あり
- 被害者は**どちらにも請求可能**（選択的）

**実務的ポイント：**
被害者は資力のある配送会社に請求するのが一般的です。

---

### ケース2：アルバイト従業員の顧客情報漏洩（監督義務違反）

**事案：** 飲食店のアルバイトがSNSで顧客の個人情報を無断投稿。顧客はプライバシー侵害で30万円を請求。

```rust
use legalis_jp::tort::{
    Article709, Article715, Intent, Damage, CausalLink, ProtectedInterest,
    EmploymentType
};

// Step 1: アルバイトのArticle 709不法行為（プライバシー侵害）
let employee_tort = Article709::new()
    .with_act("顧客の個人情報を無断でSNSに投稿")
    .with_intent(Intent::Intentional { age: 20 })
    .with_victim_interest(ProtectedInterest::Privacy)
    .with_damage(Damage::new(300_000, "プライバシー侵害"))
    .with_causal_link(CausalLink::Direct);

// Step 2: 使用者責任（監督義務の観点）
let employer_liability = Article715::new()
    .employee_tort(employee_tort)
    .employer("飲食店オーナー")
    .employee("アルバイト 佐藤花子")
    .employment_type(EmploymentType::PartTime)  // アルバイト
    .during_business_execution(true)
    .business_context("勤務中に店舗で撮影した顧客情報を投稿")
    .reasonable_care_supervision(false);  // 監督義務違反

match employer_liability.build() {
    Ok(claim) => {
        if claim.is_liability_established() {
            println!("✅ 使用者（店舗オーナー）に責任あり");
            println!();
            println!("理由:");
            println!("• 個人情報保護の教育・監督義務を怠った");
            println!("• SNS利用規程が不十分だった");
            println!();
            println!("💡 実務的教訓:");
            println!("アルバイトであっても、業務中の行為については");
            println!("使用者が監督義務を負います。");
            println!("適切な研修と監督が必須です。");
        }
    }
    Err(e) => println!("ビルドエラー: {:?}", e),
}
```

**重要ポイント：**
- アルバイト・パートでも使用者責任の対象
- SNS教育、個人情報保護研修の実施が重要
- 就業規則にSNS禁止条項を明記すべき

---

### ケース3：無免許ドライバーの雇用（過失ある選任）

**事案：** 運送会社が免許確認を怠り無免許ドライバーを雇用。配送中に事故で車両破損120万円。

```rust
use legalis_jp::tort::{
    Article709, Article715, Intent, Damage, CausalLink, ProtectedInterest,
    EmploymentType
};

// Step 1: 無免許ドライバーの事故
let employee_tort = Article709::new()
    .with_act("無免許運転で配送中に車両を破損")
    .with_intent(Intent::NegligenceWithDuty {
        duty_of_care: "免許保持義務違反 + 前方注視義務違反".to_string()
    })
    .with_victim_interest(ProtectedInterest::Property("駐車車両"))
    .with_damage(Damage::new(1_200_000, "車両修理費 + 代車費用"))
    .with_causal_link(CausalLink::Direct);

// Step 2: 使用者の過失（選任・監督義務違反）
let employer_liability = Article715::new()
    .employee_tort(employee_tort)
    .employer("運送会社XYZ")
    .employee("無免許ドライバー 田中一郎")
    .employment_type(EmploymentType::Contract)
    .during_business_execution(true)
    .business_context("配送業務中の事故")
    .reasonable_care_appointment(false)  // 選任に相当の注意なし
    .care_evidence("免許確認を怠り、形式的な面接のみで採用");

match employer_liability.build() {
    Ok(claim) => {
        println!("使用者の過失:");
        println!("• 選任の際の注意義務違反");
        println!("• 免許証の確認を怠った");
        println!("• 形式的な面接のみで技能確認せず");
        println!();

        if claim.is_liability_established() {
            println!("✅ 使用者責任成立（免責不可）");
            println!();
            println!("📋 免責の抗弁の分析:");
            println!();
            println!("Article 715 ただし書:");
            println!("「使用者が被用者の選任及びその事業の監督について");
            println!(" 相当の注意をしたとき」は免責される");
            println!();
            println!("本件では:");
            println!("❌ 選任の注意: 免許確認せず → 相当の注意なし");
            println!("❌ 監督の注意: 無免許を見過ごす → 相当の注意なし");
            println!();
            println!("結論: 使用者は免責されず、全額賠償責任を負う");
        }
    }
    Err(e) => println!("ビルドエラー: {:?}", e),
}
```

**実務チェックリスト（採用時）：**
- ☑ 運転免許証の原本確認
- ☑ 職務経歴書の裏付け調査
- ☑ 前職の照会（リファレンスチェック）
- ☑ 適性検査の実施
- ☑ 試用期間の設定

---

## 🔧 API リファレンス

### Article715 Builder Methods

#### `Article715::new() -> Self`
新しいArticle 715請求を作成します。

```rust
let claim = Article715::new();
```

#### `.employee_tort(tort: Article709<'a>) -> Self`
**必須**：従業員のArticle 709不法行為を設定します。

```rust
.employee_tort(employee_tort_claim)
```

#### `.employer(name: impl Into<String>) -> Self`
使用者（雇用主）の名前を設定します（便利メソッド）。

```rust
.employer("株式会社ABC配送")
```

#### `.employee(name: impl Into<String>) -> Self`
被用者（従業員）の名前を設定します（便利メソッド）。

```rust
.employee("配達員 山田太郎")
```

#### `.employment_type(etype: EmploymentType) -> Self`
雇用形態を設定します（便利メソッド）。

```rust
.employment_type(EmploymentType::FullTime)
```

**EmploymentType の種類：**
- `FullTime` - 正社員
- `PartTime` - アルバイト・パート
- `Contract` - 契約社員
- `Dispatch` - 派遣社員
- `Independent` - 独立請負人（使用者責任なし）
- `Agent` - 代理人

#### `.employment(relationship: EmploymentRelationship<'a>) -> Self`
雇用関係を直接設定します（上級者向け）。

```rust
.employment(EmploymentRelationship {
    employer_name: "会社名",
    employee_name: "従業員名",
    employment_type: EmploymentType::FullTime,
    relationship_duration: Some("2年間".to_string()),
})
```

#### `.during_business_execution(during: bool) -> Self`
**必須**：事業執行性（業務中か）を設定します。

```rust
.during_business_execution(true)
```

#### `.business_context(context: impl Into<String>) -> Self`
事業執行の文脈・説明を設定します。

```rust
.business_context("通常の配達業務中の事故")
```

#### `.reasonable_care_appointment(exercised: bool) -> Self`
選任の際の相当の注意（免責の抗弁）を設定します。

```rust
.reasonable_care_appointment(false)  // 注意を怠った
```

#### `.reasonable_care_supervision(exercised: bool) -> Self`
監督の際の相当の注意（免責の抗弁）を設定します。

```rust
.reasonable_care_supervision(false)  // 監督を怠った
```

#### `.care_evidence(evidence: impl Into<String>) -> Self`
相当の注意の証拠を設定します。

```rust
.care_evidence("免許確認を怠り、形式的な面接のみで採用")
```

#### `.unavoidable_damage(unavoidable: bool) -> Self`
相当の注意をしても損害が発生したか設定します。

```rust
.unavoidable_damage(false)
```

#### `.build() -> Result<Article715<'a>, TortClaimError>`
必須フィールドがすべて設定されているか確認してビルドします。

```rust
let claim = Article715::new()
    .employee_tort(tort)
    .employer("会社名")
    .employee("従業員名")
    .employment_type(EmploymentType::FullTime)
    .during_business_execution(true)
    .build()?;
```

#### `.validate() -> Result<(), ValidationError>`
Article 715の要件をすべて満たしているか検証します。

```rust
match claim.validate() {
    Ok(_) => println!("使用者責任成立"),
    Err(e) => println!("不成立: {:?}", e),
}
```

#### `.is_liability_established() -> bool`
使用者責任が成立しているか簡易チェックします。

```rust
if claim.is_liability_established() {
    println!("使用者に責任あり");
}
```

---

## 📊 事業執行性の判断基準

### 外形理論（外形標準説）

判例は「外形理論」を採用しています：

| 基準 | 説明 | 例 |
|------|------|-----|
| **時間的関連性** | 就業時間中か | 勤務時間中 → ○ / 休憩時間中 → △ |
| **場所的関連性** | 職場またはその周辺か | 配達先 → ○ / 自宅 → × |
| **行為の外観** | 業務のように見えるか | 制服着用 → ○ / 私服 → △ |
| **職務権限との関連** | 職務の範囲内か | 営業活動 → ○ / 私用 → × |

### 判例の傾向

**○ 事業執行性あり（使用者責任成立）:**
- 配達業務中の交通事故
- 営業活動中の不法行為
- 業務上の判断ミスによる損害

**△ グレーゾーン（個別判断）:**
- 休憩時間中の事故
- 業務後の飲み会での行為
- 出張先での私的行為

**× 事業執行性なし（使用者責任不成立）:**
- 完全に私用の行為
- 通勤途中の事故（通常）
- 業務と無関係な犯罪

---

## 🎯 実務上のTips

### 1. 使用関係の実質的判断

形式的な契約形態ではなく、実質的な指揮監督関係で判断します。

```rust
// ❌ 形式だけで判断しない
.employment_type(EmploymentType::Independent)  // 独立請負契約

// ✅ 実質的な関係を考慮
// 実際には指揮監督下にある → FullTime または Contract として扱う
.employment_type(EmploymentType::Contract)
```

**実質判断のポイント：**
- 業務の具体的指示があるか
- 勤務時間・場所の拘束があるか
- 報酬が給与形式か
- 社会保険の加入状況

### 2. 免責の抗弁は原則認められない

実務上、使用者が免責される例は極めて稀です。

```rust
// 免責を主張する場合でも、立証責任は使用者側
.reasonable_care_appointment(true)
.reasonable_care_supervision(true)
.care_evidence("詳細な証拠が必要");
// → それでも裁判所が認めないケースが多い
```

**免責が認められた稀な例：**
- 従業員が完全に逸脱した犯罪行為（窃盗、横領等）
- 使用者が相当の注意を尽くし、かつ予測不可能な事故

### 3. 被害者の選択と求償関係

```
被害者
  ↓ 請求可能（選択的）
  ├→ 従業員個人（Article 709）
  └→ 使用者（Article 715）
       ↓ 求償権
       └→ 従業員へ求償可能
```

**実務の流れ：**
1. 被害者は資力のある使用者に請求
2. 使用者が被害者に全額支払い
3. 使用者が従業員に求償（雇用契約・就業規則に基づく）

---

## 🔗 関連条文との関係

### Article 709（不法行為の一般規定）
Article 715の**前提条件**です。まず従業員のArticle 709成立を確認してください。

→ [ARTICLE709_GUIDE.md](./ARTICLE709_GUIDE.md) 参照

### Article 710（非財産的損害）
使用者責任でも慰謝料（Article 710）を請求可能です。

→ [ARTICLE710_GUIDE.md](./ARTICLE710_GUIDE.md) 参照

### Article 714（責任能力なき者の監督者責任）
未成年者や精神障害者の監督者責任。Article 715の親戚的な規定です。

### Article 716（請負人の責任）
注文者の責任。Article 715とは異なる特殊な責任です。

### Article 717（土地工作物責任）
建物・構築物の所有者責任。占有者→所有者の順で責任を負います。

---

## 📁 サンプルコード

完全な動作例は以下のexampleプロジェクトを参照してください：

```bash
cd examples/minpo-715-employer-liability
cargo run
```

**実行内容：**
1. 直接的な使用者責任（配達事故）
2. 監督義務違反（アルバイト従業員の情報漏洩）
3. 過失ある選任（無免許ドライバー雇用）

---

## ❓ FAQ

### Q1: 業務委託契約でも使用者責任は成立しますか？

**A:** 形式的に「業務委託」でも、実質的に指揮監督下にあれば使用者責任が成立します。

**判断基準：**
- 業務の具体的指示の有無
- 時間・場所の拘束の有無
- 報酬形態（時給制か成果報酬か）

### Q2: 通勤途中の事故でも使用者責任は成立しますか？

**A:** 原則として通勤途中は「事業の執行について」に該当せず、使用者責任は成立しません。

**例外：**
- 会社の車両を使用している場合
- 業務の一環としての移動（出張等）
- 会社の指示による移動

### Q3: アルバイト・パートでも使用者責任の対象ですか？

**A:** はい。雇用形態に関わらず、使用関係があれば対象です。

正社員、契約社員、アルバイト、パート、派遣社員すべて含まれます。

### Q4: 使用者が免責される可能性はどのくらいですか？

**A:** 実務上、免責が認められるケースは5%未満と言われています。

免責の立証責任は使用者側にあり、極めて高いハードルです。

### Q5: 被害者は使用者と従業員の両方に請求できますか？

**A:** はい。被害者は両方に対して請求できます（選択的）。

実務上は資力のある使用者に請求するのが一般的です。使用者が支払った後、従業員に求償します。

---

## 📞 お問い合わせ・フィードバック

本ツールに関するご質問・ご要望は、GitHubリポジトリのIssueからお寄せください。

---

**Document Version:** 1.0
**Last Updated:** 2026-01-09
**Author:** Legalis-RS Project Team
