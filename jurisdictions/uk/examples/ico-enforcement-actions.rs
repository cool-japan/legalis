//! ICO Enforcement Actions Examples
//!
//! Demonstrates Information Commissioner's Office enforcement powers
//! under DPA 2018 Part 6 and UK GDPR Article 83.
//!
//! The ICO has various enforcement tools:
//! - Information Notices (s.142)
//! - Assessment Notices (s.146)
//! - Enforcement Notices (s.149)
//! - Monetary Penalties (s.155, Article 83)
//! - Prosecution (s.196-197)

use chrono::{Duration, Utc};
use legalis_uk::data_protection::enforcement::*;

fn main() {
    println!("=== ICO Enforcement Actions ===\n");
    println!("Information Commissioner's Office Powers under DPA 2018\n");
    println!("================================================\n");

    // Example 1: Information Notice
    example_1_information_notice();

    // Example 2: Enforcement Notice
    example_2_enforcement_notice();

    // Example 3: Assessment Notice
    example_3_assessment_notice();

    // Example 4: Monetary Penalty (Tier 4)
    example_4_monetary_penalty_tier4();

    // Example 5: Monetary Penalty (Tier 5)
    example_5_monetary_penalty_tier5();

    // Example 6: Fine calculation with factors
    example_6_fine_calculation();

    // Example 7: Criminal Prosecution
    example_7_prosecution();
}

fn example_1_information_notice() {
    println!("Example 1: Information Notice (DPA 2018 s.142)");
    println!("===============================================\n");

    println!("Scenario: ICO investigating complaint about data processing");
    println!();

    let notice = IcoEnforcement::information_notice(
        30,
        vec![
            "Provide Records of Processing Activities (ROPA) under Article 30".to_string(),
            "List all third-party processors and sub-processors".to_string(),
            "Provide copies of all processor contracts (Article 28)".to_string(),
            "Describe security measures under Article 32".to_string(),
            "Provide evidence of DPIA for high-risk processing (Article 35)".to_string(),
        ],
    );

    if let IcoEnforcement::InformationNotice {
        deadline_days,
        information_required,
        issued_date,
    } = notice
    {
        println!("📋 ICO Information Notice Issued");
        println!();
        println!("Statutory basis: DPA 2018 s.142");
        println!("Purpose: Require controller/processor to provide information");
        println!();
        println!("Notice Details:");
        println!("  Issued date: {}", issued_date.format("%Y-%m-%d"));
        println!("  Deadline: {} days from issue", deadline_days);
        println!(
            "  Response due: {}",
            (issued_date + Duration::days(deadline_days as i64)).format("%Y-%m-%d")
        );
        println!();
        println!("Information Required:");
        for (i, item) in information_required.iter().enumerate() {
            println!("  {}. {}", i + 1, item);
        }
        println!();
        println!("⚠️  Failure to comply:");
        println!("  • Criminal offence under DPA 2018 s.196 (obstruction of ICO)");
        println!("  • Prosecution may follow");
        println!();
    }
}

fn example_2_enforcement_notice() {
    println!("Example 2: Enforcement Notice (DPA 2018 s.149)");
    println!("===============================================\n");

    println!("Scenario: ICO found UK GDPR violations, requires corrective action");
    println!();

    let deadline = Utc::now() + Duration::days(90);
    let notice = IcoEnforcement::enforcement_notice(
        vec![
            "Implement pseudonymization for customer personal data (Article 32)".to_string(),
            "Conduct DPIA for automated decision-making system (Article 35)".to_string(),
            "Update privacy notices to comply with Article 13 transparency requirements"
                .to_string(),
            "Implement data retention policy and delete data beyond retention period".to_string(),
            "Appoint Data Protection Officer and register with ICO (Article 37)".to_string(),
        ],
        deadline,
    );

    if let IcoEnforcement::EnforcementNotice {
        required_actions,
        deadline,
        issued_date,
    } = notice
    {
        println!("🛑 ICO Enforcement Notice Issued");
        println!();
        println!("Statutory basis: DPA 2018 s.149");
        println!("Purpose: Require controller/processor to take steps to comply with UK GDPR");
        println!();
        println!("Notice Details:");
        println!("  Issued date: {}", issued_date.format("%Y-%m-%d"));
        println!("  Compliance deadline: {}", deadline.format("%Y-%m-%d"));
        println!("  Days to comply: {}", (deadline - issued_date).num_days());
        println!();
        println!("Required Actions:");
        for (i, action) in required_actions.iter().enumerate() {
            println!("  {}. {}", i + 1, action);
        }
        println!();
        println!("⚠️  Failure to comply:");
        println!("  • Monetary penalty may be imposed (Article 83)");
        println!("  • Further enforcement action");
        println!("  • Criminal prosecution (DPA 2018 s.196)");
        println!();
    }
}

