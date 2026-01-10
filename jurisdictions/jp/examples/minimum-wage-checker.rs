//! Minimum Wage Checker (最低賃金チェッカー)
//!
//! Interactive tool to check if a monthly salary meets the minimum wage requirements
//! for a specific prefecture.
//!
//! Usage:
//! ```bash
//! cargo run --example minimum-wage-checker
//! ```

use legalis_jp::labor_law::{Prefecture, minimum_wage};
use std::io::{self, Write};

fn main() {
    println!("{}", "=".repeat(80));
    println!("最低賃金チェッカー (Minimum Wage Checker)");
    println!("{}", "=".repeat(80));
    println!();

    // Get prefecture
    let prefecture = select_prefecture();

    // Get employment details
    let monthly_salary = get_monthly_salary();
    let hours_per_day = get_hours_per_day();
    let days_per_week = get_days_per_week();

    println!();
    println!("{}", "-".repeat(80));
    println!("検証結果 (Validation Results)");
    println!("{}", "-".repeat(80));

    // Calculate hourly rate
    let monthly_hours = minimum_wage::calculate_monthly_hours(hours_per_day, days_per_week);
    let hourly_rate = monthly_salary as f64 / monthly_hours;

    // Get minimum wage for prefecture
    let min_wage = minimum_wage::get_minimum_wage(prefecture, chrono::Utc::now().date_naive());

    println!();
    println!("📍 勤務地: {:?}", prefecture);
    println!("💰 月給: ¥{}", monthly_salary);
    println!(
        "⏰ 労働時間: {}時間/日、週{}日",
        hours_per_day, days_per_week
    );
    println!("📊 月間労働時間: {:.1}時間", monthly_hours);
    println!();
    println!("計算された時給: ¥{:.0}/時", hourly_rate);
    println!(
        "最低賃金 ({}): ¥{}/時",
        prefecture_name(prefecture),
        min_wage
    );
    println!();

    // Check compliance
    if (hourly_rate as u64) >= min_wage {
        println!("✅ 合格 (COMPLIANT)");
        println!();
        println!(
            "この給与は{}の最低賃金を満たしています。",
            prefecture_name(prefecture)
        );
        println!(
            "差額: +¥{}/時 (¥{:.0} - ¥{})",
            (hourly_rate as u64) - min_wage,
            hourly_rate,
            min_wage
        );
    } else {
        println!("❌ 違反 (NON-COMPLIANT)");
        println!();
        println!(
            "⚠️ この給与は{}の最低賃金を下回っています!",
            prefecture_name(prefecture)
        );
        println!(
            "不足額: -¥{}/時 (¥{:.0} - ¥{})",
            min_wage - (hourly_rate as u64),
            hourly_rate,
            min_wage
        );
        println!();
        println!("📋 必要な措置:");

        // Calculate required increase
        let required_monthly = (min_wage as f64 * monthly_hours).ceil() as u64;
        let increase_needed = required_monthly - monthly_salary;

        println!("  1. 月給を¥{}以上に増額", required_monthly);
        println!("     (現在より+¥{}の増額が必要)", increase_needed);
        println!("  または");
        println!("  2. 労働時間を削減");
        println!();
        println!("⚖️ 法的根拠: 最低賃金法");
        println!("   罰則: 最低賃金額以上の賃金支払義務違反は50万円以下の罰金");
    }

    println!();
    println!("{}", "=".repeat(80));

    // Show regional comparison
    show_regional_comparison(monthly_salary, hours_per_day, days_per_week);
}

fn select_prefecture() -> Prefecture {
    println!("都道府県を選択してください (Select Prefecture):");
    println!();
    println!("主要都道府県 (Major Prefectures):");
    println!("  1. 東京都 (Tokyo) - ¥1,113/時");
    println!("  2. 神奈川県 (Kanagawa) - ¥1,112/時");
    println!("  3. 大阪府 (Osaka) - ¥1,064/時");
    println!("  4. 愛知県 (Aichi) - ¥1,027/時");
    println!("  5. 埼玉県 (Saitama) - ¥1,028/時");
    println!("  6. 千葉県 (Chiba) - ¥1,026/時");
    println!("  7. 北海道 (Hokkaido) - ¥960/時");
    println!("  8. 福岡県 (Fukuoka) - ¥941/時");
    println!("  9. 京都府 (Kyoto) - ¥1,008/時");
    println!(" 10. 兵庫県 (Hyogo) - ¥1,001/時");
    println!(" 11. 沖縄県 (Okinawa) - ¥896/時");
    println!();

    loop {
        print!("番号を入力 (1-11): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(1) => return Prefecture::Tokyo,
            Ok(2) => return Prefecture::Kanagawa,
            Ok(3) => return Prefecture::Osaka,
            Ok(4) => return Prefecture::Aichi,
            Ok(5) => return Prefecture::Saitama,
            Ok(6) => return Prefecture::Chiba,
            Ok(7) => return Prefecture::Hokkaido,
            Ok(8) => return Prefecture::Fukuoka,
            Ok(9) => return Prefecture::Kyoto,
            Ok(10) => return Prefecture::Hyogo,
            Ok(11) => return Prefecture::Okinawa,
            _ => println!("❌ 無効な入力です。1-11の番号を入力してください。"),
        }
    }
}

