use crate::solve::Solve;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (brinks, children, parents) = parse(input);
        let mut p1 = 0;
        children.into_iter().for_each(|(key, value)| {
            for val in &value {
                if parents.get(val).unwrap().len() <= 1 {
                    return;
                }
            }
            p1 += 1;
        });
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let (mut brinks, mut children, mut parents) = parse(input);
        let mut p1 = 0;
        children.clone().into_iter().for_each(|(key, value)| {
            let mut chld = children.clone();
            let mut p = parents.clone();
            p1 += detonate(&mut chld, &mut p, key);
        });
        p1.to_string()
    }
}

fn detonate(
    children: &mut HashMap<usize, HashSet<usize>>,
    parents: &mut HashMap<usize, HashSet<usize>>,
    target: usize,
) -> usize {
    let mut res = 0;

    let mut todo = vec![];
    {
        let target_children = children.get(&target).unwrap();

        target_children.iter().for_each(|child| {
            parents.get_mut(child).unwrap().remove(&target);
            if parents.get(child).unwrap().len() == 0 {
                todo.push(*child);
            }
        });
    }
    res += todo.len();

    for td in todo {
        res += detonate(children, parents, td);
    }

    return res;
}

fn parse(
    input: &String,
) -> (
    Vec<(usize, usize, usize, usize, usize, usize)>,
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let mut bricks: Vec<(usize, usize, usize, usize, usize, usize)> = vec![];
    let mut children: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut parent: HashMap<usize, HashSet<usize>> = HashMap::new();

    input.lines().for_each(|line| {
        let d: Vec<usize> = line
            .split("~")
            .map(|half| half.split(",").map(|elem| elem.parse::<usize>().unwrap()))
            .flat_map(|half| half.into_iter())
            .collect();
        let dimensions = (d[0], d[1], d[2], d[3], d[4], d[5]);
        if d[2] > d[5] || d[1] > d[4] || d[0] > d[3] {
            unreachable!("{}", line);
        }
        bricks.push(dimensions)
    });

    bricks.sort_by(|a, b| {
        return match a.2.cmp(&b.2) {
            Ordering::Less => Less,
            Ordering::Equal => return a.5.cmp(&b.5),
            Ordering::Greater => Greater,
        };
    });

    for i in 0..bricks.len() {
        let mut current = bricks[i];

        children.insert(i, HashSet::new());

        let mut parents = HashSet::new();

        let mut found = false;
        let mut z = -1 as isize;
        for j in 0..i {
            let k = i - j - 1;
            let standing = bricks[k];

            if x_y_overlap(current, standing) {
                if z == -1 {
                    z = standing.5 as isize;
                }

                if standing.5 > z as usize {
                    z = standing.5 as isize
                }
            }
        }

        if z != -1 {
            for j in 0..i {
                let k = i - j - 1;
                let standing = bricks[k];

                if x_y_overlap(current, standing) && z == standing.5 as isize {
                    if !found {
                        let fallen = move_z(current, standing.5 + 1);
                        bricks[i] = fallen;
                        current = fallen;
                        found = true;
                    }

                    if found {
                        if standing.5 == (current.2 - 1) {
                            match children.get_mut(&k) {
                                None => {}
                                Some(chld) => {
                                    chld.insert(i);
                                }
                            }
                            parents.insert(k);
                        }
                    }
                }
            }
        }

        if !found {
            let fallen = move_z(current, 1);
            bricks[i] = fallen;
        }

        parent.insert(i, parents);
    }

    (bricks, children, parent)
}

fn move_z(
    falling: (usize, usize, usize, usize, usize, usize),
    new_z_start: usize,
) -> (usize, usize, usize, usize, usize, usize) {
    let (f_x_s, f_y_s, f_z_s, f_x_e, f_y_e, f_z_e) = falling;

    let mut diff = f_z_e - f_z_s;

    (f_x_s, f_y_s, new_z_start, f_x_e, f_y_e, new_z_start + diff)
}
fn x_y_overlap(
    falling: (usize, usize, usize, usize, usize, usize),
    standing: (usize, usize, usize, usize, usize, usize),
) -> bool {
    let (f_x_s, f_y_s, f_z_s, f_x_e, f_y_e, f_z_e) = falling;
    let (s_x_s, s_y_s, s_z_s, s_x_e, s_y_e, s_z_e) = standing;

    let x_fit = s_x_s <= f_x_s && f_x_s <= s_x_e
        || s_x_s <= f_x_e && f_x_e <= s_x_e
        || f_x_s <= s_x_s && s_x_e <= f_x_e;
    let y_fit = s_y_s <= f_y_s && f_y_s <= s_y_e
        || s_y_s <= f_y_e && f_y_e <= s_y_e
        || f_y_s <= s_y_s && s_y_e <= f_y_e;

    let my = x_fit && y_fit;
    my
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

#[test]
fn solve_2023_22() {
    let contents = fs::read_to_string("./resources/twenty_three/day_22.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_22 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_2 {p2_res}");
}
