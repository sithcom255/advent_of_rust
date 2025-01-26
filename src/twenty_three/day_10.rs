use crate::solve::Solve;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (y_len, x_len, start, adj) = parse(input);
        let mut mx = 0;
        let mut mx_steps = 0;

        let mut visited: HashSet<usize> = HashSet::new();
        let mut stack = vec![];
        stack.push(start);

        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            let todo = adj.get(&current).unwrap();

            for next in todo {
                if visited.contains(next) {
                    continue;
                }

                stack.push(*next);
            }
            let distance = to_distance(start, current, y_len, x_len);
            if distance > mx {
                mx = distance;
                mx_steps = visited.len();
            }

            visited.insert(current);
        }

        (visited.len() / 2).to_string()
    }

    fn p2(input: &String) -> String {
        let (y_len, x_len, start, adj) = parse(input);

        let mut visited: HashSet<usize> = HashSet::new();
        let mut stack = vec![];
        stack.push(start);

        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            let todo = adj.get(&current).unwrap();

            for next in todo {
                if visited.contains(next) {
                    continue;
                }

                stack.push(*next);
            }

            visited.insert(current);
        }

        let mut count = 0;

        for y in 0..y_len {
            let mut walls = 0;
            let mut in_wall = false;
            let mut down = false;
            for x in 0..x_len {
                let pos: isize = (x_len * y + x) as isize;
                if visited.contains(&(pos as usize)) {
                    if is_vertical(pos as usize, &adj, x_len) {
                        walls += 1;
                        continue;
                    }

                    if in_wall && is_diagonal(pos as usize, &adj, x_len) {
                        continue;
                    }

                    let up = points_up(pos as usize, &adj, x_len);

                    if !in_wall {
                        in_wall = true;
                        down = points_down(pos as usize, &adj, x_len);
                    } else if down && up || !down && !up {
                        walls += 1;
                        in_wall = false;
                    } else if down && !up || !down && up {
                        in_wall = false;
                    }
                    continue;
                }

                if walls % 2 == 1 {
                    count += 1;
                }
            }
        }

        count.to_string()
    }
}

fn parse(input: &str) -> (usize, usize, usize, HashMap<usize, HashSet<usize>>) {
    let mut adj: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut start: usize = 0;
    let rows: Vec<&str> = input.lines().collect();
    let y_len = rows.len();
    let x_len = rows[0].len();

    let mut start: usize = 0;

    for y in 0..y_len {
        let chars: Vec<char> = rows[y].chars().collect();
        for x in 0..x_len {
            let node: isize = (x_len * y + x) as isize;
            let mut cur_adj = HashSet::new();
            match chars[x] {
                '|' => {
                    up(x_len, node, &mut cur_adj);
                    down(y_len, x_len, node, &mut cur_adj);
                }
                '-' => {
                    left(x_len, node, &mut cur_adj);
                    right(x_len, node, &mut cur_adj);
                }
                'L' => {
                    up(x_len, node, &mut cur_adj);
                    right(x_len, node, &mut cur_adj);
                }
                'J' => {
                    up(x_len, node, &mut cur_adj);
                    left(x_len, node, &mut cur_adj);
                }
                '7' => {
                    down(y_len, x_len, node, &mut cur_adj);
                    left(x_len, node, &mut cur_adj);
                }
                'F' => {
                    down(y_len, x_len, node, &mut cur_adj);
                    right(x_len, node, &mut cur_adj);
                }
                'S' => {
                    left(x_len, node, &mut cur_adj);
                    right(x_len, node, &mut cur_adj);
                    up(x_len, node, &mut cur_adj);
                    down(y_len, x_len, node, &mut cur_adj);
                    start = node as usize;
                }
                _ => {}
            }
            if cur_adj.len() > 0 {
                adj.insert(node as usize, cur_adj);
            }
        }
    }

    let mut remove = vec![];
    {
        match &adj.get(&start) {
            None => {}
            Some(nodes) => {
                for node in nodes.iter() {
                    match adj.get(node) {
                        None => {
                            remove.push(*node);
                        }
                        Some(option) => {
                            if !option.contains(&start) {
                                remove.push(*node);
                            }
                        }
                    };
                }
            }
        }
    }

    match adj.get_mut(&start) {
        None => {}
        Some(nodes) => {
            for node in remove.iter() {
                nodes.remove(node);
            }
        }
    }
    (y_len, x_len, start, adj)
}

fn is_vertical(pos: usize, adj: &HashMap<usize, HashSet<usize>>, x_len: usize) -> bool {
    let nodes = adj.get(&pos).unwrap();
    for node in nodes {
        if (*node as isize - pos as isize) % x_len as isize != 0 {
            return false;
        }
    }
    true
}

fn is_diagonal(pos: usize, adj: &HashMap<usize, HashSet<usize>>, x_len: usize) -> bool {
    let nodes = adj.get(&pos).unwrap();
    for node in nodes {
        if (pos as isize - *node as isize).abs() != 1 {
            return false;
        }
    }
    true
}

fn points_up(pos: usize, adj: &HashMap<usize, HashSet<usize>>, x_len: usize) -> bool {
    let nodes = adj.get(&pos).unwrap();
    for node in nodes {
        if (pos as isize - *node as isize) == x_len as isize {
            return true;
        }
    }
    false
}

fn points_down(pos: usize, adj: &HashMap<usize, HashSet<usize>>, x_len: usize) -> bool {
    let nodes = adj.get(&pos).unwrap();
    for node in nodes {
        if (pos as isize - *node as isize) == -(x_len as isize) {
            return true;
        }
    }
    false
}

fn to_distance(a: usize, b: usize, y_len: usize, x_len: usize) -> usize {
    let y = ((a as isize / x_len as isize) - (b as isize / x_len as isize)).abs();
    let x = ((a as isize % x_len as isize) - (b as isize % x_len as isize)).abs();
    return (y + x) as usize;
}

fn left(x_len: usize, node: isize, adj: &mut HashSet<usize>) {
    let left = node - 1;
    if node % x_len as isize > 0 {
        adj.insert(left as usize);
    }
}

fn right(x_len: usize, node: isize, adj: &mut HashSet<usize>) {
    let right = node as usize + 1;
    if ((node % x_len as isize) + 1) < x_len as isize {
        adj.insert(right);
    }
}

fn up(x_len: usize, node: isize, adj: &mut HashSet<usize>) {
    let up = node - x_len as isize;
    if up >= 0 {
        adj.insert(up as usize);
    }
}

fn down(y_len: usize, x_len: usize, node: isize, adj: &mut HashSet<usize>) {
    let down = node as usize + x_len;
    if down <= (x_len * y_len) {
        adj.insert(down);
    }
}
#[test]
fn solve_2023_10() {
    let contents = fs::read_to_string("./resources/twenty_three/day_10.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_10 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_10 P2 {p2_res}");
}
