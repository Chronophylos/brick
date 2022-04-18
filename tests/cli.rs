use assert_cmd::Command;
use predicates::prelude::*;

fn command() -> Command {
    Command::cargo_bin("brick").unwrap()
}

#[test]
fn command_exists() {
    command();
}

#[test]
fn requires_subcommand() {
    command()
        .assert()
        .append_context("main", "no args")
        .failure()
        .code(predicate::eq(2))
        .stderr(predicate::str::contains(
            "requires a subcommand but one was not provided",
        ));
}

#[test]
fn pack_without_args_displays_help() {
    command()
        .arg("pack")
        .assert()
        .append_context("pack", "no args")
        .failure()
        .code(predicate::eq(2))
        .stderr(predicate::str::contains("USAGE:"));
}
