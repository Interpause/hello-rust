use ferris_says::say;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let msg = String::from("Hello Rust!");
    let width = msg.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(msg.as_bytes(), width, &mut writer).unwrap();
    guess();
}

fn guess() {
    println!("Input guess: ");
    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("oof");
    println!("Input was: {}", guess);
}
