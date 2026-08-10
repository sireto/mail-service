use aws_sdk_sesv2::error::SdkError;
use aws_sdk_sesv2::operation::send_email::SendEmailError;
use axum::http::status::StatusCode;
use axum::response::IntoResponse;
use lettre::transport::smtp::Error as SmtpError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("Internal server error: {0:?}")]
    InternalServerError(Option<String>),
    #[error("Not Found error: {0:?}")]
    NotFoundError(Option<String>),
    #[error("UUID parsing error: {0}")]
    UuidError(#[from] uuid::Error),
    #[error("Bad request: {0:?}")]
    BadRequestError(Option<String>),
    #[error("AWS SES error: {0}")]
    AwsSesError(Box<SdkError<SendEmailError>>),
    #[error("SMTP error: {0}")]
    SmtpError(#[from] SmtpError),
}

impl From<SdkError<SendEmailError>> for AppError {
    fn from(error: SdkError<SendEmailError>) -> Self {
        Self::AwsSesError(Box::new(error))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database Error: {:?}", e)),
            AppError::InternalServerError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                e.unwrap_or("Internal server error".to_string()),
            ),
            AppError::NotFoundError(e) => (StatusCode::NOT_FOUND, e.unwrap_or("Not found".to_string())),
            AppError::UuidError(e) => (StatusCode::BAD_REQUEST, format!("UUID parsing error: {:?}", e)),
            AppError::BadRequestError(e) => (StatusCode::BAD_REQUEST, format!("Bad request: {:?}", e)),
            AppError::AwsSesError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("AWS SES error: {:?}", e)),
            AppError::SmtpError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        (status, message).into_response()
    }
}
