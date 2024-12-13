use std::sync::Mutex;
use std::str::from_utf8;
use std::io::{Write, Result};
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

impl Write for Pipe {
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let s = from_utf8(buf).unwrap().to_owned();
        self.chan
            .send(WriteMsg {
                // finish method emit empty string
                done: s.is_empty(),
                level: self.level,
                string: s,
            })
            .unwrap();
        Ok(buf.len())
    }
}