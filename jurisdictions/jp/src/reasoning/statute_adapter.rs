//! Statute adapters for Japanese Labor Law (労働法令アダプター).
//!
//! Converts Japanese labor law provisions into legalis-core Statute abstractions.
//! 日本の労働法規定をlegalis-core Statute抽象化に変換

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

// ============================================================================
// Labor Standards Act (労働基準法)
// ============================================================================

/// Creates a Statute for Article 32 - Statutory Working Hours
/// 第32条 - 法定労働時間
#[must_use]
pub fn lsa_article_32_working_hours() -> Statute {
    Statute::new(
        "LSA_Art32",
        "法定労働時間 / Statutory Working Hours (Art. 32)",
        Effect::new(
            EffectType::Prohibition,
            "1日8時間、週40時間を超えて労働させてはならない / Max 8 hours/day, 40 hours/week",
        )
        .with_parameter("max_hours_per_day", "8")
        .with_parameter("max_hours_per_week", "40"),
    )
    .with_jurisdiction("JP")
}

/// Creates a Statute for Article 34 - Rest Periods
/// 第34条 - 休憩時間
#[must_use]
pub fn lsa_article_34_rest_periods() -> Statute {
    Statute::new(
        "LSA_Art34",
        "休憩時間 / Rest Periods (Art. 34)",
        Effect::new(
            EffectType::Grant,
            "6時間超で45分、8時間超で60分の休憩を与えなければならない / 45min for 6h+, 60min for 8h+",
        )
        .with_parameter("rest_6h_minutes", "45")
        .with_parameter("rest_8h_minutes", "60"),
    )
    .with_jurisdiction("JP")
}

/// Creates a Statute for Article 35 - Weekly Day Off
/// 第35条 - 週休
#[must_use]
pub fn lsa_article_35_weekly_day_off() -> Statute {
    Statute::new(
        "LSA_Art35",
        "週休 / Weekly Day Off (Art. 35)",
        Effect::new(
            EffectType::Grant,
            "毎週少なくとも1日の休日を与えなければならない / At least 1 day off per week",
        )
        .with_parameter("min_days_off_per_week", "1"),
    )
    .with_jurisdiction("JP")
}

