// Load Ctf State
// process command line arguments
// setup and run levels

use anyhow::{Ok, Result};
use colored::Colorize;

use crate::{cli::Cli, level::Level, state::State};

pub struct Ctf {
    state: State,
    levels: Vec<Box<dyn Level>>,
}

impl Ctf {
    pub fn new() -> Result<Self> {
        // Initialize game state and levels
        Ok(Self {
            state: State::load()?,
            levels: vec![],
        })
    }

    pub async fn run(&self, cli: Cli) -> Result<()> {
        match &cli.command {
            crate::cli::Commands::New => self.start_new_game().await,
            crate::cli::Commands::Continue => self.continue_game().await,
            crate::cli::Commands::Retry { level } => self.retry_level(level).await,
            crate::cli::Commands::Stats => Ok(()),
        }
    }

    async fn start_new_game(&self) -> Result<()> {
        // todo
        // clean and save Ctf stats
        // start from level 1
        println!("{}", "Starting a new game!".green());
        Ok(())
    }

    async fn continue_game(&self) -> Result<()> {
        // todo
        println!(
            "{} {}",
            "Continuing the game from level".green(),
            self.state.current_level()
        );
        Ok(())
    }

    async fn retry_level(&self, level: &u32) -> Result<()> {
        // Implement retry level logic
        println!("{}", format!("Retrying level {}", level).green());
        Ok(())
    }
}
