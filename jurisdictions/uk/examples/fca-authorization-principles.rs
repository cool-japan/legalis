//! FCA Authorization and Principles Examples (FSMA 2000, PRIN)
//!
//! Demonstrates FCA authorization requirements and the 11 Principles for Businesses.

use chrono::NaiveDate;
use legalis_uk::financial_services::*;

fn main() {
    println!("=== UK Financial Services: FCA Authorization & Principles (FSMA 2000, PRIN) ===\n");

    example_1_authorized_firm();
    example_2_unauthorized_activity();
    example_3_all_principles_compliant();
    example_4_breach_principle_6_customers_interests();
    example_5_breach_principle_10_client_assets();
    example_6_client_categorization();
}

/// Example 1: Properly authorized firm conducting permitted activity
fn example_1_authorized_firm() {
    println!("Example 1: Authorized Firm - Permitted Activity");
    println!("────────────────────────────────────────────────");

    let authorization = FcaAuthorization {
        firm_reference_number: "123456".to_string(),
        firm_name: "London Investment Advisers Ltd".to_string(),
        status: AuthorizationStatus::Authorized,
        authorization_date: NaiveDate::from_ymd_opt(2020, 3, 15).unwrap(),
        regulated_activities: vec![
            RegulatedActivity::AdvisingOnInvestments {
                investment_type: InvestmentType::Shares,
            },
            RegulatedActivity::AdvisingOnInvestments {
                investment_type: InvestmentType::Bonds,
            },
            RegulatedActivity::ArrangingDeals,
        ],
        passporting_rights: vec![],
    };

    let activity = RegulatedActivity::AdvisingOnInvestments {
        investment_type: InvestmentType::Shares,
    };

    match validate_fca_authorization(&authorization, &activity) {
        Ok(_) => {
            println!("✓ FCA Authorization VALID");
            println!("  FRN: {}", authorization.firm_reference_number);
            println!("  Firm: {}", authorization.firm_name);
            println!("  Status: {:?}", authorization.status);
            println!("  Activity: {:?}", activity);
            println!("\n  Permitted Regulated Activities:");
            for activity in &authorization.regulated_activities {
                println!("  • {:?}", activity);
            }
            println!("\n  FSMA 2000 s.19 (General Prohibition):");
            println!("  'No person may carry on a regulated activity in the United Kingdom");
            println!("   unless he is an authorised person or an exempt person.'");
            println!("\n  This firm is AUTHORIZED to conduct this activity.");
            println!(
                "  Reference: FSMA 2000 Part 4A (Permission to carry on regulated activities)"
            );
        }
        Err(e) => println!("✗ Authorization invalid: {}", e),
    }
    println!();
}

/// Example 2: Unauthorized activity - firm tries to deal as principal
fn example_2_unauthorized_activity() {
    println!("Example 2: UNAUTHORIZED Activity - Criminal Offence");
    println!("────────────────────────────────────────────────────");

    let authorization = FcaAuthorization {
        firm_reference_number: "789012".to_string(),
        firm_name: "Advisory Only Ltd".to_string(),
        status: AuthorizationStatus::Authorized,
        authorization_date: NaiveDate::from_ymd_opt(2021, 6, 1).unwrap(),
        regulated_activities: vec![RegulatedActivity::AdvisingOnInvestments {
            investment_type: InvestmentType::Shares,
        }],
        passporting_rights: vec![],
    };

    let activity = RegulatedActivity::DealingInInvestmentsPrincipal; // NOT PERMITTED

    match validate_fca_authorization(&authorization, &activity) {
        Ok(_) => println!("✓ Authorized"),
        Err(e) => {
            println!("✗ UNAUTHORIZED ACTIVITY: {}", e);
            println!("\n  CRIMINAL OFFENCE under FSMA 2000 s.19");
            println!("\n  Firm Permissions:");
            for permitted in &authorization.regulated_activities {
                println!("  ✓ {:?}", permitted);
            }
            println!("\n  Attempted Activity:");
            println!("  ✗ {:?}", activity);
            println!("\n  Consequences:");
            println!("  • Criminal offence (FSMA 2000 s.23)");
            println!("  • Up to 2 years imprisonment");
            println!("  • Unlimited fine");
            println!("  • Agreements made may be unenforceable (FSMA 2000 s.26)");
            println!("  • FCA enforcement action");
            println!("\n  Firm must EITHER:");
            println!("  1. Apply for variation of permission (VOP) to add this activity, or");
            println!("  2. Cease conducting this activity immediately");
            println!("\n  Reference: FSMA 2000 s.19 (General prohibition), s.23 (Contravention)");
        }
    }
    println!();
}

