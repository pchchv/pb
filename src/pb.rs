use std::io::Write;
use std::time::{Duration, Instant};

// Output type format,
// indicate which format wil be used in the speed box.
#[derive(Debug)]
pub enum Units {
    Default,
    Bytes,
}

pub struct ProgressBar<T: Write> {
    start_time: Instant,
    units: Units,
    pub total: u64,
    current: u64,
    bar_start: String,
    bar_current: String,
    bar_current_n: String,
    bar_remain: String,
    bar_end: String,
    tick: Vec<String>,
    tick_state: usize,
    width: Option<usize>,
    message: String,
    last_refresh_time: Instant,
    max_refresh_rate: Option<Duration>,
    pub is_finish: bool,
    pub is_multibar: bool,
    pub show_bar: bool,
    pub show_speed: bool,
    pub show_percent: bool,
    pub show_counter: bool,
    pub show_time_left: bool,
    pub show_tick: bool,
    pub show_message: bool,
    handle: T,
}

impl<T: Write> ProgressBar<T> {
    /// Set custom format to the drawing bar, default is `[=>-]`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut pb = ProgressBar::new(...);
    /// pb.format("[=>_]");
    /// ```
    pub fn format(&mut self, fmt: &str) {
        if fmt.len() >= 5 {
            let v: Vec<&str> = fmt.split("").collect();
            self.bar_start = v[1].to_owned();
            self.bar_current = v[2].to_owned();
            self.bar_current_n = v[3].to_owned();
            self.bar_remain = v[4].to_owned();
            self.bar_end = v[5].to_owned();
        }
    }
}
