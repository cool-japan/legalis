//! Example: Company formation (SA and SRL)

use legalis_mx::company_law::*;

fn main() {
    println!("=== Mexican Company Formation ===\n");

    // Create SA (Stock Corporation)
    println!("Creating SA (Sociedad Anónima):");
    match StockCorporation::new(
        "Innovación Tecnológica SA de CV".to_string(),
        "Desarrollo de software y consultoría".to_string(),
        10_000_000, // 100,000 pesos (in cents)
        5,
    ) {
        Ok(sa) => {
            println!("  ✓ SA created successfully");
            println!("  Company: {}", sa.denominacion);
            println!("  Capital: ${}.00 MXN", sa.capital_social / 100);
            println!("  Shareholders: {}", sa.num_accionistas);
            println!(
                "  (Minimum capital: ${}.00 MXN)\n",
                StockCorporation::MINIMUM_CAPITAL / 100
            );
        }
        Err(e) => println!("  ✗ Error: {}\n", e),
    }

    // Create SRL (Limited Liability Company)
    println!("Creating SRL (Sociedad de Responsabilidad Limitada):");
    match LimitedLiabilityCompany::new(
        "Servicios Profesionales SRL de CV".to_string(),
        "Consultoría y asesoría empresarial".to_string(),
        7_500_000, // 75,000 pesos (in cents)
        3,
    ) {
        Ok(srl) => {
            println!("  ✓ SRL created successfully");
            println!("  Company: {}", srl.razon_social);
            println!("  Capital: ${}.00 MXN", srl.capital_social / 100);
            println!("  Partners: {}", srl.num_socios);
            println!(
                "  (Maximum partners: {})",
                LimitedLiabilityCompany::MAX_PARTNERS
            );
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }
}
