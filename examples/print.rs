use std::io::stdin;

use termal::reset_terminal;

fn main() {
    println!("\x1b[52mhello");
    _ = stdin().read_line(&mut String::new());
    reset_terminal();
}
