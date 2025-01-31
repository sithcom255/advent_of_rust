use crate::solve::Solve;
use std::collections::HashSet;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let rows = parse(input);
        let p1 = Self::get_enegized(&rows, 0, 0, 0, 1);
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let rows = parse(input);

        let y_len = rows.len();
        let x_len = rows[1].len();
        let mut mx = 0;
        for x in 0..x_len {
            {
                let cnd = Self::get_enegized(&rows, 0, x as isize, 1, 0);
                if cnd > mx {
                    mx = cnd;
                }
            }
            {
                let cnd = Self::get_enegized(&rows, y_len as isize - 1, x as isize, -1, 0);
                if cnd > mx {
                    mx = cnd;
                }
            }
        }
        for y in 0..y_len {
            {
                let cnd = Self::get_enegized(&rows, y as isize, 0, 0, 1);
                if cnd > mx {
                    mx = cnd;
                }
            }

            {
                let cnd = Self::get_enegized(&rows, y as isize, x_len as isize - 1, 0, -1);
                if cnd > mx {
                    mx = cnd;
                }
            }
        }
        mx.to_string()
    }
}

impl Solver {
    fn get_enegized(rows: &Vec<Vec<char>>, y: isize, x: isize, y_inc: isize, x_inc: isize) -> i32 {
        let y_len = rows.len();
        let x_len = rows[1].len();
        let mut visited: Vec<Vec<HashSet<(isize, isize)>>> =
            vec![vec![HashSet::new(); x_len]; y_len];

        walk(&rows, &mut visited, y, x, y_inc, x_inc);
        let mut p1 = 0;
        for y in 0..y_len {
            for x in 0..x_len {
                if visited[y][x].len() > 0 {
                    p1 += 1;
                }
            }
        }
        p1
    }
}

fn walk(
    rows: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<HashSet<(isize, isize)>>>,
    y: isize,
    x: isize,
    y_inc: isize,
    x_inc: isize,
) {
    let y_len = rows.len();
    let x_len = rows[1].len();

    if x < 0 || x >= x_len as isize || y < 0 || y >= y_len as isize {
        return;
    }

    if visited[y as usize][x as usize].contains(&(y_inc, x_inc)) {
        return;
    }

    let ch = rows[y as usize][x as usize];
    visited[y as usize][x as usize].insert((y_inc, x_inc));
    if ch == '.' {
        walk(rows, visited, y + y_inc, x + x_inc, y_inc, x_inc);
    } else if ch == '/' {
        if y_inc == 0 {
            if x_inc == 1 {
                walk(rows, visited, y - 1, x, -1, 0);
            } else {
                walk(rows, visited, y + 1, x, 1, 0);
            }
        }
        if x_inc == 0 {
            if y_inc == 1 {
                walk(rows, visited, y, x - 1, 0, -1);
            } else {
                walk(rows, visited, y, x + 1, 0, 1);
            }
        }
    } else if ch == '\\' {
        if y_inc == 0 {
            if x_inc == 1 {
                walk(rows, visited, y + 1, x, 1, 0);
            } else {
                walk(rows, visited, y - 1, x, -1, 0);
            }
        }
        if x_inc == 0 {
            if y_inc == 1 {
                walk(rows, visited, y, x + 1, 0, 1);
            } else {
                walk(rows, visited, y, x - 1, 0, -1);
            }
        }
    } else if ch == '|' {
        if y_inc == 0 {
            walk(rows, visited, y + 1, x, 1, 0);
            walk(rows, visited, y - 1, x, -1, 0);
        } else {
            walk(rows, visited, y + y_inc, x + x_inc, y_inc, x_inc);
        }
    } else if ch == '-' {
        if x_inc == 0 {
            walk(rows, visited, y, x - 1, 0, -1);
            walk(rows, visited, y, x + 1, 0, 1);
        } else {
            walk(rows, visited, y + y_inc, x + x_inc, y_inc, x_inc);
        }
    }
}

fn parse(input: &String) -> Vec<Vec<char>> {
    let mut rows = vec![];

    input
        .lines()
        .for_each(|line| rows.push(line.chars().collect::<Vec<char>>()));

    rows
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
fn solve_2023_16() {
    let contents = fs::read_to_string("./resources/twenty_three/day_16.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_16 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_16 P2 {p2_res}");
}
