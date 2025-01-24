use termal::{codes, printcln, reset_terminal};

fn main() {
    printcln!("{}", codes::set_selection([], b"hello there"));
    _ = std::io::stdin().read_line(&mut String::new());
    reset_terminal();
}
