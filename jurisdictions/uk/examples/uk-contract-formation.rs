//! Contract Formation Examples
//!
//! Demonstrates English common law contract formation principles through
//! case law scenarios.
//!
//! Essential elements:
//! 1. Offer
//! 2. Acceptance (mirror image rule)
//! 3. Consideration
//! 4. Intention to create legal relations
//! 5. Capacity

use chrono::Utc;
use legalis_uk::contract::*;

fn main() {
    println!("=== English Contract Law - Formation Examples ===\n");
    println!("Common law contract formation with case law integration\n");
    println!("====================================================\n");

    // Example 1: Valid contract formation (commercial)
    example_1_valid_commercial_contract();

    // Example 2: Mirror image rule violated (Hyde v Wrench)
    example_2_mirror_image_rule();

    // Example 3: Past consideration invalid (Re McArdle)
    example_3_past_consideration();

    // Example 4: Domestic agreement - no intention (Balfour v Balfour)
    example_4_domestic_agreement();

    // Example 5: Invitation to treat vs offer (Pharmaceutical Society v Boots)
    example_5_invitation_to_treat();

    // Example 6: Postal rule (Adams v Lindsell)
    example_6_postal_rule();

    // Example 7: Breach of condition vs warranty
    example_7_breach_classification();
}

fn example_1_valid_commercial_contract() {
    println!("Example 1: Valid Commercial Contract Formation");
    println!("===============================================\n");

    let offer = Offer {
        offeror: Party {
            name: "Acme Ltd".to_string(),
            party_type: PartyType::Company,
            age: None,
        },
        offeree: Party {
            name: "Beta Corp".to_string(),
            party_type: PartyType::Company,
            age: None,
        },
        terms: vec![
            "Supply 1000 widgets".to_string(),
            "Price: £10,000".to_string(),
            "Delivery: 30 days".to_string(),
        ],
        offer_date: Utc::now(),
        expiry_date: None,
        still_open: true,
        offer_type: OfferType::Bilateral,
    };

    let acceptance = Acceptance {
        acceptance_date: Utc::now(),
        method: AcceptanceMethod::Written,
        unqualified: true,
        modifications: vec![], // Mirror image rule satisfied
    };

    let consideration = Consideration {
        description: "Payment of £10,000 for widgets".to_string(),
        provided_by: Party {
            name: "Beta Corp".to_string(),
            party_type: PartyType::Company,
            age: None,
        },
        consideration_type: ConsiderationType::Money,
        sufficient: true,
        is_past: false,
    };

    let intention = IntentionToCreateLegalRelations {
        context: AgreementContext::Commercial,
        presumption: IntentionPresumption::IntentionPresumed,
        rebuttal_evidence: vec![],
        intention_exists: true,
    };

    let capacity = ContractualCapacity {
        party: Party {
            name: "Beta Corp".to_string(),
            party_type: PartyType::Company,
            age: None,
        },
        incapacity: None,
        has_capacity: true,
    };

    let formation = ContractFormation {
        offer: offer.clone(),
        acceptance: Some(acceptance),
        consideration,
        intention,
        capacity,
        is_formed: true,
    };

    println!("Scenario: Commercial contract between two companies");
    println!();
    println!("Offer:");
    println!("  Offeror: {}", formation.offer.offeror.name);
    println!("  Offeree: {}", formation.offer.offeree.name);
    for term in &formation.offer.terms {
        println!("  Term: {}", term);
    }
    println!();

    match validate_contract_formation(&formation) {
        Ok(()) => {
            println!("✅ VALID CONTRACT FORMED");
            println!();
            println!("All five elements satisfied:");
            println!("  1. ✓ Offer (definite and certain)");
            println!("  2. ✓ Acceptance (unqualified, mirror image rule)");
            println!("  3. ✓ Consideration (payment for goods)");
            println!("  4. ✓ Intention (commercial context - Esso v Commissioners [1976])");
            println!("  5. ✓ Capacity (both companies have capacity)");
            println!();
            println!("Legal Effect: Binding contract enforceable in law");
            println!();
        }
        Err(e) => {
            println!("❌ Contract formation failed: {}\n", e);
        }
    }
}

