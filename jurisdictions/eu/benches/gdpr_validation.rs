//! Benchmarks for GDPR validation operations
//!
//! Run with: cargo bench --bench gdpr_validation

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use legalis_eu::gdpr::*;

fn bench_consent_validation(c: &mut Criterion) {
    c.bench_function("consent_validation", |b| {
        b.iter(|| {
            let processing = DataProcessing::new()
                .with_controller(black_box("Acme Corp"))
                .with_purpose(black_box("Marketing emails"))
                .add_data_category(PersonalDataCategory::Regular("email".to_string()))
                .with_lawful_basis(LawfulBasis::Consent {
                    freely_given: true,
                    specific: true,
                    informed: true,
                    unambiguous: true,
                });

            processing.validate()
        })
    });
}

fn bench_special_category_check(c: &mut Criterion) {
    c.bench_function("special_category_check", |b| {
        b.iter(|| {
            let processing = Article9Processing::new()
                .with_controller(black_box("Hospital Inc"))
                .with_purpose(black_box("Patient treatment"))
                .add_special_category(SpecialCategory::HealthData)
                .with_exception(Article9Exception::ExplicitConsent {
                    purposes: vec!["Medical treatment".to_string()],
                    consent_documented: true,
                });

            processing.validate()
        })
    });
}

fn bench_cross_border_transfer(c: &mut Criterion) {
    use legalis_eu::gdpr::cross_border::*;

    c.bench_function("cross_border_transfer_validation", |b| {
        b.iter(|| {
            let transfer = CrossBorderTransfer::new()
                .with_origin(black_box("EU"))
                .with_destination_country(black_box("Switzerland"))
                .with_adequate_destination(AdequateCountry::Switzerland);

            transfer.validate()
        })
    });
}

criterion_group!(
    benches,
    bench_consent_validation,
    bench_special_category_check,
    bench_cross_border_transfer
);
criterion_main!(benches);
