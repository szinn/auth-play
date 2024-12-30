use auth_domain_api::HealthApi;
use auth_utils::arcbox::ArcBox;
use axum::{extract::State, response::IntoResponse};
use hyper::StatusCode;

#[tracing::instrument(level = "trace", skip(health_api))]
pub(crate) async fn health(State(health_api): State<ArcBox<dyn HealthApi>>) -> impl IntoResponse {
    match health_api.is_healthy().await {
        true => StatusCode::OK,
        false => StatusCode::BAD_REQUEST,
    }
}
