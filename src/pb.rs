use std::io::{Stdout, Write};
use std::time::{Duration, Instant};
use crate::tty::{terminal_size, Width};

macro_rules! kb_fmt {
    ($n: ident) => {{
        let kb = 1024f64;
        match $n {
            $n if $n >= kb.powf(4_f64) => format!("{:.*} TB", 2, $n / kb.powf(4_f64)),
            $n if $n >= kb.powf(3_f64) => format!("{:.*} GB", 2, $n / kb.powf(3_f64)),
            $n if $n >= kb.powf(2_f64) => format!("{:.*} MB", 2, $n / kb.powf(2_f64)),
            $n if $n >= kb => format!("{:.*} KB", 2, $n / kb),
            _ => format!("{:.*} B", 0, $n),
        }
    }};
}

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

    /// Set width, or `None` for default.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut pb = ProgressBar::new(...);
    /// pb.set_width(Some(80));
    /// ```
    pub fn set_width(&mut self, w: Option<usize>) {
        self.width = w;
    }

    /// Set units, default is simple numbers.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pbr::{ProgressBar, Units};
    ///
    /// let n_bytes = 100;
    /// let mut pb = ProgressBar::new(n_bytes);
    /// pb.set_units(Units::Bytes);
    /// ```
    pub fn set_units(&mut self, u: Units) {
        self.units = u;
    }

    /// Set max refresh rate,
    /// above which the progress bar will not redraw,
    /// or `None` for none.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut pb = ProgressBar::new(...);
    /// pb.set_max_refresh_rate(Some(Duration::from_millis(100)));
    /// ```
    pub fn set_max_refresh_rate(&mut self, w: Option<Duration>) {
        self.max_refresh_rate = w;
        if let Some(dur) = self.max_refresh_rate {
            self.last_refresh_time = self.last_refresh_time - dur;
        }
    }

    /// Resets the start time to now
    pub fn reset_start_time(&mut self) {
        self.start_time = Instant::now();
    }

    /// Set message to display in the prefix,
    /// call with "" to stop printing a message.
    ///
    /// All newlines are replaced with spaces.
    ///
    /// # Examples
    /// ```ignore
    /// let mut pb = ProgressBar::new(20);
    ///
    /// for x in 0..20 {
    ///    match x {
    ///       0 => pb.message("Doing 1st Quarter"),
    ///       5 => pb.message("Doing 2nd Quarter"),
    ///       10 => pb.message("Doing 3rd Quarter"),
    ///       15 => pb.message("Doing 4th Quarter"),
    ///    }
    ///    pb.inc().
    /// }
    ///
    /// ```
    pub fn message(&mut self, message: &str) {
        self.message = message.replace(['\n', '\r'], " ")
    }

    /// Get terminal width, from configuration, terminal size, or default(80)
    fn width(&mut self) -> usize {
        if let Some(w) = self.width {
            w
        } else if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80
        }
    }

    fn draw(&mut self) {
        let now = Instant::now();
        if let Some(mrr) = self.max_refresh_rate {
            if now - self.last_refresh_time < mrr && self.current < self.total {
                return;
            }
        }

        let mut time_elapsed = now - self.start_time;
        if time_elapsed.is_zero() {
            time_elapsed = Duration::from_nanos(1);
        }

        let speed = self.current as f64 / time_elapsed.as_secs_f64();
        let width = self.width();
        let mut out;
        let mut parts = Vec::new();
        let mut base = String::new();
        let mut prefix = String::new();
        let mut suffix = String::from(" ");
        // precent box
        if self.show_percent {
            let percent = self.current as f64 / (self.total as f64 / 100f64);
            parts.push(format!(
                "{:.*} %",
                2,
                if percent.is_nan() { 0.0 } else { percent }
            ));
        }

        // speed box
        if self.show_speed {
            match self.units {
                Units::Default => parts.push(format!("{:.*}/s", 2, speed)),
                Units::Bytes => parts.push(format!("{}/s", kb_fmt!(speed))),
            };
        }

        // time left box
        if self.show_time_left && self.current > 0 && self.total > self.current {
            let left = 1. / speed * (self.total - self.current) as f64;
            if left < 60. {
                parts.push(format!("{:.0}s", left));
            } else {
                parts.push(format!("{:.0}m", left / 60.));
            };
        }

        suffix += &parts.join(" ");
        // message box
        if self.show_message {
            prefix = prefix + &self.message;
        }

        // counter box
        if self.show_counter {
            let (c, t) = (self.current as f64, self.total as f64);
            prefix = prefix
                + &match self.units {
                    Units::Default => format!("{} / {} ", c, t),
                    Units::Bytes => format!("{} / {} ", kb_fmt!(c), kb_fmt!(t)),
                };
        }

        // tick box
        if self.show_tick {
            prefix = prefix + &format!("{} ", self.tick[self.tick_state]);
        }

        // bar box
        if self.show_bar {
            let p = prefix.chars().count() + suffix.chars().count() + 3;
            if p < width {
                let size = width - p;
                let curr_count =
                    ((self.current as f64 / self.total as f64) * size as f64).ceil() as usize;
                if size >= curr_count {
                    let rema_count = size - curr_count;
                    base = self.bar_start.clone();
                    if rema_count > 0 && curr_count > 0 {
                        base =
                            base + &self.bar_current.repeat(curr_count - 1) + &self.bar_current_n;
                    } else {
                        base = base + &self.bar_current.repeat(curr_count);
                    }
                    base = base + &self.bar_remain.repeat(rema_count) + &self.bar_end;
                }
            }
        }

        out = prefix + &base + &suffix;
        // pad
        if out.len() < width {
            let gap = width - out.len();
            out = out + &" ".repeat(gap);
        }

        printfl!(self.handle, "\r{}", out);

        self.last_refresh_time = Instant::now();
    }

    // finish_draw ensure that the progress bar is reached to its end,
    // and do the last drawing if needed.
    fn finish_draw(&mut self) {
        let mut redraw = false;
        if let Some(mrr) = self.max_refresh_rate {
            if Instant::now() - self.last_refresh_time < mrr {
                self.max_refresh_rate = None;
                redraw = true;
            }
        }

        if self.current < self.total {
            self.current = self.total;
            redraw = true;
        }

        if redraw {
            self.draw();
        }

        self.is_finish = true;
    }
