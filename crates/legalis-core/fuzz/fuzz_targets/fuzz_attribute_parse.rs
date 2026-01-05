#![no_main]

use libfuzzer_sys::fuzz_target;
use legalis_core::AttributeValue;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string, ignoring invalid UTF-8
    if let Ok(s) = std::str::from_utf8(data) {
        // Fuzzing target: parse arbitrary strings
        // This should never panic, only produce valid AttributeValue variants
        let parsed = AttributeValue::parse_from_string(s);

        // Verify the parsed value can be converted back to string
        let _string_repr = parsed.to_string_value();

        // Verify Display implementation doesn't panic
        let _display = format!("{}", parsed);
    }
});
