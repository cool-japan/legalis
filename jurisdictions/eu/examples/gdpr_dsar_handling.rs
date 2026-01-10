//! GDPR Data Subject Access Request (DSAR) Example
//!
//! This example demonstrates handling various data subject rights requests under Chapter III.

use legalis_eu::gdpr::*;

fn main() {
    println!("=== GDPR Data Subject Rights Request (DSAR) Example ===\n");

    // Example 1: Right of Access (Article 15)
    println!("Example 1: Right of Access (Article 15)");
    let access_request = DataSubjectRequest::new()
        .with_data_subject("john.doe@example.com")
        .with_right(DataSubjectRight::Access)
        .with_controller("E-commerce Platform Inc");

    match access_request.validate() {
        Ok(validation) => {
            println!("   ✅ Valid request");
            println!("   Right: {:?}", validation.right);
            println!("   Response deadline: {} days", validation.deadline_days);
            println!("\n   Controller must provide:");
            println!("   - Purposes of processing");
            println!("   - Categories of personal data");
            println!("   - Recipients of data");
            println!("   - Retention period");
            println!("   - Right to rectification/erasure/restriction");
            println!("   - Right to lodge complaint with supervisory authority");
            println!("   - Copy of personal data undergoing processing");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 2: Right to Erasure (Article 17)
    println!("Example 2: Right to Erasure - 'Right to be Forgotten' (Article 17)");
    let erasure_request = DataSubjectRequest::new()
        .with_data_subject("jane.smith@example.com")
        .with_right(DataSubjectRight::Erasure)
        .with_controller("Social Media Platform")
        .with_grounds("Personal data no longer necessary for the purposes collected");

    match erasure_request.validate() {
        Ok(validation) => {
            println!("   ✅ Valid request with grounds");
            println!("   Response deadline: {} days", validation.deadline_days);

            if !validation.must_comply {
                println!("\n   ⚠️  Possible exceptions to erasure obligation:");
                for exception in &validation.exceptions {
                    println!("      - {}", exception);
                }
                println!("\n   Controller must assess if any exception applies");
            } else {
                println!("   Must comply with erasure request");
            }

            println!("\n   If erasure granted:");
            println!("   - Delete personal data from all systems");
            println!("   - Inform processors to erase data");
            println!("   - If data made public, take reasonable steps to inform others");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 3: Right to Data Portability (Article 20)
    println!("Example 3: Right to Data Portability (Article 20)");
    let portability_request = DataSubjectRequest::new()
        .with_data_subject("user123@example.com")
        .with_right(DataSubjectRight::DataPortability)
        .with_controller("Cloud Storage Provider");

    match portability_request.validate() {
        Ok(validation) => {
            println!("   ✅ Valid request");
            println!("   Response deadline: {} days", validation.deadline_days);

            if !validation.exceptions.is_empty() {
                println!("\n   ⚠️  Portability restrictions:");
                for exception in &validation.exceptions {
                    println!("      - {}", exception);
                }
            }

            println!("\n   Requirements:");
            println!("   - Provide data in structured, commonly used format");
            println!("   - Machine-readable format (CSV, JSON, XML)");
            println!("   - Transmit directly to another controller if feasible");
            println!("   - Only applies to automated processing");
            println!("   - Only for consent or contract-based processing");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 4: Right to Object (Article 21)
    println!("Example 4: Right to Object to Processing (Article 21)");
    let objection_request = DataSubjectRequest::new()
        .with_data_subject("privacy.advocate@example.com")
        .with_right(DataSubjectRight::Object)
        .with_controller("Marketing Analytics Company")
        .with_grounds("Object to processing for direct marketing purposes");

    match objection_request.validate() {
        Ok(validation) => {
            println!("   ✅ Valid objection with grounds");
            println!("   Response deadline: {} days", validation.deadline_days);

            if !validation.exceptions.is_empty() {
                println!("\n   ⚠️  Controller may continue if:");
                for exception in &validation.exceptions {
                    println!("      - {}", exception);
                }
            }

            println!("\n   Special cases:");
            println!("   - Direct marketing: Must ALWAYS stop (no exceptions)");
            println!("   - Legitimate interests: Stop unless compelling grounds");
            println!("   - Public interest/official authority: Stop unless compelling grounds");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 5: Right to Rectification (Article 16)
    println!("Example 5: Right to Rectification (Article 16)");
    let rectification_request = DataSubjectRequest::new()
        .with_data_subject("correct.me@example.com")
        .with_right(DataSubjectRight::Rectification)
        .with_controller("Customer Database System");

    match rectification_request.validate() {
        Ok(validation) => {
            println!("   ✅ Valid request");
            println!("   Response deadline: {} days", validation.deadline_days);
            println!("\n   Controller must:");
            println!("   - Rectify inaccurate personal data without delay");
            println!("   - Complete incomplete personal data");
            println!("   - Inform recipients of rectification (if applicable)");
            println!("   - Inform data subject of recipients (if requested)");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 6: Invalid request (missing grounds)
    println!("Example 6: Invalid Request - Missing Required Grounds");
    let invalid_request = DataSubjectRequest::new()
        .with_data_subject("user@example.com")
        .with_right(DataSubjectRight::Erasure)
        .with_controller("Service Provider");
    // Note: No grounds provided for erasure request

    match invalid_request.validate() {
        Ok(_) => println!("   Request validated"),
        Err(e) => {
            println!("   ❌ Request validation failed: {}", e);
            println!("   Reason: Erasure requests require grounds under Article 17");
        }
    }

    println!("\n---\n");

    // Summary
    println!("=== DSAR Handling Best Practices ===");
    println!("1. Respond within 30 days (extendable by 60 days for complex requests)");
    println!("2. Verify identity of data subject before processing request");
    println!("3. Provide information free of charge (1st request)");
    println!("4. Can charge reasonable fee for repetitive/excessive requests");
    println!("5. If refusing request, explain reasons and right to complain");
    println!("6. Document all DSARs received and responses provided");
    println!("7. Train staff on DSAR handling procedures");
    println!("8. Have technical capability to search/export personal data");

    println!("\n=== End of Example ===");
}
