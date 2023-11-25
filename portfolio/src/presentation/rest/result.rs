use super::error::RestError;

pub type RestResult<T> = std::result::Result<T, RestError>;
