use axum::{http::status::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthCheckResponse {
    #[serde(rename = "statusCode")]
    status_code: u16,
    data: String,
    message: String,
    success: bool,
}

pub async fn check_health() -> (StatusCode, Json<HealthCheckResponse>) {
    (
        StatusCode::OK,
        Json(HealthCheckResponse {
            status_code: StatusCode::OK.as_u16(),
            data: "Ok".to_string(),
            message: "healthy".to_string(),
            success: true,
        }),
    )
}
