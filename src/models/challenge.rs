use chrono::{serde::ts_milliseconds, DateTime, Utc};

#[derive(Debug, serde::Serialize)]
pub struct Challenge {
    pub id: String,
    pub title: String,
    pub why: String,
    pub note: String,
    #[serde(with = "ts_milliseconds")]
    pub started_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub end_at: DateTime<Utc>,
    pub finished: bool,
}
