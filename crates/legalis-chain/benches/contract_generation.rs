//! Benchmarks for smart contract generation performance.

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use legalis_chain::{
    BatchOperationConfig, ContractGenerator, FormalVerificationConfig, MultiNetworkConfig,
    NetworkConfig, ProxyPattern, TargetPlatform, TestSuiteConfig,
};
use legalis_core::{Effect, EffectType, Statute};
use std::hint::black_box;

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

/// Benchmark code size analysis
fn bench_code_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("code_analysis");

    let statute = create_sample_statute("analysis-test", 10);
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate(&statute).unwrap();

    group.bench_function("measure_source_length", |b| {
        b.iter(|| black_box(contract.source.len()))
    });

    group.bench_function("count_lines", |b| {
        b.iter(|| black_box(contract.source.lines().count()))
    });

    group.finish();
}

/// Benchmark multi-platform compilation comparison
fn bench_platform_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("platform_comparison");

    let statute = create_sample_statute("platform-test", 5);

    let platforms = vec![
        ("solidity", TargetPlatform::Solidity),
        ("vyper", TargetPlatform::Vyper),
        ("rust_wasm", TargetPlatform::RustWasm),
        ("move", TargetPlatform::Move),
        ("cairo", TargetPlatform::Cairo),
        ("cosmwasm", TargetPlatform::CosmWasm),
        ("ton", TargetPlatform::Ton),
        ("teal", TargetPlatform::Teal),
    ];

    for (name, platform) in platforms {
        group.bench_with_input(
            BenchmarkId::new("generate", name),
            &platform,
            |b, &platform| {
                let generator = ContractGenerator::new(platform);
                b.iter(|| black_box(generator.generate(&statute).unwrap()))
            },
        );
    }

    group.finish();
}

/// Benchmark contract size analysis
fn bench_contract_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("contract_size");

    for num_conditions in [1, 5, 10, 25, 50].iter() {
        let statute =
            create_sample_statute(&format!("size-test-{}", num_conditions), *num_conditions);
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        group.bench_with_input(
            BenchmarkId::new("measure_size", num_conditions),
            &contract,
            |b, contract| b.iter(|| black_box(contract.source.len())),
        );
    }

    group.finish();
}

/// Benchmark ABI operations
fn bench_abi_operations(c: &mut Criterion) {
    let statute = create_sample_statute("abi-test", 5);
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate(&statute).unwrap();

    c.bench_function("check_abi_presence", |b| {
        b.iter(|| black_box(contract.abi.is_some()))
    });

    if let Some(ref abi) = contract.abi {
        c.bench_function("clone_abi", |b| b.iter(|| black_box(abi.clone())));
    }
}

/// Benchmark deployment operations
fn bench_deployment_operations(c: &mut Criterion) {
    let statute = create_sample_statute("deploy-test", 5);
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate(&statute).unwrap();

    c.bench_function("check_deployment_script", |b| {
        b.iter(|| black_box(contract.deployment_script.is_some()))
    });

    c.bench_function("clone_contract", |b| b.iter(|| black_box(contract.clone())));
}

/// Benchmark contract metadata operations
fn bench_contract_metadata(c: &mut Criterion) {
    let statute = create_sample_statute("metadata-test", 5);
    let generator = ContractGenerator::new(TargetPlatform::Solidity);
    let contract = generator.generate(&statute).unwrap();

    c.bench_function("check_contract_name", |b| {
        b.iter(|| black_box(&contract.name))
    });

    c.bench_function("check_platform", |b| {
        b.iter(|| black_box(contract.platform))
    });
}

/// Benchmark contract string operations
fn bench_contract_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("contract_strings");

    for num_conditions in [1, 5, 10, 20].iter() {
        let statute =
            create_sample_statute(&format!("string-test-{}", num_conditions), *num_conditions);
        let generator = ContractGenerator::new(TargetPlatform::Solidity);
        let contract = generator.generate(&statute).unwrap();

        group.bench_with_input(
            BenchmarkId::new("source_contains", num_conditions),
            &contract,
            |b, contract| b.iter(|| black_box(contract.source.contains("function"))),
        );

        group.bench_with_input(
            BenchmarkId::new("source_split_lines", num_conditions),
            &contract,
            |b, contract| b.iter(|| black_box(contract.source.lines().count())),
        );
    }

    group.finish();
}

/// Benchmark parallel contract generation
fn bench_parallel_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_generation");

    for count in [10, 25, 50].iter() {
        let statutes: Vec<_> = (0..*count)
            .map(|i| create_sample_statute(&format!("parallel-{}", i), 3))
            .collect();

        group.bench_with_input(
            BenchmarkId::new("sequential", count),
            &statutes,
            |b, stats| {
                let generator = ContractGenerator::new(TargetPlatform::Solidity);
                b.iter(|| {
                    let _: Vec<_> = stats
                        .iter()
                        .map(|s| generator.generate(s).unwrap())
                        .collect();
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel", count),
            &statutes,
            |b, stats| {
                let generator = ContractGenerator::new(TargetPlatform::Solidity);
                b.iter(|| black_box(generator.generate_batch(stats)))
            },
        );
    }

    group.finish();
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
    bench_code_analysis,
    bench_platform_comparison,
    bench_contract_size,
    bench_abi_operations,
    bench_deployment_operations,
    bench_contract_metadata,
    bench_contract_strings,
    bench_parallel_generation,
);

criterion_main!(benches);
