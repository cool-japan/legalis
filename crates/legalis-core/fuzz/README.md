# Fuzzing Targets for legalis-core

This directory contains fuzzing targets for the legalis-core crate using cargo-fuzz (libFuzzer).

## Prerequisites

Install cargo-fuzz:

```bash
cargo install cargo-fuzz
```

## Available Fuzz Targets

### 1. `fuzz_attribute_parse`

Fuzzes the `AttributeValue::parse_from_string()` function with arbitrary UTF-8 strings.

**What it tests:**
- Parsing arbitrary strings into typed attribute values
- String-to-value conversions (bool, u32, u64, i64, f64, dates)
- Display trait implementation for all AttributeValue variants

**Run:**
```bash
cargo fuzz run fuzz_attribute_parse
```

### 2. `fuzz_statute_validation`

Fuzzes statute creation and validation with arbitrary inputs.

**What it tests:**
- Statute construction with fuzzed IDs, titles, and descriptions
- Validation logic with invalid inputs (empty strings, invalid characters, etc.)
- Temporal validity with arbitrary dates (including edge cases)
- Age conditions with extreme values
- Version validation

**Run:**
```bash
cargo fuzz run fuzz_statute_validation
```

### 3. `fuzz_condition_display`

Fuzzes deeply nested Condition structures and their Display implementation.

**What it tests:**
- Recursive condition construction (AND/OR/NOT combinators)
- Display trait with deeply nested structures
- Stack overflow prevention in recursive operations
- All condition variants (Age, Income, HasAttribute, etc.)

**Run:**
```bash
cargo fuzz run fuzz_condition_display
```

## Running Fuzzing

### Basic Usage

Run a specific target for a limited time:

```bash
# Run for 60 seconds
cargo fuzz run fuzz_attribute_parse -- -max_total_time=60
```

### With Custom Options

```bash
# Run with multiple jobs and custom timeout
cargo fuzz run fuzz_statute_validation -- -jobs=4 -timeout=10
```

### Coverage Report

Generate coverage information:

```bash
cargo fuzz coverage fuzz_attribute_parse
```

## Corpus and Artifacts

- **Corpus**: Input files that triggered new code paths are saved in `corpus/<target_name>/`
- **Artifacts**: Inputs that caused crashes or panics are saved in `artifacts/<target_name>/`

## Continuous Integration

For CI, run fuzzing for a short duration to catch obvious issues:

```bash
for target in fuzz_attribute_parse fuzz_statute_validation fuzz_condition_display; do
    cargo fuzz run $target -- -max_total_time=30 -max_len=4096
done
```

## What We're Testing For

1. **No Panics**: All public APIs should handle arbitrary input gracefully
2. **No Stack Overflows**: Recursive structures should have depth limits
3. **No Undefined Behavior**: All unsafe code (if any) is sound
4. **Consistent Behavior**: Round-trip properties (parse → display → parse)

## Known Limitations

- Fuzzing requires nightly Rust: `rustup default nightly`
- Some targets may find intentional panics in debug assertions
- Date fuzzing is limited to reasonable year ranges to avoid overflow
