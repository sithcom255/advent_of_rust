use std::collections::{HashMap, HashSet, VecDeque};
use crate::solve::Solve;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let map = parse(input);
        let size = map.len();
        let mut next = map.iter().next();
        let mut visited = HashSet::new();

        let mut todo = VecDeque::new();
        todo.push_back(next.unwrap().0);
        while let Some(current) = todo.pop_front() {
            if visited.contains(current) {
                continue;
            }
            match map.get(current) {
                None => {}
                Some(entry) => {
                    for n in entry {
                        todo.push_back(n);
                    }
                }
            };

            visited.insert(current);
        }



        (visited.len() * (size - visited.len())).to_string()
    }

    fn p2(input: &String) -> String {
       "".to_owned()
    }
}

fn parse(input: &String) -> HashMap<&str, HashSet<&str>> {
    let mut map:HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut i = 0;
    input.lines().for_each(|elem| {
        let p:Vec<&str> = elem.split(":").collect();
        let edge:HashSet<&str> = p[1].trim_end().trim_start().split(" ").collect();
        let s = p[0];
        for e in &edge {
            println!("{s}--{e} [label=\"{i}\" labelfloat=true];");
            i += 1;
            match map.get_mut(e) {
                None => {
                    let mut set = HashSet::new();
                    set.insert(s);
                    map.insert(e, set);}
                Some(prev) => {
                    prev.insert(s);
                }
            }
        }
        match map.get_mut(s) {
            None => {map.insert(s, edge);}
            Some(prev) => {
                edge.iter().for_each(|e| { prev.insert(e); })
            }
        }

    });
    map
}

#[test]
fn solve_2023_25() {
    let contents = fs::read_to_string("./resources/twenty_three/day_25.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_25 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_25 {p2_res}");
}
