use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn choose_word(str: &str) -> String {
    let mut random = rand::thread_rng();
    let words: Vec<&str> = str.split_whitespace().collect();
    let len = words.len();
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
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let word = choose_word(&contents);
    Ok(word)
}
