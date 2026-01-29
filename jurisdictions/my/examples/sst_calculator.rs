//! Example: SST (Sales and Service Tax) calculator.

use legalis_my::common::format_myr_cents;
use legalis_my::tax_law::sst::*;

fn main() {
    println!("=== Malaysian SST Calculator ===\n");

    // Sales Tax examples
    println!("--- Sales Tax (10% standard rate) ---");
    let sales1 = SalesTax::new("Electronics", 100_000); // RM 1,000
    println!(
        "Goods: {} ({})",
        sales1.goods_description,
        format_myr_cents(sales1.value_sen)
    );
    println!(
        "Sales Tax (10%): {}\n",
        format_myr_cents(sales1.calculate())
    );

    let sales2 = SalesTax::new("Certain goods", 200_000).with_rate(5.0); // RM 2,000 at 5%
    println!(
        "Goods: {} ({})",
        sales2.goods_description,
        format_myr_cents(sales2.value_sen)
    );
    println!("Sales Tax (5%): {}\n", format_myr_cents(sales2.calculate()));

    // Service Tax examples
    println!("--- Service Tax (6% standard rate) ---");
    let service1 = ServiceTax::new("Legal services", 500_000, ServiceType::Professional); // RM 5,000
    println!(
        "Service: {} ({})",
        service1.service_description,
        format_myr_cents(service1.value_sen)
    );
    println!(
        "Service Tax (6%): {}\n",
        format_myr_cents(service1.calculate())
    );

    let service2 = ServiceTax::new("Restaurant meal", 150_000, ServiceType::FoodAndBeverage); // RM 1,500
    println!(
        "Service: {} ({})",
        service2.service_description,
        format_myr_cents(service2.value_sen)
    );
    println!(
        "Service Tax (6%): {}\n",
        format_myr_cents(service2.calculate())
    );
}
