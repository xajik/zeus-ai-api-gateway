use reqwest::StatusCode;

#[derive(Debug)]
pub enum CustomError {
    HttpRequestError(reqwest::Error),
    JsonDeserializationError(serde_json::Error),
    NoContentFromAssistant,
    NonSuccessfulResponse(u16),
    IoError(std::io::Error),
    File(String),
    InternalServerError(StatusCode),
    SqlError(sqlx::Error),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CustomError::HttpRequestError(e) => write!(f, "HttpRequestError: {}", e),
            CustomError::JsonDeserializationError(e) => {
                write!(f, "JsonDeserializationError: {}", e)
            }
            CustomError::NoContentFromAssistant => write!(f, "NoContentFromAssistant"),
            CustomError::NonSuccessfulResponse(status_code) => {
                write!(f, "NonSuccessfulResponse: {}", status_code)
            }
            CustomError::IoError(e) => write!(f, "IoError: {}", e),
            CustomError::File(e) => write!(f, "File error: {}", e),
            CustomError::InternalServerError(e) => write!(f, "HttpError: {}", e),
            CustomError::SqlError(e) => write!(f, "SqlError: {}", e),
        }
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        CustomError::HttpRequestError(error)
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError::JsonDeserializationError(error)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IoError(error)
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(error: sqlx::Error) -> Self {
        CustomError::SqlError(error)
    }
}