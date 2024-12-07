use rand::Rng;
use serde_json::from_str;
use std::io;

fn main() {
    println!("Welcome to Wordle CLI!");

    let word_list = std::fs::read_to_string("./src/words-list.json").unwrap();

    let word_list = match from_str::<Vec<String>>(&word_list) {
        Err(e) => {
            println!("Error: {e}");
            return;
        }
        Ok(word_list) => word_list,
    };

    let rand_word_index = rand::thread_rng().gen_range(0..word_list.len());

    let rand_word = &word_list[rand_word_index];

    println!("The random word is: {}", rand_word);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess == rand_word {
            println!("You win!");
            break;
        } else {
            println!("Try again!");
        }
    }
}
