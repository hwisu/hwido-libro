use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_message() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "A command-line book tracking tool",
    ));
}

#[test]
fn test_add_command() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.arg("add");
    cmd.assert()
        .failure() // Will fail due to no terminal for interactive input
        .stderr(predicate::str::contains("IO Error"));
}



#[test]
fn test_browse_command() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["browse"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_browse_with_query() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["browse", "test"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found matching"));
}

#[test]
fn test_report_books() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["report", "--books"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_report_reviews() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["report", "--reviews"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_report_authors() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["report", "--authors"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}



#[test]
fn test_report_with_years_flag() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["report", "--years"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_review_with_id() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["review", "5"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Book with ID 5 not found"));
}



#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}
