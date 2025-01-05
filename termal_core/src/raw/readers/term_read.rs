use std::{
    collections::VecDeque,
    io::{stdout, IsTerminal, Write},
    mem,
    ops::RangeBounds,
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

use super::{Predicate, ReadConf, Vec2};

/// Terminal reader. Supports only single line. Newlines are skipped.
pub struct TermRead<'t, 'p, P>
where
    P: Predicate<Event>,
{
    buf: Vec<char>,
    prompt: TermText<'p>,
    pbuf: String,
    pos: usize,
    term: &'t mut Terminal,
    exit: P,
    size: Vec2,
    finished: bool,
    last_event: Option<Event>,
    queue: VecDeque<Event>,
}

impl<'t> TermRead<'t, '_, KeyCode> {
    /// Gets reader that ends on enter.
    pub fn lines(term: &'t mut Terminal) -> Self {
        Self::new(term, KeyCode::Enter)
    }
}

impl<'t, 'p, P> TermRead<'t, 'p, P>
where
    P: Predicate<Event>,
{
    /// Creates new terminal reader that exits with the given predicate.
    pub fn new(term: &'t mut Terminal, exit: P) -> Self {
        Self::from_config(term, exit, Default::default())
    }

    /// Create terminal reader from configuration.
    pub fn from_config(
        term: &'t mut Terminal,
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
        // TODO: change when newlines supported
        self.buf.extend(
            TermText::new(s.as_ref())
                .spans()
                .filter(|s| !s.is_control())
                .flat_map(|s| s.text().chars()),
        );
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
    /// [`TermRead::get_readed`], [`TermRead::finish`] or
    /// [`TermRead::finish_to_str`]. This is unsafe because calls to `set_`
    /// functions may break the input if this is called.
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

    pub fn queue(&mut self, evt: impl IntoIterator<Item = Event>) {
        self.queue.extend(evt);
    }

    fn read_one_inner(&mut self) -> Result<bool> {
        if !self.queue.is_empty() || self.term.has_buffered_input() {
            return self.read_next();
        }

        let r = wait_for_stdin(Duration::from_millis(100));
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
            return self.handle_event(evt);
        }

        let evt = match self.term.read() {
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

    fn handle_event(&mut self, evt: Event) -> Result<bool> {
        if self.exit.matches(&evt) {
            self.last_event = Some(evt);
            self.end();
            self.commit()?;
            return Ok(true);
        }

        let Event::KeyPress(key) = evt else {
            self.last_event = Some(evt);
            return Ok(false);
        };

        self.last_event = Some(evt);

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

    fn backspace(&mut self) {
        if self.pos != 0 {
            self.move_left();
            self.delete();
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

        /*let new_line_adj = new.x.saturating_sub(old.x) > 0
            && new.x == 0
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
        }*/
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
        let save = self.pos;
        self.move_to_pos(pos);

        self.reprint_dont_move(pos);
        self.move_to_pos(save);
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
        self.pbuf.extend(self.buf[pos..].iter().flat_map(|c| {
            Some(c).into_iter().chain(if *c == '\n' {
                Some(&'\r')
            } else {
                None
            })
        }));
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
