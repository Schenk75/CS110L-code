// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let len = secret_word_chars.len();
    let mut all_guess = vec!['-'; len];
    let mut guess_word: String;
    let mut guess_so_far: String = String::from("");
    let mut count = 0;
    while count < NUM_INCORRECT_GUESSES {
        guess_word = all_guess.iter().collect();
        if guess_word == secret_word {
            println!("Congratulations you guessed the secret word: {}", guess_word);
            break;
        }
        println!("The word so far is {}", guess_word);

        println!("You have guessed the following letters: {}", guess_so_far);

        println!("You have {} guesses left", NUM_INCORRECT_GUESSES-count);

        println!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let guess = if let Some(x) = guess.chars().next(){x} else {'1'};
        guess_so_far.push(guess);

        let mut flag =false;
        for (i, val) in secret_word_chars.iter().enumerate() {
            if *val == guess {
                all_guess[i] = guess;
                flag = true;
            }
        }
        if !flag {
            count += 1;
            println!("Sorry, that letter is not in the word");
        }
        println!();
    }
    if count == NUM_INCORRECT_GUESSES {
        println!("Sorry, you ran out of guesses!");
    }
    
}
