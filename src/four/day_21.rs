use std::collections::HashMap;
use std::fs;

pub fn aoc_p1(contents: String) -> String {
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    input.iter().for_each(|line| {
        let mul = line
            .trim_start()
            .strip_suffix("A")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut cache: HashMap<usize, HashMap<char, Vec<char>>> = HashMap::new();
        let todo: Vec<char> = line.trim_start().trim_end().chars().collect();
        let done = solve_keypad(&mut cache, &todo, 1, 'R');

        println!("{:?}", done.iter().collect::<String>());
        p1 += done.len() * mul;

        let mut cache2: HashMap<usize, HashMap<char, HashMap<char, usize>>> = HashMap::new();

        let memo = solve_keypad_memo(&mut cache2, &todo, 1, 'R', 3);
        println!("p2 {}", memo)
    });

    p1.to_string()
}

pub fn aoc_p2(contents: String) -> String {
    let mut p2 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();
    let mut cache: HashMap<usize, HashMap<char, HashMap<char, usize>>> = HashMap::new();

    input.iter().for_each(|line| {
        let mul = line
            .trim_start()
            .strip_suffix("A")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let todo: Vec<char> = line.trim_start().trim_end().chars().collect();

        let done = solve_keypad_memo(&mut cache, &todo, 1, 'R', 26);
        p2 += done* mul;
    });

    p2.to_string()
}

fn solve_keypad(
    cache: &mut HashMap<usize, HashMap<char, Vec<char>>>,
    todo: &Vec<char>,
    level: usize,
    key: char,
) -> Vec<char> {
    let mut res = Vec::new();

    if level == 1 {
        let mut start_state: [usize; 2] = get_numerical_to_yx('A');

        for i in 0..todo.len() {
            let ch = todo[i];
            let end_state: [usize; 2] = get_numerical_to_yx(ch);

            if start_state != end_state {
                let path = path_numerical(start_state, end_state);

                let mut vec = eval_dirs_to_arrows(&path[0]);

                let mut recursive_solution = solve_keypad(cache, &vec, level + 1, ch);

                for k in 1..path.len() {
                    let mut vec = eval_dirs_to_arrows(&path[k]);
                    let mut path_one = solve_keypad(cache, &vec, level + 1, ch);

                    if &path_one.len() < &recursive_solution.len() {
                        recursive_solution = path_one;
                    }
                }

                for j in 0..recursive_solution.len() {
                    res.push(recursive_solution[j]);
                }
            } else {
                let mut vec = vec![];
                vec.push('A');
                let recursive_solution = solve_keypad(cache, &vec, level + 1, ch);

                for j in 0..recursive_solution.len() {
                    res.push(recursive_solution[j]);
                }
            }
            start_state = end_state;
        }
    } else if level > 1 && level <= 3 {
        let mut start_state: [usize; 2] = get_arrows_to_yx('A');

        for i in 0..todo.len() {
            let ch = todo[i];
            let end_state: [usize; 2] = get_arrows_to_yx(ch);

            if start_state != end_state {
                let path = path_arrows(start_state, end_state);

                let mut recursive_solution = get_recursive_arrows(cache, level, ch, &path, 0);

                for k in 1..path.len() {
                    let mut candidate = get_recursive_arrows(cache, level, ch, &path, k);
                    if &candidate.len() < &recursive_solution.len() {
                        recursive_solution = candidate;
                    }
                }

                for j in 0..recursive_solution.len() {
                    res.push(recursive_solution[j]);
                }
            } else {
                let mut vec = vec![];
                vec.push('A');
                let recursive_solution = solve_keypad(cache, &vec, level + 1, ch);

                for j in 0..recursive_solution.len() {
                    res.push(recursive_solution[j]);
                }
            }

            start_state = end_state;
        }
    } else {
        res = todo.clone();
    }

    if level == 1 {
        return res;
    }

    res
}

