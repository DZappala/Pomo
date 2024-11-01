use std::sync::Arc;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Gauge, Widget},
};

use tokio::sync::{mpsc::unbounded_channel, Mutex};

use crate::timer_backend::Timer;

#[derive(Debug, Default, Clone)]
pub struct TimerTab {
    pub timer: Arc<Mutex<Timer>>,
    pub is_running: bool,
    pub progress: Arc<std::sync::Mutex<u16>>,
}

impl TimerTab {
    pub async fn run(&self) {
        let (tx, mut rx) = unbounded_channel::<u16>();
        self.timer.lock().await.run(tx);
        while let Some(prog) = rx.recv().await {
            *self.progress.lock().unwrap() = prog;
            println!("prog: {:?}", prog)
        }
    }
}

impl Widget for &TimerTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Min(0)]);
        let [timer] = horizontal.areas(area);
        let gauge = Gauge::default()
            .block(Block::bordered())
            .gauge_style(Style::default())
            .percent(*self.progress.lock().unwrap());

        Widget::render(gauge, timer, buf);
    }
}
