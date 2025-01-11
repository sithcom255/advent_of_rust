use regex::Regex;
use std::{collections::HashSet, fs};

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("This is the error message");
        
    let mut sliding_window: Vec<char> = vec![];
    for i in 0..contents.len() {
        if sliding_window.len() == 14 {
            let _ = sliding_window.remove(0);
        }
        sliding_window.push(contents.chars().nth(i).unwrap());
        if sliding_window.len() == 14 && evaluate(& sliding_window) {
            println!("{}, {:?}", i + 1, sliding_window);
            return;
        }
    }
}

pub fn evaluate(window:& Vec<char>) -> bool {
    let mut uniq = HashSet::new();
    window.into_iter().all(|x| uniq.insert(x.clone()))
}
