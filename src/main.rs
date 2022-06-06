mod app;
mod runner;
mod ui;

use crate::runner::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}
