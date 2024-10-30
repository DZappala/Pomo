use ratatui::Frame;

const DELAY: usize = 120;

pub fn shutdown(frame: &mut Frame<'_>) {
    let frame_count = frame.count().saturating_sub(DELAY);
    if frame_count == 0 {
        return;
    }

    let area = frame.area();
    let buf = frame.buffer_mut();

    // other functios??
}
