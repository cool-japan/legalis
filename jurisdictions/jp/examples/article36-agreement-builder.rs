//! Article 36 Agreement Builder (36協定ビルダー)
//!
//! Interactive tool to create and validate Article 36 Agreements (overtime agreements)
//! according to Japanese Labor Standards Act.
//!
//! Usage:
//! ```bash
//! cargo run --example article36-agreement-builder
//! ```

use chrono::{Duration, Utc};
use legalis_jp::labor_law::Article36Agreement;
use std::io::{self, Write};

fn main() {
    println!("{}", "=".repeat(80));
    println!("36協定ビルダー (Article 36 Agreement Builder)");
    println!("{}", "=".repeat(80));
    println!();
    println!("労働基準法第36条に基づく時間外・休日労働協定を作成します。");
    println!();

    // Build agreement interactively
    let agreement = build_agreement();

    println!();
    println!("{}", "=".repeat(80));
    println!("作成された36協定 (Generated Agreement)");
    println!("{}", "=".repeat(80));
    println!();

    display_agreement(&agreement);

    println!();
    println!("{}", "-".repeat(80));
    println!("検証結果 (Validation Results)");
    println!("{}", "-".repeat(80));
    println!();

    // Validate agreement
    validate_and_report(&agreement);

    println!();
    println!("{}", "=".repeat(80));
    println!("📋 次のステップ (Next Steps)");
    println!("{}", "-".repeat(80));
    println!();
    println!("1. 労働者代表の選出");
    println!("   - 従業員の過半数を代表する者を民主的に選出");
    println!();
    println!("2. 協定書への署名");
    println!("   - 事業主と労働者代表が協定書に署名");
    println!();
    println!("3. 労働基準監督署への届出");
    println!("   - 管轄の労働基準監督署に協定書を提出");
    println!("   - 届出様式: 様式第9号");
    println!();
    println!("4. 社内での周知");
    println!("   - 協定内容を全従業員に周知");
    println!("   - 職場への掲示または書面交付");
    println!();
    println!("5. 勤怠管理システムの設定");
    println!("   - 協定で定めた上限を勤怠システムに設定");
    println!("   - 上限超過を防ぐアラート設定");
    println!();
    println!("{}", "=".repeat(80));
}

fn build_agreement() -> Article36Agreement {
    println!("【基本情報】");
    println!();

    // Employer name
    let employer_name = get_input("事業主名を入力してください: ");

    // Labor representative
    let labor_representative = get_input("労働者代表の氏名を入力してください: ");

    // Effective period
    println!();
    println!("【有効期間】");
    println!("※ 通常は1年間です");
    println!();

    let effective_date = Utc::now().date_naive();
    let expiration_date = (Utc::now() + Duration::days(365)).date_naive();

    println!("有効期間: {} ～ {}", effective_date, expiration_date);

    // Overtime limits
    println!();
    println!("【時間外労働の上限】");
    println!("※ 労働基準法の標準上限: 月45時間、年360時間");
    println!();

    let max_overtime_per_day = get_number("1日の時間外労働上限 (時間): ", 1, 8, 3);

    let max_overtime_per_month =
        get_number("1ヶ月の時間外労働上限 (時間、標準45時間): ", 1, 45, 45);

    let max_overtime_per_year =
        get_number("1年の時間外労働上限 (時間、標準360時間): ", 1, 360, 360);

    // Special circumstances
    println!();
    println!("【特別条項】");
    println!("一時的、例外的な事情がある場合の上限延長");
    println!();

    let has_special = get_yes_no("特別条項を設定しますか？ (y/n): ");

    let (special_max_per_month, special_months_per_year) = if has_special {
        println!();
        println!("⚠️ 特別条項の上限:");
        println!("  - 月最大100時間 (休日労働含む)");
        println!("  - 年6ヶ月まで");
        println!();

        let special_month = get_number(
            "特別条項の月上限 (時間、最大100): ",
            max_overtime_per_month + 1,
            100,
            80,
        );

        let special_months = get_number("特別条項の適用可能月数 (月、最大6): ", 1, 6, 6);

        (Some(special_month), Some(special_months))
    } else {
        (None, None)
    };

    // Permitted reasons
    println!();
    println!("【時間外労働の理由】");
    println!("※ 具体的な理由を入力してください");
    println!();

    let mut permitted_reasons = Vec::new();
    let mut count = 1;

    loop {
        let reason = get_input(&format!("理由 {}: ", count));
        if reason.is_empty() {
            if count == 1 {
                println!("❌ 少なくとも1つの理由が必要です。");
                continue;
            }
            break;
        }
        permitted_reasons.push(reason);
        count += 1;

        if count > 10 {
            println!("⚠️ 理由は10個までです。");
            break;
        }

        print!("さらに理由を追加しますか？ (y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            break;
        }
    }

    Article36Agreement {
        employer_name,
        labor_representative,
        effective_date,
        expiration_date,
        max_overtime_per_day,
        max_overtime_per_month,
        max_overtime_per_year,
        has_special_circumstances: has_special,
        special_max_per_month,
        special_months_per_year,
        permitted_reasons,
    }
}

