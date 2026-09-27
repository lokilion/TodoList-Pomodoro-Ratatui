mod app;
mod handle_events;
mod events;

use std::{error::Error};
use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| App::new().run(terminal))
}