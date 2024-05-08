use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

use crate::error::{Error, Result};

use super::{AmbigousEvent, AnyEvent, Event};

#[derive(Default)]
pub struct Terminal {
    buffer: VecDeque<u8>,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            buffer: VecDeque::new(),
        }
    }

    pub fn read(&mut self) -> Result<Event> {
        loop {
            if let AnyEvent::Known(ev) = self.read_ambigous()?.event {
                return Ok(ev);
            }
        }
    }

    pub fn read_ambigous(&mut self) -> Result<AmbigousEvent> {
        if self.cur()? == 0x1b && self.buffer.len() != 1 {
            self.read_escape()
        } else {
            self.read_char()
        }
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

    fn cur(&mut self) -> Result<u8> {
        if let Some(b) = self.buffer.front() {
            Ok(*b)
        } else {
            self.fill_buffer()?;
            self.buffer.front().ok_or(Error::StdInEof).copied()
        }
    }

    fn read_escape(&mut self) -> Result<AmbigousEvent> {
        self.read_byte()?;
        let cur = self.cur()?;
        match cur {
            b'[' => self.read_csi(),
            b'O' if self.buffer.len() > 1 => self.read_ss3(),
            _ => self.read_alt(),
        }
    }

    fn read_csi(&mut self) -> Result<AmbigousEvent> {
        let mut code: Vec<_> = b"\x1b[".into();
        self.read_byte()?;
        if self.buffer.is_empty() {
            return Ok(AmbigousEvent::from_code(&code));
        }
        let mut cur = self.read_byte()?;

        while (0x30..=0x3F).contains(&cur) {
            code.push(cur);
            cur = self.read_byte()?;
        }

        while (0x20..=0x2F).contains(&cur) {
            code.push(cur);
            cur = self.read_byte()?;
        }

        code.push(cur);
        Ok(AmbigousEvent::from_code(&code))
    }

    fn read_ss3(&mut self) -> Result<AmbigousEvent> {
        let mut code: Vec<_> = b"\x1bO".into();
        self.read_byte()?;
        if self.buffer.is_empty() {
            return Ok(AmbigousEvent::from_code(&code));
        }
        let mut cur = self.read_byte()?;

        while (0x30..=0x3F).contains(&cur) {
            code.push(cur);
            cur = self.read_byte()?;
        }

        while (0x20..=0x2F).contains(&cur) {
            code.push(cur);
            cur = self.read_byte()?;
        }

        code.push(cur);
        Ok(AmbigousEvent::from_code(&code))
    }

    fn read_alt(&mut self) -> Result<AmbigousEvent> {
        let mut buf: [u8; 5] = [0x1b, 0, 0, 0, 0];
        self.read_utf8((&mut buf[1..]).try_into().unwrap())?;
        Ok(AmbigousEvent::from_code(&buf))
    }

    fn read_char(&mut self) -> Result<AmbigousEvent> {
        if !self.cur()?.is_ascii() {
            let mut buf: [u8; 4] = [0; 4];
            Ok(AmbigousEvent::from_char_code(self.read_utf8(&mut buf)?))
        } else {
            let chr = self.read_byte()? as char;
            Ok(AmbigousEvent::from_char_code(chr))
        }
    }

    fn read_utf8(&mut self, buf: &mut [u8; 4]) -> Result<char> {
        for i in 1..=4 {
            if self.buffer.len() < i {
                self.fill_buffer()?;
                if self.buffer.len() < i {
                    return Ok(self.read_byte()? as char);
                }
            }

            buf[i - 1] = self.buffer[i - 1];
            if let Ok(code) = std::str::from_utf8(&buf[..i]) {
                self.buffer.consume(i);
                return Ok(code.chars().next().unwrap());
            }
        }
        Ok(self.read_byte()? as char)
    }
}
