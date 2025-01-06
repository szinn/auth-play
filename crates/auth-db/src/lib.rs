pub mod entities;
pub mod error;

use std::{future::Future, pin::Pin, sync::Arc};

use async_trait::async_trait;
pub use error::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait};
use sea_orm_migration::{cli, MigrationTrait, MigratorTrait};
use tracing_log::log;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250106_194018_users::Migration), Box::new(m20250106_205738_sessions::Migration)]
    }
}

pub struct Repository {
    pub database: DatabaseConnection,
}

impl Repository {
    pub async fn transaction<F, T>(&self, operation: F) -> Result<T, Error>
    where
        F: FnOnce(&mut DatabaseTransaction) -> Pin<Box<dyn Future<Output = Result<T, Error>> + '_ + Send>>,
    {
        let mut tx = self.database.begin().await?;

        let result = operation(&mut tx).await;
        if result.is_err() {
            tx.rollback().await?
        } else {
            tx.commit().await?;
        }

        result
    }
}

pub struct RepositoryAdapters {
    pub repository: Arc<Repository>,
    // pub item_adapter: ArcBox<dyn ItemAdapter>,
}

pub async fn connect_database(url: &str) -> Result<Arc<RepositoryAdapters>, Error> {
    tracing::debug!("Connecting to database...");
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let database = Database::connect(opt).await?;
    Migrator::up(&database, None).await?;
    tracing::debug!("...connected to database");

    let repository = Arc::new(Repository { database });

    // let item_adapter = ItemAdapterImpl::new();
    // let item_adapter: ArcBox<dyn ItemAdapter> = arcbox!(item_adapter);

    let adapters = Arc::new(RepositoryAdapters {
        repository,
        // item_adapter,
    });

    Ok(adapters)
}

pub async fn run_migration_cli() {
    cli::run_cli(Migrator).await
}

pub(crate) mod m20250106_194018_users;
pub(crate) mod m20250106_205738_sessions;