fn display_agreement(agreement: &Article36Agreement) {
    println!("事業主名: {}", agreement.employer_name);
    println!("労働者代表: {}", agreement.labor_representative);
    println!();
    println!(
        "有効期間: {} ～ {}",
        agreement.effective_date, agreement.expiration_date
    );
    println!();
    println!("【標準上限】");
    println!("  1日: {}時間", agreement.max_overtime_per_day);
    println!("  1ヶ月: {}時間", agreement.max_overtime_per_month);
    println!("  1年: {}時間", agreement.max_overtime_per_year);
    println!();

    if agreement.has_special_circumstances {
        println!("【特別条項】");
        if let (Some(monthly), Some(months)) = (
            agreement.special_max_per_month,
            agreement.special_months_per_year,
        ) {
            println!("  月上限: {}時間", monthly);
            println!("  適用月数: {}ヶ月/年", months);
        }
        println!();
    }

    println!("【時間外労働の理由】");
    for (i, reason) in agreement.permitted_reasons.iter().enumerate() {
        println!("  {}. {}", i + 1, reason);
    }
}

fn validate_and_report(agreement: &Article36Agreement) {
    // Check standard limits
    let within_standard = agreement.is_within_standard_limits();
    println!(
        "📊 標準上限チェック: {}",
        if within_standard {
            "✅ 合格"
        } else {
            "❌ 違反"
        }
    );

    if !within_standard {
        println!("   ⚠️ 月45時間、年360時間の標準上限を超過しています");
    }

    // Check special circumstances
    if agreement.has_special_circumstances {
        let special_valid = agreement.is_special_circumstances_valid();
        println!(
            "📊 特別条項チェック: {}",
            if special_valid {
                "✅ 合格"
            } else {
                "❌ 違反"
            }
        );

        if !special_valid {
            println!("   ⚠️ 特別条項の設定が無効です");
            println!("   - 月上限は100時間以内");
            println!("   - 適用月数は6ヶ月以内");
        }
    }

    // Check validity period
    let currently_valid = agreement.is_currently_valid(Utc::now().date_naive());
    println!(
        "📊 有効期間チェック: {}",
        if currently_valid {
            "✅ 有効"
        } else {
            "❌ 期限切れ"
        }
    );

    // Overall validation
    println!();
    match agreement.validate() {
        Ok(_) => {
            println!("✅ この協定は有効です!");
            println!();
            println!("📋 労働基準法第36条の要件を満たしています。");

            if agreement.max_overtime_per_month < 45 {
                let buffer = 45 - agreement.max_overtime_per_month;
                println!(
                    "💡 標準上限まで{}時間の余裕があります（保守的な設定）。",
                    buffer
                );
            }
        }
        Err(e) => {
            println!("❌ この協定は無効です!");
            println!();
            println!("エラー: {}", e);
            println!();
            println!("📋 修正が必要です。上記のエラーを確認してください。");
        }
    }

    // Additional recommendations
    println!();
    println!("💡 推奨事項:");

    if agreement.max_overtime_per_month == 45 {
        println!("  - 上限を標準の45時間より低く設定することを検討してください");
        println!("    （例: 40時間で5時間のバッファを確保）");
    }

    if agreement.has_special_circumstances {
        println!("  - 特別条項の適用は一時的、例外的な事情に限定してください");
        println!("  - 2-6ヶ月平均で80時間を超えないよう管理してください");
    }

    println!("  - 従業員の健康確保措置を必ず実施してください");
    println!("  - 月次で実績を確認し、上限超過を防いでください");
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

fn get_number(prompt: &str, min: u32, max: u32, default: u32) -> u32 {
    loop {
        print!("{} (デフォルト: {}): ", prompt, default);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return default;
        }

        match trimmed.parse::<u32>() {
            Ok(num) if num >= min && num <= max => return num,
            _ => println!("❌ {}-{}の範囲で入力してください。", min, max),
        }
    }
}

fn get_yes_no(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" | "はい" => return true,
            "n" | "no" | "いいえ" => return false,
            _ => println!("❌ y/n で答えてください。"),
        }
    }
}
