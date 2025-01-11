use crate::four::day_10::bound_check;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}

pub fn aoc_p1(contents: String, at_least: usize) -> usize {
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    let y_len = input.len();
    let mut x_len = input[0].len();

    let mut walls: Vec<Vec<bool>> = Vec::with_capacity(y_len);

    let mut x_start = 0;
    let mut y_start = 0;
    let mut x_end = 0;
    let mut y_end = 0;

    for y in 0..y_len {
        let mut line = input[y];
        let mut row = vec![];
        let chars: Vec<char> = line.chars().collect();
        for x in 0..x_len {
            let ch = chars[x];
            if ch == '#' {
                row.push(true);
            } else {
                row.push(false);
            }

            if ch == 'S' {
                x_start = x;
                y_start = y;
            }
            if ch == 'E' {
                x_end = x;
                y_end = y;
            }
        }
        walls.push(row);
    }

    let mut cost: Vec<Vec<isize>> = Vec::with_capacity(y_len);
    setup_isize(y_len, x_len, &mut cost);

    solve_run(&walls, &mut cost, y_len, x_len, y_start, x_start);

    for y in 0..y_len {
        for x in 0..x_len {

            if walls[y][x] {
                continue;
            }

            let source = cost[y][x];

            for x_inc in [-2, -1, 1, 2isize] {
                let x_target = x as isize + x_inc;
                if !bound_check(y as isize, x_target, y_len, x_len) {
                    continue;
                }

                let target = cost[y][x_target as usize];
                if walls[y][x_target as usize] {
                    continue
                }
                if target == -1 {
                    println!("PROBLEM");
                    continue;
                }

                let savings = source - target - x_inc.abs();
                if savings >= at_least as isize {
                    p1 += 1;
                }
            }

            for y_inc in [-2, -1, 1, 2] {
                let y_target = y as isize + y_inc;
                if !bound_check(y_target, x as isize, y_len, x_len) {
                    continue;
                }

                let target = cost[y_target as usize][x];
                if walls[y_target as usize][x] {
                    continue
                }
                if target == -1 {
                    println!("PROBLEM");
                    continue;
                }


                let savings = source - target - y_inc.abs();
                if savings >= at_least as isize {
                    p1 += 1;
                }
            }
        }
    }
    p1
}


pub fn aoc_p2(contents: String, at_least: usize) -> usize {
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    let y_len = input.len();
    let mut x_len = input[0].len();

    let mut walls: Vec<Vec<bool>> = Vec::with_capacity(y_len);

    let mut x_start = 0;
    let mut y_start = 0;
    let mut x_end = 0;
    let mut y_end = 0;

    for y in 0..y_len {
        let mut line = input[y];
        let mut row = vec![];
        let chars: Vec<char> = line.chars().collect();
        for x in 0..x_len {
            let ch = chars[x];
            if ch == '#' {
                row.push(true);
            } else {
                row.push(false);
            }

            if ch == 'S' {
                x_start = x;
                y_start = y;
            }
            if ch == 'E' {
                x_end = x;
                y_end = y;
            }
        }
        walls.push(row);
    }

    let mut cost: Vec<Vec<isize>> = Vec::with_capacity(y_len);
    setup_isize(y_len, x_len, &mut cost);

    solve_run(&walls, &mut cost, y_len, x_len, y_start, x_start);

    for y in 0..y_len {
        for x in 0..x_len {

            if walls[y][x] {
                continue;
            }

            let source = cost[y][x];

            for x_inc in -20..21isize {
                for y_inc in -20..21isize {

                    if x_inc.abs() + y_inc.abs() > 20 {
                        continue;
                    }

                    let y_target = y as isize + y_inc;
                    let x_target = x as isize + x_inc;
                    if !bound_check(y_target, x_target, y_len, x_len) {
                        continue;
                    }

                    let target = cost[y_target as usize][x_target as usize];
                    if walls[y_target as usize][x_target as usize] {
                        continue
                    }
                    if target == -1 {
                        println!("PROBLEM");
                        continue;
                    }

                    let savings = source - target - x_inc.abs() - y_inc.abs();
                    if savings >= at_least as isize {
                        p1 += 1;
                    }
                }
            }
        }
    }
    p1
}

