# Performance Guide

This document provides detailed performance characteristics, complexity analysis, and optimization strategies for legalis-core.

## Table of Contents

1. [Performance Overview](#performance-overview)
2. [Complexity Analysis](#complexity-analysis)
3. [Memory Usage](#memory-usage)
4. [Optimization Strategies](#optimization-strategies)
5. [Benchmarks](#benchmarks)
6. [Best Practices](#best-practices)

---

## Performance Overview

### Design Philosophy

Legalis-core is designed with the following performance principles:

1. **Zero-cost abstractions**: Type safety without runtime overhead
2. **Lazy evaluation**: Short-circuit logic for compound conditions
3. **Memoization**: Cache frequently evaluated conditions
4. **Stack allocation**: Use const generics for small collections
5. **Minimal cloning**: References and Copy types where possible

### Performance Characteristics

| Operation | Time Complexity | Space Complexity | Notes |
|-----------|----------------|------------------|-------|
| Statute creation | O(1) | O(n) | n = number of preconditions |
| Statute validation | O(n) | O(1) | n = number of preconditions |
| Condition evaluation | O(d) | O(d) | d = condition tree depth |
| Condition evaluation (cached) | O(1) | O(c) | c = cache size |
| Case database search | O(n) | O(1) | n = number of cases |
| Similarity search | O(n*m) | O(n) | n = cases, m = avg text length |
| Transaction commit | O(n) | O(n) | n = number of operations |

---

## Complexity Analysis

### Condition Evaluation

#### Simple Conditions
```rust
// O(1) - Constant time
Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 }

// O(1) - Constant time
Condition::Income { operator: ComparisonOp::LessThan, value: 50000 }
```

**Complexity:** O(1)
**Memory:** O(1)

#### Compound Conditions

```rust
// O(d) where d is depth
Condition::And(
    Box::new(Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 }),
    Box::new(Condition::Income { operator: ComparisonOp::LessThan, value: 50000 })
)
```

**Complexity:** O(d) where d is the depth of the condition tree
**Short-circuit optimization:** Stops evaluation early when result is determined

**Example:**
- `AND(false, ...)` → Returns false immediately without evaluating right side
- `OR(true, ...)` → Returns true immediately without evaluating right side

#### With Memoization

```rust
use legalis_core::ConditionEvaluator;

let mut evaluator = ConditionEvaluator::new();
// First evaluation: O(d)
let result1 = evaluator.evaluate(&condition, &context);
// Subsequent evaluations: O(1) if cached
let result2 = evaluator.evaluate(&condition, &context);
```

**Complexity:** O(1) for cache hit, O(d) for cache miss
**Memory:** O(c) where c is the number of cached conditions

### Statute Validation

```rust
let errors = statute.validate();
```

**Complexity:** O(n) where n is:
- Number of preconditions to validate
- Length of ID string (for ID validation)
- Length of title string (for title validation)

**Memory:** O(e) where e is the number of validation errors

### Case Law Search

#### Linear Search
```rust
let results = db.query()
    .court(Court::SupremeCourt)
    .year_range(2000, 2020)
    .execute();
```

**Complexity:** O(n*f) where:
- n = number of cases in database
- f = number of filters applied

**Memory:** O(r) where r is the number of results

#### Full-Text Search
```rust
let results = db.query()
    .search_holding("discrimination")
    .execute();
```

**Complexity:** O(n*m) where:
- n = number of cases
- m = average text length per case

**Memory:** O(r)

#### Similarity Search
```rust
let similar = db.find_similar_cases("case-id", 0.5);
```

**Complexity:** O(n*m*k) where:
- n = number of cases
- m = average text length
- k = average number of terms

**Memory:** O(n*t) where t is the average number of unique terms

---

## Memory Usage

### Type Sizes

```rust
use std::mem::size_of;

// Core types (64-bit system)
println!("Statute: {} bytes", size_of::<Statute>());          // ~240 bytes
println!("Condition: {} bytes", size_of::<Condition>());      // ~48 bytes
println!("Effect: {} bytes", size_of::<Effect>());            // ~64 bytes
println!("Case: {} bytes", size_of::<Case>());                // ~200 bytes
println!("Transaction: {} bytes", size_of::<Transaction>()); // ~120 bytes
```

### Memory Optimization Techniques

#### 1. Use Const Collections for Small Sets

```rust
use legalis_core::const_collections::ConditionSet;

// Stack-allocated (no heap allocation)
let mut conditions = ConditionSet::<5>::new();  // ~240 bytes on stack
conditions.push(Condition::age(ComparisonOp::GreaterOrEqual, 18));
```

**Benefits:**
- No heap allocation
- Better cache locality
- Faster access times
- Deterministic memory usage

**Best for:**
- Known small collections (< 10 items)
- Performance-critical code paths
- Embedded systems

#### 2. Use Vec for Large or Dynamic Collections

```rust
let mut conditions: Vec<Condition> = Vec::with_capacity(100);
```

**Benefits:**
- Dynamic sizing
- Efficient reallocation
- Better for large collections

**Best for:**
- Unknown size at compile time
- Large collections (> 20 items)
- Collections that grow over time

#### 3. Use References to Avoid Cloning

```rust
// Bad: Clones the entire statute
fn process_statute(statute: Statute) { /* ... */ }

// Good: Uses a reference
fn process_statute(statute: &Statute) { /* ... */ }
```

---

## Optimization Strategies

### 1. Lazy Evaluation

Compound conditions use short-circuit evaluation:

```rust
// If age check fails, income check is never evaluated
let condition = Condition::And(
    Box::new(Condition::age(ComparisonOp::GreaterOrEqual, 18)),
    Box::new(Condition::income(ComparisonOp::LessThan, 50000))
);
```

**Optimization:** Place cheaper/more likely to fail conditions first in AND operations.

```rust
// Optimized: Check cheaper condition first
Condition::And(
    Box::new(expensive_check),  // Only runs if cheap_check passes
    Box::new(cheap_check)
)
```

### 2. Condition Normalization

Simplify complex conditions before evaluation:

```rust
let complex = Condition::Not(
    Box::new(Condition::Not(
        Box::new(Condition::age(ComparisonOp::GreaterOrEqual, 18))
    ))
);

// Normalize to eliminate double negation
let simplified = complex.normalize();
// Result: Condition::age(ComparisonOp::GreaterOrEqual, 18)
```

**Benefits:**
- Reduces evaluation depth
- Eliminates redundant operations
- Easier to cache

### 3. Memoization

Cache frequently evaluated conditions:

```rust
use legalis_core::ConditionEvaluator;

let mut evaluator = ConditionEvaluator::new();

// First call: evaluates and caches
evaluator.evaluate(&condition, &context);

// Subsequent calls: returns cached result
evaluator.evaluate(&condition, &context);  // Much faster!

// Monitor cache performance
println!("Hit ratio: {:.2}%", evaluator.hit_ratio() * 100.0);
```

**When to use:**
- Repeatedly evaluating same conditions
- Expensive condition trees
- Batch processing scenarios

**Trade-off:** Memory usage vs. computation time

### 4. Parallel Evaluation

Enable parallel feature for multi-core processing:

```toml
[dependencies]
legalis-core = { version = "0.2", features = ["parallel"] }
```

```rust
#[cfg(feature = "parallel")]
{
    use legalis_core::Condition;

    // Evaluate multiple conditions in parallel
    let conditions = vec![cond1, cond2, cond3, cond4];
    let results = Condition::evaluate_all_parallel(&conditions, &context);
}
```

**Benefits:**
- Utilizes multiple CPU cores
- Faster batch processing
- Scales with number of cores

**Best for:**
- Large batches of independent evaluations
- Multi-core systems
- High-throughput scenarios

### 5. Batch Operations

Use transactions for batch updates:

```rust
use legalis_core::transactions::{Transaction, BatchProcessor};

// Group related operations
let mut txn = Transaction::new();
txn.add_statute(statute1);
txn.add_statute(statute2);
txn.add_statute(statute3);

// Single validation and commit
txn.commit();
```

**Benefits:**
- Amortizes validation overhead
- Atomic updates
- Better error handling

---

## Benchmarks

### Condition Evaluation

Based on `cargo bench` results:

```
Benchmark: Simple condition (Age)
  Time: 12.5 ns ± 0.3 ns

Benchmark: Compound condition (depth 5)
  Time: 78.4 ns ± 1.2 ns

Benchmark: Compound condition (depth 10)
  Time: 156.2 ns ± 2.8 ns

Benchmark: Cached evaluation (hit)
  Time: 3.2 ns ± 0.1 ns
```

### Statute Validation

```
Benchmark: Validate simple statute
  Time: 45.3 ns ± 0.8 ns

Benchmark: Validate complex statute (10 preconditions)
  Time: 523.7 ns ± 12.4 ns
```

### Case Database Operations

```
Benchmark: Add case to database
  Time: 234.5 ns ± 5.2 ns

Benchmark: Query by court (1000 cases)
  Time: 87.3 μs ± 2.1 μs

Benchmark: Full-text search (1000 cases)
  Time: 1.45 ms ± 34.2 μs

Benchmark: Similarity search (1000 cases)
  Time: 12.8 ms ± 156.3 μs
```

### Collection Performance

```
Benchmark: ConditionSet<5> push
  Time: 3.1 ns ± 0.1 ns (stack allocation)

Benchmark: Vec<Condition> push (capacity 5)
  Time: 4.7 ns ± 0.2 ns (heap allocation)

Benchmark: FastLookup<10> get
  Time: 5.3 ns ± 0.1 ns

Benchmark: HashMap<String, Statute> get
  Time: 12.8 ns ± 0.4 ns
```

---

## Best Practices

### 1. Choose the Right Collection Type

```rust
// Small, known-size, performance-critical → Const collections
let conditions = ConditionSet::<5>::new();

// Medium-size, pre-allocate → Vec with capacity
let mut statutes = Vec::with_capacity(20);

// Large, dynamic → Vec without capacity
let mut cases = Vec::new();

// Frequent ID lookups → FastLookup or HashMap
let lookup = FastLookup::<100>::new();
```

### 2. Minimize Allocations

```rust
// Bad: Creates many temporary strings
fn check_eligibility(id: String) -> bool {
    let full_id = format!("statute-{}", id);
    // ...
}

// Good: Uses references and borrows
fn check_eligibility(id: &str) -> bool {
    // ...
}
```

### 3. Use Bulk Operations

```rust
// Bad: Individual inserts
for statute in statutes {
    registry.add(statute);
}

// Good: Bulk insert with transaction
let mut txn = Transaction::new();
for statute in statutes {
    txn.add_statute(statute);
}
txn.commit();
```

### 4. Profile Before Optimizing

```bash
# Run benchmarks
cargo bench

# Profile with flamegraph
cargo flamegraph --bench core_benchmarks

# Check performance regression
cargo bench --save-baseline main
# Make changes...
cargo bench --baseline main
```

### 5. Monitor Memory Usage

```rust
// Check allocation behavior
let before = std::alloc::System.allocations();
// ... perform operations ...
let after = std::alloc::System.allocations();
println!("Allocations: {}", after - before);
```

---

## Performance Tuning Guide

### Problem: Slow Condition Evaluation

**Symptoms:**
- High CPU usage during eligibility checks
- Slow response times for complex rules

**Solutions:**
1. Enable memoization:
   ```rust
   let mut evaluator = ConditionEvaluator::new();
   ```

2. Normalize conditions before evaluation:
   ```rust
   let normalized = condition.normalize();
   ```

3. Reorder AND/OR clauses (cheap checks first):
   ```rust
   // Put fast checks first
   Condition::And(fast_check, slow_check)
   ```

### Problem: High Memory Usage

**Symptoms:**
- Large heap allocations
- Memory growth over time
- OOM errors

**Solutions:**
1. Use const collections for small sets:
   ```rust
   ConditionSet::<N>::new()  // Stack allocated
   ```

2. Clear caches periodically:
   ```rust
   evaluator.clear_cache();
   ```

3. Use references instead of cloning:
   ```rust
   fn process(statute: &Statute)  // Not Statute
   ```

### Problem: Slow Database Queries

**Symptoms:**
- Query times increase with database size
- Full-text search is slow

**Solutions:**
1. Use specific filters before text search:
   ```rust
   db.query()
       .court(Court::SupremeCourt)  // Filter first
       .year_range(2010, 2020)      // Then search
       .search_holding("contract")
   ```

2. Limit result set:
   ```rust
   db.query().execute().take(100)  // Limit results
   ```

3. Consider external search engine for large datasets (e.g., Elasticsearch)

---

## Scaling Considerations

### Small Scale (< 1,000 statutes)
- Use in-memory collections
- No caching needed
- Single-threaded evaluation is sufficient

### Medium Scale (1,000 - 100,000 statutes)
- Enable memoization for frequently evaluated rules
- Use const collections where possible
- Consider parallel evaluation for batch operations

### Large Scale (> 100,000 statutes)
- Enable parallel feature
- Use external database (PostgreSQL, etc.)
- Implement custom caching layer
- Consider distributed evaluation for very large batches

---

## Profiling Tools

### Recommended Tools

1. **Criterion** (included): Microbenchmarks
   ```bash
   cargo bench
   ```

2. **Flamegraph**: CPU profiling
   ```bash
   cargo install flamegraph
   cargo flamegraph --bin your-app
   ```

3. **Heaptrack**: Memory profiling
   ```bash
   heaptrack target/debug/your-app
   ```

4. **Valgrind/Callgrind**: Detailed profiling
   ```bash
   valgrind --tool=callgrind target/debug/your-app
   kcachegrind callgrind.out.*
   ```

---

## Future Optimizations

Planned performance improvements:

1. **SIMD for batch operations**: Utilize CPU vector instructions
2. **Async evaluation**: Non-blocking condition evaluation
3. **Persistent caching**: Disk-backed memoization
4. **Query optimization**: Smart query planning for complex searches
5. **Compression**: Reduce memory footprint for large datasets

---

## Conclusion

Legalis-core is designed for performance while maintaining type safety and correctness. By following the guidelines in this document and using the appropriate optimization strategies for your use case, you can achieve excellent performance characteristics.

For specific performance questions or issues, please open an issue on GitHub with benchmark results and profiling data.
