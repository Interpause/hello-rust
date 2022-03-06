use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let msg = String::from("Hello Rust!");
    let width = msg.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(msg.as_bytes(), width, &mut writer).expect("stdout error");

    let secret = rand_int();

    while !guess(secret) {}
}

fn guess(secret: u8) -> bool {
    println!("Input guess: ");
    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("stdin error");

    let guess: u8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Lmao unsigned 8-bit int only");
            return false;
        }
    };
    // println!("Input was: {}", guess);
    // println!("Secret was: {}", secret);

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }
    return false;
}

fn rand_int() -> u8 {
    let secret = rand::thread_rng().gen_range(1..101);
    return secret;
}
