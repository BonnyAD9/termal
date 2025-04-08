use std::{
    collections::VecDeque, io::Write, mem, ops::RangeBounds, time::Duration,
};

use crate::{
    codes,
    error::{Error, Result},
    raw::{
        IoProvider, StdioProvider, Terminal,
        events::{
            AmbigousEvent, AnyEvent, Event, Key, KeyCode, Modifiers, Status,
        },
        term_size,
    },
    term_text::TermText,
};

use super::{Predicate, ReadConf, Vec2};

/// Terminal reader. Supports only single line. Newlines are skipped.
///
/// ## Unstable API
///
/// API of [`TermRead`] will likely change in the future.
pub struct TermRead<'t, 'p, P, T: IoProvider = StdioProvider>
where
    P: Predicate<Event>,
{
    buf: Vec<char>,
    prompt: TermText<'p>,
    pbuf: String,
    pos: usize,
    term: &'t mut Terminal<T>,
    exit: P,
    size: Vec2,
    // TODO: use bitflags
    // TODO: option to exit on ctrl+c
    finished: bool,
    paste: bool,
    last_event: Option<Event>,
    queue: VecDeque<Event>,
}

impl<'t, T: IoProvider> TermRead<'t, '_, KeyCode, T> {
    /// Gets reader that ends on enter.
    pub fn lines(term: &'t mut Terminal<T>) -> Self {
        Self::new(term, KeyCode::Enter)
    }
}

