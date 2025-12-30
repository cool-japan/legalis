//! Benchmarks for smart contract generation performance.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use legalis_chain::{
    BatchOperationConfig, ContractGenerator, FormalVerificationConfig, MultiNetworkConfig,
    NetworkConfig, ProxyPattern, TargetPlatform, TestSuiteConfig,
};
use legalis_core::{Effect, EffectType, Statute};

fn create_sample_statute(name: &str, _num_conditions: usize) -> Statute {
    Statute::new(
        name.to_string(),
        format!("{} Title", name),
        Effect::new(EffectType::Grant, "Grant permission"),
    )
}

fn bench_solidity_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("solidity_generation");

    for num_conditions in [1, 5, 10, 20].iter() {
        let statute =
            create_sample_statute(&format!("TestStatute{}", num_conditions), *num_conditions);

        group.bench_with_input(
            BenchmarkId::new("basic", num_conditions),
            num_conditions,
            |b, _| {
                let generator = ContractGenerator::new(TargetPlatform::Solidity);
                b.iter(|| black_box(generator.generate(&statute).unwrap()));
            },
        );
    }

    group.finish();
}

fn bench_vyper_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("vyper_generation");

    for num_conditions in [1, 5, 10].iter() {
        let statute =
            create_sample_statute(&format!("TestStatute{}", num_conditions), *num_conditions);

        group.bench_with_input(
            BenchmarkId::new("basic", num_conditions),
            num_conditions,
            |b, _| {
                let generator = ContractGenerator::new(TargetPlatform::Vyper);
                b.iter(|| black_box(generator.generate(&statute).unwrap()));
            },
        );
    }

    group.finish();
}

fn bench_wasm_generation(c: &mut Criterion) {
    let statute = create_sample_statute("WasmTest", 5);
    let generator = ContractGenerator::new(TargetPlatform::RustWasm);

    c.bench_function("rust_wasm_generation", |b| {
        b.iter(|| black_box(generator.generate(&statute).unwrap()));
    });
}

fn bench_move_generation(c: &mut Criterion) {
    let statute = create_sample_statute("MoveTest", 5);
    let generator = ContractGenerator::new(TargetPlatform::Move);

    c.bench_function("move_generation", |b| {
        b.iter(|| black_box(generator.generate(&statute).unwrap()));
    });
}

fn bench_cairo_generation(c: &mut Criterion) {
    let statute = create_sample_statute("CairoTest", 5);
    let generator = ContractGenerator::new(TargetPlatform::Cairo);

    c.bench_function("cairo_generation", |b| {
        b.iter(|| black_box(generator.generate(&statute).unwrap()));
    });
}

fn bench_cosmwasm_generation(c: &mut Criterion) {
    let statute = create_sample_statute("CosmWasmTest", 5);
    let generator = ContractGenerator::new(TargetPlatform::CosmWasm);

    c.bench_function("cosmwasm_generation", |b| {
        b.iter(|| black_box(generator.generate(&statute).unwrap()));
    });
}

fn bench_ton_generation(c: &mut Criterion) {
    let statute = create_sample_statute("TonTest", 5);
    let generator = ContractGenerator::new(TargetPlatform::Ton);

    c.bench_function("ton_generation", |b| {
        b.iter(|| black_box(generator.generate(&statute).unwrap()));
    });
}

fn bench_teal_generation(c: &mut Criterion) {
    let statute = create_sample_statute("TealTest", 5);
    let generator = ContractGenerator::new(TargetPlatform::Teal);

    c.bench_function("teal_generation", |b| {
        b.iter(|| black_box(generator.generate(&statute).unwrap()));
    });
}

fn bench_batch_generation(c: &mut Criterion) {
    let statutes: Vec<Statute> = (0..10)
        .map(|i| create_sample_statute(&format!("Statute{}", i), 3))
        .collect();

    c.bench_function("batch_generation", |b| {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        b.iter(|| black_box(generator.generate_batch(&statutes)));
    });
}

fn bench_factory_generation(c: &mut Criterion) {
    let statute_ids = vec!["Statute1", "Statute2", "Statute3", "Statute4", "Statute5"];

    c.bench_function("factory_generation", |b| {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        b.iter(|| black_box(generator.generate_factory(&statute_ids).unwrap()));
    });
}