fn example_2_mirror_image_rule() {
    println!("Example 2: Mirror Image Rule - Counter-Offer");
    println!("==============================================\n");
    println!("Case Law: Hyde v Wrench [1840]");
    println!("Rule: Counter-offer destroys original offer\n");

    let offer = Offer {
        offeror: Party {
            name: "Mr. Wrench".to_string(),
            party_type: PartyType::Individual,
            age: Some(45),
        },
        offeree: Party {
            name: "Mr. Hyde".to_string(),
            party_type: PartyType::Individual,
            age: Some(35),
        },
        terms: vec!["Sell farm for £1,000".to_string()],
        offer_date: Utc::now() - chrono::Duration::days(5),
        expiry_date: None,
        still_open: true,
        offer_type: OfferType::Bilateral,
    };

    let counter_offer = Acceptance {
        acceptance_date: Utc::now() - chrono::Duration::days(3),
        method: AcceptanceMethod::Written,
        unqualified: false,
        modifications: vec!["I accept but will pay £950".to_string()],
    };

    println!("Timeline:");
    println!("  Day 1: Wrench offers to sell farm for £1,000");
    println!("  Day 3: Hyde responds: 'I'll pay £950'");
    println!("  Day 5: Hyde changes mind: 'I'll pay £1,000'");
    println!();

    println!("Legal Analysis:");
    println!();

    match validate_acceptance(&counter_offer, &offer) {
        Ok(()) => {
            println!("✅ Valid acceptance");
        }
        Err(e) => {
            println!("❌ ACCEPTANCE INVALID\n");
            println!("{}\n", e);
            println!("Consequence:");
            println!("  • Hyde's response (£950) is a COUNTER-OFFER, not acceptance");
            println!("  • Original offer (£1,000) is DESTROYED");
            println!("  • Hyde cannot later accept original offer");
            println!("  • New offer required from Wrench");
            println!();
        }
    }

    println!("Mirror Image Rule:");
    println!("  Acceptance must match offer EXACTLY");
    println!("  Any modification = counter-offer");
    println!("  Counter-offer destroys original offer");
    println!();
}

fn example_3_past_consideration() {
    println!("Example 3: Past Consideration Invalid");
    println!("======================================\n");
    println!("Case Law: Re McArdle [1951]");
    println!("Rule: Past consideration is no consideration\n");

    println!("Scenario:");
    println!("  1. Mrs. McArdle improves house (August)");
    println!("  2. Family promises to pay £488 (September)");
    println!("  3. Mrs. McArdle claims payment");
    println!();

    let past_consideration = Consideration {
        description: "Improvements to house completed last month".to_string(),
        provided_by: Party {
            name: "Mrs. McArdle".to_string(),
            party_type: PartyType::Individual,
            age: Some(42),
        },
        consideration_type: ConsiderationType::Act,
        sufficient: true,
        is_past: true, // Already performed before promise
    };

    println!("Consideration Analysis:");
    println!();

    match validate_consideration(&past_consideration) {
        Ok(()) => {
            println!("✅ Valid consideration");
        }
        Err(e) => {
            println!("❌ CONSIDERATION INVALID\n");
            println!("{}\n", e);
            println!("Timeline Problem:");
            println!("  August:    Work completed (consideration provided)");
            println!("  September: Promise to pay made");
            println!("  ❌ Consideration already PAST when promise made");
            println!();
            println!("Rule: Consideration must be given in exchange for promise");
            println!("      Not for something already done");
            println!();
            println!("Result: Promise not enforceable (no valid consideration)");
            println!();
        }
    }

    println!("Exception (Lampleigh v Brathwait [1615]):");
    println!("  Past consideration valid if:");
    println!("  1. Act done at promisor's request, AND");
    println!("  2. Parties understood payment would be made, AND");
    println!("  3. Promise enforceable if made beforehand");
    println!();
}

