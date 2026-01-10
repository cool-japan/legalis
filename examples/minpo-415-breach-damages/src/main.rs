use legalis_jp::contract::{Article415, Attribution, AttributionType, BreachType, ObligationType};
use legalis_jp::tort::{CausalLink, Damage};

fn main() {
    println!("=== Article 415 Breach of Obligation (債務不履行) Demo ===\n");

    check_basic_breach_liability();
    println!("\n{}\n", "=".repeat(80));

    detailed_breach_with_foreseeability();
    println!("\n{}\n", "=".repeat(80));

    mitigation_obligation_check();
}

// 1. 基本的な債務不履行責任（Seller fails to deliver goods）
fn check_basic_breach_liability() {
    println!("1. 基本的な債務不履行責任 (Basic Breach Liability)");
    println!("   事案: 売主が商品を引き渡さない");
    println!("   Case: Seller fails to deliver goods");
    println!();

    let claim = Article415::new()
        .with_obligation(ObligationType::Delivery {
            description: "コンピュータ機器10台の引渡".to_string(),
        })
        .with_breach(BreachType::NonPerformance)
        .with_attribution(Attribution::new(
            AttributionType::Negligence,
            "正当な理由なく引渡しを拒否",
        ))
        .with_damage(Damage::new(5_000_000, "代替品購入費用"))
        .with_causal_link(CausalLink::Direct)
        .creditor("株式会社ABC")
        .debtor("供給業者XYZ");

    match claim.build() {
        Ok(breach_claim) => {
            println!("✅ Article 415の5要件をすべて充足");
            println!("   All 5 requirements of Article 415 satisfied");
            println!();
            println!("   【5要件の確認】(Five Requirements Check):");
            println!("   1. 債務の存在 (Obligation exists): ✅");
            println!("      商品引渡債務 (Delivery obligation for computer equipment)");
            println!();
            println!("   2. 不履行 (Non-performance): ✅");
            println!("      履行拒絶 (Refusal to perform)");
            println!();
            println!("   3. 帰責事由 (Attribution): ✅");
            println!("      過失 - 正当理由なく拒否 (Negligence - refusal without justification)");
            println!();
            println!("   4. 因果関係 (Causation): ✅");
            println!("      直接因果関係 (Direct causation)");
            println!();
            println!("   5. 損害 (Damage): ✅");
            println!("      代替品購入費用 ¥5,000,000");
            println!();

            match breach_claim.validate() {
                Ok(_) => {
                    println!("✅ 債務不履行責任成立");
                    println!("   Breach of obligation liability established");
                    println!();
                    println!("   推定損害額: ¥{}", breach_claim.estimated_damages());
                    println!();
                    println!("💡 Legal Note:");
                    println!("   Article 415は契約違反による損害賠償の基本条文です。");
                    println!(
                        "   Article 415 is the fundamental provision for contract breach damages."
                    );
                    println!("   不法行為（709条）と異なり、故意・過失は不要で、");
                    println!("   「帰責事由」（債務者に責任があること）のみが必要です。");
                    println!();
                    println!("   Unlike tort (Article 709), intent/negligence is not required,");
                    println!("   only 'attribution to debtor' is necessary.");
                }
                Err(e) => {
                    println!("❌ 検証失敗: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ ビルドエラー: {:?}", e);
        }
    }
}

// 2. 予見可能性と損害範囲（Hadley v. Baxendale-style analysis）
fn detailed_breach_with_foreseeability() {
    println!("2. 予見可能性と損害範囲 (Foreseeability and Damage Scope)");
    println!("   事案: 配送遅延により工場が操業停止");
    println!("   Case: Delivery delay causes factory shutdown");
    println!();

    // 直接損害（予見可能）
    // Direct damages (foreseeable)
    let _direct_claim = Article415::new()
        .with_obligation(ObligationType::Delivery {
            description: "重要部品の納期までの配送".to_string(),
        })
        .with_breach(BreachType::DelayedPerformance { days_late: 7 })
        .with_attribution(Attribution::new(
            AttributionType::Negligence,
            "配送手配を怠った",
        ))
        .with_damage(Damage::new(300_000, "追加配送費用"))
        .with_causal_link(CausalLink::Direct)
        .creditor("製造会社A")
        .debtor("配送業者B")
        .with_due_date("2026-01-10");

    println!("【直接損害】(Direct Damages):");
    println!("  追加配送費用（急送料金）: ¥300,000");
    println!("  → 予見可能 (Foreseeable): ✅");
    println!();

    // 間接損害（予見困難？）
    // Indirect damages (foreseeable?)
    let indirect_claim = Article415::new()
        .with_obligation(ObligationType::Delivery {
            description: "重要部品の納期までの配送".to_string(),
        })
        .with_breach(BreachType::DelayedPerformance { days_late: 7 })
        .with_attribution(Attribution::new(
            AttributionType::Negligence,
            "配送手配を怠った",
        ))
        .with_damage(Damage::new(10_000_000, "工場操業停止による逸失利益"))
        .with_causal_link(CausalLink::Adequate(
            "部品なしで工場が停止、契約時に通知済み",
        ))
        .creditor("製造会社A")
        .debtor("配送業者B");

    println!("【間接損害】(Indirect/Consequential Damages):");
    println!("  工場操業停止による逸失利益: ¥10,000,000");
    println!("  → 予見可能性の判断: 🔍");
    println!();

    match indirect_claim.build() {
        Ok(claim) => match claim.validate() {
            Ok(_) => {
                println!("   判断要素 (Key factors for foreseeability):");
                println!("   • 契約時に部品の重要性を通知していたか？");
                println!("     (Was the part's criticality communicated at contract time?)");
                println!("   • 遅延で工場停止することは当然予見可能か？");
                println!("     (Was factory shutdown reasonably foreseeable from delay?)");
                println!();
                println!("   Hadley v. Baxendale原則:");
                println!("   「契約時に通常予見できた損害」または");
                println!("   「当事者が特別の事情を知っていた場合の損害」のみ賠償");
                println!();
                println!("   Hadley v. Baxendale principle:");
                println!("   Damages recoverable are those 'fairly and reasonably");
                println!("   considered arising naturally' or 'in the contemplation");
                println!("   of both parties at contract time'");
                println!();
                println!("   📊 結論 (Conclusion):");
                println!("   直接損害 ¥300,000 → 確実に認容");
                println!("   間接損害 ¥10,000,000 → 通知の有無で判断が分かれる");
                println!();
                println!("   Direct damages ¥300,000 → Certainly recoverable");
                println!("   Indirect damages ¥10,000,000 → Depends on notice");
            }
            Err(e) => {
                println!("❌ 検証失敗: {:?}", e);
            }
        },
        Err(e) => {
            println!("❌ ビルドエラー: {:?}", e);
        }
    }
}

// 3. 損害軽減義務（Creditor's duty to mitigate）
fn mitigation_obligation_check() {
    println!("3. 損害軽減義務 (Duty to Mitigate Damages)");
    println!("   事案: 賃貸契約の中途解約と再募集義務");
    println!("   Case: Lease termination and duty to re-let");
    println!();

    // 賃借人の中途解約（債務不履行）
    // Tenant's premature termination (breach)
    let breach_claim = Article415::new()
        .with_obligation(ObligationType::Monetary {
            amount: 100_000,
            currency: "JPY".to_string(),
        })
        .with_breach(BreachType::NonPerformance)
        .with_attribution(Attribution::new(
            AttributionType::Intentional,
            "一方的に契約解除を通告",
        ))
        .with_damage(Damage::new(1_200_000, "残期間12ヶ月分の家賃"))
        .with_causal_link(CausalLink::Direct)
        .creditor("家主")
        .debtor("賃借人")
        .contract_date("2025-01-01");

    println!("事実関係 (Facts):");
    println!("  • 2年契約の賃貸借、月額家賃 ¥100,000");
    println!("    (2-year lease, monthly rent ¥100,000)");
    println!("  • 1年目で賃借人が一方的に解約");
    println!("    (Tenant terminated unilaterally after 1 year)");
    println!("  • 残期間 12ヶ月 → 損害額 ¥1,200,000");
    println!("    (Remaining 12 months → Claimed damages ¥1,200,000)");
    println!();

    match breach_claim.build() {
        Ok(_claim) => {
            println!("✅ Article 415債務不履行成立");
            println!("   Breach of obligation established");
            println!();
            println!("   しかし...");
            println!("   However...");
            println!();
            println!("   🔍 損害軽減義務の検討 (Mitigation Analysis):");
            println!();
            println!("   債権者（家主）の義務:");
            println!("   • 新たな賃借人を探す努力をすべき");
            println!("     (Creditor/landlord should seek new tenant)");
            println!("   • 空室期間を合理的に短縮する義務");
            println!("     (Duty to reasonably minimize vacancy period)");
            println!();
            println!("   シナリオA: 家主が何もしない場合");
            println!("   Scenario A: Landlord does nothing");
            println!("   → 全額 ¥1,200,000 の請求は認められない可能性");
            println!("   → Full claim of ¥1,200,000 may be denied");
            println!();
            println!("   シナリオB: 家主が募集し3ヶ月後に新賃借人");
            println!("   Scenario B: Landlord seeks tenant, finds one after 3 months");
            println!("   → 認容額: ¥300,000（3ヶ月分のみ）");
            println!("   → Recoverable: ¥300,000 (only 3 months)");
            println!();
            println!("   💡 Legal Principle:");
            println!("   判例法理: 「債権者も損害の拡大を防ぐ義務を負う」");
            println!("   Case law: 'Creditors bear duty to prevent damage expansion'");
            println!();
            println!("   比較法:");
            println!("   • 英米法: Duty to mitigate（明文化）");
            println!("   • 日本法: 信義則（民法1条2項）から導出");
            println!();
            println!("   Comparative law:");
            println!("   • Common law: Explicit duty to mitigate");
            println!("   • Japanese law: Derived from good faith principle (Article 1(2))");
            println!();
            println!("   📊 実務上の推奨 (Practical Recommendation):");
            println!("   賃貸人は速やかに再募集し、その証拠を保全すること。");
            println!("   Landlords should promptly seek new tenants and preserve evidence.");
        }
        Err(e) => {
            println!("❌ ビルドエラー: {:?}", e);
        }
    }
}
