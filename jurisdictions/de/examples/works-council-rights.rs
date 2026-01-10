//! Works Council Co-Determination Rights Example
//!
//! Demonstrates co-determination rights (Mitbestimmungsrechte) of works councils
//! under the Works Constitution Act (Betriebsverfassungsgesetz - BetrVG).
//!
//! # Legal Context
//!
//! ## Betriebsverfassungsgesetz (BetrVG)
//!
//! The BetrVG governs the relationship between employers and works councils
//! (Betriebsrat) in German workplaces with 5 or more employees.
//!
//! ### Works Council Formation
//!
//! - **§1 BetrVG**: Works council required in establishments with 5+ employees
//! - **§9 BetrVG**: Council size based on employee count
//! - Elected by workforce for 4-year term
//! - Protected position (cannot be dismissed for council work)
//!
//! ### Co-Determination Rights (Mitbestimmungsrechte)
//!
//! Works councils have three levels of participation:
//!
//! 1. **Full Co-Determination** (Volle Mitbestimmung):
//!    - Employer cannot decide without works council agreement
//!    - §87 BetrVG - Social matters
//!    - §99 BetrVG - Personnel matters
//!
//! 2. **Right to Information** (Informationsrecht):
//!    - Works council must be informed
//!    - No veto right
//!
//! 3. **Right to Consultation** (Beratungsrecht):
//!    - Works council must be consulted
//!    - Employer must consider opinion but can proceed
//!
//! ### §87 BetrVG - Social Matters
//!
//! Full co-determination on social matters including working hours, overtime,
//! payment methods, leave scheduling, technical monitoring, health & safety,
//! and social facilities.
//!
//! ### §99 BetrVG - Personnel Matters
//!
//! Works council consent required for hiring, classification, transfer, and termination.
//!
//! ###  §98 BetrVG - Vocational Training
//!
//! Co-determination on training programs and trainer selection.

use legalis_de::arbeitsrecht::*;

