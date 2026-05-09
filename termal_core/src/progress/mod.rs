mod progress_formatter;
mod state_progress;
mod update_policy;

use std::{
    borrow::Cow,
    fmt::{Debug, Display, Write},
    mem,
    time::{Duration, Instant},
};

pub use self::{progress_formatter::*, state_progress::*, update_policy::*};

/// The core type for tracking progress.
#[derive(Debug, Clone)]
pub struct Progress<F, S = ()> {
    formatter: F,
    task: Cow<'static, str>,
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

impl<F: ProgressFormatter, S: Display> Progress<F, S> {
    /// Create new progress tracker.
    ///
    /// - `formatter` decides how progress is displayed.
    /// - `task` name of the task.
    /// - `state` additional state information.
    pub fn new(
        formatter: F,
        task: impl Into<Cow<'static, str>>,
        state: S,
    ) -> Self {
        let now = Instant::now();
        Self {
            formatter,
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
    pub fn set_update_policy(&mut self, policy: UpdatePolicy) {
        self.update = Update::new(policy, self.start_time);
    }

    /// Reuse this progress for another task.
    pub fn reuse(&mut self, task: impl Into<Cow<'static, str>>) {
        self.task = task.into();
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
        let elapsed = (now - self.start_time).as_secs_f32();
        let eta = Duration::from_secs_f32((1. - p) * (elapsed / p));
        self.update_state_str();
        self.formatter.update(p, &self.task, &self.state_str, eta);
    }

    /// Finish the progress.
    pub fn finish(&mut self) {
        let now = Instant::now();
        self.update.reset(now);
        let elapsed = now - self.start_time;
        self.update_state_str();
        self.formatter.finish(&self.task, &self.state_str, elapsed);
    }

    fn update_state_str(&mut self) {
        if self.state_str.is_empty() {
            // Writing to string is infallible.
            _ = write!(&mut self.state_str, "{}", self.state);
        }
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