fn example_4_domestic_agreement() {
    println!("Example 4: Domestic Agreement - No Intention");
    println!("==============================================\n");
    println!("Case Law: Balfour v Balfour [1919]");
    println!("Rule: Domestic agreements presumed NOT to create legal relations\n");

    println!("Scenario:");
    println!("  Husband working abroad promises wife £30/month housekeeping");
    println!("  Later separates and stops paying");
    println!("  Wife sues for breach of contract");
    println!();

    let domestic_intention = IntentionToCreateLegalRelations {
        context: AgreementContext::Domestic,
        presumption: IntentionPresumption::NoIntentionPresumed,
        rebuttal_evidence: vec![],
        intention_exists: false,
    };

    println!("Intention Analysis:");
    println!();

    match validate_intention(&domestic_intention) {
        Ok(()) => {
            println!("✅ Intention exists");
        }
        Err(e) => {
            println!("❌ NO INTENTION TO CREATE LEGAL RELATIONS\n");
            println!("{}\n", e);
            println!("Court Reasoning (Atkin LJ):");
            println!("  'Agreements such as these are outside the realm of");
            println!("   contracts altogether... They are not contracts");
            println!("   because the parties did not intend that they should");
            println!("   be attended by legal consequences.'");
            println!();
            println!("Presumption:");
            println!("  Domestic/social agreements → NO intention presumed");
            println!("  Commercial agreements → Intention IS presumed");
            println!();
            println!("Result: No binding contract (no legal recourse)");
            println!();
        }
    }

    println!("Contrast: Merritt v Merritt [1970]");
    println!("  Separated spouses in writing → Rebuts presumption");
    println!("  Legal relations intended when relationship broken down");
    println!();
}

fn example_5_invitation_to_treat() {
    println!("Example 5: Invitation to Treat vs Offer");
    println!("========================================\n");
    println!("Case Law: Pharmaceutical Society v Boots [1953]");
    println!("Rule: Goods displayed in shop are invitation to treat, not offer\n");

    println!("Scenario: Pharmacy displays goods on self-service shelves");
    println!("Question: At what point is contract formed?");
    println!();

    let invitation = Offer {
        offeror: Party {
            name: "Boots Pharmacy".to_string(),
            party_type: PartyType::Company,
            age: None,
        },
        offeree: Party {
            name: "Customer".to_string(),
            party_type: PartyType::Individual,
            age: Some(30),
        },
        terms: vec!["Aspirin on shelf for £2.99".to_string()],
        offer_date: Utc::now(),
        expiry_date: None,
        still_open: true,
        offer_type: OfferType::InvitationToTreat, // Not an offer!
    };

    println!("Legal Analysis:");
    println!();

    match validate_offer(&invitation) {
        Ok(()) => {
            println!("✅ Valid offer");
        }
        Err(e) => {
            println!("❌ NOT AN OFFER\n");
            println!("{}\n", e);
            println!("Correct Analysis:");
            println!("  1. Goods on shelf = INVITATION TO TREAT");
            println!("  2. Customer takes to till = OFFER to buy");
            println!("  3. Cashier accepts = ACCEPTANCE");
            println!("  4. Contract formed at TILL, not shelf");
            println!();
            println!("Why this matters:");
            println!("  • Shop can refuse to sell (e.g., age-restricted goods)");
            println!("  • Shop not bound by displayed price if error");
            println!("  • Pharmacist must supervise sale (satisfied at till)");
            println!();
        }
    }

    println!("Other Invitations to Treat:");
    println!("  • Shop window displays (Fisher v Bell [1961])");
    println!("  • Advertisements (Partridge v Crittenden [1968])");
    println!("  • Auction (bidder makes offer, auctioneer accepts)");
    println!();

    println!("Exception - Unilateral Contracts:");
    println!("  Carlill v Carbolic Smoke Ball Co [1893]");
    println!("  Advertisement can be offer if shows intention to be bound");
    println!();
}

fn example_6_postal_rule() {
    println!("Example 6: Postal Rule");
    println!("======================\n");
    println!("Case Law: Adams v Lindsell [1818]");
    println!("Rule: Acceptance complete when letter posted, not when received\n");

    println!("Scenario:");
    println!("  Monday:    Seller posts offer to buyer");
    println!("  Wednesday: Buyer receives offer, posts acceptance");
    println!("  Thursday:  Seller changes mind, sends revocation");
    println!("  Friday:    Seller receives acceptance");
    println!("  Saturday:  Buyer receives revocation");
    println!();

    let postal_acceptance = Acceptance {
        acceptance_date: Utc::now() - chrono::Duration::days(2), // Wednesday
        method: AcceptanceMethod::Post,
        unqualified: true,
        modifications: vec![],
    };

    println!("Question: Is there a binding contract?");
    println!();

    println!("✅ YES - Contract formed on WEDNESDAY\n");
    println!("Postal Rule (Adams v Lindsell [1818]):");
    println!("  • Acceptance complete when letter POSTED");
    println!("  • Not when received");
    println!("  • Even if letter lost in post!");
    println!();

    println!("Timeline Analysis:");
    println!("  Wednesday: Acceptance posted → CONTRACT FORMED ✓");
    println!("  Thursday:  Revocation sent → TOO LATE (contract exists)");
    println!("  Friday:    Seller receives acceptance → Already bound");
    println!();

    println!("Method: {:?}", postal_acceptance.method);
    println!("  Postal rule applies automatically to postal acceptance");
    println!();

    println!("Exceptions to Postal Rule:");
    println!("  1. Offer expressly states acceptance must be received");
    println!("  2. Application would produce absurd result");
    println!("  3. Instantaneous communication (email, fax, telex)");
    println!();

    println!("Modern Application:");
    println!("  • Email: NOT postal rule (Entores v Miles Far East [1955])");
    println!("  • Text/SMS: Likely instantaneous communication");
    println!("  • Letter: Postal rule still applies");
    println!();
}

