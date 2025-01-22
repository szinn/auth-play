use std::{net::SocketAddr, sync::Arc};

use auth_domain_api::AuthDomainApi;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use session::SessionAdapter;
use tokio::net::TcpListener;
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle};

use crate::ApiError;

pub(crate) mod auth;
pub(crate) mod handlers;
pub(crate) mod health;
pub(crate) mod session;
pub(crate) mod v1;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub port: u16,
    pub secret_key: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
}

#[derive(Clone)]
pub(crate) struct ApiData {
    pub(crate) config: Configuration,
    pub(crate) auth_domain_api: Arc<AuthDomainApi>,
    pub(crate) session_adapter: SessionAdapter,
}

pub async fn start_server(config: Configuration, auth_domain_api: Arc<AuthDomainApi>, subsys: SubsystemHandle) -> Result<(), ApiError> {
    tracing::trace!("Starting http service");

    let port = config.port;
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().map_err(|_| ApiError::BadPort(port))?;
    let listener = TcpListener::bind(addr).await.unwrap();

    let api_data = Arc::new(ApiData {
        config,
        auth_domain_api: auth_domain_api.clone(),
        session_adapter: SessionAdapter::new(auth_domain_api.auth_api.clone()),
    });

    let routes = handlers::get_routes(api_data);

    tracing::info!("Listening on port {}", port);
    loop {
        let (socket, remote_addr) = tokio::select! {
            _ = subsys.on_shutdown_requested() => {
                break;
            }

            result = listener.accept() => {
                result.unwrap()
            }
        };

        tracing::debug!("connection {} accepted", remote_addr);
        let tower_service = routes.clone();
        let name = format!("handler-{remote_addr}");
        subsys.start(SubsystemBuilder::new(name, move |h| handlers::handle(socket, remote_addr, tower_service, h)));
    }

    Ok(())
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", self)).into_response()
    }
}
