extern crate rayon;

use std::collections::HashMap;
use rayon::prelude::*;

fn main() {
    let dict = include_str!("../dict.txt");
    let dict: Vec<&str> = dict.split('\n').collect();
    let answers = include_str!("../answers.txt");
    let answers: Vec<&str> = answers.split('\n').collect();
    let mut global_letters = HashMap::<char, usize>::with_capacity(26);
    let mut global_total = 0usize;
    let mut letters = [HashMap::<char, usize>::with_capacity(26), HashMap::<char, usize>::with_capacity(26), HashMap::<char, usize>::with_capacity(26), HashMap::<char, usize>::with_capacity(26), HashMap::<char, usize>::with_capacity(26)];
    for answer in answers {
        global_total += 5;
        for (pos, c) in answer.chars().enumerate() {
            let gcounter = global_letters.entry(c).or_insert(0);
            *gcounter += 1;
            let lcounter = letters[pos].entry(c).or_insert(0);
            *lcounter += 1;
        }
    }
    let total = global_total/5;

    let mut gl: Vec<(&char, &usize)> = global_letters.iter().collect();
    gl.sort_unstable_by(|a, b| a.1.cmp(b.1));
    gl.reverse();
    println!("global letter percentages:");
    for (c, count) in &gl {
        println!(" {} {} {:.2}%", c, count, (**count as f64)/(global_total as f64)*100.0);
    }
    for i in 0..5 {
        println!("position {} percentages:", i+1);
        let mut l: Vec<(&char, &usize)> = letters[i].iter().collect();
        l.sort_unstable_by(|a, b| a.1.cmp(b.1));
        l.reverse();
        for (c, count) in &l {
            println!(" {} {} {:.2}%", c, count, (**count as f64)/(total as f64)*100.0);
        }
    }
}
