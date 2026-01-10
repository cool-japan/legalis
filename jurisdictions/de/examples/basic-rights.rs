//! Basic Rights Example (Grundrechte)
//!
//! Demonstrates German basic rights under Grundgesetz (Basic Law) Articles 1-19:
//! - Human dignity and fundamental rights
//! - Rights restrictions and justification
//! - Constitutional complaint process
//! - Citizens' rights vs human rights

use chrono::NaiveDate;
use legalis_de::grundgesetz::*;

fn main() {
    println!("=== German Basic Rights Examples (Grundrechte) ===\n");
    println!("Constitutional Law under Grundgesetz (GG) Articles 1-19\n");

    // Example 1: Human Dignity (Art. 1 GG)
    println!("📋 Example 1: Human Dignity - Art. 1 GG");
    println!("Inviolable and absolute, foundation of all other rights\n");

    let holder = RightHolder {
        name: "Max Mustermann".to_string(),
        holder_type: RightHolderType::NaturalPerson,
        german_citizen: true,
    };

    let human_dignity_right = BasicRight {
        article: BasicRightArticle::HumanDignity,
        holder: holder.clone(),
        content: "Right to recognition as person with inherent value".to_string(),
        restrictions: vec![], // Art. 1 - No restrictions allowed!
    };

    match validate_basic_right(&human_dignity_right) {
        Ok(()) => {
            println!("✅ Basic Right: VALID");
            println!("   - Article: {:?}", human_dignity_right.article);
            println!(
                "   - Holder: {} ({})",
                holder.name,
                if holder.german_citizen {
                    "German citizen"
                } else {
                    "Non-citizen"
                }
            );
            println!(
                "   - Restrictions: {} (Art. 1 is ABSOLUTE)",
                human_dignity_right.restrictions.len()
            );
            println!("\n💡 Art. 1 Para. 1 GG:");
            println!("   'Human dignity shall be inviolable.");
            println!("    To respect and protect it shall be the duty of all state authority.'");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 2: Freedom of Expression (Art. 5 GG)
    println!("\n📋 Example 2: Freedom of Expression - Art. 5 Para. 1 GG");
    println!("Subject to restrictions by general laws\n");

    let authority = PublicAuthority {
        name: "Bundestag".to_string(),
        authority_type: AuthorityType::Legislative,
        level: FederalLevel::Federal,
    };

    let expression_right = BasicRight {
        article: BasicRightArticle::FreedomOfExpression,
        holder: holder.clone(),
        content: "Right to express opinions freely in speech, writing, and pictures".to_string(),
        restrictions: vec![RightsRestriction {
            restricting_authority: authority.clone(),
            legal_basis: "Criminal Code §185 (Insult)".to_string(),
            restriction_type: RestrictionType::Prohibition,
            date_of_restriction: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            justification: "Protection of personal honor".to_string(),
        }],
    };

    match validate_basic_right(&expression_right) {
        Ok(()) => {
            println!("✅ Freedom of Expression: VALID with restrictions");
            println!("   - Holder: {}", holder.name);
            println!("   - Restrictions: {}", expression_right.restrictions.len());
            for (i, restriction) in expression_right.restrictions.iter().enumerate() {
                println!("     {}. {}", i + 1, restriction.legal_basis);
                println!("        Justification: {}", restriction.justification);
            }
            println!("\n💡 Art. 5 Para. 2 GG allows restrictions by:");
            println!("   - General laws (allgemeine Gesetze)");
            println!("   - Legal provisions for protection of youth");
            println!("   - Right to personal honor");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 3: Freedom of Assembly - German Citizens Only
    println!("\n📋 Example 3: Freedom of Assembly - Art. 8 GG (Germans Only)");
    println!("Example of Deutschenrecht (citizens' right)\n");

    let german_holder = RightHolder {
        name: "Erika Schmidt".to_string(),
        holder_type: RightHolderType::NaturalPerson,
        german_citizen: true,
    };

    let non_german_holder = RightHolder {
        name: "John Doe".to_string(),
        holder_type: RightHolderType::NaturalPerson,
        german_citizen: false,
    };

    // German citizen - should be valid
    match validate_right_holder(&german_holder, BasicRightArticle::FreedomOfAssembly) {
        Ok(()) => {
            println!("✅ German citizen: {} - ENTITLED", german_holder.name);
            println!("   - Art. 8 GG: 'All Germans have the right to assemble...'");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Non-German - should fail
    match validate_right_holder(&non_german_holder, BasicRightArticle::FreedomOfAssembly) {
        Ok(()) => println!("✅ Non-citizen entitled (unexpected)"),
        Err(e) => {
            println!("❌ Non-citizen: {} - NOT ENTITLED", non_german_holder.name);
            println!("   Error: {}", e);
            println!("\n💡 Deutschenrechte (Citizens' Rights):");
            println!("   - Art. 8: Freedom of assembly");
            println!("   - Art. 9: Freedom of association");
            println!("   - Art. 11: Freedom of movement");
            println!("   - Art. 12: Occupational freedom");
        }
    }

    // Example 4: Property Rights (Art. 14 GG)
    println!("\n📋 Example 4: Property Rights - Art. 14 GG");
    println!("Property and inheritance guaranteed, with social obligations\n");

    let property_right = BasicRight {
        article: BasicRightArticle::PropertyRights,
        holder: holder.clone(),
        content: "Ownership of land and assets".to_string(),
        restrictions: vec![RightsRestriction {
            restricting_authority: authority.clone(),
            legal_basis: "Building Code - Height restrictions".to_string(),
            restriction_type: RestrictionType::ContentRegulation,
            date_of_restriction: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            justification: "Urban planning and public welfare".to_string(),
        }],
    };

    match validate_basic_right(&property_right) {
        Ok(()) => {
            println!("✅ Property Rights: VALID");
            println!("   - Art. 14 Para. 1: Property and inheritance are guaranteed");
            println!("   - Art. 14 Para. 2: Property entails obligations (social function)");
            println!(
                "   - Art. 14 Para. 3: Expropriation only for public welfare with compensation"
            );
            println!(
                "\n   Restriction: {}",
                property_right.restrictions[0].legal_basis
            );
            println!(
                "   Justification: {}",
                property_right.restrictions[0].justification
            );
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 5: Occupational Freedom (Art. 12 GG)
    println!("\n📋 Example 5: Occupational Freedom - Art. 12 GG");
    println!("Freedom to choose occupation, profession, or trade\n");

    let lawyer_restriction = RightsRestriction {
        restricting_authority: PublicAuthority {
            name: "Federal Bar Association".to_string(),
            authority_type: AuthorityType::Executive,
            level: FederalLevel::Federal,
        },
        legal_basis: "Federal Lawyers Act (BRAO) - Bar exam requirement".to_string(),
        restriction_type: RestrictionType::PermitRequirement,
        date_of_restriction: NaiveDate::from_ymd_opt(2015, 1, 1).unwrap(),
        justification: "Protection of clients and administration of justice".to_string(),
    };

    let occupational_right = BasicRight {
        article: BasicRightArticle::OccupationalFreedom,
        holder: german_holder.clone(),
        content: "Right to practice law".to_string(),
        restrictions: vec![lawyer_restriction],
    };

    match validate_basic_right(&occupational_right) {
        Ok(()) => {
            println!("✅ Occupational Freedom: VALID with restrictions");
            println!("   - Art. 12 Para. 1: All Germans have right to choose occupation");
            println!("   - Three-step theory (Drei-Stufen-Theorie):");
            println!("     1. Occupational execution (Berufsausübung) - easiest to restrict");
            println!("     2. Objective admission requirements (objektive Zulassung)");
            println!("     3. Subjective admission requirements (subjektive Zulassung) - hardest");
            println!("\n   Example: Bar exam is subjective admission requirement");
            println!("   Justification: Protection of important public goods");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 6: Constitutional Complaint (Verfassungsbeschwerde)
    println!("\n📋 Example 6: Constitutional Complaint - Art. 93 Para. 1 No. 4a GG");
    println!("Individual complaint to Federal Constitutional Court\n");

    let complaint = ConstitutionalComplaint {
        complainant: holder.clone(),
        violated_right: BasicRightArticle::FreedomOfExpression,
        infringing_act: InfringingAct::Statute {
            name: "Network Enforcement Act".to_string(),
            date: NaiveDate::from_ymd_opt(2021, 6, 1).unwrap(),
        },
        subsidiarity_met: true,
        directly_affected: true,
        filed_within_deadline: true,
        complaint_date: NaiveDate::from_ymd_opt(2022, 1, 15).unwrap(),
    };

    match validate_constitutional_complaint(&complaint) {
        Ok(()) => {
            println!("✅ Constitutional Complaint: ADMISSIBLE");
            println!("   - Complainant: {}", complaint.complainant.name);
            println!("   - Violated right: {:?}", complaint.violated_right);
            println!("   - Infringing act: Statute");
            println!("\n   Admissibility requirements met:");
            println!("   ✓ Subsidiarity (legal remedies exhausted)");
            println!("   ✓ Standing (personally, currently, directly affected)");
            println!("   ✓ Deadline (within one year of enactment)");
            println!("\n💡 Verfassungsbeschwerde Process:");
            println!("   1. File complaint with Federal Constitutional Court (BVerfG)");
            println!("   2. Court checks admissibility (90% rejected)");
            println!("   3. If admissible, merits hearing");
            println!("   4. Decision: Successful/Unsuccessful/Inadmissible");
        }
        Err(e) => println!("❌ Complaint: INADMISSIBLE - {}", e),
    }

    // Example 7: Public Authority Cannot Be Right Holder
    println!("\n📋 Example 7: Public Authority Not a Basic Right Holder");
    println!("Grundrechte protect individuals AGAINST the state\n");

    let public_authority_holder = RightHolder {
        name: "City of Berlin".to_string(),
        holder_type: RightHolderType::PublicAuthority,
        german_citizen: true,
    };

    match validate_right_holder(
        &public_authority_holder,
        BasicRightArticle::FreedomOfExpression,
    ) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Public Authority: NOT A RIGHT HOLDER");
            println!("   Error: {}", e);
            println!("\n💡 Basic rights are defensive rights (Abwehrrechte):");
            println!("   - Protect individuals AGAINST state action");
            println!("   - Public authorities are OBLIGATED to respect rights");
            println!("   - Exception: Legal entities can hold some rights (Art. 19 Para. 3 GG)");
        }
    }

    // Summary
    println!("\n=== Summary: German Basic Rights (Grundrechte) ===");
    println!("\n📚 Article Structure:");
    println!("   Art. 1: Human dignity (absolute, inviolable)");
    println!("   Art. 2: General freedom of action, right to life");
    println!("   Art. 3: Equality before the law");
    println!("   Art. 4: Freedom of faith and conscience");
    println!("   Art. 5: Freedom of expression, press, art, science");
    println!("   Art. 6-7: Marriage, family, education");
    println!("   Art. 8-12: Assembly, association, movement, occupation (Germans only)");
    println!("   Art. 13-19: Home, property, citizenship, petition, legal recourse");
    println!("\n⚖️ Key Principles:");
    println!("   - Menschenrechte (Human rights): For all persons");
    println!("   - Deutschenrechte (Citizens' rights): For German citizens only");
    println!("   - Abwehrrechte (Defensive rights): Against state action");
    println!("   - Art. 19 Para. 2: Essential content cannot be violated");
    println!("   - Art. 19 Para. 3: Legal entities can hold applicable rights");
    println!("\n🏛️ Enforcement:");
    println!("   - Verfassungsbeschwerde (Constitutional complaint) to BVerfG");
    println!("   - Subsidiarity: Exhaust other remedies first");
    println!("   - Standing: Self, current, immediate (selbst, gegenwärtig, unmittelbar)");
    println!("   - Deadline: 1 month (decisions) or 1 year (statutes)");
}