impl<'t, 'p, P, T> TermRead<'t, 'p, P, T>
where
    P: Predicate<Event>,
    T: IoProvider,
{
    /// Creates new terminal reader that exits with the given predicate.
    pub fn new(term: &'t mut Terminal<T>, exit: P) -> Self {
        Self::from_config(term, exit, Default::default())
    }

    /// Create terminal reader from configuration.
    pub fn from_config(
        term: &'t mut Terminal<T>,
        exit: P,
        mut conf: ReadConf<'p>,
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
            prompt: conf.prompt,
            size: (usize::MAX, usize::MAX).into(),
            finished: false,
            paste: false,
            last_event: None,
            queue: VecDeque::new(),
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
    pub fn get_input(&self) -> &[char] {
        &self.buf
    }

    /// Get the position of cursor within the readed characters.
    pub fn get_pos(&self) -> usize {
        self.pos
    }

    /// Set string to edit.
    pub fn set_edit(&mut self, s: impl AsRef<str>, pos: Option<usize>) {
        self.buf.clear();
        self.buf.extend(s.as_ref().chars());
        self.set_pos(pos);
    }

    /// Set the position within the buffer.
    pub fn set_pos(&mut self, pos: Option<usize>) {
        self.pos = pos.unwrap_or(self.buf.len()).min(self.buf.len());
    }

    /// Reset the buffer.
    pub fn clear(&mut self) {
        self.pos = 0;
        self.buf.clear();
        self.finished = false;
    }

    /// Refresh the view.
    pub fn reshow(&mut self) -> Result<()> {
        self.reprint_all();
        self.commit()
    }

    /// Set the prompt.
    pub fn set_prompt(&mut self, prompt: impl Into<TermText<'p>>) {
        self.prompt = prompt.into();
    }

    /// Reconfigure the reader.
    pub fn configure(&mut self, conf: ReadConf<'p>) {
        self.set_buf(conf.edit, conf.edit_pos);
        self.set_prompt(conf.prompt);
    }

    /// Set the read buffer. It is filtered for non control characters.
    pub fn set_buf(&mut self, buf: Vec<char>, pos: Option<usize>) {
        self.buf = buf;
        self.buf.retain(|c| !c.is_ascii_control());
        self.set_pos(pos);
    }

    /// Read one next character or nothing. Doesn't block. Returns `true` if
    /// the input has ended and the result may be retrieved with
    /// [`TermRead::finish`] or [`TermRead::finish_to_str`].
    ///
    /// # Safety
    /// This is unsafe because calls to `set_` functions may break the input if
    /// this is called.
    pub unsafe fn read_one(&mut self) -> Result<bool> {
        if self.finished {
            return Ok(true);
        }

        self.finished = self.read_one_inner()?;
        Ok(self.finished)
    }

    /// Check if the reading is finished.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Sets the exit condition for the reader.
    pub fn set_end_condition(&mut self, c: P) {
        self.exit = c;
    }

    /// Modify the buffer. Control characters are ignored.
    pub fn splice(
        &mut self,
        range: impl RangeBounds<usize>,
        it: impl IntoIterator<Item = char>,
    ) {
        self.buf
            .splice(range, it.into_iter().filter(|c| !c.is_ascii_control()));
        self.pos = self.pos.min(self.buf.len());
    }

    /// Gets the last event.
    pub fn last_event(&self) -> Option<&Event> {
        self.last_event.as_ref()
    }

    /// Queue event to the reader.
    pub fn queue(&mut self, evt: impl IntoIterator<Item = Event>) {
        self.queue.extend(evt);
    }

    fn read_one_inner(&mut self) -> Result<bool> {
        if !self.queue.is_empty() || self.term.has_buffered_input() {
            return self.read_next();
        }

        let r = self.term.wait_for_input(Duration::from_millis(100));
        self.resize();
        self.commit()?;

        if matches!(r, Ok(false)) {
            self.last_event = None;
            return Ok(false);
        }

        self.read_next()
    }

    fn get_all(&mut self) -> Result<()> {
        if self.finished {
            return Ok(());
        }

        while !self.read_one_inner()? {}
        self.finished = true;
        Ok(())
    }

    fn resize(&mut self) {
        let Ok(size) =
            term_size().map(|s| Vec2::new(s.char_width, s.char_height))
        else {
            return;
        };
        self.size.map(|a| if a == 0 { usize::MAX } else { a });
        if self.size == size {
            return;
        }
        let pos = self.cur_pos();
        if pos.x == 0 && pos.y != 0 && self.pos == self.buf.len() {
            if size.x > self.size.x {
                self.pbuf += &codes::move_up!(pos.y);
            } else {
                self.pbuf += &codes::move_up!(
                    self.pos / size.x + (self.pos % size.y > 0) as usize
                );
            }
        }
        self.pbuf += &codes::move_left!(pos.x);
        self.size = size;
        let pos = self.pos;
        self.reprint_with_prompt_dont_move();
        self.move_to_pos(pos);
    }

    fn read_next(&mut self) -> Result<bool> {
        if let Some(evt) = self.queue.pop_front() {
            return self.handle_event(AmbigousEvent::event(evt));
        }

        let evt = match self.term.read_ambigous() {
            Ok(e) => e,
            Err(Error::StdInEof) => {
                self.end();
                self.commit()?;
                return Ok(true);
            }
            Err(e) => Err(e)?,
        };

        self.handle_event(evt)
    }

    fn handle_event(&mut self, evt: AmbigousEvent) -> Result<bool> {
        let AnyEvent::Known(known) = evt.event else {
            return Ok(false);
        };

        if self.exit.matches(&known) {
            self.last_event = Some(known);
            self.end();
            self.commit()?;
            return Ok(true);
        }

        match known {
            Event::KeyPress(key) => {
                self.last_event = Some(known);
                self.handle_key_press(key, evt.other.first())
            }
            Event::Status(Status::SelectionData(data)) => {
                if !self.paste {
                    return Ok(false);
                }
                self.paste = false;
                if let Ok(s) = std::str::from_utf8(&data) {
                    self.insert(s);
                }
                Ok(false)
            }
            _ => {
                self.last_event = Some(known);
                Ok(false)
            }
        }
    }

    fn handle_key_press(
        &mut self,
        key: Key,
        amb: Option<&Event>,
    ) -> Result<bool> {
        if let Some(chr) = key.key_char {
            self.buf.insert(self.pos, chr);

            if self.pos + 1 < self.buf.len() {
                self.reprint_pos();
                self.move_right();
            } else {
                self.print_from_dont_move(self.pos);
                self.pos += 1;
                if self.cur_pos().x == 0 {
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
            KeyCode::Backspace => {
                if key.modifiers.contains(Modifiers::CONTROL) {
                    self.backspace_word();
                } else {
                    self.backspace();
                }
            }
            KeyCode::Delete => {
                if key.modifiers.contains(Modifiers::CONTROL) {
                    self.delete_word();
                } else {
                    self.delete();
                }
            },
            KeyCode::Home => self.home(),
            KeyCode::End => self.end(),
            KeyCode::Char('v') => {
                if key.modifiers.contains(Modifiers::CONTROL) {
                    self.paste = true;
                    self.pbuf += codes::REQUEST_SELECTION;
                }
            }
            _ => {
                if let Some(Event::KeyPress(evt)) = amb {
                    let res = self.handle_key_press(*evt, None)?;
                    self.commit()?;
                    return Ok(res);
                }
            }
        }

        self.commit()?;

        Ok(false)
    }

    fn insert(&mut self, s: &str) {
        let len = self.buf.len();
        self.buf.splice(self.pos..self.pos, s.chars());
        self.reprint_from_move_to(self.pos, self.pos + self.buf.len() - len);
    }

    fn move_word_right(&mut self) {
        self.move_to_pos(self.word_pos_right());
    }

    fn move_word_left(&mut self) {
        self.move_to_pos(self.word_pos_left());
    }

    fn word_pos_left(&self) -> usize {
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
        pos.min(self.pos)
    }

    fn word_pos_right(&self) -> usize {
        let mut pos = self.pos;
        pos = pos.min(self.buf.len());
        while pos < self.buf.len() && self.buf[pos].is_ascii_whitespace() {
            pos += 1;
        }
        while pos < self.buf.len() && !self.buf[pos].is_ascii_whitespace() {
            pos += 1;
        }
        pos
    }

    /// Gets the position + prompt lentgth
    fn len(&self) -> usize {
        self.pos + self.prompt.display_char_cnt()
    }

    fn cur_pos(&self) -> Vec2 {
        self.size.pos_of_idx(self.len())
    }

    fn start_pos(&self) -> Vec2 {
        self.size.pos_of_idx(self.prompt.display_char_cnt())
    }

    fn move_start(&mut self) {
        self.move_rd_dif(self.start_pos(), self.cur_pos());
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

    fn delete_word(&mut self) {
        let pos = self.word_pos_right();
        if pos != self.pos {
            self.buf.splice(self.pos..pos, []);
            self.reprint_pos();
        }
    }

    fn backspace(&mut self) {
        if self.pos != 0 {
            self.move_left();
            self.delete();
        }
    }

    fn backspace_word(&mut self) {
        let pos = self.word_pos_left();
        if pos != self.pos {
            self.buf.splice(pos..self.pos, []);
            self.reprint_from_move_to(pos, pos);
        }
    }

    fn move_right_dif(&mut self, a: usize, b: usize) {
        if a > b {
            self.pbuf += &codes::move_right!(a - b);
        } else {
            self.pbuf += &codes::move_left!(b - a);
        }
    }

    fn move_down_dif(&mut self, a: usize, b: usize) {
        if a > b {
            self.pbuf += &codes::move_down!(a - b);
        } else {
            self.pbuf += &codes::move_up!(b - a);
        }
    }

    fn move_rd_dif(&mut self, a: Vec2, b: Vec2) {
        self.move_right_dif(a.x, b.x);
        self.move_down_dif(a.y, b.y);
    }

    fn move_to_pos(&mut self, pos: usize) {
        if pos == self.pos {
            return;
        }

        let old = self.cur_pos();
        self.pos = pos;
        let new = self.cur_pos();
        self.move_rd_dif(new, old);
    }

    fn reprint_all(&mut self) {
        let save = self.pos;
        self.move_rd_dif((0, 0).into(), self.cur_pos());
        self.reprint_with_prompt_dont_move();
        self.move_to_pos(save);
    }

    fn reprint_pos(&mut self) {
        self.reprint_from(self.pos);
    }

    fn reprint_from(&mut self, pos: usize) {
        self.reprint_from_move_to(pos, self.pos);
    }

    fn reprint_from_move_to(&mut self, from: usize, to: usize) {
        self.move_to_pos(from);

        self.reprint_dont_move(from);
        self.move_to_pos(to);
    }

    fn reprint_with_prompt_dont_move(&mut self) {
        self.pbuf += codes::ERASE_TO_END;
        self.pbuf += self.prompt.as_str();
        self.print_from_dont_move(0);

        self.pos = self.buf.len();
        if self.cur_pos().x == 0 && !self.buf.is_empty() {
            self.pbuf += "\r\n";
        }
    }

    fn reprint_dont_move(&mut self, pos: usize) {
        self.pbuf += codes::ERASE_TO_END;
        self.print_from_dont_move(pos);

        self.pos = self.buf.len();
        if self.cur_pos().x == 0 && !self.buf.is_empty() {
            self.pbuf += "\r\n";
        }
    }

    fn print_from_dont_move(&mut self, pos: usize) {
        self.pbuf
            .extend(self.buf[pos..].iter().copied().map(get_printable));
    }

    fn commit(&mut self) -> Result<()> {
        if !self.pbuf.is_empty() && self.term.is_out_terminal() {
            self.term.write_all(self.pbuf.as_bytes())?;
            self.term.flush()?;
        }
        self.pbuf.clear();
        Ok(())
    }
}

/// Get printable `non-control` character.
pub fn get_printable(c: char) -> char {
    if !c.is_ascii_control() {
        if c.is_control() { '␦' } else { c }
    } else {
        match c as u32 {
            c if c < 32 => char::from_u32(c + 0x2400).unwrap(),
            127 => '␡',
            _ => c,
        }
    }
}
