mod bar;
mod bar_theme;
mod dots;
mod iter;
mod no_state;
mod progress_ext;
mod progress_formatter;
mod state_progress;
mod update_policy;

use std::{
    borrow::Cow,
    fmt::{Debug, Display, Write},
    mem,
    time::{Duration, Instant},
};

pub use self::{
    bar::*, bar_theme::*, dots::*, iter::*, no_state::*, progress_ext::*,
    progress_formatter::*, state_progress::*, update_policy::*,
};

/// Track progress with progress bar.
pub fn track_bar<T>(task: &str, f: impl FnOnce(&mut ProgressBar) -> T) -> T {
    let mut pb = ProgressBar::bar(task);
    pb.track_this(f)
}

/// Track fallible progress with progress bar.
pub fn try_track_bar<T, E: Display>(
    task: &str,
    f: impl FnOnce(&mut ProgressBar) -> Result<T, E>,
) -> Result<T, E> {
    let mut pb = ProgressBar::bar(task);
    pb.try_track_this(f)
}

/// Track progress with progress dots.
pub fn track_dots<T>(task: &str, f: impl FnOnce(&mut ProgressDots) -> T) -> T {
    let mut pb = ProgressDots::dots(task);
    pb.track_this(f)
}

/// Track fallible progress with progress dots.
pub fn try_track_dots<T, E: Display>(
    task: &str,
    f: impl FnOnce(&mut ProgressDots) -> Result<T, E>,
) -> Result<T, E> {
    let mut pb = ProgressDots::dots(task);
    pb.try_track_this(f)
}

/// A progress bar progress.
type ProgressBar<'a, T = DefaultBarTheme, S = NoState> =
    Progress<'a, Bar<T>, S>;

/// Progress with three dots.
type ProgressDots<'a, S = NoState> = Progress<'a, Dots, S>;

/// The core type for tracking progress.
#[derive(Debug, Clone)]
pub struct Progress<'a, F, S = NoState> {
    formatter: F,
    task: Cow<'a, str>,
    pos: Option<f32>,
    state_str: String,
    start_time: Instant,
    update: Update,
    state: S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Update {
    Iterations { every: usize, remain: usize },
    Time { every: Duration, last: Instant },
}

impl<'a, T: BarTheme + Default, S: Display + Default> ProgressBar<'a, T, S> {
    pub fn bar(task: impl Into<Cow<'a, str>>) -> Self {
        Self::new(Bar::default(), task, S::default())
    }
}

impl<'a, S: Display + Default> ProgressDots<'a, S> {
    pub fn dots(task: impl Into<Cow<'a, str>>) -> Self {
        Self::new(Dots::default(), task, S::default())
    }
}

impl<'a, F: ProgressFormatter, S: Display> Progress<'a, F, S> {
    /// Create new progress tracker.
    ///
    /// - `formatter` decides how progress is displayed.
    /// - `task` name of the task.
    /// - `state` additional state information.
    pub fn new(formatter: F, task: impl Into<Cow<'a, str>>, state: S) -> Self {
        let now = Instant::now();
        Self {
            formatter,
            pos: Some(0.),
            task: task.into(),
            state_str: String::new(),
            start_time: now,
            update: Update::new(UpdatePolicy::default(), now),
            state,
        }
    }

    /// Mutate the state.
    ///
    /// State contains additional information about the progress.
    pub fn state_mut(&mut self) -> &mut S {
        self.state_str.clear();
        &mut self.state
    }

    /// Set the state.
    ///
    /// State contains additional information about the progress.
    pub fn set_state(&mut self, state: S) -> S {
        self.state_str.clear();
        mem::replace(&mut self.state, state)
    }

    /// Set the update policy.
    ///
    /// The update policy decides how often should the progress update. The
    /// default updates every 100 ms.
    pub fn set_update_policy(&mut self, policy: impl Into<UpdatePolicy>) {
        self.update = Update::new(policy.into(), self.start_time);
    }

    /// Set this progress to track the new task.
    pub fn track<T>(
        &mut self,
        task: impl Into<Cow<'a, str>>,
        f: impl FnOnce(&mut Progress<F, S>) -> T,
    ) -> T {
        self.reuse(task);
        self.track_this(f)
    }

    /// Set this progress to try track the new task.
    pub fn try_track<T, E: Display>(
        &mut self,
        task: impl Into<Cow<'a, str>>,
        f: impl FnOnce(&mut Progress<F, S>) -> Result<T, E>,
    ) -> Result<T, E> {
        self.reuse(task);
        self.try_track_this(f)
    }

    /// Track this progress.
    pub fn track_this<T>(
        &mut self,
        f: impl FnOnce(&mut Progress<F, S>) -> T,
    ) -> T {
        self.start();
        let res = f(self);
        self.finish();
        res
    }

    /// Try to track this process.
    pub fn try_track_this<T, E: Display>(
        &mut self,
        f: impl FnOnce(&mut Progress<F, S>) -> Result<T, E>,
    ) -> Result<T, E> {
        self.start();
        match f(self) {
            res @ Ok(_) => {
                self.finish();
                res
            }
            Err(e) => {
                self.fail(&format!("{e}"));
                Err(e)
            }
        }
    }

    /// Reuse this progress for another task.
    pub fn reuse(&mut self, task: impl Into<Cow<'a, str>>) {
        self.task = task.into();
        self.pos = Some(0.);
        let now = Instant::now();
        self.start_time = now;
        self.update.reset(now);
    }

