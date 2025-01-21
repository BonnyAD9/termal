use termal::reset_terminal;

fn main() {
    println!("\nsomething hello there\x1b#4");
    // _ = stdin().read_line(&mut String::new());
    reset_terminal();
}
