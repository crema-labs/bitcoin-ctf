// Traits for working with levels

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Level {
    async fn setup(&self) -> Result<()>;
    async fn run(&self) -> Result<bool>;
    async fn cleanup(&self) -> Result<()>;
}
