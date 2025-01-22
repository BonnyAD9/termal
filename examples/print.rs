use termal::{printcln, reset_terminal};

fn main() {
    printcln!("\x1b]4;220;?\x07");
    _ = std::io::stdin().read_line(&mut String::new());
    reset_terminal();
}
