use std::{sync::Arc, time::Duration};

use auth_domain_api::AuthApi;
use axum::Router;
use tower_http::timeout::TimeoutLayer;

pub(crate) fn get_routes(_auth_api: Arc<AuthApi>) -> Router<()> {
    axum::Router::new().layer(TimeoutLayer::new(Duration::from_secs(2)))
}
