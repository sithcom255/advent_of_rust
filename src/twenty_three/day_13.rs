use crate::solve::Solve;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let inputs = parse(input);
        let mut p1 = 0;
        for rows in inputs {
            let y_len = rows.len();
            let x_len = rows[0].len();
            for i in 1..x_len {
                if is_vertically_symmetrical(&rows, i as isize) {
                    p1 += i;
                    continue
                };
            }
            for i in 1..y_len {
                if is_horizontally_symmetrical(&rows, i as isize) {
                    p1 += i * 100;
                    continue
                };
            }
        }
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let mut inputs = parse(input);
        let mut p1 = 0isize;
        'outer: for rows in inputs.iter_mut() {
            let y_len = rows.len();
            let x_len = rows[0].len();

            let start = Self::eval( &rows, y_len, x_len, 0);

            for y in 0..y_len {
                for x in 0..x_len {
                    rows[y][x] = !rows[y][x];

                    let res = Self::eval(&rows, y_len, x_len, start);
                    if res != 0 && res != start {
                        p1 += res;
                        continue 'outer;
                    }

                    rows[y][x] = !rows[y][x];
                }
            }
        }
        p1.to_string()
    }
}

impl Solver {
    fn eval(rows: &&mut Vec<Vec<bool>>, y_len: usize, x_len: usize, ignore: isize) -> isize {

        for i in 1..x_len {
            if is_vertically_symmetrical(&rows, i as isize) {
                if ignore != i  as isize {
                    return i as isize;
                }
            };
        }
        for i in 1..y_len {
            if is_horizontally_symmetrical(&rows, i as isize) {
                if ignore != i  as isize  * 100{
                    return i as isize * 100;
                }
            };
        }
        return 0;
    }
}

fn is_vertically_symmetrical(rows: &Vec<Vec<bool>>, i: isize) -> bool {
    let mut inc = 0isize;
    let y_len = rows.len() as isize;
    let x_len = rows[0].len() as isize;
    while i - 1 - inc >= 0 && i + inc < x_len {
        let left = (i - 1 - inc) as usize;
        let right = (i + inc) as usize;
        for y in 0..y_len as usize {
            let row = &rows[y];
            if row[left] != row[right] {
                return false;
            }
        }
        inc += 1;
    }
    true
}

fn is_horizontally_symmetrical(rows: &Vec<Vec<bool>>, i: isize) -> bool {
    let mut inc = 0;
    let y_len = rows.len() as isize;
    let x_len = rows[0].len() as isize;
    while i - 1 - inc >= 0 && i + inc < y_len {
        let left = i - 1 - inc;
        let right = i + inc;

        let l_row = &rows[left as usize];
        let r_row = &rows[right as usize];
        for x in 0..x_len as usize {
            if l_row[x] != r_row[x] {
                return false;
            }
        }
        inc += 1;
    }
    true
}

fn parse(input: &String) -> Vec<Vec<Vec<bool>>> {
    let mut out: Vec<Vec<Vec<bool>>> = vec![];
    let mut rows = vec![];
    input.lines().for_each(|line| {
        if line.is_empty() {
            out.push(rows.clone());
            rows = vec![];
            return;
        }
        rows.push(
            line.chars()
                .into_iter()
                .map(|elem| {
                    return elem == '#';
                })
                .collect(),
        );
    });
    out.push(rows.clone());
    out
}

#[test]
fn solve_2023_13() {
    let contents = fs::read_to_string("./resources/twenty_three/day_13.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_13 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_13 {p2_res}");
}
