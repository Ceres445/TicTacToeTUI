mod app;
// mod runner;
mod crossterm;
mod ui;
mod game;
mod computer_player;

use crate::crossterm::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}
