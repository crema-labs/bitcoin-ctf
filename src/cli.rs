// CLI to
// - start new fresh ctf
// - continue existing ctf
// - retry one of past passed ctf

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "Jayendra Madaram <jayendramadaram@gmail.com>",
    version = "1.0.0",
    about = "A CTF-style educational tool for learning Bitcoin concepts",
    long_about = "Bitcoin CTF is an interactive command-line game designed to teach Bitcoin concepts through hands-on challenges. Players progress through levels, each focusing on different aspects of Bitcoin technology and protocol."
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start a new game
    New,
    /// Continue the existing game
    Continue,
    /// Retry a specific level
    Retry {
        /// The level number to retry
        #[arg(value_name = "LEVEL")]
        level: u32,
    },
    /// Display game statistics
    Stats,
}
