use std::{
    cmp::Ordering,
    io::{stdout, IsTerminal, Write},
    mem,
    time::Duration,
};

use crate::{
    codes,
    error::{Error, Result},
    raw::{
        events::{Event, KeyCode, Modifiers},
        term_size, wait_for_stdin, Terminal,
    },
    term_text::TermText,
};

use super::{Predicate, ReadConf};

/// Terminal reader. Supports only single line. Newlines are skipped.
pub struct TermRead<'a, P>
where
    P: Predicate<Event>,
{
    buf: Vec<char>,
    pbuf: String,
    pos: usize,
    term: &'a mut Terminal,
    exit: P,
    size: (usize, usize),
}

impl<'a> TermRead<'a, KeyCode> {
    /// Gets reader that ends on enter.
    pub fn lines(term: &'a mut Terminal) -> Self {
        Self::new(term, KeyCode::Enter)
    }
}

impl<'a, P> TermRead<'a, P>
where
    P: Predicate<Event>,
{
    /// Creates new terminal reader that exits with the given predicate.
    pub fn new(term: &'a mut Terminal, exit: P) -> Self {
        Self::from_config(term, exit, Default::default())
    }

    /// Create terminal reader from configuration.
    pub fn from_config(
        term: &'a mut Terminal,
        exit: P,
        mut conf: ReadConf,
    ) -> Self {
        let pos = conf
            .edit_pos
            .unwrap_or(conf.edit.len())
            .min(conf.edit.len());
        conf.edit.retain(|c| !c.is_ascii_control());
        Self {
            buf: conf.edit,
            pbuf: String::new(),
            pos,
            term,
            exit,
            size: (usize::MAX, usize::MAX),
        }
    }

    /// Edit the given string.
    pub fn edit_str(
        &mut self,
        s: &mut String,
        pos: Option<usize>,
    ) -> Result<()> {
        self.set_edit(&s, pos);
        self.reshow()?;
        s.clear();
        self.finish_to_str(s)
    }

    /// Edit the given string. Return new edited string (the old is unchanged).
    pub fn edit(
        &mut self,
        s: impl AsRef<str>,
        pos: Option<usize>,
    ) -> Result<String> {
        self.set_edit(s, pos);
        self.reshow()?;
        self.finish()
    }

    /// Edit the given vector. This is the most optimal edit as it doesn't do
    /// any copy of the passed data.
    pub fn edit_vec(
        &mut self,
        s: &mut Vec<char>,
        pos: Option<usize>,
    ) -> Result<()> {
        mem::swap(&mut self.buf, s);
        // TODO: change when lines are supported
        self.buf.retain(|c| !matches!(c, '\n' | '\r'));
        self.set_pos(pos);
        self.reshow()?;
        self.get_all()?;
        mem::swap(&mut self.buf, s);
        self.clear();
        Ok(())
    }

    /// Appends readed text to a string.
    pub fn read_to_str(&mut self, s: &mut String) -> Result<()> {
        self.clear();
        self.reshow()?;
        self.finish_to_str(s)
    }

    /// Reads from console and returns the readed string.
    pub fn read_str(&mut self) -> Result<String> {
        self.clear();
        self.reshow()?;
        self.finish()
    }

    /// Continue reading all data and reset.
    pub fn finish_to_str(&mut self, s: &mut String) -> Result<()> {
        self.get_all()?;
        s.extend(&self.buf);
        self.clear();
        Ok(())
    }

    /// Continue reading all the data and reset.
    pub fn finish(&mut self) -> Result<String> {
        let mut r = String::new();
        self.finish_to_str(&mut r)?;
        Ok(r)
    }

    /// Get the readed characters.
    pub fn get_readed(&self) -> &[char] {
        &self.buf
    }

    /// Get the position of cursor within the readed characters.
    pub fn get_pos(&self) -> usize {
        self.pos
    }

    /// Set string to edit.
    pub fn set_edit(&mut self, s: impl AsRef<str>, pos: Option<usize>) {
        self.buf.clear();
        // TODO: change when newlines supported
        self.buf.extend(
            TermText::new(s.as_ref())
                .spans()
                .filter(|s| !s.is_control())
                .flat_map(|s| s.text().chars()),
        );
        self.set_pos(pos);
    }

    pub fn set_pos(&mut self, pos: Option<usize>) {
        self.pos = pos.unwrap_or(self.buf.len()).min(self.buf.len());
    }

    /// Reset the buffer.
    pub fn clear(&mut self) {
        self.pos = 0;
        self.buf.clear();
    }

    /// Refresh the view.
    pub fn reshow(&mut self) -> Result<()> {
        self.reprint_all();
        self.commit()
    }

    fn get_all(&mut self) -> Result<()> {
        loop {
            while matches!(
                wait_for_stdin(Duration::from_millis(100)),
                Ok(false)
            ) {
                self.resize();
                self.commit()?;
            }
            self.resize();
            self.commit()?;
            if self.read_next()? {
                return Ok(());
            }
            while self.term.has_buffered_input() {
                if self.read_next()? {
                    return Ok(());
                }
            }
        }
    }

    fn resize(&mut self) {
        let Ok(size) = term_size().map(|s| (s.char_width, s.char_height))
        else {
            return;
        };
        if self.size == size {
            return;
        }
        let Ok(size) = term_size().map(|s| (s.char_width, s.char_height))
        else {
            return;
        };
        let pos = self.cur_pos();
        if pos.0 == 0 && pos.1 != 0 && self.pos == self.buf.len() {
            if size.0 > self.size.0 {
                self.pbuf += &codes::move_up!(pos.1);
            } else {
                self.pbuf += &codes::move_up!(
                    self.pos / size.0 + (self.pos % size.0 > 0) as usize
                );
            }
        }
        self.pbuf += &codes::move_left!(pos.0);
        self.size = size;
        let pos = self.pos;
        self.reprint_dont_move(0);
        self.move_to_pos(pos);
    }

    fn read_next(&mut self) -> Result<bool> {
        let evt = match self.term.read() {
            Ok(e) => e,
            Err(Error::StdInEof) => {
                self.end();
                self.commit()?;
                return Ok(true);
            }
            Err(e) => Err(e)?,
        };

        if self.exit.matches(&evt) {
            self.end();
            self.commit()?;
            return Ok(true);
        }

        let Event::KeyPress(key) = evt else {
            return Ok(false);
        };

        if let Some(chr) = key.key_char {
            // TODO: change when newlines are supported.
            if chr == '\n' {
                return Ok(false);
            }

            self.buf.insert(self.pos, chr);

            if self.pos + 1 < self.buf.len() {
                self.reprint_pos();
                self.move_right();
            } else {
                if chr == '\n' {
                    self.pbuf += "\r\n";
                } else {
                    self.pbuf.push(chr);
                }
                self.pos += 1;
                if self.cur_pos().0 == 0 {
                    self.pbuf += "\r\n";
                }
            }

            self.commit()?;
            return Ok(false);
        }

        match key.code {
            KeyCode::Left => {
                if key.modifiers.contains(Modifiers::CONTROL) {
                    self.move_word_left();
                } else {
                    self.move_left()
                }
            }
            KeyCode::Right => {
                if key.modifiers.contains(Modifiers::CONTROL) {
                    self.move_word_right();
                } else {
                    self.move_right()
                }
            }
            KeyCode::Backspace => self.backspace(),
            KeyCode::Delete => self.delete(),
            KeyCode::Home => self.home(),
            KeyCode::End => self.end(),
            _ => {}
        }

        self.commit()?;

        Ok(false)
    }

    fn move_word_right(&mut self) {
        let mut pos = self.pos;
        pos = pos.min(self.buf.len());
        while pos < self.buf.len() && self.buf[pos].is_ascii_whitespace() {
            pos += 1;
        }
        while pos < self.buf.len() && !self.buf[pos].is_ascii_whitespace() {
            pos += 1;
        }
        self.move_to_pos(pos);
    }

    fn move_word_left(&mut self) {
        let mut pos = self.pos;
        pos = pos.saturating_sub(1);
        while pos > 0 && self.buf[pos].is_ascii_whitespace() {
            pos -= 1;
        }
        while pos > 0 && !self.buf[pos].is_ascii_whitespace() {
            pos -= 1;
        }
        if pos < self.buf.len() && self.buf[pos].is_ascii_whitespace() {
            pos += 1;
        }
        self.move_to_pos(pos);
    }

    fn cur_pos(&self) -> (usize, usize) {
        (self.pos % self.size.0, self.pos / self.size.0)
    }

    fn move_start(&mut self) {
        let pos = self.cur_pos();
        self.pbuf += &codes::move_left!(pos.0);
        self.pbuf += &codes::move_up!(pos.1);
    }

    fn home(&mut self) {
        self.move_start();
        self.pos = 0;
    }

    fn end(&mut self) {
        self.move_to_pos(self.buf.len());
    }

    fn move_left(&mut self) {
        if self.pos != 0 {
            self.move_to_pos(self.pos - 1);
        }
    }

    fn move_right(&mut self) {
        if self.pos < self.buf.len() {
            self.move_to_pos(self.pos + 1);
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

    fn move_to_pos(&mut self, pos: usize) {
        if pos == self.pos {
            return;
        }

        let old = self.cur_pos();
        self.pos = pos;
        let new = self.cur_pos();

        let new_line_adj = new.0.saturating_sub(old.0) > 0
            && new.0 == 0
            && !self.buf.is_empty();

        match new.0.cmp(&old.0) {
            Ordering::Greater => {
                self.pbuf += &codes::move_right!(new.0 - old.0)
            }
            Ordering::Less => self.pbuf += &codes::move_left!(old.0 - new.0),
            _ => {}
        }
        match new.1.cmp(&old.1) {
            Ordering::Greater => {
                if new_line_adj {
                    self.pbuf += &codes::move_down!(new.1 - old.1 - 1);
                } else {
                    self.pbuf += &codes::move_down!(new.1 - old.1);
                }
            }
            Ordering::Less => self.pbuf += &codes::move_up!(old.1 - new.1),
            _ => {}
        }

        if new_line_adj {
            self.pbuf.push('\n');
        }
    }

    fn reprint_all(&mut self) {
        let pos = self.pos;
        self.reprint_dont_move(0);
        self.move_to_pos(pos);
    }

    fn reprint_pos(&mut self) {
        self.reprint_from(self.pos);
    }

    fn reprint_from(&mut self, pos: usize) {
        let save = self.pos;
        //self.move_to_pos(pos);

        self.reprint_dont_move(pos);
        self.move_to_pos(save);
    }

    fn reprint_dont_move(&mut self, pos: usize) {
        self.pbuf += codes::ERASE_TO_END;
        self.pbuf.extend(self.buf[pos..].iter().flat_map(|c| {
            Some(c).into_iter().chain(if *c == '\n' {
                Some(&'\r')
            } else {
                None
            })
        }));

        self.pos = self.buf.len();
        if self.cur_pos().0 == 0 && !self.buf.is_empty() {
            self.pbuf += "\r\n";
        }
    }

    fn commit(&mut self) -> Result<()> {
        if !self.pbuf.is_empty() {
            print_str(&self.pbuf)?;
            self.pbuf.clear();
        }
        Ok(())
    }
}

fn print_str(s: &str) -> Result<()> {
    let mut out = stdout().lock();
    if out.is_terminal() {
        out.write_all(s.as_bytes())?;
        out.flush()?;
    }
    Ok(())
}
