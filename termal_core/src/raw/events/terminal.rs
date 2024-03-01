use std::{collections::VecDeque, io::{self, BufRead}};

use crate::error::{Error, Result};

pub struct Terminal {
    buffer: VecDeque<u8>,
}

impl Terminal {
    pub fn read(&mut self) {

    }

    fn read_byte(&mut self) -> Result<u8> {
        if let Some(b) = self.buffer.pop_front() {
            return Ok(b);
        }
        self.fill_buffer()?;
        self.buffer.pop_front().ok_or(Error::StdInEof)
    }

    fn fill_buffer(&mut self) -> Result<()> {
        let mut stdio = io::stdin().lock();
        let buf = stdio.fill_buf()?;
        self.buffer.extend(buf);
        let len = buf.len();
        stdio.consume(len);
        Ok(())
    }
}
