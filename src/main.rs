use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::collections::HashSet;

/**
 * Hangman implementation.
 * 1. We generate a word from a word bank.
 * 2. We draw the UI, given the word
 * 3. We update the UI, given a guess
 */

fn main() {
    // Randomly Generate a word
    println!("Welcome to Hangman! Finding a word for you!");
    let word = random_word();

    println!("Found a word, starting the game for you!");
    let mut state = HangmanState { word, guessed: HashSet::new(), n_wrong: 0 };

    while !(state.game_win() || state.game_loss()) {
        println!("Pick a letter, any letter!");
        state.display();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        state.update(input.chars().next().unwrap());
    }

    if state.game_win() {
        println!("Congratulations, you won!")
    } else {
        println!("You lose, better luck next time!")
    }
}

#[derive(Debug)]
pub struct HangmanState {
    word: String,
    guessed: HashSet<char>,
    n_wrong: i32
}

static LOSE_AFTER : u8 = 7;

impl HangmanState {

    // Additional benefit that a repeat guess doesn't hurt you
    pub fn update(&mut self, guessed_char: char) {
        if !self.word.contains(guessed_char) {
            self.n_wrong += 1;
        }
        self.guessed.insert(guessed_char);
    }

    pub fn display(&self) {
        let disp : String = self.word.chars().map(|c| {
            if self.guessed.contains(&c) {
                c
            } else {
                '_'
            }
        }).collect();

        println!("{}", &disp);
        println!("Guessed: {:?}", self.guessed);
        println!("Number of wrong guesses before you lose: {}", LOSE_AFTER as i32 - self.n_wrong)
    }

    pub fn game_win(&self) -> bool {
        self.word.chars().fold(true, |agg, c| self.guessed.contains(&c) && agg)
    }

    pub fn game_loss(&self) -> bool {
        self.n_wrong >= LOSE_AFTER as i32
    }
}

// Although this function has a side-effect, let's keep it as such, since it only gets used once per program
fn random_word() -> String {
    let mut rng: ThreadRng = rand::thread_rng();
    let num_words = 65081;
    let random = rng.gen_range(0..num_words);

    // ? operator pooping out on me for some reason...
    let file: File = File::open("data/dictionary.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().nth(random).unwrap().unwrap()
}