fn example_7_breach_classification() {
    println!("Example 7: Breach Classification - Condition vs Warranty");
    println!("=========================================================\n");

    println!("Case A: Breach of Condition (Essential Term)");
    println!("---------------------------------------------");
    println!("Case Law: Poussard v Spiers [1876]\n");

    println!("Facts:");
    println!("  Opera singer (Poussard) ill, misses opening performances");
    println!("  Producer (Spiers) hires replacement, dismisses Poussard");
    println!();

    let condition_term = ContractTerm {
        text: "Perform from opening night onwards".to_string(),
        classification: TermClassification::Condition,
        term_source: TermSource::Express,
    };

    let condition_breach = ContractBreach {
        breaching_party: Party {
            name: "Poussard".to_string(),
            party_type: PartyType::Individual,
            age: Some(35),
        },
        term_breached: condition_term,
        breach_type: BreachType::Fundamental,
        breach_date: Utc::now(),
        description: "Unable to perform opening night".to_string(),
    };

    match validate_breach(&condition_breach) {
        Err(ContractError::BreachOfCondition { term, .. }) => {
            println!("Classification: CONDITION (essential term)");
            println!("Term: {}", term);
            println!();
            println!("✅ Remedies Available:");
            println!("  1. TERMINATE contract (treat as discharged)");
            println!("  2. Claim DAMAGES for losses");
            println!();
            println!("Held: Spiers entitled to terminate");
            println!("      Opening performances were essence of contract");
            println!();
        }
        _ => {}
    }

    println!("─────────────────────────────────────────────────────────\n");

    println!("Case B: Breach of Warranty (Minor Term)");
    println!("----------------------------------------");
    println!("Case Law: Bettini v Gye [1876]\n");

    println!("Facts:");
    println!("  Opera singer (Bettini) arrives late for rehearsals (not performances)");
    println!("  Contract required attendance 6 days before opening");
    println!("  Arrived only 3 days before");
    println!();

    let warranty_term = ContractTerm {
        text: "Attend rehearsals 6 days before opening".to_string(),
        classification: TermClassification::Warranty,
        term_source: TermSource::Express,
    };

    let warranty_breach = ContractBreach {
        breaching_party: Party {
            name: "Bettini".to_string(),
            party_type: PartyType::Individual,
            age: Some(40),
        },
        term_breached: warranty_term,
        breach_type: BreachType::Minor,
        breach_date: Utc::now(),
        description: "Missed first 3 days of rehearsals".to_string(),
    };

    match validate_breach(&warranty_breach) {
        Err(ContractError::BreachOfWarranty { term, .. }) => {
            println!("Classification: WARRANTY (minor term)");
            println!("Term: {}", term);
            println!();
            println!("❌ Limited Remedy:");
            println!("  • DAMAGES only (compensation for loss)");
            println!("  • CANNOT terminate contract");
            println!();
            println!("Held: Gye NOT entitled to terminate");
            println!("      Rehearsals not fundamental to contract");
            println!("      (Performances were the essence)");
            println!();
        }
        _ => {}
    }

    println!("Innominate Terms (Hong Kong Fir Shipping [1962]):");
    println!("  • Some terms neither clearly condition nor warranty");
    println!("  • Classification depends on CONSEQUENCES of breach");
    println!("  • Serious consequences → treat as condition");
    println!("  • Minor consequences → treat as warranty");
    println!();
}
