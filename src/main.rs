use anyhow::{Ok, Result};
use clap::Parser;

mod cli;
mod ctf;
mod level;
mod state;
mod bitcoin;

#[tokio::main]
async fn main() -> Result<()> {
    // env_logger::init(); // TODO: Do we need a logger?

    // initialize cli
    let cli = cli::Cli::parse();

    let ctf = ctf::Ctf::new()?;
    ctf.run(cli).await?;
    Ok(())
}
