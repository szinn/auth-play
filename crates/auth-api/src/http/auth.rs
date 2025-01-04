use std::time::Duration;

use auth_domain_api::AuthApi;
use auth_utils::arcbox::ArcBox;
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use tower_http::timeout::TimeoutLayer;

use crate::ApiError;

#[derive(Debug, Serialize)]
struct SessionResponse {
    pub email: String,
    pub name: String,
}

pub(crate) fn get_routes(auth_api: ArcBox<dyn AuthApi>) -> Router<()> {
    axum::Router::new()
        .route("/session", get(get_session))
        .with_state(auth_api)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

#[tracing::instrument(level = "trace", skip(_auth_api))]
async fn get_session(State(_auth_api): State<ArcBox<dyn AuthApi>>) -> Result<Json<SessionResponse>, ApiError> {
    let response = SessionResponse {
        email: "foo@bar.com".into(),
        name: "Foo Bar".into(),
    };

    Ok(Json(response))
}