/// Example 3: All 11 FCA Principles compliant
fn example_3_all_principles_compliant() {
    println!("Example 3: All 11 FCA Principles for Businesses - Compliant");
    println!("────────────────────────────────────────────────────────────");

    let principles = PrinciplesCompliance {
        integrity: PrincipleCompliance {
            compliant: true,
            evidence: "Firm conducts business with integrity. No misleading statements. Honest dealings with clients and regulators.".to_string(),
            breach_details: None,
        },
        skill_care_diligence: PrincipleCompliance {
            compliant: true,
            evidence: "Staff properly trained. Professional qualifications maintained. Adequate resources and systems.".to_string(),
            breach_details: None,
        },
        management_control: PrincipleCompliance {
            compliant: true,
            evidence: "Robust governance framework. Clear management responsibilities. Adequate systems and controls.".to_string(),
            breach_details: None,
        },
        financial_prudence: PrincipleCompliance {
            compliant: true,
            evidence: "Capital requirements met. Financial resources adequate. Regular monitoring.".to_string(),
            breach_details: None,
        },
        market_conduct: PrincipleCompliance {
            compliant: true,
            evidence: "Proper standards of market conduct observed. No market abuse. Fair dealing.".to_string(),
            breach_details: None,
        },
        customers_interests: PrincipleCompliance {
            compliant: true,
            evidence: "Due regard to customers' interests. Fair treatment. TCF principles embedded.".to_string(),
            breach_details: None,
        },
        communications: PrincipleCompliance {
            compliant: true,
            evidence: "All communications clear, fair and not misleading. Risk warnings prominent. Plain language used.".to_string(),
            breach_details: None,
        },
        conflicts_of_interest: PrincipleCompliance {
            compliant: true,
            evidence: "Conflicts policy in place. Conflicts identified and managed. Client interests prioritized.".to_string(),
            breach_details: None,
        },
        customer_trust: PrincipleCompliance {
            compliant: true,
            evidence: "Suitability assessments conducted. Appropriate advice given. Customers treated fairly.".to_string(),
            breach_details: None,
        },
        client_assets: PrincipleCompliance {
            compliant: true,
            evidence: "Client money segregated. CASS rules complied with. Daily reconciliations performed.".to_string(),
            breach_details: None,
        },
        relations_with_regulators: PrincipleCompliance {
            compliant: true,
            evidence: "Open and cooperative with FCA. Prompt disclosure. Regulatory returns submitted on time.".to_string(),
            breach_details: None,
        },
    };

    match validate_principles_compliance(&principles) {
        Ok(_) => {
            println!("✓ All 11 Principles for Businesses COMPLIANT\n");
            println!("  Principle 1: Integrity ✓");
            println!("  Principle 2: Skill, care and diligence ✓");
            println!("  Principle 3: Management and control ✓");
            println!("  Principle 4: Financial prudence ✓");
            println!("  Principle 5: Market conduct ✓");
            println!("  Principle 6: Customers' interests ✓");
            println!("  Principle 7: Communications with clients ✓");
            println!("  Principle 8: Conflicts of interest ✓");
            println!("  Principle 9: Customers: relationships of trust ✓");
            println!("  Principle 10: Clients' assets ✓");
            println!("  Principle 11: Relations with regulators ✓");
            println!("\n  These are HIGH-LEVEL standards that apply to ALL authorized firms.");
            println!("  They represent the fundamental obligations of financial services firms.");
            println!("\n  Breach of Principles can result in:");
            println!("  • FCA enforcement action");
            println!("  • Fines (unlimited)");
            println!("  • Censure (public statement)");
            println!("  • Restriction of permissions");
            println!("  • Withdrawal of authorization");
            println!("\n  Reference: PRIN (Principles for Businesses) in FCA Handbook");
        }
        Err(e) => println!("✗ Principle breach: {}", e),
    }
    println!();
}

