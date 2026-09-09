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

impl From<aws_sdk_sesv2::error::BuildError> for AppError {
    fn from(error: aws_sdk_sesv2::error::BuildError) -> Self {
        Self::InternalServerError(Some(format!("Failed to build SES request: {error}")))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            // A missing row is a 404, not a 500. Previously every Diesel error, including
            // NotFound, became a 500 with the raw error text in the body — which leaked
            // table names, constraint names and sometimes column values to the caller.
            AppError::DatabaseError(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, "Resource not found".to_string())
            }
            AppError::DatabaseError(e) => {
                // Keep the detail in the server log where it is useful, out of the response
                // where it is an information leak.
                eprintln!("database error: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred".to_string(),
                )
            }
            AppError::InternalServerError(e) => {
                if let Some(detail) = &e {
                    eprintln!("internal error: {detail}");
                }
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::NotFoundError(e) => (StatusCode::NOT_FOUND, e.unwrap_or("Not found".to_string())),
            AppError::UuidError(e) => (StatusCode::BAD_REQUEST, format!("Invalid UUID: {e}")),
            AppError::BadRequestError(e) => (StatusCode::BAD_REQUEST, e.unwrap_or("Bad request".to_string())),
            AppError::AwsSesError(e) => {
                eprintln!("AWS SES error: {e:?}");
                (
                    StatusCode::BAD_GATEWAY,
                    "The email provider rejected the request".to_string(),
                )
            }
            AppError::SmtpError(e) => {
                eprintln!("SMTP error: {e:?}");
                (
                    StatusCode::BAD_GATEWAY,
                    "The SMTP server rejected the request".to_string(),
                )
            }
        };
        (status, message).into_response()
    }
}
