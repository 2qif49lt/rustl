use tracing_subscriber::{self, fmt::time::FormatTime};
use chrono::Local;

pub fn init_log() {
    tracing_subscriber::fmt()
    .with_line_number(true)
    .with_thread_names(true)
    .with_target(false)
    .with_timer(SelfTimeFormater)
    .init();
}

struct SelfTimeFormater;

impl FormatTime for SelfTimeFormater {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().to_rfc3339())
    }   
}