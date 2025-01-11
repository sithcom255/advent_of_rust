use regex::Regex;
use std::fs;

pub fn advent_of_rust(file_path: &str) -> [usize; 2] {
    let mut p1_res = 0;
    let mut p2_res: usize = 0;

    let regex = Regex::new(r"p=([\d]+),([\d]+) v=(-?[\d]+),(-?[\d]+)").unwrap();

    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut rows: Vec<&str> = contents.split("\n").collect();

    let y_len = rows.len();
    let x_len = rows[0].len();

    {
        let mut walls: Vec<Vec<bool>> = Vec::with_capacity(y_len);
        setup(y_len, x_len, &mut walls);
        let mut objects: Vec<Vec<bool>> = Vec::with_capacity(y_len);
        setup(y_len, x_len, &mut objects);

        let mut y_pos = 0;
        let mut x_pos = 0;

        let mut loaded = false;
        for y in 0..rows.len() {
            let line = rows[y];

            if line.is_empty() {
                loaded |= line.is_empty();
                continue;
            }

            if !loaded {
                let chs: Vec<char> = line.chars().collect();
                for x in 0..chs.len() {
                    let ch = chs[x];
                    if ch == '#' {
                        walls[y][x] = true;
                    }
                    if ch == '@' {
                        y_pos = y;
                        x_pos = x;
                    }
                    if ch == 'O' {
                        objects[y][x] = true;
                    }
                }
            } else {
                let chs: Vec<char> = line.chars().collect();
                for x in 0..chs.len() {
                    let ch = chs[x];
                    let dir = get_dir(ch);

                    let y_t = (y_pos as isize + dir[0]) as usize;
                    let x_t = (x_pos as isize + dir[1]) as usize;

                    if !objects[y_t][x_t] && !walls[y_t][x_t] {
                        y_pos = y_t;
                        x_pos = x_t;
                        continue;
                    }

                    if objects[y_t][x_t] {
                        if try_push(&walls, &mut objects, y_t, x_t, dir) {
                            y_pos = y_t;
                            x_pos = x_t;
                        }
                    }
                }
            }
        }
        p1_res = get_gps(&objects);
    }

    {
        let mut walls: Vec<Vec<bool>> = Vec::with_capacity(y_len);
        setup(y_len, x_len * 2, &mut walls);
        let mut objects: Vec<Vec<usize>> = Vec::with_capacity(y_len);
        setup_usize(y_len, x_len * 2, &mut objects);

        let mut y_pos = 0;
        let mut x_pos = 0;

        let mut loaded = false;
        for y in 0..rows.len() {
            let line = rows[y];

            if line.is_empty() {
                loaded |= line.is_empty();
                continue;
            }

            if !loaded {
                let chs: Vec<char> = line.chars().collect();
                for x in 0..chs.len() {
                    let ch = chs[x];
                    if ch == '#' {
                        walls[y][x * 2] = true;
                        walls[y][(x * 2) + 1] = true;
                    }
                    if ch == '@' {
                        y_pos = y;
                        x_pos = x * 2;
                    }
                    if ch == 'O' {
                        objects[y][x * 2] = 1;
                        objects[y][(x * 2) + 1] = 2;
                    }
                }
            } else {
                let chs: Vec<char> = line.chars().collect();
                for x in 0..chs.len() {
                    let ch = chs[x];
                    let dir = get_dir(ch);

                    let y_t = (y_pos as isize + dir[0]) as usize;
                    let x_t = (x_pos as isize + dir[1]) as usize;

                    let target = objects[y_t][x_t];

                    if target == 0 && !walls[y_t][x_t] {
                        y_pos = y_t;
                        x_pos = x_t;
                        continue;
                    }

                    if target != 0 {
                        if try_push_p2(&walls, &mut objects, y_t, x_t, dir, ch, false) {
                            y_pos = y_t;
                            x_pos = x_t;
                        }
                    }
                }
            }
        }
        p2_res = get_gps_p2(&objects);
    }

    [p1_res, p2_res]
}

fn try_push(
    walls: &Vec<Vec<bool>>,
    objects: &mut Vec<Vec<bool>>,
    y_pos: usize,
    x_pos: usize,
    dir: [isize; 2],
) -> bool {
    let y_t = (y_pos as isize + dir[0]) as usize;
    let x_t = (x_pos as isize + dir[1]) as usize;

    if walls[y_t][x_t] {
        return false;
    }

    if objects[y_t][x_t] {
        if try_push(walls, objects, y_t, x_t, dir) {
            objects[y_pos][x_pos] = false;
            objects[y_t][x_t] = true;
            return true;
        }
        return false;
    }

    objects[y_pos][x_pos] = false;
    objects[y_t][x_t] = true;
    true
}

