//! Non-Compete Clause Analyzer (競業避止条項分析器)
//!
//! Interactive tool to analyze the reasonableness of non-compete clauses
//! under Civil Code Article 90 (public policy doctrine).
//!
//! Usage:
//! ```bash
//! cargo run --example non-compete-analyzer
//! ```

use legalis_jp::labor_law::{
    NonCompeteClause, ReasonablenessReport, validate_non_compete_reasonableness,
};
use std::io::{self, Write};

fn main() {
    println!("{}", "=".repeat(80));
    println!("競業避止条項分析器 (Non-Compete Clause Analyzer)");
    println!("{}", "=".repeat(80));
    println!();
    println!("民法第90条（公序良俗）に基づき、競業避止義務条項の合理性を分析します。");
    println!();
    println!("【競業避止義務条項とは】");
    println!("従業員が退職後、一定期間・地域において同業他社への就職や");
    println!("競業行為を行うことを制限する契約条項です。");
    println!();
    println!("【合理性の判断基準】");
    println!("  1. 期間の合理性 (6-12ヶ月が目安)");
    println!("  2. 地理的範囲の合理性 (限定的であること)");
    println!("  3. 代償措置の有無 (補償金の支払い)");
    println!("  4. 禁止される活動の範囲 (具体的であること)");
    println!();

    // Get non-compete clause details
    let clause = build_non_compete_clause();

    // Get employee position
    println!();
    let position = get_input("従業員の職位を入力してください (例: エンジニア、営業部長): ");

    println!();
    println!("{}", "=".repeat(80));
    println!("入力された競業避止条項");
    println!("{}", "=".repeat(80));
    println!();

    display_clause(&clause);

    println!();
    println!("{}", "=".repeat(80));
    println!("合理性分析結果");
    println!("{}", "=".repeat(80));
    println!();

    analyze_clause(&clause, &position);

    println!();
    println!("{}", "=".repeat(80));
}

