use crate::utils::grid::bound_check;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}

pub fn aoc_p1(contents: String, iterations: usize, y_len: usize, x_len: usize) -> usize {
    let mut input: Vec<Vec<usize>> = contents
        .split("\n")
        .map(|line| line.split(",").map(|elem| {
            elem.parse::<usize>().unwrap()
        }).collect())
        .collect();

    let mut rows: Vec<Vec<bool>> = Vec::with_capacity(y_len);

    for y in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        for x in 0..x_len {
            row.push(false)
        }
        rows.push(row);
    }

    for iter in 0..iterations {
        let wrong = &input[iter];
        rows[wrong[0]][wrong[1]] = true;
    }

    // solve
    {
        let mut heap: BinaryHeap<Position> = BinaryHeap::new();
        heap.push(Position{cost: 0, x: 0, y: 0});

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

            if x as usize == (x_len - 1)  && y as usize == y_len -1 {
                return cost;
            }

            for x_inc in [-1, 1] {
                heap.push(Position {
                    cost: cost + 1,
                    x: x + x_inc ,
                    y
                })
            }


            for y_inc in [-1, 1] {
                heap.push(Position {
                    cost: cost + 1,
                    x ,
                    y: y + y_inc
                })
            }

            visited[y as usize][x as usize] = true;
        }
    }
    return 0;
}

pub fn aoc_p2(contents: String, iterations: usize, y_len: usize, x_len: usize) {}


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

#[test]
fn example() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    assert_eq!(22, aoc_p1(contents.to_owned(), 12, 7, 7));
    aoc_p2(contents.to_owned(), 12, 7, 7);
}

#[test]
fn solve_() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/day_25.txt")
        .expect("This is the error message");

    let p1_res = aoc_p1(contents.to_owned(), 1024, 71, 71);
    println!("RES P1 {p1_res}");
    for i in 0..3000 {
        let iterations = 1024 + i;
        let p2 = aoc_p1(contents.to_owned(), iterations, 71, 71);
        if p2 == 0 {
            println!("RES P1 {p1_res} {iterations}");
            break
        }
    }
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
