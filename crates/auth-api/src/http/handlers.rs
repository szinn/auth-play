use crate::ApiError;
use std::{net::SocketAddr, sync::Arc, time::Duration};

use auth_domain_api::AuthApi;
use axum::{extract::Request, routing::get, Router};
use hyper::body::Incoming;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio_graceful_shutdown::SubsystemHandle;
use tower::Service;
use tower_http::timeout::TimeoutLayer;

use super::{health, v1};

pub fn get_routes(arch_api: Arc<AuthApi>) -> Router<()> {
    let v1_routes = v1::get_routes(arch_api.clone());

    let api_routes = Router::new().nest("/v1", v1_routes);

    let health_route: Router = Router::new().route("/", get(health::health)).with_state(arch_api.health_api.clone());

    axum::Router::new()
        .nest("/health", health_route)
        .nest("/api", api_routes)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

pub async fn handle(socket: TcpStream, remote_addr: SocketAddr, tower_service: Router<()>, subsys: SubsystemHandle) -> Result<(), ApiError> {
    let socket = TokioIo::new(socket);
    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| tower_service.clone().call(request));
    let conn = hyper::server::conn::http1::Builder::new().serve_connection(socket, hyper_service);
    let mut conn = std::pin::pin!(conn);

    tokio::select! {
        result = conn.as_mut() => {
            if let Err(err) = result {
                tracing::warn!("Failed to serve connection{}: {:#}", remote_addr, err);
            }
        }

        _ = subsys.on_shutdown_requested() => {
            tracing::debug!("signal received, starting graceful shutdown");
        }
    }

    tracing::debug!("Connection {} closed", remote_addr);
    Ok(())
}
