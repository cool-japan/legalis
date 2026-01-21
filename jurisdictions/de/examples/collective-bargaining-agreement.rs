//! Collective Bargaining Agreement (Tarifvertrag) Example - TVG
//!
//! Demonstrates the creation and validation of collective bargaining agreements
//! under the German Collective Bargaining Act (Tarifvertragsgesetz - TVG).
//!
//! # Legal Context
//!
//! ## Tarifvertragsgesetz (TVG)
//!
//! The TVG governs collective agreements between unions (Gewerkschaften) and
//! employer associations (Arbeitgeberverbände) or individual employers.
//!
//! ### Key Provisions
//!
//! - **§1 TVG**: Collective agreement formation
//!   - Normative provisions (binding contract terms): wages, hours, leave
//!   - Obligational provisions (union-employer relationship)
//!
//! - **§3 TVG**: Binding effect (Tarifbindung)
//!   - Directly applies to union members and association member companies
//!   - Mandatory and cannot be contracted around (unless more favorable)
//!
//! - **§4 Abs. 5 TVG**: After-effect (Nachwirkung)
//!   - Provisions continue after expiry until replaced by new agreement
//!   - Ensures continuity during bargaining periods
//!
//! ### Agreement Types
//!
//! 1. **Industry-wide** (Branchentarifvertrag): Covers entire industry/sector
//! 2. **Company-level** (Firmentarifvertrag): Single company agreement
//! 3. **Framework** (Manteltarifvertrag): General working conditions
//! 4. **Wage** (Lohntarifvertrag): Compensation structures only

