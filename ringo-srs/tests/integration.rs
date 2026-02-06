use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn ringo_srs() -> Command {
    Command::cargo_bin("ringo-srs").unwrap()
}

fn setup_empty(dir: &TempDir) -> std::path::PathBuf {
    let path = dir.path().join("learning-items.json");
    fs::write(
        &path,
        r#"{"version":"1.0","last_updated":"2026-02-04T12:00:00Z","items":[]}"#,
    )
    .unwrap();
    path
}

fn setup_with_item(dir: &TempDir) -> std::path::PathBuf {
    let path = dir.path().join("learning-items.json");
    fs::write(
        &path,
        r#"{
  "version": "1.0",
  "last_updated": "2026-02-04T12:00:00Z",
  "items": [
    {
      "id": "item_20260204_001",
      "type": "word",
      "front": "implement",
      "back": "実装する",
      "context": "We need to implement the feature.",
      "context_ja": "機能を実装する必要がある。",
      "source": "ringo-learning",
      "created_at": "2026-02-04T12:00:00Z",
      "last_quizzed": null,
      "next_review": "2020-01-01T00:00:00Z",
      "times_quizzed": 0,
      "times_correct": 0,
      "ease_factor": 2.5,
      "interval_days": 0,
      "status": "new"
    }
  ]
}"#,
    )
    .unwrap();
    path
}

#[test]
fn test_add_item() {
    let dir = TempDir::new().unwrap();
    let path = setup_empty(&dir);

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "add", "--front", "hello", "--back", "こんにちは"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""ok": true"#))
        .stdout(predicate::str::contains(r#""front": "hello""#))
        .stdout(predicate::str::contains(r#""type": "word""#));
}

#[test]
fn test_add_duplicate() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "add", "--front", "implement", "--back", "実装する"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("duplicate"));
}

#[test]
fn test_add_with_type() {
    let dir = TempDir::new().unwrap();
    let path = setup_empty(&dir);

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "add", "--front", "break the ice", "--back", "場を和ませる", "--type", "idiom"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""type": "idiom""#));
}

#[test]
fn test_add_auto_creates_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("subdir").join("learning-items.json");

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "add", "--front", "test", "--back", "テスト"])
        .assert()
        .success();

    assert!(path.exists());
}

#[test]
fn test_list_all() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""count": 1"#))
        .stdout(predicate::str::contains("implement"));
}

#[test]
fn test_list_due() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    // Item has next_review in the past, so it should be due
    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "list", "--due"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""count": 1"#));
}

#[test]
fn test_list_due_limit() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "list", "--due", "--limit", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""count": 0"#));
}

#[test]
fn test_list_no_data_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nonexistent.json");

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no_data_file"));
}

#[test]
fn test_review_single_correct() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args([
            "--data",
            path.to_str().unwrap(),
            "review",
            "item_20260204_001",
            "correct",
            "good",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""result": "correct""#))
        .stdout(predicate::str::contains(r#""correct": 1"#));
}

#[test]
fn test_review_single_incorrect() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args([
            "--data",
            path.to_str().unwrap(),
            "review",
            "item_20260204_001",
            "incorrect",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""incorrect": 1"#));
}

#[test]
fn test_review_prefix_match() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args([
            "--data",
            path.to_str().unwrap(),
            "review",
            "item_20260204",
            "correct",
            "easy",
        ])
        .assert()
        .success();
}

#[test]
fn test_review_not_found() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args([
            "--data",
            path.to_str().unwrap(),
            "review",
            "nonexistent",
            "correct",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not_found"));
}

#[test]
fn test_review_batch() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "review"])
        .write_stdin(r#"[{"id":"item_20260204_001","result":"correct","difficulty":"good"}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""correct": 1"#));
}

#[test]
fn test_stats() {
    let dir = TempDir::new().unwrap();
    let path = setup_with_item(&dir);

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "stats"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""total_items": 1"#))
        .stdout(predicate::str::contains(r#""due_now": 1"#));
}

#[test]
fn test_stats_no_data_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nonexistent.json");

    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "stats"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no_data_file"));
}

#[test]
fn test_full_flow() {
    let dir = TempDir::new().unwrap();
    let path = setup_empty(&dir);
    let data = path.to_str().unwrap();

    // 1. Add an item
    ringo_srs()
        .args(["--data", data, "add", "--front", "test word", "--back", "テスト単語"])
        .assert()
        .success();

    // 2. List due items
    ringo_srs()
        .args(["--data", data, "list", "--due", "--limit", "5"])
        .assert()
        .success();

    // 3. Stats
    ringo_srs()
        .args(["--data", data, "stats"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""total_items": 1"#));
}

#[test]
fn test_round_trip_preserves_extra_fields() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("learning-items.json");
    fs::write(
        &path,
        r#"{
  "version": "1.0",
  "last_updated": "2026-02-04T12:00:00Z",
  "items": [
    {
      "id": "item_001",
      "type": "word",
      "front": "hello",
      "back": "こんにちは",
      "created_at": "2026-02-04T12:00:00Z",
      "next_review": "2020-01-01T00:00:00Z",
      "ease_factor": 2.5,
      "interval_days": 0,
      "status": "new",
      "custom_field": "should_be_preserved"
    }
  ]
}"#,
    )
    .unwrap();

    // Do a review to trigger save
    ringo_srs()
        .args(["--data", path.to_str().unwrap(), "review", "item_001", "correct", "good"])
        .assert()
        .success();

    // Read back and check custom_field is preserved
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("custom_field"));
    assert!(content.contains("should_be_preserved"));
}
