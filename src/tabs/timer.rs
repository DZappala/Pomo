use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Gauge, Widget},
};

#[derive(Debug, Default, Clone)]
pub struct TimerTab {
    pub is_running: bool,
    pub progress: u16,
}

impl Widget for &TimerTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Min(0)]);
        let [timer] = horizontal.areas(area);
        let gauge = Gauge::default()
            .block(Block::bordered())
            .gauge_style(Style::default())
            .percent(self.progress);

        // println!("p: {:?}", self.progress);
        Widget::render(gauge, timer, buf);
    }
}
