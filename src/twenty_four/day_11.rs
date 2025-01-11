use std::collections::HashMap;
use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let mut p1_res = 0;
    let mut p2_res: usize = 0;

    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut rows: Vec<Vec<usize>> = contents
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|ch| ch.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut memo: Vec<HashMap<usize, usize>> = Vec::with_capacity(75);

    for i in 0..75 {
        memo.push(HashMap::new());
    }

    let mut original = rows.pop().unwrap();
    let mut vec;
    let mut next = Vec::new();

    for y in 0..original.len() {
        vec = vec![original[y]];
        p2_res += eval(original[y], 0, 74, &mut memo, &mut next);

        vec = next;
        next = Vec::new();
    }

    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}
pub fn eval(
    n: usize,
    iteration: usize,
    end: usize,
    memo: &mut Vec<HashMap<usize, usize>>,
    blink: &mut Vec<usize>,
) -> usize {
    if iteration == end {
        return if n == 0 {
            1
        } else if n.to_string().len() % 2 == 0 {
            2
        } else {
            1
        };
    }

    if memo[iteration].contains_key(&n) {
        return *memo[iteration].get(&n).unwrap();
    }

    if n == 0 {
        let res = eval(1, iteration + 1, end, memo, blink);
        memo[iteration].insert(n, res);
        res
    } else if n.to_string().len() % 2 == 0 {
        let s = n.to_string();
        let halves = s.split_at(s.len() / 2);
        let mut res = eval(
            halves.0.parse::<usize>().unwrap(),
            iteration + 1,
            end,
            memo,
            blink,
        );
        res += eval(
            halves.1.parse::<usize>().unwrap(),
            iteration + 1,
            end,
            memo,
            blink,
        );

        memo[iteration].insert(n, res);
        res
    } else {
        let mut res = eval(n * 2024, iteration + 1, end, memo, blink);
        memo[iteration].insert(n, res);
        res
    }
}
