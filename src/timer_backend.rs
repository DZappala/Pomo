use std::future::Future;
use tokio::{
    sync::mpsc::UnboundedSender,
    time::{interval, Duration, Instant},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TimerUsageMode {
    #[default]
    Normal,
    Zen,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TimerMode {
    #[default]
    Pomodoro,
    Basic,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Started,
    #[default]
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    pub secs: u16,
    pub progress: u16,
    pub is_paused: bool,
    pub started_at: Instant,
    pub ended_at: Option<Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            secs: 1500,
            progress: 0,
            is_paused: false,
            started_at: Instant::now(),
            ended_at: None,
        }
    }
}

impl Timer {
    pub fn run(mut self, sender: UnboundedSender<u16>) {
        tokio::spawn(async move {
            self.start(sender).await;
        });
    }

    const fn max_u16() -> u64 {
        u16::MAX as u64
    }

    pub fn set_timer_end(&mut self, at_secs: u16) {
        self.secs = at_secs;
    }

    pub async fn start(&mut self, sender: UnboundedSender<u16>) {
        let mut interval = interval(Duration::from_secs(1));
        for i in 0..self.secs {
            if self.is_paused {
                self.secs -= self.progress;
                break;
            }

            interval.tick().await;
            let secs = interval.period().as_secs();
            assert!(secs < Self::max_u16());
            sender.send((i / self.secs) * 100).unwrap();
        }
    }
}
