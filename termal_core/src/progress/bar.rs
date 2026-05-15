use std::{
    fmt::Write,
    io::{Write as _, stdout},
    iter,
    time::Duration,
};

use crate::{
    codes,
    progress::{BarTheme, DefaultBarTheme, ProgressFormatter},
};

/// Customizable progress bar.
#[derive(Debug, Clone)]
pub struct Bar<T: BarTheme = DefaultBarTheme> {
    /// The width of the bar.
    pub width: usize,
    /// Theme of the progress bar.
    pub theme: T,
    /// If ture, bar will buffer clearing so that `println` will work. Note
    /// that the println will overwrite the bar.
    pub log: bool,
    buf: String,
}

impl<T: BarTheme> Bar<T> {
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
        let reset = self.theme.reset();
        self.buf += "  ";
        if !task.is_empty() {
            self.buf += self.theme.task();
            self.buf += task;
            self.buf += reset;
            self.buf.push(' ');
        }

        if let Some(done) = done {
            _ = write!(
                self.buf,
                "{}{:.2} %{reset} ",
                self.theme.percent(),
                done * 100.
            );
        } else {
            self.buf += self.theme.percent();
            self.buf += "?? %";
            self.buf += reset;
            self.buf.push(' ');
        }

        self.buf += self.theme.start();
        let empty = self.theme.empty();
        if let Some(done) = done {
            let full = self.theme.full();
            let cnt = (self.width as f32 * done).round() as usize;
            for _ in 0..cnt {
                self.buf += full;
            }
            self.buf += self.theme.thumb();
            for _ in 0..self.width - cnt {
                self.buf += empty;
            }
        } else {
            let mut cnt = eta.unwrap_or_default().as_millis() as usize / 100
                % (self.width * 2);
            if cnt > self.width {
                cnt = self.width * 2 - cnt;
            }
            for _ in 0..cnt {
                self.buf += empty;
            }
            self.buf += self.theme.thumb();
            for _ in 0..self.width - cnt {
                self.buf += empty;
            }
        }

        self.buf += self.theme.end();
        if let Some(eta) = eta {
            self.buf.push(' ');
            self.buf += self.theme.time();
            duration_to_string(eta, true, &mut self.buf);
            self.buf += reset;
        }

        if !info.is_empty() {
            self.buf.push(' ');
            self.buf += self.theme.info();
            self.buf += info;
            self.buf += reset;
        }
    }
}

impl<T: BarTheme + Default> Default for Bar<T> {
    fn default() -> Self {
        Self {
            width: 40,
            theme: T::default(),
            log: true,
            buf: Default::default(),
        }
    }
}

impl ProgressFormatter for Bar {
    fn start(&mut self, task: &str, info: &str) {
        self.show_progress(None, task, info, None);
    }

    fn update(
        &mut self,
        done: Option<f32>,
        task: &str,
        info: &str,
        eta: Duration,
    ) {
        self.show_progress(done, task, info, Some(eta));
    }

    fn finish(&mut self, task: &str, info: &str, time: Duration) {
        self.buf.clear();
        self.buf += codes::ERASE_TO_END;
        self.format_progress(Some(1.), task, info, Some(time));
        println!("{}", self.buf);
    }
}

pub fn duration_to_string(dur: Duration, trunc: bool, res: &mut String) {
    // Number of seconds in the time frame
    const MIN: u64 = 60;
    const HOUR: u64 = 60 * MIN;
    const DAY: u64 = 24 * HOUR;

    let mut secs = dur.as_secs();

    let d = secs / DAY;
    secs %= DAY;
    let h = secs / HOUR;
    secs %= HOUR;
    let m = secs / MIN;
    secs %= MIN;

    if d != 0 {
        _ = write!(res, "{d}d");
    }
    if h != 0 {
        _ = write!(res, "{h:02}:");
    }
    if trunc {
        _ = write!(res, "{m:02}:{secs:02}");
    } else {
        _ = write!(res, "{m:02}:{secs:02}");
        if dur.subsec_nanos() != 0 {
            let s = dur.subsec_nanos().to_string();
            res.push('.');
            res.extend(iter::repeat_n('0', 9 - s.len()));
            *res += s.trim_end_matches('0');
        }
    }
}
