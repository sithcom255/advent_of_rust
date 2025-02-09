use crate::solve::Solve;
use crate::utils::grid::bound_check;
use std::fs;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (y_start, x_start, rows) = parse(input);

        let y_len = rows.len();
        let x_len = rows[0].len();
        let result = Self::after_steps(y_start, x_start, &rows, y_len, x_len, 64);

        result.to_string()
    }

    fn p2(input: &String) -> String {
        let (y_start, x_start, rows) = parse(input);

        let mut n = 26501365;

        let y_len = rows.len();
        let x_len = rows[0].len();
        let same = Self::after_steps(y_start, x_start, &rows, y_len, x_len, 26501365 % 1000);
        let other = Self::after_steps(y_start, x_start, &rows, y_len, x_len, 26501365 % 999);

        let remaining = (n - 66) % 131;
        let remaining_edge = remaining + 131 - 66;
        let remaining_edge_small = remaining - 66;
        let blocks = (n - 66) / 131 as i64;

        let mut blocks_val =
            ((blocks / 2) * same) + ((blocks / 2) * other) + ((blocks % 2) * other);

        let mut cur = blocks_val - if blocks % 2 == 0 { same } else { other };
        let mut block_alternative = cur;

        for i in 0..blocks - 1 {
            let value = (blocks - 1 - i);
            let rm = if value % 2 == 0 { same } else { other };
            cur -= rm;
            if cur < 0 {
                break;
            }
            block_alternative += cur;
        }

        let horizontal = (blocks_val * 2) + same;
        let vertical = blocks_val * 2;

        let total = horizontal + vertical + (block_alternative * 4);

        let mut corners = Self::after_steps(65, 0, &rows, y_len, x_len, remaining as usize);
        corners += Self::after_steps(65, 130, &rows, y_len, x_len, remaining as usize);
        corners += Self::after_steps(0, 65, &rows, y_len, x_len, remaining as usize);
        corners += Self::after_steps(130, 65, &rows, y_len, x_len, remaining as usize);

        let mut edges =
            Self::after_steps(130, 0, &rows, y_len, x_len, remaining_edge as usize) * (blocks);
        edges += Self::after_steps(0, 130, &rows, y_len, x_len, remaining_edge as usize) * (blocks);
        edges += Self::after_steps(130, 130, &rows, y_len, x_len, remaining_edge as usize) * (blocks);
        edges += Self::after_steps(0, 0, &rows, y_len, x_len, remaining_edge as usize) * (blocks);

        edges += Self::after_steps(130, 0, &rows, y_len, x_len, remaining_edge_small as usize)
            * (blocks + 1);
        edges += Self::after_steps(0, 130, &rows, y_len, x_len, remaining_edge_small as usize)
            * (blocks + 1);
        edges += Self::after_steps(130, 130, &rows, y_len, x_len, remaining_edge_small as usize)
            * (blocks + 1);
        edges += Self::after_steps(0, 0, &rows, y_len, x_len, remaining_edge_small as usize)
            * (blocks + 1);

        (total + corners + edges).to_string()
    }
}

impl Solver {
    fn after_steps(
        y_start: usize,
        x_start: usize,
        rows: &Vec<Vec<bool>>,
        y_len: usize,
        x_len: usize,
        steps: usize,
    ) -> i64 {
        let mut current = vec![vec![false; x_len]; y_len];
        current[y_start][x_start] = true;

        for i in 0..steps {
            walk(&rows, &mut current, y_len, x_len);
        }

        let result = current
            .iter()
            .flat_map(|row| row.iter())
            .map(|elem| return if *elem { 1 } else { 0 })
            .fold(0, |res, next| {
                return res + next;
            });
        result
    }
}

fn walk(fields: &Vec<Vec<bool>>, current: &mut Vec<Vec<bool>>, y_len: usize, x_len: usize) {
    let mut next = vec![vec![false; x_len]; y_len];

    for y in 0..y_len {
        for x in 0..x_len {
            if !current[y][x] {
                continue;
            }

            for inc in [-1, 1] {
                if bound_check(y as isize + inc, x as isize, y_len, x_len)
                    && !fields[(y as isize + inc) as usize][x]
                {
                    next[(y as isize + inc) as usize][x] = true;
                }
                if bound_check(y as isize, x as isize + inc, y_len, x_len)
                    && !fields[y][(x as isize + inc) as usize]
                {
                    next[y][(x as isize + inc) as usize] = true;
                }
            }
        }
    }
    *current = next;
}

fn parse(input: &String) -> (usize, usize, Vec<Vec<bool>>) {
    let mut y_start = 0;
    let mut x_start = 0;
    let mut rows = vec![];

    input.lines().for_each(|line| {
        let mut row = vec![];
        line.chars().for_each(|ch| {
            if ch == 'S' {
                y_start = rows.len();
                x_start = row.len();
            }
            if ch == '#' {
                row.push(true);
            } else {
                row.push(false)
            }
        });
        rows.push(row);
    });
    (y_start, x_start, rows)
}



#[test]
fn solve_2023_21() {
    let contents = fs::read_to_string("./resources/twenty_three/day_21.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_21 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_21 P2 {p2_res}");
}
