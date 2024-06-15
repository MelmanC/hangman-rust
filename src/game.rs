use std::io::{self, Write};

fn enter_letter() -> String {
    let mut input = String::new();
    print!("Enter a letter: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn check_if_letter_exist(letter: &str, word: &str, guessed: &mut Vec<char>) ->
    bool {
    let mut found = false;

    for (i, c) in word.chars().enumerate() {
        if c.to_string() == letter {
            guessed[i] = c;
            found = true;
        }
    }
    found
}

pub fn game(word: &str) -> () {
    let mut win: bool = false;
    let word_len = word.len();
    let mut guessed = vec!['_'; word_len];
    let mut tries = 10;

    while win == false {
        println!("Word: {}", guessed.iter().collect::<String>());
        println!("Tries left: {}", tries);
        let input = enter_letter();
        let found = check_if_letter_exist(&input, word, &mut guessed);
        if !found {
            tries -= 1;
        }
        if tries == 0 {
            println!("You lost! The word was: {}", word);
            break;
        }
        if guessed.iter().collect::<String>() == word {
            println!("You won! The word was: {}", word);
            win = true;
        }
        println!();
    }
}