pub fn solve_run(
    rows: &Vec<Vec<bool>>,
    costs: &mut Vec<Vec<isize>>,
    y_len: usize,
    x_len: usize,
    y_start: usize,
    x_start: usize,
) {
    // solve
    {
        let mut heap: BinaryHeap<Position> = BinaryHeap::new();
        heap.push(Position {
            cost: 0,
            x: x_start as isize,
            y: y_start as isize,
        });

        let mut visited: Vec<Vec<bool>> = Vec::with_capacity(y_len);
        setup(y_len, x_len, &mut visited);

        while let Some(Position { cost, x, y }) = heap.pop() {
            if !bound_check(y, x, y_len, x_len) {
                continue;
            }

            if visited[y as usize][x as usize] {
                continue;
            }

            if rows[y as usize][x as usize] {
                continue;
            }

            costs[y as usize][x as usize] = cost as isize;

            for x_inc in [-1, 1] {
                heap.push(Position {
                    cost: cost + 1,
                    x: x + x_inc,
                    y,
                })
            }

            for y_inc in [-1, 1] {
                heap.push(Position {
                    cost: cost + 1,
                    x,
                    y: y + y_inc,
                })
            }

            visited[y as usize][x as usize] = true;
        }
    }
}

#[test]
fn example() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    assert_eq!(16, aoc_p1(contents.to_owned(), 6));
    assert_eq!(14, aoc_p1(contents.to_owned(), 8));
    assert_eq!(10, aoc_p1(contents.to_owned(), 10));
    assert_eq!(8, aoc_p1(contents.to_owned(), 12));
    assert_eq!(5, aoc_p1(contents.to_owned(), 20));
    assert_eq!(2, aoc_p1(contents.to_owned(), 40));

    assert_eq!(41 + 14 + 12 + 19 + 20 + 23 + 25 + 39, aoc_p2(contents.to_owned(), 56));
    assert_eq!(41 + 14 + 12 + 19 + 20 + 23 + 25, aoc_p2(contents.to_owned(), 58));
    assert_eq!(41 + 14 + 12 + 19 + 20 + 23, aoc_p2(contents.to_owned(), 60));
    assert_eq!(41 + 14 + 12 + 19 + 20, aoc_p2(contents.to_owned(), 62));
    assert_eq!(41 + 14 + 12 + 19, aoc_p2(contents.to_owned(), 64));
    assert_eq!(41 + 14 + 12, aoc_p2(contents.to_owned(), 66));
    assert_eq!(41 + 14, aoc_p2(contents.to_owned(), 68));
    assert_eq!(41, aoc_p2(contents.to_owned(), 70));
    assert_eq!(29, aoc_p2(contents.to_owned(), 72));
    assert_eq!(7, aoc_p2(contents.to_owned(), 74));
    assert_eq!(3, aoc_p2(contents.to_owned(), 76));
}

#[test]
fn solve_() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/input")
        .expect("This is the error message");

    let p1_res = aoc_p1(contents.to_owned(), 100);
    println!("RES P1 {p1_res}");

    let p2_res = aoc_p2(contents.to_owned(), 100);
    println!("RES P2 {p2_res}");
}

fn setup(y_len: usize, x_len: usize, final_pos: &mut Vec<Vec<bool>>) {
    for i in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        for j in 0..x_len {
            row.push(false);
        }
        final_pos.push(row);
    }
}

fn setup_isize(y_len: usize, x_len: usize, final_pos: &mut Vec<Vec<isize>>) {
    for i in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        for j in 0..x_len {
            row.push(-1);
        }
        final_pos.push(row);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position {
    cost: usize,
    x: isize,
    y: isize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.x.cmp(&self.x))
            .then_with(|| other.y.cmp(&self.y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
