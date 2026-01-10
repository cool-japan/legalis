use legalis_jp::tort::{Article709, CausalLink, Damage, Intent, ProtectedInterest};
use legalis_jp::tort::{LiabilityStatus, validate_tort_claim};

fn main() {
    println!("=== Legalis-JP Enhanced Builder API Demo ===\n");

    check_basic_tort();
    println!("\n{}\n", "=".repeat(80));

    detailed_validation_example();
    println!("\n{}\n", "=".repeat(80));

    with_supervisor_liability();
}

// 1. 基本的な709条適用チェック（シンプルケース）
fn check_basic_tort() {
    println!("1. 基本的な709条適用チェック (Basic Article 709 Check)");
    println!();

    let claim = Article709::new()
        .with_act("交通事故で相手の車に衝突")
        .with_intent(Intent::Negligence) // 過失（うっかりスマホ見てた）
        .with_victim_interest(ProtectedInterest::Property("車両所有権"))
        .with_damage(Damage::new(500_000, "修理費 + レッカー代"))
        .with_causal_link(CausalLink::Direct); // 直接因果

    let result = validate_tort_claim(&claim);
    match result {
        Ok(liability) => {
            match liability.status {
                LiabilityStatus::Established => {
                    println!("✅ 709条成立！賠償責任あり");
                    println!("   Article 709 established! Tortfeasor is liable.");
                }
                LiabilityStatus::InsufficientEvidence(ref reason) => {
                    println!("⚠️  過失が弱いかも: {}", reason);
                }
                _ => println!("❌ 不成立"),
            }

            assert_eq!(liability.article.number, "709");
            assert!(liability.is_liability_established());

            println!("\n検証詳細 (Validation Details):");
            for detail in &liability.validation_details {
                println!("  • {}", detail);
            }
        }
        Err(e) => {
            println!("❌ エラー: {}", e);
        }
    }
}

// 2. 詳細要件を細かく検証（実務向け）
fn detailed_validation_example() {
    println!("2. 詳細要件の検証 (Detailed Validation Example)");
    println!();

    let claim = Article709::builder()
        .act("歩行者を自転車でひいた")
        .intent(Intent::NegligenceWithDuty {
            duty_of_care: "前方不注視".to_string(),
        })
        .injured_interest(ProtectedInterest::BodyAndHealth) // 身体・健康（710条連動）
        .damage(Damage::new(3_000_000, "治療費 + 慰謝料 + 休業損害"))
        .causal_link(CausalLink::Adequate("事故がなければ損害発生せず"))
        .responsibility_capacity(true); // 責任能力あり（18歳以上想定）

    // 自動検証（Rustの型安全で要件漏れ防止）
    let validation = claim.validate();
    match validation {
        Ok(_) => {
            println!("✅ 完全709条成立");
            println!("   Article 709 fully established");
            println!();
            println!(
                "   損害賠償額推定 (Estimated Compensation): ¥{}",
                claim.estimated_compensation()
            );
            println!("   ※ 実際の賠償額は過失相殺等で調整されます");
            println!("      (Actual compensation may be adjusted for comparative negligence)");
        }
        Err(e) => {
            println!("❌ 不成立理由: {:?}", e);
            println!("   Not established. Reason: {:?}", e);
        }
    }
}

// 3. 判例風拡張（過失相殺や監督者責任連動）
fn with_supervisor_liability() {
    println!("3. 監督者責任（Article 715）との連動");
    println!("   Supervisor Liability (Article 715 Connection)");
    println!();

    let child_act = Article709::new()
        .with_act("小学生がボールで他人の窓ガラス破損")
        .with_intent(Intent::Intentional { age: 10 }) // 故意だが責任能力微妙
        .with_victim_interest(ProtectedInterest::Property("窓ガラス"))
        .with_damage(Damage::new(50_000, "窓ガラス修理費"))
        .with_causal_link(CausalLink::Direct);

    // 709条での責任能力チェック
    if !child_act.has_full_capacity() {
        println!("⚠️  子供に責任能力なし（age < 12）");
        println!("   Child lacks responsibility capacity (age < 12)");
        println!();
        println!("   💡 Legal Interpretation:");
        println!("      While the child lacks capacity under Article 709,");
        println!("      the parent may be liable under Article 715(1) or 714 for");
        println!("      failure to properly supervise the child.");
        println!();
        println!("      子供は709条の責任能力を欠くが、親は715条1項や714条により");
        println!("      監督義務違反として責任を負う可能性がある。");
    }
}
