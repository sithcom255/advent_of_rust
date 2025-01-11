use crate::twenty_four::day_24::State::{DIRECT_AND, DIRECT_XOR, MERGE_OVERFLOW, PROBLEM};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs;
use std::ops::Add;



pub fn aoc_p1(contents: String) -> String {
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    let mut known: HashMap<&str, bool> = HashMap::new();
    let mut todo: Vec<Vec<&str>> = vec![];

    let mut done: bool = false;
    input.iter().for_each(|line| {
        if line.is_empty() {
            done = true;
            return;
        }

        if !done {
            let x: Vec<&str> = line.split(":").collect();
            let i = x[1].trim_start().trim_end().parse::<usize>().unwrap();
            let bit = i == 1;
            known.insert(x[0], bit);
        } else {
            let part: Vec<&str> = line.split(" -> ").collect();
            let mut left: Vec<&str> = part[0]
                .trim_start()
                .trim_end()
                .split(" ")
                .map(|elem| {
                    return elem.trim_end().trim_start();
                })
                .collect();
            left.push(part[1]);
            todo.push(left);
        }
    });

    while !todo.is_empty() {
        let mut removed_i = vec![];
        for i in 0..todo.len() {
            let vec = &todo[i];
            if known.contains_key(vec[0]) && known.contains_key(vec[2]) {
                let res = match vec[1] {
                    "XOR" => known.get(vec[0]).unwrap() ^ known.get(vec[2]).unwrap(),
                    "AND" => *known.get(vec[0]).unwrap() && *known.get(vec[2]).unwrap(),
                    "OR" => *known.get(vec[0]).unwrap() || *known.get(vec[2]).unwrap(),
                    _ => {
                        println!("OROBLEM");
                        false
                    }
                };
                known.insert(vec[3], res);
                removed_i.push(i);
            }
        }
        for i in 0..removed_i.len() {
            let to_remove = removed_i[i];
            todo.remove(to_remove - i);
        }
    }

    let mut res: Vec<(&&str, &bool)> = known
        .iter()
        .filter(|(key, value)| {
            return key.starts_with("z");
        })
        .collect();
    res.sort();

    let mut result: usize = 0;
    res.reverse();
    println!("{:?}", res);

    res.iter().for_each(|(key, value)| {
        result = result << 1;
        let x1 = **value;
        if x1 {
            result += 1;
            print!("1");
        } else {
            print!("0");
        }
    });

    result.to_string()
}

