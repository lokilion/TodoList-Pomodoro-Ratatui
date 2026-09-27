pub mod app;
pub mod events;
pub mod handle_events;
pub mod models;
pub mod pomodoro;
use std::{error::Error};
use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| App::new().run(terminal))
}