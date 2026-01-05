#![no_main]

use libfuzzer_sys::fuzz_target;
use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp, TemporalValidity};
use chrono::NaiveDate;

fuzz_target!(|data: &[u8]| {
    if data.len() < 20 {
        return;
    }

    // Extract fuzzing parameters from input bytes
    let id_len = (data[0] as usize % 50).max(1);
    let title_len = (data[1] as usize % 100).max(1);
    let desc_len = (data[2] as usize % 100).max(1);
    let age = u32::from_le_bytes([data[3], data[4], data[5], data[6]]);
    let version = u32::from_le_bytes([data[7], data[8], data[9], data[10]]);

    // Create strings from remaining bytes
    let remaining = &data[11..];
    if remaining.is_empty() {
        return;
    }

    let id = String::from_utf8_lossy(&remaining[..id_len.min(remaining.len())]);
    let title_start = id_len.min(remaining.len());
    let title = String::from_utf8_lossy(
        &remaining[title_start..title_start.saturating_add(title_len).min(remaining.len())]
    );
    let desc_start = title_start.saturating_add(title_len).min(remaining.len());
    let desc = String::from_utf8_lossy(
        &remaining[desc_start..desc_start.saturating_add(desc_len).min(remaining.len())]
    );

    // Create a statute with fuzzed inputs
    let mut statute = Statute::new(
        id.as_ref(),
        title.as_ref(),
        Effect::new(EffectType::Grant, desc.as_ref()),
    );

    // Add conditions with fuzzed values
    statute = statute.with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: age % 200, // Keep age reasonable
    });

    // Set version (may be invalid)
    statute = statute.with_version(version);

    // Fuzz temporal validity with potentially invalid dates
    if data.len() > 30 {
        let year1 = 1900 + (u16::from_le_bytes([data[20], data[21]]) % 300) as i32;
        let month1 = 1 + (data[22] % 12) as u32;
        let day1 = 1 + (data[23] % 28) as u32; // Keep safe for all months

        let year2 = 1900 + (u16::from_le_bytes([data[24], data[25]]) % 300) as i32;
        let month2 = 1 + (data[26] % 12) as u32;
        let day2 = 1 + (data[27] % 28) as u32;

        if let (Some(date1), Some(date2)) = (
            NaiveDate::from_ymd_opt(year1, month1, day1),
            NaiveDate::from_ymd_opt(year2, month2, day2),
        ) {
            statute = statute.with_temporal_validity(
                TemporalValidity::new()
                    .with_effective_date(date1)
                    .with_expiry_date(date2)
            );
        }
    }

    // Test validation - should never panic
    let _errors = statute.validate();

    // Test is_valid
    let _is_valid = statute.is_valid();

    // Test validated method
    let _result = statute.clone().validated();

    // Test Display implementation
    let _display = format!("{}", statute);

    // Test is_active with various dates
    if let Some(test_date) = NaiveDate::from_ymd_opt(2025, 6, 15) {
        let _active = statute.is_active(test_date);
    }
});
