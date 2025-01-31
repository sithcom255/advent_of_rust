use crate::solve::Solve;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ptr::null;
use std::str::Split;
use std::thread::yield_now;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut p1 = 0;
        let commands: Vec<&str> = input
            .split(",")
            .map(|elem| elem.trim_end().trim_start())
            .collect();
        commands.iter().for_each(|cmd| {
            p1 += Self::hash_code(cmd);
        });
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let mut p2 = 0;
        let commands: Vec<&str> = input
            .split(",")
            .map(|elem| elem.trim_end().trim_start())
            .collect();
        let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();

        for command in commands {
            if command.contains("=") {
                let parts: Vec<&str> = command.split("=").collect();
                let code = parts[0];
                let box_i = Solver::hash_code(&code);
                let value = parts[1].parse::<usize>().unwrap();
                match boxes.get_mut(&box_i) {
                    None => {
                        boxes.insert(box_i, vec![(code, value)]);
                    }
                    Some(already) => {
                        let size = already.len();

                        let mut done = false;
                        for i in 0..size {
                            let (key, _val) = already[i];
                            if key == code {
                                already[i] = (code, value);
                                done = true;
                            }
                        }
                        if !done {
                            already.push((code, value));
                        }
                    }
                }
            } else {
                match command.strip_suffix("-") {
                    None => {}
                    Some(result) => {
                        let box_i = Solver::hash_code(&result);
                        match boxes.get_mut(&box_i) {
                            None => {}
                            Some(already) => {
                                let size = already.len();
                                for i in 0..size {
                                    let (key, value) = already[i];
                                    if key == result {
                                        already.remove(i);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                };
            }
        }
        evaluate(&boxes).to_string()
    }
}

impl Solver {
    fn hash_code(cmd: &&str) -> usize {
        let mut x = 0;
        cmd.chars().for_each(|ch| {
            x = step(ch, x);
        });
        x
    }
}

fn evaluate(boxes: &HashMap<usize, Vec<(&str, usize)>>) -> usize {
    let mut p = 0;
    boxes.iter().for_each(|(key, value)| {
        for i in 0..value.len() {
            let (code, val) = value[i];
            p += (i + 1) * (key + 1) * val;
        }
    });
    p
}

fn step(ch: char, current: usize) -> usize {
    let ascii_value = ch as u8;
    let i = ((current + ascii_value as usize) * 17) % 256;
    i
}

#[test]
fn solve_2023_15() {
    let contents = fs::read_to_string("./resources/twenty_three/day_15.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_15 P2 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_15 P2 {p2_res}");
}
