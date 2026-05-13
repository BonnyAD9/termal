use std::{thread, time::Duration};

use termal::{Result, progress::ProgressExt};

fn main() -> Result<()> {
    for _ in (0..1270).progress_bar("something") {
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
