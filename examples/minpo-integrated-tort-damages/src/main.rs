use legalis_jp::contract::{Article415, Attribution, AttributionType, BreachType, ObligationType};
use legalis_jp::tort::{
    Article709, Article710, Article715, CausalLink, Damage, EmploymentType, HarmSeverity, Intent,
    NonPecuniaryDamageType, ProtectedInterest, validate_tort_claim,
};

fn main() {
    println!("=== 統合事例: レストラン配達事故 ===");
    println!("=== Integrated Case: Restaurant Delivery Accident ===\n");

    println!("📖 事案の概要 (Case Summary):");
    println!("   レストラン「和食亭」が配送業務を外部委託。");
    println!("   配達員が配達中に交通事故を起こし、歩行者に重傷を負わせた。");
    println!("   また、注文客への配達も遅延した。");
    println!();
    println!("   Restaurant 'Washokutei' outsourced delivery service.");
    println!("   Delivery driver caused accident during delivery, seriously injured pedestrian.");
    println!("   Also, delivery to customer was delayed.");
    println!("\n{}\n", "=".repeat(80));

    restaurant_delivery_integrated_analysis();
}

fn restaurant_delivery_integrated_analysis() {
    println!("## 総合的法的分析 (Comprehensive Legal Analysis)\n");

    // ===== STEP 1: Article 709 - 配達員の不法行為 =====
    println!("### STEP 1: Article 709 - 配達員の不法行為責任");
    println!("### STEP 1: Article 709 - Delivery Driver's Tort Liability\n");

    let driver_tort = Article709::new()
        .with_act("信号無視により横断歩道の歩行者をはねた")
        .with_intent(Intent::NegligenceWithDuty {
            duty_of_care: "信号遵守義務・前方注視義務違反".to_string(),
        })
        .with_victim_interest(ProtectedInterest::BodyAndHealth)
        .with_damage(Damage::new(3_000_000, "治療費 + 入院費"))
        .with_causal_link(CausalLink::Direct);

    match validate_tort_claim(&driver_tort) {
        Ok(_liability) => {
            println!("✅ 配達員の709条不法行為責任成立");
            println!("   Driver's Article 709 tort liability established");
            println!();
            println!("   要件充足:");
            println!("   • 故意・過失: 信号無視（過失）");
            println!("   • 権利侵害: 身体・健康への侵害");
            println!("   • 損害: ¥3,000,000（治療費等）");
            println!("   • 因果関係: 直接因果関係");
            println!();
            println!("   Requirements met:");
            println!("   • Intent/Negligence: Traffic light violation (negligence)");
            println!("   • Rights infringement: Body & health");
            println!("   • Damages: ¥3,000,000 (medical expenses)");
            println!("   • Causation: Direct causation");
        }
        Err(e) => {
            println!("❌ Article 709不成立: {:?}", e);
            return;
        }
    }

    println!("\n{}\n", "-".repeat(80));

    // ===== STEP 2: Article 710 - 非財産的損害（慰謝料） =====
    println!("### STEP 2: Article 710 - 非財産的損害（慰謝料）");
    println!("### STEP 2: Article 710 - Non-Pecuniary Damages (Consolation Money)\n");

    let consolation_claim = Article710::new()
        .with_article_709(driver_tort.clone())
        .damage_type(NonPecuniaryDamageType::BodyAndHealth)
        .harm_severity(HarmSeverity::Severe)
        .emotional_distress("骨折による2ヶ月入院、痛みと精神的苦痛、後遺症への不安");

    match consolation_claim.validate() {
        Ok(_) => {
            let consolation_amount = consolation_claim.recommended_consolation_money();
            println!("✅ Article 710に基づく慰謝料請求成立");
            println!("   Article 710 consolation money claim established");
            println!();
            println!("   推奨慰謝料額: ¥{}", consolation_amount);
            println!("   Recommended consolation money: ¥{}", consolation_amount);
            println!();
            println!("   被害者が請求できる合計額:");
            println!("   Total claimable by victim:");
            println!("   • 財産的損害 (Article 709): ¥3,000,000");
            println!("   • 慰謝料 (Article 710): ¥{}", consolation_amount);
            println!("   • 合計: ¥{}", 3_000_000 + consolation_amount);
        }
        Err(e) => {
            println!("❌ Article 710不成立: {:?}", e);
        }
    }

    println!("\n{}\n", "-".repeat(80));

    // ===== STEP 3: Article 715 - 使用者責任（配送会社） =====
    println!("### STEP 3: Article 715 - 配送会社の使用者責任");
    println!("### STEP 3: Article 715 - Delivery Company's Vicarious Liability\n");

    let employer_liability = Article715::new()
        .employee_tort(driver_tort.clone())
        .employer("配送業者「クイック配送」")
        .employee("配達員 田中一郎")
        .employment_type(EmploymentType::Contract)
        .during_business_execution(true)
        .business_context("レストランからの委託配達業務中")
        .reasonable_care_appointment(false)
        .reasonable_care_supervision(false);

    match employer_liability.build() {
        Ok(claim) => match claim.validate() {
            Ok(_) => {
                println!("✅ 配送会社の使用者責任成立");
                println!("   Delivery company's vicarious liability established");
                println!();
                println!("   判断:");
                println!("   • 使用関係: 業務委託契約（実質的な指揮監督あり）");
                println!("   • 事業執行性: 配達業務中の事故");
                println!("   • 免責の抗弁: 選任・監督の注意義務違反");
                println!();
                println!("   Analysis:");
                println!("   • Employment relation: Contract (substantive supervision exists)");
                println!("   • During business: Accident during delivery");
                println!("   • Defense: Failed to exercise reasonable care in hiring/supervision");
                println!();
                println!("   💡 実務的意義:");
                println!("   被害者は配達員個人ではなく、資力のある配送会社に請求可能。");
                println!();
                println!("   💡 Practical significance:");
                println!("   Victim can claim from delivery company (solvent) rather than driver.");
            }
            Err(e) => {
                println!("❌ Article 715検証失敗: {:?}", e);
            }
        },
        Err(e) => {
            println!("❌ Article 715ビルド失敗: {:?}", e);
        }
    }

    println!("\n{}\n", "-".repeat(80));

    // ===== STEP 4: Article 415 - レストランと注文客の契約違反 =====
    println!("### STEP 4: Article 415 - レストランと注文客の契約違反");
    println!("### STEP 4: Article 415 - Breach Between Restaurant and Customer\n");

    println!("   事故により配達が2時間遅延し、注文客のパーティーに間に合わなかった。");
    println!("   Delivery delayed 2 hours due to accident, missed customer's party.");
    println!();

    let contract_breach = Article415::new()
        .with_obligation(ObligationType::Service {
            description: "30分以内の配達サービス".to_string(),
            duration: Some("30分".to_string()),
        })
        .with_breach(BreachType::DelayedPerformance { days_late: 0 }) // 時間遅延
        .with_attribution(Attribution::new(
            AttributionType::Negligence,
            "配達員の交通事故により遅延",
        ))
        .with_damage(Damage::new(50_000, "代替飲食費用 + パーティー中止損害"))
        .with_causal_link(CausalLink::Direct)
        .creditor("注文客 山田花子")
        .debtor("レストラン和食亭");

    match contract_breach.build() {
        Ok(claim) => match claim.validate() {
            Ok(_) => {
                println!("✅ レストランの債務不履行責任成立");
                println!("   Restaurant's breach of obligation liability established");
                println!();
                println!("   5要件:");
                println!("   1. 債務: 30分配達サービス ✅");
                println!("   2. 不履行: 2時間遅延 ✅");
                println!("   3. 帰責事由: 配達員事故（レストラン側リスク） ✅");
                println!("   4. 因果関係: 直接 ✅");
                println!("   5. 損害: ¥50,000 ✅");
                println!();
                println!("   損害額: ¥{}", claim.estimated_damages());
                println!();
                println!("   💡 契約法と不法行為法の交錯:");
                println!("   同じ事故が:");
                println!("   • 歩行者 → 不法行為責任（709条+710条+715条）");
                println!("   • 注文客 → 契約責任（415条）");
                println!();
                println!("   💡 Contract vs. Tort intersection:");
                println!("   Same accident triggers:");
                println!("   • Pedestrian → Tort liability (Articles 709+710+715)");
                println!("   • Customer → Contract liability (Article 415)");
            }
            Err(e) => {
                println!("❌ Article 415検証失敗: {:?}", e);
            }
        },
        Err(e) => {
            println!("❌ Article 415ビルド失敗: {:?}", e);
        }
    }

    println!("\n{}\n", "=".repeat(80));

    // ===== 総括 =====
    println!("## 📊 総合的損害額と責任関係の整理\n");
    println!("## 📊 Summary of Total Damages and Liability Relations\n");

    println!("### 被害者別の請求先 (Claims by victim):\n");

    println!("【被害者A: 歩行者（交通事故）】");
    println!("[Victim A: Pedestrian (traffic accident)]");
    println!("  請求先候補:");
    println!("  • 配達員個人（709条 + 710条） → 合計 ¥4,500,000");
    println!("  • 配送会社（715条） → 合計 ¥4,500,000（連帯責任）");
    println!();
    println!("  Claim options:");
    println!("  • Driver personally (Articles 709 + 710) → Total ¥4,500,000");
    println!("  • Delivery company (Article 715) → Total ¥4,500,000 (joint liability)");
    println!();

    println!("【被害者B: 注文客（配達遅延）】");
    println!("[Victim B: Customer (delivery delay)]");
    println!("  請求先:");
    println!("  • レストラン和食亭（415条） → ¥50,000");
    println!();
    println!("  Claim to:");
    println!("  • Restaurant Washokutei (Article 415) → ¥50,000");
    println!();

    println!("### 求償関係 (Reimbursement relations):\n");

    println!("  1️⃣  配送会社が被害者Aに支払った場合:");
    println!("      → 配送会社は配達員に求償可能（雇用契約に基づく）");
    println!();
    println!("      If delivery company pays victim A:");
    println!("      → Company may seek reimbursement from driver (based on employment contract)");
    println!();

    println!("  2️⃣  レストランが注文客Bに支払った場合:");
    println!("      → レストランは配送会社に求償可能（委託契約に基づく）");
    println!();
    println!("      If restaurant pays customer B:");
    println!("      → Restaurant may seek reimbursement from delivery company (based on contract)");
    println!();

    println!("### 💡 本事例が示す法原則の統合 (Integration of legal principles):\n");

    println!("  1. 不法行為法（Articles 709, 710, 715）:");
    println!("     - 第三者への加害行為に対する責任");
    println!("     - 非財産的損害の救済");
    println!("     - 使用者の代位責任");
    println!();
    println!("     Tort law (Articles 709, 710, 715):");
    println!("     - Liability for harm to third parties");
    println!("     - Remedy for non-pecuniary damages");
    println!("     - Vicarious liability of employers");
    println!();

    println!("  2. 契約法（Article 415）:");
    println!("     - 契約当事者間の債務不履行責任");
    println!("     - 予見可能性による損害範囲の限定");
    println!();
    println!("     Contract law (Article 415):");
    println!("     - Breach liability between contracting parties");
    println!("     - Damage scope limited by foreseeability");
    println!();

    println!("  3. 実務的選択 (Practical choices):");
    println!("     - 被害者は資力ある相手を選択");
    println!("     - 使用者責任により「深いポケット」へアクセス");
    println!("     - 契約関係と不法行為関係の使い分け");
    println!();
    println!("     Practical choices:");
    println!("     - Victims choose solvent defendants");
    println!("     - Vicarious liability provides access to 'deep pockets'");
    println!("     - Strategic use of contract vs. tort claims");

    println!("\n{}\n", "=".repeat(80));
    println!("✅ 統合分析完了 (Integrated analysis complete)");
}
