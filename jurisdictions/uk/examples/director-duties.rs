//! Director Duties Examples (Companies Act 2006 ss.171-177)
//!
//! Demonstrates validation of the seven statutory director duties under CA 2006.

use chrono::NaiveDate;
use legalis_uk::company::*;

fn main() {
    println!("=== UK Company Law: Seven Statutory Director Duties (CA 2006 ss.171-177) ===\n");

    example_1_all_duties_compliant();
    example_2_breach_s172_promote_success();
    example_3_breach_s174_reasonable_care();
    example_4_breach_s175_conflicts();
    example_5_breach_s177_declare_interest();
}

/// Example 1: All seven duties compliant
fn example_1_all_duties_compliant() {
    println!("Example 1: All Seven Duties Compliant");
    println!("──────────────────────────────────────");

    let duties = DirectorDutiesCompliance {
        // s.171: Act within powers
        act_within_powers: DutyCompliance {
            compliant: true,
            evidence: "Director acted within powers granted by articles of association. All decisions within board authority.".to_string(),
            breach_details: None,
        },

        // s.172: Promote success of company (6 considerations)
        promote_success: PromoteSuccessCompliance {
            compliant: true,
            long_term_consequences_considered: true,
            employee_interests_considered: true,
            business_relationships_considered: true,
            community_environment_considered: true,
            reputation_considered: true,
            fairness_between_members_considered: true,
            evidence: "Board minutes show consideration of: long-term strategy, employee welfare programs, supplier relationships, environmental impact assessment, brand reputation, and equitable treatment of shareholders.".to_string(),
        },

        // s.173: Independent judgment
        independent_judgment: DutyCompliance {
            compliant: true,
            evidence: "Director exercised independent judgment in all decisions. No improper delegation or fettering of discretion.".to_string(),
            breach_details: None,
        },

        // s.174: Reasonable care, skill and diligence
        reasonable_care: ReasonableCareCompliance {
            compliant: true,
            objective_standard_met: true,
            subjective_standard_met: true,
            evidence: "Director has relevant qualifications (MBA, ACCA), attended all board meetings, reviewed financial reports, sought professional advice where appropriate.".to_string(),
        },

        // s.175: Avoid conflicts of interest
        avoid_conflicts: ConflictsCompliance {
            compliant: true,
            conflicts_declared: vec![],
            board_authorization_obtained: true,
        },

        // s.176: Not accept benefits from third parties
        no_third_party_benefits: DutyCompliance {
            compliant: true,
            evidence: "No benefits accepted from third parties. All remuneration approved by company in accordance with articles.".to_string(),
            breach_details: None,
        },

        // s.177: Declare interest in proposed transaction
        declare_interest: DeclareInterestCompliance {
            compliant: true,
            interests_declared: vec![],
        },
    };

    match validate_director_duties(&duties) {
        Ok(_) => {
            println!("✓ All seven statutory duties COMPLIANT");
            println!("  s.171: Act within powers ✓");
            println!("  s.172: Promote success of company ✓");
            println!("  s.173: Exercise independent judgment ✓");
            println!("  s.174: Reasonable care, skill and diligence ✓");
            println!("  s.175: Avoid conflicts of interest ✓");
            println!("  s.176: Not accept benefits from third parties ✓");
            println!("  s.177: Declare interest in proposed transaction ✓");
            println!("\n  Reference: CA 2006 Part 10, Chapter 2 (ss.171-177)");
        }
        Err(e) => println!("✗ Duty breach: {}", e),
    }
    println!();
}

/// Example 2: Breach of s.172 - Failed to promote success of company
fn example_2_breach_s172_promote_success() {
    println!("Example 2: Breach of s.172 - Promote Success of Company");
    println!("────────────────────────────────────────────────────────");

    let duties = DirectorDutiesCompliance {
        act_within_powers: DutyCompliance {
            compliant: true,
            evidence: "Within powers".to_string(),
            breach_details: None,
        },
        promote_success: PromoteSuccessCompliance {
            compliant: false, // BREACH
            long_term_consequences_considered: false, // Failed
            employee_interests_considered: false,     // Failed
            business_relationships_considered: true,
            community_environment_considered: false, // Failed
            reputation_considered: true,
            fairness_between_members_considered: true,
            evidence: "Decision to relocate production overseas considered only short-term cost savings. No consideration of long-term employee impact or community effects.".to_string(),
        },
        independent_judgment: DutyCompliance {
            compliant: true,
            evidence: "Independent".to_string(),
            breach_details: None,
        },
        reasonable_care: ReasonableCareCompliance {
            compliant: true,
            objective_standard_met: true,
            subjective_standard_met: true,
            evidence: "Reasonable care".to_string(),
        },
        avoid_conflicts: ConflictsCompliance {
            compliant: true,
            conflicts_declared: vec![],
            board_authorization_obtained: true,
        },
        no_third_party_benefits: DutyCompliance {
            compliant: true,
            evidence: "No benefits".to_string(),
            breach_details: None,
        },
        declare_interest: DeclareInterestCompliance {
            compliant: true,
            interests_declared: vec![],
        },
    };

    match validate_director_duties(&duties) {
        Ok(_) => println!("✓ Duties compliant"),
        Err(e) => {
            println!("✗ BREACH DETECTED: {}", e);
            println!("\n  Failed Considerations:");
            println!("  • Long-term consequences NOT considered");
            println!("  • Employee interests NOT considered");
            println!("  • Community and environment NOT considered");
            println!("\n  s.172 Requirements:");
            println!("  Directors must have regard to:");
            println!("  (a) Likely consequences in the long term");
            println!("  (b) Interests of company's employees");
            println!("  (c) Business relationships with suppliers, customers, others");
            println!("  (d) Impact on community and environment");
            println!("  (e) Desirability of maintaining high standards reputation");
            println!("  (f) Need to act fairly between members");
            println!(
                "\n  This is 'enlightened shareholder value' - considering wider stakeholders"
            );
            println!("  while promoting shareholder value.");
            println!("\n  Reference: CA 2006 s.172");
        }
    }
    println!();
}

