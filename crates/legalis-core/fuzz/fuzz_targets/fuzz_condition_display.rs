#![no_main]

use libfuzzer_sys::fuzz_target;
use legalis_core::{Condition, ComparisonOp};

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    // Build nested conditions from fuzzer data
    let condition = build_condition_from_bytes(data, 0);

    // Test Display implementation - should never panic or stack overflow
    let _display = format!("{}", condition);

    // Test clone
    let _cloned = condition.clone();
});

fn build_condition_from_bytes(data: &[u8], depth: usize) -> Condition {
    if data.is_empty() || depth > 10 {
        // Base case: simple age condition
        return Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
    }

    let choice = data[0] % 10;
    let remaining = &data[1..];

    match choice {
        0 => {
            // Age condition
            let op = match data[0] % 6 {
                0 => ComparisonOp::Equal,
                1 => ComparisonOp::NotEqual,
                2 => ComparisonOp::GreaterThan,
                3 => ComparisonOp::GreaterOrEqual,
                4 => ComparisonOp::LessThan,
                _ => ComparisonOp::LessOrEqual,
            };
            let value = if remaining.len() >= 4 {
                u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]]) % 150
            } else {
                25
            };
            Condition::Age { operator: op, value }
        }
        1 => {
            // Income condition
            let op = match data[0] % 6 {
                0 => ComparisonOp::Equal,
                1 => ComparisonOp::NotEqual,
                2 => ComparisonOp::GreaterThan,
                3 => ComparisonOp::GreaterOrEqual,
                4 => ComparisonOp::LessThan,
                _ => ComparisonOp::LessOrEqual,
            };
            let value = if remaining.len() >= 8 {
                u64::from_le_bytes([
                    remaining[0], remaining[1], remaining[2], remaining[3],
                    remaining[4], remaining[5], remaining[6], remaining[7],
                ])
            } else {
                50000
            };
            Condition::Income { operator: op, value }
        }
        2 => {
            // HasAttribute
            let key = String::from_utf8_lossy(&remaining[..remaining.len().min(20)]);
            Condition::HasAttribute { key: key.to_string() }
        }
        3 => {
            // AttributeEquals
            let mid = remaining.len() / 2;
            let key = String::from_utf8_lossy(&remaining[..mid.min(20)]);
            let value = String::from_utf8_lossy(&remaining[mid..remaining.len().min(mid + 20)]);
            Condition::AttributeEquals {
                key: key.to_string(),
                value: value.to_string(),
            }
        }
        4 => {
            // ResidencyDuration
            let op = match data[0] % 6 {
                0 => ComparisonOp::Equal,
                1 => ComparisonOp::NotEqual,
                2 => ComparisonOp::GreaterThan,
                3 => ComparisonOp::GreaterOrEqual,
                4 => ComparisonOp::LessThan,
                _ => ComparisonOp::LessOrEqual,
            };
            let months = if remaining.len() >= 4 {
                u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]]) % 1200
            } else {
                12
            };
            Condition::ResidencyDuration { operator: op, months }
        }
        5 => {
            // Custom
            let desc = String::from_utf8_lossy(&remaining[..remaining.len().min(50)]);
            Condition::Custom { description: desc.to_string() }
        }
        6 | 7 => {
            // AND - split data between two sub-conditions
            if remaining.len() < 2 {
                return Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                };
            }
            let split = (remaining.len() / 2).max(1);
            let left = build_condition_from_bytes(&remaining[..split], depth + 1);
            let right = build_condition_from_bytes(&remaining[split..], depth + 1);
            Condition::And(Box::new(left), Box::new(right))
        }
        8 => {
            // OR - split data between two sub-conditions
            if remaining.len() < 2 {
                return Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                };
            }
            let split = (remaining.len() / 2).max(1);
            let left = build_condition_from_bytes(&remaining[..split], depth + 1);
            let right = build_condition_from_bytes(&remaining[split..], depth + 1);
            Condition::Or(Box::new(left), Box::new(right))
        }
        _ => {
            // NOT
            let inner = build_condition_from_bytes(remaining, depth + 1);
            Condition::Not(Box::new(inner))
        }
    }
}