fn example_3_assessment_notice() {
    println!("Example 3: Assessment Notice (DPA 2018 s.146)");
    println!("==============================================\n");

    println!("Scenario: ICO requires on-site inspection of processing operations");
    println!();

    let inspection_date = Utc::now() + Duration::days(7);
    let notice = IcoEnforcement::AssessmentNotice {
        scope: "Inspection of automated decision-making systems for credit scoring, \
                including algorithms, training data, and compliance with Article 22 \
                (right not to be subject to automated decision-making)."
            .to_string(),
        inspection_date,
    };

    if let IcoEnforcement::AssessmentNotice {
        scope,
        inspection_date,
    } = notice
    {
        println!("🔍 ICO Assessment Notice Issued");
        println!();
        println!("Statutory basis: DPA 2018 s.146");
        println!("Purpose: ICO inspection of processing operations");
        println!();
        println!("Notice Details:");
        println!(
            "  Inspection date: {}",
            inspection_date.format("%Y-%m-%d %H:%M")
        );
        println!();
        println!("Scope of Assessment:");
        println!("  {}", scope);
        println!();
        println!("ICO Powers During Inspection:");
        println!("  • Enter premises");
        println!("  • Inspect equipment and documents");
        println!("  • Observe processing operations");
        println!("  • Interview staff");
        println!();
        println!("⚠️  Obstruction of ICO inspection:");
        println!("  • Criminal offence under DPA 2018 s.196");
        println!();
    }
}

fn example_4_monetary_penalty_tier4() {
    println!("Example 4: Monetary Penalty - Tier 4 (Article 83(4))");
    println!("=====================================================\n");

    println!("Scenario: Processor failed to maintain Article 30 Records (ROPA)");
    println!();

    let penalty = IcoEnforcement::monetary_penalty(
        250_000.0,
        "Controller failed to maintain Records of Processing Activities (ROPA) \
         as required by UK GDPR Article 30. This is an infringement of Article 30, \
         which falls under Article 83(4) tier."
            .to_string(),
        4,
    );

    if let IcoEnforcement::MonetaryPenalty {
        amount_gbp,
        reason,
        article_83_tier,
        issued_date,
    } = penalty
    {
        println!("💰 ICO Monetary Penalty Imposed");
        println!();
        println!("Statutory basis: DPA 2018 s.155, UK GDPR Article 83(4)");
        println!();
        println!("Penalty Details:");
        println!("  Amount: £{:.2}", amount_gbp);
        println!("  Tier: {} (Article 83(4) - lower tier)", article_83_tier);
        println!("  Issued date: {}", issued_date.format("%Y-%m-%d"));
        println!();
        println!("Article 83(4) Maximum:");
        println!("  Up to £8,700,000 OR 2% of total annual worldwide turnover");
        println!("  (whichever is higher)");
        println!();
        println!("Reason for Penalty:");
        println!("  {}", reason);
        println!();
        println!("Article 83(4) applies to infringements of:");
        println!("  • Article 8 (child's consent)");
        println!("  • Article 11 (no identification required)");
        println!("  • Articles 25-39 (controller/processor obligations)");
        println!("  • Article 42-43 (certification)");
        println!("  • Article 58 (supervisory authority powers)");
        println!();
    }
}

