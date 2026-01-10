//! SA (Société Anonyme) formation example
//!
//! Demonstrates French company law for SA formation and governance

use chrono::Utc;
use legalis_fr::company::{
    ArticlesOfIncorporation, BoardOfDirectors, Capital, CompanyType, Director, MeetingType,
    ResolutionType, Shareholder, ShareholdersMeeting, validate_articles_of_incorporation,
    validate_sa_board, validate_shareholders_meeting,
};

fn main() {
    println!("=== French Company Law - SA Formation Example ===\n");
    println!("Code de commerce - Société Anonyme (Stock Company)\n");

    // Example 1: Valid SA formation
    println!("📋 Example 1: Valid SA Formation (Article L225-1)");
    println!("────────────────────────────────────────────────");

    let articles = ArticlesOfIncorporation::new(
        "TechInnovation SA".to_string(),
        CompanyType::SA,
        Capital::new(100_000), // €100,000 (> €37,000 minimum)
    )
    .with_business_purpose("Development and commercialization of software solutions".to_string())
    .with_business_purpose("IT consulting services".to_string())
    .with_head_office("15 Avenue des Champs-Élysées, 75008 Paris, France".to_string())
    .with_shareholder(Shareholder::new(
        "Fondateur 1 (Marie Dupont)".to_string(),
        5_000,
        50_000,
    ))
    .with_shareholder(Shareholder::new(
        "Fondateur 2 (Pierre Martin)".to_string(),
        5_000,
        50_000,
    ))
    .with_fiscal_year_end(12) // December
    .with_incorporation_date(Utc::now().naive_utc().date());

    match validate_articles_of_incorporation(&articles) {
        Ok(_) => {
            println!("✅ Articles of incorporation are valid!");
            println!("   Company: {}", articles.company_name);
            println!(
                "   Type: {} ({})",
                articles.company_type.french_name(),
                articles.company_type.abbreviation()
            );
            println!("   Capital: €{}", articles.capital.amount_eur);
            println!("   Shareholders: {}", articles.shareholders.len());
            println!("   Total shares: {}", articles.total_shares());
            println!("   Head office: {}", articles.head_office);
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!();

    // Example 2: Invalid SA - insufficient capital
    println!("❌ Example 2: Invalid SA - Insufficient Capital");
    println!("─────────────────────────────────────────────");

    let invalid_sa = ArticlesOfIncorporation::new(
        "SmallCorp SA".to_string(),
        CompanyType::SA,
        Capital::new(30_000), // Below €37,000 minimum!
    )
    .with_business_purpose("Business".to_string())
    .with_head_office("Paris".to_string())
    .with_shareholder(Shareholder::new("Owner".to_string(), 100, 30_000));

    match validate_articles_of_incorporation(&invalid_sa) {
        Ok(_) => println!("✅ Valid"),
        Err(e) => {
            println!("❌ Validation failed: {}", e);
            println!("\nExplanation: SA requires minimum €37,000 capital (Article L225-1)");
            println!("Suggestion: Increase capital or use SARL/SAS (minimum €1)");
        }
    }

    println!();

    // Example 3: Board of Directors validation
    println!("🏛️  Example 3: Board of Directors (Article L225-17)");
    println!("────────────────────────────────────────────────");

    let board = BoardOfDirectors::new()
        .with_director(Director::new(
            "Marie Dupont (Présidente)".to_string(),
            Utc::now().naive_utc().date(),
            6, // 6-year term
        ))
        .with_director(Director::new(
            "Pierre Martin".to_string(),
            Utc::now().naive_utc().date(),
            6,
        ))
        .with_director(Director::new(
            "Jean Lefebvre (Administrateur indépendant)".to_string(),
            Utc::now().naive_utc().date(),
            6,
        ))
        .with_chairman("Marie Dupont (Présidente)".to_string());

    match validate_sa_board(&board) {
        Ok(_) => {
            println!("✅ Board composition is valid!");
            println!("   Directors: {} (required: 3-18)", board.size());
            println!("   Chairman: {}", board.chairman.as_ref().unwrap());
            println!("\n   Board members:");
            for (i, director) in board.members.iter().enumerate() {
                println!(
                    "     {}. {} (term: {} years)",
                    i + 1,
                    director.name,
                    director.term_years
                );
            }
        }
        Err(e) => println!("❌ Invalid board: {}", e),
    }

    println!();

    // Example 4: Invalid board - only 2 directors
    println!("❌ Example 4: Invalid Board - Too Few Directors");
    println!("─────────────────────────────────────────────");

    let invalid_board = BoardOfDirectors::new()
        .with_director(Director::new(
            "Director 1".to_string(),
            Utc::now().naive_utc().date(),
            6,
        ))
        .with_director(Director::new(
            "Director 2".to_string(),
            Utc::now().naive_utc().date(),
            6,
        ));
    // Only 2 directors - need at least 3!

    match validate_sa_board(&invalid_board) {
        Ok(_) => println!("✅ Valid"),
        Err(e) => {
            println!("❌ Validation failed: {}", e);
            println!("\nExplanation: SA board requires 3-18 directors (Article L225-17)");
            println!("Current: {} directors", invalid_board.size());
        }
    }

    println!();

    // Example 5: Shareholders meeting - ordinary resolution
    println!("🗳️  Example 5: Shareholders Meeting - Ordinary Resolution");
    println!("────────────────────────────────────────────────────────");

    let meeting = ShareholdersMeeting::new(
        MeetingType::OrdinaryGeneralMeeting,
        Utc::now().naive_utc().date(),
        10_000, // Total shares
    )
    .with_votes(
        3_500, // Shares represented (35% quorum)
        2_800, // Votes for
        500,   // Votes against
        200,   // Abstentions
    );

    println!("Meeting type: Ordinary General Meeting (AGO)");
    println!("Total shares: {}", meeting.total_shares);
    println!(
        "Shares represented: {} ({:.1}% quorum)",
        meeting.shares_represented,
        meeting.quorum_percentage()
    );
    println!("Votes for: {}", meeting.votes_for);
    println!("Votes against: {}", meeting.votes_against);
    println!("Abstentions: {}", meeting.abstentions);
    println!("Approval: {:.1}%", meeting.approval_percentage());

    match validate_shareholders_meeting(&meeting, ResolutionType::Ordinary, false) {
        Ok(_) => {
            println!("\n✅ Resolution APPROVED!");
            println!(
                "   Quorum met: {:.1}% (required: 20%)",
                meeting.quorum_percentage()
            );
            println!(
                "   Approval: {:.1}% (required: > 50%)",
                meeting.approval_percentage()
            );
        }
        Err(e) => println!("\n❌ Resolution failed: {}", e),
    }

    println!();

    // Example 6: Special resolution (2/3 majority required)
    println!("⚖️  Example 6: Extraordinary Meeting - Special Resolution (2/3 majority)");
    println!("─────────────────────────────────────────────────────────────────────");

    let special_meeting = ShareholdersMeeting::new(
        MeetingType::ExtraordinaryGeneralMeeting,
        Utc::now().naive_utc().date(),
        10_000,
    )
    .with_votes(
        4_000, // 40% quorum
        2_700, // 2,700 for
        1_000, // 1,000 against
        300,   // 300 abstentions
    );

    println!("Scenario: Amending articles of incorporation (AGE)");
    println!(
        "Approval: {:.1}% (votes for / votes cast)",
        special_meeting.approval_percentage()
    );

    match validate_shareholders_meeting(&special_meeting, ResolutionType::Special, false) {
        Ok(_) => println!("\n✅ Special resolution APPROVED!"),
        Err(e) => {
            println!("\n❌ Resolution REJECTED: {}", e);
            println!("\nExplanation: Special resolutions require > 66.67% approval");
            println!(
                "Current approval: {:.1}%",
                special_meeting.approval_percentage()
            );
            println!("Required: > 66.67%");
        }
    }

    println!();

    // Summary
    println!("════════════════════════════════════════════════════════");
    println!("📚 Summary - French SA (Société Anonyme) Key Rules");
    println!("════════════════════════════════════════════════════════");
    println!();
    println!("🔹 Article L225-1: Minimum capital €37,000");
    println!("🔹 Article L225-17: Board of 3-18 directors");
    println!("🔹 Article L225-18: Director terms max 6 years");
    println!("🔹 Shareholders meetings:");
    println!("   • Ordinary (AGO): 20% quorum, > 50% approval");
    println!("   • Extraordinary (AGE): 25% quorum, > 66.67% approval");
    println!();
    println!("Comparison with other company types:");
    println!("  ┌────────┬─────────────┬────────────┬──────────────┐");
    println!("  │ Type   │ Min Capital │ Max Partners│ Governance  │");
    println!("  ├────────┼─────────────┼────────────┼──────────────┤");
    println!("  │ SA     │ €37,000     │ Unlimited  │ Board (3-18) │");
    println!("  │ SARL   │ €1          │ 100        │ Manager      │");
    println!("  │ SAS    │ €1          │ Unlimited  │ Flexible     │");
    println!("  └────────┴─────────────┴────────────┴──────────────┘");
    println!();
    println!("🇯🇵 Comparison with Japanese 株式会社 (KK):");
    println!("  - Similar: Limited liability, share structure");
    println!("  - France: €37,000 minimum (vs. Japan: ¥1 since 2006)");
    println!("  - France: 3-18 board (vs. Japan: 3+ board, no max)");
    println!();
}
