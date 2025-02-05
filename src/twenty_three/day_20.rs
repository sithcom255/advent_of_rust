use crate::solve::Solve;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (mut b, mut f, mut c) = parse(input);
        let mut low_c = 0;
        let mut high_c = 0;
        for i in 0..1000 {
            let mut stack: VecDeque<(&str, bool, &str)> = VecDeque::new();
            low_c += 1;
            for elem in &b {
                stack.push_back((elem, false, "broadcast"))
            }

            while let Some((elem, high, source)) = stack.pop_front() {
                if high {
                    high_c += 1;
                } else {
                    low_c += 1;
                }

                if f.contains_key(elem) {
                    if !high {
                        let (val, todo) = f.remove(&elem).unwrap();
                        for t in &todo {
                            stack.push_back((t, !val, elem));
                        }
                        f.insert(elem, (!val, todo));
                    }
                } else if c.contains_key(elem) {
                    let (ref mut map, ref mut todo) = c.get_mut(elem).unwrap();
                    map.insert(source, high);

                    let mut good = true;
                    for k in map.values() {
                        good = good && *k;
                    }

                    for k in todo {
                        stack.push_back((k, !good, elem));
                    }
                }
            }
        }
        (low_c * high_c).to_string()
    }

    fn p2(input: &String) -> String {
        let (mut b, mut f, mut c) = parse(input);
        let mut low_c = 0;
        let mut high_c = 0;

        let mut deps;
        {
            let (x, todo) = c.get("gq").unwrap();
            deps = x.keys().map(|elem| elem.clone()).collect::<Vec<&str>>().clone();
        }
        for i in 0..10000000 {
            let mut stack: VecDeque<(&str, bool, &str)> = VecDeque::new();
            low_c += 1;
            for elem in &b {
                stack.push_back((elem, false, "broadcast"))
            }

            while let Some((elem, high, source)) = stack.pop_front() {
                if deps.contains(&&source) && high {
                    for i in 0..deps.len() {
                        if deps[i] == source {
                            deps.remove(i);
                            break;
                        }
                    }
                    println!("{source} - > {i}");
                    if deps.is_empty() {
                        return "".to_string();
                    }
                }

                if high {
                    high_c += 1;
                } else {
                    low_c += 1;
                }

                if f.contains_key(elem) {
                    if !high {
                        let (val, todo) = f.remove(&elem).unwrap();
                        for t in &todo {
                            stack.push_back((t, !val, elem));
                        }
                        f.insert(elem, (!val, todo));
                    }
                } else if c.contains_key(elem) {
                    let (ref mut map, ref mut todo) = c.get_mut(elem).unwrap();
                    map.insert(source, high);

                    let mut good = true;
                    for k in map.values() {
                        good = good && *k;
                    }

                    for k in todo {
                        stack.push_back((k, !good, elem));
                    }
                }
            }
        }
        unreachable!()
    }
}

fn parse(
    input: &String,
) -> (
    Vec<&str>,
    HashMap<&str, (bool, Vec<&str>)>,
    HashMap<&str, (HashMap<&str, bool>, Vec<&str>)>,
) {
    let mut b: Vec<&str> = vec![];
    let mut f: HashMap<&str, (bool, Vec<&str>)> = HashMap::new();
    let mut c: HashMap<&str, (HashMap<&str, bool>, Vec<&str>)> = HashMap::new();

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split("->").collect();
        if parts[0].contains(&"&") {
            c.insert(
                &parts[0].trim_end().trim_start().strip_prefix("&").unwrap(),
                (
                    HashMap::new(),
                    parts[1]
                        .trim_end()
                        .trim_start()
                        .split(",")
                        .map(|elem| {
                            return elem.trim_end().trim_start();
                        })
                        .collect(),
                ),
            );
        }
    });

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split("->").collect();
        if parts[0].contains(&"&") {
            let target = parts[0].trim_end().trim_start().strip_prefix("&").unwrap();
            parts[1]
                .trim_end()
                .trim_start()
                .split(",")
                .for_each(|elem| {
                    let transformed = elem.trim_end().trim_start();
                    if c.contains_key(&transformed) {
                        let (x, todo) = c.get_mut(&transformed).unwrap();
                        x.insert(target, false);
                    }
                });
        } else if parts[0].contains(&"%") {
            let target = parts[0].trim_end().trim_start().strip_prefix("%").unwrap();
            let todo = parts[1]
                .trim_end()
                .trim_start()
                .split(",")
                .map(|elem| {
                    let transformed = elem.trim_end().trim_start();
                    if c.contains_key(&transformed) {
                        let (x, todo) = c.get_mut(&transformed).unwrap();
                        x.insert(target, false);
                    }
                    return transformed;
                })
                .collect();
            f.insert(&target, (false, todo));
        } else {
            b = parts[1]
                .trim_end()
                .trim_start()
                .split(",")
                .map(|elem| {
                    return elem.trim_end().trim_start();
                })
                .collect();
        }
    });

    (b, f, c)
}

#[test]
fn test() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_12 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_12 P2 {p2_res}");
}

#[test]
fn solve_2023_1() {
    let contents = fs::read_to_string("./resources/twenty_three/day_20.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_20 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_20 P2 {p2_res}");
}
