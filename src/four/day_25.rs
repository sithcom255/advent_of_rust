use crate::four::day_10::bound_check;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

pub fn aoc_p1(contents: String) -> String {
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();
    let mut keys: Vec<Vec<usize>> = Vec::new();
    let mut holes: Vec<Vec<usize>> = Vec::new();

    holes.push(vec![0,0,0,0,0]);
    let mut hole: bool = true;
    let mut reset: bool = false;
    input.iter().for_each(|line| {
        if reset {
            if line.contains("##") {
                hole = true;
                holes.push(vec![0,0,0,0,0]);
            } else {
                keys.push(vec![0,0,0,0,0]);
            }
            reset = false;
        }

        if line.is_empty() {
            reset = true;
            hole = false;
            return;
        }

        if hole {
            let mut i = 0;
            line.chars().for_each(|cj| {
                if cj == '#' {
                    let i1 = holes.len();
                    holes[i1 - 1][i] += 1;
                }

                i += 1;
            })
        } else {
            let mut i = 0;
            line.chars().for_each(|cj| {
                if cj == '#' {
                    let i2 = keys.len();
                    keys[i2 - 1][i] += 1;
                }
                i += 1;
            })
        }
    });


    println!("{:?}", keys);
    println!("{:?}", holes);

    for hole_i in 0..holes.len() {
        let hole = &holes[hole_i];
        for key_i in 0..keys.len() {
            let key = &keys[key_i];
            let mut ok: bool = true;
            for i in 0..key.len() {
                ok &= hole[i] + key[i] <= 7;
            }
            if ok {
                p1 += 1;
            }
        }
    }

    p1.to_string()
}

pub fn aoc_p2(contents: String) -> String {
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    p1.to_string()
}

#[test]
fn example() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    assert_eq!("3", aoc_p1(contents.to_owned()));
}

#[test]
fn solve_() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/input")
        .expect("This is the error message");

    let p1_res = aoc_p1(contents.to_owned());
    println!("RES P1 {p1_res}");

    let p2_res = aoc_p2(contents.to_owned());
    println!("RES P2 {p2_res}");
}

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}