use chrono::NaiveDate;
use legalis_de::arbeitsrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== Collective Bargaining Agreement (Tarifvertrag) Example ===\n");
    println!("Tarifvertragsgesetz (TVG) - Collective Bargaining Act\n");

    // =================================================================
    // Example 1: Valid Industry-Wide Agreement (Branchentarifvertrag)
    // =================================================================

    println!("📋 Example 1: Industry-Wide Agreement - Metal Industry");
    println!("─────────────────────────────────────────────────────────\n");

    // Create union party (Gewerkschaft)
    let union = Union {
        name: "IG Metall".to_string(),
        registered: true,
        member_count: 2_200_000, // One of Germany's largest unions
    };

    // Create employer association (Arbeitgeberverband)
    let employer_association = EmployerAssociation {
        name: "Gesamtmetall - Arbeitgeberverband der Metall- und Elektroindustrie".to_string(),
        member_companies: 6_500,
    };

    // Define bargaining parties
    let parties = BargainingParties {
        union,
        employer_association: Some(employer_association),
        individual_employer: None,
    };

    // Define normative provisions (§1 TVG) - directly binding
    let normative_provisions = vec![
        // Wage scale structure (Lohngruppen)
        NormativeProvision {
            provision_type: ProvisionType::Compensation,
            description: "Wage scale for metal and electrical industry (Entgeltgruppen)"
                .to_string(),
            details: ProvisionDetails::WageScale {
                grades: vec![
                    WageGrade {
                        grade: 1,
                        description: "Unskilled workers (Ungelernte Arbeitskräfte)".to_string(),
                        monthly_wage: Capital::from_euros(3_200), // €3,200/month
                    },
                    WageGrade {
                        grade: 5,
                        description: "Skilled workers (Facharbeiter)".to_string(),
                        monthly_wage: Capital::from_euros(4_200), // €4,200/month
                    },
                    WageGrade {
                        grade: 9,
                        description: "Master craftsmen (Meister)".to_string(),
                        monthly_wage: Capital::from_euros(5_500), // €5,500/month
                    },
                    WageGrade {
                        grade: 12,
                        description: "Engineers (Ingenieure)".to_string(),
                        monthly_wage: Capital::from_euros(7_400), // €7,400/month
                    },
                ],
            },
        },
        // Working hours
        NormativeProvision {
            provision_type: ProvisionType::WorkingHours,
            description: "Standard working week (Regelarbeitszeit)".to_string(),
            details: ProvisionDetails::WorkingHoursSpec {
                hours_per_week: 35,
                days_per_week: 5,
            },
        },
        // Annual leave
        NormativeProvision {
            provision_type: ProvisionType::Leave,
            description: "Annual leave entitlement (Jahresurlaub)".to_string(),
            details: ProvisionDetails::LeaveSpec {
                days_per_year: 30, // 6 weeks for 5-day week
            },
        },
        // Notice periods
        NormativeProvision {
            provision_type: ProvisionType::NoticePeriods,
            description: "Notice periods for termination (Kündigungsfristen)".to_string(),
            details: ProvisionDetails::NoticeSpec {
                weeks: 6, // 6 weeks notice
            },
        },
    ];

    // Define obligational provisions (union-employer relationship)
    let obligational_provisions = vec![
        "Industrial peace obligation during agreement term (Friedenspflicht)".to_string(),
        "Quarterly joint committee meetings (Tarifkommission)".to_string(),
        "Information exchange on industry developments".to_string(),
    ];

    // Create collective bargaining agreement
    let agreement = CollectiveBargainingAgreement {
        agreement_name: "Tarifvertrag für die Metall- und Elektroindustrie Nordrhein-Westfalen"
            .to_string(),
        parties,
        effective_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        expiry_date: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        agreement_type: AgreementType::IndustryWide,
        coverage: AgreementCoverage::Regional {
            region: "Nordrhein-Westfalen".to_string(),
        },
        normative_provisions,
        obligational_provisions,
        registered: true,
    };

    // Validate agreement
    match validate_collective_agreement(&agreement) {
        Ok(()) => {
            println!("✅ Agreement Validation: PASSED");
            println!("   Agreement: {}", agreement.agreement_name);
            println!("   Union: {}", agreement.parties.union.name);
            println!(
                "   Employer Association: {}",
                agreement
                    .parties
                    .employer_association
                    .as_ref()
                    .unwrap()
                    .name
            );
            println!("   Type: {:?}", agreement.agreement_type);
            println!("   Effective: {}", agreement.effective_date);
            println!(
                "   Expiry: {}",
                agreement
                    .expiry_date
                    .unwrap_or(NaiveDate::from_ymd_opt(9999, 12, 31).unwrap())
            );
            println!(
                "   Normative Provisions: {}",
                agreement.normative_provisions.len()
            );
            println!();

            // Display wage scale
            println!("   Wage Scale (Entgeltgruppen):");
            for provision in &agreement.normative_provisions {
                if matches!(provision.provision_type, ProvisionType::Compensation)
                    && let ProvisionDetails::WageScale { grades } = &provision.details
                {
                    for grade in grades {
                        println!(
                            "     - Grade {}: {} - €{:.2}/month",
                            grade.grade,
                            grade.description,
                            grade.monthly_wage.to_euros()
                        );
                    }
                }
            }
            println!();

            // Check validity on different dates
            let check_date_1 = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
            let check_date_2 = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();

            println!("   Validity Check:");
            println!(
                "     - {} (during term): {}",
                check_date_1,
                if agreement.is_valid(check_date_1) {
                    "✅ VALID"
                } else {
                    "❌ INVALID"
                }
            );
            println!(
                "     - {} (after expiry): {}",
                check_date_2,
                if agreement.is_valid(check_date_2) {
                    "✅ VALID"
                } else {
                    "❌ EXPIRED"
                }
            );
            println!();

            // Check after-effect (§4 Abs. 5 TVG)
            println!("   After-Effect (Nachwirkung - §4 Abs. 5 TVG):");
            println!(
                "     - {} (during term): N/A (agreement still valid)",
                check_date_1
            );
            println!(
                "     - {} (after expiry): {}",
                check_date_2,
                if agreement.has_after_effect(check_date_2) {
                    "✅ AFTER-EFFECT APPLIES (provisions continue until replaced)"
                } else {
                    "N/A"
                }
            );
        }
        Err(e) => {
            println!("❌ Agreement Validation: FAILED");
            println!("   Error: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Example 2: Company-Level Agreement (Firmentarifvertrag)
    // =================================================================

    println!("📋 Example 2: Company-Level Agreement");
    println!("─────────────────────────────────────\n");

    let company_agreement = CollectiveBargainingAgreement {
        agreement_name: "Firmentarifvertrag - Tech Solutions GmbH".to_string(),
        parties: BargainingParties {
            union: Union {
                name: "ver.di - Vereinte Dienstleistungsgewerkschaft".to_string(),
                registered: true,
                member_count: 1_900_000,
            },
            employer_association: None,
            individual_employer: Some("Tech Solutions GmbH".to_string()),
        },
        effective_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
        expiry_date: Some(NaiveDate::from_ymd_opt(2026, 3, 31).unwrap()),
        agreement_type: AgreementType::CompanyLevel,
        coverage: AgreementCoverage::Company {
            company_name: "Tech Solutions GmbH".to_string(),
        },
        normative_provisions: vec![
            NormativeProvision {
                provision_type: ProvisionType::WorkingHours,
                description: "Flexible working hours with core time".to_string(),
                details: ProvisionDetails::WorkingHoursSpec {
                    hours_per_week: 38,
                    days_per_week: 5,
                },
            },
            NormativeProvision {
                provision_type: ProvisionType::Leave,
                description: "Enhanced annual leave".to_string(),
                details: ProvisionDetails::LeaveSpec { days_per_year: 28 },
            },
        ],
        obligational_provisions: vec![
            "Remote work policy framework (Homeoffice-Regelung)".to_string(),
            "Training and development commitments".to_string(),
        ],
        registered: true,
    };

    match validate_collective_agreement(&company_agreement) {
        Ok(()) => {
            println!("✅ Company Agreement Validation: PASSED");
            println!("   Agreement: {}", company_agreement.agreement_name);
            println!("   Type: Company-level (Firmentarifvertrag)");
            println!(
                "   Individual Employer: {}",
                company_agreement
                    .parties
                    .individual_employer
                    .as_ref()
                    .unwrap()
            );
            println!("   Duration: 2 years");
        }
        Err(e) => {
            println!("❌ Validation Failed: {}", e);
        }
    }

    println!("\n");

    // =================================================================
    // Example 3: Invalid Agreement (Missing Normative Provisions)
    // =================================================================

    println!("📋 Example 3: Invalid Agreement - No Normative Provisions");
    println!("─────────────────────────────────────────────────────────\n");

    let invalid_agreement = CollectiveBargainingAgreement {
        agreement_name: "Invalid Tarifvertrag".to_string(),
        parties: BargainingParties {
            union: Union {
                name: "Test Union".to_string(),
                registered: true,
                member_count: 1_000,
            },
            employer_association: Some(EmployerAssociation {
                name: "Test Association".to_string(),
                member_companies: 10,
            }),
            individual_employer: None,
        },
        effective_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        expiry_date: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
        agreement_type: AgreementType::IndustryWide,
        coverage: AgreementCoverage::National,
        normative_provisions: vec![], // Empty - INVALID!
        obligational_provisions: vec!["Some obligations".to_string()],
        registered: false,
    };

    match validate_collective_agreement(&invalid_agreement) {
        Ok(()) => {
            println!("✅ Unexpectedly Valid");
        }
        Err(e) => {
            println!("❌ Expected Validation Error:");
            println!("   {}", e);
            println!("\n   Explanation: §1 TVG requires normative provisions");
            println!("   (direct and mandatory contract terms like wages, hours)");
        }
    }

    println!("\n");

    // =================================================================
    // Summary
    // =================================================================

    println!("════════════════════════════════════════════════════════");
    println!("📊 Summary: Collective Bargaining Agreements (TVG)");
    println!("════════════════════════════════════════════════════════\n");

    println!("✅ Valid Agreements:");
    println!("   1. Industry-wide agreement (Branchentarifvertrag)");
    println!("      - Union + Employer Association");
    println!("      - Regional coverage (Nordrhein-Westfalen)");
    println!("      - Comprehensive normative provisions");
    println!("      - After-effect (§4 Abs. 5 TVG) applies after expiry\n");

    println!("   2. Company-level agreement (Firmentarifvertrag)");
    println!("      - Union + Individual Employer");
    println!("      - Single company coverage");
    println!("      - Flexible working arrangements\n");

    println!("❌ Invalid Agreement:");
    println!("   - Missing normative provisions (§1 TVG requirement)");
    println!("   - Cannot have purely obligational agreement\n");

    println!("🔑 Key Legal Principles:");
    println!("   - §1 TVG: Normative provisions are mandatory");
    println!("   - §3 TVG: Direct binding effect (Tarifbindung)");
    println!("   - §4 Abs. 5 TVG: After-effect ensures continuity");
    println!("   - Provisions cannot be contracted around (unless more favorable)");
}
