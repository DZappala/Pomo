use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style},
    widgets::{
        Block, HighlightSpacing, List, ListState, StatefulWidget, Table, TableState, Widget,
    },
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::Task;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq, Eq, Debug)]
pub enum Button {
    #[default]
    #[strum(to_string = "start/stop timer")]
    StartStop,
    #[strum(to_string = "Dummy")]
    Dummy,
    #[strum(to_string = "Dummy2")]
    Dummy2,
}

impl Button {
    fn prev(self) -> Self {
        let curr_idx: usize = self as usize;
        let prev_idx = curr_idx.saturating_sub(1);
        Self::from_repr(prev_idx).unwrap_or(self)
    }

    fn next(self) -> Self {
        let curr_idx: usize = self as usize;
        let next_idx = curr_idx.saturating_add(1);
        Self::from_repr(next_idx).unwrap_or(self)
    }

    fn current_index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HomeTab {
    row_index: usize,
    currently_selected_list_item: Button,
    history: Vec<Task>,
}

impl HomeTab {
    pub fn new(history: Vec<Task>) -> Self {
        Self {
            history,
            ..Default::default()
        }
    }

    pub fn prev_row(&mut self) {
        self.currently_selected_list_item = self.currently_selected_list_item.prev();
    }

    pub fn next_row(&mut self) {
        self.currently_selected_list_item = self.currently_selected_list_item.next();
    }

    fn render_home_list(&self, area: Rect, buf: &mut Buffer) {
        let list = List::from_iter(Button::iter().map(|t| t.to_string()))
            .block(Block::bordered())
            .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);

        let mut state =
            ListState::default().with_selected(Some(self.currently_selected_list_item as usize));

        StatefulWidget::render(list, area, buf, &mut state);
    }

    fn render_historical_data(&self, area: Rect, buf: &mut Buffer) {
        let table = Table::from_iter(self.history.iter()).block(Block::bordered());

        let mut state = TableState::default();
        StatefulWidget::render(table, area, buf, &mut state);
    }
}

impl Widget for &HomeTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Min(0), Constraint::Percentage(25)]);
        let [list, table] = vertical.areas(area);
        self.render_home_list(list, buf);
        self.render_historical_data(table, buf);
    }
}
