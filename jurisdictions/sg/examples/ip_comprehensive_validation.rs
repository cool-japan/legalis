//! Comprehensive IP Validation Example
//!
//! This example demonstrates validation across all four IP types in Singapore:
//! 1. Patents (Patents Act Cap. 221)
//! 2. Trademarks (Trade Marks Act Cap. 332)
//! 3. Copyright (Copyright Act 2021)
//! 4. Registered Designs (Registered Designs Act Cap. 266)

use chrono::NaiveDate;
use legalis_sg::ip::*;

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║   SINGAPORE INTELLECTUAL PROPERTY - COMPREHENSIVE VALIDATION ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Patent Validation
    println!("━━━ 1. PATENT VALIDATION (Patents Act Cap. 221) ━━━\n");
    validate_patent_example();

    println!("\n{}\n", "═".repeat(66));

    // Trademark Validation
    println!("━━━ 2. TRADEMARK VALIDATION (Trade Marks Act Cap. 332) ━━━\n");
    validate_trademark_example();

    println!("\n{}\n", "═".repeat(66));

    // Copyright Validation
    println!("━━━ 3. COPYRIGHT VALIDATION (Copyright Act 2021) ━━━\n");
    validate_copyright_example();

    println!("\n{}\n", "═".repeat(66));

    // Registered Design Validation
    println!("━━━ 4. REGISTERED DESIGN VALIDATION (Cap. 266) ━━━\n");
    validate_design_example();

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("✅ All IP validations completed");
    println!("═══════════════════════════════════════════════════════════════\n");
}

