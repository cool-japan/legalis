//! Example: Calculate IVA (Value Added Tax)

use legalis_mx::common::MexicanCurrency;
use legalis_mx::tax_law::*;

fn main() {
    println!("=== IVA Calculator ===\n");

    let base = MexicanCurrency::from_pesos(1000);

    // Standard rate (16%)
    println!("Standard rate (16%):");
    let iva_standard = calculate_iva(base, IVARate::Standard);
    let total_standard = calculate_with_iva(base, IVARate::Standard);
    println!("  Base: ${}.00 MXN", base.pesos());
    println!("  IVA (16%): ${}.00 MXN", iva_standard.pesos());
    println!("  Total: ${}.00 MXN\n", total_standard.pesos());

    // Border zone rate (8%)
    println!("Border zone rate (8%):");
    let iva_border = calculate_iva(base, IVARate::Border);
    let total_border = calculate_with_iva(base, IVARate::Border);
    println!("  Base: ${}.00 MXN", base.pesos());
    println!("  IVA (8%): ${}.00 MXN", iva_border.pesos());
    println!("  Total: ${}.00 MXN\n", total_border.pesos());

    // Extract IVA from total
    println!("Extract IVA from total:");
    let total_with_iva = MexicanCurrency::from_pesos(1160);
    let extracted_iva = extract_iva_from_total(total_with_iva, IVARate::Standard);
    println!("  Total: ${}.00 MXN", total_with_iva.pesos());
    println!("  Extracted IVA: ${}.00 MXN", extracted_iva.pesos());
}
