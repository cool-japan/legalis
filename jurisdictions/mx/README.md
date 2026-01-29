# Legalis-MX: Mexican Law Library

[![Crates.io](https://img.shields.io/crates/v/legalis-mx.svg)](https://crates.io/crates/legalis-mx)
[![Documentation](https://docs.rs/legalis-mx/badge.svg)](https://docs.rs/legalis-mx)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Comprehensive Mexican legal framework implementation in **Pure Rust**.

## Overview

Mexico operates under a **Civil Law** system, based on the Romano-Germanic tradition with influences from Spanish colonial law and French civil code. This library provides validation, types, and utilities for key Mexican legislation.

## Supported Laws

| Domain | Law | Year | Implementation |
|--------|-----|------|----------------|
| **Civil** | Código Civil Federal (CCF) | 1928 | ✅ Persons, property, obligations, contracts |
| **Criminal** | Código Penal Federal | 1931 | ✅ Offenses, penalties |
| **Labor** | Ley Federal del Trabajo (LFT) | 1970 | ✅ Working hours, aguinaldo, vacation |
| **Tax** | Código Fiscal de la Federación | 1981 | ✅ Tax obligations |
| **Tax - ISR** | Ley del Impuesto Sobre la Renta | - | ✅ Income tax (30% corporate) |
| **Tax - IVA** | Ley del Impuesto al Valor Agregado | - | ✅ VAT (16% standard) |
| **Tax - IEPS** | IEPS | - | ✅ Special production tax |
| **Data Protection** | LFPDPPP | 2010 | ✅ ARCO rights, privacy |
| **Corporate** | LGSM | 1934 | ✅ SA, SRL companies |
| **IP** | Ley de Propiedad Industrial | 1991 | ✅ Patents, trademarks |
| **Competition** | LFCE | 2014 | ✅ Antitrust, market concentration |

## Features

- **Pure Rust** - No C/Fortran dependencies
- **Spanish/English** - Bilingual support (Spanish authoritative)
- **Type-safe** - Strong typing for legal entities
- **Currency** - MXN (Mexican Peso) with centavo precision
- **Documents** - RFC, CURP, NSS validation
- **States** - All 32 Mexican states
- **Holidays** - Federal holiday calculations
- **Citations** - Mexican legal citation formatting

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
legalis-mx = "0.1.4"
```

### Examples

#### Labor Law - Aguinaldo (Christmas Bonus)

```rust
use legalis_mx::labor_law::*;
use legalis_mx::common::MexicanCurrency;

fn main() {
    // Daily salary: 300 pesos
    let daily_salary = MexicanCurrency::from_pesos(300);

    // Calculate aguinaldo for full year (365 days)
    // Article 87 LFT: minimum 15 days
    let aguinaldo = calculate_aguinaldo(daily_salary, 365);

    println!("Aguinaldo: {} pesos", aguinaldo.pesos());
    // Output: Aguinaldo: 4500 pesos (15 days × 300)
}
```

#### Tax Law - IVA (Value Added Tax)

```rust
use legalis_mx::tax_law::*;
use legalis_mx::common::MexicanCurrency;

fn main() {
    // Base amount: 1,000 pesos
    let base = MexicanCurrency::from_pesos(1000);

    // Calculate IVA at standard rate (16%)
    let iva = calculate_iva(base, IVARate::Standard);
    let total = calculate_with_iva(base, IVARate::Standard);

    println!("Base: {} pesos", base.pesos());
    println!("IVA (16%): {} pesos", iva.pesos());
    println!("Total: {} pesos", total.pesos());
    // Output:
    // Base: 1000 pesos
    // IVA (16%): 160 pesos
    // Total: 1160 pesos
}
```

#### Company Law - SA (Stock Corporation)

```rust
use legalis_mx::company_law::*;

fn main() {
    // Create SA with minimum capital (50,000 pesos)
    let sa = StockCorporation::new(
        "Mi Empresa SA de CV".to_string(),
        "Comercio y servicios".to_string(),
        100_000_00, // 100,000 pesos in centavos
        5,          // 5 shareholders
    );

    match sa {
        Ok(company) => {
            println!("SA created: {}", company.denominacion);
            assert!(company.validate().is_ok());
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

#### Data Protection - LFPDPPP

```rust
use legalis_mx::data_protection::*;

fn main() {
    let processing = PersonalDataProcessing {
        responsable: "Empresa SA de CV".to_string(),
        titulares: vec![
            DataSubject {
                nombre: "Juan Pérez".to_string(),
                categorias_datos: vec![
                    DataCategory::Identification,
                    DataCategory::Contact,
                ],
            }
        ],
        finalidad: vec!["Prestación de servicios".to_string()],
        base_legal: LegalBasis::Consent,
        consentimiento: true,
    };

    if let Err(e) = processing.validate() {
        println!("Validation error: {}", e);
    } else {
        println!("Data processing is compliant");
    }
}
```

## Key Features by Module

### Labor Law (`labor_law`)

- **Working Hours**: 8h/day maximum (Article 61)
- **Aguinaldo**: 15 days minimum, paid before Dec 20 (Article 87)
- **Vacation**: 12 days (1st year) + 25% premium (Articles 76, 80)
- **Overtime**: Calculation and validation

### Tax Law (`tax_law`)

- **ISR**: Income tax (30% corporate, progressive individual)
- **IVA**: Value added tax (16% standard, 8% border zone)
- **IEPS**: Special production tax (tobacco, alcohol, sugary drinks)

### Data Protection (`data_protection`)

- **ARCO Rights**: Access, Rectification, Cancellation, Opposition
- **Privacy Notice**: Required elements validation
- **Consent**: Explicit consent for sensitive data

### Company Law (`company_law`)

- **SA**: Stock Corporation (min 50,000 pesos capital)
- **SRL**: Limited Liability Company (max 50 partners)
- **CV**: Variable capital structures

## Documentation

Full documentation available at [docs.rs/legalis-mx](https://docs.rs/legalis-mx)

## References

- [Cámara de Diputados - Federal Legislation](http://www.diputados.gob.mx/LeyesBiblio/)
- [Suprema Corte de Justicia de la Nación](https://www.scjn.gob.mx/)
- [INAI - Data Protection Authority](https://home.inai.org.mx/)
- [IMPI - Industrial Property Institute](https://www.gob.mx/impi)
- [COFECE - Economic Competition Commission](https://www.cofece.mx/)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

COOLJAPAN OU (Team Kitasan)
