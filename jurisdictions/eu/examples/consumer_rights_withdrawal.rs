//! Consumer Rights Directive - Withdrawal Rights Example
//!
//! Demonstrates the 14-day right of withdrawal for distance and off-premises contracts

use chrono::{Duration, Utc};
use legalis_eu::consumer_rights::*;

fn main() {
    println!("=== Consumer Rights Directive - Withdrawal Rights ===\n");

    // Example 1: Standard 14-day withdrawal (all information provided)
    println!("Example 1: Distance Contract - Standard 14-Day Withdrawal");
    let contract1 = DistanceContract::new()
        .with_trader("Online Electronics Shop")
        .with_consumer("Alice Johnson")
        .with_contract_date(Utc::now() - Duration::days(3))
        .with_delivery_date(Utc::now() - Duration::days(2))
        .with_goods_description("Smartphone - Model X")
        .with_price_eur(599.0)
        .with_information(InformationRequirement::TotalPrice)
        .with_information(InformationRequirement::RightOfWithdrawal)
        .with_information(InformationRequirement::ModelWithdrawalForm)
        .with_withdrawal_form(true);

    let right1 = WithdrawalRight::from_distance_contract(&contract1);

    match right1.calculate_period() {
        Ok(period) => {
            println!("   ✅ Withdrawal right active");
            println!("   Start date: {}", period.start_date.format("%Y-%m-%d"));
            println!("   Deadline: {}", period.deadline.format("%Y-%m-%d"));
            println!("   Period: {} days", period.period_days);
            println!("   Days remaining: {}", period.days_remaining);
            println!("   Extended: {}", period.extended);
            println!("\n   Consumer can withdraw without giving any reason");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 2: Extended period (missing information)
    println!("Example 2: Distance Contract - Extended Period (Missing Information)");
    let contract2 = DistanceContract::new()
        .with_trader("Quick Sale Online")
        .with_consumer("Bob Smith")
        .with_contract_date(Utc::now() - Duration::days(30))
        .with_delivery_date(Utc::now() - Duration::days(28))
        .with_goods_description("Laptop")
        .with_price_eur(899.0)
        .with_withdrawal_form(false); // Missing withdrawal form!

    let right2 = WithdrawalRight::from_distance_contract(&contract2);

    match right2.calculate_period() {
        Ok(period) => {
            println!("   ⚠️  Withdrawal period EXTENDED");
            println!("   Reason: Trader failed to provide required information");
            println!("   Period: {} days (12 months)", period.period_days);
            println!("   Days remaining: {}", period.days_remaining);
            if let Some(reason) = &period.extension_reason {
                println!("   Extension reason: {}", reason);
            }
            println!("\n   Article 10: Period extended to 12 months from delivery");
            println!("   Trader violated information requirements!");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 3: Expired withdrawal period
    println!("Example 3: Distance Contract - Withdrawal Period Expired");
    let contract3 = DistanceContract::new()
        .with_trader("Fashion Store Online")
        .with_consumer("Carol Williams")
        .with_contract_date(Utc::now() - Duration::days(20))
        .with_delivery_date(Utc::now() - Duration::days(18))
        .with_goods_description("Designer Jacket")
        .with_price_eur(299.0)
        .with_withdrawal_form(true);

    let right3 = WithdrawalRight::from_distance_contract(&contract3);

    match right3.calculate_period() {
        Ok(period) => {
            println!("   ❌ Withdrawal period EXPIRED");
            println!("   Deadline was: {}", period.deadline.format("%Y-%m-%d"));
            println!("   Days past deadline: {}", period.days_remaining.abs());
            println!("\n   Consumer can no longer withdraw from contract");

            // Try to withdraw now
            let withdrawal_attempt = Utc::now();
            match right3.validate_withdrawal(withdrawal_attempt) {
                Ok(_) => println!("   Withdrawal valid"),
                Err(e) => println!("   Withdrawal rejected: {}", e),
            }
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 4: Off-premises contract
    println!("Example 4: Off-Premises Contract - Door-to-Door Sales");
    let contract4 = OffPremisesContract::new()
        .with_trader("Home Improvement Sales Rep")
        .with_consumer("David Brown")
        .with_contract_date(Utc::now() - Duration::days(5))
        .with_location("Consumer's home")
        .with_goods_description("Vacuum Cleaner")
        .with_price_eur(499.0)
        .with_information(InformationRequirement::RightOfWithdrawal)
        .with_withdrawal_form(true);

    let right4 = WithdrawalRight::from_off_premises_contract(&contract4);

    match right4.calculate_period() {
        Ok(period) => {
            println!("   ✅ Withdrawal right active");
            println!("   Contract concluded: Outside trader's business premises");
            println!("   Days remaining: {}", period.days_remaining);
            println!("\n   Same 14-day period applies to off-premises contracts");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 5: Exception - Perishable goods
    println!("Example 5: Exception - Perishable Goods (Article 17(d))");
    let contract5 = DistanceContract::new()
        .with_trader("Fresh Food Delivery")
        .with_consumer("Emma Davis")
        .with_contract_date(Utc::now() - Duration::days(1))
        .with_delivery_date(Utc::now())
        .with_goods_description("Fresh fruit basket")
        .with_price_eur(45.0)
        .with_withdrawal_form(true);

    let right5 = WithdrawalRight::from_distance_contract(&contract5)
        .with_exception(WithdrawalException::PerishableGoods);

    match right5.calculate_period() {
        Ok(_) => println!("   Unexpected: Should have exception"),
        Err(e) => {
            println!("   ❌ Withdrawal right does NOT apply");
            println!("   Error: {}", e);
            println!("\n   Article 17(d) exception:");
            println!("   Goods liable to deteriorate or expire rapidly");
            println!("   Consumer cannot withdraw from this contract");
        }
    }

    println!("\n---\n");

    // Example 6: Exception - Custom-made goods
    println!("Example 6: Exception - Custom-Made Goods (Article 17(c))");
    let contract6 = DistanceContract::new()
        .with_trader("Custom Furniture Maker")
        .with_consumer("Frank Miller")
        .with_contract_date(Utc::now() - Duration::days(7))
        .with_goods_description("Custom-built bookshelf to specific dimensions")
        .with_price_eur(1200.0)
        .with_withdrawal_form(true);

    let right6 = WithdrawalRight::from_distance_contract(&contract6)
        .with_exception(WithdrawalException::CustomMadeGoods);

    match right6.calculate_period() {
        Ok(_) => println!("   Unexpected: Should have exception"),
        Err(e) => {
            println!("   ❌ Withdrawal right does NOT apply");
            println!("   Error: {}", e);
            println!("\n   Article 17(c) exception:");
            println!("   Goods made to consumer's specifications");
            println!("   Or clearly personalized goods");
        }
    }

    println!("\n---\n");

    // Example 7: Exception - Sealed goods unsealed
    println!("Example 7: Exception - Sealed Media Unsealed (Article 17(i))");
    let contract7 = DistanceContract::new()
        .with_trader("Software Retailer")
        .with_consumer("Grace Taylor")
        .with_contract_date(Utc::now() - Duration::days(2))
        .with_goods_description("Video game (sealed)")
        .with_price_eur(59.99)
        .with_withdrawal_form(true);

    let right7 = WithdrawalRight::from_distance_contract(&contract7)
        .with_exception(WithdrawalException::SealedMediaUnsealed);

    match right7.calculate_period() {
        Ok(_) => println!("   Unexpected: Should have exception"),
        Err(e) => {
            println!("   ❌ Withdrawal right lost after unsealing");
            println!("   Error: {}", e);
            println!("\n   Article 17(i) exception:");
            println!("   Sealed audio/video recordings or software");
            println!("   Exception applies AFTER consumer unseals the product");
            println!("   (Right exists until unsealing)");
        }
    }

    println!("\n---\n");

    // Summary
    println!("=== Consumer Rights Directive Key Points ===");
    println!("\n1. Right of Withdrawal (Articles 9-16):");
    println!("   - 14 days from receipt of goods (distance contracts)");
    println!("   - 14 days from contract conclusion (off-premises contracts)");
    println!("   - No reason required");
    println!("   - Free of charge (except return costs)");
    println!("\n2. Information Requirements (Article 6):");
    println!("   - Trader must provide model withdrawal form");
    println!("   - If not provided: period extends to 12 months");
    println!("   - Consumer must be informed of right to withdraw");
    println!("\n3. Exceptions to Withdrawal Right (Article 17):");
    println!("   - Perishable goods");
    println!("   - Custom-made/personalized goods");
    println!("   - Sealed goods unsealed (hygiene, media)");
    println!("   - Services fully performed (with consent)");
    println!("   - Accommodation/transport with specific dates");
    println!("   - Digital content delivered (with consent)");
    println!("\n4. Effects of Withdrawal (Article 16):");
    println!("   - Consumer returns goods within 14 days");
    println!("   - Trader refunds within 14 days of withdrawal notice");
    println!("   - Trader can withhold refund until goods returned");

    println!("\n=== End of Example ===");
}
