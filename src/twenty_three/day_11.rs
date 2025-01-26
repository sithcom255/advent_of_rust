use crate::solve::Solve;
use std::collections::HashMap;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (galaxies, empty_row, empty_col, y_len, x_len) = parse(input);
        let mut distances = 0;

        for i in 0..galaxies.len() {
            let galaxy = galaxies[i];
            for j in i..galaxies.len() {
                let other = galaxies[j];
                distances +=
                    to_distance(galaxy, other, y_len, x_len, &empty_row, &empty_col, 1);
            }
        }

        distances.to_string()
    }

    fn p2(input: &String) -> String {
        let (galaxies, empty_row, empty_col, y_len, x_len) = parse(input);
        let mut distances = 0;

        for i in 0..galaxies.len() {
            let galaxy = galaxies[i];
            for j in i..galaxies.len() {
                let other = galaxies[j];
                distances +=
                    to_distance(galaxy, other, y_len, x_len, &empty_row, &empty_col, 999_999);
            }
        }

        distances.to_string()
    }
}

fn parse(input: &String) -> (Vec<usize>, Vec<usize>, Vec<usize>, usize, usize) {
    let mut galaxies: Vec<usize> = vec![];
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    let rows: Vec<&str> = input.lines().collect();

    let mut y_len = rows.len();
    let mut x_len = rows[1].len();

    let mut empty_lines: HashMap<usize, bool> = HashMap::new();

    let mut empty_row_counter = 0;
    for y in 0..rows.len() {
        let line = rows[y];
        if !line.contains("#") {
            empty_row_counter += 1;
        }
        empty_rows.push(empty_row_counter);
        let row: Vec<char> = line.chars().collect();
        for x in 0..row.len() {
            if row[x] == '#' {
                galaxies.push(y * x_len + x);
                empty_lines.insert(x, false);
            }
        }
    }

    let mut counter = 0;
    for i in 0..x_len {
        if !empty_lines.contains_key(&i) {
            counter += 1;
        }
        empty_cols.push(counter);
    }

    (galaxies, empty_rows, empty_cols, y_len, x_len)
}

fn to_distance(
    a: usize,
    b: usize,
    y_len: usize,
    x_len: usize,
    rows: &Vec<usize>,
    cols: &Vec<usize>,
    time: isize,
) -> usize {
    let a_y = (a as isize / x_len as isize);
    let b_y = (b as isize / x_len as isize);
    let empty_rows = (rows[a_y as usize] as isize - rows[b_y as usize] as isize).abs() * time;
    let y = (a_y - b_y).abs();
    let a_x = (a as isize % x_len as isize);
    let b_x = (b as isize % x_len as isize);
    let empty_cols = (cols[a_x as usize] as isize - cols[b_x as usize] as isize).abs() * time;

    let x = (a_x - b_x).abs();
    (y + x + empty_cols + empty_rows) as usize
}

#[test]
fn solve_2023_11() {
    let contents = fs::read_to_string("./resources/twenty_three/day_11.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_11 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_11 P2 {p2_res}");
}
