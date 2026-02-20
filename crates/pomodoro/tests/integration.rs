use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_start_work_session_default() {
    cargo_bin_cmd!()
        .args(["--in-memory", "--no-hooks", "start"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Started a new focus session."));
}

#[test]
fn test_start_break_session() {
    cargo_bin_cmd!()
        .args(["--in-memory", "--no-hooks", "start", "--mode", "break"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Started a new break session."));
}

#[test]
fn test_start_custom_duration() {
    cargo_bin_cmd!()
        .args(["--in-memory", "--no-hooks", "start", "--duration", "10m"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Started a new focus session."));
}

#[test]
fn test_stop_command() {
    cargo_bin_cmd!()
        .args(["--in-memory", "--no-hooks", "stop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No active session found."));
}

#[test]
fn test_stop_with_reset() {
    cargo_bin_cmd!()
        .args(["--in-memory", "--no-hooks", "stop", "--reset"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No active session found."));
}

#[test]
fn test_status_command() {
    cargo_bin_cmd!()
        .args(["--in-memory", "status"])
        .assert()
        .success();
}

#[test]
fn test_help_flag() {
    cargo_bin_cmd!()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A simple pomodoro timer"));
}

#[test]
fn test_start_with_no_hooks() {
    cargo_bin_cmd!()
        .args(["--in-memory", "--no-hooks", "start"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Started a new focus session."));
}

#[test]
fn test_stop_with_no_hooks() {
    cargo_bin_cmd!()
        .args(["--in-memory", "--no-hooks", "stop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No active session found."));
}

#[test]
fn test_status_with_no_hooks() {
    cargo_bin_cmd!()
        .args(["--in-memory", "--no-hooks", "status"])
        .assert()
        .success();
}
