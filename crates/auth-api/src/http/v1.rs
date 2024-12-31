use std::{sync::Arc, time::Duration};

use auth_domain_api::AuthDomainApi;
use axum::Router;
use tower_http::timeout::TimeoutLayer;

pub(crate) fn get_routes(_auth_api: Arc<AuthDomainApi>) -> Router<()> {
    axum::Router::new().layer(TimeoutLayer::new(Duration::from_secs(2)))
}
