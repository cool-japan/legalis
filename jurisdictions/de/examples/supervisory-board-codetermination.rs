//! Supervisory Board Co-Determination Example
//!
//! Demonstrates supervisory board (Aufsichtsrat) co-determination under German law,
//! including the Mitbestimmungsgesetz (MitbestG) and Drittelbeteiligungsgesetz (DrittelbG).
//!
//! # Legal Context
//!
//! ## Co-Determination in Germany (Mitbestimmung)
//!
//! German law requires employee representation on supervisory boards of larger
//! companies, creating a unique two-tier board system.
//!
//! ### Thresholds and Requirements
//!
//! | Employee Count | Law | Co-Determination Type | Employee Representation |
//! |----------------|-----|------------------------|-------------------------|
//! | < 500 | None | None | 0% |
//! | 500-1,999 | DrittelbG | One-Third | 33% |
//! | 2,000+ | MitbestG | Full Parity | 50% |
//! | Coal/Steel | MontanMitbestG | Montan | 50% + special rules |
//!
//! ### Drittelbeteiligungsgesetz (DrittelbG)
//!
//! - Applies to companies with 500-1,999 employees
//! - **One-third** of supervisory board must be employee representatives
//! - Board size: Typically 9 members (3 employees, 6 shareholders)
//! - Simpler structure than full parity co-determination
//!
//! ### Mitbestimmungsgesetz (MitbestG)
//!
//! - Applies to companies with 2,000+ employees
//! - **Full parity** (50%) employee representation
//! - Board size: 12-20 members depending on employee count
//! - Chairperson (shareholder representative) has casting vote in deadlock
//!
//! ### Board Size Requirements (MitbestG)
//!
//! | Employee Count | Board Size | Employee Reps | Shareholder Reps |
//! |----------------|------------|---------------|------------------|
//! | 2,000-9,999 | 12 | 6 | 6 |
//! | 10,000-19,999 | 16 | 8 | 8 |
//! | 20,000+ | 20 | 10 | 10 |

use legalis_de::arbeitsrecht::*;

