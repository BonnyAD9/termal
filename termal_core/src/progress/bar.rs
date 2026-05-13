use std::{
    borrow::Cow,
    fmt::Write,
    io::{Write as _, stdout},
    iter,
    time::Duration,
};

use crate::{codes, progress::ProgressFormatter};

/// Customizable progress bar.
#[derive(Debug, Clone)]
pub struct Bar {
    /// The width of the bar.
    pub width: usize,
    /// Sequence starting the bar.
    pub start: Option<Cow<'static, str>>,
    /// Sequence ending the bar.
    pub end: Option<Cow<'static, str>>,
    /// Empty bar positions (from the end)
    pub empty: Cow<'static, str>,
    /// Full bar position (from the start)
    pub full: Cow<'static, str>,
    /// If ture, bar will buffer clearing so that `println` will work. Note
    /// that if the println will overwrite the bar.
    pub log: bool,
    buf: String,
}

impl Bar {
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
        if !task.is_empty() {
            self.buf += task;
            self.buf.push(' ');
        }
        let cnt = if let Some(done) = done {
            _ = write!(self.buf, "{:.2} % ", done * 100.);
            (self.width as f32 * done).round() as usize
        } else {
            self.buf += "?? % ";
            0
        };
        if let Some(s) = &self.start {
            self.buf += s;
        }
        for _ in 0..cnt {
            self.buf += &self.full;
        }
        for _ in 0..self.width - cnt {
            self.buf += &self.empty;
        }
        if let Some(e) = &self.end {
            self.buf += e;
        }
        if let Some(eta) = eta {
            self.buf.push(' ');
            duration_to_string(eta, true, &mut self.buf);
        }
        if !info.is_empty() {
            self.buf.push(' ');
            self.buf += info;
        }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            width: 40,
            start: Some("[".into()),
            end: Some("]".into()),
            empty: " ".into(),
            full: "#".into(),
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
