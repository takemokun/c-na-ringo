use chrono::{DateTime, Duration, Utc};

/// Apply SM-2 algorithm for a correct answer.
/// Returns (new_interval_days, new_ease_factor).
pub fn review_correct(
    times_quizzed: u32,
    interval_days: f64,
    ease_factor: f64,
    difficulty: &str,
) -> (f64, f64) {
    let new_interval = if times_quizzed == 0 {
        1.0
    } else if times_quizzed == 1 {
        3.0
    } else {
        interval_days * ease_factor
    };

    let new_ease = match difficulty {
        "easy" => ease_factor + 0.15,
        "hard" => (ease_factor - 0.15).max(1.3),
        _ => ease_factor, // "good" or anything else
    };

    (new_interval, new_ease)
}

/// Apply SM-2 algorithm for an incorrect answer.
/// Returns (new_interval_days, new_ease_factor).
pub fn review_incorrect(ease_factor: f64) -> (f64, f64) {
    let new_ease = (ease_factor - 0.2).max(1.3);
    (1.0, new_ease)
}

/// Compute next_review datetime from now + interval_days.
pub fn next_review(now: &DateTime<Utc>, interval_days: f64) -> DateTime<Utc> {
    let secs = (interval_days * 86400.0) as i64;
    *now + Duration::seconds(secs)
}

/// Compute status based on current state. Used for self-healing.
pub fn compute_status(
    interval_days: f64,
    accuracy: Option<f64>,
    times_quizzed: u32,
    _now: &DateTime<Utc>,
    _next_review: &DateTime<Utc>,
) -> String {
    if interval_days == 0.0 {
        "new".to_string()
    } else if interval_days >= 30.0
        && accuracy.unwrap_or(0.0) >= 90.0
        && times_quizzed >= 5
    {
        "mastered".to_string()
    } else if interval_days >= 7.0 {
        "reviewing".to_string()
    } else {
        "learning".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_correct() {
        let (interval, ease) = review_correct(0, 0.0, 2.5, "good");
        assert_eq!(interval, 1.0);
        assert_eq!(ease, 2.5);
    }

    #[test]
    fn test_second_correct() {
        let (interval, ease) = review_correct(1, 1.0, 2.5, "good");
        assert_eq!(interval, 3.0);
        assert_eq!(ease, 2.5);
    }

    #[test]
    fn test_third_correct() {
        let (interval, ease) = review_correct(2, 3.0, 2.5, "good");
        assert_eq!(interval, 7.5);
        assert_eq!(ease, 2.5);
    }

    #[test]
    fn test_correct_easy() {
        let (interval, ease) = review_correct(2, 3.0, 2.5, "easy");
        assert_eq!(interval, 7.5);
        assert_eq!(ease, 2.65);
    }

    #[test]
    fn test_correct_hard() {
        let (interval, ease) = review_correct(2, 3.0, 2.5, "hard");
        assert_eq!(interval, 7.5);
        assert_eq!(ease, 2.35);
    }

    #[test]
    fn test_correct_hard_ease_floor() {
        let (_, ease) = review_correct(2, 3.0, 1.3, "hard");
        assert_eq!(ease, 1.3);
    }

    #[test]
    fn test_incorrect() {
        let (interval, ease) = review_incorrect(2.5);
        assert_eq!(interval, 1.0);
        assert!((ease - 2.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_incorrect_ease_floor() {
        let (_, ease) = review_incorrect(1.3);
        assert_eq!(ease, 1.3);
    }

    #[test]
    fn test_incorrect_ease_near_floor() {
        let (_, ease) = review_incorrect(1.4);
        assert_eq!(ease, 1.3);
    }

    #[test]
    fn test_next_review() {
        let now = Utc::now();
        let nr = next_review(&now, 3.0);
        let diff = (nr - now).num_seconds();
        assert_eq!(diff, 3 * 86400);
    }

    #[test]
    fn test_status_new() {
        let now = Utc::now();
        assert_eq!(compute_status(0.0, None, 0, &now, &now), "new");
    }

    #[test]
    fn test_status_learning() {
        let now = Utc::now();
        assert_eq!(compute_status(3.0, Some(50.0), 2, &now, &now), "learning");
    }

    #[test]
    fn test_status_reviewing() {
        let now = Utc::now();
        assert_eq!(compute_status(10.0, Some(75.0), 4, &now, &now), "reviewing");
    }

    #[test]
    fn test_status_mastered() {
        let now = Utc::now();
        assert_eq!(compute_status(30.0, Some(95.0), 6, &now, &now), "mastered");
    }

    #[test]
    fn test_status_not_mastered_low_accuracy() {
        let now = Utc::now();
        assert_eq!(compute_status(30.0, Some(80.0), 6, &now, &now), "reviewing");
    }

    #[test]
    fn test_status_not_mastered_low_quizzed() {
        let now = Utc::now();
        assert_eq!(compute_status(30.0, Some(95.0), 4, &now, &now), "reviewing");
    }
}
