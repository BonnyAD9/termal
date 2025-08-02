use std::{io::stdin, time::Duration};

use termal::{
    Result, codes,
    raw::{
        Terminal, disable_raw_mode, enable_raw_mode, raw_guard, wait_for_stdin,
    },
};

pub fn show_stdio() -> Result<()> {
    let mut term = Terminal::stdio();

    term.flushed(codes::CLEAR)?;
    term.println("This will print to stdout.")?;
    enable_raw_mode()?;
    let data = term.prompt("Enter data to stdin: ")?;
    disable_raw_mode()?;
    term.println(format!("\nData you entered to stdin: {data}"))?;

    Ok(())
}

pub fn show_read_byte() -> Result<()> {
    let mut term = Terminal::stdio();

    term.flushed(codes::CLEAR)?;
    term.flushed("Enter single byte: ")?;
    enable_raw_mode()?;
    let byte = term.read_byte()?;
    disable_raw_mode()?;
    term.println(format!("\nYou entered byte `0x{byte:2x}`."))?;
    term.println(format!(
        "It coresponds to the character `{}`.",
        byte as char
    ))?;

    Ok(())
}

pub fn show_has_input() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    println!("Before entering:");
    println!("has_input: {}", term.has_input());
    println!("has_buffered_input: {}", term.has_buffered_input());
    println!(
        "wait_for_stdin(ZERO): {}\n",
        wait_for_stdin(Duration::ZERO)?
    );

    term.flushed("Please enter something: ")?;
    // Wait for the user to type something.
    term.wait_for_input(Duration::MAX)?;
    println!();

    // Now there should be input but it is only buffered in the underlaying
    // input stream.
    println!("After entering, before reading:");
    println!("has_input: {}", term.has_input());
    println!("has_buffered_input: {}", term.has_buffered_input());
    println!(
        "wait_for_stdin(ZERO): {}\n",
        wait_for_stdin(Duration::ZERO)?
    );

    term.read_byte()?;

    // Now there is also buffered input in the terminal if there was more than
    // one byte.
    println!("After reading single byte:");
    println!("has_input: {}", term.has_input());
    println!("has_buffered_input: {}", term.has_buffered_input());
    println!(
        "wait_for_stdin(ZERO): {}\n",
        wait_for_stdin(Duration::ZERO)?
    );

    // Wait for next input on stdin (not counting what is buffered in term).
    term.flushed("Enter something more: ")?;
    wait_for_stdin(Duration::MAX)?;
    println!();

    println!("After next input before consuming previous:");
    println!("has_input: {}", term.has_input());
    println!("has_buffered_input: {}", term.has_buffered_input());
    println!("wait_for_stdin(ZERO): {}", wait_for_stdin(Duration::ZERO)?);

    term.consume_available()?; // Consume all the input

    Ok(())
}

pub fn show_wait_for_input() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    term.flushed("You have one second to enter \"wait for input\"\n> ")?;
    if term.wait_for_input(Duration::from_secs(1))? {
        let mut data = String::new();
        // Using the standart blocking read_line without raw mode. It won't
        // block because there is input ready and stdin is line buffered.
        stdin().read_line(&mut data)?;
        if data != "wait for input\n" {
            println!("You misspelled it!");
        } else {
            println!("Good work!");
        }
    } else {
        println!("\nOoops! Too late!");
    }

    // Consume the data that has already been typed but not consumed because of
    // line buffering.
    raw_guard(true, || term.consume_available())?;

    Ok(())
}
