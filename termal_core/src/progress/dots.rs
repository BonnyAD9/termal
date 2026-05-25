use std::{
    fmt::{Display, Write as _},
    io::{Write as _, stdout},
    time::Duration,
};

use crate::{
    codes,
    progress::{ProgressFormatter, duration_to_string},
};

/// A progress tracker that uses triple dots.
pub struct Dots {
    pub log: bool,
    buf: String,
}

/// A simple tracker that tracks only completion.
pub fn dots<T>(task: &str, f: impl FnOnce() -> T) -> T {
    let mut d = Dots::default();
    d.start(task, "");
    let res = f();
    d.finish(task, "", Duration::default());
    res
}

/// A simple tracker that tracks only completion and failure.
pub fn dots_try<T, E: Display>(
    task: &str,
    f: impl FnOnce() -> Result<T, E>,
) -> Result<T, E> {
    let mut d = Dots::default();
    d.start(task, "");
    match f() {
        res @ Ok(_) => {
            d.finish(task, "", Duration::default());
            res
        }
        Err(e) => {
            d.fail(None, task, "", Duration::default(), &format!("{e}"));
            Err(e)
        }
    }
}

impl Dots {
    fn show_progress(
        &mut self,
        done: Option<f32>,
        task: &str,
        info: &str,
        eta: Option<Duration>,
    ) {
        self.buf.clear();
        self.buf += codes::ERASE_TO_END;
        self.buf += codes::CUR_SAVE;
        self.format_progress(done, task, info, eta);
        self.buf += codes::CUR_LOAD;
        print!("{}", self.buf);
        _ = stdout().flush();
        if self.log {
            print!("{}", codes::ERASE_TO_END);
        }
    }

    fn format_progress(
        &mut self,
        done: Option<f32>,
        task: &str,
        info: &str,
        eta: Option<Duration>,
    ) {
        self.buf += task;
        self.buf += "...";

        if let Some(done) = done {
            _ = write!(self.buf, " \x1b[96m{:.2} %\x1b[0m", done * 100.);
        }

        if let Some(time) = eta {
            self.buf += " [\x1b[95m";
            duration_to_string(time, true, &mut self.buf);
            self.buf += "\x1b[0m]"
        }

        if !info.is_empty() {
            self.buf.push(' ');
            self.buf += info;
        }
    }
}

impl ProgressFormatter for Dots {
    fn start(&mut self, task: &str, info: &str) {
        self.show_progress(None, task, info, None);
    }

    fn update(
        &mut self,
        done: Option<f32>,
        task: &str,
        info: &str,
        time: Duration,
    ) {
        self.show_progress(done, task, info, Some(time));
    }

    fn finish(&mut self, task: &str, _: &str, _: Duration) {
        self.buf.clear();
        self.buf += codes::ERASE_TO_END;
        self.format_progress(None, task, "\x1b[92mDone!\x1b[0m", None);
        println!("{}", self.buf);
    }

    fn fail(
        &mut self,
        _: Option<f32>,
        task: &str,
        _: &str,
        _: Duration,
        err: &str,
    ) {
        self.buf.clear();
        self.buf += codes::ERASE_TO_END;
        self.format_progress(None, task, "\x1b[91mFailed!\x1b[0m", None);
        println!("{}\n\x1b[91merror:\x1b[0m {err}", self.buf);
    }
}

impl Default for Dots {
    fn default() -> Self {
        Self {
            log: true,
            buf: String::new(),
        }
    }
}