/// Example 4: Breach of Principle 6 - Customers' interests
fn example_4_breach_principle_6_customers_interests() {
    println!("Example 4: BREACH of Principle 6 - Customers' Interests");
    println!("────────────────────────────────────────────────────────");

    let principles = PrinciplesCompliance {
        integrity: PrincipleCompliance {
            compliant: true,
            evidence: "Integrity maintained".to_string(),
            breach_details: None,
        },
        skill_care_diligence: PrincipleCompliance {
            compliant: true,
            evidence: "Skill maintained".to_string(),
            breach_details: None,
        },
        management_control: PrincipleCompliance {
            compliant: true,
            evidence: "Management adequate".to_string(),
            breach_details: None,
        },
        financial_prudence: PrincipleCompliance {
            compliant: true,
            evidence: "Financially prudent".to_string(),
            breach_details: None,
        },
        market_conduct: PrincipleCompliance {
            compliant: true,
            evidence: "Market conduct proper".to_string(),
            breach_details: None,
        },
        customers_interests: PrincipleCompliance {
            compliant: false, // BREACH
            evidence: "".to_string(),
            breach_details: Some(
                "Firm recommended high-commission products over more suitable lower-commission alternatives. Products chosen to maximize firm profit rather than customer benefit. Treating Customers Fairly (TCF) principles not embedded.".to_string()
            ),
        },
        communications: PrincipleCompliance {
            compliant: true,
            evidence: "Communications clear".to_string(),
            breach_details: None,
        },
        conflicts_of_interest: PrincipleCompliance {
            compliant: true,
            evidence: "Conflicts managed".to_string(),
            breach_details: None,
        },
        customer_trust: PrincipleCompliance {
            compliant: true,
            evidence: "Trust maintained".to_string(),
            breach_details: None,
        },
        client_assets: PrincipleCompliance {
            compliant: true,
            evidence: "Assets protected".to_string(),
            breach_details: None,
        },
        relations_with_regulators: PrincipleCompliance {
            compliant: true,
            evidence: "Cooperative with regulators".to_string(),
            breach_details: None,
        },
    };

    match validate_principles_compliance(&principles) {
        Ok(_) => println!("✓ Compliant"),
        Err(e) => {
            println!("✗ PRINCIPLE BREACH DETECTED: {}", e);
            println!("\n  Principle 6: Customers' Interests");
            println!("  'A firm must pay due regard to the interests of its customers");
            println!("   and treat them fairly.'");
            println!("\n  Breach Details:");
            println!("  Firm prioritized commission income over customer suitability");
            println!("\n  Treating Customers Fairly (TCF) Outcomes:");
            println!(
                "  1. Consumers confident they are dealing with firms where fair treatment is central"
            );
            println!("  2. Products and services marketed and sold meet customer needs");
            println!(
                "  3. Consumers provided with clear information and kept appropriately informed"
            );
            println!("  4. Advice suitability takes account of customer circumstances");
            println!("  5. Products perform as expected");
            println!(
                "  6. No unreasonable barriers to change product, switch provider, submit claim"
            );
            println!("\n  This breach represents a fundamental failure to put customers first.");
            println!("\n  FCA Action:");
            println!("  • Skilled Persons Report (Section 166)");
            println!("  • Financial penalty (unlimited)");
            println!("  • Requirement to review all recommendations");
            println!("  • Redress scheme for affected customers");
            println!("  • Public censure");
            println!("\n  Reference: PRIN 2.1.1R (Principle 6), TCF Initiative");
        }
    }
    println!();
}

