use crate::solve::Solve;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut p1 = 0;
        let rows = parse(input);

        (dijkstra(&rows) - rows[0][0]).to_string()
    }

    fn p2(input: &String) -> String {

        let mut p1 = 0;
        let rows = parse(input);

        (dijkstra_p2(&rows) - rows[0][0]).to_string()
    }
}

fn dijkstra(rows: &Vec<Vec<usize>>) -> usize {
    let y_len = rows.len();
    let x_len = rows[0].len();

    let mut visited: Vec<Vec<Vec<HashSet<(isize, isize)>>>> = vec![vec![vec![HashSet::new(); 4]; x_len]; y_len];
    let mut queue: BinaryHeap<Point> = BinaryHeap::new();

    queue.push(Point {
        value: 0,
        cord: (-1, 0),
        dir: (1, 0),
        steps: -1,
    });

    while let Some(Point {
        value,
        cord,
        dir: original_dir,
        steps,
    }) = queue.pop()
    {
        if cord.0 == y_len as isize - 1 && cord.1 == x_len as isize - 1 {
            return value;
        }

        if !(cord.0 == -1 && cord.1 == 0) && visited[cord.0 as usize][cord.1 as usize][steps as usize].contains(&original_dir) {
            continue;
        }

        let dirs = get_dirs(original_dir.0, original_dir.1, steps);
        for d in dirs {
            let y_t = cord.0 + d.0;
            let x_t = cord.1 + d.1;

            if x_t < 0 || x_t >= x_len as isize  || y_t < 0 || y_t >= y_len as isize {
                continue;
            }
            let add_value = rows[y_t as usize][x_t as usize];

            if d == original_dir {
                queue.push(Point {
                    value: value + add_value,
                    cord: (y_t, x_t),
                    dir: d,
                    steps: steps + 1,
                })
            } else {
                queue.push(Point {
                    value: value + add_value,
                    cord: (y_t, x_t),
                    dir: d,
                    steps: 1,
                })
            }
        }
        if !(cord.0 == -1 && cord.1 == 0)  {
            visited[cord.0 as usize][cord.1 as usize][steps as usize].insert(original_dir);
        }
    }

    0
}

fn dijkstra_p2(rows: &Vec<Vec<usize>>) -> usize {
    let y_len = rows.len();
    let x_len = rows[0].len();

    let mut visited: Vec<Vec<Vec<HashSet<(isize, isize)>>>> = vec![vec![vec![HashSet::new(); 11]; x_len]; y_len];
    let mut queue: BinaryHeap<Point> = BinaryHeap::new();

    queue.push(Point {
        value: 0,
        cord: (-1, 0),
        dir: (1, 0),
        steps: 4,
    });

    while let Some(Point {
                       value,
                       cord,
                       dir: original_dir,
                       steps,
                   }) = queue.pop()
    {
        if cord.0 == y_len as isize - 1 && cord.1 == x_len as isize - 1 && steps >= 4 {
            return value;
        }

        if !(cord.0 == -1 && cord.1 == 0) && visited[cord.0 as usize][cord.1 as usize][steps as usize].contains(&original_dir) {
            continue;
        }

        let dirs = get_dirs_2(original_dir.0, original_dir.1, steps);
        for d in dirs {
            let y_t = cord.0 + d.0;
            let x_t = cord.1 + d.1;

            if x_t < 0 || x_t >= x_len as isize  || y_t < 0 || y_t >= y_len as isize {
                continue;
            }
            let add_value = rows[y_t as usize][x_t as usize];

            if d == original_dir {
                queue.push(Point {
                    value: value + add_value,
                    cord: (y_t, x_t),
                    dir: d,
                    steps: steps + 1,
                })
            } else {
                queue.push(Point {
                    value: value + add_value,
                    cord: (y_t, x_t),
                    dir: d,
                    steps: 1,
                })
            }
        }
        if !(cord.0 == -1 && cord.1 == 0)  {
            visited[cord.0 as usize][cord.1 as usize][steps as usize].insert(original_dir);
        }
    }

    0
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    value: usize,
    cord: (isize, isize),
    dir: (isize, isize),
    steps: isize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.value.cmp(&self.value)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_dirs(y_inc: isize, x_inx: isize, steps: isize) -> Vec<(isize, isize)> {
    let mut dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let revers = (-y_inc, -x_inx);

    for i in 0..dirs.len() {
        let c = dirs[i];
        if c == revers {
            dirs.remove(i);
            break;
        }
    }

    if steps == 3 {
        for i in 0..dirs.len() {
            let c = dirs[i];
            if c == (y_inc, x_inx) {
                dirs.remove(i);
                break;
            }
        }
    }

    dirs
}

fn get_dirs_2(y_inc: isize, x_inx: isize, steps: isize) -> Vec<(isize, isize)> {
    if steps < 4 {
        return vec![(y_inc, x_inx)];
    }

    let mut dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let revers = (-y_inc, -x_inx);

    for i in 0..dirs.len() {
        let c = dirs[i];
        if c == revers {
            dirs.remove(i);
            break;
        }
    }

    if steps == 10 {
        for i in 0..dirs.len() {
            let c = dirs[i];
            if c == (y_inc, x_inx) {
                dirs.remove(i);
                break;
            }
        }
    }

    dirs
}

fn parse(input: &String) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = vec![];
    input.lines().for_each(|line| {
        let row: Vec<usize> = line
            .chars()
            .map(|ch| ch.to_string().parse::<usize>().unwrap())
            .collect();
        rows.push(row);
    });

    rows
}

#[test]
fn solve_2023_17() {
    let contents = fs::read_to_string("./resources/twenty_three/day_17.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_17 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_17 P2 {p2_res}");
}
