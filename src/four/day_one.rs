use std::fs;
use std::str::FromStr;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let rows = contents.split("\n").collect::<Vec<&str>>();
    let mut row_iter = rows.iter().peekable();

    let result: String = String::from_str("").expect("Error message");
    while row_iter.peek().is_some() {}
    println!("{}", result);
}
