use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

use super::validation::FieldError;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error")]
    Internal,

    #[error("Validation error")]
    Validation(Vec<FieldError>),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorBody {
    error: &'static str,
    status_code: u16,
    timestamp: DateTime<Utc>,
    details: ErrorDetails,
}

#[derive(Serialize)]
struct ErrorDetails {
    code: &'static str,
    message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidationBody {
    errors: Vec<FieldError>,
    status_code: u16,
    timestamp: DateTime<Utc>,
}

impl AppError {
    fn status(&self) -> StatusCode {
        match self {
            AppError::Database(_) | AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::BadRequest(_) | AppError::Validation(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_json(&self) -> Value {
        let timestamp = Utc::now();
        let status_code = self.status().as_u16();

        if let AppError::Validation(errors) = self {
            return serde_json::to_value(ValidationBody {
                errors: errors.clone(),
                status_code,
                timestamp,
            })
            .expect("validation body should serialize");
        }

        let (error, code, message) = match self {
            AppError::Database(_) | AppError::Internal => (
                "Internal server error",
                "INTERNAL",
                "Internal server error".to_string(),
            ),
            AppError::NotFound(message) => ("Not found", "NOT_FOUND", message.clone()),
            AppError::Unauthorized => ("Unauthorized", "UNAUTHORIZED", "Unauthorized".to_string()),
            AppError::Forbidden => ("Forbidden", "FORBIDDEN", "Forbidden".to_string()),
            AppError::Conflict(message) => ("Conflict", "CONFLICT", message.clone()),
            AppError::BadRequest(message) => ("Bad request", "BAD_REQUEST", message.clone()),
            AppError::Validation(_) => unreachable!(),
        };

        serde_json::to_value(ErrorBody {
            error,
            status_code,
            timestamp,
            details: ErrorDetails { code, message },
        })
        .expect("error body should serialize")
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status(), Json(self.error_json())).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_error_uses_string_error_and_details() {
        let value = AppError::Unauthorized.error_json();

        assert_eq!(value["error"], "Unauthorized");
        assert_eq!(value["statusCode"], 401);
        assert_eq!(value["details"]["code"], "UNAUTHORIZED");
        assert_eq!(value["details"]["message"], "Unauthorized");
        assert!(value["timestamp"].is_string());
    }

    #[test]
    fn validation_error_returns_field_list() {
        let value = AppError::Validation(vec![FieldError {
            field: "title".to_string(),
            message: "Title is required".to_string(),
        }])
        .error_json();

        assert_eq!(value["errors"][0]["field"], "title");
        assert_eq!(value["errors"][0]["message"], "Title is required");
        assert_eq!(value["statusCode"], 400);
        assert!(value.get("error").is_none());
        assert!(value["timestamp"].is_string());
    }

    #[test]
    fn conflict_keeps_public_error_and_specific_details_message() {
        let value = AppError::Conflict("email already taken".to_string()).error_json();

        assert_eq!(value["error"], "Conflict");
        assert_eq!(value["statusCode"], 409);
        assert_eq!(value["details"]["code"], "CONFLICT");
        assert_eq!(value["details"]["message"], "email already taken");
    }

    #[test]
    fn not_found_uses_provided_message() {
        let value = AppError::NotFound("User not found".to_string()).error_json();

        assert_eq!(value["error"], "Not found");
        assert_eq!(value["statusCode"], 404);
        assert_eq!(value["details"]["code"], "NOT_FOUND");
        assert_eq!(value["details"]["message"], "User not found");
    }
}
