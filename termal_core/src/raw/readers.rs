use std::io::{stdout, Write};

use crate::{codes, error::Result, raw::events::Key};

use super::{
    events::{Event, KeyCode},
    Terminal,
};

pub trait Predicate<T> {
    fn matches(&self, value: &T) -> bool;
}

impl<F, T> Predicate<T> for F
where
    F: Fn(&T) -> bool,
{
    fn matches(&self, value: &T) -> bool {
        self(value)
    }
}

pub struct TermRead<'a, P>
where
    P: Predicate<Event>,
{
    buf: Vec<char>,
    pbuf: String,
    pos: usize,
    term: &'a mut Terminal,
    exit: P,
}

impl Predicate<Event> for KeyCode {
    fn matches(&self, value: &Event) -> bool {
        matches!(value, Event::KeyPress(Key { code, .. }) if code == self)
    }
}

impl<'a, P> From<TermRead<'a, P>> for Vec<char>
where
    P: Predicate<Event>,
{
    fn from(value: TermRead<'a, P>) -> Self {
        value.buf
    }
}

impl<'a> TermRead<'a, KeyCode> {
    pub fn lines(term: &'a mut Terminal) -> Self {
        Self::new(term, KeyCode::Enter)
    }
}

impl<'a, P> TermRead<'a, P>
where
    P: Predicate<Event>,
{
    pub fn new(term: &'a mut Terminal, exit: P) -> Self {
        Self::reuse(term, exit, vec![])
    }

    pub(crate) fn reuse(
        term: &'a mut Terminal,
        exit: P,
        mut buf: Vec<char>,
    ) -> Self {
        buf.clear();
        Self {
            buf,
            pbuf: String::new(),
            pos: 0,
            term,
            exit,
        }
    }

    pub fn read_to_str(&mut self, s: &mut String) -> Result<()> {
        self.init()?;
        self.get_all()?;
        s.extend(self.buf.iter().copied());
        self.buf.clear();
        Ok(())
    }

    pub fn read_str(&mut self) -> Result<String> {
        self.init()?;
        let mut s = String::new();
        self.read_to_str(&mut s)?;
        Ok(s)
    }

    pub fn init(&mut self) -> Result<()> {
        print_str(codes::CUR_SAVE)?;
        Ok(())
    }

    fn get_all(&mut self) -> Result<()> {
        while !self.read_next()? {}
        Ok(())
    }

    fn read_next(&mut self) -> Result<bool> {
        let evt = self.term.read()?;
        if self.exit.matches(&evt) {
            self.end();
            self.commit()?;
            return Ok(true);
        }

        let Event::KeyPress(key) = evt else {
            return Ok(false);
        };

        if let Some(chr) = key.key_char {
            self.buf.insert(self.pos, chr);

            if self.pos + 1 < self.buf.len() {
                self.reprint_pos();
                self.move_right()
            } else {
                if key.code == KeyCode::Enter {
                    print_str("\r\n")?;
                } else {
                    print_char(chr)?;
                }
                self.pos += 1;
            }

            self.commit()?;
            return Ok(false);
        }

        match key.code {
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Delete => self.delete(),
            KeyCode::Home => self.home(),
            KeyCode::End => self.end(),
            _ => {}
        }

        self.commit()?;

        Ok(false)
    }

    fn home(&mut self) {
        self.pos = 0;
        self.pbuf += codes::CUR_LOAD;
    }

    fn end(&mut self) {
        self.pos = self.buf.len();
        self.move_to_pos();
    }

    fn move_left(&mut self) {
        if self.pos != 0 {
            self.pos -= 1;
            self.pbuf += codes::move_left!(1);
        }
    }

    fn move_right(&mut self) {
        if self.pos < self.buf.len() {
            self.pos += 1;
            self.pbuf += codes::move_right!(1);
        }
    }

    fn delete(&mut self) {
        if self.pos < self.buf.len() {
            self.buf.remove(self.pos);
            self.reprint_pos();
        }
    }

    fn backspace(&mut self) {
        if self.pos != 0 {
            self.move_left();
            self.delete();
        }
    }

    fn move_to_pos(&mut self) {
        self.pbuf += codes::CUR_LOAD;
        self.pbuf += &codes::move_right!(self.pos);
    }

    fn reprint_pos(&mut self) {
        self.pbuf += codes::ERASE_TO_END;
        self.pbuf.extend(self.buf[self.pos..].iter().flat_map(|c| Some(c).into_iter().chain(if *c == '\n' { Some(&'\r') } else { None })));
        self.move_to_pos();
    }

    fn commit(&mut self) -> Result<()> {
        if !self.pbuf.is_empty() {
            print_str(&self.pbuf)?;
            self.pbuf.clear();
        }
        Ok(())
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
    out.write_all(s.as_bytes())?;
    out.flush()?;
    Ok(())
}
