//! End-to-end integration tests for the enterprise, UX, and self-healing
//! commands wired into the CLI (audit-log, policy, central-config, assistant,
//! diagnose, repair, recover) plus the cross-cutting policy/compliance gating.
//!
//! Each test isolates CLI state via `LEGALIS_DATA_DIR` pointed at a fresh
//! temporary directory, and disables color with `NO_COLOR` for stable matching.

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;
use std::path::PathBuf;
use std::process::Command;

/// A unique temporary data directory for an isolated CLI run.
fn temp_data_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("legalis-ent-it-{tag}-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).expect("create data dir");
    dir
}

/// Builds a `legalis` command pre-configured with an isolated data dir and no
/// color, so the policy file under `<data_dir>/policy.toml` is also discovered.
fn legalis(data_dir: &PathBuf) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("legalis"));
    cmd.env("LEGALIS_DATA_DIR", data_dir);
    cmd.env("NO_COLOR", "1");
    // Ensure no stray policy/compliance env from the host leaks in.
    cmd.env_remove("LEGALIS_POLICY_FILE");
    cmd.env_remove("LEGALIS_COMPLIANCE");
    cmd.env_remove("LEGALIS_CENTRAL_CONFIG");
    cmd
}

#[test]
fn test_diagnose_runs_and_reports() {
    let dir = temp_data_dir("diagnose");
    legalis(&dir)
        .arg("diagnose")
        .assert()
        .success()
        .stdout(predicate::str::contains("data-dir"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_diagnose_json_format() {
    let dir = temp_data_dir("diagnose-json");
    legalis(&dir)
        .args(["--format", "json", "diagnose"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"checks\""));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_policy_init_then_show() {
    let dir = temp_data_dir("policy");
    legalis(&dir)
        .args(["policy", "init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Starter policy"));
    legalis(&dir)
        .args(["policy", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("enterprise"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_policy_denies_command() {
    let dir = temp_data_dir("policy-deny");
    std::fs::write(
        dir.join("policy.toml"),
        "name = \"corp\"\ndenied_commands = [\"clean\"]\n",
    )
    .expect("write policy");

    // The denied command must fail with a clear message.
    legalis(&dir)
        .args(["clean", "--all"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Blocked by enterprise policy"));

    // A non-denied command must still run.
    legalis(&dir)
        .args(["list", "-d", "/tmp/does-not-exist-statutes"])
        .assert()
        .success();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_policy_allow_list_blocks_others() {
    let dir = temp_data_dir("policy-allow");
    std::fs::write(
        dir.join("policy.toml"),
        "name = \"locked\"\nallowed_commands = [\"diagnose\", \"policy\"]\n",
    )
    .expect("write policy");

    // 'diagnose' is allowed.
    legalis(&dir).arg("diagnose").assert().success();

    // 'list' is not in the allow-list -> blocked.
    legalis(&dir)
        .args(["list", "-d", "/tmp/x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("allow-list"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_policy_check_limit_enforced() {
    let dir = temp_data_dir("policy-limit");
    std::fs::write(
        dir.join("policy.toml"),
        "name = \"capped\"\n[limits]\nmax_population = 500\n",
    )
    .expect("write policy");

    legalis(&dir)
        .args(["policy", "check-limit", "-k", "population", "-v", "400"])
        .assert()
        .success();

    legalis(&dir)
        .args(["policy", "check-limit", "-k", "population", "-v", "600"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("limits population to 500"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_compliance_blocks_unconfirmed_sensitive() {
    let dir = temp_data_dir("compliance");
    legalis(&dir)
        .args(["--compliance", "publish", "-i", "/tmp/nope.ldsl"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Blocked by compliance mode"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_compliance_allows_confirmed_sensitive() {
    let dir = temp_data_dir("compliance-ok");
    // With --dry-run (treated as confirmation), the sensitive publish is not
    // blocked by compliance; it then fails for a *different* reason (missing
    // file), which proves the compliance gate let it through.
    legalis(&dir)
        .args([
            "--compliance",
            "publish",
            "-i",
            "/tmp/nope.ldsl",
            "--dry-run",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Blocked by compliance mode").not());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_audit_log_records_and_verifies() {
    let dir = temp_data_dir("audit");
    // Run a couple of harmless commands to generate audit records.
    let _ = legalis(&dir).arg("diagnose").assert();
    let _ = legalis(&dir).args(["assistant", "stats"]).assert();

    // The audit log must verify clean and contain records.
    legalis(&dir)
        .args(["audit-log", "verify"])
        .assert()
        .success()
        .stdout(predicate::str::contains("intact"));

    legalis(&dir)
        .args(["audit-log", "show", "--limit", "5"])
        .assert()
        .success();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_assistant_learns_and_suggests() {
    let dir = temp_data_dir("assistant");
    // Teach the assistant a verify -> publish pattern.
    for _ in 0..3 {
        let _ = legalis(&dir)
            .args(["assistant", "record", "-c", "verify"])
            .assert();
        let _ = legalis(&dir)
            .args(["assistant", "record", "-c", "publish"])
            .assert();
    }
    legalis(&dir)
        .args(["assistant", "suggest", "--previous", "verify"])
        .assert()
        .success()
        .stdout(predicate::str::contains("publish"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_central_config_validate() {
    let dir = temp_data_dir("central");
    legalis(&dir)
        .args(["central-config", "validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_central_config_show_provenance() {
    let dir = temp_data_dir("central-show");
    legalis(&dir)
        .args(["--format", "json", "central-config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("default"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_repair_fixes_invalid_config() {
    let dir = temp_data_dir("repair");
    let cfg = dir.join("legalis.toml");
    std::fs::write(&cfg, "[output]\nformat = \"bogus\"\n").expect("write config");

    legalis(&dir)
        .args(["repair", "--config"])
        .arg(&cfg)
        .assert()
        .success()
        .stdout(predicate::str::contains("output.format"));

    let fixed = std::fs::read_to_string(&cfg).expect("read");
    assert!(fixed.contains("text"), "format should be reset to text");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_repair_dry_run_keeps_config() {
    let dir = temp_data_dir("repair-dry");
    let cfg = dir.join("legalis.toml");
    std::fs::write(&cfg, "[output]\nformat = \"bogus\"\n").expect("write config");

    legalis(&dir)
        .args(["repair", "--dry-run", "--config"])
        .arg(&cfg)
        .assert()
        .success();

    let unchanged = std::fs::read_to_string(&cfg).expect("read");
    assert!(
        unchanged.contains("bogus"),
        "dry-run must not modify the file"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_recover_list_empty() {
    let dir = temp_data_dir("recover");
    legalis(&dir)
        .args(["recover", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No resumable operations"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_verbosity_flag_accepted() {
    let dir = temp_data_dir("verbosity");
    // An explicit --verbosity should be accepted and influence detail; we only
    // assert the command still succeeds with the flag present.
    legalis(&dir)
        .args(["--verbosity", "verbose", "diagnose"])
        .assert()
        .success();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_high_contrast_theme_flag_accepted() {
    let dir = temp_data_dir("theme");
    legalis(&dir)
        .args(["--theme", "high-contrast", "diagnose"])
        .assert()
        .success();
    let _ = std::fs::remove_dir_all(&dir);
}
