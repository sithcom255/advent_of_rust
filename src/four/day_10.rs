use std::collections::HashSet;
use std::fs;


pub fn advent_of_rust(file_path: &str) {
    let mut p1_res = 0;
    let mut p2_res: usize = 0;

    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut lines: Vec<Vec<usize>> = contents
        .split("\n")
        .map(|line| line.chars().map(|ch| ch as usize - 0x30usize).collect())
        .collect();

    let mut y_len = lines.len();

    {
        let mut memo: Vec<Vec<HashSet<[usize; 2]>>> = Vec::with_capacity(lines.len());
        lines.iter().for_each(|row| {
            let mut memo_row = Vec::with_capacity(row.len());
            for i in 0..row.len() {
                memo_row.push(HashSet::new());
            }
            memo.push(memo_row);
        });

        let empty = HashSet::new();

        let mut visited = Vec::with_capacity(lines.len());
        lines.iter().for_each(|row| {
            let mut memo_row = Vec::with_capacity(row.len());
            for i in 0..row.len() {
                memo_row.push(false);
            }
            visited.push(memo_row);
        });

        for y in 0..y_len {
            let x_len = lines[0].len();
            for x in 0..x_len {
                if lines[y][x] == 0 {
                    p1_res += solve_from_p1(
                        &lines,
                        &mut memo,
                        &mut visited,
                        y as isize,
                        x as isize,
                        y_len,
                        x_len,
                        -1,
                        &empty,
                    )
                    .len();
                    reset_visited(&mut visited);
                }
            }
        }
    }
    {
        let mut memo: Vec<Vec<isize>> = Vec::with_capacity(lines.len());
        lines.iter().for_each(|row| {
            let mut memo_row = Vec::with_capacity(row.len());
            for i in 0..row.len() {
                memo_row.push(-1);
            }
            memo.push(memo_row);
        });

        let mut visited = Vec::with_capacity(lines.len());
        lines.iter().for_each(|row| {
            let mut memo_row = Vec::with_capacity(row.len());
            for i in 0..row.len() {
                memo_row.push(false);
            }
            visited.push(memo_row);
        });

        for y in 0..y_len {
            let x_len = lines[0].len();
            for x in 0..x_len {
                if lines[y][x] == 0 {
                    p2_res += solve_from_p2(
                        &lines,
                        &mut memo,
                        &mut visited,
                        y as isize,
                        x as isize,
                        y_len,
                        x_len,
                        -1,
                    );
                    reset_visited(&mut visited);
                }
            }
        }
    }

    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}

pub fn solve_from_p1<'a>(
    rows: &Vec<Vec<usize>>,
    memo: &'a mut Vec<Vec<HashSet<[usize; 2]>>>,
    visited: &mut Vec<Vec<bool>>,
    y: isize,
    x: isize,
    y_len: usize,
    x_len: usize,
    previous_value: isize,
    empty: &'a HashSet<[usize; 2]>,
) -> &'a HashSet<[usize; 2]> {
    if !bound_check(y, x, y_len, x_len) {
        return empty;
    }

    let y_safe = y as usize;
    let x_safe = x as usize;

    if visited[y_safe][x_safe] {
        return &memo[y_safe][x_safe];
    }

    if previous_value + 1 != rows[y_safe][x_safe] as isize {
        return empty;
    }

    let value = rows[y_safe][x_safe];

    if value == 9 {
        visited[y_safe][x_safe] = true;
        let mut result = HashSet::new();
        result.insert([y_safe, x_safe]);
        memo[y_safe][x_safe] = result;
        return &memo[y_safe][x_safe];
    }

    let mut result = HashSet::new();
    solve_from_p1(
        rows,
        memo,
        visited,
        y + 1,
        x,
        y_len,
        x_len,
        value as isize,
        empty,
    )
    .iter()
    .for_each(|e| {
        result.insert(*e);
    });
    solve_from_p1(
        rows,
        memo,
        visited,
        y - 1,
        x,
        y_len,
        x_len,
        value as isize,
        empty,
    )
    .iter()
    .for_each(|e| {
        result.insert(*e);
    });
    solve_from_p1(
        rows,
        memo,
        visited,
        y,
        x + 1,
        y_len,
        x_len,
        value as isize,
        empty,
    )
    .iter()
    .for_each(|e| {
        result.insert(*e);
    });
    solve_from_p1(
        rows,
        memo,
        visited,
        y,
        x - 1,
        y_len,
        x_len,
        value as isize,
        empty,
    )
    .iter()
    .for_each(|e| {
        result.insert(*e);
    });

    memo[y_safe][x_safe] = result;

    visited[y_safe][x_safe] = true;
    return &memo[y_safe][x_safe];
}

pub fn bound_check(y: isize, x: isize, y_len: usize, x_len: usize) -> bool {
    (0 <= y && y < y_len as isize) && (0 <= x && x < x_len as isize)
}

pub fn reset_visited(visited: &mut Vec<Vec<bool>>) {
    for y in 0..visited.len() {
        for x in 0..visited[0].len() {
            visited[y][x] = false;
        }
    }
}

pub fn solve_from_p2<'a>(
    rows: &Vec<Vec<usize>>,
    memo: &'a mut Vec<Vec<isize>>,
    visited: &mut Vec<Vec<bool>>,
    y: isize,
    x: isize,
    y_len: usize,
    x_len: usize,
    previous_value: isize,
) -> usize {
    if !bound_check(y, x, y_len, x_len) {
        return 0;
    }

    let y_safe = y as usize;
    let x_safe = x as usize;


    // if visited[y_safe][x_safe] {
    //     return 0;
    // }


    if previous_value + 1 != rows[y_safe][x_safe] as isize {
        return 0;
    }

    if memo[y_safe][x_safe] != -1 {
        return memo[y_safe][x_safe] as usize;
    }

    let value = rows[y_safe][x_safe];

    if value == 9 {
        visited[y_safe][x_safe] = true;
        memo[y_safe][x_safe] = 1;
        return 1;
    }

    let mut result = 0;
    result += solve_from_p2(rows, memo, visited, y + 1, x, y_len, x_len, value as isize);
    result += solve_from_p2(rows, memo, visited, y - 1, x, y_len, x_len, value as isize);
    result += solve_from_p2(rows, memo, visited, y, x + 1, y_len, x_len, value as isize);
    result += solve_from_p2(rows, memo, visited, y, x - 1, y_len, x_len, value as isize);

    memo[y_safe][x_safe] = result as isize;
    visited[y_safe][x_safe] = true;

    return memo[y_safe][x_safe] as usize;
}
