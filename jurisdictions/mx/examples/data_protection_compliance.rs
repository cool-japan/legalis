//! Example: Data protection compliance check

use legalis_mx::data_protection::*;

fn main() {
    println!("=== Data Protection Compliance (LFPDPPP) ===\n");

    // Example 1: Basic data processing with consent
    println!("Example 1: Basic data processing");
    let processing1 = PersonalDataProcessing {
        responsable: "Comercio Electrónico SA de CV".to_string(),
        titulares: vec![DataSubject {
            nombre: "Cliente".to_string(),
            categorias_datos: vec![DataCategory::Identification, DataCategory::Contact],
        }],
        finalidad: vec!["Procesamiento de pedidos".to_string()],
        base_legal: LegalBasis::Contract,
        consentimiento: true,
    };

    match processing1.validate() {
        Ok(_) => println!("  ✓ Data processing is compliant\n"),
        Err(e) => println!("  ✗ Compliance error: {}\n", e),
    }

    // Example 2: Sensitive data processing
    println!("Example 2: Sensitive data processing (health data)");
    let processing2 = PersonalDataProcessing {
        responsable: "Hospital General".to_string(),
        titulares: vec![DataSubject {
            nombre: "Paciente".to_string(),
            categorias_datos: vec![DataCategory::Health, DataCategory::Identification],
        }],
        finalidad: vec!["Atención médica".to_string()],
        base_legal: LegalBasis::Consent,
        consentimiento: true, // Explicit consent required
    };

    match processing2.validate() {
        Ok(_) => {
            println!("  ✓ Sensitive data processing is compliant");
            println!("  Note: Explicit consent obtained for health data\n");
        }
        Err(e) => println!("  ✗ Compliance error: {}\n", e),
    }

    // Example 3: Privacy notice
    println!("Example 3: Privacy notice validation");
    let notice = PrivacyNotice {
        identidad_responsable: "Empresa SA de CV".to_string(),
        finalidades: vec![
            "Prestación de servicios".to_string(),
            "Facturación".to_string(),
        ],
        datos_recabados: vec![
            DataCategory::Identification,
            DataCategory::Contact,
            DataCategory::Financial,
        ],
        transferencias: vec!["Proveedor de servicios de pago".to_string()],
        medios_arco: "privacidad@empresa.com / +52 55 1234 5678".to_string(),
    };

    match notice.validate() {
        Ok(_) => {
            println!("  ✓ Privacy notice is complete");
            println!("  ARCO rights can be exercised at: {}", notice.medios_arco);
        }
        Err(e) => println!("  ✗ Privacy notice error: {}", e),
    }
}
