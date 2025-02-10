use crate::solve::Solve;
use crate::utils::grid::{bound_check, get_next};
use clap::Parser;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str::Split;

struct Solver {}

impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (nodes, target) = parse(input);
        longest(&nodes, &mut HashSet::new(), 1, target).to_string()
    }

    fn p2(input: &String) -> String {
        let (nodes, target) = parse_2(input);
        longest(&nodes, &mut HashSet::new(), 1, target).to_string()
    }
}

fn longest(
    nodes: &HashMap<usize, Node23>,
    visited: &mut HashSet<usize>,
    pos: usize,
    target: usize,
) -> isize {
    if visited.contains(&pos) {
        return -10000;
    }

    if pos == target {
        return 0;
    }

    visited.insert(pos);

    let x = nodes.get(&pos).unwrap();
    let mut res = 0;

    for edge in &x.edge {
        let candidate = longest(nodes, visited, edge.target, target) + edge.steps as isize;
        res = max(res, candidate);
    }

    visited.remove(&pos);
    res
}

fn parse(input: &String) -> (HashMap<usize, Node23>, usize) {
    let mut rows: Vec<Vec<char>> = vec![];
    input.lines().for_each(|line| {
        rows.push(line.chars().collect());
    });

    let mut nodes: HashMap<usize, Node23> = HashMap::new();

    let mut y_start = 0;
    let mut x_start = 1;

    nodes.insert(
        1,
        Node23 {
            id: 1,
            edge: HashSet::new(),
            steps: 0,
            visited: false,
            max: 0,
        },
    );
    walk(&rows, &mut nodes, y_start + 1, x_start, 1, 1, 1, false);

    return (
        nodes,
        ((rows.len() - 1) * (rows[0].len())) + rows[0].len() - 2,
    );
}

fn parse_2(input: &String) -> (HashMap<usize, Node23>, usize) {
    let mut rows: Vec<Vec<char>> = vec![];
    input.lines().for_each(|line| {
        rows.push(line.chars().collect());
    });

    let mut nodes: HashMap<usize, Node23> = HashMap::new();

    let mut y_start = 0;
    let mut x_start = 1;

    nodes.insert(
        1,
        Node23 {
            id: 1,
            edge: HashSet::new(),
            steps: 0,
            visited: false,
            max: 0,
        },
    );
    walk_2(&rows, &mut nodes, y_start + 1, x_start, 1, 1, 1);

    return (
        nodes,
        ((rows.len() - 1) * (rows[0].len())) + rows[0].len() - 2,
    );
}

fn walk(
    rows: &Vec<Vec<char>>,
    nodes: &mut HashMap<usize, Node23>,
    y: usize,
    x: usize,
    steps: usize,
    previous_pos: usize,
    origin: usize,
    one_direction: bool,
) {
    let y_len = rows.len();
    let x_len = rows[1].len();

    let prev_y = (previous_pos / x_len) as isize;
    let prev_x = (previous_pos % x_len) as isize;

    let mut pos = y * x_len + x;

    if pos != 1 && nodes.contains_key(&pos) {
        nodes.get_mut(&origin).unwrap().edge.insert(Edge23 {
            steps: steps,
            target: pos,
        });
        if !one_direction {
            nodes.get_mut(&pos).unwrap().edge.insert(Edge23 {
                steps: steps,
                target: origin,
            });
        }
        return;
    }

    let c_ch = rows[y][x];

    let mut c = 0;
    for dir in 0..4 {
        let (y_t, x_t) = get_next(dir, y, x);

        if y_t == prev_y as isize && prev_x as isize == x_t {
            continue;
        }

        if bound_check(y_t, x_t, y_len, x_len) {
            let ch = rows[y_t as usize][x_t as usize];
            if ch != '#' {
                c += 1;
            }
        } else {
            c += 1;
        }
    }

    if c == 1 {
        for dir in 0..4 {
            let (y_t, x_t) = get_next(dir, y, x);

            if y_t == prev_y && prev_x == x_t {
                continue;
            }

            if bound_check(y_t, x_t, y_len, x_len) {
                let ch = rows[y_t as usize][x_t as usize];
                if can_really_go(dir, c_ch, ch) {
                    let british_pop_group =
                        one_direction || (ch == 'v' || ch == '<' || ch == '>' || ch == '^');

                    let continue_y = y_t;
                    let continue_x = x_t;

                    if continue_y != -1
                        && previous_pos != continue_y as usize * x_len + continue_x as usize
                    {
                        if bound_check(continue_y, continue_x, y_len, x_len) {
                            if rows[continue_y as usize][continue_x as usize] != '#' {
                                walk(
                                    rows,
                                    nodes,
                                    continue_y as usize,
                                    continue_x as usize,
                                    steps + 1,
                                    pos,
                                    origin,
                                    british_pop_group,
                                );
                            }
                        }
                    }
                }
            } else {
                nodes.insert(
                    pos,
                    Node23 {
                        id: pos,
                        edge: HashSet::new(),
                        steps,
                        visited: false,
                        max: 0,
                    },
                );
                nodes
                    .get_mut(&origin)
                    .unwrap()
                    .edge
                    .insert(Edge23 { steps, target: pos });
            }
        }
    } else if c > 1 {
        nodes.insert(
            pos,
            Node23 {
                id: pos,
                edge: HashSet::new(),
                steps: steps + 1,
                visited: false,
                max: 0,
            },
        );
        nodes.get_mut(&origin).unwrap().edge.insert(Edge23 {
            steps: steps,
            target: pos,
        });

        for dir in 0..4 {
            let (y_t, x_t) = get_next(dir, y, x);
            if bound_check(y_t, x_t, y_len, x_len) {
                let ch = rows[y_t as usize][x_t as usize];
                if can_really_go(dir, c_ch, ch) {
                    let add_y = y_t;
                    let add_x = x_t;

                    let next_pos = add_y as usize * x_len + add_x as usize;
                    if previous_pos != next_pos {
                        if bound_check(add_y, add_x, y_len, x_len) {
                            let ch = rows[add_y as usize][add_x as usize];
                            if ch != '#' {
                                let one_direction =
                                    ch == 'v' || ch == '<' || ch == '>' || ch == '^';
                                walk(
                                    rows,
                                    nodes,
                                    add_y as usize,
                                    add_x as usize,
                                    1,
                                    pos,
                                    pos,
                                    one_direction,
                                );
                            }
                        }
                    }
                }
            }
        }
    };
}

