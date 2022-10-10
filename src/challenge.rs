use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(feature = "okapi")]
mod open_api_features {
    use super::*;
    use okapi::openapi3::Responses;
    use rocket_okapi::{gen::OpenApiGenerator, response::OpenApiResponderInner};
    use serde::Serialize;

    impl OpenApiResponderInner for Challenge {
        fn responses(
            _: &mut rocket_okapi::gen::OpenApiGenerator,
        ) -> rocket_okapi::Result<okapi::openapi3::Responses> {
            Ok(Responses {
                ..Default::default()
            })
        }
    }
}
