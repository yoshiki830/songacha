use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::tempdir;

#[test]
fn pull_command_draws_songs_and_saves_history() {
    let temp_dir = tempdir().unwrap();
    let save_file = temp_dir.path().join("save.json");

    let mut command = Command::cargo_bin("songacha").unwrap();
    command
        .args([
            "--save-file",
            save_file.to_str().unwrap(),
            "pull",
            "--count",
            "3",
            "--seed",
            "42",
        ])
        .assert()
        .success()
        .stdout(contains("Gacha result"))
        .stdout(contains("Collection:"));

    assert!(save_file.exists());
}

#[test]
fn collection_command_shows_progress() {
    let temp_dir = tempdir().unwrap();
    let save_file = temp_dir.path().join("save.json");

    let mut pull_command = Command::cargo_bin("songacha").unwrap();
    pull_command
        .args([
            "--save-file",
            save_file.to_str().unwrap(),
            "pull",
            "--count",
            "2",
            "--seed",
            "42",
        ])
        .assert()
        .success();

    let mut collection_command = Command::cargo_bin("songacha").unwrap();
    collection_command
        .args(["--save-file", save_file.to_str().unwrap(), "collection"])
        .assert()
        .success()
        .stdout(contains("Collection:"))
        .stdout(contains("Collected songs"));
}

#[test]
fn reset_command_removes_save_file() {
    let temp_dir = tempdir().unwrap();
    let save_file = temp_dir.path().join("save.json");

    fs::write(&save_file, r#"{"pull_counts":{"1":1}}"#).unwrap();

    let mut command = Command::cargo_bin("songacha").unwrap();
    command
        .args(["--save-file", save_file.to_str().unwrap(), "reset"])
        .assert()
        .success()
        .stdout(contains("Reset local save data."));

    assert!(!save_file.exists());
}
