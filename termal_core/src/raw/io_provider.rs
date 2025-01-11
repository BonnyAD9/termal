use std::{
    io::{BufRead, Write},
    ops::{Deref, DerefMut},
};

use super::WaitForIn;

/// Represents mutable value that is either owned or borrowed.
pub enum ValueOrMut<'a, T> {
    Value(T),
    Mut(&'a mut T),
}

/// Proveder of input and output stream.
pub trait IoProvider: WaitForIn {
    type Out: Write;
    type In: BufRead + WaitForIn;

    /// Gets the output writer.
    fn get_out(&mut self) -> ValueOrMut<'_, Self::Out>;

    /// Gets the input writer.
    fn get_in(&mut self) -> ValueOrMut<'_, Self::In>;

    /// Checks if the output stream is terminal.
    fn is_out_terminal(&self) -> bool {
        false
    }

    /// Checks if the input stream is terminal
    fn is_in_terminal(&self) -> bool {
        false
    }

    /// Checks if the output is raw.
    fn is_out_raw(&self) -> bool {
        false
    }
}

impl<T> AsRef<T> for ValueOrMut<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            ValueOrMut::Value(v) => v,
            ValueOrMut::Mut(v) => v,
        }
    }
}

impl<T> AsMut<T> for ValueOrMut<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            ValueOrMut::Value(v) => v,
            ValueOrMut::Mut(v) => v,
        }
    }
}

impl<T> Deref for ValueOrMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ValueOrMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
