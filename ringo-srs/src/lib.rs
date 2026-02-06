pub mod cli;
pub mod error;
pub mod models;
pub mod sm2;
pub mod storage;

use chrono::Utc;
use cli::Command;
use error::{success_json, AppError};
use models::{LearningItem, ReviewInput};
use std::collections::HashMap;
use std::io::Read;

/// Main dispatch: run CLI command, return JSON value for stdout.
pub fn run(cli: &cli::Cli) -> Result<serde_json::Value, AppError> {
    match &cli.command {
        Command::Add {
            front,
            back,
            item_type,
            context,
            context_ja,
            source,
        } => cmd_add(
            &cli.data,
            front,
            back,
            item_type.as_deref(),
            context.as_deref(),
            context_ja.as_deref(),
            source.as_deref(),
        ),
        Command::List { due, weak, status, limit } => {
            cmd_list(&cli.data, *due, *weak, status.as_deref(), *limit)
        }
        Command::Review { id, result, difficulty } => {
            cmd_review(&cli.data, id.as_deref(), result.as_deref(), difficulty.as_deref())
        }
        Command::Stats => cmd_stats(&cli.data),
    }
}

fn read_stdin() -> Result<String, AppError> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn cmd_add(
    data_path: &std::path::Path,
    front: &str,
    back: &str,
    item_type: Option<&str>,
    context: Option<&str>,
    context_ja: Option<&str>,
    source: Option<&str>,
) -> Result<serde_json::Value, AppError> {
    if front.trim().is_empty() || back.trim().is_empty() {
        return Err(AppError::InvalidInput("'front' and 'back' are required".to_string()));
    }

    let mut db = storage::load(data_path)?;

    // Duplicate check (case-insensitive on front)
    let front_lower = front.to_lowercase();
    if db.items.iter().any(|item| item.front.to_lowercase() == front_lower) {
        return Err(AppError::Duplicate(front.to_string()));
    }

    let now = Utc::now();
    let id = generate_id(&now, db.items.len());
    let resolved_type = item_type
        .map(String::from)
        .unwrap_or_else(|| auto_detect_type(front));

    let next_review = sm2::next_review(&now, 1.0); // Review tomorrow

    let item = LearningItem {
        id,
        item_type: resolved_type,
        front: front.to_string(),
        back: back.to_string(),
        context: context.map(String::from),
        context_ja: context_ja.map(String::from),
        source: source.map(String::from),
        created_at: now,
        last_quizzed: None,
        next_review,
        times_quizzed: 0,
        times_correct: 0,
        ease_factor: 2.5,
        interval_days: 0.0,
        status: "new".to_string(),
        extra: HashMap::new(),
    };

    db.items.push(item);
    let added = db.items.last().unwrap();
    let total = db.items.len();

    let response = success_json(serde_json::json!({
        "id": added.id,
        "front": added.front,
        "back": added.back,
        "type": added.item_type,
        "total_items": total,
    }));

    storage::save(data_path, &mut db)?;

    Ok(response)
}

fn cmd_list(
    data_path: &std::path::Path,
    due: bool,
    weak: bool,
    status_filter: Option<&str>,
    limit: Option<usize>,
) -> Result<serde_json::Value, AppError> {
    let db = storage::load_existing(data_path)?;
    let now = Utc::now();

    let mut items: Vec<&LearningItem> = if due {
        let mut due_items: Vec<_> = db.items.iter().filter(|i| i.next_review <= now).collect();
        // Sort: overdue first (earliest next_review), then lowest ease_factor
        due_items.sort_by(|a, b| {
            a.next_review
                .cmp(&b.next_review)
                .then(a.ease_factor.partial_cmp(&b.ease_factor).unwrap())
        });
        due_items
    } else if weak {
        db.items
            .iter()
            .filter(|i| {
                i.times_quizzed >= 2
                    && i.accuracy().map_or(false, |acc| acc < 70.0)
            })
            .collect()
    } else if let Some(st) = status_filter {
        db.items.iter().filter(|i| i.status == st).collect()
    } else {
        db.items.iter().collect()
    };

    if let Some(lim) = limit {
        items.truncate(lim);
    }

    let items_json: Vec<serde_json::Value> = items
        .iter()
        .map(|i| {
            serde_json::json!({
                "id": i.id,
                "front": i.front,
                "back": i.back,
                "type": i.item_type,
                "context": i.context,
                "context_ja": i.context_ja,
            })
        })
        .collect();

    Ok(success_json(serde_json::json!({
        "count": items_json.len(),
        "items": items_json,
    })))
}

fn cmd_review(
    data_path: &std::path::Path,
    id: Option<&str>,
    result: Option<&str>,
    difficulty: Option<&str>,
) -> Result<serde_json::Value, AppError> {
    if let (Some(item_id), Some(res)) = (id, result) {
        // Single-item mode
        review_single(data_path, item_id, res, difficulty)
    } else {
        // Batch mode: read JSON array from stdin
        review_batch(data_path)
    }
}

fn review_single(
    data_path: &std::path::Path,
    id_prefix: &str,
    result: &str,
    difficulty: Option<&str>,
) -> Result<serde_json::Value, AppError> {
    let inputs = vec![ReviewInput {
        id: id_prefix.to_string(),
        result: result.to_string(),
        difficulty: difficulty.map(String::from),
    }];
    review_items(data_path, &inputs)
}

