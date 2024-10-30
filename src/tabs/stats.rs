use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, List, Widget},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatsTab {
    row_index: usize,
}

impl StatsTab {
    pub fn prev_row(&mut self) {
        self.row_index = self.row_index.saturating_sub(1);
    }

    pub fn next_row(&mut self) {
        self.row_index = self.row_index.saturating_add(1);
    }
}

impl Widget for StatsTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Min(0)]);
        let [list] = horizontal.areas(area);
        // render_settings_list(list, buf);
    }
}
