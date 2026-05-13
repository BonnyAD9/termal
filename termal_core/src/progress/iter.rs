use std::{borrow::Cow, fmt::Display, marker::PhantomData};

use crate::progress::{
    Bar, NoState, Progress, ProgressFormatter, UpdatePolicy,
};

/// Iterator tracking progress with progress bar.
pub type ProgressBarIter<I, S = NoState, P = Progress<Bar, S>> =
    Iter<I, Bar, S, P>;

/// Iterator tracking progress.
#[derive(Debug, Clone)]
pub struct Iter<I, F, S, P> {
    len: usize,
    pos: usize,
    pub progress: P,
    iter: I,
    _p: PhantomData<(F, S)>,
}

impl<I: Iterator, F: ProgressFormatter, S: Display, P: AsMut<Progress<F, S>>>
    Iter<I, F, S, P>
{
    /// Create new progress iterator.
    pub fn new(iter: I, progress: P) -> Self {
        let len = iter.size_hint().1.unwrap_or_default();
        Self {
            len,
            pos: 0,
            progress,
            iter,
            _p: PhantomData,
        }
    }

    /// Change how often the progress updates. This may be either usize
    /// specifying number of iterations or duration specifying time.
    pub fn every(mut self, policy: impl Into<UpdatePolicy>) -> Self {
        self.progress.as_mut().set_update_policy(policy);
        self
    }
}

impl<I: Iterator> ProgressBarIter<I> {
    /// Create new progress bar tracking iterator.
    pub fn bar(iter: I, task: impl Into<Cow<'static, str>>) -> Self {
        Self::new(iter, Progress::new(Bar::default(), task, NoState))
    }
}

impl<I: Iterator, F: ProgressFormatter, S: Display, P: AsMut<Progress<F, S>>>
    Iterator for Iter<I, F, S, P>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.iter.next();
        if res.is_some() {
            if self.pos <= self.len {
                self.progress.as_mut().update_of(self.pos, self.len);
                self.pos += 1;
            } else {
                self.progress.as_mut().update_unknown();
            }
        } else {
            self.progress.as_mut().finish();
        }
        res
    }
}
