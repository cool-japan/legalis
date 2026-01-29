//! Example: Contract validation under Contracts Act 1950.

use legalis_my::contract_law::*;

fn main() {
    println!("=== Contract Validation Example ===\n");

    // Valid contract
    println!("--- Valid Service Agreement ---");
    let party1 = Party::new("Ahmad bin Ali", "850123-01-5678", PartyType::Individual);
    let party2 = Party::new(
        "Tech Innovations Sdn Bhd",
        "201601012345",
        PartyType::Company,
    );
    let consideration = Consideration::new("Software development services").with_value_sen(5000000); // RM 50,000 (in sen)

    let contract = Contract::builder()
        .contract_type(ContractType::ServiceAgreement)
        .add_party(party1)
        .add_party(party2)
        .consideration(consideration)
        .add_term("Deliver software within 6 months")
        .add_term("Payment upon completion")
        .free_consent(true)
        .lawful_object(true)
        .build()
        .expect("Valid contract");

    println!("Contract Type: {:?}", contract.contract_type);
    println!("Parties: {}", contract.parties.len());
    println!("Terms: {}", contract.terms.len());

    match contract.validate() {
        Ok(report) => {
            if report.valid {
                println!("\n✅ Contract is valid under Contracts Act 1950");
            } else {
                println!("\n❌ Contract has issues:");
                for issue in report.issues {
                    println!("  - {}", issue);
                }
                println!("Status: {:?}", report.status);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Voidable contract (no free consent)
    println!("\n--- Voidable Contract (Consent Not Free) ---");
    let party3 = Party::new("Victim", "900101-02-1234", PartyType::Individual);
    let party4 = Party::new("Company", "201501012345", PartyType::Company);
    let consideration2 = Consideration::new("Services");

    let contract2 = Contract::builder()
        .contract_type(ContractType::ServiceAgreement)
        .add_party(party3)
        .add_party(party4)
        .consideration(consideration2)
        .free_consent(false) // Consent vitiated by coercion/fraud
        .lawful_object(true)
        .build()
        .expect("Contract built");

    match contract2.validate() {
        Ok(report) => {
            println!("Valid: {}", report.valid);
            println!("Status: {:?}", report.status);
            for issue in report.issues {
                println!("  - {}", issue);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
