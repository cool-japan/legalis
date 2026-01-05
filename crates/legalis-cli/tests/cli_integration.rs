//! CLI command integration tests.
//!
//! These tests verify that all CLI commands work correctly end-to-end.

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to create a test statute file.
fn create_test_statute(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(name);
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}

/// Helper to get a simple valid statute DSL.
fn simple_statute_dsl() -> &'static str {
    r#"
STATUTE simple-test: "Simple Test Act" {
    WHEN AGE >= 18
    THEN GRANT "Adult rights"
}
"#
}

/// Helper to get a complex statute DSL.
fn complex_statute_dsl() -> &'static str {
    r#"
STATUTE complex-test: "Complex Test Act" {
    WHEN AGE >= 18 AND INCOME > 30000
    THEN GRANT "Complex rights"
    DISCRETION "Consider individual circumstances"
}
"#
}

#[test]
fn test_parse_command_text_output() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("parse")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("simple-test"));
}

#[test]
fn test_parse_command_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--format")
        .arg("json")
        .arg("parse")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("simple-test"));
}

#[test]
fn test_verify_command_success() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("verify")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .assert()
        .success();
}

#[test]
fn test_verify_command_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let statute1 = create_test_statute(&temp_dir, "test1.leg", simple_statute_dsl());
    let statute2 = create_test_statute(&temp_dir, "test2.leg", complex_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("verify")
        .arg("-i")
        .arg(statute1.to_str().unwrap())
        .arg("-i")
        .arg(statute2.to_str().unwrap())
        .assert()
        .success();
}

#[test]
fn test_verify_command_strict_mode() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("verify")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .arg("--strict")
        .assert()
        .success();
}

#[test]
fn test_viz_command_mermaid() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());
    let output_file = temp_dir.path().join("output.mmd");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("viz")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .arg("--viz-format")
        .arg("mermaid")
        .assert()
        .success();

    assert!(output_file.exists());
    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("flowchart TD"));
}

#[test]
fn test_viz_command_dot() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());
    let output_file = temp_dir.path().join("output.dot");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("viz")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .arg("--viz-format")
        .arg("dot")
        .assert()
        .success();

    assert!(output_file.exists());
}

#[test]
fn test_viz_command_ascii() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());
    let output_file = temp_dir.path().join("output.txt");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("viz")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .arg("--viz-format")
        .arg("ascii")
        .assert()
        .success();

    assert!(output_file.exists());
}

#[test]
fn test_export_command_json() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());
    let output_file = temp_dir.path().join("output.json");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("export")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .arg("--export-format")
        .arg("json")
        .assert()
        .success();

    assert!(output_file.exists());
    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("simple-test"));
}

#[test]
fn test_export_command_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());
    let output_file = temp_dir.path().join("output.yaml");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("export")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .arg("--export-format")
        .arg("yaml")
        .assert()
        .success();

    assert!(output_file.exists());
}

#[test]
fn test_init_command() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_project");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("init")
        .arg(project_path.to_str().unwrap())
        .assert()
        .success();

    // Verify project structure was created
    assert!(project_path.exists());
    assert!(project_path.join("legalis.yaml").exists());
    assert!(project_path.join("statutes").exists());
    assert!(project_path.join("output").exists());
    assert!(project_path.join("statutes/sample.legal").exists());
}

#[test]
fn test_init_command_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test_project_dry");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("init")
        .arg(project_path.to_str().unwrap())
        .arg("--dry-run")
        .assert()
        .success();

    // With dry-run, nothing should be created
    assert!(!project_path.exists());
}

#[test]
fn test_diff_command() {
    let temp_dir = TempDir::new().unwrap();

    let old_dsl = r#"
STATUTE diff-test: "Old Version" {
    WHEN AGE >= 18
    THEN GRANT "Basic rights"
}
"#;

    let new_dsl = r#"
STATUTE diff-test: "New Version" {
    WHEN AGE >= 21
    THEN GRANT "Extended rights"
}
"#;

    let old_file = create_test_statute(&temp_dir, "old.leg", old_dsl);
    let new_file = create_test_statute(&temp_dir, "new.leg", new_dsl);

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("diff")
        .arg("--old")
        .arg(old_file.to_str().unwrap())
        .arg("--new")
        .arg(new_file.to_str().unwrap())
        .assert()
        .success();
}

#[test]
fn test_complexity_command() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", complex_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("complexity")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Complexity"));
}

#[test]
fn test_format_command() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());
    let output_file = temp_dir.path().join("formatted.leg");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("format")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .assert()
        .success();

    assert!(output_file.exists());
}

#[test]
fn test_format_command_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("format")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .arg("--dry-run")
        .assert()
        .success();
}

#[test]
fn test_lint_command() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("lint")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .assert()
        .success();
}

#[test]
fn test_new_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("new_statute.leg");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("new")
        .arg("-n")
        .arg("test-statute")
        .arg("-t")
        .arg("basic")
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .assert()
        .success();

    assert!(output_file.exists());
    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("test-statute"));
}

#[test]
fn test_new_command_income_template() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("income_statute.leg");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("new")
        .arg("-n")
        .arg("income-test")
        .arg("-t")
        .arg("income")
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .assert()
        .success();

    assert!(output_file.exists());
    let content = fs::read_to_string(output_file).unwrap();
    assert!(content.contains("INCOME"));
}

#[test]
fn test_doctor_command() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Legalis"));
}

#[test]
fn test_doctor_command_verbose() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("doctor").arg("--verbose").assert().success();
}

#[test]
fn test_completions_command() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("completions")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("legalis"));
}

#[test]
fn test_man_page_command() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("man-page")
        .assert()
        .success()
        .stdout(predicate::str::contains("legalis"));
}

#[test]
fn test_man_page_command_with_output_dir() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("man-page")
        .arg("-o")
        .arg(temp_dir.path().to_str().unwrap())
        .assert()
        .success();

    // Check that man pages were created
    assert!(temp_dir.path().join("legalis.1").exists());
}

#[test]
fn test_quiet_mode() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--quiet")
        .arg("verify")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .assert()
        .success();
}

#[test]
fn test_verbose_mode() {
    let temp_dir = TempDir::new().unwrap();
    let statute_file = create_test_statute(&temp_dir, "test.leg", simple_statute_dsl());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("-v")
        .arg("verify")
        .arg("-i")
        .arg(statute_file.to_str().unwrap())
        .assert()
        .success();
}

#[test]
fn test_help_command() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Legalis"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("legalis"));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("invalid-command").assert().failure();
}

#[test]
fn test_parse_missing_input() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("parse").assert().failure();
}

#[test]
fn test_parse_nonexistent_file() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("parse")
        .arg("-i")
        .arg("/nonexistent/file.leg")
        .assert()
        .failure();
}
