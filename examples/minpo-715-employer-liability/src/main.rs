use legalis_jp::tort::{
    Article709, Article715, CausalLink, Damage, EmploymentType, Intent, ProtectedInterest,
    validate_tort_claim,
};

fn main() {
    println!("=== Article 715 Employer/Supervisor Liability (使用者責任) Demo ===\n");

    check_direct_employer_liability();
    println!("\n{}\n", "=".repeat(80));

    check_supervisor_duty_violation();
    println!("\n{}\n", "=".repeat(80));

    corporate_negligent_hiring();
}

// 1. 直接的な使用者責任（Employee negligence → employer liable）
fn check_direct_employer_liability() {
    println!("1. 直接的な使用者責任 (Direct Employer Liability)");
    println!("   事案: 配達業務中のドライバーが交通事故");
    println!("   Case: Delivery driver causes traffic accident during work");
    println!();

    // まず従業員のArticle 709不法行為を確立
    // First establish employee's Article 709 tort
    let employee_tort = Article709::new()
        .with_act("配達中に前方不注意で歩行者に衝突")
        .with_intent(Intent::NegligenceWithDuty {
            duty_of_care: "前方注視義務違反".to_string(),
        })
        .with_victim_interest(ProtectedInterest::BodyAndHealth)
        .with_damage(Damage::new(800_000, "治療費 + 休業損害"))
        .with_causal_link(CausalLink::Direct);

    match validate_tort_claim(&employee_tort) {
        Ok(_liability) => {
            println!("✅ 従業員の709条不法行為成立");
            println!("   Employee's Article 709 tort established");
            println!("   損害額: ¥800,000");
            println!();

            // Article 715で使用者責任を検討
            // Examine employer liability under Article 715
            let employer_liability = Article715::new()
                .employee_tort(employee_tort)
                .employer("株式会社ABC配送")
                .employee("配達員 山田太郎")
                .employment_type(EmploymentType::FullTime)
                .during_business_execution(true)
                .business_context("通常の配達業務中の事故");

            match employer_liability.build() {
                Ok(claim) => match claim.validate() {
                    Ok(_) => {
                        println!("✅ Article 715使用者責任成立");
                        println!("   Employer vicarious liability established");
                        println!();
                        println!("   判断要素 (Key factors):");
                        println!("   • 使用関係: 正社員 (Employment: Full-time employee)");
                        println!(
                            "   • 事業執行性: 通常の配達業務中 (During business: Regular delivery)"
                        );
                        println!("   • 外形理論: 業務の外形上明白 (Apparent authority)");
                        println!();
                        println!("💡 Legal Consequence:");
                        println!("   使用者（会社）は従業員の不法行為について連帯責任を負います。");
                        println!("   被害者は従業員または使用者のいずれにも請求可能です。");
                        println!();
                        println!("   The employer is jointly liable for the employee's tort.");
                        println!("   The victim may claim from either the employee or employer.");
                    }
                    Err(e) => {
                        println!("❌ Article 715検証失敗: {:?}", e);
                    }
                },
                Err(e) => {
                    println!("❌ ビルドエラー: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ 従業員の709条不成立: {:?}", e);
        }
    }
}

// 2. 監督義務違反（Parent/supervisor liability for child）
fn check_supervisor_duty_violation() {
    println!("2. 監督義務違反 (Supervisor Duty Violation)");
    println!("   事案: アルバイト従業員が勤務中に顧客情報を漏洩");
    println!("   Case: Part-time employee leaks customer information during work");
    println!();

    // アルバイト従業員の不法行為
    // Part-time employee's tort
    let employee_tort = Article709::new()
        .with_act("顧客の個人情報を無断でSNSに投稿")
        .with_intent(Intent::Intentional { age: 20 })
        .with_victim_interest(ProtectedInterest::Privacy)
        .with_damage(Damage::new(300_000, "プライバシー侵害"))
        .with_causal_link(CausalLink::Direct);

    match validate_tort_claim(&employee_tort) {
        Ok(_liability) => {
            println!("✅ アルバイト従業員の709条不法行為成立");
            println!("   Part-time employee's Article 709 tort established");
            println!();

            // 使用者責任の検討（監督義務の観点）
            // Employer liability (supervision duty perspective)
            let employer_liability = Article715::new()
                .employee_tort(employee_tort)
                .employer("飲食店オーナー")
                .employee("アルバイト 佐藤花子")
                .employment_type(EmploymentType::PartTime)
                .during_business_execution(true)
                .business_context("勤務中に店舗で撮影した顧客情報を投稿")
                .reasonable_care_supervision(false); // 監督義務違反

            match employer_liability.build() {
                Ok(claim) => {
                    println!("   雇用形態: アルバイト (Employment type: Part-time)");
                    println!("   事業執行性: 勤務時間中 (During business hours)");
                    println!();

                    if claim.is_liability_established() {
                        println!("✅ 使用者責任成立");
                        println!("   Employer liability established");
                        println!();
                        println!("   理由 (Reasoning):");
                        println!("   • 個人情報保護の教育・監督義務を怠った");
                        println!("     (Failed to educate/supervise on privacy protection)");
                        println!("   • SNS利用規程が不十分だった");
                        println!("     (Inadequate social media usage policies)");
                        println!();
                        println!("💡 Practical Implication:");
                        println!("   アルバイトであっても、業務中の行為については");
                        println!("   使用者が監督義務を負います。適切な研修と監督が必須です。");
                        println!();
                        println!(
                            "   Even for part-time workers, employers bear supervision duties"
                        );
                        println!(
                            "   for acts during work. Proper training and oversight are essential."
                        );
                    } else {
                        println!("❌ 使用者責任不成立");
                    }
                }
                Err(e) => {
                    println!("❌ ビルドエラー: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ 従業員の709条不成立: {:?}", e);
        }
    }
}

// 3. 企業の過失ある選任（Negligent hiring case）
fn corporate_negligent_hiring() {
    println!("3. 過失ある選任と監督 (Negligent Hiring and Supervision)");
    println!("   事案: 運送会社が無免許ドライバーを雇用");
    println!("   Case: Transport company hired unlicensed driver");
    println!();

    // 無免許ドライバーの事故
    // Unlicensed driver's accident
    let employee_tort = Article709::new()
        .with_act("無免許運転で配送中に車両を破損")
        .with_intent(Intent::NegligenceWithDuty {
            duty_of_care: "免許保持義務違反 + 前方注視義務違反".to_string(),
        })
        .with_victim_interest(ProtectedInterest::Property("駐車車両"))
        .with_damage(Damage::new(1_200_000, "車両修理費 + 代車費用"))
        .with_causal_link(CausalLink::Direct);

    match validate_tort_claim(&employee_tort) {
        Ok(_liability) => {
            println!("✅ 無免許ドライバーの709条不法行為成立");
            println!("   Unlicensed driver's Article 709 tort established");
            println!("   損害額: ¥1,200,000");
            println!();

            // 使用者の過失（選任・監督義務違反）
            // Employer's fault (negligent hiring/supervision)
            let employer_liability = Article715::new()
                .employee_tort(employee_tort)
                .employer("運送会社XYZ")
                .employee("無免許ドライバー 田中一郎")
                .employment_type(EmploymentType::Contract)
                .during_business_execution(true)
                .business_context("配送業務中の事故")
                .reasonable_care_appointment(false) // 選任に相当の注意なし
                .care_evidence("免許確認を怠り、形式的な面接のみで採用");

            match employer_liability.build() {
                Ok(claim) => {
                    println!("   使用者の過失 (Employer's negligence):");
                    println!("   • 選任の際の注意義務違反");
                    println!("     (Negligence in hiring process)");
                    println!("   • 免許証の確認を怠った");
                    println!("     (Failed to verify driver's license)");
                    println!("   • 形式的な面接のみで技能確認せず");
                    println!("     (No skills verification, only formal interview)");
                    println!();

                    if claim.is_liability_established() {
                        println!("✅ 使用者責任成立（免責不可）");
                        println!("   Employer liability established (cannot escape liability)");
                        println!();
                        println!("   📋 Article 715 Defense Analysis:");
                        println!();
                        println!("   免責の抗弁（Article 715 Proviso）:");
                        println!("   「使用者が被用者の選任及びその事業の監督について");
                        println!("    相当の注意をしたとき」は免責される");
                        println!();
                        println!("   \"If the employer exercised reasonable care in");
                        println!("    appointing the employee and supervising the undertaking\"");
                        println!();
                        println!("   本件では:");
                        println!("   ❌ 選任の注意: 免許確認せず → 相当の注意なし");
                        println!("   ❌ 監督の注意: 無免許を見過ごす → 相当の注意なし");
                        println!();
                        println!("   In this case:");
                        println!("   ❌ Hiring care: No license check → No reasonable care");
                        println!(
                            "   ❌ Supervision: Overlooked unlicensed status → No reasonable care"
                        );
                        println!();
                        println!("   結論: 使用者は免責されず、全額賠償責任を負う");
                        println!(
                            "   Conclusion: Employer cannot escape liability, must pay full damages"
                        );
                    } else {
                        println!("⚠️  使用者責任の成否は司法判断を要する");
                    }
                }
                Err(e) => {
                    println!("❌ ビルドエラー: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ 従業員の709条不成立: {:?}", e);
        }
    }
}
