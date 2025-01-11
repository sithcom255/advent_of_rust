use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

pub fn aoc_p1(contents: String) -> String {
    let mut p1 = 0;

    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
    let mut input: Vec<&str> = contents.split("\n").collect();
    input.iter().for_each(|line| {
        let x: Vec<&str> = line.split("-").collect();

        let from = x[0];
        let to = x[1];

        insert_to_adj(&mut adj, from, to);
        insert_to_adj(&mut adj, to, from);
    });

    let string = "".to_owned();
    let cycles = easy_find(&adj, &string);

    cycles.iter().for_each(|v| {
        let mut starts = false;
        for i in 0..v.len() {
            if v[i].starts_with("t") {
                p1 += 1;
                break;
            };
        }
    });

    p1.to_string()
}

fn easy_find<'a>(
    adj: &'a HashMap<String, HashSet<String>>,
    start: &'a String,
) -> HashSet<Vec<&'a String>> {
    let mut cycles: HashSet<Vec<&String>> = HashSet::new();

    adj.iter().for_each(|(key, second_values)| {
        second_values.iter().for_each(|second_level| {
            adj.get(second_level).iter().for_each(|third_value| {
                third_value.iter().for_each(|third_level| {
                    adj.get(third_level).iter().for_each(|candidate| {
                        candidate.iter().for_each(|ffs| {
                            if ffs == key {
                                let mut vec1 = vec![key, second_level, third_level];
                                vec1.sort();
                                cycles.insert(vec1);
                            }
                        })
                    });
                });
            });
        });
    });
    cycles
}

fn find_cycles<'a>(
    adj: &'a HashMap<String, HashSet<String>>,
    start: &'a String,
) -> HashSet<Vec<&'a String>> {
    let mut done = HashSet::new();

    let mut cycles: HashSet<Vec<&String>> = HashSet::new();

    adj.iter().for_each(|(key, value)| {
        if done.contains(key) {
            return;
        }
        let mut so_far: Vec<&String> = Vec::new();
        let mut visited: HashMap<&String, usize> = HashMap::new();

        find_cycles_rec(
            adj,
            &mut visited,
            &mut so_far,
            &mut cycles,
            &mut done,
            start,
            key,
        )
    });
    cycles
}

fn find_cycles_rec<'a>(
    adj: &'a HashMap<String, HashSet<String>>,
    visited: &mut HashMap<&'a String, usize>,
    so_far: &mut Vec<&'a String>,
    cycles: &mut HashSet<Vec<&'a String>>,
    done: &mut HashSet<&'a String>,
    previous: &'a String,
    current: &'a String,
) {
    match visited.get(current) {
        None => {}
        Some(index) => {
            if *index == 0 {
                return;
            }
            let cycle_size = so_far.len() - index;
            if cycle_size == 3 {
                let mut starts_t = false;
                for i in *index..so_far.len() {
                    let string = &so_far[i];
                    if string.starts_with("t") {
                        starts_t = true;
                        break;
                    }
                }
                if starts_t {
                    let mut x = so_far[*index..so_far.len()].to_owned();
                    x.sort();
                    cycles.insert(x);
                }
            }
            return;
        }
    }

    let outgoing = match adj.get(current) {
        None => {
            return;
        }
        Some(outgoing) => outgoing,
    };

    outgoing.iter().for_each(|out| {
        if out == previous {
            return;
        }
        if done.contains(out) {
            return;
        }
        visited.insert(&current, so_far.len());
        so_far.push(&current);
        find_cycles_rec(adj, visited, so_far, cycles, done, current, out);
        visited.remove(current);
        so_far.pop();
    });

    done.insert(&current);
}

pub fn aoc_p2(contents: String) -> String {
    let mut p1 = 0;

    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
    let mut input: Vec<&str> = contents.split("\n").collect();
    input.iter().for_each(|line| {
        let x: Vec<&str> = line.split("-").collect();

        let from = x[0];
        let to = x[1];

        insert_to_adj(&mut adj, from, to);
        insert_to_adj(&mut adj, to, from);
    });

    let string = "".to_owned();
    let cycles = easy_find(&adj, &string);

    let mut all: HashSet<&String> = HashSet::new();

    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();

    let mut max: usize = 0;
    let mut maxvec: Vec<String> = vec!();
    cycles.iter().for_each(|vec| {
        vec.iter().for_each(|st| {
            all.insert(st);
        });
        let a = vec[0];
        let b = vec[1];
        let c = vec[2];
        insert_to_adj(&mut adj, a, b);
        insert_to_adj(&mut adj, b, a);
        insert_to_adj(&mut adj, a, c);
        insert_to_adj(&mut adj, c, a);
        insert_to_adj(&mut adj, b, c);
        insert_to_adj(&mut adj, c, b);
    });


    let todo:Vec<&String> = adj.keys().collect();
    for key in todo {
        let value = adj.get(key).unwrap();
        if value.len() < max {
            continue;
        }
        let mut candidate = value.clone();
        candidate.insert(key.clone());
        let mut ordered: Vec<&String> = value.iter().collect();

        ordered.sort_by(|a, b| {
            return adj.get(*a).unwrap().len().cmp(&adj.get(*b).unwrap().len());
        });

        for i in 0..ordered.len() {
            let other = ordered[i];

            if !candidate.contains(other) {
                continue;
            }

            match adj.get(other) {
                None => {}
                Some(value) => {
                    let mut set = value.clone();
                    set.insert(other.to_owned());
                    candidate = candidate.intersection(&set).map(|elem| elem.to_owned()).collect();
                }
            };
        };

        if candidate.len() > max {
            max = candidate.len() + 1;
            maxvec = candidate.iter().map(|e| {e.to_owned()}).collect();
        }
    }

    p1 = max;
    println!("RES P2 {}", max);
    maxvec.sort();
    maxvec.join(",")
}

fn insert_to_adj(mut adj: &mut HashMap<String, HashSet<String>>, from: &str, to: &str) {
    match adj.get_mut(from) {
        None => {
            let mut set = HashSet::new();
            set.insert(to.to_owned());
            adj.insert(from.to_owned(), set);
        }
        Some(set) => {
            set.insert(to.to_owned());
        }
    };
}

#[test]
fn example() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    assert_eq!("7", aoc_p1(contents.to_owned()));
    assert_eq!("co,de,ka,ta", aoc_p2(contents.to_owned()));
}

#[test]
fn solve_() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/input")
        .expect("This is the error message");

    let p1_res = aoc_p1(contents.to_owned());
    println!("RES P1 {p1_res}");

    for i in 0..50 {
        let p2_res = aoc_p2(contents.to_owned());
        println!("RES P2 {p2_res}");
    }
}

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}
