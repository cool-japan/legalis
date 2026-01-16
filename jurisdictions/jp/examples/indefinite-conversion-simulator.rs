//! Indefinite-Term Conversion Simulator (無期転換シミュレーター)
//!
//! Interactive tool to check eligibility for indefinite-term conversion
//! under Labor Contract Act Article 18 (5-year rule).
//!
//! Usage:
//! ```bash
//! cargo run --example indefinite-conversion-simulator
//! ```

use chrono::{Duration, NaiveDate, Utc};
use legalis_jp::labor_law::{
    EmploymentContract, EmploymentContractBuilder, EmploymentType, Prefecture, WorkPattern,
};
use std::io::{self, Write};

fn main() {
    println!("{}", "=".repeat(80));
    println!("無期転換シミュレーター (Indefinite-Term Conversion Simulator)");
    println!("{}", "=".repeat(80));
    println!();
    println!("労働契約法第18条「無期転換ルール」の適用判定を行います。");
    println!();
    println!("【5年ルールとは】");
    println!("有期労働契約が通算5年を超えて更新された場合、");
    println!("労働者の申込みにより無期労働契約に転換される制度です。");
    println!();

    // Get contract information
    let contract = build_fixed_term_contract();

    println!();
    println!("{}", "=".repeat(80));
    println!("現在の契約状況");
    println!("{}", "=".repeat(80));
    println!();

    display_contract(&contract);

    println!();
    println!("{}", "-".repeat(80));
    println!("無期転換ルール適用判定");
    println!("{}", "-".repeat(80));
    println!();

    check_conversion_eligibility(&contract);

    println!();
    println!("{}", "=".repeat(80));
}

