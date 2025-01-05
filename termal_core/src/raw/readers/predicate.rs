use crate::raw::events::{Event, Key, KeyCode};

pub trait Predicate<T> {
    fn matches(&self, value: &T) -> bool;
}

impl<F, T> Predicate<T> for F
where
    F: Fn(&T) -> bool,
{
    fn matches(&self, value: &T) -> bool {
        self(value)
    }
}

impl Predicate<Event> for KeyCode {
    fn matches(&self, value: &Event) -> bool {
        matches!(value, Event::KeyPress(Key { code, .. }) if code == self)
    }
}

impl Predicate<Event> for Key {
    fn matches(&self, value: &Event) -> bool {
        matches!(value, Event::KeyPress(key) if self.same_key(key))
    }
}
