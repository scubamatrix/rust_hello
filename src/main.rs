use ferris_says::say;
use std::io::{stdout, BufWriter};

fn say_hello() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn hello() {
    println!("Hello, world!");
}

fn main() {
    hello();
    say_hello();
}

