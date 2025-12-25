use criterion::{Criterion, black_box, criterion_group, criterion_main};
use legalis_i18n::{
    CurrencyFormatter, DateTimeFormatter, LegalDictionary, Locale, NumberFormatter,
    TranslationManager, TranslationMemory,
};

fn locale_parsing_benchmark(c: &mut Criterion) {
    c.bench_function("parse simple locale", |b| {
        b.iter(|| Locale::parse(black_box("en-US")))
    });

    c.bench_function("parse complex locale", |b| {
        b.iter(|| Locale::parse(black_box("zh-Hans-CN")))
    });
}

fn locale_matching_benchmark(c: &mut Criterion) {
    let locale1 = Locale::new("en").with_country("US");
    let locale2 = Locale::new("en").with_country("GB");
    let locale3 = Locale::new("en");

    c.bench_function("locale exact match", |b| {
        b.iter(|| black_box(&locale1).matches(black_box(&locale1)))
    });

    c.bench_function("locale different countries", |b| {
        b.iter(|| black_box(&locale1).matches(black_box(&locale2)))
    });

    c.bench_function("locale fallback match", |b| {
        b.iter(|| black_box(&locale3).matches(black_box(&locale1)))
    });
}

fn locale_fallback_chain_benchmark(c: &mut Criterion) {
    let locale = Locale::new("zh").with_script("Hans").with_country("CN");

    c.bench_function("fallback chain generation", |b| {
        b.iter(|| black_box(&locale).fallback_chain())
    });
}

fn translation_lookup_benchmark(c: &mut Criterion) {
    let mut manager = TranslationManager::new();

    // Add English dictionary
    let en_dict = LegalDictionary::english_us();
    manager.add_dictionary(en_dict);

    // Add Japanese dictionary
    let ja_dict = LegalDictionary::japanese();
    manager.add_dictionary(ja_dict);

    let _en_locale = Locale::new("en").with_country("US");
    let ja_locale = Locale::new("ja").with_country("JP");

    c.bench_function("translation lookup - hit", |b| {
        b.iter(|| manager.translate(black_box("contract"), black_box(&ja_locale)))
    });

    c.bench_function("translation lookup - miss", |b| {
        b.iter(|| manager.translate(black_box("nonexistent_term"), black_box(&ja_locale)))
    });
}

fn translation_memory_benchmark(c: &mut Criterion) {
    let mut memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");

    // Add some translations
    for i in 0..100 {
        memory.add_translation(
            format!("term_{}", i),
            en.clone(),
            format!("用語_{}", i),
            ja.clone(),
        );
    }

    c.bench_function("translation memory exact match", |b| {
        b.iter(|| memory.find_exact(black_box("term_50"), black_box(&en), black_box(&ja)))
    });

    c.bench_function("translation memory fuzzy match", |b| {
        b.iter(|| memory.find_fuzzy(black_box("term_5"), black_box(&en), black_box(&ja), 0.8))
    });
}

fn date_formatting_benchmark(c: &mut Criterion) {
    let us_locale = Locale::new("en").with_country("US");
    let ja_locale = Locale::new("ja").with_country("JP");
    let de_locale = Locale::new("de").with_country("DE");

    let us_formatter = DateTimeFormatter::new(us_locale);
    let ja_formatter = DateTimeFormatter::new(ja_locale);
    let de_formatter = DateTimeFormatter::new(de_locale);

    c.bench_function("date formatting - US", |b| {
        b.iter(|| us_formatter.format_date(black_box(2024), black_box(12), black_box(19)))
    });

    c.bench_function("date formatting - Japanese", |b| {
        b.iter(|| ja_formatter.format_date(black_box(2024), black_box(12), black_box(19)))
    });

    c.bench_function("date formatting - German", |b| {
        b.iter(|| de_formatter.format_date(black_box(2024), black_box(12), black_box(19)))
    });
}

fn currency_formatting_benchmark(c: &mut Criterion) {
    let us_locale = Locale::new("en").with_country("US");
    let formatter = CurrencyFormatter::new(us_locale);

    c.bench_function("currency formatting - USD", |b| {
        b.iter(|| formatter.format(black_box(1234.56), black_box("USD")))
    });

    c.bench_function("currency formatting - JPY", |b| {
        b.iter(|| formatter.format(black_box(123456.0), black_box("JPY")))
    });
}

fn number_formatting_benchmark(c: &mut Criterion) {
    let us_locale = Locale::new("en").with_country("US");
    let formatter = NumberFormatter::new(us_locale);

    c.bench_function("number formatting - small", |b| {
        b.iter(|| formatter.format_integer(black_box(1234)))
    });

    c.bench_function("number formatting - large", |b| {
        b.iter(|| formatter.format_integer(black_box(1234567890)))
    });

    c.bench_function("number formatting - decimal", |b| {
        b.iter(|| formatter.format_decimal(black_box(1234567.89), black_box(2)))
    });
}

criterion_group!(
    benches,
    locale_parsing_benchmark,
    locale_matching_benchmark,
    locale_fallback_chain_benchmark,
    translation_lookup_benchmark,
    translation_memory_benchmark,
    date_formatting_benchmark,
    currency_formatting_benchmark,
    number_formatting_benchmark,
);
criterion_main!(benches);
