use email_address::EmailAddress;
use serde::Deserialize;

use crate::api::{AppError, FieldError};

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl CreateUserDto {
    pub fn validate(&self) -> Result<(), AppError> {
        let mut errors = Vec::new();

        if !EmailAddress::is_valid(&self.email) {
            errors.push(FieldError {
                field: "email".to_string(),
                message: "Email must be valid".to_string(),
            });
        }

        let username_length = self.username.trim().chars().count();

        if !(3..=30).contains(&username_length) {
            errors.push(FieldError {
                field: "username".to_string(),
                message: "Username must be between 3 and 30 characters".to_string(),
            });
        }

        if self.password.chars().count() < 8 {
            errors.push(FieldError {
                field: "password".to_string(),
                message: "Password must be at least 8 characters".to_string(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::Validation(errors))
        }
    }
}
