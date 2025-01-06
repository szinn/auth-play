use auth_db::run_migration_cli;

#[tokio::main]
async fn main() {
    run_migration_cli().await;
}
