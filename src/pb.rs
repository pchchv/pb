use std::io::Write;
use std::io::Stdout;
use std::time::{Duration, Instant};

const FORMAT: &str = "[=>-]";
const TICK_FORMAT: &str = "\\|/-";

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

impl ProgressBar<Stdout> {
    /// Create a new ProgressBar with default configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::thread;
    /// use pbr::{ProgressBar, Units};
    ///
    /// let count = 1000;
    /// let mut pb = ProgressBar::new(count);
    /// pb.set_units(Units::Bytes);
    ///
    /// for _ in 0..count {
    ///    pb.inc();
    ///    thread::sleep_ms(100);
    /// }
    /// ```
    pub fn new(total: u64) -> ProgressBar<Stdout> {
        let handle = ::std::io::stdout();
        ProgressBar::on(handle, total)
    }
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

    /// Set tick format for the progressBar, default is \\|/-
    ///
    /// Format is not limited to 4 characters,
    /// any string can be used as a tick format
    /// (the tick will successively take the value of
    /// each char but won't loop backwards).
    ///
    /// # Examples
    /// ```ignore
    /// let mut pb = ProgressBar::new(...);
    /// pb.tick_format("▀▐▄▌")
    /// ```
    pub fn tick_format(&mut self, tick_fmt: &str) {
        if tick_fmt != TICK_FORMAT {
            self.show_tick = true;
        }
        self.tick = tick_fmt
            .split("")
            .map(|x| x.to_owned())
            .filter(|x| !x.is_empty())
            .collect();
    }

    /// Create a new ProgressBar with default configuration
    /// but pass an arbitrary writer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::thread;
    /// use std::io::stderr;
    /// use pbr::{ProgressBar, Units};
    ///
    /// let count = 1000;
    /// let mut pb = ProgressBar::on(stderr(), count);
    /// pb.set_units(Units::Bytes);
    ///
    /// for _ in 0..count {
    ///    pb.inc();
    ///    thread::sleep_ms(100);
    /// }
    /// ```
    pub fn on(handle: T, total: u64) -> ProgressBar<T> {
        let mut pb = ProgressBar {
            total,
            current: 0,
            start_time: Instant::now(),
            units: Units::Default,
            is_finish: false,
            is_multibar: false,
            show_bar: true,
            show_speed: true,
            show_percent: true,
            show_counter: true,
            show_time_left: true,
            show_tick: false,
            show_message: true,
            bar_start: String::new(),
            bar_current: String::new(),
            bar_current_n: String::new(),
            bar_remain: String::new(),
            bar_end: String::new(),
            tick: Vec::new(),
            tick_state: 0,
            width: None,
            message: String::new(),
            last_refresh_time: Instant::now(),
            max_refresh_rate: None,
            handle,
        };
        pb.format(FORMAT);
        pb.tick_format(TICK_FORMAT);
        pb
    }
}
