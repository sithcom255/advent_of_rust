use crate::solve::Solve;
use std::collections::HashMap;
use std::fs;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut rows = parse(input);
        let mut p1 = 0;
        for (todo, numbers) in rows.iter_mut() {
            let mut memo: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
            p1 += resolve_p2(todo, numbers, 0, 0, &mut memo);
        }
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let mut rows = parse_p2(input);
        let mut p1 = 0;
        for (todo, numbers) in rows.iter_mut() {
            let mut memo: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
            p1 += resolve_p2(todo, numbers, 0, 0, &mut memo);
        }
        p1.to_string()
    }
}

fn resolve(chars: &mut Vec<char>, numbers: &Vec<usize>, ch_i: usize, n_i: usize) -> usize {
    if ch_i == chars.len() {
        if matches(chars, numbers, 0, 0) {
            println!("{:?}", chars);
            return 1;
        }
        return 0;
    }

    let mut p = 0;
    for i in ch_i..chars.len() {
        let ch = chars[i];
        if ch == '?' {
            chars[i] = '.';
            p += resolve(chars, numbers, i + 1, n_i);
            chars[i] = '#';
            p += resolve(chars, numbers, i + 1, n_i);
            chars[i] = '?';
            return p;
        }
        if i == chars.len() - 1 {
            if matches(chars, numbers, 0, 0) {
                println!("{:?}", chars);
                return 1;
            }
        }
    }
    0
}

fn resolve_p2(
    chars: &mut Vec<char>,
    numbers: &Vec<usize>,
    ch_i: usize,
    n_i: usize,
    memo: &mut HashMap<usize, HashMap<usize, usize>>,
) -> usize {
    if ch_i >= chars.len() {
        if numbers.len() == n_i {
            return 1;
        }
        return 0;
    }

    if numbers.len() == n_i {
        if empty(chars, ch_i) {
            return 1;
        }
        return 0;
    }

    match memo.get(&ch_i) {
        None => {}
        Some(values) => {
            match values.get(&n_i) {
                None => {}
                Some(res) => {
                    return *res;
                }
            };
        }
    }

    let mut p = 0;
    let ch = chars[ch_i];
    if ch == '?' {
        p += resolve_p2(chars, numbers, ch_i + 1, n_i, memo);
        let number = numbers[n_i];
        if can_consume(chars, number, ch_i) {
            p += resolve_p2(chars, numbers, ch_i + number + 1, n_i + 1, memo);
        }
    }

    if ch == '#' {
        let number = numbers[n_i];
        if can_consume(chars, number, ch_i) {
            p += resolve_p2(chars, numbers, ch_i + number + 1, n_i + 1, memo);
        }
    }

    if ch == '.' {
        p += resolve_p2(chars, numbers, ch_i + 1, n_i, memo);
    }

    match memo.get_mut(&ch_i) {
        None => {
            let mut map = HashMap::new();
            map.insert(n_i, p);
            memo.insert(ch_i, map);
        }
        Some(values) => {
            match values.get(&n_i) {
                None => values.insert(n_i, p),
                Some(res) => {
                    panic!("fuck");
                }
            };
        }
    }
    p
}

fn empty(chars: &Vec<char>, ch_i: usize) -> bool {
    for i in ch_i..chars.len() {
        if chars[i] == '#' {
            return false;
        }
    }
    true
}

fn can_consume(chars: &Vec<char>, number: usize, ch_i: usize) -> bool {
    if ch_i + number == chars.len() {
        for i in ch_i..(ch_i + number) {
            if chars[i] == '.' {
                return false;
            }
        }
        return true;
    }

    if ch_i + number < chars.len() {
        for i in ch_i..(ch_i + number) {
            if chars[i] == '.' {
                return false;
            }
        }
        if chars[ch_i + number] != '#' {
            return true;
        }
    }

    false
}

fn matches(chars: &Vec<char>, numbers: &Vec<usize>, ch_i: usize, n_i: usize) -> bool {
    let mut inside = false;
    let mut count = 0;
    let mut j = n_i;
    for i in ch_i..chars.len() {
        if j > numbers.len() {
            return false;
        }

        let ch = chars[i];

        if !inside && ch == '#' {
            inside = true;
            count += 1;
            continue;
        }

        if inside && ch == '#' {
            count += 1;
        } else if inside {
            inside = false;
            if j >= numbers.len() {
                return false;
            }
            if count == numbers[j] {
                j += 1;
            } else {
                return false;
            }
            count = 0;
        }
    }

    if j > numbers.len() {
        return false;
    }

    if inside {
        if j >= numbers.len() {
            return false;
        }
        if count == numbers[j] {
            j += 1;
        }
    }
    j == numbers.len()
}

fn parse(input: &String) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut rows: Vec<(Vec<char>, Vec<usize>)> = vec![];
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let number: Vec<usize> = parts[1]
            .trim_end()
            .trim_start()
            .split(",")
            .map(|elem| {
                return elem.trim_end().trim_start().parse::<usize>().unwrap();
            })
            .collect();
        rows.push((parts[0].chars().collect(), number));
    });
    rows
}

fn parse_p2(input: &String) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut rows: Vec<(Vec<char>, Vec<usize>)> = vec![];
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let number: Vec<usize> = parts[1]
            .trim_end()
            .trim_start()
            .split(",")
            .map(|elem| {
                return elem.trim_end().trim_start().parse::<usize>().unwrap();
            })
            .collect();

        let chars: Vec<char> = parts[0].chars().collect();

        let mut partx5 = vec![];
        let mut numberx5 = vec![];
        for i in 0..5 {
            partx5.extend(chars.iter());
            if i < 4 {
                partx5.push('?');
            }
            numberx5.extend(number.iter());
        }

        rows.push((partx5, numberx5));
    });
    rows
}

#[test]
fn solve_2023_12() {
    let contents = fs::read_to_string("./resources/twenty_three/day_12.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_12 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_12 P2 {p2_res}");
}

#[test]
fn test() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_12 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_12 P2 {p2_res}");
}
