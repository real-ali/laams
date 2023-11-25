use std::{error::Error, fmt::Display, ops::Deref};

use actix_web::{http::StatusCode, ResponseError};

use crate::domain::common::DomainError;

#[derive(Debug)]
pub struct RestError {
    code: String,
    message: String,
}

impl From<DomainError> for RestError {
    fn from(value: DomainError) -> Self {
        Self {
            code: value.code,
            message: value.message,
        }
    }
}

impl Display for RestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code:\t{}\nMessage:\t{}", self.code, self.message)
    }
}

impl Error for RestError {}

impl ResponseError for RestError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.code.deref() {
            "NotFound" => StatusCode::NOT_FOUND,
            "InvalidDomain" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
