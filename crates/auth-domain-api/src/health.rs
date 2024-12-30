use async_trait::async_trait;

#[async_trait]
pub trait HealthApi: Send + Sync {
    async fn is_healthy(&self) -> bool;
}
