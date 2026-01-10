//! EU Copyright Example - InfoSoc Directive 2001/29/EC & DSM Directive 2019/790
//!
//! Demonstrates copyright protection validation under EU law.

use legalis_eu::intellectual_property::{CopyrightException, CopyrightWork, WorkType};

fn main() {
    println!("=== EU Copyright Validation Examples ===\n");

    // Scenario 1: Valid literary work with originality
    println!("Scenario 1: Original Literary Work");
    println!("-----------------------------------");
    let novel = CopyrightWork::new()
        .with_title("The Digital Frontier")
        .with_author("Alice Writer")
        .with_work_type(WorkType::Literary)
        .with_creation_date(chrono::Utc::now() - chrono::Duration::days(180))
        .with_originality(true)
        .with_fixation(true);

    match novel.validate() {
        Ok(validation) => {
            println!("✅ Work is protectable by copyright");
            println!(
                "   Originality established: {}",
                validation.originality_established
            );
            println!(
                "   Fixation requirement met: {}",
                validation.fixation_requirement_met
            );
            println!("   Currently protected: {}", validation.is_protected);
            println!("   Applicable exceptions:");
            for exception in &validation.applicable_exceptions {
                println!("     - {:?}", exception);
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Scenario 2: Database lacking originality
    println!("\n\nScenario 2: Database Lacking Originality");
    println!("------------------------------------------");
    let phone_directory = CopyrightWork::new()
        .with_title("City Phone Directory")
        .with_author("Directory Co")
        .with_work_type(WorkType::Database)
        .with_originality(false) // Mere compilation, not author's intellectual creation
        .with_fixation(true);

    match phone_directory.validate() {
        Ok(_) => println!("✅ Work is protectable"),
        Err(e) => {
            println!("❌ Not protectable: {}", e);
            println!("   InfoSoc Directive requires 'author's own intellectual creation'");
            println!("   Mere alphabetical compilation lacks originality");
        }
    }

    // Scenario 3: Software with fixation requirement
    println!("\n\nScenario 3: Software Copyright (Software Directive 2009/24/EC)");
    println!("---------------------------------------------------------------");
    let software = CopyrightWork::new()
        .with_title("DataViz Pro")
        .with_author("Tech Developers Ltd")
        .with_work_type(WorkType::Software)
        .with_originality(true)
        .with_fixation(true)
        .with_country_of_origin("Germany");

    match software.validate() {
        Ok(validation) => {
            println!("✅ Software is protectable as literary work");
            println!("   Software Directive: Protected as literary works");
            println!(
                "   Fixation requirement: {} (mandatory for software)",
                validation.fixation_requirement_met
            );
            println!("\n   Applicable exceptions:");
            for exception in &validation.applicable_exceptions {
                if matches!(exception, CopyrightException::TextDataMining) {
                    println!("     - Text/Data Mining (DSM Directive Art. 3-4)");
                    println!("       Allows analysis for research purposes");
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Scenario 4: Protection duration calculation (life + 70 years)
    println!("\n\nScenario 4: Protection Duration (Term Directive 2006/116/EC)");
    println!("-------------------------------------------------------------");
    let death_date = chrono::Utc::now() - chrono::Duration::days(60 * 365); // Author died 60 years ago

    let classic_work = CopyrightWork::new()
        .with_title("Historic Masterpiece")
        .with_author("Classic Author")
        .with_work_type(WorkType::Artistic)
        .with_creation_date(chrono::Utc::now() - chrono::Duration::days(80 * 365))
        .with_death_date_of_author(death_date)
        .with_originality(true);

    match classic_work.validate() {
        Ok(validation) => {
            println!("✅ Work protection status:");
            println!("   Still protected: {}", validation.is_protected);
            if let Some(expiry) = validation.protection_expires {
                let years_remaining = (expiry - chrono::Utc::now()).num_days() / 365;
                println!("   Years remaining: ~{}", years_remaining);
                println!("   Protection rule: Life + 70 years (Term Directive)");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Scenario 5: Public domain work
    println!("\n\nScenario 5: Work in Public Domain");
    println!("----------------------------------");
    let ancient_death = chrono::Utc::now() - chrono::Duration::days(100 * 365); // Author died 100 years ago

    let ancient_work = CopyrightWork::new()
        .with_title("Ancient Text")
        .with_author("Ancient Author")
        .with_work_type(WorkType::Literary)
        .with_death_date_of_author(ancient_death)
        .with_originality(true);

    match ancient_work.validate() {
        Ok(validation) => {
            if validation.is_protected {
                println!("✅ Work is still protected");
            } else {
                println!("📖 Work is in PUBLIC DOMAIN");
                println!(
                    "   Protection expired (life + 70 years = {} years ago)",
                    100 - 70
                );
                println!("   Can be freely used without permission");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Scenario 6: Audiovisual work with parody exception
    println!("\n\nScenario 6: Audiovisual Work with Exceptions");
    println!("---------------------------------------------");
    let film = CopyrightWork::new()
        .with_title("Digital Dreams")
        .with_author("Film Director")
        .with_work_type(WorkType::Audiovisual)
        .with_originality(true)
        .with_fixation(true);

    match film.validate() {
        Ok(validation) => {
            println!("✅ Film is protectable");
            println!("   Notable exceptions applicable:");
            for exception in &validation.applicable_exceptions {
                match exception {
                    CopyrightException::Parody => {
                        println!("     - Parody (InfoSoc Directive Art. 5(3)(k))");
                        println!("       Allows parody, caricature, pastiche without permission");
                    }
                    CopyrightException::Quotation => {
                        println!("     - Quotation (InfoSoc Directive Art. 5(3)(d))");
                        println!("       Allows quotation for criticism or review");
                    }
                    CopyrightException::NewsReporting => {
                        println!("     - News Reporting (InfoSoc Directive Art. 5(3)(c))");
                        println!("       Allows use for current events reporting");
                    }
                    _ => {}
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Scenario 7: Educational use exception
    println!("\n\nScenario 7: Educational Use Exception (DSM Directive)");
    println!("------------------------------------------------------");
    let textbook = CopyrightWork::new()
        .with_title("Introduction to Algorithms")
        .with_author("Professor Smith")
        .with_work_type(WorkType::Literary)
        .with_originality(true);

    match textbook.validate() {
        Ok(validation) => {
            println!("✅ Textbook is protectable");
            if validation
                .applicable_exceptions
                .contains(&CopyrightException::EducationalUse)
            {
                println!("   Educational use exception applies:");
                println!("   - DSM Directive allows limited use for illustration in teaching");
                println!("   - Use must be for non-commercial educational purposes");
                println!("   - Source must be indicated");
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n=== Summary ===");
    println!("EU copyright law implements:");
    println!("  • InfoSoc Directive 2001/29/EC: Harmonized copyright rules");
    println!("  • DSM Directive (EU) 2019/790: Digital Single Market provisions");
    println!("  • Software Directive 2009/24/EC: Software as literary works");
    println!("  • Database Directive 96/9/EC: Sui generis database rights");
    println!("  • Term Directive 2006/116/EC: Life + 70 years protection");
    println!("\nKey principles:");
    println!("  1. Originality: Must be author's own intellectual creation");
    println!("  2. No formalities: Protection automatic upon creation");
    println!("  3. Harmonized exceptions: Quotation, parody, education, etc.");
    println!("  4. Member state implementation: Some flexibility in transposition");
}
