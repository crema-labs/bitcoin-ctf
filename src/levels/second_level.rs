use anyhow::Result;
use async_trait::async_trait;
use bitcoin::Transaction;

use crate::{bitcoin::CtfFramework, level::Level};

#[allow(dead_code)]
pub struct LevelTwo {
    target_tx: Transaction,
    ctf_framework: CtfFramework,
}

#[async_trait]
impl Level for LevelTwo {
    async fn setup() -> Result<Self> {
        todo!()
    }

    async fn run(&self) -> Result<bool> {
        todo!()
    }

    async fn cleanup(&self) -> Result<()> {
        todo!()
    }

    fn print_problem_statement() {}
}
