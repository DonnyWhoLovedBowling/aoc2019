use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String
}

impl <R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines{reader: io::BufReader::new(r), buf: String::new()}
    }
    fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>>{
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(nbytes) => if nbytes == 0 {
                None // no more lines!
            } else {
                let line = self.buf.trim_right();
                Some(Ok(line))
            },
            Err(e) => Some(Err(e))
        }
    }
}
