use rand::{distributions::Alphanumeric, Rng};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut length = 16;

    if args.len() >= 2 {
        length = args[1]
            .parse()
            .expect(format!("Could not parse '{}' as an integer", args[1]).as_str());
    }

    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    println!("{}", s)
}
