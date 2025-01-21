use std::io::stdin;

use termal::reset_terminal;

fn main() {
    println!("something hello there\x1b#3");
    // _ = stdin().read_line(&mut String::new());
    reset_terminal();
}
