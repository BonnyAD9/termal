pub mod request;
pub mod terminal;

use std::time::Duration;

use termal::{
    Result, codes,
    raw::{
        Terminal,
        events::{AmbiguousEvent, AnyEvent, Event, Status},
        request as requests, request_ambiguous,
    },
};

pub fn show_request_ambiguous() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let evt = request_ambiguous(
        codes::REQUEST_TERMINAL_NAME,
        Duration::from_millis(100),
        |e| {
            if let AmbiguousEvent {
                event: AnyEvent::Known(Event::Status(Status::TerminalName(n))),
                ..
            } = e
            {
                Some(n)
            } else {
                None
            }
        },
    )?;

    term.consume_available()?;

    println!("{evt:#?}");

    Ok(())
}

pub fn show_request() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let evt = requests(
        codes::REQUEST_TERMINAL_NAME,
        Duration::from_millis(100),
        |e| {
            if let Event::Status(Status::TerminalName(n)) = e {
                Some(n)
            } else {
                None
            }
        },
    )?;

    term.consume_available()?;

    println!("{evt:#?}");

    Ok(())
}
