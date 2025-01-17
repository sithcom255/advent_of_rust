use crate::solve::Solve;
use std::collections::HashSet;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut p1 = 0;
        let rows = parse(input);

        for (winning, chosen) in rows {
            let mut i = 0;
            for n in chosen {
                if winning.contains(&n) {
                    if i == 0 {
                        i = 1;
                    } else {
                        i = i << 1
                    }
                }
            }
            p1 += i;
        }

        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let mut p2 = 0;
        let rows = parse(input);
        let mut count = vec![1; rows.len()];
        for x in 0..rows.len() {
            let (winning, chosen) = &rows[x];
            let mut i = 0;
            for n in chosen {
                if winning.contains(&n) {
                    i += 1;
                }
            }
            for add in 0 + 1..i + 1 {
                let target = x + add;
                if target < rows.len() {
                    count[target] += count[x];
                }
            }
        }

        let p2: usize = count.iter().sum();
        p2.to_string()
    }
}

pub fn parse(input: &str) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let mut vec1 = vec![];
    input.lines().for_each(|line| {
        let x: Vec<&str> = line.split("|").into_iter().collect();
        let first: Vec<&str> = x[0].split(":").into_iter().collect();
        let winning: HashSet<usize> = first[1]
            .trim_start()
            .trim_end()
            .split_whitespace()
            .map(|elem| elem.parse::<usize>().unwrap())
            .collect();
        let chosen = x[1]
            .trim_start()
            .trim_end()
            .split_whitespace()
            .map(|elem| elem.parse::<usize>().unwrap())
            .collect();
        vec1.push((winning, chosen))
    });

    vec1
}

#[test]
fn solve_2023_4() {
    let contents = fs::read_to_string("./resources/twenty_three/day_4.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_4 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_4 {p2_res}");
}
