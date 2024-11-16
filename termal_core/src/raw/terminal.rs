use std::{
    collections::VecDeque,
    io::{self, BufRead},
    mem,
};

use crate::error::{Error, Result};

use super::{
    events::{AmbigousEvent, AnyEvent, Event, KeyCode},
    TermRead,
};

#[derive(Default)]
pub struct Terminal {
    buffer: VecDeque<u8>,
    line_buf: Vec<char>,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            buffer: VecDeque::new(),
            line_buf: vec![],
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

    pub fn read_byte(&mut self) -> Result<u8> {
        if let Some(b) = self.buffer.pop_front() {
            return Ok(b);
        }
        self.fill_buffer()?;
        self.buffer.pop_front().ok_or(Error::StdInEof)
    }

    pub fn read_line_to(&mut self, s: &mut String) -> Result<()> {
        let buf = mem::take(&mut self.line_buf);
        let mut reader = TermRead::reuse(self, KeyCode::Enter, buf);
        reader.read_to_str(s)?;
        self.line_buf = reader.into();
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut s = String::new();
        self.read_line_to(&mut s)?;
        Ok(s)
    }

    pub fn has_buffered_input(&self) -> bool {
        !self.buffer.is_empty()
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

        if cur == b'M' {
            // Special mouse event that actually doesn't conform to CSI
            // sequence rules.
            code.push(cur);
            for _ in 0..3 {
                if self.buffer.is_empty() {
                    return Ok(AmbigousEvent::from_code(&code));
                }
                code.push(self.read_byte()?);
            }
            return Ok(AmbigousEvent::from_code(&code));
        }

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
        let chr = self.read_utf8((&mut buf[1..]).try_into().unwrap())?;
        Ok(AmbigousEvent::from_code(&buf[..=chr.len_utf8()]))
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
