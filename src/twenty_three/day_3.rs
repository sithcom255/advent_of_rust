use crate::solve::Solve;
use crate::twenty_four::day_10::bound_check;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let rows = parse(input);
        let mut p1 = 0;
        let y_len = rows.len();
        for y in 0..y_len {
            let row = &rows[y];
            let mut number = 0;
            let mut counting = false;
            let mut has_symbol = false;
            let x_len = row.len();
            for x in 0..x_len {
                let ch = row[x];
                if counting {
                    if ch.is_digit(10) {
                        number = number * 10 + ch.to_digit(10).unwrap();
                        has_symbol |= has_symbol_neighbour(&rows, y_len, x_len, y, x);
                    } else {
                        if has_symbol {
                            p1 += number;
                        }
                        number = 0;
                        counting = false;
                        has_symbol = false;
                    }
                } else {
                    if ch.is_digit(10) {
                        number = ch.to_digit(10).unwrap();
                        has_symbol |= has_symbol_neighbour(&rows, y_len, x_len, y, x);
                        counting = true;
                    }
                }
            }
            if has_symbol {
                p1 += number;
            }
        }
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let rows = parse(input);
        let mut p2 = 0;
        let y_len = rows.len();
        for y in 0..y_len {
            let row = &rows[y];
            let x_len = row.len();
            for x in 0..x_len {
                let ch = row[x];
                if ch == '*' {
                    p2+= has_two_adjacent(&rows, y_len, x_len, y, x);
                }
            }
        }
        p2.to_string()
    }
}

fn has_two_adjacent(
    rows: &Vec<Vec<char>>,
    y_len: usize,
    x_len: usize,
    y: usize,
    x: usize,
) -> usize {
    let mut visited = vec![vec![false; x_len]; y_len];
    let mut numbers = vec![];

    for y_inc in [-1isize, 0, 1] {
        for x_inc in [-1isize, 0, 1] {
            let y_t = y as isize + y_inc;
            let x_t = x as isize + x_inc;
            if bound_check(y_t, x_t, y_len, x_len) {
                let ch = rows[y_t as usize][x_t as usize];
                if ch.is_digit(10) && !visited[y_t as usize][x_t as usize] {

                    let row = &rows[y_t as usize];
                    let mut start = x_t ;

                    while start >= 0 && row[start as usize].is_digit(10) {
                        start -= 1;
                    }

                    let mut number = 0;
                    for i in (start + 1) as usize..row.len() {
                        if !row[i].is_digit(10) {
                            break;
                        }
                        visited[y_t as usize][i] = true;
                        number = number * 10 + row[i].to_digit(10).unwrap();
                    }
                    numbers.push(number);
                };
            }
        }
    }
    if numbers.len() == 2 {
        return numbers[0] as usize * numbers[1] as usize;
    }
    0
}

fn has_symbol_neighbour(
    rows: &Vec<Vec<char>>,
    y_len: usize,
    x_len: usize,
    y: usize,
    x: usize,
) -> bool {
    for y_inc in [-1isize, 0, 1] {
        for x_inc in [-1isize, 0, 1] {
            let y_t = y as isize + y_inc;
            let x_t = x as isize + x_inc;
            if bound_check(y_t, x_t, y_len, x_len) {
                if !rows[y_t as usize][x_t as usize].is_digit(10)
                    && rows[y_t as usize][x_t as usize] != '.'
                {
                    return true;
                }
            }
        }
    }
    false
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut rows = vec![];
    for line in input.split("\n") {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch)
        }
        rows.push(row);
    }
    rows
}

#[test]
fn solve_2023_3() {
    let contents = fs::read_to_string("./resources/twenty_three/day_3.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_3 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_3 P2 {p2_res}");
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