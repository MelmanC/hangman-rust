use std::env;
mod open_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len == 1 {
        println!("No arguments provided");
        return;
    }

}