fn walk_2(
    rows: &Vec<Vec<char>>,
    nodes: &mut HashMap<usize, Node23>,
    y: usize,
    x: usize,
    steps: usize,
    previous_pos: usize,
    origin: usize,
) {
    let y_len = rows.len();
    let x_len = rows[1].len();

    let prev_y = (previous_pos / x_len) as isize;
    let prev_x = (previous_pos % x_len) as isize;

    let mut pos = y * x_len + x;

    if pos != 1 && nodes.contains_key(&pos) {
        nodes.get_mut(&origin).unwrap().edge.insert(Edge23 {
            steps,
            target: pos,
        });

        nodes.get_mut(&pos).unwrap().edge.insert(Edge23 {
            steps,
            target: origin,
        });

        return;
    }

    let mut c = 0;
    for dir in 0..4 {
        let (y_t, x_t) = get_next(dir, y, x);

        if y_t == prev_y && prev_x == x_t {
            continue;
        }

        if bound_check(y_t, x_t, y_len, x_len) {
            let ch = rows[y_t as usize][x_t as usize];
            if ch != '#' {
                c += 1;
            }
        } else {
            c += 1;
        }
    }

    if c == 1 {
        for dir in 0..4 {
            let (y_t, x_t) = get_next(dir, y, x);

            if y_t == prev_y && prev_x == x_t {
                continue;
            }

            if bound_check(y_t, x_t, y_len, x_len) {
                let ch = rows[y_t as usize][x_t as usize];
                if ch != '#' {

                    let continue_y = y_t;
                    let continue_x = x_t;

                    if continue_y != -1
                        && previous_pos != continue_y as usize * x_len + continue_x as usize
                    {
                        if bound_check(continue_y, continue_x, y_len, x_len) {
                            if rows[continue_y as usize][continue_x as usize] != '#' {
                                walk_2(
                                    rows,
                                    nodes,
                                    continue_y as usize,
                                    continue_x as usize,
                                    steps + 1,
                                    pos,
                                    origin
                                );
                            }
                        }
                    }
                }
            } else {
                nodes.insert(
                    pos,
                    Node23 {
                        id: pos,
                        edge: HashSet::new(),
                        steps,
                        visited: false,
                        max: 0,
                    },
                );
                nodes
                    .get_mut(&origin)
                    .unwrap()
                    .edge
                    .insert(Edge23 { steps, target: pos });
            }
        }
    } else if c > 1 {
        nodes.insert(
            pos,
            Node23 {
                id: pos,
                edge: HashSet::new(),
                steps: steps + 1,
                visited: false,
                max: 0,
            },
        );
        nodes.get_mut(&origin).unwrap().edge.insert(Edge23 {
            steps,
            target: pos,
        });

        nodes.get_mut(&pos).unwrap().edge.insert(Edge23 {
            steps,
            target: origin,
        });

        for dir in 0..4 {
            let (y_t, x_t) = get_next(dir, y, x);
            if bound_check(y_t, x_t, y_len, x_len) {
                let ch = rows[y_t as usize][x_t as usize];
                if ch != '#' {
                    let add_y = y_t;
                    let add_x = x_t;

                    let next_pos = add_y as usize * x_len + add_x as usize;
                    if previous_pos != next_pos {
                        if bound_check(add_y, add_x, y_len, x_len) {
                            let ch = rows[add_y as usize][add_x as usize];
                            if ch != '#' {

                                walk_2(
                                    rows,
                                    nodes,
                                    add_y as usize,
                                    add_x as usize,
                                    1,
                                    pos,
                                    pos,
                                );
                            }
                        }
                    }
                }
            }
        }
    };
}

fn can_really_go(dir: usize, current: char, ch: char) -> bool {
    if ch == '#' {
        return false;
    }

    match dir {
        0 => (current == '.' || current == '^') && ch != 'v',
        1 => (current == '.' || current == '>') && ch != '<',
        2 => (current == '.' || current == 'v') && ch != '^',
        3 => (current == '.' || current == '<') && ch != '>',
        _ => {
            unreachable!()
        }
    }
}

#[derive(Clone, Debug)]
struct Node23 {
    id: usize,
    edge: HashSet<Edge23>,
    steps: usize,
    visited: bool,
    max: usize,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Edge23 {
    steps: usize,
    target: usize,
}
#[test]
fn test() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_23 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_23 P2 {p2_res}");
}

#[test]
fn solve_2023_23() {
    let contents = fs::read_to_string("./resources/twenty_three/day_23.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_23 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_23 P2 {p2_res}");
}
