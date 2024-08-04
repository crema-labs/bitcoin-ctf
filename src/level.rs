// Traits for working with levels

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Level: Send + Sync {
    async fn setup() -> Result<Self>
    where
        Self: Sized; // includes code to spin up regtest and setup
    async fn run(&self) -> Result<bool>; // includes code to watch for transactions
    async fn cleanup(&self) -> Result<()>; // includes code to award points and clean up
    fn print_problem_statement();
}

pub async fn start_level<T: Level>() -> Result<T> {
    T::setup().await
}
