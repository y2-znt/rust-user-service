use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::AppError;
use super::validation::FieldError;

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct PaginatedResponse<T> {
    pub data: T,
    pub pagination: Pagination,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Pagination {
    pub total: u64,
    pub last_page: u64,
    pub current_page: u64,
    pub size: u64,
}

#[allow(dead_code)]
impl Pagination {
    pub fn from_total(total: u64, page: u32, per_page: u32) -> Self {
        let size = u64::from(per_page);
        let last_page = if size == 0 { 0 } else { total.div_ceil(size) };

        Self {
            total,
            last_page,
            current_page: u64::from(page),
            size,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[allow(dead_code)]
impl PaginationQuery {
    pub fn parse(&self) -> Result<(u32, u32), AppError> {
        let page = self.page.unwrap_or(1);
        let per_page = self.per_page.unwrap_or(20);
        let mut errors = Vec::new();

        if page < 1 {
            errors.push(FieldError {
                field: "page".to_string(),
                message: "page must be at least 1".to_string(),
            });
        }

        if !(1..=100).contains(&per_page) {
            errors.push(FieldError {
                field: "per_page".to_string(),
                message: "per_page must be between 1 and 100".to_string(),
            });
        }

        if errors.is_empty() {
            Ok((page, per_page))
        } else {
            Err(AppError::Validation(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json::json;

    #[test]
    fn paginated_response_includes_camel_case_pagination() {
        let timestamp = Utc.with_ymd_and_hms(2026, 8, 18, 19, 18, 59).unwrap();
        let body = PaginatedResponse {
            data: vec![json!({ "title": "dsh-desktop" })],
            pagination: Pagination {
                total: 3121,
                last_page: 3121,
                current_page: 1,
                size: 1,
            },
            timestamp,
        };

        let value = serde_json::to_value(&body).unwrap();

        assert_eq!(value["data"][0]["title"], "dsh-desktop");
        assert_eq!(value["pagination"]["total"], 3121);
        assert_eq!(value["pagination"]["lastPage"], 3121);
        assert_eq!(value["pagination"]["currentPage"], 1);
        assert_eq!(value["pagination"]["size"], 1);
        assert_eq!(value["timestamp"], "2026-08-18T19:18:59Z");
    }

    #[test]
    fn pagination_query_defaults_to_page_one_and_twenty_items() {
        let query = PaginationQuery::default();

        assert_eq!(query.parse().unwrap(), (1, 20));
    }

    #[test]
    fn pagination_query_rejects_out_of_range_values() {
        let query = PaginationQuery {
            page: Some(0),
            per_page: Some(101),
        };

        let AppError::Validation(errors) = query.parse().unwrap_err() else {
            panic!("expected validation error");
        };

        assert!(errors.iter().any(|error| error.field == "page"));
        assert!(errors.iter().any(|error| error.field == "per_page"));
    }

    #[test]
    fn pagination_computes_last_page_from_total_and_size() {
        let pagination = Pagination::from_total(3121, 1, 1);

        assert_eq!(pagination.last_page, 3121);
        assert_eq!(pagination.current_page, 1);
        assert_eq!(pagination.size, 1);
        assert_eq!(pagination.total, 3121);
    }
}