fn main() {
    println!("=== Works Council Co-Determination Rights Example ===\n");
    println!("Betriebsverfassungsgesetz (BetrVG) - §87, §98, §99\n");

    // =================================================================
    // Example 1: Social Matters Co-Determination (§87 BetrVG)
    // =================================================================

    println!("📋 Example 1: Social Matters Co-Determination (§87 BetrVG)");
    println!("────────────────────────────────────────────────────────────\n");

    let company_name = "Tech Solutions GmbH";
    let employee_count = 250;

    println!("Company: {}", company_name);
    println!("Employee Count: {}", employee_count);
    println!("Works Council: Required (§1 BetrVG - 5+ employees)\n");

    // Check if works council is required
    if WorksCouncil::is_required(employee_count) {
        let required_size = WorksCouncil::required_size(employee_count);
        println!(
            "✅ Works Council Size: {} members (§9 BetrVG)",
            required_size
        );
    }

    println!("\n🔑 Co-Determination Rights (§87 BetrVG):\n");

    let codetermination_rights = CodeterminationRights {
        company_name: company_name.to_string(),
        employee_count,
        rights: vec![
            CodeterminationRight {
                right_type: CodeterminationRightType::WorkingHours,
                description: "Start and end of daily working hours, including breaks".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 2 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::Overtime,
                description: "Overtime and short-time work arrangements".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 3 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::PaymentMethods,
                description: "Time, place, and method of payment of remuneration".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 4 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::LeaveScheduling,
                description: "General principles for scheduling annual leave".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 5 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::TechnicalMonitoring,
                description:
                    "Technical devices designed to monitor employee behavior or performance"
                        .to_string(),
                legal_basis: "§87 Abs. 1 Nr. 6 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::HealthAndSafety,
                description: "Regulations on health and safety at work".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 7 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::SocialFacilities,
                description: "Form, structure and administration of social facilities".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 8 BetrVG".to_string(),
            },
        ],
    };

    // Validate co-determination rights
    match validate_codetermination_rights(&codetermination_rights) {
        Ok(()) => {
            println!("✅ Co-Determination Rights Framework: VALID\n");

            println!(
                "Implemented Rights ({}):",
                codetermination_rights.rights.len()
            );
            for (i, right) in codetermination_rights.rights.iter().enumerate() {
                println!("\n   {}. {:?}", i + 1, right.right_type);
                println!("      Legal Basis: {}", right.legal_basis);
                println!("      Description: {}", right.description);
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Example 2: Personnel Matters Co-Determination (§99 BetrVG)
    // =================================================================

    println!("📋 Example 2: Personnel Matters Co-Determination (§99 BetrVG)");
    println!("────────────────────────────────────────────────────────────────\n");

    let personnel_rights = CodeterminationRights {
        company_name: company_name.to_string(),
        employee_count,
        rights: vec![CodeterminationRight {
            right_type: CodeterminationRightType::PersonnelSelection,
            description: "Works council consent required for hiring, classification, transfer, and termination".to_string(),
            legal_basis: "§99 BetrVG".to_string(),
        }],
    };

    match validate_codetermination_rights(&personnel_rights) {
        Ok(()) => {
            println!("✅ Personnel Co-Determination: VALID");
            println!("\n🔑 Key Point (§99 BetrVG):");
            println!(
                "   - Employer CANNOT hire, transfer, or reclassify without works council consent"
            );
            println!("   - Works council has 1 week to object");
            println!("   - Objection grounds: violation of law, collective agreement,");
            println!("     or risk to existing employees");
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Example 3: Vocational Training Co-Determination (§98 BetrVG)
    // =================================================================

    println!("📋 Example 3: Vocational Training Co-Determination (§98 BetrVG)");
    println!("──────────────────────────────────────────────────────────────────\n");

    let training_rights = CodeterminationRights {
        company_name: company_name.to_string(),
        employee_count,
        rights: vec![CodeterminationRight {
            right_type: CodeterminationRightType::VocationalTraining,
            description:
                "Co-determination on establishment and implementation of training programs"
                    .to_string(),
            legal_basis: "§98 BetrVG".to_string(),
        }],
    };

    match validate_codetermination_rights(&training_rights) {
        Ok(()) => {
            println!("✅ Training Co-Determination: VALID");
            println!("\n🔑 Key Point (§98 BetrVG):");
            println!("   - Works council involved in all training decisions");
            println!("   - Ensures fair access to professional development");
            println!("   - Applies to apprenticeships and continuing education");
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Example 4: Invalid - Company Too Small
    // =================================================================

    println!("📋 Example 4: Invalid - No Works Council (< 5 employees)");
    println!("─────────────────────────────────────────────────────────\n");

    let small_company_rights = CodeterminationRights {
        company_name: "Small Startup GmbH".to_string(),
        employee_count: 3, // Too small!
        rights: vec![CodeterminationRight {
            right_type: CodeterminationRightType::WorkingHours,
            description: "Working hours co-determination".to_string(),
            legal_basis: "§87 Abs. 1 Nr. 2 BetrVG".to_string(),
        }],
    };

    match validate_codetermination_rights(&small_company_rights) {
        Ok(()) => {
            println!("✅ Unexpectedly Valid");
        }
        Err(e) => {
            println!("❌ Expected Validation Error:");
            println!("   {}", e);
            println!("\n   Explanation: Works council only required with 5+ employees (§1 BetrVG)");
        }
    }

    println!("\n");

    // =================================================================
    // Example 5: Complete Rights Framework
    // =================================================================

    println!("📋 Example 5: Complete Co-Determination Framework");
    println!("──────────────────────────────────────────────────────\n");

    let complete_rights = CodeterminationRights {
        company_name: "Comprehensive Corp AG".to_string(),
        employee_count: 500,
        rights: vec![
            // All §87 social matters rights
            CodeterminationRight {
                right_type: CodeterminationRightType::WorkingHours,
                description: "Working hours regulation".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 2 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::Overtime,
                description: "Overtime arrangements".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 3 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::PaymentMethods,
                description: "Payment procedures".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 4 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::LeaveScheduling,
                description: "Leave planning principles".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 5 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::TechnicalMonitoring,
                description: "Employee monitoring systems".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 6 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::HealthAndSafety,
                description: "Workplace safety regulations".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 7 BetrVG".to_string(),
            },
            CodeterminationRight {
                right_type: CodeterminationRightType::SocialFacilities,
                description: "Social facility management".to_string(),
                legal_basis: "§87 Abs. 1 Nr. 8 BetrVG".to_string(),
            },
            // §99 Personnel matters
            CodeterminationRight {
                right_type: CodeterminationRightType::PersonnelSelection,
                description: "Personnel decisions".to_string(),
                legal_basis: "§99 BetrVG".to_string(),
            },
            // §98 Training
            CodeterminationRight {
                right_type: CodeterminationRightType::VocationalTraining,
                description: "Training programs".to_string(),
                legal_basis: "§98 BetrVG".to_string(),
            },
        ],
    };

    match validate_codetermination_rights(&complete_rights) {
        Ok(()) => {
            println!("✅ Complete Framework: VALID");
            println!(
                "\n   Total Rights Implemented: {}",
                complete_rights.rights.len()
            );
            println!("   - Social Matters (§87): 7 rights");
            println!("   - Personnel Matters (§99): 1 right");
            println!("   - Vocational Training (§98): 1 right");
            println!("\n   ✅ Comprehensive co-determination framework established");
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Summary
    // =================================================================

    println!("════════════════════════════════════════════════════════");
    println!("📊 Summary: Works Council Co-Determination Rights");
    println!("════════════════════════════════════════════════════════\n");

    println!("✅ Co-Determination Categories:");
    println!("\n   1. Social Matters (§87 BetrVG) - Full Co-Determination:");
    println!("      - Working hours, overtime, payment methods");
    println!("      - Leave scheduling, technical monitoring");
    println!("      - Health & safety, social facilities");
    println!("      → Employer CANNOT decide without works council agreement\n");

    println!("   2. Personnel Matters (§99 BetrVG) - Consent Required:");
    println!("      - Hiring, classification, transfer");
    println!("      - Works council has 1 week to object");
    println!("      → Employer MUST obtain works council consent\n");

    println!("   3. Vocational Training (§98 BetrVG) - Co-Determination:");
    println!("      - Training program design and implementation");
    println!("      - Trainer selection, participation criteria");
    println!("      → Joint decision-making required\n");

    println!("❌ Invalid Scenarios:");
    println!("   - Company < 5 employees (no works council required)");
    println!("   - Missing legal basis or descriptions\n");

    println!("🔑 Key Legal Principles:");
    println!("   - Works council required with 5+ employees (§1 BetrVG)");
    println!("   - Council size scales with employee count (§9 BetrVG)");
    println!("   - Full co-determination = employer cannot proceed without agreement");
    println!("   - Violation of co-determination rights makes decision void");
    println!("   - Works council members have special dismissal protection");
}