fn example_5_monetary_penalty_tier5() {
    println!("Example 5: Monetary Penalty - Tier 5 (Article 83(5))");
    println!("=====================================================\n");

    println!("Scenario: Major data breach affecting 500,000 customers");
    println!("         No lawful basis for processing, inadequate security");
    println!();

    let penalty = IcoEnforcement::monetary_penalty(
        4_500_000.0,
        "Data breach affecting 500,000 customers. Investigation found:\n\
         • No lawful basis under Article 6 for processing personal data\n\
         • Inadequate security measures (Article 32 violation)\n\
         • Failure to notify ICO of breach within 72 hours (Article 33)\n\
         • Failure to notify data subjects (Article 34)\n\
         This is an infringement of the basic principles of UK GDPR (Articles 5-6) \
         and falls under Article 83(5) higher tier."
            .to_string(),
        5,
    );

    if let IcoEnforcement::MonetaryPenalty {
        amount_gbp,
        reason,
        article_83_tier,
        issued_date,
    } = penalty
    {
        println!("💰 ICO Monetary Penalty Imposed");
        println!();
        println!("Statutory basis: DPA 2018 s.155, UK GDPR Article 83(5)");
        println!();
        println!("Penalty Details:");
        println!("  Amount: £{:.2}", amount_gbp);
        println!("  Tier: {} (Article 83(5) - HIGHER tier)", article_83_tier);
        println!("  Issued date: {}", issued_date.format("%Y-%m-%d"));
        println!();
        println!("Article 83(5) Maximum:");
        println!("  Up to £17,500,000 OR 4% of total annual worldwide turnover");
        println!("  (whichever is higher)");
        println!();
        println!("Reason for Penalty:");
        for line in reason.lines() {
            println!("  {}", line);
        }
        println!();
        println!("Article 83(5) applies to infringements of:");
        println!("  • Article 5 (basic principles)");
        println!("  • Article 6 (lawfulness of processing)");
        println!("  • Article 7 (conditions for consent)");
        println!("  • Article 9 (special categories)");
        println!("  • Articles 12-22 (data subject rights)");
        println!("  • Articles 44-49 (international transfers)");
        println!();
    }
}

fn example_6_fine_calculation() {
    println!("Example 6: Fine Calculation with Aggravating/Mitigating Factors");
    println!("================================================================\n");

    println!("Scenario: Large retailer, £1bn turnover, data breach");
    println!();

    let turnover = 1_000_000_000.0; // £1 billion

    // Case A: Aggravating factors
    println!("Case A: Multiple Aggravating Factors");
    println!("-------------------------------------");
    let aggravating = vec![
        "Intentional infringement".to_string(),
        "Negligent conduct".to_string(),
        "Previous violations".to_string(),
        "Failure to cooperate with ICO".to_string(),
    ];
    let mitigating_a: Vec<String> = vec![];

    let fine_a = calculate_ico_fine(Article83Tier::Tier5, turnover, &aggravating, &mitigating_a);

    println!("  Global turnover: £{:.0}", turnover);
    println!("  Article 83 tier: Tier 5 (higher)");
    println!(
        "  Statutory maximum: £17,500,000 (or 4% = £{:.0})",
        turnover * 0.04
    );
    println!();
    println!("  Aggravating factors ({}):", aggravating.len());
    for factor in &aggravating {
        println!("    - {}", factor);
    }
    println!("  Mitigating factors: None");
    println!();
    println!("  💰 Calculated fine: £{:.2}", fine_a);
    println!();

    // Case B: Mitigating factors
    println!("Case B: Multiple Mitigating Factors");
    println!("------------------------------------");
    let aggravating_b: Vec<String> = vec![];
    let mitigating = vec![
        "Full cooperation with ICO".to_string(),
        "Immediate remedial action".to_string(),
        "First offence".to_string(),
        "Self-reported breach".to_string(),
        "No financial gain from violation".to_string(),
    ];

    let fine_b = calculate_ico_fine(Article83Tier::Tier5, turnover, &aggravating_b, &mitigating);

    println!("  Global turnover: £{:.0}", turnover);
    println!("  Article 83 tier: Tier 5 (higher)");
    println!(
        "  Statutory maximum: £17,500,000 (or 4% = £{:.0})",
        turnover * 0.04
    );
    println!();
    println!("  Aggravating factors: None");
    println!("  Mitigating factors ({}):", mitigating.len());
    for factor in &mitigating {
        println!("    - {}", factor);
    }
    println!();
    println!("  💰 Calculated fine: £{:.2}", fine_b);
    println!();

    println!("Article 83(2) Factors ICO Considers:");
    println!("  (a) Nature, gravity, duration of infringement");
    println!("  (b) Intentional or negligent character");
    println!("  (c) Actions to mitigate damage");
    println!("  (d) Degree of responsibility (technical/organizational measures)");
    println!("  (e) Previous relevant infringements");
    println!("  (f) Cooperation with ICO");
    println!("  (g) Categories of personal data affected");
    println!("  (h) Notification of breach");
    println!("  (i) Adherence to approved codes/certifications");
    println!("  (j) Other aggravating/mitigating factors");
    println!();
}