fn review_batch(data_path: &std::path::Path) -> Result<serde_json::Value, AppError> {
    let input_str = read_stdin()?;
    let inputs: Vec<ReviewInput> = serde_json::from_str(&input_str)
        .map_err(|e| AppError::InvalidInput(format!("Invalid JSON: {e}")))?;
    if inputs.is_empty() {
        return Err(AppError::InvalidInput("Empty review array".to_string()));
    }
    review_items(data_path, &inputs)
}

fn review_items(
    data_path: &std::path::Path,
    inputs: &[ReviewInput],
) -> Result<serde_json::Value, AppError> {
    let mut db = storage::load_existing(data_path)?;
    let now = Utc::now();
    let mut results = Vec::new();
    let mut total_correct = 0u32;
    let mut total_incorrect = 0u32;

    for input in inputs {
        let item = find_item_by_prefix(&mut db.items, &input.id)?;

        let is_correct = input.result == "correct";
        let difficulty = input.difficulty.as_deref().unwrap_or("good");

        let (new_interval, new_ease) = if is_correct {
            total_correct += 1;
            sm2::review_correct(item.times_quizzed, item.interval_days, item.ease_factor, difficulty)
        } else {
            total_incorrect += 1;
            sm2::review_incorrect(item.ease_factor)
        };

        item.interval_days = new_interval;
        item.ease_factor = new_ease;
        item.times_quizzed += 1;
        if is_correct {
            item.times_correct += 1;
        }
        item.last_quizzed = Some(now);
        item.next_review = sm2::next_review(&now, new_interval);
        item.status = sm2::compute_status(
            item.interval_days,
            item.accuracy(),
            item.times_quizzed,
            &now,
            &item.next_review,
        );

        results.push(serde_json::json!({
            "id": item.id,
            "result": input.result,
            "next_review": item.next_review.to_rfc3339(),
            "interval_days": item.interval_days,
            "status": item.status,
        }));
    }

    storage::save(data_path, &mut db)?;

    Ok(success_json(serde_json::json!({
        "results": results,
        "summary": {
            "total": inputs.len(),
            "correct": total_correct,
            "incorrect": total_incorrect,
        },
    })))
}

fn cmd_stats(data_path: &std::path::Path) -> Result<serde_json::Value, AppError> {
    let db = storage::load_existing(data_path)?;
    let now = Utc::now();

    let total = db.items.len();
    let due_now = db.items.iter().filter(|i| i.next_review <= now).count();

    let total_quizzed: u32 = db.items.iter().map(|i| i.times_quizzed).sum();
    let total_correct: u32 = db.items.iter().map(|i| i.times_correct).sum();
    let accuracy_pct = if total_quizzed > 0 {
        (total_correct as f64 / total_quizzed as f64 * 1000.0).round() / 10.0
    } else {
        0.0
    };

    // By status
    let mut by_status: HashMap<&str, usize> = HashMap::new();
    for item in &db.items {
        *by_status.entry(item.status.as_str()).or_insert(0) += 1;
    }

    // By type
    let mut by_type: HashMap<&str, usize> = HashMap::new();
    for item in &db.items {
        *by_type.entry(item.item_type.as_str()).or_insert(0) += 1;
    }

    // Next due
    let next_due = db
        .items
        .iter()
        .filter(|i| i.next_review > now)
        .min_by_key(|i| i.next_review)
        .map(|i| i.next_review.to_rfc3339());

    Ok(success_json(serde_json::json!({
        "total_items": total,
        "due_now": due_now,
        "accuracy_pct": accuracy_pct,
        "by_status": {
            "new": by_status.get("new").unwrap_or(&0),
            "learning": by_status.get("learning").unwrap_or(&0),
            "reviewing": by_status.get("reviewing").unwrap_or(&0),
            "mastered": by_status.get("mastered").unwrap_or(&0),
        },
        "by_type": by_type,
        "next_due": next_due,
    })))
}

/// Find item by ID prefix. Returns mutable ref or error.
fn find_item_by_prefix<'a>(
    items: &'a mut [LearningItem],
    prefix: &str,
) -> Result<&'a mut LearningItem, AppError> {
    let matches: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(_, i)| i.id.starts_with(prefix))
        .map(|(idx, _)| idx)
        .collect();

    match matches.len() {
        0 => Err(AppError::NotFound(prefix.to_string())),
        1 => Ok(&mut items[matches[0]]),
        _ => Err(AppError::AmbiguousId(prefix.to_string())),
    }
}

fn generate_id(now: &chrono::DateTime<Utc>, existing_count: usize) -> String {
    let date = now.format("%Y%m%d");
    format!("item_{date}_{:03}", existing_count + 1)
}

fn auto_detect_type(front: &str) -> String {
    let word_count = front.split_whitespace().count();
    if word_count <= 1 {
        "word".to_string()
    } else {
        "phrase".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_detect_type_word() {
        assert_eq!(auto_detect_type("implement"), "word");
    }

    #[test]
    fn test_auto_detect_type_phrase() {
        assert_eq!(auto_detect_type("go shopping"), "phrase");
    }

    #[test]
    fn test_auto_detect_type_multi_word() {
        assert_eq!(auto_detect_type("break the ice"), "phrase");
    }

    #[test]
    fn test_generate_id() {
        use chrono::TimeZone;
        let now = Utc.with_ymd_and_hms(2026, 2, 6, 10, 0, 0).unwrap();
        assert_eq!(generate_id(&now, 15), "item_20260206_016");
    }
}
