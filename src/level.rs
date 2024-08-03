// Traits for working with levels

use anyhow::Result;
use async_trait::async_trait;
use bitcoin::Transaction;

#[async_trait]
pub trait Level {
    async fn setup() -> Result<Transaction>; // includes code to spin up regtest and setup
    async fn run(tx : Transaction) -> Result<bool>; // includes code to watch for transactions
    async fn cleanup(&self) -> Result<()>; // includes code to award points and clean up
}
