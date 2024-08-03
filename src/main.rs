use anyhow::{Ok, Result};
use clap::Parser;

mod bitcoin;
mod cli;
mod ctf;
mod level;
mod levels;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    // env_logger::init(); // TODO: Do we need a logger?

    // initialize cli
    let cli = cli::Cli::parse();

    let mut ctf = ctf::Ctf::new()?;
    ctf.run(cli).await
}
