use std::{
    collections::VecDeque,
    io::{BufRead, Read, Write},
    time::{Duration, Instant},
};

use crate::{Error, Result};

use super::{IoProvider, StdioProvider, WaitForIn};

#[cfg(feature = "events")]
use crate::{
    codes,
    raw::events::{AmbiguousEvent, AnyEvent, Event, StateChange},
};
#[cfg(feature = "readers")]
use crate::{raw::readers::TermRead, term_text::TermText};

/// Terminal reader. Abstracts reading from terminal and parsing inputs.
///
/// Some functionality might work properly only with raw mode enabled.
///
/// It is especially useful for reading terminal events, but may be used for
/// other things such as non blocking reading from stdin.
#[derive(Debug, Default)]
pub struct Terminal<T: IoProvider = StdioProvider> {
    buffer: VecDeque<u8>,
    io: T,
    #[cfg(feature = "events")]
    bracketed_paste_open: bool,
}

impl Terminal<StdioProvider> {
    /// Creates terminal that reads from stdin and writes to stdout.
    ///
    /// # Example
    /// ```no_run
    /// use termal_core::{
    ///     codes, Result,
    ///     raw::{disable_raw_mode, enable_raw_mode, Terminal},
    /// };
    ///
    /// let mut term = Terminal::stdio();
    ///
    /// term.flushed(codes::CLEAR)?;
    /// term.println("This will print to stdout.")?;
    /// enable_raw_mode()?;
    /// let data = term.prompt("Enter data to stdin: ")?;
    /// disable_raw_mode()?;
    /// term.println(format!("\nData you entered to stdin: {data}"))?;
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/stdio.png)
    pub fn stdio() -> Self {
        Self::default()
    }
}

impl<T: IoProvider> Terminal<T> {
    /// Create new terminal with the given input and output streams.
    pub fn new(io: T) -> Self {
        Terminal {
            buffer: VecDeque::new(),
            io,
            #[cfg(feature = "events")]
            bracketed_paste_open: false,
        }
    }