fn get_monthly_salary() -> u64 {
    loop {
        print!("\n月給を入力してください (円): ¥");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().replace(",", "").parse::<u64>() {
            Ok(salary) if salary > 0 => return salary,
            _ => println!("❌ 無効な金額です。正の整数を入力してください。"),
        }
    }
}

fn get_hours_per_day() -> u32 {
    loop {
        print!("1日の労働時間を入力してください (時間): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(hours) if hours > 0 && hours <= 24 => {
                if hours > 8 {
                    println!("⚠️ 注意: 1日8時間を超える場合、第36条協定(36協定)が必要です。");
                }
                return hours;
            }
            _ => println!("❌ 無効な時間です。1-24の整数を入力してください。"),
        }
    }
}

fn get_days_per_week() -> u32 {
    loop {
        print!("週の労働日数を入力してください (日): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(days) if days > 0 && days <= 7 => {
                let weekly_hours = days * 8; // Assuming standard 8 hours/day for warning
                if weekly_hours > 40 {
                    println!("⚠️ 注意: 週40時間を超える場合、第36条協定(36協定)が必要です。");
                }
                return days;
            }
            _ => println!("❌ 無効な日数です。1-7の整数を入力してください。"),
        }
    }
}

fn prefecture_name(prefecture: Prefecture) -> &'static str {
    match prefecture {
        Prefecture::Tokyo => "東京都",
        Prefecture::Kanagawa => "神奈川県",
        Prefecture::Osaka => "大阪府",
        Prefecture::Aichi => "愛知県",
        Prefecture::Saitama => "埼玉県",
        Prefecture::Chiba => "千葉県",
        Prefecture::Hokkaido => "北海道",
        Prefecture::Fukuoka => "福岡県",
        Prefecture::Kyoto => "京都府",
        Prefecture::Hyogo => "兵庫県",
        Prefecture::Okinawa => "沖縄県",
        _ => "その他",
    }
}

fn show_regional_comparison(monthly_salary: u64, hours_per_day: u32, days_per_week: u32) {
    println!("📊 地域別比較 (Regional Comparison)");
    println!("{}", "-".repeat(80));
    println!();
    println!("同じ給与(¥{})での各都道府県での適合状況:", monthly_salary);
    println!();

    let monthly_hours = minimum_wage::calculate_monthly_hours(hours_per_day, days_per_week);
    let hourly_rate = monthly_salary as f64 / monthly_hours;

    let prefectures = vec![
        (Prefecture::Tokyo, "東京都", 1_113),
        (Prefecture::Kanagawa, "神奈川県", 1_112),
        (Prefecture::Osaka, "大阪府", 1_064),
        (Prefecture::Saitama, "埼玉県", 1_028),
        (Prefecture::Aichi, "愛知県", 1_027),
        (Prefecture::Chiba, "千葉県", 1_026),
        (Prefecture::Kyoto, "京都府", 1_008),
        (Prefecture::Hyogo, "兵庫県", 1_001),
        (Prefecture::Hokkaido, "北海道", 960),
        (Prefecture::Fukuoka, "福岡県", 941),
        (Prefecture::Okinawa, "沖縄県", 896),
    ];

    for (_pref, name, min_wage) in prefectures {
        let status = if (hourly_rate as u64) >= min_wage {
            "✅"
        } else {
            "❌"
        };
        let diff = (hourly_rate as i64) - (min_wage as i64);
        let diff_str = if diff >= 0 {
            format!("+¥{}", diff)
        } else {
            format!("-¥{}", -diff)
        };

        println!(
            "{} {:12} ¥{:>4}/時 | 時給¥{:.0} {} ({})",
            status,
            name,
            min_wage,
            hourly_rate,
            diff_str,
            if diff >= 0 { "合格" } else { "違反" }
        );
    }

    println!();
    println!("💡 ヒント: 最低賃金は毎年10月に改定されます。");
    println!("{}", "=".repeat(80));
}
