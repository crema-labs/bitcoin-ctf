// Load and Save Game state so far

use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const STATE_FILE: &str = "game_state.toml";

#[derive(Serialize, Deserialize)]
pub struct State {
    current_level: u32,
    pub completed_levels: Vec<CtfLevel>,
}

#[derive(Serialize, Deserialize)]
pub struct CtfLevel {
    pub level: u32,
    pub score: u32,
    // todo fields
}

impl State {
    pub fn load() -> Result<Self> {
        let path = Path::new(STATE_FILE);
        if path.exists() {
            let contents = fs::read_to_string(path)
                .with_context(|| format!("Failed to read state file: {}", STATE_FILE))?;
            let state: State = toml::from_str(&contents)
                .with_context(|| format!("Failed to parse state file: {}", STATE_FILE))?;
            Ok(state)
        } else {
            Ok(Self {
                current_level: 1,
                completed_levels: vec![],
            })
        }
    }

    pub fn initialize_state(&mut self) -> Result<()> {
        self.current_level = 1;
        self.completed_levels.clear();
        self.save()
    }

    pub fn save(&self) -> Result<()> {
        let toml_string =
            toml::to_string_pretty(self).context("Failed to serialize state to TOML")?;
        fs::write(STATE_FILE, toml_string)
            .with_context(|| format!("Failed to write state file: {}", STATE_FILE))?;
        Ok(())
    }

    pub fn current_level(&self) -> u32 {
        self.current_level
    }

    pub fn complete_level(&mut self, level: u32, score: u32) -> Result<()> {
        let completed_level = CtfLevel { level, score };
        self.completed_levels.push(completed_level);
        self.current_level = level + 1;
        self.save()
    }

    #[allow(dead_code)]
    pub fn get_level_score(&self, level: u32) -> Option<u32> {
        self.completed_levels
            .iter()
            .find(|&l| l.level == level)
            .map(|l| l.score)
    }
}
