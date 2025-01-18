use crate::solve::Solve;
use std::cmp::max;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut min = 2 << 60;

        let (mut seeds, mut mappings) = parse(input);
        for mut mapping in &mut mappings {
            mapping.sort_by(|a, b| {
                return a[1].cmp(&b[1]);
            })
        }

        println!("{:?}", mappings);
        println!("{:?}", seeds);

        for seed in seeds {
            let mut cur = seed;
            for mapping in &mappings {
                cur = range_binary_search(cur, mapping, 0, mapping.len());
                print!(" {} ", cur)
            }
            print!("location {}", cur);
            if cur < min {
                min = cur;
            }
        }

        min.to_string()
    }

    fn p2(input: &String) -> String {
        let mut min = 2 << 60;

        let (mut seeds, mut mappings) = parse(input);
        for mut mapping in &mut mappings {
            mapping.sort_by(|a, b| {
                return a[1].cmp(&b[1]);
            })
        }

        for i in 0..seeds.len() {
            if i % 2 == 1 {
                continue;
            }

            let mut cur = vec![(seeds[i], seeds[i + 1])];
            for mapping in &mappings {
                let mut next_depth = vec![];
                for (start, c_n) in cur {
                    for x in resolve_ranges(start, c_n, mapping) {
                        next_depth.push(x);
                    };
                }
                cur = next_depth;
            }
            cur.sort_by(|a, b| {
                return a.0.cmp(&b.0);
            });

            if cur[0].0 < min {
                min = cur[0].0;
            }
        }

        min.to_string()
    }
}

pub fn range_binary_search(target: usize, mapping: &Vec<Vec<usize>>, l: usize, r: usize) -> usize {
    let mid = (l + r) / 2;
    if mid >= mapping.len() {
        return target;
    }

    let range = &mapping[mid];

    if range[1] <= target && target < (range[1] + range[2]) {
        return range[0] + target - range[1];
    }

    if l >= r {
        return target;
    }

    return if range[1] > target {
        range_binary_search(target, mapping, l, mid)
    } else {
        range_binary_search(target, mapping, mid + 1, r)
    };

    0
}

pub fn resolve_ranges(target: usize, n: usize, mapping: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut c_target = target;
    let mut c_n = n;

    let mut resolved = vec![];

    for range in mapping {
        if c_target >= range[1] + range[2] {
            continue;
        }

        if c_target < range[1] {
            let distance_to_start = range[1] - c_target;

            if distance_to_start >= c_n {
                resolved.push((c_target, c_n));
                return resolved;
            } else {
                resolved.push((c_target, distance_to_start));
                c_n = c_n - distance_to_start;
                c_target = range[1];
            }
        }

        let mut start = range[0] + c_target - range[1];
        let mut distance = range[1] + range[2] - c_target;

        if distance < c_n {
            resolved.push((start, c_n));
            c_n = 0;
            return resolved;
        } else {
            resolved.push((start, distance));
            c_n = c_n - distance;
            c_target = range[1] + range[2];
        }
    }

    if c_n > 0 {
        resolved.push((c_target, c_n));
    }

    resolved
}

pub fn parse(input: &String) -> (Vec<usize>, Vec<Vec<Vec<usize>>>) {
    let mut seeds: Vec<usize> = vec![];
    let mut mapping: Vec<Vec<Vec<usize>>> = vec![];

    let mut curr: Vec<Vec<usize>> = vec![];
    let mut start: bool = false;
    for line in input.lines() {
        if !start {
            let line: Vec<&str> = line.split(":").collect();
            seeds = line[1]
                .trim_end()
                .trim_start()
                .split_whitespace()
                .map(|elem| elem.parse::<usize>().unwrap())
                .collect();
            start = true;
            continue;
        }

        if line.is_empty() && !&curr.is_empty() {
            mapping.push(curr);
            curr = vec![];
            continue;
        }

        if line.contains("map") {
            continue;
        }

        let one: Vec<usize> = line
            .split_whitespace()
            .map(|elem| elem.parse::<usize>().unwrap())
            .collect();
        if !one.is_empty() {
            curr.push(one);
        }

        start = true;
    }

    if !&curr.is_empty() {
        mapping.push(curr);
        curr = vec![];
    }
    (seeds, mapping)
}

#[test]
fn solve_2023_5() {
    let contents = fs::read_to_string("./resources/twenty_three/day_5.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_5 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_5 {p2_res}");
}

#[test]
fn test() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_2 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_2 P2 {p2_res}");
}
