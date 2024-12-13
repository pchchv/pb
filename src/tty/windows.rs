use super::{Height, Width};

/// Returns the size of the terminal, if available.
/// Note that this returns the size of the actual command window,
/// and not the overall size of the command window buffer
pub fn terminal_size() -> Option<(Width, Height)> {
    if let Some((_, csbi)) = get_csbi() {
        let w: Width = Width((csbi.srWindow.Right - csbi.srWindow.Left) as u16);
        let h: Height = Height((csbi.srWindow.Bottom - csbi.srWindow.Top) as u16);
        Some((w, h))
    } else {
        None
    }
}

/// move the cursor `n` lines up;
/// return an empty string,
/// just to be aligned with the unix version.
pub fn move_cursor_up(n: usize) -> String {
    use winapi::um::wincon::{SetConsoleCursorPosition, COORD};
    if let Some((hand, csbi)) = get_csbi() {
        unsafe {
            SetConsoleCursorPosition(
                hand,
                COORD {
                    X: 0,
                    Y: csbi.dwCursorPosition.Y - n as i16,
                },
            );
        }
    }
    "".to_string()
}
