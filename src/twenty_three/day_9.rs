use crate::solve::Solve;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut p1 = 0;
        let rows = parse(input);
        for i in 0..rows.len() {
            let mut expanded: Vec<Vec<isize>> = vec![];
            let row = &rows[i];

            let mut next = get_next(row);
            let mut is_zero: bool = are_null(&next);

            while !is_zero {
                let p = get_next(&next);
                expanded.push(next);
                next = p;
                is_zero = are_null(&next);
            }

            let mut res: isize = 0;
            for x in 0..expanded.len() {
                let e_row = &expanded[x];
                res += e_row.last().unwrap();
            }
            res += row.last().unwrap();
            p1 += res;
        }
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let mut p1 = 0;
        let mut rows = parse(input);
        for i in 0..rows.len() {
            let mut expanded: Vec<Vec<isize>> = vec![];
            let mut row = &mut rows[i];
            row.reverse();

            let mut next = get_next(row);
            let mut is_zero: bool = are_null(&next);

            while !is_zero {
                let p = get_next(&next);
                expanded.push(next);
                next = p;
                is_zero = are_null(&next);
            }

            let mut res: isize = 0;
            for x in 0..expanded.len() {
                let e_row = &expanded[x];
                res -= e_row.last().unwrap();
            }
            res -= row.last().unwrap();
            p1 -= res;
        }
        p1.to_string()
    }
}

fn are_null(vec: &Vec<isize>) -> bool {
    for i in 0..vec.len() {
        if vec[i] != 0 {
            return false;
        }
    }
    return true;
}
fn get_next(current: &Vec<isize>) -> Vec<isize> {
    let mut result = vec![];
    for x in 0..(current.len() - 1) {
        result.push(current[x + 1] - current[x]);
    }
    result
}

fn parse(input: &String) -> Vec<Vec<isize>> {
    let mut rows = vec![];
    input.lines().for_each(|line| {
        let row: Vec<isize> = line
            .split_whitespace()
            .map(|elem| elem.parse::<isize>().unwrap())
            .collect();
        rows.push(row);
    });
    rows
}



#[test]
fn solve_2023_9() {
    let contents = fs::read_to_string("./resources/twenty_three/day_9.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_1 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_1 {p2_res}");
}
