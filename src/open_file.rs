use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

/*
    * Function to choose a random word from a file.
    * @param str: &str
    * @return String
*/
fn choose_word(str: &str) -> String {
    let mut random = rand::thread_rng();
    let words: Vec<&str> = str.split_whitespace().collect();
    let len = words.len();
    if len == 0 {
        eprintln!("Error: No words found in the file.");
        std::process::exit(1);
    }
    let index = random.gen_range(0..len);
    let word = words[index];
    word.to_string()
}

/*
    * Open a file, read its contents and return a random word from it.
    * @param path: &str
    * @return Result<String>
*/
pub fn open_file(path: &str) -> std::io::Result<String> {
    if let Err(e) = File::open(path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let word = choose_word(&contents);
    Ok(word)
}
