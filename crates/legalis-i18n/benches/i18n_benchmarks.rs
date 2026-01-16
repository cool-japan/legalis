use criterion::{Criterion, criterion_group, criterion_main};
use legalis_i18n::{
    BatchTranslator, CurrencyFormatter, DateTimeFormatter, LazyDictionary, LegalDictionary, Locale,
    NumberFormatter, TermIndex, TranslationManager, TranslationMemory,
};
use std::hint::black_box;

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

// Performance optimization benchmarks (v0.2.3)
fn lru_cache_benchmark(c: &mut Criterion) {
    let mut manager = TranslationManager::new();
    let ja_dict = LegalDictionary::japanese();
    manager.add_dictionary(ja_dict);

    let locale = Locale::new("ja").with_country("JP");

    c.bench_function("LRU cache - first lookup (miss)", |b| {
        b.iter(|| manager.translate(black_box("contract"), black_box(&locale)))
    });

    // Warm up cache
    let _ = manager.translate("contract", &locale);

    c.bench_function("LRU cache - cached lookup (hit)", |b| {
        b.iter(|| manager.translate(black_box("contract"), black_box(&locale)))
    });
}

fn term_index_benchmark(c: &mut Criterion) {
    let dict = LegalDictionary::english_us();
    let index = dict.build_term_index();

    c.bench_function("term index - prefix lookup (2 chars)", |b| {
        b.iter(|| index.find_by_prefix(black_box("co")))
    });

    c.bench_function("term index - prefix lookup (4 chars)", |b| {
        b.iter(|| index.find_by_prefix(black_box("cont")))
    });

    c.bench_function("term index - build index", |b| {
        b.iter(|| black_box(&dict).build_term_index())
    });
}

fn lazy_loading_benchmark(c: &mut Criterion) {
    let locale = Locale::new("en").with_country("US");

    c.bench_function("lazy dictionary - first access", |b| {
        b.iter(|| {
            let lazy_dict = LazyDictionary::new(locale.clone(), LegalDictionary::english_us);
            black_box(lazy_dict.get())
        })
    });

    let lazy_dict = LazyDictionary::new(locale.clone(), LegalDictionary::english_us);
    let _ = lazy_dict.get(); // Load it

    c.bench_function("lazy dictionary - already loaded", |b| {
        b.iter(|| black_box(lazy_dict.get()))
    });
}

fn batch_translation_benchmark(c: &mut Criterion) {
    let mut manager = TranslationManager::new();

    let mut ja_dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
    ja_dict.add_translation("contract", "契約");
    ja_dict.add_translation("law", "法律");
    ja_dict.add_translation("court", "裁判所");
    ja_dict.add_translation("statute", "法令");
    ja_dict.add_translation("plaintiff", "原告");
    manager.add_dictionary(ja_dict);

    let batch = BatchTranslator::new(manager);
    let ja_locale = Locale::new("ja").with_country("JP");

    let keys = vec!["contract", "law", "court", "statute", "plaintiff"];

    c.bench_function("batch translation - 5 terms (parallel)", |b| {
        b.iter(|| batch.translate_batch(black_box(&keys), black_box(&ja_locale)))
    });

    let keys_20: Vec<&str> = (0..20).map(|_| "contract").collect();

    c.bench_function("batch translation - 20 terms (parallel)", |b| {
        b.iter(|| batch.translate_batch(black_box(&keys_20), black_box(&ja_locale)))
    });
}

fn term_index_scaling_benchmark(c: &mut Criterion) {
    let mut index = TermIndex::new();

    // Index 100 terms
    for i in 0..100 {
        index.index_term(format!("term_{}", i));
    }

    c.bench_function("term index - 100 terms - lookup", |b| {
        b.iter(|| index.find_by_prefix(black_box("term_5")))
    });

    let mut large_index = TermIndex::new();

    // Index 1000 terms
    for i in 0..1000 {
        large_index.index_term(format!("legal_term_{}", i));
    }

    c.bench_function("term index - 1000 terms - lookup", |b| {
        b.iter(|| large_index.find_by_prefix(black_box("legal_term_5")))
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
    lru_cache_benchmark,
    term_index_benchmark,
    lazy_loading_benchmark,
    batch_translation_benchmark,
    term_index_scaling_benchmark,
);
criterion_main!(benches);
