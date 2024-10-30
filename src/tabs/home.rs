use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style},
    widgets::{Block, HighlightSpacing, List, ListState, StatefulWidget, Widget},
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HomeTab {
    row_index: usize,
    currently_selected: Button,
}

impl HomeTab {
    pub fn prev_row(&mut self) {
        self.currently_selected = self.currently_selected.prev();
    }

    pub fn next_row(&mut self) {
        self.currently_selected = self.currently_selected.next();
    }

    fn render_home_list(self, area: Rect, buf: &mut Buffer) {
        let list = List::from_iter(Button::iter().map(|t| t.to_string()))
            .block(Block::bordered())
            .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);

        let mut state = ListState::default().with_selected(Some(self.currently_selected as usize));

        StatefulWidget::render(list, area, buf, &mut state);
    }
}

impl Widget for HomeTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Min(0)]);
        let [list] = horizontal.areas(area);
        self.render_home_list(list, buf);
    }
}
