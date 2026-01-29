# Article 710 Non-Pecuniary Damages Demo

Demonstration of Japanese Civil Code Article 710 (非財産的損害賠償 / Non-Pecuniary Damages / Consolation Money).

## Overview

This example demonstrates how to model and calculate non-pecuniary damages (慰謝料 / consolation money) under Article 710 of the Japanese Civil Code. It shows how Article 710 works in conjunction with Article 709 to provide comprehensive compensation for both pecuniary and non-pecuniary harm.

## What is Article 710?

Article 710 of the Japanese Civil Code provides for compensation of non-pecuniary damages:

> In the case of the preceding Article, even if the damage is not to property, compensation for such damage shall be claimed.
>
> 他人ノ身体、自由若ハ名誉ヲ害シタル場合又ハ他人ノ財産権ヲ侵害シタル場合ニ於テハ裁判所ハ損害ノ性質ニ因リ被害者ノ財産以外ノ損害ニ対シテモ其賠償ヲ命スルコトヲ得

## Relationship with Article 709

Article 710 **depends on** Article 709:
1. First establish tort liability under Article 709
2. Then claim additional non-pecuniary damages under Article 710

```
Article 709 → Tort established → Pecuniary damages
     ↓
Article 710 → Non-pecuniary damages (慰謝料)
     ↓
Total compensation = Pecuniary + Non-pecuniary
```

## Features

### Three Demonstration Scenarios

1. **Basic Non-Pecuniary Damage** - Traffic accident with emotional distress
2. **Detailed Damages Calculation** - Serious injury with hospitalization
3. **Comparative Analysis** - Defamation case comparing 709-only vs 709+710 claims

### Types of Non-Pecuniary Damage

- **BodyAndHealth** (身体・健康) - Physical injury causing emotional distress
- **ReputationDamage** (名誉毀損) - Harm to reputation
- **Privacy** (プライバシー侵害) - Privacy violation
- **FreedomOfLife** (生活妨害) - Interference with peaceful life

### Harm Severity Levels

- **Minor** (軽微) - Small emotional impact
- **Moderate** (中程度) - Significant distress
- **Severe** (重大) - Serious and lasting emotional harm
- **VerySerious** (極めて重大) - Catastrophic emotional impact

## Usage

```bash
cargo run --bin minpo-710-damages-builder
```

Or from the subcrate directory:

```bash
cargo run
```

## Builder API Example

```rust
// First establish Article 709 liability
let article_709_claim = Article709::new()
    .with_act("交通事故で歩行者に衝突")
    .with_intent(Intent::Negligence)
    .with_victim_interest(ProtectedInterest::BodyAndHealth)
    .with_damage(Damage::new(200_000, "治療費"))
    .with_causal_link(CausalLink::Direct);

// Then claim Article 710 consolation money
let article_710_claim = Article710::new()
    .with_article_709(article_709_claim)
    .damage_type(NonPecuniaryDamageType::BodyAndHealth)
    .harm_severity(HarmSeverity::Moderate)
    .emotional_distress("継続的な痛みと精神的苦痛");

// Total = Pecuniary + Non-pecuniary
```

## Key Differences: 709 vs 710

| Aspect | Article 709 | Article 710 |
|--------|-------------|-------------|
| **Nature** | General tort liability | Non-pecuniary damages |
| **Damages** | Pecuniary (財産的損害) | Non-pecuniary (非財産的損害) |
| **Examples** | Medical bills, lost wages | Pain, suffering, anxiety |
| **Quantification** | Actual expenses | Judicial discretion |
| **Independence** | Can stand alone | Requires 709 first |

## Consolation Money (慰謝料) Calculation

Japanese courts use discretion to determine appropriate consolation money amounts based on:
- **Severity of harm** - How serious was the injury?
- **Duration of suffering** - How long will the impact last?
- **Type of right violated** - Body/health, reputation, privacy?
- **Defendant's conduct** - Intentional vs. negligent?
- **Social circumstances** - Victim's age, profession, social standing

This example provides **recommended amounts** based on common judicial practices, but actual awards require judicial discretion.

## Example Output

```
✅ Article 710成立 (Non-pecuniary damages established)
   推奨慰謝料額 (Recommended consolation money): ¥800,000

   合計損害額 (Total damages):
   財産的損害 ¥200,000 + 慰謝料 ¥800,000 = ¥1,000,000
```

## Practical Importance

In many cases, especially for:
- **Personal injury** (身体傷害)
- **Defamation** (名誉毀損)
- **Privacy violations** (プライバシー侵害)

The consolation money (慰謝料) can **exceed** the pecuniary damages. For example, in defamation cases, actual financial loss may be small, but emotional harm substantial.

## Related Examples

- `minpo-709-builder` - Article 709 builder API
- `minpo-709-tort` - Tort liability simulation
- `minpo-integrated-tort-damages` - Integrated tort scenarios

## Documentation

For more information on the Legalis-RS framework, see the [main project documentation](../../README.md).

## License

Licensed under either of MIT or Apache-2.0 at your option.
