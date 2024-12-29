use anyhow::Result;
use auth_play::logging;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging()?;
    let crate_version = clap::crate_version!();
    let git_revision = env!("BUILD_GIT_HASH");
    tracing::info!("AuthPlay {}-{}", crate_version, git_revision);

    Ok(())
}
