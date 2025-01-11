use crate::solve::Solve;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

struct Solver {}

impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut p1 = 0;

        let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
        let mut input: Vec<&str> = input.split("\n").collect();
        input.iter().for_each(|line| {
            let mut split = line.split("-");

            let from = split.next().unwrap();
            let to = split.next().unwrap();

            insert_to_adj(&mut adj, from, to);
            insert_to_adj(&mut adj, to, from);
        });

        let string = "".to_owned();
        let cycles = easy_find(&adj);

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

    fn p2(input: &String) -> String {
        let mut p1 = 0;

        let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
        let mut input: Vec<&str> = input.split("\n").collect();
        input.iter().for_each(|line| {
            let x: Vec<&str> = line.split("-").collect();

            let from = x[0];
            let to = x[1];

            insert_to_adj(&mut adj, from, to);
            insert_to_adj(&mut adj, to, from);
        });

        let cycles = easy_find(&adj);

        let mut adj: HashMap<String, HashSet<String>> = HashMap::new();

        cycles.iter().for_each(|vec| {
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

        let mut maxvec: Vec<Vec<String>> = vec![vec![]];
        bron_kerbosch(
            &adj,
            &mut HashSet::new(),
            &mut adj
                .keys()
                .clone()
                .into_iter()
                .map(|e| e.to_owned())
                .collect(),
            &mut HashSet::new(),
            &mut maxvec,
        );

        maxvec[0].sort();
        maxvec[0].join(",")
    }
}

fn bron_kerbosch(
    adj: &HashMap<String, HashSet<String>>,
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    result: &mut Vec<Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > result[0].len() {
            let res: Vec<String> = r.iter().map(|e| e.to_owned()).collect();
            result[0] = res;
        }
    }

    if p.len() + r.len() < result[0].len() {
        return;
    }

    let mut set: Vec<String> = p.clone().iter().map(|e| e.to_owned()).collect();

    set.sort_by(|a, b| {
        return adj.get(a).unwrap().len().cmp(&adj.get(b).unwrap().len());
    });

    for cadidate in set.iter() {
        let mut neighbors = adj.get(cadidate).unwrap();
        r.insert(cadidate.to_owned());
        let mut new_p: HashSet<String> = neighbors.intersection(p).map(|e| e.to_owned()).collect();
        let mut new_x: HashSet<String> = neighbors.intersection(p).map(|e| e.to_owned()).collect();
        bron_kerbosch(adj, r, &mut new_p, &mut new_x, result);
        r.remove(cadidate);
        p.remove(cadidate);
        x.insert((*cadidate).clone());
    }
}

fn easy_find(all: &HashMap<String, HashSet<String>>) -> HashSet<Vec<&String>> {
    let mut cycles: HashSet<Vec<&String>> = HashSet::new();

    all.iter().for_each(|(start, adjacent)| {
        adjacent.iter().for_each(|adj_one| {
            adjacent.iter().for_each(|adj_two| {
                if all.get(adj_one).unwrap().contains(adj_two) {
                    let mut vec1 = vec![start, adj_one, adj_two];
                    vec1.sort();
                    cycles.insert(vec1);
                }
            });
        });
    });
    cycles
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
    let contents = fs::read_to_string("./src/test").expect("This is the error message");

    assert_eq!("7", Solver::p1(&contents));
    assert_eq!("co,de,ka,ta", Solver::p2(&contents));
}

#[test]
fn solve_() {
    let contents = fs::read_to_string("./resources/twenty_four/day_23.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("RES P1 {p1_res}");

    let mut p2 = "".to_owned();
    for i in 0..50 {
        let candidate = Solver::p2(&contents);
        if p2.len() < candidate.len() {
            p2 = candidate;
        }
    }
    println!("RES P2 {p2}");
}

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}