    /// Read next byte from stdin. May block if there is no buffered data.
    ///
    /// This will wait for whole line to be buffered if there is no buffered
    /// input and raw mode is not enabled.
    ///
    /// Note that if raw mode is not enabled, the byte will not be written back
    /// to stdout.
    ///
    /// # Errors
    /// - [`Error::Io`] if there were no buffered data and read failed.
    /// - [`Error::StdInEof`] if EOF was reached and there is no more data.
    ///
    /// # Example
    /// ```no_run
    /// use termal_core::{
    ///     codes, Result,
    ///     raw::{Terminal, enable_raw_mode, disable_raw_mode}
    /// };
    ///
    /// let mut term = Terminal::stdio();
    ///
    /// term.flushed(codes::CLEAR)?;
    /// term.flushed("Enter single byte: ")?;
    /// enable_raw_mode()?;
    ///
    /// // Read the byte
    /// let byte = term.read_byte()?;
    ///
    /// disable_raw_mode()?;
    ///
    /// term.println(format!("\nYou entered byte `0x{byte:2x}`."))?;
    /// term.println(format!(
    ///     "It coresponds to the character `{}`.",
    ///     byte as char
    /// ))?;
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/read_byte.png)
    pub fn read_byte(&mut self) -> Result<u8> {
        if let Some(b) = self.buffer.pop_front() {
            return Ok(b);
        }
        self.fill_buffer()?;
        self.buffer.pop_front().ok_or(Error::StdInEof)
    }

    /// Checks whether there is any buffered input in [`Terminal`].
    ///
    /// Doesn't block.
    ///
    /// This doesn't check the buffer from the underlaying input stream
    /// (stdin). If you would also like to check the underlaying stream, use
    /// [`Self::has_input`].
    ///
    /// # Returns
    /// `true` if there is buffered input. Otherwise `false`.
    ///
    /// # Example
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// use termal_core::{
    ///     codes, Result,
    ///     raw::{Terminal, wait_for_stdin},
    /// };
    ///
    /// let mut term = Terminal::stdio();
    /// term.flushed(codes::CLEAR)?;
    ///
    /// println!("Before entering:");
    /// println!("has_input: {}", term.has_input());
    /// println!("has_buffered_input: {}", term.has_buffered_input());
    /// println!(
    ///     "wait_for_stdin(ZERO): {}\n",
    ///     wait_for_stdin(Duration::ZERO)?
    /// );
    ///
    /// term.flushed("Please enter something: ")?;
    /// // Wait for the user to type something.
    /// term.wait_for_input(Duration::MAX)?;
    /// println!();
    ///
    /// // Now there should be input but it is only buffered in the underlaying
    /// // input stream.
    /// println!("After entering, before reading:");
    /// println!("has_input: {}", term.has_input());
    /// println!("has_buffered_input: {}", term.has_buffered_input());
    /// println!("wait_for_stdin(ZERO): {}\n", wait_for_stdin(Duration::ZERO)?);
    ///
    /// term.read_byte()?;
    ///
    /// // Now there is also buffered input in the terminal if there was more
    /// // than one byte.
    /// println!("After reading single byte:");
    /// println!("has_input: {}", term.has_input());
    /// println!("has_buffered_input: {}", term.has_buffered_input());
    /// println!(
    ///     "wait_for_stdin(ZERO): {}\n",
    ///     wait_for_stdin(Duration::ZERO)?
    /// );
    ///
    /// // Wait for next input on stdin (not counting what is buffered in
    /// // term).
    /// term.flushed("Enter something more: ")?;
    /// wait_for_stdin(Duration::MAX)?;
    /// println!();
    ///
    /// println!("After next input before consuming previous:");
    /// println!("has_input: {}", term.has_input());
    /// println!("has_buffered_input: {}", term.has_buffered_input());
    /// println!("wait_for_stdin(ZERO): {}", wait_for_stdin(Duration::ZERO)?);
    ///
    /// term.consume_available()?; // Consume all the input
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/has_input.png)
    pub fn has_buffered_input(&self) -> bool {
        !self.buffer.is_empty()
    }

    /// Checks whether the next input is available immidietely.
    ///
    /// Doesn't block.
    ///
    /// Unlike [`Self::has_buffered_input`] this also checks the buffer of the
    /// underlaying stream.
    ///
    /// # Returns
    /// `true` if there is available input. If there is no input or it is
    /// unknown, returns `false`.
    ///
    /// # Example
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// use termal_core::{
    ///     codes, Result,
    ///     raw::{Terminal, wait_for_stdin},
    /// };
    ///
    /// let mut term = Terminal::stdio();
    /// term.flushed(codes::CLEAR)?;
    ///
    /// println!("Before entering:");
    /// println!("has_input: {}", term.has_input());
    /// println!("has_buffered_input: {}", term.has_buffered_input());
    /// println!(
    ///     "wait_for_stdin(ZERO): {}\n",
    ///     wait_for_stdin(Duration::ZERO)?
    /// );
    ///
    /// term.flushed("Please enter something: ")?;
    /// // Wait for the user to type something.
    /// term.wait_for_input(Duration::MAX)?;
    /// println!();
    ///
    /// // Now there should be input but it is only buffered in the underlaying
    /// // input stream.
    /// println!("After entering, before reading:");
    /// println!("has_input: {}", term.has_input());
    /// println!("has_buffered_input: {}", term.has_buffered_input());
    /// println!("wait_for_stdin(ZERO): {}\n", wait_for_stdin(Duration::ZERO)?);
    ///
    /// term.read_byte()?;
    ///
    /// // Now there is also buffered input in the terminal if there was more
    /// // than one byte.
    /// println!("After reading single byte:");
    /// println!("has_input: {}", term.has_input());
    /// println!("has_buffered_input: {}", term.has_buffered_input());
    /// println!(
    ///     "wait_for_stdin(ZERO): {}\n",
    ///     wait_for_stdin(Duration::ZERO)?
    /// );
    ///
    /// // Wait for next input on stdin (not counting what is buffered in
    /// // term).
    /// term.flushed("Enter something more: ")?;
    /// wait_for_stdin(Duration::MAX)?;
    /// println!();
    ///
    /// println!("After next input before consuming previous:");
    /// println!("has_input: {}", term.has_input());
    /// println!("has_buffered_input: {}", term.has_buffered_input());
    /// println!("wait_for_stdin(ZERO): {}", wait_for_stdin(Duration::ZERO)?);
    ///
    /// term.consume_available()?; // Consume all the input
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/has_input.png)
    pub fn has_input(&self) -> bool {
        self.has_buffered_input()
            || self.io.wait_for_in(Duration::ZERO).unwrap_or_default()
    }

    /// Wait for input on the terminal. Block for at most the given duration.
    ///
    /// Using this with [`Duration::ZERO`] will just check if there is input
    /// available without blocking.
    ///
    /// Using this with [`Duration::MAX`] will wait indefinitely.
    ///
    /// If terminal is in raw mode, this will wait for any available input. If
    /// not and stdin is tty, this will wait for whole line because in this
    /// case stdin is line buffered.
    ///
    /// # Returns
    /// `true` input arrived whithn the given duration.
    ///
    /// # Errors
    /// - If unerlaying buffer failed to wait for input. For
    ///   [`Terminal::stdio`] this is:
    ///     - On unix (linux):
    ///         - [`Error::IntConvert`] if `timeout` is larger than
    ///           [`crate::raw::MAX_STDIN_WAIT`].
    ///         - [`Error::Io`] on io error.
    ///         - [`Error::WaitAbandoned`] if the wait otherwise fails.
    ///     - On windows:
    ///         - [`Error::IntConvert`] if `timeout` is larger than
    ///           [`crate::raw::MAX_STDIN_WAIT`].
    ///         - [`Error::Io`] on io error.
    ///         - [`Error::WaitAbandoned`] if the wait otherwise fails.
    ///     - [`Error::NotSupportedOnPlatform`] on other platforms.
    ///
    /// # Example
    /// ```no_run
    /// use std::{io::stdin, time::Duration};
    ///
    /// use termal_core::{
    ///     codes, Result,
    ///     raw::{Terminal, raw_guard},
    /// };
    ///
    /// let mut term = Terminal::stdio();
    /// term.flushed(codes::CLEAR)?;
    ///
    /// term.flushed("You have one second to enter \"wait for input\"\n> ")?;
    /// if term.wait_for_input(Duration::from_secs(1))? {
    ///     let mut data = String::new();
    ///     // Using the standart blocking read_line without raw mode. It won't
    ///     // block because there is input ready and stdin is line buffered.
    ///     stdin().read_line(&mut data)?;
    ///     if data != "wait for input\n" {
    ///         println!("You misspelled it!");
    ///     } else {
    ///         println!("Good work!");
    ///     }
    /// } else {
    ///     println!("\nOoops! Too late!");
    /// }
    ///
    /// // Consume the data that has already been typed but not consumed because of
    /// // line buffering.
    /// raw_guard(true, || term.consume_available())?;
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/wait_for_input.png)
    pub fn wait_for_input(&self, timeout: Duration) -> Result<bool> {
        if self.has_buffered_input() {
            Ok(true)
        } else {
            self.io.wait_for_in(timeout)
        }
    }

    /// Read raw bytes from the terminal to `res`. Returns the number of readed
    /// bytes. Returns [`Error::StdInEof`] when reaches eof. May block.
    ///
    /// If encounters EOF it returns successfully if there are any read bytes,
    /// but with shorter length. If the eof is encountered before any bytes
    /// were read, it returns the [`Error::StdInEof`]. Note that it is possible
    /// that there will be more data after eof.
    ///
    /// # Errors
    /// - [`Error::StdInEof`] when eof is encountered before any bytes are
    ///   read.
    /// - [`Error::Io`] when error occurs while reading from stdin.
    ///
    /// # Example
    /// ```no_run
    /// use termal_core::{raw::Terminal, codes, Result};
    ///
    /// const TARGET: usize = 404;
    /// const COUNT: usize = 5;
    ///
    /// let mut term = Terminal::stdio();
    /// term.flushed(codes::CLEAR)?;
    ///
    /// term.flushed(format!("Enter byte sum of {TARGET} with {COUNT} bytes: "))?;
    /// let mut buf = [0;COUNT];
    /// let len = term.read_raw(&mut buf)?;
    ///
    /// if len != COUNT {
    ///     // This can happen when eof is reached.
    ///     println!("\nYou cheater, that wasn't 5 bytes!!");
    /// }
    ///
    /// let sum = buf[..len].iter().fold(0_usize, |s, i| s + *i as usize);
    /// if sum == TARGET {
    ///     println!("Well done!");
    /// } else {
    ///     println!("Not quite, you entered {sum}.");
    /// }
    ///
    /// term.consume_available()?;
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/read_raw.png)
    pub fn read_raw(&mut self, res: &mut [u8]) -> Result<usize> {
        let mut read = self.read_buffered(res);
        let mut stdin = self.io.get_in();
        while read < res.len() {
            match read_stdin_once(&mut *stdin, &mut res[read..]) {
                Ok(v) => read += v,
                Err(Error::StdInEof) => break,
                e => return e,
            }
        }
        if read == 0 {
            Err(Error::StdInEof)
        } else {
            Ok(read)
        }
    }

    /// Read raw bytes from the terminal to `res`. Returns the number of readed
    /// bytes. Returns [`Error::StdInEof`] when reaches eof. Block for at most
    /// the given duration for each read from the terminal (so possibly for up
    /// to `res.len() * timeout`). If you want to measure the total timeout
    /// instead, use [`Self::read_raw_single_timeout`].
    ///
    /// # Returns
    /// The number of bytes read. This may return with less bytes when eof was
    /// reached.
    ///
    /// # Errors
    /// - [`Error::StdInEof`] if eof was reached and no data were read.
    /// - [`Error::Io`] on io error.
    pub fn read_raw_timeout(
        &mut self,
        res: &mut [u8],
        timeout: Duration,
    ) -> Result<usize> {
        let mut read = self.read_buffered(res);
        let mut stdin = self.io.get_in();
        while read < res.len()
            && stdin.wait_for_in(timeout).unwrap_or_default()
        {
            match read_stdin_once(&mut *stdin, &mut res[read..]) {
                Ok(v) => read += v,
                Err(Error::StdInEof) => {
                    if read == 0 {
                        return Err(Error::StdInEof);
                    } else {
                        return Ok(read);
                    }
                }
                e => return e,
            }
        }
        Ok(read)
    }

    /// Read raw bytes from the terminal to `res`. Returns the number of readed
    /// bytes. Returns [`Error::StdInEof`] when reaches eof. Block for at most
    /// the given total duration.
    ///
    /// # Returns
    /// The number of bytes read. This may return less than was the input
    /// before the timeout if eof was reached.
    ///
    /// # Errors
    /// - [`Error::StdInEof`] if eof was reached.
    /// - [`Error::Io`] on io error.
    ///
    /// # Example
    /// ```no_run
    /// use std::time::Duration;
    ///
    /// use termal_core::{raw::{Terminal, raw_guard}, codes, Result};
    ///
    /// let mut term = Terminal::stdio();
    /// term.flushed(codes::CLEAR)?;
    ///
    /// term.flushed("You have 1 second to enter `HeLlO`: ")?;
    /// let mut buf = [0; 6];
    /// let len =
    ///     term.read_raw_single_timeout(&mut buf, Duration::from_secs(1))?;
    ///
    /// if len == 0 {
    ///     println!();
    /// }
    ///
    /// if &buf == b"HeLlO\n" {
    ///     println!("Well done!");
    /// } else {
    ///     println!("YOU FAILED");
    /// }
    ///
    /// raw_guard(true, || term.consume_available())?;
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/read_raw_single_timeout.png)
    pub fn read_raw_single_timeout(
        &mut self,
        res: &mut [u8],
        mut timeout: Duration,
    ) -> Result<usize> {
        let mut read = self.read_buffered(res);
        let mut stdin = self.io.get_in();
        while read < res.len() {
            let now = Instant::now();
            let ready = stdin.wait_for_in(timeout);
            timeout = timeout.saturating_sub(Instant::now() - now);
            if !ready.unwrap_or_default() {
                break;
            }
            match read_stdin_once(&mut *stdin, &mut res[read..]) {
                Ok(v) => read += v,
                Err(Error::StdInEof) => {
                    if read == 0 {
                        return Err(Error::StdInEof);
                    } else {
                        return Ok(read);
                    }
                }
                e => return e,
            }
        }
        Ok(read)
    }

    /// Read one byte from stdin. Block for at most the given timeout. If EOF
    /// is reached, returns [`Error::StdInEof`].
    ///
    /// # Returns
    /// - The read byte.
    /// - [`Error::StdInEof`] if EOF is reached.
    /// - [`None`] if no byte is available within the given time.
    ///
    /// # Errors
    /// - [`Error::StdInEof`] on EOF.
    /// - [`Error::Io`] on io error when reading.
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

    /// Checks if the output stream is terminal.
    pub fn is_out_terminal(&self) -> bool {
        self.io.is_out_terminal()
    }

    /// Checks if the input stream is termainal.
    pub fn is_in_terminal(&self) -> bool {
        self.io.is_in_terminal()
    }

    /// Prints to the output. Properly handles newlines if output is raw
    /// terminal.
    ///
    /// The output is not flushed, either flush it with [`Self::flush`], or use
    /// [`Self::flushed`] to print and flush. Note that stdout usually also
    /// flushes on newline.
    ///
    /// # Errors
    /// - [`Error::Io`] on io error when writing to output.
    ///
    /// # Example
    /// ```no_run
    /// use termal_core::{raw::Terminal, codes, Result};
    ///
    /// let mut term = Terminal::stdio();
    /// term.flushed(codes::CLEAR)?;
    ///
    /// term.print("Hello there!\n")?;
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/print.png)
    pub fn print(&mut self, s: impl AsRef<str>) -> Result<()> {
        if !self.io.is_out_raw() || !self.is_out_terminal() {
            self.write_all(s.as_ref().as_bytes())?;
        } else {
            Self::print_escaped(&mut self.io.get_out(), s)?;
        }
        Ok(())
    }

    /// Prints to the output. Properly handles newlines if output is raw
    /// terminal. Appends newline to the output. Doesn't explicitly flush, but
    /// stdout usually flushes on newline.
    ///
    /// # Errors
    /// - [`Error::Io`] on io error when printing to output.
    ///
    /// # Example
    /// ```no_run
    /// use termal_core::{raw::Terminal, codes, Result};
    ///
    /// let mut term = Terminal::stdio();
    /// term.flushed(codes::CLEAR)?;
    ///
    /// term.println("Hello there!")?;
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/print.png)
    pub fn println(&mut self, s: impl AsRef<str>) -> Result<()> {
        if !self.io.is_out_raw() || !self.is_out_terminal() {
            let mut out = self.io.get_out();
            out.write_all(s.as_ref().as_bytes())?;
            out.write_all(b"\n")?;
        } else {
            let mut out = self.io.get_out();
            Self::print_escaped(&mut out, s)?;
            out.write_all(b"\n\r")?;
        }
        Ok(())
    }

    /// Prints to the output and flushes. Properly handles newlines if output
    /// is raw terminal.
    ///
    /// # Errors
    /// - [`Error::Io`] on io error when writing to stdout or flushing.
    ///
    /// # Example
    /// ```no_run
    /// use termal_core::{raw::Terminal, codes, Result};
    ///
    /// let mut term = Terminal::stdio();
    /// term.flushed(codes::CLEAR)?;
    ///
    /// Result::Ok(())
    /// ```
    ///
    /// ## Result in terminal
    /// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/terminal/flushed.png)
    pub fn flushed(&mut self, s: impl AsRef<str>) -> Result<()> {
        if !self.io.is_out_raw() || !self.is_out_terminal() {
            let mut out = self.io.get_out();
            out.write_all(s.as_ref().as_bytes())?;
            out.flush()?;
        } else {
            let mut out = self.io.get_out();
            Self::print_escaped(&mut out, s)?;
            out.flush()?;
        }
        Ok(())
    }

    /// Consumes all available data in the input stream. Doesn't block.
    pub fn consume_available(&mut self) -> Result<()> {
        self.buffer.clear();
        while self.has_input() {
            let mut stdin = self.io.get_in();
            let b = stdin.fill_buf()?.len();
            stdin.consume(b);
        }
        Ok(())
    }

    fn print_escaped(out: &mut T::Out, s: impl AsRef<str>) -> Result<()> {
        let mut spl = s.as_ref().split('\n');
        let Some(n) = spl.next() else {
            return Ok(());
        };
        write!(out, "{n}")?;
        for s in spl {
            write!(out, "{s}\n\r")?;
        }
        Ok(())
    }

    fn read_buffered(&mut self, mut res: &mut [u8]) -> usize {
        let (s1, s2) = self.buffer.as_slices();

        // Read from the first slice.
        if s1.len() >= res.len() {
            res.copy_from_slice(&s1[..res.len()]);
            self.buffer.consume(res.len());
            return res.len();
        }
        res[..s1.len()].copy_from_slice(s1);
        res = &mut res[s1.len()..];

        // Read from the second slice
        if s2.len() >= res.len() {
            res.copy_from_slice(&s2[..res.len()]);
            let read = s1.len() + res.len();
            self.buffer.consume(read);
            return read;
        }
        res[..s2.len()].copy_from_slice(s2);
        let read = s1.len() + s2.len();
        self.buffer.clear();
        read
    }

    fn fill_buffer(&mut self) -> Result<()> {
        let mut stdin = self.io.get_in();
        let buf = stdin.fill_buf()?;
        self.buffer.extend(buf);
        let len = buf.len();
        stdin.consume(len);
        Ok(())
    }
}