pub fn aoc_p2(contents: String) -> String {
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    let mut known: HashMap<&str, bool> = HashMap::new();
    let mut states: Vec<HashMap<State, String>> = Vec::new();
    for i in 0..60 {
        states.push(HashMap::new());
    }

    let mut by_signal_name: HashMap<String, Status> = HashMap::new();
    let mut reverse_by_signal_name: HashMap<String, Status> = HashMap::new();

    let mut signal_name_lookup: HashMap<String, String> = HashMap::new();
    let mut todo: Vec<Vec<String>> = vec![];

    let mut done: bool = false;
    input.iter().for_each(|line| {
        if line.is_empty() {
            done = true;
            return;
        }

        if !done {
            let x: Vec<&str> = line.split(":").collect();
            let i = x[1].trim_start().trim_end().parse::<usize>().unwrap();
            let bit = i == 1;
            known.insert(x[0], bit);
        } else {
            let part: Vec<String> = line
                .split(" -> ")
                .map(|elem| {
                    return elem.to_owned();
                })
                .collect();
            let mut left: Vec<String> = part[0]
                .trim_start()
                .trim_end()
                .split(" ")
                .map(|elem| {
                    return elem.trim_end().trim_start().to_owned();
                })
                .collect();
            signal_name_lookup.insert(left.join(" "), part[1].to_owned());
            left.reverse();
            signal_name_lookup.insert(left.join(" "), part[1].to_owned());
            left.reverse();
            left.push(part[1].clone());
            todo.push(left);
        }
    });

    let map = (&mut states[1]);
    map.insert(State::MERGE_OVERFLOW, "qkf".to_string());

    for i in 2..45 {
        println!("Line {}",i);
        let mut x: String;
        let mut y: String;

        if i < 10 {
            x = "x0".to_string().add(&*i.to_string());
            y = "y0".to_string().add(&*i.to_string());
        } else {
            x = "x".to_string().add(&*i.to_string());
            y = "y".to_string().add(&*i.to_string());
        }

        let direct_and_signal = signal_name_lookup
            .get(&vec![x.clone(), "AND".to_string(), y.clone()].join(" "))
            .unwrap()
            .to_owned();

        let direct_xor_signal = signal_name_lookup
            .get(&vec![x.clone(), "XOR".to_string(), y.clone()].join(" "))
            .unwrap()
            .to_owned();

        by_signal_name.insert(
            direct_and_signal.clone(),
            Status {
                state: State::DIRECT_AND,
                position: i,
            },
        );
        by_signal_name.insert(
            direct_xor_signal.clone(),
            Status {
                state: State::DIRECT_XOR,
                position: i,
            },
        );

        let previous_merge_signal = states[i - 1].get(&MERGE_OVERFLOW).unwrap().to_owned();

        let z1 = signal_name_lookup
            .get(
                &vec![
                    previous_merge_signal.clone(),
                    "XOR".to_string(),
                    direct_xor_signal.clone(),
                ]
                .join(" "),
            )
            .unwrap()
            .to_owned();

        let propagate_signal = signal_name_lookup
            .get(
                &vec![
                    previous_merge_signal.clone(),
                    "AND".to_string(),
                    direct_xor_signal.clone(),
                ]
                .join(" "),
            )
            .unwrap()
            .to_owned();

        by_signal_name.insert(
            propagate_signal.clone(),
            Status {
                state: State::PROPAGATE_OVERFLOW,
                position: i,
            },
        );

        let current_merge_overflow = signal_name_lookup
            .get(&vec![propagate_signal, "OR".to_string(),direct_and_signal ].join(" "))
            .unwrap();

        let map = (&mut states[i]);
        map.insert(State::MERGE_OVERFLOW, current_merge_overflow.to_string());

        if !z1.starts_with("z") {
            println!("XOR OUT not z, found {}", z1);
        }
    }

    p1.to_string()
}

fn get_next_state(l: State, l_signal: &String, r: State, r_signal: &String, rel: String) -> State {
    match rel.as_str() {
        "XOR" => {
            if l == State::Y && r == State::X {
                return DIRECT_XOR;
            } else if l == State::MERGE_OVERFLOW && r == State::DIRECT_XOR {
                return State::Z;
            }
        }
        "OR" => {
            if l == State::DIRECT_AND && r == State::PROPAGATE_OVERFLOW {
                return State::MERGE_OVERFLOW;
            }
        }
        "AND" => {
            if l == State::Y && r == State::X {
                return DIRECT_AND;
            }
            if l == State::MERGE_OVERFLOW && r == State::DIRECT_XOR {
                return State::PROPAGATE_OVERFLOW;
            }
            if l == State::DIRECT_XOR && r == State::PROPAGATE_OVERFLOW {
                return State::Z;
            }
        }
        _ => {}
    };
    println!("Encountered {:?}-{} {:?}-{}", l, l_signal, r, r_signal);
    return PROBLEM;
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Status {
    state: State,
    position: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum State {
    Y,
    X,
    DIRECT_AND,
    XOR_OUT,
    DIRECT_XOR,
    PROPAGATE_OVERFLOW,
    MERGE_OVERFLOW,
    Z,
    PROBLEM,
}

#[test]
fn example() {
    let contents = fs::read_to_string("./src/test")
        .expect("This is the error message");

    assert_eq!("2024", aoc_p1(contents.to_owned()));
}

#[test]
fn solve_2024_24() {
    let contents = fs::read_to_string("./resources/twenty_four/day_24.txt")
        .expect("This is the error message");

    let p1_res = aoc_p1(contents.to_owned());
    println!("RES P1 {p1_res}");

    let p2_res = aoc_p2(contents.to_owned());
    println!("RES P2 {p2_res}");
}
