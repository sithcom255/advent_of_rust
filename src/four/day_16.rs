use regex::Regex;
use std::collections::HashSet;
use std::fs;

use crate::four::day_10::bound_check;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn advent_of_rust(file_path: &str) -> [usize; 2] {
    let mut p1_res = 85440;
    let mut p2_res: usize = 0;

    let regex = Regex::new(r"p=([\d]+),([\d]+) v=(-?[\d]+),(-?[\d]+)").unwrap();

    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut rows: Vec<Vec<char>> = contents
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let y_len = rows.len();
    let x_len = rows[0].len();

    let mut y_t = 0;
    let mut x_t = 0;

    {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let mut walls: Vec<Vec<bool>> = Vec::with_capacity(y_len);
        setup(y_len, x_len, &mut walls);

        let mut visited: Vec<Vec<Vec<bool>>> = Vec::with_capacity(y_len);
        setup_bool_visited(y_len, x_len, &mut visited);

        let mut prices: Vec<Vec<Vec<usize>>> = Vec::with_capacity(y_len);
        setup_usize(y_len, x_len, &mut prices);

        for y in 0..y_len {
            let row = &rows[y];
            for x in 0..x_len {
                let ch = row[x];
                walls[y][x] = ch == '#';
                if ch == 'S' {
                    heap.push(State {
                        cost: 0,
                        x: x as isize,
                        y: y as isize,
                        dir: 1,
                    })
                }
                if ch == 'E' {
                    y_t = y;
                    x_t = x;
                }
            }
        }

        while let Some(State { cost, x, y, dir }) = heap.pop() {
            if !bound_check(y, x, y_len, x_len) {
                continue;
            }

            if x as usize == x_t && y as usize == y_t {
                p1_res = cost;
                prices[y as usize][x as usize][dir] = cost;
                break;
            }

            if visited[y as usize][x as usize][dir] {
                if cost >= prices[y as usize][x as usize][dir] {
                    continue;
                }
            }

            if walls[y as usize][x as usize] {
                continue;
            }

            for i in 0..4 {
                if is_opposite(dir, i) {
                    continue;
                }

                let mut new_cost = cost + 1;

                if i != dir {
                    new_cost += 999;
                    heap.push(State {
                        cost: new_cost,
                        x: x as isize,
                        y: y as isize,
                        dir: i,
                    })
                } else {
                    heap.push(State {
                        cost: new_cost,
                        x: x as isize + get_x(i),
                        y: y as isize + get_y(i),
                        dir: i,
                    })
                }
            }

            visited[y as usize][x as usize][dir] = true;
            prices[y as usize][x as usize][dir] = cost;
        }

        {
            // println!("{:?}", prices);
            let mut vis_p2: Vec<Vec<Vec<bool>>> = Vec::with_capacity(y_len);
            setup_bool_visited(y_len, x_len, &mut vis_p2);
            p2_res = walk_prices(&walls, &mut vis_p2, &prices, y_len, x_len, y_t, x_t, 0);
        }
    }

    [p1_res, p2_res]
}

pub fn setup_usize(y_len: usize, x_len: usize, final_pos: &mut Vec<Vec<Vec<usize>>>) {
    for i in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        for j in 0..x_len {
            row.push(vec![0, 0, 0, 0]);
        }
        final_pos.push(row);
    }
}

fn walk_prices(
    wall: &Vec<Vec<bool>>,
    visited: &mut Vec<Vec<Vec<bool>>>,
    prices: &Vec<Vec<Vec<usize>>>,
    y_len: usize,
    x_len: usize,
    y_s: usize,
    x_s: usize,
    original_dir: usize,
) -> usize {
    let mut todo: Vec<[usize; 3]> = Vec::new();
    let mut result: HashSet<[usize; 3]> = HashSet::new();

    todo.push([y_s, x_s, original_dir]);
    while !todo.is_empty() {
        let option = todo.pop().unwrap();
        let y = option[0];
        let x = option[1];
        let that_dir = option[2];

        if wall[y][x] {
            continue;
        }

        let mut todo_dirs = Vec::new();

        for dir in 0..4 {
            if dir == that_dir {
                todo_dirs.push(that_dir);
            };
            let original_price = prices[y][x][that_dir];
            let before_rot = prices[y][x][dir];
            if original_price as isize - before_rot as isize == 1000 {
                todo_dirs.push(dir);
            }
        }


        for dir in todo_dirs {
            let price = prices[y][x][dir] as isize;

            let d = get_reversed_dir(dir) as usize;

            let x_inc = get_x(d);

            if x_inc != 0 {
                let x_cand = x as isize + x_inc;

                if !bound_check(y as isize, x_cand, y_len, x_len) {
                    continue;
                }

                if wall[y][x_cand as usize] {
                    continue;
                }

                let cand = prices[y][x_cand as usize][dir] as isize;

                if price - cand == 1 {
                    todo.push([y, x_cand as usize, dir]);
                }
            }

            let y_inc = get_y(d);
            if y_inc != 0 {
                let y_cand = y as isize + y_inc;

                if !bound_check(y_cand, x as isize, y_len, x_len) {
                    continue;
                }

                if wall[y_cand as usize][x] {
                    continue;
                }

                let cand = prices[y_cand as usize][x][dir] as isize;

                if price - cand == 1 {
                    todo.push([y_cand as usize, x, dir]);
                }
            }
            visited[y][x][original_dir] = true;
        }
        result.insert(option);
    }

    {
        let mut vec = Vec::new();
        setup(y_len, x_len, &mut vec);
        result.iter().for_each(|x| vec[x[0]][x[1]] = true);
        print(&vec);
    }

    result.len()
}

fn get_x(dir: usize) -> isize {
    if dir == 1 {
        return 1;
    }
    if dir == 3 {
        return -1;
    }
    0
}

fn get_reversed_dir(dir: usize) -> isize {
    if dir == 0 {
        return 2;
    }
    if dir == 1 {
        return 3;
    }

    if dir == 2 {
        return 0;
    }

    if dir == 3 {
        return 1;
    }
    0
}

fn get_y(dir: usize) -> isize {
    if dir == 0 {
        return -1;
    }
    if dir == 2 {
        return 1;
    }
    0
}

fn is_opposite(dir: usize, i: usize) -> bool {
    dir == 0 && i == 2 || dir == 1 && i == 3 || dir == 2 && i == 0 || dir == 3 && i == 1
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

fn setup_bool_visited(y_len: usize, x_len: usize, final_pos: &mut Vec<Vec<Vec<bool>>>) {
    for i in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        for j in 0..x_len {
            row.push(vec![false, false, false, false]);
        }
        final_pos.push(row);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: isize,
    y: isize,
    dir: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.x.cmp(&self.x))
            .then_with(|| other.y.cmp(&self.y))
            .then_with(|| other.dir.cmp(&self.dir))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn print(rows: &Vec<Vec<bool>>) -> bool {
    let mut print = true;
    let mut str: Vec<String> = Vec::with_capacity(110);
    for i in 0..rows.len() {
        let row = &rows[i];
        let mut s = "".to_owned();
        for j in 0..row.len() {
            if !row[j] {
                s.push_str(".")
            } else {
                s.push_str("#")
            }
        }
        str.push(s);
    }
    if print {
        str.iter().for_each(|line| println!("{line}"))
    }
    return print;
}
