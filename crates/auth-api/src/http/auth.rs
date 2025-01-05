use std::time::Duration;

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use tower_http::timeout::TimeoutLayer;
use tower_sessions::Session;

use crate::ApiError;

use super::session::SessionAdapter;

#[derive(Debug, Serialize)]
struct SessionResponse {
    pub email: String,
    pub name: String,
}

pub(crate) fn get_routes(session_adapter: SessionAdapter) -> Router<()> {
    axum::Router::new()
        .route("/session", get(get_session))
        .with_state(session_adapter)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

#[tracing::instrument(level = "trace", skip(_session_adapter))]
async fn get_session(session: Session, State(_session_adapter): State<SessionAdapter>) -> Result<Json<SessionResponse>, ApiError> {
    let response = SessionResponse {
        email: "foo@bar.com".into(),
        name: "Foo Bar".into(),
    };

    Ok(Json(response))
}
