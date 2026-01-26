use std::io::{self, Write};

pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn has_no_color_flag() -> bool {
    std::env::args().any(|arg| arg == "--no-color")
}

pub fn colorize<'a>(code: &'a str, disabled: bool) -> &'a str {
    if disabled {
        ""
    } else {
        code
    }
}
