mod app;
mod runner;
mod ui;
mod game;

use crate::runner::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}