fn build_non_compete_clause() -> NonCompeteClause {
    println!("【制限期間】");
    println!();

    let duration_months = loop {
        print!("退職後の制限期間 (ヶ月): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(months) if months > 0 && months <= 60 => break months,
            _ => println!("❌ 1-60の範囲で入力してください。"),
        }
    };

    println!();
    println!("【地理的範囲】");
    println!("例: 東京23区内、関東圏、全国、全世界");
    println!();

    let geographic_scope = get_input("地理的範囲を入力してください: ");

    println!();
    println!("【禁止される活動】");
    println!("※ 具体的な活動を入力してください");
    println!();

    let mut prohibited_activities = Vec::new();
    let mut count = 1;

    loop {
        let activity = get_input(&format!("禁止活動 {}: ", count));
        prohibited_activities.push(activity);
        count += 1;

        if count > 5 {
            println!("⚠️ 禁止活動は5個までです。");
            break;
        }

        print!("さらに禁止活動を追加しますか？ (y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            break;
        }
    }

    println!();
    println!("【代償措置】");
    println!();

    print!("代償措置（補償金）を支払いますか？ (y/n): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let consideration_provided = input.trim().eq_ignore_ascii_case("y");

    let compensation_amount_jpy = if consideration_provided {
        println!();
        loop {
            print!("補償金額 (総額、円): ¥");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().replace(",", "").parse::<u64>() {
                Ok(amount) if amount > 0 => break Some(amount),
                _ => println!("❌ 無効な金額です。"),
            }
        }
    } else {
        None
    };

    NonCompeteClause {
        duration_months,
        geographic_scope,
        prohibited_activities,
        consideration_provided,
        compensation_amount_jpy,
    }
}

fn display_clause(clause: &NonCompeteClause) {
    println!("📋 制限期間: {}ヶ月", clause.duration_months);
    println!("🌍 地理的範囲: {}", clause.geographic_scope);
    println!();
    println!("🚫 禁止される活動:");
    for (i, activity) in clause.prohibited_activities.iter().enumerate() {
        println!("   {}. {}", i + 1, activity);
    }
    println!();
    if clause.consideration_provided {
        if let Some(amount) = clause.compensation_amount_jpy {
            println!("💰 代償措置: あり (¥{})", amount);
        } else {
            println!("💰 代償措置: あり (金額未定)");
        }
    } else {
        println!("💰 代償措置: なし");
    }
}

fn analyze_clause(clause: &NonCompeteClause, position: &str) {
    // Validate using the system
    match validate_non_compete_reasonableness(clause, position) {
        Ok(report) => {
            // Display overall result
            if report.is_reasonable() {
                println!("✅ この競業避止条項は合理的と判断されます");
            } else {
                println!("❌ この競業避止条項は不合理と判断されます");
            }

            println!();
            println!("📊 リスクレベル: {:?}", report.risk_level);
            println!();

            // Display positive factors
            if !report.positive_factors.is_empty() {
                println!("✅ 合理性を支持する要素:");
                for factor in &report.positive_factors {
                    println!("   • {}", factor);
                }
                println!();
            }

            // Display issues
            if !report.issues.is_empty() {
                println!("❌ 問題点:");
                for issue in &report.issues {
                    println!("   • {}", issue);
                }
                println!();
            }

            // Display warnings
            if !report.warnings.is_empty() {
                println!("⚠️ 警告:");
                for warning in &report.warnings {
                    println!("   • {}", warning);
                }
                println!();
            }

            // Detailed analysis by factor
            println!("{}", "-".repeat(80));
            println!("詳細分析");
            println!("{}", "-".repeat(80));
            println!();

            analyze_duration(clause.duration_months);
            analyze_geographic_scope(&clause.geographic_scope);
            analyze_consideration(
                clause.consideration_provided,
                clause.compensation_amount_jpy,
            );
            analyze_activities(&clause.prohibited_activities);

            // Recommendations
            println!();
            println!("{}", "-".repeat(80));
            println!("推奨事項");
            println!("{}", "-".repeat(80));
            println!();

            provide_recommendations(clause, &report);

            // Legal context
            println!();
            println!("{}", "-".repeat(80));
            println!("法的根拠");
            println!("{}", "-".repeat(80));
            println!();
            println!("民法第90条（公序良俗）:");
            println!("  「公の秩序又は善良の風俗に反する法律行為は、無効とする。」");
            println!();
            println!("判例:");
            println!("  競業避止義務条項は、労働者の職業選択の自由（憲法第22条）を");
            println!("  制約するため、合理的な範囲内でのみ有効とされています。");
            println!();
            println!("  裁判所は以下の要素を総合的に考慮して判断します:");
            println!("  • 使用者の正当な利益保護の必要性");
            println!("  • 労働者の職業選択の自由との均衡");
            println!("  • 代償措置の有無・程度");
            println!("  • 制限の期間・地域・職種の範囲");
        }
        Err(e) => {
            println!("❌ 分析中にエラーが発生しました: {}", e);
        }
    }
}

fn analyze_duration(months: u32) {
    println!("【期間の分析】");
    println!();

    if months <= 6 {
        println!("✅ 期間: {}ヶ月 - 短期で合理的", months);
        println!("   一般的に6ヶ月以内の制限は合理的とされています。");
    } else if months <= 12 {
        println!("⚠️ 期間: {}ヶ月 - やや長いが許容範囲内", months);
        println!("   業種や職位によっては合理的と認められる可能性があります。");
    } else if months <= 24 {
        println!("❌ 期間: {}ヶ月 - 長期で問題の可能性", months);
        println!("   1年を超える制限は、相当の理由と代償措置が必要です。");
    } else {
        println!("❌ 期間: {}ヶ月 - 過度に長い", months);
        println!("   2年を超える制限は通常、不合理と判断されます。");
    }
    println!();
}

fn analyze_geographic_scope(scope: &str) {
    println!("【地理的範囲の分析】");
    println!();

    let scope_lower = scope.to_lowercase();

    if scope_lower.contains("区") || scope_lower.contains("市") {
        println!("✅ 範囲: {} - 限定的で合理的", scope);
        println!("   市区町村レベルの制限は合理的とされやすいです。");
    } else if scope_lower.contains("県") || scope_lower.contains("都") || scope_lower.contains("府")
    {
        println!("⚠️ 範囲: {} - やや広いが状況による", scope);
        println!("   都道府県単位の制限は業種により合理性が判断されます。");
    } else if scope_lower.contains("圏") || scope_lower.contains("地方") {
        println!("❌ 範囲: {} - 広範で問題の可能性", scope);
        println!("   地方・圏域レベルの制限は正当化が困難な場合があります。");
    } else if scope_lower.contains("全国") || scope_lower.contains("日本") {
        println!("❌ 範囲: {} - 過度に広い", scope);
        println!("   全国的な制限は原則として不合理と判断されます。");
    } else if scope_lower.contains("世界") || scope_lower.contains("海外") {
        println!("❌ 範囲: {} - 極めて不合理", scope);
        println!("   グローバルな制限は通常、認められません。");
    } else {
        println!("📋 範囲: {}", scope);
        println!("   具体的な範囲の検討が必要です。");
    }
    println!();
}

fn analyze_consideration(provided: bool, amount: Option<u64>) {
    println!("【代償措置の分析】");
    println!();

    if provided {
        if let Some(amt) = amount {
            println!("✅ 代償措置: あり (¥{})", amt);
            println!("   代償措置の存在は合理性を大きく支持します。");
            println!();

            // Provide guidance on adequacy
            if amt >= 1_000_000 {
                println!("   金額は相当程度と考えられます。");
            } else if amt >= 500_000 {
                println!("   金額は一定の補償となっています。");
            } else {
                println!("   ⚠️ 金額が十分か検討の余地があります。");
            }
        } else {
            println!("⚠️ 代償措置: あり（金額未定）");
            println!("   具体的な金額の設定が必要です。");
        }
    } else {
        println!("❌ 代償措置: なし");
        println!("   代償措置がない場合、合理性が認められにくくなります。");
        println!("   特に制限が厳しい場合は、代償措置が不可欠です。");
    }
    println!();
}

fn analyze_activities(activities: &[String]) {
    println!("【禁止活動の分析】");
    println!();

    if activities.is_empty() {
        println!("⚠️ 禁止活動が指定されていません。");
        return;
    }

    let mut has_broad = false;
    let mut has_specific = false;

    for activity in activities {
        let activity_lower = activity.to_lowercase();

        if activity_lower.contains("全て")
            || activity_lower.contains("一切")
            || activity_lower.contains("あらゆる")
        {
            println!("❌ 「{}」 - 過度に広範", activity);
            has_broad = true;
        } else if activity_lower.contains("同業") || activity_lower.contains("競合") {
            println!("⚠️ 「{}」 - やや抽象的", activity);
        } else {
            println!("✅ 「{}」 - 具体的", activity);
            has_specific = true;
        }
    }

    println!();
    if has_broad {
        println!("⚠️ 過度に広範な禁止事項があります。");
        println!("   具体的な業務内容に限定することを推奨します。");
    } else if has_specific {
        println!("✅ 禁止活動は比較的具体的に記載されています。");
    }
    println!();
}

fn provide_recommendations(clause: &NonCompeteClause, report: &ReasonablenessReport) {
    if report.is_reasonable() {
        println!("この条項は概ね合理的ですが、以下の点に留意してください:");
        println!();
        println!("  • 条項の内容を従業員に十分説明すること");
        println!("  • 書面による合意を取得すること");
        println!("  • 定期的に条項の妥当性を見直すこと");
    } else {
        println!("この条項には改善が必要です。以下の変更を検討してください:");
        println!();

        if clause.duration_months > 12 {
            println!("  📅 期間を12ヶ月以内に短縮");
        }

        if !clause.consideration_provided {
            println!("  💰 代償措置（補償金）の追加");
            println!("     目安: 制限期間中の収入減少分を考慮した金額");
        }

        if clause.geographic_scope.contains("全") {
            println!("  🌍 地理的範囲を市区町村または都道府県レベルに限定");
        }

        if clause
            .prohibited_activities
            .iter()
            .any(|a| a.contains("全て") || a.contains("一切"))
        {
            println!("  🚫 禁止活動を具体的な業務内容に限定");
        }
    }

    println!();
    println!("💡 一般的な推奨事項:");
    println!("  • 業種・職位に応じた制限内容とすること");
    println!("  • 会社の正当な利益保護に必要な範囲に限定すること");
    println!("  • 従業員の生計維持を不当に妨げないこと");
    println!("  • 必要に応じて弁護士に相談すること");
}

fn get_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim().to_string();

        if !trimmed.is_empty() {
            return trimmed;
        }
        println!("❌ 入力が必要です。");
    }
}
