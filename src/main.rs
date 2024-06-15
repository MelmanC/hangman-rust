use std::env;
mod open_file;
mod game;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len == 1 {
        eprintln!("Usage: <path-to-wordlist>");
        return;
    }
    let word: String = open_file::open_file(&args[1]).unwrap();
    game::game(&word);
}
