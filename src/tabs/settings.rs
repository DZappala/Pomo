use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, List, Widget},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SettingsTab {
    row_index: usize,
}

impl SettingsTab {
    pub fn prev_row(&mut self) {
        self.row_index = self.row_index.saturating_sub(1);
    }

    pub fn next_row(&mut self) {
        self.row_index = self.row_index.saturating_add(1);
    }
}

impl Widget for SettingsTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Min(0)]);
        let [list] = horizontal.areas(area);
        render_settings_list(list, buf);
    }
}

//todo: this should use strum list. i.e. enum members for list items
fn render_settings_list(area: Rect, buf: &mut Buffer) {
    List::new(["Set mode", "Clear data"])
        .block(Block::bordered())
        .render(area, buf);
}
