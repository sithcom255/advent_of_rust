use std::collections::HashMap;
use std::fs;

pub fn advent_of_rust(contents: &str) {
    let mut lines: Vec<&str> = contents.split("\n").collect();
    let mut p1_res = 1;
    let mut p2_res = 0;

    let mut visited: Vec<Vec<bool>> = Vec::with_capacity(lines.len());
    let mut visited_dir: Vec<Vec<HashMap<usize, bool>>> = Vec::with_capacity(lines.len());
    let mut obstacles: Vec<Vec<bool>> = Vec::with_capacity(lines.len());

    let mut x_start = 0;
    let mut y_start = 0;

    let mut x_pos = 0;
    let mut y_pos = 0;

    let mut x_inc: isize = 0;
    let mut y_inc: isize = 0;

    let mut y_len = 0;
    let mut x_len = 0;

    for y in 0..lines.len() {
        let line = lines[y];
        y_len = line.len();
        let mut visited_row = vec![false; y_len];
        let mut obstacles_row = vec![false; y_len];

        let chars: Vec<char> = line.chars().collect();

        x_len = chars.len();
        for x in 0..chars.len() {
            let ch = chars[x];
            if ch == '.' {
                continue;
            }
            if ch == '#' {
                obstacles_row[x] = true;
            }
            if ch == '^' {
                x_pos = x;
                y_pos = y;
                x_start = x;
                y_start = y;
                y_inc = -1;
                visited_row[x] = true;
            }
        }

        let mut vis: Vec<HashMap<usize, bool>> = Vec::with_capacity(x_len);
        for i in 0..x_len {
            vis.push(HashMap::new());
        }

        visited.push(visited_row);
        obstacles.push(obstacles_row);
        visited_dir.push(vis);
    }

    while (x_pos < x_len) && (y_pos < y_len) {
        let mut next_pos_x = (x_pos as isize + x_inc) as usize;
        let mut next_pos_y = (y_pos as isize + y_inc) as usize;

        if !((next_pos_x < x_len) && (next_pos_y < y_len)) {
            break;
        }

        if obstacles[next_pos_y][next_pos_x] {
            next_dir(&mut x_inc, &mut y_inc);
            continue;
        }

        if !visited[next_pos_y][next_pos_x] {
            visited[next_pos_y][next_pos_x] = true;
            p1_res += 1;
        }

        x_pos = next_pos_x;
        y_pos = next_pos_y;
    }

    for y in 0..y_len {
        println!("row {}", y);
        for x in 0..x_len {
            x_pos = x_start;
            y_pos = y_start;
            x_inc = 0;
            y_inc = -1;

            if x == x_start && y == y_start {
                continue;
            }

            if obstacles[y][x] {
                continue;
            }

            if !visited[y][x] {
                continue;
            }

            obstacles[y][x] = true;

            while (x_pos < x_len) && (y_pos < y_len) {
                let mut next_pos_x = (x_pos as isize + x_inc) as usize;
                let mut next_pos_y = (y_pos as isize + y_inc) as usize;

                if !((next_pos_x < x_len) && (next_pos_y < y_len)) {
                    break;
                }

                let key = get_key(&mut x_inc, &mut y_inc);
                if visited_dir[next_pos_y][next_pos_x].contains_key(&key) {
                    p2_res += 1;
                    break;
                } else {
                    visited_dir[next_pos_y][next_pos_x].insert(key, true);
                }

                if obstacles[next_pos_y][next_pos_x] {
                    next_dir(&mut x_inc, &mut y_inc);
                    continue;
                }

                x_pos = next_pos_x;
                y_pos = next_pos_y;
            }

            obstacles[y][x] = false;

            for y in 0..y_len {
                for x in 0..x_len {
                    visited_dir[y][x].clear();
                }
            }
        }
    }

    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}

fn next_dir(x_inc: &mut isize, y_inc: &mut isize) {
    if *y_inc == 1 {
        *y_inc = 0;
        *x_inc = -1;
    } else if *y_inc == -1 {
        *y_inc = 0;
        *x_inc = 1;
    } else if *x_inc == 1 {
        *y_inc = 1;
        *x_inc = 0;
    } else if *x_inc == -1 {
        *y_inc = -1;
        *x_inc = 0;
    }
}

fn get_key(x_inc: &mut isize, y_inc: &mut isize) -> usize {
    if *y_inc == 1 {
        return 0;
    } else if *y_inc == -1 {
        return 1;
    } else if *x_inc == 1 {
        return 2;
    } else if *x_inc == -1 {
        return 3;
    }
    return 5;
}

fn solve_2024_6() {
    let contents = fs::read_to_string("./resources/twenty_four/day_6.txt")
        .expect("This is the error message");

    advent_of_rust(&contents);
}
