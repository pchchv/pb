//! # Terminal progress bar for Rust
//!
//! Console progress bar for Rust.
// Macro for writing to the giving writer.
// Used in both pb.rs and multi.rs modules.
//
// # Examples
//
// ```
// let w = io::stdout();
// printfl!(w, "");
// printfl!(w, "\r{}", out);
//
// ```
macro_rules! printfl {
    ($w:expr, $($tt:tt)*) => {{
        $w.write_all(&format!($($tt)*).as_bytes()).ok().expect("write() fail");
        $w.flush().ok().expect("flush() fail");
    }}
}

mod pb;
mod tty;
use std::io::Write;
pub use pb::ProgressBar;


pub struct PbIter<T, I>
where
I: Iterator,
T: Write,
{
    iter: I,
    progress_bar: ProgressBar<T>,
}
