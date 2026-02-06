use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn default_ease() -> f64 {
    2.5
}

fn default_status() -> String {
    "new".to_string()
}

fn default_version() -> String {
    "1.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub front: String,
    pub back: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_ja: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub last_quizzed: Option<DateTime<Utc>>,
    pub next_review: DateTime<Utc>,
    #[serde(default)]
    pub times_quizzed: u32,
    #[serde(default)]
    pub times_correct: u32,
    #[serde(default = "default_ease")]
    pub ease_factor: f64,
    #[serde(default)]
    pub interval_days: f64,
    #[serde(default = "default_status")]
    pub status: String,
    /// Preserve unknown fields for forward compatibility with Claude skills
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl LearningItem {
    pub fn accuracy(&self) -> Option<f64> {
        if self.times_quizzed == 0 {
            None
        } else {
            Some(self.times_correct as f64 / self.times_quizzed as f64 * 100.0)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrsDatabase {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub last_updated: Option<DateTime<Utc>>,
    #[serde(default)]
    pub items: Vec<LearningItem>,
}

impl SrsDatabase {
    pub fn new() -> Self {
        SrsDatabase {
            version: "1.0".to_string(),
            last_updated: Some(Utc::now()),
            items: Vec::new(),
        }
    }
}

/// Input for batch review (stdin JSON array element)
#[derive(Debug, Deserialize)]
pub struct ReviewInput {
    pub id: String,
    pub result: String,
    #[serde(default)]
    pub difficulty: Option<String>,
}
