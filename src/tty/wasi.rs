use super::{Height, Width};

/// For WASI so far it will return none.
pub fn terminal_size() -> Option<(Width, Height)> {
    return None;
}
