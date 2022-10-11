use chrono::{serde::ts_milliseconds, DateTime, Utc};
#[cfg(feature = "okapi")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "okapi", derive(JsonSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub title: String,
    pub why: String,
    pub note: String,
    #[serde(rename = "startedAt")]
    #[schemars(with = "String")]
    #[serde(with = "ts_milliseconds")]
    pub started_at: DateTime<Utc>,
    #[serde(rename = "endAt")]
    #[schemars(with = "String")]
    #[serde(with = "ts_milliseconds")]
    pub end_at: DateTime<Utc>,
    pub finished: bool,
}

#[cfg(feature = "okapi")]
mod open_api_features {
    use super::*;
    use okapi::openapi3::Responses;
    use rocket_okapi::response::OpenApiResponderInner;

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

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDateTime, Utc};

    use crate::Challenge;

    #[test]
    fn can_stringify() {
        let _ = serde_json::to_string(&Challenge {
            id: "_id".to_owned(),
            title: "title".to_owned(),
            why: "why".to_owned(),
            note: "note".to_owned(),
            started_at: DateTime::from_utc(NaiveDateTime::default(), Utc),
            end_at: DateTime::from_utc(NaiveDateTime::default(), Utc),
            finished: false,
        })
        .unwrap();
    }

    #[test]
    fn can_parse_json() {
        let _: Challenge = serde_json::from_str(&"{\"id\":\"_id\",\"title\":\"title\",\"why\":\"why\",\"note\":\"note\",\"started_at\":0,\"end_at\":0,\"finished\":false}").unwrap();
    }
}
