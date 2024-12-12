//! A simple utility for getting the size of a terminal, and moving `n` lines up.
//!
//! Supports both Linux and Windows.
//!
//!

#[derive(Debug)]
pub struct Width(pub u16);
#[derive(Debug)]
pub struct Height(pub u16);

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::*;
