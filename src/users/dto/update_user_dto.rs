use email_address::EmailAddress;
use serde::Deserialize;

use crate::api::{AppError, FieldError};

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserDto {
    pub fn validate(&self) -> Result<(), AppError> {
        let mut errors = Vec::new();

        if let Some(email) = &self.email
            && !EmailAddress::is_valid(email)
        {
            errors.push(FieldError {
                field: "email".to_string(),
                message: "Email must be valid".to_string(),
            });
        }

        if let Some(username) = &self.username
            && !(3..=30).contains(&username.trim().chars().count())
        {
            errors.push(FieldError {
                field: "username".to_string(),
                message: "Username must be between 3 and 30 characters".to_string(),
            });
        }

        if let Some(password) = &self.password
            && password.chars().count() < 8
        {
            errors.push(FieldError {
                field: "password".to_string(),
                message: "Password must be at least 8 characters".to_string(),
            });
        }

        if self.email.is_none() && self.username.is_none() && self.password.is_none() {
            return Err(AppError::Validation(vec![FieldError {
                field: "body".to_string(),
                message: "At least one field must be provided".to_string(),
            }]));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::Validation(errors))
        }
    }
}