fn solve_keypad_memo(
    cache: &mut HashMap<usize, HashMap<char, HashMap<char, usize>>>,
    todo: &Vec<char>,
    level: usize,
    key: char,
    iterations: usize,
) -> usize {
    let mut res = 0;

    if level == 1 {
        let mut start_state: [usize; 2] = get_numerical_to_yx('A');

        for i in 0..todo.len() {
            let ch = todo[i];
            let end_state: [usize; 2] = get_numerical_to_yx(ch);

            if start_state != end_state {
                let path = path_numerical(start_state, end_state);

                let vec = eval_dirs_to_arrows(&path[0]);

                let mut recursive_solution =
                    solve_keypad_memo(cache, &vec, level + 1, ch, iterations);

                for k in 1..path.len() {
                    let vec = eval_dirs_to_arrows(&path[k]);

                    let mut path_one = solve_keypad_memo(cache, &vec, level + 1, ch, iterations);
                    if &path_one < &recursive_solution {
                        recursive_solution = path_one;
                    }
                }

                res += recursive_solution;
            } else {
                let mut vec = vec![];
                vec.push('A');
                let recursive_solution = solve_keypad_memo(cache, &vec, level + 1, ch, iterations);

                res += recursive_solution;
            }
            start_state = end_state;
        }
    } else if level > 1 && level <= iterations {
        let mut start_key = 'A';
        let mut end_key = 'A';

        for i in 0..todo.len() {
            end_key = todo[i];

            let mut start_state: [usize; 2] = get_arrows_to_yx(start_key);
            let end_state: [usize; 2] = get_arrows_to_yx(end_key);

            if let Some(value) = check_cache(cache, level, start_key, end_key) {
                res += value;
            } else {
                if start_state != end_state {
                    let path = path_arrows(start_state, end_state);

                    let mut arrows = eval_dirs_to_arrows(&&path[0]);
                    let mut recursive_solution =
                        solve_keypad_memo(cache, &arrows, level + 1, key, iterations);

                    for k in 1..path.len() {
                        let mut arrows = eval_dirs_to_arrows(&path[k]);
                        let candidate = solve_keypad_memo(cache, &arrows, level + 1, key, iterations);

                        if candidate < recursive_solution {
                            recursive_solution = candidate;
                        }
                    }

                    res += recursive_solution;
                    insert_res(cache, level, start_key, end_key, recursive_solution.clone());
                } else {
                    let mut vec = vec![];
                    vec.push('A');
                    let recursive_solution =
                        solve_keypad_memo(cache, &vec, level + 1, end_key, iterations);

                    res += recursive_solution;
                    insert_res(cache, level, start_key, end_key, recursive_solution.clone());
                }
            }

            start_key = end_key;
        }
    } else {
        res = todo.len();
    }

    if level <= 2 {
        return res;
    }

    res
}

fn check_cache(
    cache: &mut HashMap<usize, HashMap<char, HashMap<char, usize>>>,
    level: usize,
    from: char,
    to: char,
) -> Option<usize> {
    match cache.get(&level) {
        None => {}
        Some(level_value) => {
            match level_value.get(&from) {
                None => {}
                Some(from_map) => match from_map.get(&to) {
                    None => {}
                    Some(vector) => {
                        return Some(vector.clone());
                    }
                },
            };
        }
    }
    None
}

fn insert_res(
    cache: &mut HashMap<usize, HashMap<char, HashMap<char, usize>>>,
    level: usize,
    from: char,
    to: char,
    res: usize,
) {
    match cache.get_mut(&level) {
        None => {
            let mut to_map = HashMap::new();
            to_map.insert(to, res.clone());
            let mut from_map = HashMap::new();
            from_map.insert(from, to_map);
            cache.insert(level, from_map);
        }
        Some(from_map) => match from_map.get_mut(&from) {
            None => {
                let mut to_map = HashMap::new();
                to_map.insert(to, res.clone());
                from_map.insert(from, to_map);
            }
            Some(to_map) => match to_map.get(&to) {
                None => {
                    to_map.insert(to, res.clone());
                }
                Some(vector) => {
                    println!("reinsert")
                }
            },
        },
    }
}

fn get_recursive_arrows(
    cache: &mut HashMap<usize, HashMap<char, Vec<char>>>,
    level: usize,
    ch: char,
    path: &Vec<Vec<Direction>>,
    i: usize,
) -> Vec<char> {
    let mut vec = eval_dirs_to_arrows(&path[i]);

    let recursive_solution = solve_keypad(cache, &vec, level + 1, ch);
    recursive_solution
}