fn validate_patent_example() {
    println!("📋 Patent: Novel Battery Technology");
    println!("{}\n", "─".repeat(66));

    let mut patent = Patent::new(
        "SG 10202412345A",
        "High-Efficiency Lithium-Ion Battery Cell",
        "Tech Innovations Pte Ltd",
        NaiveDate::from_ymd_opt(2020, 3, 15).unwrap(),
    );

    patent.inventors = vec!["Dr. Li Wei".to_string(), "Dr. Tan Mei Ling".to_string()];
    patent.status = PatentStatus::Granted;
    patent.grant_date = Some(NaiveDate::from_ymd_opt(2022, 8, 10).unwrap());
    patent.ipc_classification = vec!["H01M 4/02".to_string()];
    patent.abstract_text = "A novel battery cell structure improving energy density by 30% through optimized electrode configuration".to_string();

    // Add a claim
    patent.claims.push(PatentClaim {
        number: 1,
        claim_type: ClaimType::Independent,
        text: "A battery cell comprising: (a) a cathode with layered structure; (b) an anode with silicon nanoparticles; (c) electrolyte with enhanced conductivity".to_string(),
        depends_on: Vec::new(),
    });

    match validate_patent(&patent) {
        Ok(report) => {
            println!("✅ Patent Status: VALID\n");
            println!("📊 Validation Report:");
            println!("   Application Number: {}", patent.application_number);
            println!("   Filing Date: {}", patent.filing_date);
            println!("   Expiry Date: {}", patent.expiry_date());
            println!("   Years Remaining: {} years", report.years_remaining);
            println!("   Status: {:?}", patent.status);
            println!(
                "   Claims: {} independent/dependent claims",
                patent.claims.len()
            );

            if !report.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("   • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("❌ Patent Status: INVALID\n");
            println!("🚨 Error: {}", e);
        }
    }

    // Demonstrate patentability assessment
    println!("\n🔍 Patentability Assessment:");
    match assess_patentability(
        "Novel battery structure",
        false, // No prior art
        false, // Not obvious
        true,  // Industrially applicable
    ) {
        Ok(()) => println!(
            "   ✓ Meets all patentability criteria (novelty, inventive step, industrial application)"
        ),
        Err(e) => println!("   ✗ {}", e),
    }
}

fn validate_trademark_example() {
    println!("™️  Trademark: TECHSPHERE for Electronics");
    println!("{}\n", "─".repeat(66));

    let mut trademark1 = Trademark::new(
        "T2024123456",
        "TECHSPHERE",
        "TechSphere Pte Ltd",
        NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
    );

    trademark1.classes = vec![9]; // Class 9: Computers and scientific devices
    trademark1.specifications.push(TrademarkSpecification {
        class: 9,
        description: "Computers, smartphones, tablets, electronic devices".to_string(),
    });
    trademark1.status = TrademarkStatus::Pending;
    trademark1.mark_type = TrademarkType::Word;

    // Create a potentially conflicting mark
    let mut existing_mark = Trademark::new(
        "T2010000001",
        "TECHNOSPHERE",
        "Other Tech Company",
        NaiveDate::from_ymd_opt(2010, 5, 20).unwrap(),
    );
    existing_mark.classes = vec![9];
    existing_mark.status = TrademarkStatus::Registered;
    existing_mark.registration_date = Some(NaiveDate::from_ymd_opt(2011, 3, 10).unwrap());

    println!("📊 Trademark Details:");
    println!("   Mark: {}", trademark1.mark);
    println!("   Proprietor: {}", trademark1.proprietor);
    println!("   Classes: {:?}", trademark1.classes);
    println!("   Filing Date: {}", trademark1.filing_date);
    println!("   Type: {:?}\n", trademark1.mark_type);

    // Calculate similarity
    let similarity = trademark1.similarity_score(&existing_mark);
    println!("🔍 Similarity Analysis:");
    println!(
        "   Comparing: '{}' vs '{}'",
        trademark1.mark, existing_mark.mark
    );
    println!("   Similarity Score: {}%", similarity);
    println!("   Threshold: 70% (Trade Marks Act s. 8(2))");

    if similarity >= 70 {
        println!("   ⚠️  High similarity - likely to cause confusion");
    } else {
        println!("   ✓ Acceptable similarity level");
    }

    // Validate against existing marks
    println!("\n📋 Registration Assessment:");
    match validate_trademark(&trademark1, &[existing_mark.clone()]) {
        Ok(report) => {
            if report.is_registrable {
                println!("✅ Trademark: REGISTRABLE\n");
            } else {
                println!("❌ Trademark: NOT REGISTRABLE\n");
            }

            if !report.errors.is_empty() {
                println!("🚨 Errors:");
                for error in &report.errors {
                    println!("   • {}", error);
                }
            }

            if !report.conflicts.is_empty() {
                println!("\n⚔️  Conflicts Detected:");
                for conflict in &report.conflicts {
                    println!("   • Conflicting Mark: '{}'", conflict.conflicting_mark);
                    println!("     Registration: {}", conflict.conflicting_registration);
                    println!("     Similarity: {}%", conflict.similarity_score);
                    println!("     Common Classes: {:?}", conflict.common_classes);
                }
            }

            if !report.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("   • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }

    // Distinctiveness assessment
    println!("\n🎯 Distinctiveness Assessment:");
    match assess_distinctiveness("TECHSPHERE", "electronics") {
        Ok(score) => println!("   ✓ Distinctiveness Score: {}/100", score),
        Err(e) => println!("   ✗ {}", e),
    }
}

fn validate_copyright_example() {
    println!("©️  Copyright: Literary Work");
    println!("{}\n", "─".repeat(66));

    let mut copyright = Copyright::new(
        "The Digital Age: A Comprehensive Guide",
        "Singapore Publishing House Pte Ltd",
        WorkType::Literary,
        NaiveDate::from_ymd_opt(2010, 6, 1).unwrap(),
    );

    copyright.authors.push(Author {
        name: "Prof. John Lim".to_string(),
        birth_year: Some(1965),
        death_year: None, // Still alive
        nationality: Some("Singapore".to_string()),
    });

    copyright.publication_date = Some(NaiveDate::from_ymd_opt(2012, 3, 15).unwrap());
    copyright.is_published = true;
    copyright.country_of_publication = Some("Singapore".to_string());

    println!("📊 Copyright Details:");
    println!("   Title: {}", copyright.title);
    println!("   Work Type: {:?}", copyright.work_type);
    println!("   Owner: {}", copyright.owner);
    println!("   Creation Date: {}", copyright.creation_date);
    println!("   Publication Date: {:?}", copyright.publication_date);
    println!("   Authors: {}", copyright.authors.len());
    for author in &copyright.authors {
        println!(
            "      • {} (b. {})",
            author.name,
            author.birth_year.unwrap_or(0)
        );
    }

    // Validate (assuming author still alive)
    println!("\n📋 Copyright Protection Status:");
    match validate_copyright(&copyright, None) {
        Ok(report) => {
            if report.is_protected {
                println!("✅ Copyright: PROTECTED\n");
                if let Some(years) = report.years_remaining {
                    println!("   Term: Life + 70 years");
                    println!(
                        "   Estimated Protection: {} years remaining (if author alive)",
                        years
                    );
                    println!("   Note: Actual expiry depends on author's death date");
                }
            } else {
                println!("❌ Copyright: EXPIRED\n");
            }

            if !report.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("   • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }

    // Fair dealing assessment
    println!("\n⚖️  Fair Dealing Assessment (Research Purpose):");
    match assess_fair_dealing(
        FairDealingPurpose::Research,
        0.12,  // Using 12% of work
        false, // Non-commercial research
        false, // Doesn't compete with original
    ) {
        Ok(true) => println!("   ✓ Fair dealing applies - permitted use"),
        Ok(false) => println!("   ✗ Fair dealing does not apply"),
        Err(e) => println!("   ✗ {}", e),
    }

    // Example 2: Commercial use of large portion
    println!("\n⚖️  Fair Dealing Assessment (Commercial Use):");
    match assess_fair_dealing(
        FairDealingPurpose::Criticism,
        0.60, // Using 60% of work
        true, // Commercial
        true, // Competes with original
    ) {
        Ok(true) => println!("   ✓ Fair dealing applies"),
        Ok(false) => println!("   ✗ Fair dealing does not apply"),
        Err(e) => println!("   ✗ {}", e),
    }
}

fn validate_design_example() {
    println!("🎨 Registered Design: Smartphone Case");
    println!("{}\n", "─".repeat(66));

    let mut design = RegisteredDesign::new(
        "D2022/00456",
        "Ergonomic Smartphone Protective Case",
        "Design Studio Pte Ltd",
        NaiveDate::from_ymd_opt(2022, 4, 10).unwrap(),
    );

    design.designer = "Alice Tan".to_string();
    design.registration_date = Some(NaiveDate::from_ymd_opt(2022, 9, 15).unwrap());
    design.products = vec![
        "Smartphone cases".to_string(),
        "Protective covers".to_string(),
    ];
    design.locarno_classes = vec!["19-06".to_string()]; // Stationery and office equipment
    design.status = DesignStatus::Registered;

    println!("📊 Design Details:");
    println!("   Registration: {}", design.registration_number);
    println!("   Title: {}", design.title);
    println!("   Designer: {}", design.designer);
    println!("   Proprietor: {}", design.proprietor);
    println!("   Filing Date: {}", design.filing_date);
    if let Some(reg_date) = design.registration_date {
        println!("   Registration Date: {}", reg_date);
        if let Some(renewal) = design.first_renewal_date() {
            println!("   First Renewal Due: {} (5 years)", renewal);
        }
        if let Some(expiry) = design.maximum_expiry_date() {
            println!("   Maximum Expiry: {} (15 years)", expiry);
        }
    }
    println!("   Products: {:?}", design.products);
    println!("   Status: {:?}", design.status);

    println!("\n📋 Design Validity:");
    match validate_design(&design, &[]) {
        Ok(report) => {
            if report.is_registrable {
                println!("✅ Design: VALID AND REGISTRABLE\n");
            } else {
                println!("❌ Design: ISSUES DETECTED\n");
            }

            if !report.errors.is_empty() {
                println!("🚨 Errors:");
                for error in &report.errors {
                    println!("   • {}", error);
                }
            }

            if !report.similar_designs.is_empty() {
                println!("\n⚠️  Similar Designs:");
                for (title, similarity) in &report.similar_designs {
                    println!("   • '{}' - {}% similar", title, similarity);
                }
            }

            if !report.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("   • {}", warning);
                }
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }

    if let Some(years) = design.years_since_registration() {
        println!("\n⏰ Term Status:");
        println!("   Years Since Registration: {}", years);
        println!("   Renewable Terms: 5 + 5 + 5 years (max 15 years total)");
        if years >= 5 {
            println!("   ⚠️  Renewal required!");
        } else {
            println!("   ✓ Within first 5-year term");
        }
    }

    println!("\n💡 Design Protection Notes:");
    println!("   • Must have novelty and individual character (s. 5)");
    println!("   • Cannot be purely functional (s. 6)");
    println!("   • Registered with IPOS Design Registry");
    println!("   • Protection territorial to Singapore");
}
