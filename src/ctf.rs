// Load Ctf State
// process command line arguments
// setup and run levels

use anyhow::{Ok, Result};
use colored::Colorize;

use crate::{cli::Cli, level::Level, levels::LevelOne, state::State};

pub struct Ctf {
    state: State,
}

impl Ctf {
    pub fn new() -> Result<Self> {
        // Initialize game state and levels
        Ok(Self {
            state: State::load()?,
        })
    }

    pub async fn run(&mut self, cli: Cli) -> Result<()> {
        match &cli.command {
            crate::cli::Commands::New => self.start_new_game().await,
            crate::cli::Commands::Continue => self.continue_game().await,
            crate::cli::Commands::Retry { level } => self.retry_level(level).await,
            crate::cli::Commands::Stats => Ok(()),
        }
    }

    async fn start_new_game(&mut self) -> Result<()> {
        println!("{}", "Starting a new game!".green());
        // clean and save Ctf stats
        // start from level 1
        self.state.initialize_state()?;

        let tx = LevelOne::setup().await?;
        if LevelOne::run(tx).await? {
            self.state.complete_level(1 , 100)?;
            self.state.save()?;
        }

        Ok(())
    }

    async fn continue_game(&self) -> Result<()> {
        // Note : before continuing the game if current level > 1, then check if user passed previous level with atleast 60% of score.
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
