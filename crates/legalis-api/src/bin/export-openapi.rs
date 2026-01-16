#!/usr/bin/env cargo
//! Export OpenAPI specification to JSON file

use legalis_api::openapi;
use std::fs;

fn main() {
    let spec = openapi::generate_spec();
    let json_str = serde_json::to_string_pretty(&spec).expect("Failed to serialize OpenAPI spec");

    fs::write("openapi.json", &json_str).expect("Failed to write openapi.json");

    println!("OpenAPI specification exported to openapi.json");
    println!("File size: {} bytes", json_str.len());
}
