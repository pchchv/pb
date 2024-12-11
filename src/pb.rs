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
