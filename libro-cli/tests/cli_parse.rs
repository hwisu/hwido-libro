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
fn test_brs_with_id() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["brs", "42"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_brs_with_year_flag() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["brs", "--year", "2023"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_brs_with_json_flag() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["brs", "--json"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_books_command() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["books"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_reviews_command() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["reviews"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No reviews found"));
}

#[test]
fn test_authors_command() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["authors"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No books found"));
}

#[test]
fn test_report_with_author_flag() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["report", "--author"]);
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
fn test_edit_review_with_id() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.args(&["edit-review", "10"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Book with ID 10 not found"));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("libro-cli").unwrap();
    cmd.arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}
