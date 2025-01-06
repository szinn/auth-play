use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use auth_api::http::{start_server, Configuration};
use auth_db::connect_database;
use auth_domain_core::create_auth;
use auth_play::{config::AuthPlayConfig, logging};
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

#[tokio::main]
async fn main() -> Result<()> {
    let config = AuthPlayConfig::load().context("Cannot load configuration")?;

    logging::init_logging()?;

    let crate_version = clap::crate_version!();
    let git_revision = env!("BUILD_GIT_HASH");
    tracing::info!("AuthPlay {}-{}", crate_version, git_revision);

    let database = connect_database(&config.database.database_url).await.context("Couldn't connect to database")?;
    let arch_service = Arc::new(create_auth(database).await.context("Couldn't create service")?);

    let http_config = Configuration {
        port: config.http.port,
        secret_key: config.http.secret_key,
    };

    let server = Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("http_api", |h| start_server(http_config, arch_service, h)));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_secs(5));

    server.await?;
    Ok(())
}
