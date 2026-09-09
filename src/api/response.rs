use axum::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;

use super::pagination::{PaginatedResponse, Pagination};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub timestamp: DateTime<Utc>,
}

pub fn ok<T>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        data,
        timestamp: Utc::now(),
    })
}

#[allow(dead_code)]
pub fn paginated<T>(data: T, pagination: Pagination) -> Json<PaginatedResponse<T>> {
    Json(PaginatedResponse {
        data,
        pagination,
        timestamp: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json::json;

    #[test]
    fn success_response_wraps_payload_in_data_and_timestamp() {
        let timestamp = Utc.with_ymd_and_hms(2026, 8, 18, 19, 15, 31).unwrap();
        let body = ApiResponse {
            data: json!({ "username": "octocat" }),
            timestamp,
        };

        let value = serde_json::to_value(&body).unwrap();

        assert_eq!(value["data"]["username"], "octocat");
        assert_eq!(value["timestamp"], "2026-08-18T19:15:31Z");
    }
}
