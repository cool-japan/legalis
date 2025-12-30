//! CLI contract tests.
//!
//! These tests ensure that the CLI interface remains stable and backwards compatible.
//! They verify that command structure, flags, and behavior remain consistent across versions.

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;
use std::process::Command;

/// Test that the main command exists and works.
#[test]
fn contract_main_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Legalis"));
}

/// Test that --version flag exists and works.
#[test]
fn contract_version_flag_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("legalis"));
}

/// Test that global verbose flag exists.
#[test]
fn contract_verbose_flag_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("-v").arg("--help").assert().success();
}

/// Test that global quiet flag exists.
#[test]
fn contract_quiet_flag_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--quiet").arg("--help").assert().success();
}

/// Test that global format flag exists.
#[test]
fn contract_format_flag_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--format")
        .arg("json")
        .arg("--help")
        .assert()
        .success();
}

/// Test that global config flag exists.
#[test]
fn contract_config_flag_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--config")
        .arg("test.toml")
        .arg("--help")
        .assert()
        .success();
}

/// Test that global interactive flag exists.
#[test]
fn contract_interactive_flag_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--interactive").arg("--help").assert().success();
}

// === Parse Command Contract ===

#[test]
fn contract_parse_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("parse")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Parse a legal DSL file"));
}

#[test]
fn contract_parse_has_input_flag() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("parse")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("-i, --input"));
}

#[test]
fn contract_parse_has_output_flag() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("parse")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("-o, --output"));
}

// === Verify Command Contract ===

#[test]
fn contract_verify_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("verify")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Verify statutes"));
}

#[test]
fn contract_verify_has_input_flag() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("verify")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("-i, --input"));
}

#[test]
fn contract_verify_has_strict_flag() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("verify")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--strict"));
}

// === Viz Command Contract ===

#[test]
fn contract_viz_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("viz")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate visualization"));
}

#[test]
fn contract_viz_has_format_options() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("viz")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--viz-format"))
        .stdout(predicate::str::contains("mermaid"))
        .stdout(predicate::str::contains("dot"));
}

// === Export Command Contract ===

#[test]
fn contract_export_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("export")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Export statute"));
}

#[test]
fn contract_export_has_format_options() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("export")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--export-format"))
        .stdout(predicate::str::contains("json"))
        .stdout(predicate::str::contains("yaml"));
}

// === Init Command Contract ===

#[test]
fn contract_init_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("init")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialize"));
}

#[test]
fn contract_init_has_dry_run_flag() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("init")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--dry-run"));
}

// === Diff Command Contract ===

#[test]
fn contract_diff_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("diff")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Compare two statute files"));
}

#[test]
fn contract_diff_has_old_new_flags() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("diff")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--old"))
        .stdout(predicate::str::contains("--new"));
}

// === Format Command Contract ===

#[test]
fn contract_format_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("format")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Format"));
}

#[test]
fn contract_format_has_inplace_flag() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("format")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--inplace"));
}

// === Lint Command Contract ===

#[test]
fn contract_lint_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("lint")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Lint DSL files"));
}

#[test]
fn contract_lint_has_fix_flag() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("lint")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--fix"));
}

// === New Command Contract ===

#[test]
fn contract_new_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("new")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Create a new statute"));
}

#[test]
fn contract_new_has_template_flag() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("new")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("-t, --template"));
}

#[test]
fn contract_new_has_template_options() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("new")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("basic"))
        .stdout(predicate::str::contains("income"))
        .stdout(predicate::str::contains("complex"));
}

// === Doctor Command Contract ===

#[test]
fn contract_doctor_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("doctor")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("diagnostics"));
}

// === Completions Command Contract ===

#[test]
fn contract_completions_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("completions")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("shell completions"));
}

#[test]
fn contract_completions_has_shell_options() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("completions")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("bash"));
}

// === REPL Command Contract ===

#[test]
fn contract_repl_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("repl")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("REPL"));
}

// === Tutorial Command Contract ===

#[test]
fn contract_tutorial_command_exists() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("tutorial")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("tutorial"));
}

#[test]
fn contract_tutorial_has_topic_option() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("tutorial")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("-t, --topic"));
}

// === Output Format Contract ===

#[test]
fn contract_format_supports_json() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--format")
        .arg("json")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn contract_format_supports_yaml() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--format")
        .arg("yaml")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn contract_format_supports_text() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--format")
        .arg("text")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn contract_format_supports_table() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--format")
        .arg("table")
        .arg("--help")
        .assert()
        .success();
}

// === Error Handling Contract ===

#[test]
fn contract_invalid_command_fails() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("nonexistent-command").assert().failure();
}

#[test]
fn contract_invalid_flag_fails() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.arg("--nonexistent-flag").assert().failure();
}

// === Subcommand Existence Contract ===

#[test]
fn contract_all_expected_subcommands_exist() {
    let expected_commands = vec![
        "parse",
        "verify",
        "viz",
        "export",
        "serve",
        "init",
        "diff",
        "simulate",
        "audit",
        "complexity",
        "port",
        "import",
        "convert",
        "completions",
        "man-page",
        "lod",
        "format",
        "lint",
        "watch",
        "test",
        "new",
        "doctor",
        "repl",
        "search",
        "publish",
        "validate",
        "install",
        "list",
        "add",
        "update",
        "clean",
        "outdated",
        "uninstall",
        "tutorial",
    ];

    for command in expected_commands {
        let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
        cmd.arg(command).arg("--help").assert().success();
    }
}
