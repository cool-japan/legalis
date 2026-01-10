//! French Constitution of 1958 structure example
//!
//! Demonstrates the complete structure of the Fifth Republic Constitution

use legalis_fr::constitution::*;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("    Constitution de la Cinquième République (1958)");
    println!("    Constitution of the Fifth French Republic");
    println!("═══════════════════════════════════════════════════════\n");

    // Overview
    println!("📜 Overview");
    println!("───────────");
    println!("Adopted: October 4, 1958");
    println!("In force: October 5, 1958");
    println!("System: Semi-presidential republic");
    println!("Titles: 16");
    println!("Articles: 89");
    println!();

    // Display all titles
    println!("📋 Complete Title Structure (Structure complète des titres)");
    println!("════════════════════════════════════════════════════════\n");

    let titles = all_titles();

    for title in &titles {
        println!(
            "┌─ Title {} │ Articles {}-{} ({} articles)",
            title.number,
            title.articles.0,
            title.articles.1,
            title.article_count()
        );
        println!("│");
        println!("│  🇫🇷 {}", title.title_fr);
        println!("│  🇬🇧 {}", title.title_en);
        println!("│");
        println!("│  FR: {}", title.description_fr);
        println!("│  EN: {}", title.description_en);
        println!("└────────────────────────────────────────────────────────");
        println!();
    }

    // Key institutions
    println!("🏛️  Key Institutions (Institutions clés)");
    println!("═══════════════════════════════════════════════\n");

    println!(
        "1. {} / {}",
        Institution::President.french_name(),
        Institution::President.english_name()
    );
    println!("   Articles 5-19 (Title II)");
    println!("   • Elected by direct universal suffrage for 5 years");
    println!("   • Guarantor of national independence and institutions");
    println!("   • Appoints Prime Minister, can dissolve National Assembly");
    println!();

    println!(
        "2. {} / {}",
        Institution::PrimeMinister.french_name(),
        Institution::PrimeMinister.english_name()
    );
    println!("   Articles 20-23 (Title III)");
    println!("   • Determines and conducts policy of the Nation");
    println!("   • Responsible to National Assembly");
    println!();

    println!(
        "3. {} / {}",
        Institution::NationalAssembly.french_name(),
        Institution::NationalAssembly.english_name()
    );
    println!("   Articles 24-33 (Title IV)");
    println!("   • 577 deputies elected for 5 years");
    println!("   • Votes laws, can overthrow Government (motion of censure)");
    println!();

    println!(
        "4. {} / {}",
        Institution::Senate.french_name(),
        Institution::Senate.english_name()
    );
    println!("   Articles 24-33 (Title IV)");
    println!("   • 348 senators elected for 6 years");
    println!("   • Represents territorial collectivities");
    println!();

    println!(
        "5. {} / {}",
        Institution::ConstitutionalCouncil.french_name(),
        Institution::ConstitutionalCouncil.english_name()
    );
    println!("   Articles 56-63 (Title VII)");
    println!("   • 9 members (3 appointed by each: President, Assembly, Senate)");
    println!("   • Reviews constitutionality of laws");
    println!("   • QPC (Priority Question of Constitutionality) since 2008");
    println!();

    // Fundamental rights
    println!("⚖️  Fundamental Rights (Droits fondamentaux)");
    println!("═══════════════════════════════════════════════\n");

    let rights = vec![
        FundamentalRight::Equality,
        FundamentalRight::Liberty,
        FundamentalRight::Fraternity,
        FundamentalRight::Secularism,
        FundamentalRight::Democracy,
        FundamentalRight::SocialRights,
    ];

    for right in rights {
        println!("• {} / {}", right.french_name(), right.english_name());
    }
    println!();
    println!("Note: Rights also protected by:");
    println!("  - Declaration of Rights of Man and Citizen (1789)");
    println!("  - Preamble to Constitution of 1946 (social rights)");
    println!("  - Environmental Charter (2004)");
    println!();

    // Key constitutional principles
    println!("🔑 Key Constitutional Principles");
    println!("═══════════════════════════════════════════════\n");

    println!("Article 1 - Republican Principles:");
    println!("  France is an indivisible, secular, democratic and social Republic.");
    println!("  (La France est une République indivisible, laïque, démocratique et sociale.)");
    println!();

    println!("Article 2 - National Symbols:");
    println!("  Language: French");
    println!("  Motto: Liberté, Égalité, Fraternité (Liberty, Equality, Fraternity)");
    println!("  Flag: Blue, white, red tricolor");
    println!("  Anthem: La Marseillaise");
    println!();

    println!("Article 3 - National Sovereignty:");
    println!("  National sovereignty belongs to the people.");
    println!("  No section of the people nor any individual may arrogate its exercise.");
    println!();

    println!("Article 34 - Domain of Law:");
    println!("  Parliament votes laws on:");
    println!("  • Civil rights and fundamental guarantees");
    println!("  • Nationality, status of persons");
    println!("  • Criminal law and procedure");
    println!("  • Taxation, currency");
    println!("  • Electoral systems");
    println!();

    println!("Article 55 - International Law Primacy:");
    println!("  Treaties have authority superior to laws.");
    println!("  (Primauté du droit international)");
    println!();

    // Famous provisions
    println!("🌟 Famous Constitutional Provisions");
    println!("═══════════════════════════════════════════════\n");

    println!("Article 16 - Emergency Powers (Pouvoirs exceptionnels):");
    println!("  President may take measures required by circumstances");
    println!("  when institutions, independence, territorial integrity threatened.");
    println!("  (Never used since 1961 Algerian crisis)");
    println!();

    println!("Article 49.3 - Confidence Vote (Engagement de responsabilité):");
    println!("  Prime Minister may make adoption of bill a question of confidence.");
    println!("  Bill adopted unless motion of censure passed within 24 hours.");
    println!("  (Controversial but frequently used)");
    println!();

    println!("Article 89 - Constitutional Amendment:");
    println!("  Two methods:");
    println!("  1. Referendum (popular vote)");
    println!("  2. Congress (joint session of Assembly + Senate, 3/5 majority)");
    println!();

    // Comparison with other systems
    println!("🌍 Comparative Constitutional Systems");
    println!("═══════════════════════════════════════════════\n");

    println!("┌─────────────────┬──────────────┬──────────────┬──────────────┐");
    println!("│ Feature         │ 🇫🇷 France   │ 🇯🇵 Japan    │ 🇺🇸 USA      │");
    println!("├─────────────────┼──────────────┼──────────────┼──────────────┤");
    println!("│ System          │ Semi-pres.   │ Parliamentary│ Presidential │");
    println!("│ Head of State   │ President    │ Emperor      │ President    │");
    println!("│ Head of Gov.    │ Prime Min.   │ Prime Min.   │ President    │");
    println!("│ Legislature     │ Bicameral    │ Bicameral    │ Bicameral    │");
    println!("│ Lower House     │ 577 (5y)     │ 465 (4y)     │ 435 (2y)     │");
    println!("│ Upper House     │ 348 (6y)     │ 248 (6y)     │ 100 (6y)     │");
    println!("│ Judicial Review │ Const. Counc.│ Sup. Court   │ Sup. Court   │");
    println!("│ Amendment       │ 3/5 Congress │ 2/3 + ref.   │ 2/3 + states │");
    println!("└─────────────────┴──────────────┴──────────────┴──────────────┘");
    println!();

    // Historical context
    println!("📖 Historical Context (Contexte historique)");
    println!("═══════════════════════════════════════════════\n");

    println!("Why the Fifth Republic? (Pourquoi la Cinquième République?)");
    println!();
    println!("The Fourth Republic (1946-1958) was marked by:");
    println!("  • Government instability (24 governments in 12 years)");
    println!("  • Weak executive power");
    println!("  • Algerian War crisis");
    println!();
    println!("De Gaulle's 1958 Constitution created:");
    println!("  • Strong presidential power (\"republican monarch\")");
    println!("  • Stable government (confidence vote mechanism)");
    println!("  • Rationalized parliamentarism");
    println!();
    println!("Major reforms:");
    println!("  • 1962: Direct election of President (previously electoral college)");
    println!("  • 2000: Quinquennat (5-year term, down from 7 years)");
    println!("  • 2008: QPC (Priority Question of Constitutionality)");
    println!();

    // Summary statistics
    println!("📊 Constitutional Statistics");
    println!("═══════════════════════════════════════════════\n");

    println!("Titles: {}", titles.len());
    println!("Articles: 89 (plus numerous sub-articles)");
    println!();

    println!("Title breakdown:");
    for title in &titles {
        println!(
            "  Title {:2}: {:2} articles - {}",
            title.number,
            title.article_count(),
            title.title_en
        );
    }
    println!();

    println!("Amendments: 24+ constitutional revisions since 1958");
    println!("Most recent: 2008 (major reform - QPC, new rights)");
    println!();

    // Comparison with Japanese Constitution
    println!("🇫🇷🇯🇵 France-Japan Constitutional Comparison");
    println!("═══════════════════════════════════════════════\n");

    println!("Similarities:");
    println!("  • Both have bicameral parliaments");
    println!("  • Both have judicial review (though different mechanisms)");
    println!("  • Both protect fundamental rights");
    println!();

    println!("Differences:");
    println!("  • France: Semi-presidential (President + PM)");
    println!("    Japan: Parliamentary (Emperor symbolic, PM governs)");
    println!();
    println!("  • France: President elected, can dissolve Assembly");
    println!("    Japan: PM chosen by Diet, cannot dissolve upper house");
    println!();
    println!("  • France: Constitutional Council (9 members, political appointment)");
    println!("    Japan: Supreme Court (15 justices, judicial career)");
    println!();
    println!("  • France: Easily amended (24+ times since 1958)");
    println!("    Japan: Never amended since 1947 (too difficult - 2/3 + referendum)");
    println!();

    println!("═══════════════════════════════════════════════════════");
    println!("End of Constitution structure overview");
    println!("═══════════════════════════════════════════════════════");
}