/// Example 3: Breach of s.174 - Reasonable care, skill and diligence
fn example_3_breach_s174_reasonable_care() {
    println!("Example 3: Breach of s.174 - Reasonable Care, Skill and Diligence");
    println!("──────────────────────────────────────────────────────────────────");

    let duties = DirectorDutiesCompliance {
        act_within_powers: DutyCompliance {
            compliant: true,
            evidence: "Within powers".to_string(),
            breach_details: None,
        },
        promote_success: PromoteSuccessCompliance {
            compliant: true,
            long_term_consequences_considered: true,
            employee_interests_considered: true,
            business_relationships_considered: true,
            community_environment_considered: true,
            reputation_considered: true,
            fairness_between_members_considered: true,
            evidence: "All factors considered".to_string(),
        },
        independent_judgment: DutyCompliance {
            compliant: true,
            evidence: "Independent".to_string(),
            breach_details: None,
        },
        reasonable_care: ReasonableCareCompliance {
            compliant: false, // BREACH
            objective_standard_met: false, // Failed objective test
            subjective_standard_met: true,
            evidence: "Director failed to attend board meetings (attended 2 of 12). Did not review financial statements before approval. No professional advice sought for major acquisition.".to_string(),
        },
        avoid_conflicts: ConflictsCompliance {
            compliant: true,
            conflicts_declared: vec![],
            board_authorization_obtained: true,
        },
        no_third_party_benefits: DutyCompliance {
            compliant: true,
            evidence: "No benefits".to_string(),
            breach_details: None,
        },
        declare_interest: DeclareInterestCompliance {
            compliant: true,
            interests_declared: vec![],
        },
    };

    match validate_director_duties(&duties) {
        Ok(_) => println!("✓ Duties compliant"),
        Err(e) => {
            println!("✗ BREACH DETECTED: {}", e);
            println!("\n  s.174 Dual Standard Test:");
            println!("  1. OBJECTIVE TEST: Care reasonably expected from person in that position");
            println!("     → Failed: Poor attendance, inadequate preparation");
            println!(
                "  2. SUBJECTIVE TEST: Care expected given director's actual knowledge/experience"
            );
            println!("     → Met: But objective standard is minimum");
            println!("\n  A director must meet BOTH standards. The higher of the two applies.");
            println!("\n  Examples of breach:");
            println!("  • Non-attendance at board meetings");
            println!("  • Failure to read board papers");
            println!("  • Rubber-stamping decisions without inquiry");
            println!("  • Failing to seek professional advice when needed");
            println!("\n  Reference: CA 2006 s.174");
        }
    }
    println!();
}

