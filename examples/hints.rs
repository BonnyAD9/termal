use std::io::{Write, stdout};

use termal::{
    Result, codes, formatc,
    raw::{
        StdioProvider, Terminal, disable_raw_mode, enable_raw_mode,
        events::{Event, Key, KeyCode},
        readers::TermRead,
    },
};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut terminal = Terminal::<StdioProvider>::default();
    let mut reader = TermRead::lines(&mut terminal);
    let mut buf = String::new();

    while unsafe { !reader.read_one()? } {
        // What has been read.
        let read: &[char] = reader.get_input();
        // Position in what has been read.
        let pos: usize = reader.get_pos();

        let word = "autocomplete";
        let to_complete = &word[read.len().min(word.len())..];

        if to_complete.is_empty() {
            continue;
        }

        if matches!(
            reader.last_event(),
            Some(Event::KeyPress(Key {
                code: KeyCode::Tab,
                ..
            }))
        ) {
            let rl = read.len();
            reader.splice(rl..rl, to_complete.chars());
            reader.reshow()?;
            reader.queue([Event::KeyPress(Key::code(KeyCode::End))]);
        } else {
            buf.clear();
            buf += codes::CUR_SAVE;
            buf.extend(&read[pos..]);
            buf +=
                &formatc!("{'gr}{}{'_}", &word[read.len().min(word.len())..]);
            buf += codes::CUR_LOAD;
            print!("{buf}");
            _ = stdout().flush();
        }
    }

    // get the entered string
    let s = reader.finish()?;

    disable_raw_mode()?;

    println!("\nread(ed): {s}");

    Ok(())
}