fn bench_proxy_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("proxy_generation");

    let patterns = vec![
        ("transparent", ProxyPattern::Transparent),
        ("uups", ProxyPattern::Uups),
        ("beacon", ProxyPattern::Beacon),
    ];

    for (name, pattern) in patterns {
        group.bench_with_input(
            BenchmarkId::new("pattern", name),
            &pattern,
            |b, &pattern| {
                let generator = ContractGenerator::new(TargetPlatform::Solidity);
                b.iter(|| {
                    black_box(
                        generator
                            .generate_proxy_with_pattern("TestContract", pattern)
                            .unwrap(),
                    )
                });
            },
        );
    }

    group.finish();
}

fn bench_test_suite_generation(c: &mut Criterion) {
    let statute = create_sample_statute("TestStatute", 5);
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate(&statute).unwrap();
    let config = TestSuiteConfig::default();

    c.bench_function("test_suite_generation", |b| {
        b.iter(|| black_box(generator.generate_test_suite(&contract, &config).unwrap()));
    });
}

fn bench_security_analysis(c: &mut Criterion) {
    use legalis_chain::SecurityAnalyzer;

    let statute = create_sample_statute("SecurityTest", 10);
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate(&statute).unwrap();

    c.bench_function("security_analysis", |b| {
        b.iter(|| black_box(SecurityAnalyzer::analyze(&contract)));
    });
}

fn bench_formal_verification_generation(c: &mut Criterion) {
    let statute = create_sample_statute("FormalVerification", 5);
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate(&statute).unwrap();
    let config = FormalVerificationConfig::default();

    c.bench_function("formal_verification_generation", |b| {
        b.iter(|| {
            black_box(
                generator
                    .generate_formal_verification(&contract, &config)
                    .unwrap(),
            )
        });
    });
}

fn bench_batch_operations(c: &mut Criterion) {
    let statute = create_sample_statute("BatchOps", 5);
    let config = BatchOperationConfig::default();

    c.bench_function("batch_operations_generation", |b| {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        b.iter(|| {
            black_box(
                generator
                    .generate_with_batch_operations(&statute, &config)
                    .unwrap(),
            )
        });
    });
}

fn bench_multi_network_config(c: &mut Criterion) {
    let statute = create_sample_statute("MultiNetwork", 5);
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate(&statute).unwrap();

    let config = MultiNetworkConfig {
        networks: vec![
            NetworkConfig {
                name: "mainnet".to_string(),
                rpc_url: "https://mainnet.infura.io".to_string(),
                chain_id: 1,
                gas_limit: Some(8000000),
                gas_price: Some(20),
                etherscan_api_key: None,
            },
            NetworkConfig {
                name: "goerli".to_string(),
                rpc_url: "https://goerli.infura.io".to_string(),
                chain_id: 5,
                gas_limit: Some(8000000),
                gas_price: Some(10),
                etherscan_api_key: None,
            },
        ],
        default_network: "goerli".to_string(),
    };

    c.bench_function("multi_network_config_generation", |b| {
        b.iter(|| {
            black_box(
                generator
                    .generate_multi_network_config(&contract, &config)
                    .unwrap(),
            )
        });
    });
}

fn bench_modular_generation(c: &mut Criterion) {
    let statute = create_sample_statute("Modular", 8);

    c.bench_function("modular_generation", |b| {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        b.iter(|| black_box(generator.generate_modular(&statute).unwrap()));
    });
}

fn bench_inheritance_generation(c: &mut Criterion) {
    let statute = create_sample_statute("Inheritance", 5);
    let base_contracts = vec!["Ownable", "Pausable"];

    c.bench_function("inheritance_generation", |b| {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        b.iter(|| {
            black_box(
                generator
                    .generate_with_inheritance(&statute, &base_contracts)
                    .unwrap(),
            )
        });
    });
}

fn bench_diamond_pattern(c: &mut Criterion) {
    let statutes = vec![
        create_sample_statute("Diamond1", 5),
        create_sample_statute("Diamond2", 5),
        create_sample_statute("Diamond3", 5),
    ];

    c.bench_function("diamond_pattern_generation", |b| {
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        b.iter(|| black_box(generator.generate_diamond(&statutes).unwrap()));
    });
}

criterion_group!(
    benches,
    bench_solidity_generation,
    bench_vyper_generation,
    bench_wasm_generation,
    bench_move_generation,
    bench_cairo_generation,
    bench_cosmwasm_generation,
    bench_ton_generation,
    bench_teal_generation,
    bench_batch_generation,
    bench_factory_generation,
    bench_proxy_generation,
    bench_test_suite_generation,
    bench_security_analysis,
    bench_formal_verification_generation,
    bench_batch_operations,
    bench_multi_network_config,
    bench_modular_generation,
    bench_inheritance_generation,
    bench_diamond_pattern,
);

criterion_main!(benches);
