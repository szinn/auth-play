use std::{net::SocketAddr, sync::Arc};

use auth_domain_api::AuthDomainApi;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use tokio::net::TcpListener;
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle};

use crate::ApiError;

pub(crate) mod auth;
pub(crate) mod handlers;
pub(crate) mod health;
pub(crate) mod session;
pub(crate) mod v1;

pub async fn start_server(port: u16, auth_domain_api: Arc<AuthDomainApi>, subsys: SubsystemHandle) -> Result<(), ApiError> {
    tracing::trace!("Starting http service");

    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().map_err(|_| ApiError::BadPort(port))?;
    let listener = TcpListener::bind(addr).await.unwrap();

    let routes = handlers::get_routes(auth_domain_api.clone());

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
