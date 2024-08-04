use anyhow::Result;
use bitcoin::CtfFramework;
use clap::Parser;

mod bitcoin;
mod cli;
mod constrants;
mod ctf;
mod level;
mod levels;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // env_logger::init(); // TODO: Do we need a logger?

    // initialize cli
    let cli = cli::Cli::parse();

    let mut ctf = ctf::Ctf::new()?;
    ctf.run(cli).await?;
    CtfFramework::clean()
}
