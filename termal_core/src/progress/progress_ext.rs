use std::borrow::Cow;

use crate::progress::BarIter;

pub trait ProgressExt: Sized + Iterator {
    fn progress_bar(
        self,
        task: impl Into<Cow<'static, str>>,
    ) -> BarIter<Self> {
        BarIter::bar(self, task)
    }
}

impl<T: Iterator> ProgressExt for T {}
