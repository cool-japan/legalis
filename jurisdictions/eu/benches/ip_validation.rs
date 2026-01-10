//! Benchmarks for IP validation operations
//!
//! Run with: cargo bench --bench ip_validation

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use legalis_eu::intellectual_property::*;

fn bench_trademark_validation(c: &mut Criterion) {
    c.bench_function("trademark_validation", |b| {
        b.iter(|| {
            let trademark = EuTrademark::new()
                .with_mark_text(black_box("INNOVATECH"))
                .with_mark_type(MarkType::WordMark)
                .with_applicant(black_box("Tech Company GmbH"))
                .add_nice_class(9)
                .unwrap()
                .add_goods_services(black_box("Computer software"));

            trademark.validate()
        })
    });
}

fn bench_design_validation(c: &mut Criterion) {
    c.bench_function("design_validation", |b| {
        b.iter(|| {
            let design = CommunityDesign::new()
                .with_design_type(DesignType::Registered)
                .with_appearance(DesignAppearance {
                    features: vec![black_box("Curved edges").to_string()],
                    product_indication: black_box("Smartphone case").to_string(),
                })
                .with_creator(black_box("Designer"))
                .with_owner(black_box("Company"))
                .with_novelty(true)
                .with_individual_character(true);

            design.validate()
        })
    });
}

fn bench_copyright_validation(c: &mut Criterion) {
    c.bench_function("copyright_validation", |b| {
        b.iter(|| {
            let work = CopyrightWork::new()
                .with_title(black_box("Software Application"))
                .with_author(black_box("Developer"))
                .with_work_type(WorkType::Software)
                .with_originality(true)
                .with_fixation(true);

            work.validate()
        })
    });
}

fn bench_trade_secret_validation(c: &mut Criterion) {
    c.bench_function("trade_secret_validation", |b| {
        b.iter(|| {
            let secret = TradeSecret::new()
                .with_description(black_box("Proprietary algorithm"))
                .with_holder(black_box("Tech Corp"))
                .with_characteristics(TradeSecretCharacteristics {
                    is_secret: true,
                    has_commercial_value: true,
                    reasonable_steps_taken: true,
                })
                .add_protective_measure(black_box("NDA"));

            secret.validate()
        })
    });
}

fn bench_trade_secret_misappropriation_analysis(c: &mut Criterion) {
    c.bench_function("misappropriation_analysis", |b| {
        let secret = TradeSecret::new()
            .with_description("Algorithm")
            .with_holder("Company")
            .with_characteristics(TradeSecretCharacteristics {
                is_secret: true,
                has_commercial_value: true,
                reasonable_steps_taken: true,
            });

        b.iter(|| secret.analyze_misappropriation(black_box(AcquisitionMethod::UnauthorizedAccess)))
    });
}

criterion_group!(
    benches,
    bench_trademark_validation,
    bench_design_validation,
    bench_copyright_validation,
    bench_trade_secret_validation,
    bench_trade_secret_misappropriation_analysis
);
criterion_main!(benches);
