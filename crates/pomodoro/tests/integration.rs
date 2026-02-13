use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_start_work_session_default() {
    cargo_bin_cmd!()
        .args(["start"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Starting a Work session for 1500 seconds",
        ));
}

#[test]
fn test_start_break_session() {
    cargo_bin_cmd!()
        .args(["start", "--mode", "break"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Break session for 300 seconds"));
}

#[test]
fn test_start_custom_duration() {
    cargo_bin_cmd!()
        .args(["start", "--duration", "10m"])
        .assert()
        .success()
        .stdout(predicate::str::contains("600 seconds"));
}

#[test]
fn test_stop_command() {
    cargo_bin_cmd!()
        .args(["stop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stopping the pomodoro timer"));
}

#[test]
fn test_stop_with_reset() {
    cargo_bin_cmd!()
        .args(["stop", "--reset"])
        .assert()
        .success()
        .stdout(predicate::str::contains("resetting"));
}

#[test]
fn test_status_command() {
    cargo_bin_cmd!()
        .args(["status"])
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
