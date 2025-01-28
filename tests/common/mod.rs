use std::io::{BufRead, Read, Write};

use termal::{
    error::Result,
    raw::{IoProvider, ValueOrMut, WaitForIn},
};

pub struct BufProvider {
    buf: &'static [&'static [u8]],
    idx: usize,
    pos: usize,
    eof_panic: usize,
}

impl BufProvider {
    pub fn new(buf: &'static [&'static [u8]]) -> Self {
        Self {
            buf,
            idx: 0,
            pos: 0,
            eof_panic: 0,
        }
    }

    pub fn eof_panic(eof_cnt: usize, buf: &'static [&'static [u8]]) -> Self {
        Self {
            buf,
            idx: 0,
            pos: 0,
            eof_panic: eof_cnt + 1,
        }
    }

    pub fn has_data(&self) -> bool {
        self.idx < self.buf.len()
    }

    fn check_eof_panic(&mut self) {
        if self.eof_panic == 1 {
            panic!("Infinite wait.");
        }
        self.eof_panic = self.eof_panic.saturating_sub(1);
    }
}

impl Read for BufProvider {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if !self.has_data() {
            self.check_eof_panic();
            return Ok(0);
        }

        let len = buf.len().min(self.buf[self.idx].len());
        buf[..len].copy_from_slice(&self.buf[self.idx][..len]);
        self.consume(len);
        Ok(len)
    }
}

impl BufRead for BufProvider {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if !self.has_data() {
            self.check_eof_panic();
            Ok(&[])
        } else {
            Ok(&self.buf[self.idx][self.pos..])
        }
    }

    fn consume(&mut self, amt: usize) {
        if !self.has_data() {
            return;
        }
        self.pos += amt;
        if self.pos >= self.buf[self.idx].len() {
            self.pos = 0;
            self.idx += 1;
        }
    }
}

impl Write for BufProvider {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl WaitForIn for BufProvider {
    fn wait_for_in(&self, _timeout: std::time::Duration) -> Result<bool> {
        Ok(true)
    }
}

impl IoProvider for BufProvider {
    type Out = Self;
    type In = Self;

    fn get_out(&mut self) -> ValueOrMut<'_, Self::Out> {
        ValueOrMut::Mut(self)
    }

    fn get_in(&mut self) -> ValueOrMut<'_, Self::In> {
        ValueOrMut::Mut(self)
    }

    fn is_in_terminal(&self) -> bool {
        true
    }

    fn is_out_raw(&self) -> bool {
        true
    }

    fn is_out_terminal(&self) -> bool {
        true
    }
}