fn build_fixed_term_contract() -> EmploymentContract {
    println!("【従業員情報】");
    println!();

    let employee_name = get_input("従業員名: ");
    let employer_name = get_input("事業主名: ");

    println!();
    println!("【契約情報】");
    println!();

    // Start date
    println!("契約開始日を入力してください:");
    let _start_date = get_date();

    // Current date (assumed to be today)
    let current_date = Utc::now();

    println!();
    println!("【給与・労働条件】");
    println!();

    let salary = get_salary();
    let (hours_per_day, days_per_week) = get_working_hours();
    let prefecture = get_prefecture();

    println!();
    println!("【契約更新歴】");
    println!();

    let _renewal_count = get_renewal_count();

    // Build the contract
    // Note: We'll use the actual start date for eligibility calculation
    // The builder requires Utc::now() but we track the real start date separately
    EmploymentContractBuilder::new()
        .with_employee(&employee_name)
        .with_employer(&employer_name)
        .with_employment_type(EmploymentType::FixedTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_salary(salary)
        .with_working_hours(hours_per_day, days_per_week)
        .with_prefecture(prefecture)
        .with_start_date(current_date)
        .with_job_description("General Employment")
        .with_work_location("Workplace")
        .build()
        .expect("Failed to build contract")
}

fn display_contract(contract: &EmploymentContract) {
    println!("従業員: {}", contract.employee_name);
    println!("事業主: {}", contract.employer_name);
    println!("契約形態: 有期雇用契約");
    println!("月給: ¥{}", contract.base_wage_jpy);
    println!(
        "労働時間: {}時間/日、週{}日",
        contract.hours_per_day, contract.days_per_week
    );
    println!("勤務地: {}", contract.work_location);
}

fn check_conversion_eligibility(contract: &EmploymentContract) {
    println!("📋 契約開始日を入力してください:");
    let start_date = get_date();

    println!();
    println!("📋 これまでの契約更新回数: ");
    let renewal_count = get_renewal_count();

    // Calculate total duration
    let today = Utc::now().date_naive();
    let total_days = (today - start_date).num_days();
    let total_years = total_days as f64 / 365.25;
    let _total_months = (total_days as f64 / 30.44).round() as u32;

    println!();
    println!("📊 通算期間:");
    println!("  開始日: {}", start_date);
    println!("  現在: {}", today);
    println!("  経過日数: {}日", total_days);
    println!("  経過年数: {:.2}年", total_years);
    println!("  契約更新: {}回", renewal_count);
    println!();

    // Check 5-year rule
    let five_years_date = start_date + Duration::days(365 * 5 + 1); // 5 years + 1 day for leap years
    let days_until_five_years = (five_years_date - today).num_days();

    if total_years >= 5.0 {
        println!("✅ 無期転換申込権が発生しています!");
        println!();
        println!("🎯 適用要件:");
        println!("  ✅ 有期労働契約である");
        println!("  ✅ 通算契約期間が5年を超えている");
        println!("  ✅ 契約が更新されている ({}回)", renewal_count);
        println!();
        println!("📋 労働者の権利:");
        println!("  1. 無期労働契約への転換を申し込むことができます");
        println!("  2. 申込み時点で無期労働契約が成立します");
        println!("  3. 事業主は申込みを拒否できません");
        println!();

        // Simulate conversion
        println!("{}", "-".repeat(80));
        println!("無期転換後の条件");
        println!("{}", "-".repeat(80));
        println!();

        simulate_conversion(contract);
    } else {
        println!("⏳ まだ無期転換申込権は発生していません");
        println!();
        println!("📊 現在の状況:");
        println!("  - 経過期間: {:.2}年", total_years);
        println!("  - 必要期間: 5年");
        println!("  - 残り期間: {:.2}年", 5.0 - total_years);
        println!("  - 5年到達日: {}", five_years_date);
        println!();

        if days_until_five_years > 0 {
            println!(
                "⏰ あと{}日で無期転換申込権が発生します",
                days_until_five_years
            );
        }

        println!();
        println!("📋 今後の流れ:");
        println!("  1. {}に5年が経過", five_years_date);
        println!("  2. その後の契約更新時に無期転換申込権が発生");
        println!("  3. 労働者が申込みを行うと無期労働契約に転換");
        println!();
    }

    // Warning about clawback period (cooling period)
    println!("⚠️ 重要な注意事項:");
    println!();
    println!("【クーリング期間】");
    println!("  契約がない期間（空白期間）が6ヶ月以上ある場合、");
    println!("  それ以前の契約期間は通算されません。");
    println!();
    println!("【不利益変更の禁止】");
    println!("  無期転換後の労働条件は、原則として");
    println!("  転換前の有期契約と同一でなければなりません。");
    println!("  給与の減額などの不利益変更は認められません。");
    println!();
}

fn simulate_conversion(original_contract: &EmploymentContract) {
    println!("現在の契約を無期雇用に転換した場合のシミュレーション:");
    println!();

    // Ask about salary increase
    print!("給与の増額を希望しますか？ (y/n): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let new_salary = if input.trim().eq_ignore_ascii_case("y") {
        println!();
        get_salary_with_validation(original_contract.base_wage_jpy)
    } else {
        original_contract.base_wage_jpy
    };

    // Ask about job description change
    print!("\n職務内容の変更を希望しますか？ (y/n): ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let new_job_description = if input.trim().eq_ignore_ascii_case("y") {
        println!();
        get_input("新しい職務内容: ")
    } else {
        original_contract.job_description.clone()
    };

    // Simulate building the converted contract
    let builder = EmploymentContractBuilder::new()
        .with_employee(&original_contract.employee_name)
        .with_employer(&original_contract.employer_name)
        .with_employment_type(EmploymentType::IndefiniteTerm) // Changed to indefinite-term
        .with_work_pattern(WorkPattern::Regular)
        .with_salary(new_salary)
        .with_working_hours(
            original_contract.hours_per_day,
            original_contract.days_per_week,
        )
        .with_prefecture(Prefecture::Tokyo) // Using default prefecture
        .with_start_date(Utc::now())
        .with_job_description(&new_job_description)
        .with_work_location(&original_contract.work_location);

    match builder.build() {
        Ok(converted_contract) => {
            println!();
            println!("✅ 転換後の契約:");
            println!("{}", "-".repeat(80));
            println!("契約形態: 無期雇用契約 (期間の定めなし)");
            println!("月給: ¥{}", converted_contract.base_wage_jpy);
            if new_salary > original_contract.base_wage_jpy {
                println!(
                    "  (増額: +¥{})",
                    new_salary - original_contract.base_wage_jpy
                );
            }
            println!("職務内容: {}", converted_contract.job_description);
            println!(
                "労働時間: {}時間/日、週{}日",
                converted_contract.hours_per_day, converted_contract.days_per_week
            );
            println!("勤務地: {}", converted_contract.work_location);
            println!();
            println!("💼 無期雇用契約のメリット:");
            println!("  ✅ 雇用の安定性が向上");
            println!("  ✅ 契約更新の不安がなくなる");
            println!("  ✅ 長期的なキャリア形成が可能");
            println!("  ✅ 住宅ローンなどの審査で有利");
        }
        Err(e) => {
            println!("❌ 転換契約の作成に失敗しました: {}", e);
        }
    }
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

fn get_date() -> NaiveDate {
    loop {
        print!("年 (YYYY): ");
        io::stdout().flush().unwrap();
        let mut year_input = String::new();
        io::stdin().read_line(&mut year_input).unwrap();

        print!("月 (1-12): ");
        io::stdout().flush().unwrap();
        let mut month_input = String::new();
        io::stdin().read_line(&mut month_input).unwrap();

        print!("日 (1-31): ");
        io::stdout().flush().unwrap();
        let mut day_input = String::new();
        io::stdin().read_line(&mut day_input).unwrap();

        if let (Ok(year), Ok(month), Ok(day)) = (
            year_input.trim().parse::<i32>(),
            month_input.trim().parse::<u32>(),
            day_input.trim().parse::<u32>(),
        ) {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                return date;
            }
        }
        println!("❌ 無効な日付です。もう一度入力してください。");
    }
}

fn get_salary() -> u64 {
    loop {
        print!("月給 (円): ¥");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().replace(",", "").parse::<u64>() {
            Ok(salary) if salary > 0 => return salary,
            _ => println!("❌ 無効な金額です。"),
        }
    }
}

fn get_salary_with_validation(current_salary: u64) -> u64 {
    println!("現在の月給: ¥{}", current_salary);
    println!("⚠️ 注意: 無期転換時の給与は現在の給与より低くできません（不利益変更禁止）");

    loop {
        print!("新しい月給 (円): ¥");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().replace(",", "").parse::<u64>() {
            Ok(salary) if salary >= current_salary => return salary,
            Ok(_salary) => println!("❌ 現在の給与(¥{})より低くできません。", current_salary),
            _ => println!("❌ 無効な金額です。"),
        }
    }
}

fn get_working_hours() -> (u32, u32) {
    let hours_per_day = loop {
        print!("1日の労働時間 (時間): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(hours) if hours > 0 && hours <= 24 => break hours,
            _ => println!("❌ 1-24の範囲で入力してください。"),
        }
    };

    let days_per_week = loop {
        print!("週の労働日数 (日): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(days) if days > 0 && days <= 7 => break days,
            _ => println!("❌ 1-7の範囲で入力してください。"),
        }
    };

    (hours_per_day, days_per_week)
}

fn get_prefecture() -> Prefecture {
    println!("勤務地の都道府県を選択してください:");
    println!("  1. 東京都");
    println!("  2. 大阪府");
    println!("  3. 神奈川県");
    println!("  4. その他");

    loop {
        print!("選択 (1-4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return Prefecture::Tokyo,
            "2" => return Prefecture::Osaka,
            "3" => return Prefecture::Kanagawa,
            "4" => return Prefecture::Hokkaido, // Default for "other"
            _ => println!("❌ 1-4で選択してください。"),
        }
    }
}

fn get_renewal_count() -> u32 {
    loop {
        print!("これまでの契約更新回数: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(count) => return count,
            _ => println!("❌ 数値を入力してください。"),
        }
    }
}
