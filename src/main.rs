mod app;
// mod runner;
mod crossterm;
mod ui;
mod game;

use crate::crossterm::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}
