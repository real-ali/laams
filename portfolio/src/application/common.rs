pub mod libraries;
pub mod persistence;

use crate::domain::common::DomainError;

pub type AppResult<T> = std::result::Result<T, DomainError>;
