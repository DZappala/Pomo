#![feature(assert_matches, async_closure)]
mod app;
mod shutdown;
mod tabs;
mod timer_backend;

use crate::app::App;
use ratatui::{
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    },
    init_with_options,
    prelude::*,
    Terminal, TerminalOptions, Viewport,
};
use std::{
    error::Error,
    io::{stdout, Stdout},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let viewport = Viewport::Fullscreen;
    let options = TerminalOptions { viewport };
    let terminal: Terminal<CrosstermBackend<Stdout>> = init_with_options(options);
    execute!(stdout(), EnterAlternateScreen).expect("Failed to enter alternate screen");

    let results = App::default().run(terminal).await;
    execute!(stdout(), LeaveAlternateScreen).expect("Failed to leave alternate screen");

    ratatui::restore();
    results
}