#[cfg(feature = "readers")]
impl<T: IoProvider> Terminal<T> {
    /// Appends next line of input from stdin to `s`. May block.
    pub fn read_line_to(&mut self, s: &mut String) -> Result<()> {
        let mut reader = TermRead::lines(self);
        reader.read_to_str(s)?;
        Ok(())
    }

    /// Read the next line from stdin. May block.
    pub fn read_line(&mut self) -> Result<String> {
        let mut s = String::new();
        self.read_line_to(&mut s)?;
        Ok(s)
    }

    /// Edit the given string. Newlines are ignored.
    pub fn edit_line_in(&mut self, s: &mut String) -> Result<()> {
        let mut reader = TermRead::lines(self);
        reader.edit_str(s, None)
    }

    /// Edit the given string. Newlines are ignored.
    pub fn edit_line(&mut self, s: impl AsRef<str>) -> Result<String> {
        let mut reader = TermRead::lines(self);
        reader.edit(s, None)
    }

    /// Prompt the user with the given prompt and return the entered result.
    pub fn prompt<'a>(
        &mut self,
        s: impl Into<TermText<'a>>,
    ) -> Result<String> {
        let mut reader = TermRead::lines(self);
        reader.set_prompt(s);
        reader.read_str()
    }

    /// Prompt the user with the given prompt and append the entered result to
    /// the given string.
    pub fn prompt_to<'a>(
        &mut self,
        s: &mut String,
        prompt: impl Into<TermText<'a>>,
    ) -> Result<()> {
        let mut reader = TermRead::lines(self);
        reader.set_prompt(prompt);
        reader.read_to_str(s)
    }
}

