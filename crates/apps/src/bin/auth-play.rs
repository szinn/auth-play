use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use auth_api::http::start_server;
use auth_domain_core::create_auth;
use auth_play::logging;
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging()?;
    let crate_version = clap::crate_version!();
    let git_revision = env!("BUILD_GIT_HASH");
    tracing::info!("AuthPlay {}-{}", crate_version, git_revision);

    let arch_service = Arc::new(create_auth().await.context("Couldn't create service")?);

    let server = Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("http_api", |h| start_server(3000, arch_service, h)));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_secs(5));

    server.await?;
    Ok(())
}
