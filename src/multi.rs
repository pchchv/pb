use std::io::Write;

struct State<T: Write> {
    lines: Vec<String>,
    nlines: usize,
    handle: T,
}
