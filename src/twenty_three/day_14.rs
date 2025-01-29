use std::collections::HashMap;
use crate::solve::Solve;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut rows = parse(input);
        let y_len = rows.len() as isize;
        let x_len = rows[0].len() as isize;
        to_north(&mut rows, -1, 0, 0, y_len, 0, x_len);
        weight(&rows).to_string()
    }

    fn p2(input: &String) -> String {
        let mut rows = parse(input);

        let y_len = rows.len() as isize;
        let x_len = rows[0].len() as isize;

        let mut start_period = 0;
        let mut period = 0;
        let mut cache : HashMap<Vec<Vec<char>>,usize> =HashMap::new();
        for i in 0..1_000 {
            to_north(&mut rows, -1, 0, 0, y_len, 0, x_len);
            to_north(&mut rows, 0, -1, 0, y_len, 0, x_len);
            to_north(&mut rows, 1, 0, y_len - 1, 0 - 1, 0, x_len);
            to_north(&mut rows, 0, 1, 0, y_len, x_len - 1, 0 - 1);
            if cache.contains_key(&rows) {
                start_period = i;
                match cache.get(&rows) {
                    None => {}
                    Some(prev) => {
                        period = start_period - prev;
                        break;
                    }
                } cache.get(&rows);
            }
            cache.insert(rows.clone(), i);
        }

        let target = 1_000_000_000;
        let t_start = target - ((target - start_period) % period);
        for i in t_start + 1..target {
            to_north(&mut rows, -1, 0, 0, y_len, 0, x_len);
            to_north(&mut rows, 0, -1, 0, y_len, 0, x_len);
            to_north(&mut rows, 1, 0, y_len - 1, 0 - 1, 0, x_len);
            to_north(&mut rows, 0, 1, 0, y_len, x_len - 1, 0 - 1);
        }

        weight(&rows).to_string()
    }
}

fn weight(rows: &Vec<Vec<char>>) -> usize {
    let y_len = rows.len();
    let mut res = 0;
    for i in 0..y_len {
        let weight = y_len - i;
        let row = &rows[i];
        for ch in row {
            if *ch == 'O' {
                res += weight;
            }
        }
    }

    res
}

fn to_north(
    rows: &mut Vec<Vec<char>>,
    y_inc: isize,
    x_inc: isize,
    y_start: isize,
    y_end: isize,
    x_start: isize,
    x_end: isize,
) {
    let y_len = rows.len();
    let x_len = rows[0].len();

    let mut x_iter = std::iter::successors(Some(x_start), |n| {
        if x_start > x_end {
            return if n - 1 == x_end { None } else { Some(n - 1) };
        } else {
            return if n + 1 == x_end { None } else { Some(n + 1) };
        }
    });

    let mut y_iter = std::iter::successors(Some(y_start), |n| {
        if y_start > y_end {
            return if n - 1 == y_end { None } else { Some(n - 1) };
        } else {
            return if n + 1 == y_end { None } else { Some(n + 1) };
        }
    });

    for y in y_iter {
        for x in x_iter.clone() {
            let ch = rows[y as usize][x as usize];
            if ch == 'O' {
                let mut y_source = y;
                let mut x_source = x;
                let mut y_res = y + y_inc;
                let mut x_res = x + x_inc;
                while y_res >= 0 && y_res < y_len as isize && x_res >= 0 && x_res < x_len as isize {
                    if rows[y_res as usize][x_res as usize] == '.' {
                        rows[y_source as usize][x_source as usize] = '.';
                        rows[y_res as usize][x_res as usize] = 'O';
                        y_source = y_res;
                        x_source = x_res;
                        y_res = y_res + y_inc;
                        x_res = x_res + x_inc;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn parse(input: &String) -> Vec<Vec<char>> {
    let mut rows = vec![];
    input.lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        rows.push(row);
    });
    rows
}

#[test]
fn solve_2023_14() {
    let contents = fs::read_to_string("./resources/twenty_three/day_14.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_14 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_14 P@ {p2_res}");
}
