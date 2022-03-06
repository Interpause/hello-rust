use ferris_says::say;
use rand::Rng;
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
    let secret = rand_int();

    stdin().read_line(&mut guess).expect("oof");
    println!("Input was: {}", guess);
    println!("Secret was: {}", secret);
}

fn rand_int() -> i8 {
    let secret = rand::thread_rng().gen_range(1..101);
    return secret;
}
