use axum::body::to_bytes;
use axum::{
    body::Body,
    http::{header, Request},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// Error handling middleware...
pub async fn error_handling_middleware(request: Request<Body>, next: Next) -> Response {
    // Process the request and get response...
    let response = next.run(request).await;
    let status = response.status();

    // Only handle client and server errors otherwise return the response as it is...
    if !status.is_client_error() && !status.is_server_error() {
        return response;
    }

    // Extract the original error message from the response body...
    let (mut parts, body) = response.into_parts();
    let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
    let message = String::from_utf8_lossy(&bytes);

    // Preserve existing headers but set content type to JSON
    parts
        .headers
        .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    // Create our standardized JSON error format...
    let json_error = json!({
        "code": status.as_u16(),
        "type": status.canonical_reason().unwrap_or("Unknown"),
        "message": if message.trim().is_empty() {
            None
        } else {
            Some(message.to_string())
        },
    });

    // Build the new response with the JSON error...
    Response::from_parts(parts, Json(json_error).into_response().into_body())
}