    /// Start the task. This is not required to be called.
    pub fn start(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.update.reset(now);

        self.update_state_str();
        self.formatter.start(&self.task, &self.state_str);
    }

    /// Update the progress.
    ///
    /// This respects the update policy so it may skip the update.
    pub fn update(&mut self, progress: impl FnOnce() -> f32) {
        let Some(now) = self.update.should() else {
            return;
        };
        self.update_inner(now, progress());
    }

    /// Update the progress with the given ratio.
    ///
    /// This respects the update policy so it may skip the update.
    pub fn update_of(&mut self, cur: usize, of: usize) {
        self.update(|| cur as f32 / of as f32)
    }

    /// Force the progress update.
    ///
    /// This doesn't respect the update policy and so it will always update.
    pub fn force_update(&mut self, progress: f32) {
        let now = Instant::now();
        self.update.reset(now);
        self.update_inner(now, progress);
    }

    /// Update the state and progress according to the state.
    ///
    /// This respects the update policy so it may skip the update.
    pub fn state_update(&mut self, state: S)
    where
        S: StateProgress,
    {
        self.set_state(state);
        let Some(now) = self.update.should() else {
            return;
        };
        self.update_inner(now, self.state.progress());
    }

    /// Update the state and progress according to the state.
    ///
    /// This respects the update policy so it may skip the update.
    pub fn state_update_mut(&mut self, f: impl FnOnce(&mut S))
    where
        S: StateProgress,
    {
        f(self.state_mut());
        let Some(now) = self.update.should() else {
            return;
        };
        self.update_inner(now, self.state.progress());
    }

    /// Update the state and progress according to the state.
    ///
    /// This doesn't respect the update policy and so it will always update.
    pub fn force_state_update(&mut self, state: S)
    where
        S: StateProgress,
    {
        self.set_state(state);
        let now = Instant::now();
        self.update.reset(now);
        self.update_inner(now, self.state.progress());
    }

    /// Update the state and progress according to the state.
    ///
    /// This doesn't respect the update policy and so it will always update.
    pub fn force_state_update_mut(&mut self, f: impl FnOnce(&mut S))
    where
        S: StateProgress,
    {
        f(self.state_mut());
        let now = Instant::now();
        self.update.reset(now);
        self.update_inner(now, self.state.progress());
    }

    fn update_inner(&mut self, now: Instant, p: f32) {
        self.pos = Some(p);
        let elapsed = (now - self.start_time).as_secs_f32();
        let ets = (1. - p) * (elapsed / p);
        let eta = if ets > u64::MAX as f32 {
            Duration::ZERO
        } else {
            Duration::from_secs_f32(ets)
        };
        self.update_state_str();
        self.formatter
            .update(Some(p), &self.task, &self.state_str, eta);
    }

    /// Update with unknown progress.
    pub fn update_unknown(&mut self) {
        self.pos = None;
        let Some(now) = self.update.should() else {
            return;
        };
        let elapsed = now - self.start_time;
        self.update_state_str();
        self.formatter
            .update(None, &self.task, &self.state_str, elapsed);
    }

    /// Update with unknown progress.
    pub fn force_update_unknown(&mut self) {
        let now = Instant::now();
        self.update.reset(now);
        let elapsed = now - self.start_time;
        self.update_state_str();
        self.formatter
            .update(None, &self.task, &self.state_str, elapsed);
    }

    /// Finish the progress.
    pub fn finish(&mut self) {
        self.pos = None;
        let now = Instant::now();
        self.update.reset(now);
        let elapsed = now - self.start_time;
        self.update_state_str();
        self.formatter.finish(&self.task, &self.state_str, elapsed);
    }

    /// Finish the progress with the given error.
    pub fn fail(&mut self, err: &str) {
        let now = Instant::now();
        self.update.reset(now);
        let elapsed = now - self.start_time;
        self.update_state_str();
        self.formatter.fail(
            self.pos,
            &self.task,
            &self.state_str,
            elapsed,
            err,
        );
    }

    fn update_state_str(&mut self) {
        if self.state_str.is_empty() {
            // Writing to string is infallible.
            _ = write!(&mut self.state_str, "{}", self.state);
        }
    }
}

impl<'a, F, S> AsMut<Progress<'a, F, S>> for Progress<'a, F, S> {
    fn as_mut(&mut self) -> &mut Progress<'a, F, S> {
        self
    }
}

impl Update {
    pub fn new(policy: UpdatePolicy, now: Instant) -> Self {
        match policy {
            UpdatePolicy::Iterations(every) => Self::Iterations {
                every,
                remain: every,
            },
            UpdatePolicy::Time(every) => Self::Time { every, last: now },
        }
    }

    pub fn reset(&mut self, now: Instant) {
        match self {
            Update::Iterations { every, remain } => *remain = *every,
            Update::Time { last, .. } => *last = now,
        }
    }

    pub fn should(&mut self) -> Option<Instant> {
        match self {
            Update::Iterations { every, remain } => {
                if *remain == 0 {
                    *remain = *every;
                    Some(Instant::now())
                } else {
                    None
                }
            }
            Update::Time { every, last } => {
                let now = Instant::now();
                if now - *last >= *every {
                    *last = now;
                    Some(now)
                } else {
                    None
                }
            }
        }
    }
}

pub(crate) fn duration_to_string(
    dur: Duration,
    trunc: bool,
    res: &mut String,
) {
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
            res.extend(std::iter::repeat_n('0', 9 - s.len()));
            *res += s.trim_end_matches('0');
        }
    }
}
