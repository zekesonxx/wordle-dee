extern crate rayon;

use std::collections::HashMap;
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

fn main() {
    let dict = include_str!("../dict.txt");
    let dict: Vec<&str> = dict.split('\n').collect();
    let answers = include_str!("../answers.txt");
    let answers: Vec<&str> = answers.split('\n').collect();
    let mut letters = HashMap::<char, LetterStats>::with_capacity(26);
    let mut global_total = 0usize;
    for answer in answers {
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
    println!("{:?}", letters);
}
