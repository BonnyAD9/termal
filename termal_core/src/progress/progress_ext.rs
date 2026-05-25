use std::borrow::Cow;

use crate::progress::{BarIter, DotsIter};

pub trait ProgressExt: Sized + Iterator {
    /// Track the iterator progress using inline progress bar.
    fn progress_bar<'a>(
        self,
        task: impl Into<Cow<'a, str>>,
    ) -> BarIter<'a, Self> {
        BarIter::bar(self, task)
    }

    /// Track the iterator progress using inline dots.
    fn progress_dots<'a>(
        self,
        task: impl Into<Cow<'a, str>>,
    ) -> DotsIter<'a, Self> {
        DotsIter::dots(self, task)
    }
}

impl<T: Iterator> ProgressExt for T {}
