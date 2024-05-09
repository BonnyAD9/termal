use std::io::{stdout, Write};

use crate::{error::Result, raw::events::Key};

use super::{events::{Event, KeyCode}, Terminal};

pub trait Predicate<T> {
    fn matches(&self, value: &T) -> bool;
}

impl<F, T> Predicate<T> for F where F: Fn(&T) -> bool {
    fn matches(&self, value: &T) -> bool {
        self(value)
    }
}

pub struct TermRead<'a, P> where P: Predicate<Event> {
    buf: Vec<char>,
    pos: usize,
    term: &'a mut Terminal,
    exit: P,
}

impl Predicate<Event> for KeyCode {
    fn matches(&self, value: &Event) -> bool {
        matches!(value, Event::KeyPress(Key { code, .. }) if code == self)
    }
}

impl<'a, P> Into<Vec<char>> for TermRead<'a, P> where P: Predicate<Event> {
    fn into(self) -> Vec<char> {
        self.buf
    }
}

impl<'a> TermRead<'a, KeyCode> {
    pub fn lines(term: &'a mut Terminal) -> Self {
        Self::new(term, KeyCode::Enter)
    }
}

impl<'a, P> TermRead<'a, P> where P: Predicate<Event> {
    pub fn new(term: &'a mut Terminal, exit: P) -> Self {
        Self::reuse(term, exit, vec![])
    }

    pub(crate) fn reuse(term: &'a mut Terminal, exit: P, mut buf: Vec<char>) -> Self {
        buf.clear();
        Self {
            buf,
            pos: 0,
            term,
            exit
        }
    }

    pub fn read_to_str(&mut self, s: &mut String) -> Result<()> {
        self.get_all()?;
        s.extend(self.buf.iter().copied());
        self.buf.clear();
        Ok(())
    }

    pub fn read_str(&mut self) -> Result<String> {
        let mut s = String::new();
        self.read_to_str(&mut s)?;
        Ok(s)
    }

    fn get_all(&mut self) -> Result<()> {
        while !self.read_next()? {}
        Ok(())
    }

    fn read_next(&mut self) -> Result<bool> {
        let evt = self.term.read()?;
        let exit = self.exit.matches(&evt);

        let Event::KeyPress(key) = evt else {
            return Ok(exit)
        };

        if let Some(chr) = key.key_char {
            if key.code == KeyCode::Enter {
                print_str("\r\n")?;
            } else {
                print_char(chr)?;
            }

            self.buf.insert(self.pos, chr);
            self.pos += 1;
        }

        Ok(exit)
    }
}

fn print_char(chr: char) -> Result<()> {
    let mut out = stdout().lock();
    write!(out, "{chr}")?;
    out.flush()?;
    Ok(())
}

fn print_str(s: &str) -> Result<()> {
    let mut out = stdout().lock();
    out.write(s.as_bytes())?;
    out.flush()?;
    Ok(())
}
