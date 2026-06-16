//! Companies Act 2006 — company name validation examples.
//!
//! Demonstrates the rules on the choice of a company name under the Companies Act
//! 2006, using the `legalis_uk::company` library APIs.
//!
//! The principal constraints modelled here are:
//!
//! - **Required name ending** (CA 2006 ss.58-59): a private company limited by
//!   shares must end with "Limited" (or "Ltd"); a public company must end with
//!   "public limited company" (or "plc").
//! - **Sensitive words and expressions** (CA 2006 ss.54-56): certain words (e.g.
//!   "Royal", "British", "Trust", "University", "Police") suggest a connection
//!   with government or imply a particular status and require approval.
//! - **Prohibited / permitted characters** and a minimum length (CA 2006 s.57 and
//!   the Company, LLP and Business (Names and Trading Disclosures) Regulations).
//! - **Names that are the same as an existing name** (CA 2006 s.66) — too similar
//!   to a name already on the index of company names.
//!
//! `validate_company_name` returns `Ok(())` for a permissible name, or the first
//! `CompanyLawError` encountered. This example also assembles a
//! `CompanyNameValidation` record summarising the outcome.

use legalis_uk::company::{CompanyNameValidation, CompanyType, validate_company_name};

fn main() {
    println!("=== Companies Act 2006 — Company Name Validation ===\n");

    check("Acme Trading Limited", CompanyType::PrivateLimitedByShares);
    check("Acme Trading Ltd", CompanyType::PrivateLimitedByShares);
    check("Acme Trading", CompanyType::PrivateLimitedByShares); // missing suffix
    check("Britannia Steel PLC", CompanyType::PublicLimitedCompany);
    check("Royal Acme Ltd", CompanyType::PrivateLimitedByShares); // sensitive word
    check("Acme Trust Ltd", CompanyType::PrivateLimitedByShares); // sensitive word
    check("Acme @ Home Ltd", CompanyType::PrivateLimitedByShares); // prohibited char
}

/// Validate one proposed name, print the result and build a summary record.
fn check(name: &str, company_type: CompanyType) {
    println!("Proposed name: \"{name}\"  (as {company_type:?})");
    println!("  Required ending: \"{}\"", company_type.required_suffix());

    let result = validate_company_name(name, company_type);

    let summary = match &result {
        Ok(()) => {
            println!("  => PERMISSIBLE");
            CompanyNameValidation {
                name: name.to_string(),
                valid: true,
                has_correct_suffix: true,
                contains_sensitive_words: false,
                too_similar_to_existing: false,
                contains_prohibited_words: false,
                validation_errors: Vec::new(),
            }
        }
        Err(e) => {
            println!("  => REJECTED: {e}");
            CompanyNameValidation {
                name: name.to_string(),
                valid: false,
                // Field flags are inferred from the error message for display only.
                has_correct_suffix: !e.to_string().to_lowercase().contains("suffix")
                    && !e.to_string().to_lowercase().contains("end"),
                contains_sensitive_words: e.to_string().to_lowercase().contains("sensitive"),
                too_similar_to_existing: false,
                contains_prohibited_words: e.to_string().to_lowercase().contains("prohibited"),
                validation_errors: vec![e.to_string()],
            }
        }
    };

    println!(
        "  Summary record: valid={}, errors={}\n",
        summary.valid,
        summary.validation_errors.len()
    );
}
