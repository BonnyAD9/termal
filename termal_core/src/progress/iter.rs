use std::{
    borrow::Cow, fmt::Display, iter::FusedIterator, marker::PhantomData,
};

use crate::progress::{
    Bar, DefaultBarTheme, Dots, NoState, Progress, ProgressFormatter,
    UpdatePolicy,
};

/// Iterator tracking progress with progress bar.
pub type BarIter<
    'a,
    I,
    T = DefaultBarTheme,
    S = NoState,
    P = Progress<'a, Bar<T>, S>,
> = Iter<I, Bar<T>, S, P>;

/// Iterator tracking progress with progress bar.
pub type DotsIter<'a, I, S = NoState, P = Progress<'a, Dots, S>> =
    Iter<I, Dots, S, P>;

/// Iterator tracking progress.
#[derive(Debug, Clone)]
pub struct Iter<I, F, S, P> {
    len: usize,
    pos: usize,
    pub progress: P,
    iter: I,
    _p: PhantomData<(F, S)>,
}

impl<
    'a,
    I: Iterator,
    F: ProgressFormatter,
    S: Display,
    P: AsMut<Progress<'a, F, S>>,
> Iter<I, F, S, P>
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
    ///
    /// The default is every 100 ms.
    pub fn every(mut self, policy: impl Into<UpdatePolicy>) -> Self {
        self.progress.as_mut().set_update_policy(policy);
        self
    }

    fn update_progress(&mut self, fin: bool) {
        if fin {
            self.progress.as_mut().finish();
        } else {
            if self.pos <= self.len {
                self.progress.as_mut().update_of(self.pos, self.len);
                self.pos += 1;
            } else {
                self.progress.as_mut().update_unknown();
            }
        }
    }
}

impl<'a, I: Iterator> BarIter<'a, I> {
    /// Create new progress bar tracking iterator.
    pub fn bar(iter: I, task: impl Into<Cow<'a, str>>) -> Self {
        Self::new(iter, Progress::new(Bar::default(), task, NoState))
    }
}

impl<'a, I: Iterator> DotsIter<'a, I> {
    /// Create new progress bar tracking iterator.
    pub fn dots(iter: I, task: impl Into<Cow<'a, str>>) -> Self {
        Self::new(iter, Progress::new(Dots::default(), task, NoState))
    }
}

impl<
    'a,
    I: Iterator,
    F: ProgressFormatter,
    S: Display,
    P: AsMut<Progress<'a, F, S>>,
> Iterator for Iter<I, F, S, P>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.iter.next();
        self.update_progress(res.is_none());
        res
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<
    'a,
    I: DoubleEndedIterator,
    F: ProgressFormatter,
    S: Display,
    P: AsMut<Progress<'a, F, S>>,
> DoubleEndedIterator for Iter<I, F, S, P>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let res = self.iter.next_back();
        self.update_progress(res.is_none());
        res
    }
}

impl<
    'a,
    I: ExactSizeIterator,
    F: ProgressFormatter,
    S: Display,
    P: AsMut<Progress<'a, F, S>>,
> ExactSizeIterator for Iter<I, F, S, P>
{
}

impl<
    'a,
    I: FusedIterator,
    F: ProgressFormatter,
    S: Display,
    P: AsMut<Progress<'a, F, S>>,
> FusedIterator for Iter<I, F, S, P>
{
}
