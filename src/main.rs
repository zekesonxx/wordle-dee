#[macro_use]
extern crate lazy_static;
extern crate rayon;
extern crate rand;

use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::cmp;
use rand::seq::SliceRandom;
use rayon::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
struct LetterStats {
    pub global_count: usize,
    pub global_freq: f64,
    pub local_count: [usize; 5],
    pub local_freq: [f64; 5]
}

impl LetterStats {
    fn new() -> Self {
        LetterStats::default()
    }
    fn calculate_freqs(&mut self, global_total: usize) {
        self.global_freq = self.global_count as f64 / global_total as f64;
        let local_total = (global_total/5) as f64;
        self.local_freq = [self.local_count[0] as f64/local_total,
                           self.local_count[1] as f64/local_total,
                           self.local_count[2] as f64/local_total,
                           self.local_count[3] as f64/local_total,
                           self.local_count[4] as f64/local_total];
    }
}

fn strtoarr(input: &str) -> [char; 5] {
    assert_eq!(input.len(), 5);
    let chars: Vec<char> = input.chars().collect();
    chars.try_into().unwrap()
}

/// Struct representing a game of Wordle
/// Works with words as 5 element arrays of chars
#[derive(Default, Debug)]
struct WordleGame {
    pub word: [char; 5],
    pub guesses: Vec<[char; 5]>,
    pub yellow: [Vec<char>; 5],
    pub green: [Option<char>; 5],
    pub grey: Vec<char>
}

impl WordleGame {
    /// Make a new Wordle game, with a particular word
    fn new(word: &str) -> Self {
        assert_eq!(word.len(), 5);
        let mut game = WordleGame::default();
        game.word = strtoarr(word);
        game
    }

    /// Execute a guess for a word
    fn guess(&mut self, word: &str) {
        assert_eq!(word.len(), 5);
        let guess = strtoarr(word);
        self.guesses.push(guess);
        for i in 0..5 {
            if guess[i] == self.word[i] {
                self.green[i] = Some(guess[i]);
            } else if self.word.contains(&guess[i]) {
                match self.yellow[i].binary_search(&guess[i]) {
                    Ok(pos) => {},
                    Err(pos) => self.yellow[i].insert(pos, guess[i])
                }
            } else {
                match self.grey.binary_search(&guess[i]) {
                    Ok(pos) => {},
                    Err(pos) => self.grey.insert(pos, guess[i])
                }
            }
        }
    }

    /// Return true if the game has been won, or false if not
    fn has_won(&self) -> bool {
        if self.guesses.len() > 0 {
            *self.guesses.last().unwrap() == self.word
        } else {
            false
        }
    }

    /// Consider how good a guess could be
    /// Returns number of letters matching each color, as so:
    /// (green, yellow, grey, G%, L%)
    /// Percentage at the end is the % of letters revealed by the unseen letters
    /// G% is global percentage, L% is local percentage
    /// Returns None if guess has already been guessed
    fn consider_guess(&self, word: &str) -> Option<(usize, usize, usize, f64, f64)> {
        let guess = strtoarr(word);
        if self.guesses.contains(&guess) {
            return None;
        }
        let mut green = 0;
        let mut yellow = 0;
        let mut grey = 0;
        let mut global_revealed = 0.0;
        let mut local_revealed = 0.0;
        let mut revealed_letters = vec![];
        'outer: for i in 0..5 {
            if let Some(c) = self.green[i] {
                if guess[i] == c {
                    green += 1;
                    continue;
                }
            }
            if self.grey.contains(&guess[i]) {
                grey += 1;
                continue;
            }
            for (lpos, l) in self.yellow.iter().enumerate() {
                if l.contains(&guess[i]) {
					if lpos == i {
						// yellow on this spot counts as a grey
						grey += 1
					} else {
						yellow += 1;
					}
                    continue 'outer;
                }
            }
            if !revealed_letters.contains(&guess[i]) {
				revealed_letters.push(guess[i]);
				global_revealed += LETTERS[&guess[i]].global_freq;
				local_revealed += LETTERS[&guess[i]].local_freq[i];
            }
        }

