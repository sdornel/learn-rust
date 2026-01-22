use std::io::Write;

fn main() {
    loop {
        let mut input = String::new();
        // keep terminal output visible
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();

        println!("You entered: {}", input.trim());
    }
}
