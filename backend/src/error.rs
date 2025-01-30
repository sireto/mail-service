use aws_sdk_sesv2::Error as SdkError;
use aws_smithy_runtime_api::client::result::SdkError as GenericSdkError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailServiceError {
    #[error("Receiver email address is required")]
    MissingReceiverEmail,

    #[error("Invalid email address format: {0}")]
    InvalidEmailAddress(String),

    // Wrap AWS SDK Errors
    #[error("AWS SDK Error: {0}")]
    SdkError(#[from] GenericSdkError<SdkError, aws_smithy_runtime_api::http::Response>),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
