use crate::ApiError;
use std::{net::SocketAddr, sync::Arc, time::Duration};

use auth_api_frontend::Dist;
use auth_domain_api::AuthDomainApi;
use axum::{
    extract::Request,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use hyper::{body::Incoming, header, StatusCode, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio_graceful_shutdown::SubsystemHandle;
use tower::Service;
use tower_http::timeout::TimeoutLayer;

use super::{health, v1};

static INDEX_HTML: &str = "index.html";

pub fn get_routes(arch_api: Arc<AuthDomainApi>) -> Router<()> {
    let v1_routes = v1::get_routes(arch_api.clone());

    let api_routes = Router::new().nest("/v1", v1_routes);

    let health_route: Router = Router::new().route("/", get(health::health)).with_state(arch_api.health_api.clone());

    axum::Router::new()
        .nest("/health", health_route)
        .nest("/api", api_routes)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
        .fallback(static_handler)
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

#[tracing::instrument(level = "trace")]
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return Redirect::permanent("/app").into_response();
    }
    let path = path.trim_start_matches("app/");

    match Dist::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            tracing::info!("Found content for {} with mime type {}", path, mime);
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                tracing::debug!("{} not found", path);
                return not_found().await;
            }

            index_html().await
        }
    }
}

#[tracing::instrument(level = "trace")]
async fn index_html() -> Response<axum::body::Body> {
    match Dist::get(INDEX_HTML) {
        Some(content) => {
            let mime = mime_guess::from_path(INDEX_HTML).first_or_octet_stream();
            tracing::info!("Found actual content for index.html with mime type {}", mime);
            Html(content.data).into_response()
        }
        None => not_found().await,
    }
}

#[tracing::instrument(level = "trace")]
async fn not_found() -> Response<axum::body::Body> {
    (StatusCode::NOT_FOUND, "404").into_response()
}
