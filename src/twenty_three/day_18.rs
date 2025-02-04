use crate::solve::Solve;
use std::cmp::{max, min};
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let rows = parse(input);

        let mut y_len = 0usize;
        let mut x_len = 0usize;

        let mut y_min = 0isize;
        let mut x_min = 0isize;

        let mut y_c = 0isize;
        let mut x_c = 0isize;
        for r in &rows {
            if r.0 == 'U' {
                y_c = y_c - r.1 as isize;
                y_min = min(y_min as isize, y_c - 1);
            }
            if r.0 == 'D' {
                y_c += r.1 as isize;
                y_len = max(y_len as isize, y_c + 1) as usize;
            }
            if r.0 == 'L' {
                x_c -= r.1 as isize;
                x_min = min(x_min, x_c - 1);
            }
            if r.0 == 'R' {
                x_c += r.1 as isize;
                x_len = max(x_c + 1, x_len as isize) as usize;
            }
        }

        y_len += y_min.abs() as usize;
        x_len += x_min.abs() as usize;

        let mut visited: Vec<Vec<bool>> = vec![vec![false; x_len]; y_len];
        {
            let mut x = x_min.abs() as usize - 1;
            let mut y = y_min.abs() as usize - 1;
            let mut row = &rows[0];
            let mut steps = row.1;
            let mut i = 1;
            while steps > 0 {
                visited[y][x] = true;
                let dir = get_dir(row.0);
                y = (y as isize + dir.0) as usize;
                x = (x as isize + dir.1) as usize;

                steps -= 1;
                if steps == 0 && i < rows.len() {
                    row = &rows[i];
                    steps = row.1;
                    i += 1;
                }
            }
        }

        {
            let mut y_s = 0;
            let mut x_s = 0;
            'out: for y in 0..y_len {
                for x in 0..x_len {
                    let mut good = true;
                    if y < 1 {
                        continue;
                    }
                    if y as isize - 2 >= 0 {
                        good &= !visited[y - 2][x];
                    }
                    good &= visited[y - 1][x];
                    good &= !visited[y][x];

                    if good {
                        y_s = y;
                        x_s = x;
                        break 'out;
                    }
                }
            }

            let mut stack: Vec<(usize, usize)> = vec![];
            stack.push((y_s, x_s));

            while let Some((y, x)) = stack.pop() {
                if visited[y][x] {
                    continue;
                }

                for x_inc in [-1isize, 1] {
                    let res = x as isize + x_inc;
                    if res < 0 || res >= x_len as isize {
                        continue;
                    }
                    stack.push((y, res as usize));
                }

                for y_inc in [-1isize, 1] {
                    let res = y as isize + y_inc;
                    if res < 0 || res >= y_len as isize {
                        continue;
                    }
                    stack.push((res as usize, x));
                }
                visited[y][x] = true
            }
        }
        println!("{:?}", visited);
        weight(&visited).to_string()
    }

    fn p2(input: &String) -> String {
        let mut rows = parse_p2(input);

        let mut x_start = 0;
        let mut y_start = 0;

        let mut p2 = 0;
        for r in &rows {
            let dir = get_dir(r.0);
            let y_next = y_start + (dir.0 * r.1 as isize);
            let x_next = x_start + (dir.1 * r.1 as isize);
            let d = x_start * y_next - y_start * x_next;
            p2 += d;
            p2 += r.1 as isize;
            println!("{}", r.1);
            y_start = y_next;
            x_start = x_next;
        }

        ((p2 / 2) + 1).to_string()
    }
}

fn weight(rows: &Vec<Vec<bool>>) -> usize {
    let y_len = rows.len();
    let mut res = 0;
    for i in 0..y_len {
        let row = &rows[i];
        for ch in row {
            if *ch {
                res += 1;
            }
        }
    }

    res
}

fn get_dir(ch: char) -> (isize, isize) {
    match ch {
        'U' | '3' => (-1, 0),
        'D' | '1' => (1, 0),
        'L' | '2' => (0, -1),
        'R' | '0' => (0, 1),
        _ => {
            println!("err");
            return (-10000000000, 0);
        }
    }
}

fn parse(input: &String) -> Vec<(char, usize, &str)> {
    let mut rows: Vec<(char, usize, &str)> = vec![];
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line
            .split_whitespace()
            .map(|elem| elem.trim_end().trim_start())
            .collect();
        rows.push((
            parts[0].parse::<char>().unwrap(),
            parts[1].parse::<usize>().unwrap(),
            parts[2],
        ))
    });
    rows
}

fn parse_p2(input: &String) -> Vec<(char, usize)> {
    let mut rows: Vec<(char, usize)> = vec![];
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line
            .split_whitespace()
            .map(|elem| elem.trim_end().trim_start())
            .collect();
        let upp = parts[2]
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .to_uppercase();
        let part = upp.split_at(5);
        rows.push((
            part.1.parse::<char>().unwrap(),
            usize::from_str_radix(&*part.0, 16).unwrap(),
        ))
    });
    rows
}



#[test]
fn solve_2023_18() {
    let contents = fs::read_to_string("./resources/twenty_three/day_18.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_18 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_18 {p2_res}");
}