fn path_numerical(start: [usize; 2], end: [usize; 2]) -> Vec<Vec<Direction>> {
    let y_diff = start[0] as isize - end[0] as isize;
    let x_diff = start[1] as isize - end[1] as isize;

    let mut y_poison = -1000;
    let mut x_poison = -1000;

    let mut res = vec![];
    if start[0] as isize - y_diff == 3 && start[1] == 0 {
        x_poison = x_diff;
    }

    if start[1] as isize - x_diff == 0 && start[0] == 3 {
        y_poison = y_diff;
    }

    generate(y_diff, x_diff, &mut vec![], &mut res, y_poison, x_poison);
    res
}

fn generate(
    y: isize,
    x: isize,
    so_far: &mut Vec<Direction>,
    res: &mut Vec<Vec<Direction>>,
    y_poison: isize,
    x_poison: isize,
) {
    if y == 0 && x == 0 {
        res.push(so_far.clone());
        return;
    }

    let mut y_next = 0;
    let mut x_next = 0;

    let mut x_step = 0;
    let mut y_step = 0;

    if y != 0 {
        if y < 0 {
            y_next = y + 1;
            y_step = 1;
        } else {
            y_next = y - 1;
            y_step = -1;
        }
    }

    if x != 0 {
        if x < 0 {
            x_next = x + 1;
            x_step = 1;
        } else {
            x_next = x - 1;
            x_step = -1;
        }
    }

    if y != 0 && x != x_poison {
        so_far.push(Direction {
            is_y: true,
            movement: y_step,
        });
        generate(y_next, x, so_far, res, y_poison, x_poison);
        so_far.pop();
    }

    if x != 0 && y != y_poison {
        so_far.push(Direction {
            is_y: false,
            movement: x_step,
        });
        generate(y, x_next, so_far, res, y_poison, x_poison);
        so_far.pop();
    }
}

fn path_arrows(start: [usize; 2], end: [usize; 2]) -> Vec<Vec<Direction>> {
    // 0, 2
    // 1, 0
    let y_diff = start[0] as isize - end[0] as isize;
    let x_diff = start[1] as isize - end[1] as isize;
    let mut y_poison = -1000;
    let mut x_poison = -1000;

    let mut res = vec![];
    if start[1] as isize - x_diff == 0 && start[0] == 0 {
        y_poison = y_diff;
    }

    if start[0] as isize - y_diff == 0 && start[1] == 0 {
        x_poison = x_diff;
    }

    generate(y_diff, x_diff, &mut vec![], &mut res, y_poison, x_poison);
    res
}

fn eval_dirs_to_arrows(dirs: &Vec<Direction>) -> Vec<char> {
    let mut arrows = vec![];

    for i in 0..dirs.len() {
        let d = &dirs[i];
        if d.movement == 0 {
            continue;
        }
        for i in 0..d.movement.abs() as usize {
            if d.is_y {
                arrows.push(get_char(d.movement, 0));
            } else {
                arrows.push(get_char(0, d.movement));
            }
        }
    }

    arrows.push('A');

    arrows
}
fn get_char(y: isize, x: isize) -> char {
    if x < 0 {
        return '<';
    }
    if x > 0 {
        return '>';
    }
    if y < 0 {
        return '^';
    }
    if y > 0 {
        return 'v';
    }
    'O'
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Direction {
    is_y: bool,
    movement: isize,
}

fn get_numerical_to_yx(key: char) -> [usize; 2] {
    match key {
        'A' => [3, 2],
        '0' => [3, 1],
        '1' => [2, 0],
        '2' => [2, 1],
        '3' => [2, 2],
        '4' => [1, 0],
        '5' => [1, 1],
        '6' => [1, 2],
        '7' => [0, 0],
        '8' => [0, 1],
        '9' => [0, 2],
        _ => {
            println!("ERROR get_numerical_to_yx");
            [0, 0]
        }
    }
}

fn get_arrows_to_yx(key: char) -> [usize; 2] {
    match key {
        '^' => [0, 1],
        'A' => [0, 2],
        '<' => [1, 0],
        'v' => [1, 1],
        '>' => [1, 2],
        _ => {
            println!("ERROR get_arrows_to_yx");
            [0, 0]
        }
    }
}

#[test]
fn example() {
    let example = "029A
980A
179A
456A
379A";

    assert_eq!("126384", aoc_p1(example.to_owned()));
}

#[test]
fn solve_() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/input")
        .expect("This is the error message");

    let p1_res = aoc_p1(contents.to_owned());
    println!("RES P1 {p1_res}");

    let p2_res = aoc_p2(contents.to_owned());
    println!("RES P2 {p2_res}");
}

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}