/// Creates a Statute for Article 36 - Overtime Agreement (36協定)
/// 第36条 - 時間外・休日労働
#[must_use]
pub fn lsa_article_36_overtime_agreement() -> Statute {
    Statute::new(
        "LSA_Art36",
        "36協定 / Overtime Agreement (Art. 36)",
        Effect::new(
            EffectType::Grant,
            "36協定締結により時間外労働が可能 / Overtime permitted with agreement",
        ),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "has_36_agreement".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("JP")
}

/// Creates a Statute for Article 37 - Overtime Premium Pay
/// 第37条 - 割増賃金
#[must_use]
pub fn lsa_article_37_overtime_premium() -> Statute {
    Statute::new(
        "LSA_Art37",
        "割増賃金 / Overtime Premium Pay (Art. 37)",
        Effect::new(
            EffectType::Grant,
            "時間外25%、深夜25%、休日35%以上の割増賃金 / 25% overtime, 25% late night, 35% holiday premium",
        )
        .with_parameter("overtime_rate", "0.25")
        .with_parameter("late_night_rate", "0.25")
        .with_parameter("holiday_rate", "0.35"),
    )
    .with_jurisdiction("JP")
}

/// Creates a Statute for Article 20 - Advance Notice of Dismissal
/// 第20条 - 解雇予告
#[must_use]
pub fn lsa_article_20_dismissal_notice() -> Statute {
    Statute::new(
        "LSA_Art20",
        "解雇予告 / Advance Notice of Dismissal (Art. 20)",
        Effect::new(
            EffectType::Obligation,
            "30日前の予告または30日分の平均賃金支払が必要 / 30 days notice or payment in lieu",
        )
        .with_parameter("notice_days", "30"),
    )
    .with_jurisdiction("JP")
}

/// Creates a Statute for Article 39 - Annual Paid Leave
/// 第39条 - 年次有給休暇
#[must_use]
pub fn lsa_article_39_annual_leave() -> Statute {
    Statute::new(
        "LSA_Art39",
        "年次有給休暇 / Annual Paid Leave (Art. 39)",
        Effect::new(
            EffectType::Grant,
            "6ヶ月継続勤務で10日、以降増加 / 10 days after 6 months, increasing thereafter",
        )
        .with_parameter("initial_days", "10"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 6,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("JP")
}

// ============================================================================
// Labor Contract Act (労働契約法)
// ============================================================================

/// Creates a Statute for Article 18 - Conversion to Indefinite Term
/// 第18条 - 無期転換ルール
#[must_use]
pub fn lca_article_18_indefinite_conversion() -> Statute {
    Statute::new(
        "LCA_Art18",
        "無期転換ルール / Indefinite Conversion Rule (Art. 18)",
        Effect::new(
            EffectType::Grant,
            "有期契約5年超で無期転換申込権 / Right to convert after 5+ years of fixed-term",
        ),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 60, // 5 years
        unit: DurationUnit::Months,
    })
    .with_precondition(Condition::AttributeEquals {
        key: "employment_type".to_string(),
        value: "fixed_term".to_string(),
    })
    .with_jurisdiction("JP")
}

/// Creates a Statute for Article 16 - Abusive Dismissal
/// 第16条 - 解雇権濫用法理
#[must_use]
pub fn lca_article_16_abusive_dismissal() -> Statute {
    Statute::new(
        "LCA_Art16",
        "解雇権濫用法理 / Abusive Dismissal Doctrine (Art. 16)",
        Effect::new(
            EffectType::Grant,
            "客観的合理的理由と社会通念上の相当性を欠く解雇は無効 / Dismissal void if lacking objective reason",
        ),
    )
    .with_jurisdiction("JP")
}

// ============================================================================
// Minimum Wage Act (最低賃金法)
// ============================================================================

/// Creates a Statute for Minimum Wage
/// 最低賃金
#[must_use]
pub fn minimum_wage_act() -> Statute {
    Statute::new(
        "MWA",
        "最低賃金 / Minimum Wage Act",
        Effect::new(
            EffectType::Grant,
            "地域別または特定産業別の最低賃金 / Regional or industry-specific minimum wage",
        ),
    )
    .with_jurisdiction("JP")
}

// ============================================================================
// Overtime Limits (時間外労働上限規制)
// ============================================================================

/// Creates a Statute for Overtime Limits (2019 Reform)
/// 時間外労働の上限規制（2019年改正）
#[must_use]
pub fn overtime_limit_regulation() -> Statute {
    Statute::new(
        "OT_LIMIT",
        "時間外労働上限規制 / Overtime Limit Regulation",
        Effect::new(
            EffectType::Prohibition,
            "原則月45時間・年360時間、特別条項でも月100時間未満・年720時間以内 / Max 45h/month, 360h/year normally",
        )
        .with_parameter("standard_monthly_limit", "45")
        .with_parameter("standard_yearly_limit", "360")
        .with_parameter("special_monthly_limit", "100")
        .with_parameter("special_yearly_limit", "720"),
    )
    .with_jurisdiction("JP")
}

/// Get all Japanese labor law statutes
/// 全日本労働法令を取得
#[must_use]
pub fn all_labor_statutes() -> Vec<Statute> {
    vec![
        lsa_article_32_working_hours(),
        lsa_article_34_rest_periods(),
        lsa_article_35_weekly_day_off(),
        lsa_article_36_overtime_agreement(),
        lsa_article_37_overtime_premium(),
        lsa_article_20_dismissal_notice(),
        lsa_article_39_annual_leave(),
        lca_article_18_indefinite_conversion(),
        lca_article_16_abusive_dismissal(),
        minimum_wage_act(),
        overtime_limit_regulation(),
    ]
}
