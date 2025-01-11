use crate::solve::Solve;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut rows = input.split("\n");
        let mut l: Vec<i32> = Vec::with_capacity(1000);
        let mut r: Vec<i32> = Vec::with_capacity(1000);
        Self::populate_ds(&mut rows, &mut l, &mut r);

        merge_sort(&mut l);
        merge_sort(&mut r);

        let mut distance = 0;
        for i in 0..l.len() {
            let mut i1 = l[i] - r[i];
            if i1 < 0 {
                i1 = -i1;
            }
            distance += i1;
        }

        // l.iter().zip();
        distance.to_string()
    }

    fn p2(input: &String) -> String {
        let mut rows = input.split("\n");
        let mut l: Vec<i32> = Vec::with_capacity(1000);
        let mut r: Vec<i32> = Vec::with_capacity(1000);
        loop {
            match rows.next() {
                Some(line) => {
                    let mut split = line.split("   ");
                    l.push(split.next().expect("").parse().unwrap());
                    r.push(split.next().expect("").parse().unwrap());
                }
                None => {
                    break;
                }
            };
        }

        merge_sort(&mut l);
        merge_sort(&mut r);

        // p2
        let mut similarity = 0;
        for i in 0..l.len() {
            let n = l[i];
            let found = binary_search(&r, n, 0, r.len());
            match found {
                None => {}
                Some(i) => {
                    similarity += n;
                    let mut l_move = 1;
                    while i - l_move > 0 && r[i - l_move] == n {
                        l_move += 1;
                        similarity += n;
                    }
                    let mut r_move = 1;
                    while i + r_move < r.len() - 1 && r[i + r_move] == n {
                        r_move += 1;
                        similarity += n;
                    }
                }
            }
        }
        similarity.to_string()
    }
}

impl Solver {
    fn populate_ds(rows: &mut Split<&str>, l: &mut Vec<i32>, r: &mut Vec<i32>) {
        loop {
            match rows.next() {
                Some(line) => {
                    let mut split = line.split("   ");
                    l.push(split.next().expect("").parse().unwrap());
                    r.push(split.next().expect("").parse().unwrap());
                }
                None => {
                    break;
                }
            };
        }
    }
}

fn binary_search(numbers: &Vec<i32>, n: i32, l: usize, r: usize) -> Option<usize> {
    let i = (l + r) / 2;
    // 0 + 5 -> 2
    // 0 + 1 -> 0
    //
    if l == r {
        if r < numbers.len() && n == numbers[r] {
            Some(r);
        }
        return None;
    }
    let mid = numbers[i];
    if mid == n {
        Some(i)
    } else if mid < n {
        return binary_search(numbers, n, i + 1, r);
    } else {
        return binary_search(numbers, n, l, i);
    }
}

fn merge_sort(numbers: &mut Vec<i32>) {
    let size = numbers.len() as u32;
    let mut height: u32 = (size as u32).ilog2();
    if 2u32.pow(height) < size {
        height += 1;
    }

    let mut from: Vec<i32> = numbers.to_vec();
    for i in 0..height {
        let step: u32 = 2_u32.pow(i + 1);
        let compare: u32 = 2_u32.pow(i);
        let mut start = 0;

        if !(start + compare <= size) {
            continue;
        }

        let mut to: Vec<i32> = Vec::with_capacity(size as usize);
        for j in 0..size {
            to.push(from[j as usize]);
        }

        while start + compare <= size {
            let l_start: u32 = start;
            let l_end: u32 = start + compare;
            let r_start: u32 = start + compare;
            let r_end: u32 = std::cmp::min(start + step, size);
            sort_range(
                &from,
                &mut to,
                l_start as usize,
                l_end as usize,
                r_start as usize,
                r_end as usize,
            );
            start += step;
        }

        from = to;
    }

    for i in 0..size {
        numbers[i as usize] = from[i as usize];
    }
}

fn sort_range(
    from: &Vec<i32>,
    to: &mut Vec<i32>,
    l_start: usize,
    l_end: usize,
    r_start: usize,
    r_end: usize,
) {
    let mut j = l_start;
    let mut k = r_start;

    let mut i = l_start;

    while j < l_end {
        if k < r_end {
            let left_val = from[j];
            let right_val = from[k];
            if left_val <= right_val {
                to[i] = from[j];
                j += 1;
                i += 1;
            } else if left_val > right_val {
                to[i] = from[k];
                k += 1;
                i += 1;
            }
        } else {
            to[i] = from[j];
            j += 1;
            i += 1;
        }
    }

    while k < r_end {
        to[i] = from[k];
        i += 1;
        k += 1;
    }
}

#[test]
fn solve_2024_1() {
    let contents = fs::read_to_string("./resources/twenty_four/day_1.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2024_1 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2024_1 {p2_res}");
}