        debug_assert!(green+yellow+grey <= 5);
        Some((green, yellow, grey, global_revealed, local_revealed))
    }

}

impl fmt::Display for WordleGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "word:  ")?;
        for i in self.word {
            write!(f, "{}", i)?;
        }
        write!(f, " | ")?;

        for (i, guess) in self.guesses.iter().enumerate() {
            for i in guess {
                write!(f, "{}", i)?;
            }
            if 2+i < self.guesses.len()+1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "\ngreen: ")?;
        for i in self.green {
            if let Some(c) = i {
                write!(f, "{}", c)?;
            } else {
                write!(f, "*")?;
            }
        }

        write!(f, " | grey: ")?;
        for i in &self.grey {
            write!(f, "{}", i)?;
        }
        write!(f, "\n{:?}", self.yellow)?;

        Ok(())
    }
}

lazy_static! {
    static ref NONANSWER_DICT: Vec<&'static str> = {
        let dict = include_str!("../dict.txt");
        let dict: Vec<&str> = dict.split('\n').filter(|x| x.len() == 5).collect();
        dict
    };
    static ref ANSWER_DICT: Vec<&'static str> = {
        let answers = include_str!("../answers.txt");
        let answers: Vec<&str> = answers.split('\n').filter(|x| x.len() == 5).collect();
        answers
    };
    static ref DICT: Vec<&'static str> = {
		let mut dict = NONANSWER_DICT.clone();
		dict.append(&mut ANSWER_DICT.clone());
		dict
    };
    static ref LETTERS: HashMap::<char, LetterStats> = {
        let mut letters = HashMap::<char, LetterStats>::with_capacity(26);
        let mut global_total = 0usize;
        for answer in ANSWER_DICT.iter() {
            global_total += 5;
            for (pos, c) in answer.chars().enumerate() {
                let letter = letters.entry(c).or_insert_with(LetterStats::default);
                letter.global_count += 1;
                letter.local_count[pos] += 1;
            }
        }
        for letter in letters.values_mut() {
            (*letter).calculate_freqs(global_total);
        }
        letters
    };
}

fn main() {
    lazy_static::initialize(&DICT);
    lazy_static::initialize(&LETTERS);
    let starter = "bears";
    let mut game = WordleGame::new(ANSWER_DICT.choose(&mut rand::thread_rng()).unwrap());
    println!("{}", game);
    game.guess(starter);
    println!("{:?}", game);
    let mut guesses = 1;
    while !game.has_won() && guesses < 6 {
        guesses += 1;
        let mut guess: Vec<(&&str, (usize, usize, usize, f64, f64))> = ANSWER_DICT.par_iter().filter_map(|word| {
            if let Some(x) = game.consider_guess(word) {
                Some((word, x))
            } else {
                None
            }
        })
        .filter(|x| x.1.2 == 0)
        .collect();
        
        guess.par_sort_by(|x, y| x.1.3.partial_cmp(&y.1.3).unwrap());
        guess.par_sort_by(|x, y| x.1.4.partial_cmp(&y.1.4).unwrap());
        guess.par_sort_by_key(|x| x.1.1);
        //guess.par_sort_by(|x, y| x.1.2.cmp(&y.1.2).reverse());
        guess.par_sort_by_key(|x| x.1.0);
        
        guess.reverse();
        for i in 0..cmp::min(10, guess.len()) {
            println!("{:2.}. {} G{} Y{} G{} {:.2}%G {:.2}%L",
                    i+1, guess[i].0, guess[i].1.0, guess[i].1.1, guess[i].1.2, guess[i].1.3, guess[i].1.4);
        }
        println!();
        println!("guess: {:?}", guess[0]);
        game.guess(guess[0].0);
        println!("{}", game);
    }
    if game.has_won() {
		println!("Solved after {} guesses", guesses);
    } else {
		println!("Failed after {} guesses", guesses);
    }

}
