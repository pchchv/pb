use std::io::Write;

struct State<T: Write> {
    lines: Vec<String>,
    nlines: usize,
    handle: T,
}

// WriteMsg is the message format used for
// communication between MultiBar and its bars.
struct WriteMsg {
    done: bool,
    level: usize,
    string: String,
}