/// Example 4: Breach of s.175 - Conflicts of interest
fn example_4_breach_s175_conflicts() {
    println!("Example 4: Breach of s.175 - Conflicts of Interest");
    println!("───────────────────────────────────────────────────");

    let duties = DirectorDutiesCompliance {
        act_within_powers: DutyCompliance {
            compliant: true,
            evidence: "Within powers".to_string(),
            breach_details: None,
        },
        promote_success: PromoteSuccessCompliance {
            compliant: true,
            long_term_consequences_considered: true,
            employee_interests_considered: true,
            business_relationships_considered: true,
            community_environment_considered: true,
            reputation_considered: true,
            fairness_between_members_considered: true,
            evidence: "All factors considered".to_string(),
        },
        independent_judgment: DutyCompliance {
            compliant: true,
            evidence: "Independent".to_string(),
            breach_details: None,
        },
        reasonable_care: ReasonableCareCompliance {
            compliant: true,
            objective_standard_met: true,
            subjective_standard_met: true,
            evidence: "Reasonable care".to_string(),
        },
        avoid_conflicts: ConflictsCompliance {
            compliant: false, // BREACH
            conflicts_declared: vec![
                ConflictOfInterest {
                    nature_of_conflict: "Director set up competing business using company's customer list and trade secrets".to_string(),
                    date_declared: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                    authorization_obtained: false, // NOT AUTHORIZED
                    authorization_reference: None,
                },
                ConflictOfInterest {
                    nature_of_conflict: "Director took corporate opportunity for personal benefit".to_string(),
                    date_declared: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                    authorization_obtained: false, // NOT AUTHORIZED
                    authorization_reference: None,
                },
            ],
            board_authorization_obtained: false,
        },
        no_third_party_benefits: DutyCompliance {
            compliant: true,
            evidence: "No benefits".to_string(),
            breach_details: None,
        },
        declare_interest: DeclareInterestCompliance {
            compliant: true,
            interests_declared: vec![],
        },
    };

    match validate_director_duties(&duties) {
        Ok(_) => println!("✓ Duties compliant"),
        Err(e) => {
            println!("✗ BREACH DETECTED: {}", e);
            println!("\n  Conflicts Declared but NOT Authorized:");
            println!("  1. Set up competing business (exploitation of company information)");
            println!("  2. Took corporate opportunity for personal benefit");
            println!("\n  s.175 Requirements:");
            println!("  Director must avoid situations where they have (or can have) a direct");
            println!("  or indirect interest that conflicts (or possibly may conflict) with");
            println!("  company interests.");
            println!("\n  This applies to exploitation of:");
            println!("  • Property, information, or opportunity");
            println!("  • Whether or not the company could take advantage of it");
            println!("\n  Authorization:");
            println!("  Conflict situations may be authorized by independent directors");
            println!("  (if permitted by articles of association).");
            println!("\n  Classic examples:");
            println!("  • Competing business");
            println!("  • Taking corporate opportunities");
            println!("  • Using confidential information");
            println!("\n  Reference: CA 2006 s.175");
        }
    }
    println!();
}

/// Example 5: Breach of s.177 - Failure to declare interest
fn example_5_breach_s177_declare_interest() {
    println!("Example 5: Breach of s.177 - Declare Interest in Proposed Transaction");
    println!("───────────────────────────────────────────────────────────────────────");

    let duties = DirectorDutiesCompliance {
        act_within_powers: DutyCompliance {
            compliant: true,
            evidence: "Within powers".to_string(),
            breach_details: None,
        },
        promote_success: PromoteSuccessCompliance {
            compliant: true,
            long_term_consequences_considered: true,
            employee_interests_considered: true,
            business_relationships_considered: true,
            community_environment_considered: true,
            reputation_considered: true,
            fairness_between_members_considered: true,
            evidence: "All factors considered".to_string(),
        },
        independent_judgment: DutyCompliance {
            compliant: true,
            evidence: "Independent".to_string(),
            breach_details: None,
        },
        reasonable_care: ReasonableCareCompliance {
            compliant: true,
            objective_standard_met: true,
            subjective_standard_met: true,
            evidence: "Reasonable care".to_string(),
        },
        avoid_conflicts: ConflictsCompliance {
            compliant: true,
            conflicts_declared: vec![],
            board_authorization_obtained: true,
        },
        no_third_party_benefits: DutyCompliance {
            compliant: true,
            evidence: "No benefits".to_string(),
            breach_details: None,
        },
        declare_interest: DeclareInterestCompliance {
            compliant: false, // BREACH
            interests_declared: vec![
                InterestDeclaration {
                    transaction_description:
                        "Purchase of office premises from company owned by director's spouse"
                            .to_string(),
                    nature_of_interest: "Indirect interest through spouse's ownership".to_string(),
                    date_declared: NaiveDate::from_ymd_opt(2024, 5, 10).unwrap(),
                    declared_to_board: false, // NOT DECLARED
                },
                InterestDeclaration {
                    transaction_description:
                        "Service contract with supplier where director is shareholder".to_string(),
                    nature_of_interest: "10% shareholding in supplier company".to_string(),
                    date_declared: NaiveDate::from_ymd_opt(2024, 5, 15).unwrap(),
                    declared_to_board: false, // NOT DECLARED
                },
            ],
        },
    };

    match validate_director_duties(&duties) {
        Ok(_) => println!("✓ Duties compliant"),
        Err(e) => {
            println!("✗ BREACH DETECTED: {}", e);
            println!("\n  Interests NOT Declared to Board:");
            println!("  1. Property transaction - indirect interest through spouse");
            println!("  2. Service contract - director is shareholder in supplier");
            println!("\n  s.177 Requirements:");
            println!("  If director is in any way, directly or indirectly, interested in a");
            println!("  proposed transaction or arrangement with the company, they MUST declare");
            println!("  the nature and extent of that interest to the other directors.");
            println!("\n  Declaration must be made:");
            println!("  • BEFORE the company enters into the transaction/arrangement");
            println!("  • At a meeting of directors, or");
            println!("  • By notice to directors (written or general notice)");
            println!("\n  'Interested' includes:");
            println!("  • Direct interest (director is party)");
            println!("  • Indirect interest (through connected person - spouse, children, etc.)");
            println!("\n  Criminal offence if not declared (CA 2006 s.183)");
            println!("\n  Note: s.177 applies to PROPOSED transactions");
            println!("        s.182 applies to EXISTING transactions");
            println!("\n  Reference: CA 2006 s.177");
        }
    }
    println!();
}
