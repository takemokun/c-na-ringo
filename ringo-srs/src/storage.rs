use crate::error::AppError;
use crate::models::SrsDatabase;
use crate::sm2;
use chrono::Utc;
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

/// Load database from JSON file. Returns empty database if file doesn't exist.
pub fn load(path: &Path) -> Result<SrsDatabase, AppError> {
    if !path.exists() {
        return Ok(SrsDatabase::new());
    }
    let content = fs::read_to_string(path)?;
    let mut db: SrsDatabase = serde_json::from_str(&content)?;
    self_heal(&mut db);
    Ok(db)
}

/// Load database, returning error if the file doesn't exist.
pub fn load_existing(path: &Path) -> Result<SrsDatabase, AppError> {
    if !path.exists() {
        return Err(AppError::NoDataFile(path.display().to_string()));
    }
    let content = fs::read_to_string(path)?;
    let mut db: SrsDatabase = serde_json::from_str(&content)?;
    self_heal(&mut db);
    Ok(db)
}

/// Atomic write: write to tempfile, backup existing, then persist.
pub fn save(path: &Path, db: &mut SrsDatabase) -> Result<(), AppError> {
    db.last_updated = Some(Utc::now());

    let json = serde_json::to_string_pretty(db)?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Backup existing file
    if path.exists() {
        let bak = path.with_extension("json.bak");
        fs::copy(path, &bak)?;
    }

    // Atomic write via tempfile
    let dir = path.parent().unwrap_or(Path::new("."));
    let tmp = NamedTempFile::new_in(dir)?;
    fs::write(tmp.path(), json.as_bytes())?;
    tmp.persist(path).map_err(|e| AppError::IoError(e.error))?;

    Ok(())
}

/// Validate and self-heal database items.
fn self_heal(db: &mut SrsDatabase) {
    let now = Utc::now();
    for item in &mut db.items {
        // Clamp ease_factor to minimum 1.3
        if item.ease_factor < 1.3 {
            item.ease_factor = 1.3;
        }
        // Ensure times_correct <= times_quizzed
        if item.times_correct > item.times_quizzed {
            item.times_correct = item.times_quizzed;
        }
        // Recompute status (self-healing)
        item.status = sm2::compute_status(item.interval_days, item.accuracy(), item.times_quizzed, &now, &item.next_review);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::LearningItem;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn sample_item() -> LearningItem {
        LearningItem {
            id: "test_001".to_string(),
            item_type: "word".to_string(),
            front: "test".to_string(),
            back: "テスト".to_string(),
            context: None,
            context_ja: None,
            source: None,
            created_at: Utc::now(),
            last_quizzed: None,
            next_review: Utc::now(),
            times_quizzed: 0,
            times_correct: 0,
            ease_factor: 2.5,
            interval_days: 0.0,
            status: "new".to_string(),
            extra: HashMap::new(),
        }
    }

    #[test]
    fn test_round_trip() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.json");

        let mut db = SrsDatabase::new();
        db.items.push(sample_item());
        save(&path, &mut db).unwrap();

        let loaded = load(&path).unwrap();
        assert_eq!(loaded.items.len(), 1);
        assert_eq!(loaded.items[0].front, "test");
    }

    #[test]
    fn test_load_nonexistent_returns_empty() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("nonexistent.json");
        let db = load(&path).unwrap();
        assert_eq!(db.items.len(), 0);
    }

    #[test]
    fn test_load_existing_nonexistent_returns_error() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("nonexistent.json");
        let result = load_existing(&path);
        assert!(matches!(result, Err(AppError::NoDataFile(_))));
    }

    #[test]
    fn test_backup_created() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.json");

        let mut db = SrsDatabase::new();
        db.items.push(sample_item());
        save(&path, &mut db).unwrap();

        // Save again — should create .bak
        save(&path, &mut db).unwrap();

        let bak = path.with_extension("json.bak");
        assert!(bak.exists());
    }

    #[test]
    fn test_self_heal_ease_factor() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.json");

        let mut item = sample_item();
        item.ease_factor = 0.5; // Invalid
        let mut db = SrsDatabase::new();
        db.items.push(item);
        save(&path, &mut db).unwrap();

        let loaded = load(&path).unwrap();
        assert!(loaded.items[0].ease_factor >= 1.3);
    }

    #[test]
    fn test_self_heal_times_correct() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.json");

        let mut item = sample_item();
        item.times_quizzed = 3;
        item.times_correct = 5; // Invalid: more correct than quizzed
        let mut db = SrsDatabase::new();
        db.items.push(item);
        save(&path, &mut db).unwrap();

        let loaded = load(&path).unwrap();
        assert_eq!(loaded.items[0].times_correct, loaded.items[0].times_quizzed);
    }

    #[test]
    fn test_round_trip_existing_data() {
        // Test with a structure matching the real data format
        let json = r#"{
  "version": "1.0",
  "last_updated": "2026-02-04T12:00:00Z",
  "items": [
    {
      "id": "item_20260204_120001_001",
      "type": "grammar",
      "front": "apparently の位置（文頭・文中どちらもOK）",
      "back": "Apparently, he is busy. / He is apparently busy. どちらも正しい",
      "context": "Apparently, he is busy. = He is apparently busy.",
      "context_ja": "どうやら彼は忙しいらしい（どちらの語順も可）",
      "source": "ringo-explain",
      "created_at": "2026-02-04T12:00:01Z",
      "last_quizzed": null,
      "next_review": "2026-02-05T12:00:00Z",
      "times_quizzed": 0,
      "times_correct": 0,
      "ease_factor": 2.5,
      "interval_days": 0,
      "status": "new"
    }
  ]
}"#;
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.json");
        fs::write(&path, json).unwrap();

        let mut db = load(&path).unwrap();
        assert_eq!(db.items.len(), 1);
        assert_eq!(db.items[0].item_type, "grammar");

        save(&path, &mut db).unwrap();

        // Verify it can be loaded again
        let reloaded = load(&path).unwrap();
        assert_eq!(reloaded.items.len(), 1);
        assert_eq!(reloaded.items[0].id, "item_20260204_120001_001");
        assert_eq!(reloaded.items[0].item_type, "grammar");
    }
}
