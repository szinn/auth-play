use std::time::Duration;

use axum::Router;
use tower_http::timeout::TimeoutLayer;

pub(crate) fn get_routes() -> Router<()> {
    axum::Router::new().layer(TimeoutLayer::new(Duration::from_secs(2)))
}
