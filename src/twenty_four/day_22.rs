use std::collections::HashMap;
use std::fs;

pub fn aoc_p1(contents: String) -> String {
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    let mask = 16777216 - 1;

    input
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .for_each(|number| {
            let mut res = number;
            for i in 0..2000 {
                let a = res << 6;
                res = res ^ a;
                res = res & mask;

                let b = res >> 5;
                res = res ^ b;
                res = res & mask;

                let c = res << 11;
                res = res ^ c;
                res = res & mask;
            }
            p1 += res;
        });

    p1.to_string()
}

fn mix(n: usize) {}

pub fn aoc_p2(contents: String) -> String {
    let mut input: Vec<&str> = contents.split("\n").collect();

    let mask = 16777216 - 1;

    let floating_mask = (1 << 20) - 1;

    let mut cache: HashMap<usize, isize> = HashMap::with_capacity(floating_mask);

    input
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .for_each(|number| {
            let mut res = number;
            let mut floating_hash = 0usize;
            let mut least_important_decimal = 0isize;

            let mut current_cache: HashMap<usize, isize> = HashMap::new();

            for i in 0..2000 {
                least_important_decimal = (res % 10) as isize;

                let a = res << 6;
                res = res ^ a;
                res = res & mask;

                let b = res >> 5;
                res = res ^ b;
                res = res & mask;

                let c = res << 11;
                res = res ^ c;
                res = res & mask;

                let current = (res % 10) as isize;
                let change = current - least_important_decimal;
                if change < 0 {
                    floating_hash = (floating_hash << 5) + (change.abs() as usize + 16);
                } else {
                    floating_hash = (floating_hash << 5) + change.abs() as usize;
                }
                floating_hash = floating_hash & floating_mask;

                if i > 2 {
                    let option = current_cache.get(&floating_hash);
                    match option {
                        None => {
                            current_cache.insert(floating_hash, current);
                        }
                        Some(value) => {

                        }
                    }
                }
            }

            current_cache.iter().for_each(|(key, val)| {
                let option = cache.get(&key);
                match option {
                    None => {
                        cache.insert(*key, *val);
                    }
                    Some(value) => {
                        cache.insert(*key, value + *val as isize);
                    }
                }
            });
        });

    let mut max = 0isize;
    cache.iter().for_each(|(key,val)| {
        if *val > max {
            max = *val;
            println!("key {}, {}", key, val)
        }
    });

    max.to_string()
}

#[test]
fn example() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    assert_eq!("37990510", aoc_p1(contents.to_owned()));
    assert_eq!("23", aoc_p2(contents));
}

#[test]
fn solve_() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/day_25.txt")
        .expect("This is the error message");

    let p1_res = aoc_p1(contents.to_owned());
    println!("RES P1 {p1_res}");

    let p2_res = aoc_p2(contents.to_owned());
    println!("RES P2 {p2_res}");
}

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}
