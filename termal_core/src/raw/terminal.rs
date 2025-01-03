use std::{
    collections::VecDeque,
    io::{self, BufRead, StdinLock},
    time::{Duration, Instant},
};

use crate::error::{Error, Result};

use super::{
    events::{AmbigousEvent, AnyEvent, Event, KeyCode},
    utf8_code_len, wait_for_stdin, TermRead,
};

/// Terminal reader. Abstracts reading from terminal and parsing inputs. Works
/// properly only if raw mode is enabled.
#[derive(Default)]
pub struct Terminal {
    buffer: VecDeque<u8>,
}

impl Terminal {
    /// Create new terminal.
    pub fn new() -> Self {
        Terminal {
            buffer: VecDeque::new(),
        }
    }

    /// Read the next known event on stdin. May block.
    pub fn read(&mut self) -> Result<Event> {
        loop {
            if let AnyEvent::Known(ev) = self.read_ambigous()?.event {
                return Ok(ev);
            }
        }
    }

    /// Read the next event on stdin. May block.
    pub fn read_ambigous(&mut self) -> Result<AmbigousEvent> {
        if self.cur()? == 0x1b && self.buffer.len() != 1 {
            self.read_escape()
        } else {
            self.read_char()
        }
    }

    /// Read next byte from stdin. May block.
    pub fn read_byte(&mut self) -> Result<u8> {
        if let Some(b) = self.buffer.pop_front() {
            return Ok(b);
        }
        self.fill_buffer()?;
        self.buffer.pop_front().ok_or(Error::StdInEof)
    }

    /// Appends next line of input from stdin to `s`. May block.
    pub fn read_line_to(&mut self, s: &mut String) -> Result<()> {
        let mut reader = TermRead::new(self, KeyCode::Enter);
        reader.read_to_str(s)?;
        Ok(())
    }

    /// Read the next line from stdin. May block.
    pub fn read_line(&mut self) -> Result<String> {
        let mut s = String::new();
        self.read_line_to(&mut s)?;
        Ok(s)
    }

    /// Checks whether there is any buffered input in [`Terminal`]
    pub fn has_buffered_input(&self) -> bool {
        !self.buffer.is_empty()
    }

    /// Checks whether the next input is available immidietely.
    pub fn has_input(&self) -> bool {
        self.has_buffered_input()
            || wait_for_stdin(Duration::ZERO).unwrap_or_default()
    }

    /// Wait for input on the terminal. Block for at most the given duration.
    pub fn wait_for_input(&self, timeout: Duration) -> Result<bool> {
        if self.has_buffered_input() {
            Ok(true)
        } else {
            wait_for_stdin(timeout)
        }
    }