fn main() {
    println!("=== Supervisory Board Co-Determination Example ===\n");
    println!("MitbestG & DrittelbG - Employee Board Representation\n");

    // =================================================================
    // Example 1: Small Company (< 500 employees) - No Co-Determination
    // =================================================================

    println!("📋 Example 1: Small Company - No Co-Determination Required");
    println!("──────────────────────────────────────────────────────────\n");

    let small_company_employees = 350;
    let required_codetermination =
        SupervisoryBoard::required_codetermination(small_company_employees);
    let required_size = SupervisoryBoard::required_size(small_company_employees);

    println!("Company: Small Manufacturing GmbH");
    println!("Employee Count: {}", small_company_employees);
    println!("Required Co-Determination: {:?}", required_codetermination);
    println!("Required Board Size: {}", required_size);
    println!("\n✅ No employee representation required for companies < 500 employees");

    let small_board = SupervisoryBoard {
        company_name: "Small Manufacturing GmbH".to_string(),
        employee_count: small_company_employees,
        codetermination_type: CodeterminationType::None,
        total_members: 6,
        employee_representatives: 0,
        shareholder_representatives: 6,
        members: vec![
            BoardMember {
                name: "Dr. Schmidt".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Chairperson),
            },
            BoardMember {
                name: "Dr. Müller".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::DeputyChairperson),
            },
            // ... other shareholder representatives
        ],
    };

    match validate_supervisory_board(&small_board) {
        Ok(()) => {
            println!("   ✅ Board Composition: VALID");
            println!("   - All shareholder representatives");
            println!("   - Minimum 6 members");
        }
        Err(e) => {
            println!("   ❌ Validation Error: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Example 2: Medium Company - DrittelbG (One-Third Participation)
    // =================================================================

    println!("📋 Example 2: Medium Company - One-Third Co-Determination (DrittelbG)");
    println!("─────────────────────────────────────────────────────────────────────\n");

    let medium_company_employees = 850;
    let required_codetermination_2 =
        SupervisoryBoard::required_codetermination(medium_company_employees);
    let required_size_2 = SupervisoryBoard::required_size(medium_company_employees);

    println!("Company: Medium Tech GmbH");
    println!("Employee Count: {}", medium_company_employees);
    println!("Applicable Law: Drittelbeteiligungsgesetz (DrittelbG)");
    println!(
        "Required Co-Determination: {:?}",
        required_codetermination_2
    );
    println!("Required Board Size: {} members", required_size_2);
    println!(
        "Employee Representatives: {} (1/3 of board)",
        required_size_2 / 3
    );
    println!(
        "Shareholder Representatives: {} (2/3 of board)\n",
        required_size_2 - required_size_2 / 3
    );

    let medium_board = SupervisoryBoard {
        company_name: "Medium Tech GmbH".to_string(),
        employee_count: medium_company_employees,
        codetermination_type: CodeterminationType::OneThird,
        total_members: 9,
        employee_representatives: 3,
        shareholder_representatives: 6,
        members: vec![
            // Shareholder representatives (6)
            BoardMember {
                name: "Dr. Anna Weber".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Chairperson),
            },
            BoardMember {
                name: "Prof. Klaus Bauer".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::DeputyChairperson),
            },
            BoardMember {
                name: "Sabine Fischer".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Thomas Schneider".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Dr. Maria Hoffmann".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Stefan Wagner".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            // Employee representatives (3)
            BoardMember {
                name: "Michael Koch".to_string(),
                representative_type: RepresentativeType::Employee,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Julia Zimmermann".to_string(),
                representative_type: RepresentativeType::Employee,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Andreas Schröder".to_string(),
                representative_type: RepresentativeType::Employee,
                position: Some(BoardPosition::Member),
            },
        ],
    };

    match validate_supervisory_board(&medium_board) {
        Ok(()) => {
            println!("✅ Board Composition: VALID (DrittelbG)");
            println!("   Total Members: {}", medium_board.total_members);
            println!(
                "   Employee Representatives: {} (33%)",
                medium_board.employee_representatives
            );
            println!(
                "   Shareholder Representatives: {} (67%)",
                medium_board.shareholder_representatives
            );
            println!("\n   Employee Representatives:");
            for member in &medium_board.members {
                if matches!(member.representative_type, RepresentativeType::Employee) {
                    println!("     - {} ({:?})", member.name, member.position);
                }
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Example 3: Large Company - MitbestG (Full Parity)
    // =================================================================

    println!("📋 Example 3: Large Company - Full Parity Co-Determination (MitbestG)");
    println!("──────────────────────────────────────────────────────────────────────\n");

    let large_company_employees = 5_500;
    let required_codetermination_3 =
        SupervisoryBoard::required_codetermination(large_company_employees);
    let required_size_3 = SupervisoryBoard::required_size(large_company_employees);

    println!("Company: Large Automotive AG");
    println!("Employee Count: {}", large_company_employees);
    println!("Applicable Law: Mitbestimmungsgesetz (MitbestG)");
    println!(
        "Required Co-Determination: {:?}",
        required_codetermination_3
    );
    println!("Required Board Size: {} members", required_size_3);
    println!("Employee Representatives: {} (50%)", required_size_3 / 2);
    println!(
        "Shareholder Representatives: {} (50%)\n",
        required_size_3 / 2
    );

    let large_board = SupervisoryBoard {
        company_name: "Large Automotive AG".to_string(),
        employee_count: large_company_employees,
        codetermination_type: CodeterminationType::Full,
        total_members: 12,
        employee_representatives: 6,
        shareholder_representatives: 6,
        members: vec![
            // Shareholder representatives (6) - including chairperson with casting vote
            BoardMember {
                name: "Dr. Heinrich Müller".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Chairperson), // Has casting vote in deadlock
            },
            BoardMember {
                name: "Prof. Sabine Weber".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Klaus Fischer".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Dr. Maria Schmidt".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Thomas Becker".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Dr. Andrea Hoffmann".to_string(),
                representative_type: RepresentativeType::Shareholder,
                position: Some(BoardPosition::Member),
            },
            // Employee representatives (6)
            BoardMember {
                name: "Wolfgang Schneider".to_string(),
                representative_type: RepresentativeType::Employee,
                position: Some(BoardPosition::DeputyChairperson), // Typically an employee rep
            },
            BoardMember {
                name: "Petra Wagner".to_string(),
                representative_type: RepresentativeType::Employee,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Michael Koch".to_string(),
                representative_type: RepresentativeType::Union, // IG Metall representative
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Julia Zimmermann".to_string(),
                representative_type: RepresentativeType::Employee,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Andreas Schröder".to_string(),
                representative_type: RepresentativeType::Employee,
                position: Some(BoardPosition::Member),
            },
            BoardMember {
                name: "Sabrina Krüger".to_string(),
                representative_type: RepresentativeType::Employee,
                position: Some(BoardPosition::Member),
            },
        ],
    };

    match validate_supervisory_board(&large_board) {
        Ok(()) => {
            println!("✅ Board Composition: VALID (MitbestG - Full Parity)");
            println!("   Total Members: {}", large_board.total_members);
            println!(
                "   Employee Representatives: {} (50%)",
                large_board.employee_representatives
            );
            println!(
                "   Shareholder Representatives: {} (50%)",
                large_board.shareholder_representatives
            );
            println!("\n   🔑 Key Feature: Chairperson has casting vote in deadlock");
            println!("\n   Employee Representatives:");
            for member in &large_board.members {
                if matches!(
                    member.representative_type,
                    RepresentativeType::Employee | RepresentativeType::Union
                ) {
                    println!(
                        "     - {} ({:?}, {:?})",
                        member.name, member.position, member.representative_type
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Example 4: Very Large Company - 20-Member Board
    // =================================================================

    println!("📋 Example 4: Very Large Company - Maximum Board Size");
    println!("──────────────────────────────────────────────────────\n");

    let very_large_employees = 45_000;
    let required_size_4 = SupervisoryBoard::required_size(very_large_employees);

    println!("Company: Global Manufacturing AG");
    println!("Employee Count: {}", very_large_employees);
    println!("Required Board Size: {} members (maximum)", required_size_4);
    println!("Employee Representatives: {}", required_size_4 / 2);
    println!("Shareholder Representatives: {}", required_size_4 / 2);

    println!("\n");

    // =================================================================
    // Example 5: Invalid Board - Incorrect Ratio
    // =================================================================

    println!("📋 Example 5: Invalid Board - Incorrect Employee Ratio");
    println!("───────────────────────────────────────────────────────\n");

    let invalid_board = SupervisoryBoard {
        company_name: "Incorrect Corp AG".to_string(),
        employee_count: 3_000, // Should require 50% employee representation
        codetermination_type: CodeterminationType::Full,
        total_members: 12,
        employee_representatives: 4, // WRONG! Should be 6 (50%)
        shareholder_representatives: 8,
        members: vec![], // Empty for brevity
    };

    match validate_supervisory_board(&invalid_board) {
        Ok(()) => {
            println!("✅ Unexpectedly Valid");
        }
        Err(e) => {
            println!("❌ Expected Validation Error:");
            println!("   {}", e);
            println!("\n   Explanation: MitbestG requires 50% employee representation");
            println!("   for companies with 2,000+ employees.");
        }
    }

    println!("\n");

    // =================================================================
    // Summary
    // =================================================================

    println!("════════════════════════════════════════════════════════");
    println!("📊 Summary: Supervisory Board Co-Determination");
    println!("════════════════════════════════════════════════════════\n");

    println!("✅ Valid Board Structures:");
    println!("   1. Small Company (350 employees):");
    println!("      - No co-determination required");
    println!("      - 6 shareholder representatives only\n");

    println!("   2. Medium Company (850 employees - DrittelbG):");
    println!("      - One-third employee representation");
    println!("      - 9 members: 3 employees, 6 shareholders\n");

    println!("   3. Large Company (5,500 employees - MitbestG):");
    println!("      - Full parity (50%) employee representation");
    println!("      - 12 members: 6 employees, 6 shareholders");
    println!("      - Chairperson has casting vote\n");

    println!("❌ Invalid Board:");
    println!("   - Incorrect employee/shareholder ratio");
    println!("   - Must comply with statutory requirements\n");

    println!("🔑 Key Legal Principles:");
    println!("   - < 500 employees: No co-determination");
    println!("   - 500-1,999: One-third (DrittelbG)");
    println!("   - 2,000+: Full parity 50% (MitbestG)");
    println!("   - Board size scales with employee count");
    println!("   - Chairperson (shareholder) has casting vote in deadlock");
}
