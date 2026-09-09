use axum::{http::status::StatusCode, Json};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::GLOBAL_APP_STATE;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthCheckResponse {
    #[serde(rename = "statusCode")]
    status_code: u16,
    data: String,
    message: String,
    success: bool,
}

/// Liveness and readiness.
///
/// This used to return a static OK, which made it useless as a readiness probe: the
/// container reported healthy while unable to reach Postgres. It now checks out a pooled
/// connection and runs a trivial query.
#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthCheckResponse),
        (status = 503, description = "Service cannot reach its database", body = HealthCheckResponse)
    )
)]
pub async fn check_health() -> (StatusCode, Json<HealthCheckResponse>) {
    let db_ok = match GLOBAL_APP_STATE.db_pool.get() {
        Ok(mut conn) => diesel::sql_query("SELECT 1").execute(&mut conn).is_ok(),
        Err(err) => {
            eprintln!("health check: could not get a database connection: {err}");
            false
        }
    };

    if db_ok {
        (
            StatusCode::OK,
            Json(HealthCheckResponse {
                status_code: StatusCode::OK.as_u16(),
                data: "Ok".to_string(),
                message: "healthy".to_string(),
                success: true,
            }),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthCheckResponse {
                status_code: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                data: "Unavailable".to_string(),
                message: "database unreachable".to_string(),
                success: false,
            }),
        )
    }
}