fn example_7_prosecution() {
    println!("Example 7: Criminal Prosecution (DPA 2018 Part 6)");
    println!("==================================================\n");

    // Unlawful obtaining of personal data
    let offence1 = IcoEnforcement::Prosecution {
        offence: Dpa2018Offence::UnlawfulObtaining,
        description: "Company director unlawfully obtained customer personal data from \
                      competitor's database without lawful authority. DPA 2018 s.170."
            .to_string(),
    };

    println!("Case 1: Unlawful Obtaining of Personal Data");
    println!("--------------------------------------------");
    if let IcoEnforcement::Prosecution {
        offence,
        description,
    } = offence1
    {
        println!("Criminal Offence: {:?}", offence);
        println!("Statutory basis: DPA 2018 s.170");
        println!();
        println!("Description:");
        println!("  {}", description);
        println!();
        println!("Penalty:");
        println!("  • Summary conviction: Fine");
        println!("  • Indictment: Imprisonment up to 6 months or fine (or both)");
        println!();
    }

    // Re-identification
    let offence2 = IcoEnforcement::Prosecution {
        offence: Dpa2018Offence::ReIdentification,
        description: "Data scientist re-identified de-identified personal data in breach \
                      of prohibition. DPA 2018 s.171."
            .to_string(),
    };

    println!("Case 2: Re-identification of De-identified Data");
    println!("------------------------------------------------");
    if let IcoEnforcement::Prosecution {
        offence,
        description,
    } = offence2
    {
        println!("Criminal Offence: {:?}", offence);
        println!("Statutory basis: DPA 2018 s.171");
        println!();
        println!("Description:");
        println!("  {}", description);
        println!();
        println!("Penalty:");
        println!("  • Summary conviction: Fine");
        println!("  • Indictment: Unlimited fine");
        println!();
    }

    // Obstruction of ICO
    let offence3 = IcoEnforcement::Prosecution {
        offence: Dpa2018Offence::ObstructionOfIco,
        description: "Controller refused ICO inspector access to premises during \
                      assessment notice inspection. DPA 2018 s.196."
            .to_string(),
    };

    println!("Case 3: Obstruction of ICO");
    println!("--------------------------");
    if let IcoEnforcement::Prosecution {
        offence,
        description,
    } = offence3
    {
        println!("Criminal Offence: {:?}", offence);
        println!("Statutory basis: DPA 2018 s.196");
        println!();
        println!("Description:");
        println!("  {}", description);
        println!();
        println!("Penalty:");
        println!("  • Summary conviction: Fine");
        println!();
    }

    println!("Note: Criminal prosecution is SEPARATE from monetary penalties.");
    println!("      ICO may impose both fine (Article 83) AND prosecute (DPA 2018).");
    println!();
}
