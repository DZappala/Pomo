use tokio::{
    sync::mpsc::UnboundedSender,
    time::{interval, Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct Timer {
    pub sender: UnboundedSender<u16>,
    pub secs: u16,
    pub progress: u16,
    pub is_paused: bool,
    pub started_at: Instant,
    pub ended_at: Option<Instant>,
}

impl Timer {
    pub fn new(sender: UnboundedSender<u16>) -> Self {
        let new_timer = Self {
            sender,
            // secs: 1500,
            secs: 100,
            progress: 0,
            is_paused: false,
            started_at: Instant::now(),
            ended_at: None,
        };
        new_timer
    }

    pub fn run(self) {
        tokio::spawn(async move {
            self.start().await;
        });
    }

    async fn start(mut self) {
        let mut interval = interval(Duration::from_secs(1));
        for i in 0..self.secs {
            if self.is_paused {
                self.secs -= self.progress;
                break;
            }

            interval.tick().await;
            self.progress = ((i as f32 / self.secs as f32) * 100.) as u16;
            self.sender.send(self.progress).unwrap();
        }
    }
}
