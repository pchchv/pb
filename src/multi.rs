use std::io::Write;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use crossbeam_channel::{Receiver, Sender};

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

pub struct MultiBar<T: Write> {
    state: Mutex<State<T>>,
    chan: (Sender<WriteMsg>, Receiver<WriteMsg>),
    nbars: AtomicUsize,
}

pub struct Pipe {
    level: usize,
    chan: Sender<WriteMsg>,
}
