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
    widgets::{Cell, Row},
    Terminal, TerminalOptions, Viewport,
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    io::{stdout, BufReader, Stdout},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // check if data file exists
    let path = "./src/test/data.json";
    // todo: find out how to store user data locally for different platforms
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let history: Vec<Task> = serde_json::from_reader(reader)?;

    let viewport = Viewport::Fullscreen;
    let opts = TerminalOptions { viewport };
    let terminal: Terminal<CrosstermBackend<Stdout>> = init_with_options(opts);
    execute!(stdout(), EnterAlternateScreen).expect("Failed to enter alternate screen");

    let results = App::new(history).run(terminal).await;
    execute!(stdout(), LeaveAlternateScreen).expect("Failed to leave alternate screen");

    ratatui::restore();
    return results;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    title: String,
    desc: String,
    time_spent: f32,
}

impl<'a> From<&Task> for Row<'a> {
    fn from(val: &Task) -> Self {
        let cells: Vec<Cell> = vec![
            Cell::new(val.title.clone()),
            Cell::new(val.desc.clone()),
            Cell::new(val.time_spent.to_string()),
        ];

        Row::from_iter(cells)
    }
}
