use std::{thread, time::Duration};

use termal::{Result, progress::ProgressExt};

fn main() -> Result<()> {
    for _ in (0..100).progress_bar("something").every(0) {
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