    /// Read the next event on terminal. Block for at most the given duration.
    pub fn read_ambigous_timeout(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<AmbigousEvent>> {
        if self.wait_for_input(timeout)? {
            Ok(Some(self.read_ambigous()?))
        } else {
            Ok(None)
        }
    }

    /// Read the next known event on stdin. Block for at most the given
    /// duration.
    pub fn read_timeout(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>> {
        if self.wait_for_input(timeout)? {
            Ok(Some(self.read()?))
        } else {
            Ok(None)
        }
    }

    /// Read raw bytes from the terminal to `res`. Returns the number of readed
    /// bytes. Returns [`Error::StdInEof`] when reaches eof. May block.
    pub fn read_raw(&mut self, res: &mut [u8]) -> Result<usize> {
        let mut read = self.read_buffered(res)?;
        let mut stdin = io::stdin().lock();
        while read < res.len() {
            read += read_stdin_once(&mut stdin, &mut res[read..])?;
        }
        Ok(read)
    }

    /// Read raw bytes from the terminal to `res`. Returns the number of readed
    /// bytes. Returns [`Error::StdInEof`] when reaches eof. Block for at most
    /// the given duration for each read from the terminal (so possibly
    /// idefinitely if the input comes in periodically)
    pub fn read_raw_timeout(
        &mut self,
        res: &mut [u8],
        timeout: Duration,
    ) -> Result<usize> {
        let mut read = self.read_buffered(res)?;
        let mut stdin = io::stdin().lock();
        while read < res.len() && wait_for_stdin(timeout).unwrap_or_default() {
            read += read_stdin_once(&mut stdin, &mut res[read..])?;
        }
        Ok(read)
    }

    /// Read raw bytes from the terminal to `res`. Returns the number of readed
    /// bytes. Returns [`Error::StdInEof`] when reaches eof. Block for at most
    /// the given total duration.
    pub fn read_raw_single_timeout(
        &mut self,
        res: &mut [u8],
        mut timeout: Duration,
    ) -> Result<usize> {
        let mut read = self.read_buffered(res)?;
        let mut stdin = io::stdin().lock();
        while read < res.len() && timeout >= Duration::ZERO {
            let now = Instant::now();
            let ready = wait_for_stdin(timeout);
            timeout -= Instant::now() - now;
            if !ready.unwrap_or_default() {
                break;
            }
            read += read_stdin_once(&mut stdin, &mut res[read..])?;
        }
        Ok(read)
    }

    /// Read one byte from stdin. Block for at most the given timeout.
    pub fn read_byte_timeout(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<u8>> {
        if self.wait_for_input(timeout).unwrap_or_default() {
            Ok(Some(self.read_byte()?))
        } else {
            Ok(None)
        }
    }

    fn read_buffered(&mut self, mut res: &mut [u8]) -> Result<usize> {
        let (s1, s2) = self.buffer.as_slices();

        // Read from the first slice.
        if s1.len() >= res.len() {
            res.copy_from_slice(&s1[..res.len()]);
            self.buffer.consume(res.len());
            return Ok(res.len());
        }
        res[..s1.len()].copy_from_slice(s1);
        res = &mut res[s1.len()..];

        // Read from the second slice
        if s2.len() >= res.len() {
            res.copy_from_slice(&s2[..res.len()]);
            let read = s1.len() + res.len();
            self.buffer.consume(read);
            return Ok(read);
        }
        res[..s2.len()].copy_from_slice(s2);
        let read = s1.len() + s2.len();
        self.buffer.clear();
        Ok(read)
    }

    fn fill_buffer(&mut self) -> Result<()> {
        let mut stdin = io::stdin().lock();
        let buf = stdin.fill_buf()?;
        self.buffer.extend(buf);
        let len = buf.len();
        stdin.consume(len);
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
            b'P' => self.read_dcs(),
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
                let Some(b) = self.read_byte_if(|b| b >= 32)? else {
                    return Ok(AmbigousEvent::from_code(&code));
                };
                code.push(b);
            }
            if self.buffer.is_empty() {
                return Ok(AmbigousEvent::from_code(&code));
            }
            // UTF-8 extension
            for i in (1..=3).rev() {
                if !self.buffer.is_empty()
                    && utf8_code_len(code[code.len() - i]) != 2
                {
                    let Some(b) = self.read_byte_if(|b| b >= 32)? else {
                        return Ok(AmbigousEvent::from_code(&code));
                    };
                    code.push(b);
                }
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

    fn read_dcs(&mut self) -> Result<AmbigousEvent> {
        self.read_byte()?;
        let mut code: Vec<_> = b"\x1bP".into();
        while !self.buffer.is_empty() && !code.ends_with(b"\x1b\\") {
            code.push(self.read_byte()?);
        }
        Ok(AmbigousEvent::from_code(&code))
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

    fn read_byte_if(&mut self, p: impl Fn(u8) -> bool) -> Result<Option<u8>> {
        let c = self.read_byte()?;
        if p(c) {
            Ok(Some(c))
        } else {
            self.buffer.push_front(c);
            Ok(None)
        }
    }
}

fn read_stdin_once(
    stdin: &mut StdinLock<'static>,
    res: &mut [u8],
) -> Result<usize> {
    let buf = stdin.fill_buf()?;
    if buf.is_empty() {
        return Err(Error::StdInEof);
    }
    let len = buf.len().min(res.len());
    res[..len].copy_from_slice(&buf[..len]);
    stdin.consume(len);
    Ok(len)
}
