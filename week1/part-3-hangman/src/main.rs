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
use std::collections::HashSet;
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
    // println!("random word: {}", secret_word);
    // Your code here! :)
    println!("Welcome to CS110L Hangman!");

    let mut hs: HashSet<char> = HashSet::new();
    let mut correct_hs: HashSet<char> = HashSet::new();
    let mut times = 0;

    for &c in secret_word_chars.iter() {
        correct_hs.insert(c);
    }

    while times < NUM_INCORRECT_GUESSES && correct_hs.len() > 0 {
        print!("The word so far is ");
        for c in secret_word_chars.iter() {
            print!("{}", if hs.contains(c) { *c } else { '-' });
        }
        print!("\nYou have guessed the following letters: ");
        for c in hs.iter() {
            print!("{}", *c);
        }
        println!("\nYou have {} guess left", NUM_INCORRECT_GUESSES - times);
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let guess: char = match guess.trim().parse() {
            Ok(c) => c,
            Err(_) => {
                print!("input is not a legal character!\n\n");
                times += 1;
                continue;
            }
        };
        if hs.contains(&guess) {
            times += 1;
            print!("You can not guess one character more than once\n\n");
        } else if correct_hs.contains(&guess) {
            correct_hs.remove(&guess);
            println!();
        } else {
            times += 1;
            print!("Sorry, that letter is not in the word\n\n");
        }
        hs.insert(guess);
    }
    if times >= NUM_INCORRECT_GUESSES {
        println!("Sorry, you ran out of guesses!");
    } else {
        println!(
            "Congratulations you guessed the secret word: {}",
            secret_word
        );
    }
}
