use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
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
    let guess: u8 = guess.trim().parse().expect("Lmao unsigned 8-bit int only");
    println!("Input was: {}", guess);
    println!("Secret was: {}", secret);

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn rand_int() -> u8 {
    let secret = rand::thread_rng().gen_range(1..101);
    return secret;
}
