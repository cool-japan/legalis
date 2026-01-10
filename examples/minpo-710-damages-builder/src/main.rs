use legalis_jp::tort::{
    Article709, Article710, CausalLink, Damage, HarmSeverity, Intent, NonPecuniaryDamageType,
    ProtectedInterest, validate_tort_claim,
};

fn main() {
    println!("=== Article 710 Non-Pecuniary Damages (慰謝料) Demo ===\n");

    check_basic_non_pecuniary_damage();
    println!("\n{}\n", "=".repeat(80));

    detailed_damages_calculation();
    println!("\n{}\n", "=".repeat(80));

    comparative_damages_analysis();
}

// 1. 基本的な非財産的損害（Simple emotional damage case）
fn check_basic_non_pecuniary_damage() {
    println!("1. 基本的な慰謝料請求 (Basic Non-Pecuniary Damage Claim)");
    println!();

    // まずArticle 709で不法行為の成立を確認
    // First establish Article 709 tort liability
    let article_709_claim = Article709::new()
        .with_act("交通事故で歩行者に衝突")
        .with_intent(Intent::Negligence)
        .with_victim_interest(ProtectedInterest::BodyAndHealth)
        .with_damage(Damage::new(200_000, "治療費"))
        .with_causal_link(CausalLink::Direct);

    let validation_709 = validate_tort_claim(&article_709_claim);
    match validation_709 {
        Ok(_liability) => {
            println!("✅ Article 709成立 (Tort liability established)");
            println!("   財産的損害 (Pecuniary damages): ¥200,000");
            println!();

            // Article 710で慰謝料を請求
            // Claim non-pecuniary damages under Article 710
            let article_710_claim = Article710::new()
                .with_article_709(article_709_claim)
                .damage_type(NonPecuniaryDamageType::BodyAndHealth)
                .harm_severity(HarmSeverity::Moderate)
                .emotional_distress("継続的な痛みと精神的苦痛");

            match article_710_claim.validate() {
                Ok(_) => {
                    println!("✅ Article 710成立 (Non-pecuniary damages established)");
                    println!(
                        "   推奨慰謝料額 (Recommended consolation money): ¥{}",
                        article_710_claim.recommended_consolation_money()
                    );
                    println!();
                    println!("   合計損害額 (Total damages):");
                    println!(
                        "   財産的損害 ¥200,000 + 慰謝料 ¥{} = ¥{}",
                        article_710_claim.recommended_consolation_money(),
                        200_000 + article_710_claim.recommended_consolation_money()
                    );
                }
                Err(e) => {
                    println!("❌ Article 710検証失敗: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Article 709不成立: {:?}", e);
        }
    }
}

// 2. 詳細な損害計算（Multiple damage types）
fn detailed_damages_calculation() {
    println!("2. 詳細な損害計算 (Detailed Damages Calculation)");
    println!("   事案: 重傷事故（骨折、3ヶ月入院）");
    println!("   Case: Serious injury accident (fracture, 3 months hospitalization)");
    println!();

    // Article 709の基礎
    let article_709_claim = Article709::new()
        .with_act("赤信号無視で横断中の歩行者をはねた")
        .with_intent(Intent::NegligenceWithDuty {
            duty_of_care: "信号遵守義務違反".to_string(),
        })
        .with_victim_interest(ProtectedInterest::BodyAndHealth)
        .with_damage(Damage::new(5_000_000, "治療費 + 入院費 + 休業損害"))
        .with_causal_link(CausalLink::Direct);

    println!("財産的損害の内訳 (Breakdown of pecuniary damages):");
    println!("  • 治療費 (Medical expenses): ¥2,000,000");
    println!("  • 入院費 (Hospitalization): ¥1,500,000");
    println!("  • 休業損害 (Lost wages): ¥1,500,000");
    println!("  合計 (Subtotal): ¥5,000,000");
    println!();

    // Article 710で慰謝料を追加
    let article_710_claim = Article710::new()
        .with_article_709(article_709_claim)
        .damage_type(NonPecuniaryDamageType::BodyAndHealth)
        .harm_severity(HarmSeverity::Severe)
        .emotional_distress("3ヶ月間の入院生活による精神的苦痛、後遺症への不安")
        .consolation_money(1_500_000); // 明示的に指定

    match article_710_claim.validate() {
        Ok(_) => {
            println!("非財産的損害 (Non-pecuniary damages):");
            println!("  • 慰謝料 (Consolation money): ¥1,500,000");
            println!("    - 入院慰謝料 (Hospitalization distress)");
            println!("    - 後遺症不安 (Anxiety about aftereffects)");
            println!();
            println!("✅ 合計請求額 (Total claim amount): ¥6,500,000");
            println!();
            println!("💡 Legal Note:");
            println!("   Article 710は「財産以外の損害」を認める規定です。");
            println!("   Article 710 provides for compensation of non-pecuniary damages.");
            println!("   治療費等の財産的損害は709条、精神的苦痛は710条の根拠となります。");
            println!(
                "   Pecuniary damages (medical bills) under 709, emotional distress under 710."
            );
        }
        Err(e) => {
            println!("❌ 検証失敗: {:?}", e);
        }
    }
}

// 3. 比較分析（709単独 vs 709+710）
fn comparative_damages_analysis() {
    println!("3. 比較分析: 709条単独 vs 709条+710条");
    println!("   Comparative Analysis: Article 709 alone vs 709+710");
    println!();

    // ケース: 名誉毀損事件
    // Case: Defamation case
    println!("事案: SNSでの誹謗中傷による名誉毀損");
    println!("Case: Defamation through social media slander");
    println!();

    let article_709_claim = Article709::new()
        .with_act("SNSで虚偽の情報を拡散し被害者の名誉を毀損")
        .with_intent(Intent::Intentional { age: 25 })
        .with_victim_interest(ProtectedInterest::Reputation)
        .with_damage(Damage::new(100_000, "弁護士費用"))
        .with_causal_link(CausalLink::Direct);

    println!("【パターンA】Article 709のみで請求");
    println!("【Pattern A】Claim under Article 709 only");
    println!();
    println!("  財産的損害:");
    println!("  • 弁護士費用: ¥100,000");
    println!("  合計: ¥100,000");
    println!();

    println!("【パターンB】Article 709 + 710で請求");
    println!("【Pattern B】Claim under Article 709 + 710");
    println!();

    let article_710_claim = Article710::new()
        .with_article_709(article_709_claim)
        .damage_type(NonPecuniaryDamageType::ReputationDamage)
        .harm_severity(HarmSeverity::Moderate)
        .emotional_distress("社会的信用の低下、精神的ショック、不眠");

    match article_710_claim.validate() {
        Ok(_) => {
            let consolation = article_710_claim.recommended_consolation_money();
            println!("  財産的損害 (Article 709):");
            println!("  • 弁護士費用: ¥100,000");
            println!();
            println!("  非財産的損害 (Article 710):");
            println!("  • 慰謝料: ¥{}", consolation);
            println!("    - 社会的信用低下 (Loss of social credibility)");
            println!("    - 精神的ショック (Emotional shock)");
            println!("    - 不眠 (Insomnia)");
            println!();
            println!("  合計: ¥{}", 100_000 + consolation);
            println!();
            println!("💰 差額 (Difference): ¥{} の増額", consolation);
            println!();
            println!("📊 分析 (Analysis):");
            println!("   名誉毀損のような人格権侵害では、財産的損害が少額でも、");
            println!("   Article 710により精神的苦痛に対する慰謝料を別途請求できます。");
            println!();
            println!("   In personality rights infringement like defamation,");
            println!("   even if pecuniary damages are small, Article 710 allows");
            println!("   separate claims for emotional distress (consolation money).");
        }
        Err(e) => {
            println!("❌ 検証失敗: {:?}", e);
        }
    }
}
