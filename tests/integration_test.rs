use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn binary_with_no_args_prints_usage() {
    let mut cmd = Command::cargo_bin("rskey").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn binary_with_set_writes_correct_data_to_new_file() {
    let tmp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("rskey").unwrap();
    cmd.current_dir(&tmp_dir)
        .args(["set", "key1", "value1"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("rskey").unwrap();
    cmd.arg("list")
        .current_dir(&tmp_dir)
        .assert()
        .success()
        .stdout(predicate::eq("key1: value1\n"));
}

#[test]
fn binary_with_get_reads_existing_data() {
    let tmp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("rskey").unwrap();
    cmd.current_dir(&tmp_dir)
        .args(["set", "key2", "value2"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("rskey").unwrap();
    cmd.args(["get", "key2"])
        .current_dir(&tmp_dir)
        .assert()
        .success()
        .stdout(predicate::eq("key2: value2\n"));
}