#[cfg(feature = "events")]
impl<T: IoProvider> Terminal<T> {
    /// Read the next known event on stdin. May block.
    pub fn read(&mut self) -> Result<Event> {
        loop {
            if let AnyEvent::Known(ev) = self.read_ambiguous()?.event {
                return Ok(ev);
            }
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

    /// Read the next event on stdin. May block.
    pub fn read_ambiguous(&mut self) -> Result<AmbiguousEvent> {
        if self.bracketed_paste_open {
            self.read_bracketed()
        } else if self.cur()? == 0x1b && self.buffer.len() != 1 {
            self.read_escape()
        } else {
            // TODO should \r\n be single event?
            self.read_char()
        }
    }

    /// Read the next event on terminal. Block for at most the given duration.
    pub fn read_ambiguous_timeout(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<AmbiguousEvent>> {
        if self.wait_for_input(timeout)? {
            Ok(Some(self.read_ambiguous()?))
        } else {
            Ok(None)
        }
    }

    /// Opens bracketed paste mode. It will start automatically with
    /// start of paste text and end with end of paste text if bracketed paste
    /// mode is enabled (with [`codes::ENABLE_BRACKETED_PASTE_MODE`]).
    ///
    /// When bracketed paste is opened, it will read all input verbatim as text
    /// instead of control sequences.
    pub fn open_bracketed_paste(&mut self, v: bool) {
        self.bracketed_paste_open = v;
    }

    /// Checks if bracketed paste is open. It will start automatically with
    /// start of paste text and end with end of paste text if bracketed paste
    /// mode is enabled (with [`codes::ENABLE_BRACKETED_PASTE_MODE`]).
    ///
    /// When bracketed paste is opened, it will read all input verbatim as text
    /// instead of control sequences.
    pub fn is_bracketed_paste_open(&mut self) -> bool {
        self.bracketed_paste_open
    }

    fn read_escape(&mut self) -> Result<AmbiguousEvent> {
        self.read_byte()?;
        let cur = self.cur()?;
        match cur {
            b'[' => self.read_csi(),
            b'O' if self.buffer.len() > 1 => self.read_ss3(),
            b'P' => self.read_dcs(),
            b']' => self.read_osc(),
            _ => self.read_alt(),
        }
    }

    fn read_csi(&mut self) -> Result<AmbiguousEvent> {
        let mut code: Vec<_> = b"\x1b[".into();
        self.read_byte()?;
        if self.buffer.is_empty() {
            return Ok(AmbiguousEvent::from_code(&code));
        }
        let mut cur = self.read_byte()?;

        if cur == b'M' {
            // Special mouse event that actually doesn't conform to CSI
            // sequence rules.
            code.push(cur);
            for _ in 0..3 {
                if self.buffer.is_empty() {
                    return Ok(AmbiguousEvent::from_code(&code));
                }
                let Some(b) = self.read_byte_if(|b| b >= 32)? else {
                    return Ok(AmbiguousEvent::from_code(&code));
                };
                code.push(b);
            }
            if self.buffer.is_empty() {
                return Ok(AmbiguousEvent::from_code(&code));
            }
            // UTF-8 extension
            for i in (1..=3).rev() {
                if !self.buffer.is_empty()
                    && utf8_code_len(code[code.len() - i]) != 2
                {
                    let Some(b) = self.read_byte_if(|b| b >= 32)? else {
                        return Ok(AmbiguousEvent::from_code(&code));
                    };
                    code.push(b);
                }
            }
            return Ok(AmbiguousEvent::from_code(&code));
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
        if code == codes::BRACKETED_PASTE_START.as_bytes() {
            self.bracketed_paste_open = true;
            Ok(AmbiguousEvent::state_change(
                StateChange::BracketedPasteStart,
            ))
        } else {
            Ok(AmbiguousEvent::from_code(&code))
        }
    }

    fn read_ss3(&mut self) -> Result<AmbiguousEvent> {
        let mut code: Vec<_> = b"\x1bO".into();
        self.read_byte()?;
        if self.buffer.is_empty() {
            return Ok(AmbiguousEvent::from_code(&code));
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
        Ok(AmbiguousEvent::from_code(&code))
    }

    fn read_alt(&mut self) -> Result<AmbiguousEvent> {
        let mut buf: [u8; 5] = [0x1b, 0, 0, 0, 0];
        let chr = self.read_utf8((&mut buf[1..]).try_into().unwrap())?;
        Ok(AmbiguousEvent::from_code(&buf[..=chr.len_utf8()]))
    }

    fn read_dcs(&mut self) -> Result<AmbiguousEvent> {
        self.read_byte()?;
        let mut code: Vec<_> = b"\x1bP".into();
        while !self.buffer.is_empty() && !code.ends_with(codes::ST.as_bytes())
        {
            code.push(self.read_byte()?);
        }
        Ok(AmbiguousEvent::from_code(&code))
    }

    fn read_osc(&mut self) -> Result<AmbiguousEvent> {
        self.read_byte()?;
        let mut code: Vec<_> = b"\x1b]".into();
        // TODO: don't hang if no further data.
        let r = self.read_until_st(&mut code);
        if matches!(r, Err(Error::StdInEof)) {
            Ok(AmbiguousEvent::from_code(&code))
        } else {
            r.map(|_| AmbiguousEvent::from_code(&code))
        }
    }

    fn read_until_st(&mut self, res: &mut Vec<u8>) -> Result<()> {
        while !res.ends_with(codes::ST.as_bytes()) && !res.ends_with(b"\x07") {
            res.push(self.read_byte()?);
        }
        Ok(())
    }

    fn read_char(&mut self) -> Result<AmbiguousEvent> {
        if !self.cur()?.is_ascii() {
            let mut buf: [u8; 4] = [0; 4];
            Ok(AmbiguousEvent::from_char_code(self.read_utf8(&mut buf)?))
        } else {
            let chr = self.read_byte()? as char;
            Ok(AmbiguousEvent::from_char_code(chr))
        }
    }

    fn read_bracketed(&mut self) -> Result<AmbiguousEvent> {
        let c = self.cur()?;
        if self.buffer_starts_with(codes::BRACKETED_PASTE_END.as_bytes()) {
            self.buffer.consume(codes::BRACKETED_PASTE_END.len());
            self.bracketed_paste_open = false;
            Ok(AmbiguousEvent::state_change(StateChange::BracketedPasteEnd))
        } else if c.is_ascii() {
            self.buffer.consume(1);
            if c == 0xD {
                Ok(AmbiguousEvent::verbatim('\n'))
            } else {
                Ok(AmbiguousEvent::verbatim(c as char))
            }
        } else {
            let mut buf: [u8; 4] = [0; 4];
            Ok(AmbiguousEvent::verbatim(self.read_utf8(&mut buf)?))
        }
    }

    fn buffer_starts_with(&self, b: &[u8]) -> bool {
        if self.buffer.len() < b.len() {
            return false;
        }

        let s = self.buffer.as_slices();
        if s.0.len() >= b.len() {
            s.0.starts_with(b)
        } else {
            b.starts_with(s.0) && s.1.starts_with(&b[s.0.len()..])
        }
    }

    fn cur(&mut self) -> Result<u8> {
        if let Some(b) = self.buffer.front() {
            Ok(*b)
        } else {
            self.fill_buffer()?;
            self.buffer.front().ok_or(Error::StdInEof).copied()
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

impl<T: IoProvider> Read for Terminal<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.io.get_in().read(buf)
    }
}

impl<T: IoProvider> Write for Terminal<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.io.get_out().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.io.get_out().flush()
    }
}

fn read_stdin_once(stdin: &mut impl BufRead, res: &mut [u8]) -> Result<usize> {
    let buf = stdin.fill_buf()?;
    if buf.is_empty() {
        return Err(Error::StdInEof);
    }
    let len = buf.len().min(res.len());
    res[..len].copy_from_slice(&buf[..len]);
    stdin.consume(len);
    Ok(len)
}

#[cfg(feature = "events")]
fn utf8_code_len(first: u8) -> usize {
    if (first & 0x80) == 0 {
        1
    } else if (first & 0xE0) == 0xC0 {
        2
    } else if (first & 0xF0) == 0xE0 {
        3
    } else if (first & 0xF8) == 0xF0 {
        4
    } else {
        0
    }
}
