use std::io::{self, Write};

use termal::{
    Result, codes,
    raw::{
        StdioProvider, Terminal, disable_raw_mode, enable_raw_mode,
        events::{AmbiguousEvent, AnyEvent, Event, Key, KeyCode, Modifiers},
    },
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    print!("{}", codes::REQUEST_SELECTION);
    _ = io::stdout().flush();

    start()?;

    print!("");
    disable_raw_mode()?;
    _ = io::stdout().flush();

    Ok(())
}

fn start() -> Result<()> {
    let mut stdout = io::stdout();
    let mut term = Terminal::<StdioProvider>::default();

    loop {
        let key = term.read_ambiguous()?;
        if matches!(
            key,
            AmbiguousEvent {
                event: AnyEvent::Known(Event::KeyPress(Key {
                    code: KeyCode::Char('c'),
                    modifiers: Modifiers::CONTROL,
                    ..
                })),
                ..
            }
        ) {
            return Ok(());
        }
        print!("{key:?}\n\r");
        stdout.flush()?;
    }
}