/// Example 5: Breach of Principle 10 - Client assets
fn example_5_breach_principle_10_client_assets() {
    println!("Example 5: BREACH of Principle 10 - Clients' Assets (CASS)");
    println!("───────────────────────────────────────────────────────────");

    let protection = ClientAssetsProtection {
        client_money_gbp: 5_000_000.0,
        client_assets_value_gbp: 20_000_000.0,
        segregated: false, // NOT SEGREGATED - serious breach
        trust_arrangement: false,
        daily_reconciliation: false,
        cass_audit_date: None,
    };

    match validate_client_assets_protection(&protection) {
        Ok(_) => println!("✓ CASS compliant"),
        Err(e) => {
            println!("✗ SERIOUS BREACH: {}", e);
            println!("\n  Principle 10: Clients' Assets");
            println!("  'A firm must arrange adequate protection for clients' assets");
            println!("   when it is responsible for them.'");
            println!("\n  CASS 7 (Client Money Rules) BREACHED:");
            println!("  Client Money: £{:.2}", protection.client_money_gbp);
            println!(
                "  Segregated: {} ✗ (MUST be segregated)",
                protection.segregated
            );
            println!(
                "  Trust Arrangement: {} ✗ (MUST have trust)",
                protection.trust_arrangement
            );
            println!(
                "  Daily Reconciliation: {} ✗ (REQUIRED)",
                protection.daily_reconciliation
            );
            println!("\n  Requirements:");
            println!("  1. Client money MUST be segregated from firm's own money");
            println!("  2. Held in designated CLIENT BANK ACCOUNTS");
            println!("  3. Trust arrangements required (statutory trust)");
            println!("  4. DAILY reconciliation mandatory (CASS 7.15)");
            println!("  5. Client money is ring-fenced (cannot be used by firm)");
            println!("\n  Consequences of Breach:");
            println!("  • IMMEDIATE FCA intervention");
            println!("  • Potential suspension of permissions");
            println!("  • Client money may be at risk if firm fails");
            println!("  • Criminal investigation possible");
            println!("  • Directors may face personal liability");
            println!("  • FSCS protection may not apply if rules breached");
            println!("\n  If firm becomes insolvent:");
            println!("  • Properly segregated client money = returned to clients");
            println!("  • Unsegregated money = treated as firm asset (clients become creditors)");
            println!("\n  This is one of the MOST SERIOUS breaches in financial services.");
            println!("\n  Reference: PRIN 2.1.1R (Principle 10), CASS 7 (Client money rules)");
        }
    }
    println!();
}

/// Example 6: Client categorization - different protection levels
fn example_6_client_categorization() {
    println!("Example 6: Client Categorization - Protection Levels");
    println!("─────────────────────────────────────────────────────");

    let retail = ClientCategory::RetailClient;
    let professional = ClientCategory::ProfessionalClient { elective: false };
    let counterparty = ClientCategory::EligibleCounterparty;

    println!("COBS 3: Client Categorization\n");

    println!("1. RETAIL CLIENT (Highest Protection)");
    println!("   Protection Level: {}", retail.protection_level());
    println!(
        "   Suitability Assessment: {}",
        retail.requires_suitability_assessment()
    );
    println!(
        "   Appropriateness Test: {}",
        retail.requires_appropriateness_assessment()
    );
    println!("   Best Execution: {}", retail.requires_best_execution());
    println!("   • Default category for individuals and small businesses");
    println!("   • Full COBS protections apply");
    println!("   • Highest level of conduct of business obligations\n");

    println!("2. PROFESSIONAL CLIENT (Intermediate Protection)");
    println!("   Protection Level: {}", professional.protection_level());
    println!(
        "   Suitability Assessment: {}",
        professional.requires_suitability_assessment()
    );
    println!(
        "   Appropriateness Test: {}",
        professional.requires_appropriateness_assessment()
    );
    println!(
        "   Best Execution: {}",
        professional.requires_best_execution()
    );
    println!("   • Large undertakings (balance sheet €20m, turnover €40m, own funds €2m)");
    println!("   • Per se professional (authorized firms, governments, central banks)");
    println!("   • Elective professional (client requests, meets criteria)");
    println!("   • Presumed to have knowledge and experience\n");

    println!("3. ELIGIBLE COUNTERPARTY (Minimal Protection)");
    println!("   Protection Level: {}", counterparty.protection_level());
    println!(
        "   Suitability Assessment: {}",
        counterparty.requires_suitability_assessment()
    );
    println!(
        "   Appropriateness Test: {}",
        counterparty.requires_appropriateness_assessment()
    );
    println!(
        "   Best Execution: {}",
        counterparty.requires_best_execution()
    );
    println!("   • Only authorized firms, central banks, governments");
    println!("   • Wholesale market participants");
    println!("   • Minimal COBS protections (mainly PRIN applies)");
    println!("   • No suitability, appropriateness, or best execution required\n");

    println!("Client categorization determines:");
    println!("• Level of regulatory protection");
    println!("• Conduct of business obligations");
    println!("• Information requirements");
    println!("• Basis for FCA supervision\n");

    println!("Reference: COBS 3 (Client categorization)");
    println!();
}