fn try_push_p2(
    walls: &Vec<Vec<bool>>,
    objects: &mut Vec<Vec<usize>>,
    y_pos: usize,
    x_pos: usize,
    dir: [isize; 2],
    ch: char,
    by_robot: bool,
) -> bool {
    let mut y_t = (y_pos as isize + dir[0]) as usize;
    let mut x_t = (x_pos as isize + dir[1]) as usize;

    if walls[y_t][x_t] {
        return false;
    }

    if ch == 'v' || ch == '^' {
        let mut x_pos_left = 0;
        let mut x_pos_right = 0;

        if objects[y_pos][x_pos] == 2 {
            x_pos_left = x_pos - 1;
            x_pos_right = x_pos;
        } else {
            x_pos_left = x_pos;
            x_pos_right = x_pos + 1;
        }

        if can_push_p2(walls, objects, y_pos, x_pos, dir, ch) {
            if objects[y_t][x_pos_left] != 0 {
                if !try_push_p2(walls, objects, y_t, x_pos_left, dir, ch, false){
                    println!("problem")
                };
            }

            if objects[y_t][x_pos_right] != 0 {
                if !try_push_p2(walls, objects, y_t, x_pos_right, dir, ch, false) {
                    println!("problem")
                };
            }
            objects[y_pos][x_pos_left] = 0;
            objects[y_pos][x_pos_right] = 0;

            objects[y_t][x_pos_left] = 1;
            objects[y_t][x_pos_right] = 2;
            return true;
        }
    } else if ch == '<' {
        let x_to_move = (x_t as isize + dir[1]) as usize;
        if walls[y_t][x_to_move] {
            return false;
        }
        if objects[y_t][x_to_move] == 0 || try_push_p2(walls, objects, y_t, x_to_move, dir, ch, false) {
            objects[y_pos][x_pos] = 0;
            objects[y_t][x_to_move] = 1;
            objects[y_t][x_t] = 2;

            return true;
        }
    } else if ch == '>' {
        let x_to_move = (x_t as isize + dir[1]) as usize;
        if walls[y_t][x_to_move] {
            return false;
        }
        if objects[y_t][x_to_move] == 0 || try_push_p2(walls, objects, y_t, x_to_move, dir, ch, false) {
            objects[y_pos][x_pos] = 0;
            objects[y_t][x_to_move] = 2;
            objects[y_t][x_t] = 1;
            return true;
        }
    }

    false
}

fn can_push_p2(
    walls: &Vec<Vec<bool>>,
    objects: &mut Vec<Vec<usize>>,
    y_pos: usize,
    x_pos: usize,
    dir: [isize; 2],
    ch: char,
) -> bool {
    let mut y_t = (y_pos as isize + dir[0]) as usize;
    let mut x_t = (x_pos as isize + dir[1]) as usize;

    if walls[y_t][x_t] {
        return false;
    }

    if ch == 'v' || ch == '^' {
        let mut x_pos_left = 0;
        let mut x_pos_right = 0;

        if objects[y_pos][x_pos] == 2 {
            x_pos_left = x_pos - 1;
            x_pos_right = x_pos;
        } else {
            x_pos_left = x_pos;
            x_pos_right = x_pos + 1;
        }


        if walls[y_t][x_pos_left] || walls[y_t][x_pos_right] {
            return false;
        }

        return (objects[y_t][x_pos_left] == 0
            || can_push_p2(walls, objects, y_t, x_pos_left, dir, ch))
            && (objects[y_t][x_pos_right] == 0
                || can_push_p2(walls, objects, y_t, x_pos_right, dir, ch));
    }

    true
}

fn get_dir(dir: char) -> [isize; 2] {
    if dir == '^' {
        return [-1, 0];
    }
    if dir == 'v' {
        return [1, 0];
    }
    if dir == '<' {
        return [0, -1];
    }
    if dir == '>' {
        return [0, 1];
    }
    [0, 0]
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

pub fn setup_usize(y_len: usize, x_len: usize, final_pos: &mut Vec<Vec<usize>>) {
    for i in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        for j in 0..x_len {
            row.push(0);
        }
        final_pos.push(row);
    }
}

fn get_gps(objects: &Vec<Vec<bool>>) -> usize {
    print(&objects);
    let mut result = 0;

    let y_len = objects.len();
    let x_len = objects[0].len();

    for y in 0..y_len {
        for x in 0..x_len {
            if objects[y][x] {
                result += (100 * y) + x;
            }
        }
    }
    result
}

fn get_gps_p2(objects: &Vec<Vec<usize>>) -> usize {
    for i in 0..objects.len() {
        let vec = &objects[i];
        println!("{:?}", vec)
    }
    let mut result = 0;

    let y_len = objects.len();
    let x_len = objects[0].len();

    for y in 0..y_len {
        for x in 0..x_len {
            if objects[y][x] == 1 {
                result += (100 * y) + x;
            }
        }
    }
    result
}

fn print(rows: &Vec<Vec<bool>>) {
    for i in 0..rows.len() {
        let vec = &rows[i];
        println!("{:?}", vec)
    }
}
