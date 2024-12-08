use colored::*;
use rand::Rng;
use serde_json::from_str;
use std::io;

fn main() {
    let welcome_message = "Welcome to Wordle CLI!".on_blue().bold().underline();

    println!("{welcome_message}");

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

    loop {
        let mut number_of_tries = 5;

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();

        if guess.len() != 5 {
            println!("Please enter a 5 letter word");
            continue;
        }

        for (i, character) in guess.chars().enumerate() {
            if rand_word.contains(character) {
                if rand_word.chars().nth(i).unwrap() == character {
                    let formated_char = character.to_string().to_uppercase().on_green().bold();
                    let formated_space = " ".on_green().bold();
                    print!("{formated_space}{formated_char}{formated_space}");
                } else {
                    let formated_char = character.to_string().to_uppercase().on_yellow().bold();
                    let formated_space = " ".on_yellow().bold();
                    print!("{formated_space}{formated_char}{formated_space}");
                }
            } else {
                let formated_char = character
                    .to_string()
                    .to_uppercase()
                    .on_bright_black()
                    .bold();
                let formated_space = " ".on_bright_black().bold();
                print!("{formated_space}{formated_char}{formated_space}");
            }
            print!(" ");
        }

        number_of_tries -= 1;

        if guess == *rand_word {
            println!("\nYou won!");
            break;
        }

        if number_of_tries == 0 {
            println!("\nYou lost! The word was: {}", rand_word);
            break;
        }

        println!();
    }
}
