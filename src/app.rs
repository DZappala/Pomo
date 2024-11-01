// todo
// An enum to track the app state
// An enum to track the timer mode
// An enum to track the app page

use futures::StreamExt;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, EventStream, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    symbols,
    widgets::{Block, Tabs, Widget},
    DefaultTerminal, Frame,
};
use std::{collections::HashMap, error::Error, result::Result, time::Duration};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::{
    shutdown,
    tabs::{HomeTab, SettingsTab, StatsTab, TimerTab},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Startup,
    Running,
    Shutdown,
    Exit,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq, Eq, Debug)]
pub enum CurrentTab {
    #[default]
    #[strum(to_string = "(m)ain")]
    Home,
    #[strum(to_string = "(t)imer")]
    Timer,
    #[strum(to_string = "st(a)ts")]
    Stats,
    #[strum(to_string = "s(e)ttings")]
    Settings,
}

pub struct App {
    pub state: AppState,
    // map of all the previous timer entries, loaded from json
    pub history: HashMap<String, String>,
    pub current_tab: CurrentTab,
    pub home_tab: HomeTab,
    pub settings_tab: SettingsTab,
    pub timer_tab: TimerTab,
    pub stats_tab: StatsTab,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new() -> Self {
        Self {
            state: AppState::Startup,
            history: HashMap::new(),
            current_tab: CurrentTab::default(),
            home_tab: HomeTab::default(),
            settings_tab: SettingsTab::default(),
            timer_tab: TimerTab::default(),
            stats_tab: StatsTab::default(),
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
        let period = Duration::from_secs_f32(1. / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while self.is_running() {
            tokio::select! {
                _ = interval.tick() => {terminal.draw(|frame| self.draw(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event).await,
            }
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state != AppState::Exit
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        if self.state == AppState::Shutdown {
            shutdown::shutdown(frame);
        }
    }

    async fn handle_event(&mut self, event: &Event) {
        let Event::Key(key) = event else { return };
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.state = AppState::Exit,
            // allows left or right arrow keys and vim ('l' & 'h') to navigate tabs
            KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
            KeyCode::Char('h') | KeyCode::Left => self.prev_tab(),
            KeyCode::Char('j') | KeyCode::Up => self.next(),
            KeyCode::Char('k') | KeyCode::Down => self.prev(),
            KeyCode::Char('m') => self.tab(CurrentTab::Home),
            KeyCode::Char('t') => self.tab(CurrentTab::Timer),
            KeyCode::Char('a') => self.tab(CurrentTab::Stats),
            KeyCode::Char('e') => self.tab(CurrentTab::Settings),
            KeyCode::Char('s') => {
                if self.current_tab != CurrentTab::Timer {
                    self.tab(CurrentTab::Timer)
                }

                self.timer_tab.run().await;
            }
            _ => {}
        };
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    pub fn prev_tab(&mut self) {
        self.current_tab = self.current_tab.prev();
    }

    pub fn tab(&mut self, new_tab: CurrentTab) {
        self.current_tab = new_tab;
    }

    fn next(&mut self) {
        match self.current_tab {
            CurrentTab::Home => self.home_tab.next_row(),
            CurrentTab::Settings => self.settings_tab.next_row(),
            CurrentTab::Stats => {}
            CurrentTab::Timer => {}
        }
    }

    fn prev(&mut self) {
        match self.current_tab {
            CurrentTab::Home => self.home_tab.prev_row(),
            CurrentTab::Settings => self.settings_tab.prev_row(),
            CurrentTab::Stats => {}
            CurrentTab::Timer => {}
        }
    }

    pub fn get(&'static self) -> &'static Self {
        self
    }

    pub fn quit(&mut self) {
        self.state = AppState::Exit;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
        let [header, middle] = vertical.areas(area);
        self.render_header(header, buf);
        self.render_middle(middle, buf);
        // render bottom bar? called from App not self?
    }
}

impl App {
    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([Constraint::Min(0)]);
        let [tabs] = layout.areas(area);

        let titles = CurrentTab::iter().map(|t| t.to_string());
        Tabs::new(titles)
            .select(self.current_tab as usize)
            .divider(symbols::DOT)
            .block(Block::bordered())
            .render(tabs, buf);
    }

    fn render_middle(&self, area: Rect, buf: &mut Buffer) {
        match self.current_tab {
            CurrentTab::Home => self.home_tab.render(area, buf),
            CurrentTab::Settings => self.settings_tab.render(area, buf),
            CurrentTab::Stats => self.stats_tab.render(area, buf),
            CurrentTab::Timer => self.timer_tab.render(area, buf),
        }
    }
}

impl CurrentTab {
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
}
