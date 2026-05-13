use std::borrow::Cow;

use crate::progress::ProgressBarIter;

pub trait ProgressExt: Sized + Iterator {
    fn progress_bar(
        self,
        task: impl Into<Cow<'static, str>>,
    ) -> ProgressBarIter<Self> {
        ProgressBarIter::bar(self, task)
    }
}

impl<T: Iterator> ProgressExt for T {}
