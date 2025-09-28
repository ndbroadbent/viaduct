use std::{fs, path::PathBuf};

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn invalid_fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures_invalid")
}

#[test]
fn via_gen_writes_outputs() -> Result<()> {
    let tmp = tempdir()?;
    let out_dir = tmp.path().join("generated");
    let ir_path = tmp.path().join("via.ir.json");

    Command::cargo_bin("via")?
        .arg("gen")
        .arg("--app")
        .arg(fixtures_dir())
        .arg("--out")
        .arg(&out_dir)
        .arg("--ir")
        .arg(&ir_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Parsed 1 resource(s)"));

    let model_file = out_dir.join("src/models/article.rs");
    let controller_file = out_dir.join("src/controllers/article.rs");
    let ts_file = out_dir.join("ts/models/article.ts");

    assert!(model_file.exists(), "expected generated model file to exist");
    assert!(controller_file.exists(), "expected generated controller file to exist");
    assert!(ts_file.exists(), "expected generated TypeScript file to exist");

    let ir = fs::read_to_string(&ir_path)?;
    assert!(ir.contains("Article"), "IR should contain resource name");

    Ok(())
}

#[test]
fn via_gen_outputs_cargo_check() -> Result<()> {
    let tmp = tempdir()?;
    let crate_dir = tmp.path().join("generated");

    Command::cargo_bin("via")?
        .arg("gen")
        .arg("--app")
        .arg(fixtures_dir())
        .arg("--out")
        .arg(&crate_dir)
        .assert()
        .success();

    Command::new("cargo")
        .current_dir(&crate_dir)
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_TARGET_DIR", tmp.path().join("target"))
        .arg("check")
        .assert()
        .success()
        .stderr(predicate::str::contains("Finished"));

    Ok(())
}

#[test]
fn via_gen_dry_run_lists_resources_without_writing_files() -> Result<()> {
    let tmp = tempdir()?;
    let out_dir = tmp.path().join("generated");

    Command::cargo_bin("via")?
        .arg("gen")
        .arg("--app")
        .arg(fixtures_dir())
        .arg("--out")
        .arg(&out_dir)
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Parsed 1 resource(s)"))
        .stdout(predicate::str::contains(" - Article"));

    assert!(!out_dir.exists(), "dry run should not create output directory");

    Ok(())
}

#[test]
fn via_check_reports_success() -> Result<()> {
    Command::cargo_bin("via")?
        .arg("check")
        .arg("--app")
        .arg(fixtures_dir())
        .assert()
        .success()
        .stdout(predicate::str::contains("OK: parsed 1 resource(s)"));

    Ok(())
}

#[test]
fn via_check_surfaces_parse_errors() -> Result<()> {
    Command::cargo_bin("via")?
        .arg("check")
        .arg("--app")
        .arg(invalid_fixtures_dir())
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing_colon.via"));

    Ok(())
}
