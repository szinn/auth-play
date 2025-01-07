use async_trait::async_trait;

#[async_trait]
pub trait SessionAdapter: Send + Sync {}

pub(crate) struct SessionAdapterImpl {}

impl SessionAdapterImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SessionAdapter for SessionAdapterImpl {}
