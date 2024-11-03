use std::env;
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut length = 16;

    if args.len() >= 2{
        length = args[1].parse().unwrap()
    }

    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    println!("{}", s)
}
