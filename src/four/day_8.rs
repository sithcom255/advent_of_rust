use std::collections::HashMap;
use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut lines: Vec<Vec<char>> = contents
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut p1_res = 0;
    let mut p2_res = 0;

    let y_len = lines.len();
    let x_len = lines[0].len();

    let mut antennas: HashMap<char, Vec<[usize; 2]>> = HashMap::new();
    let mut anti: Vec<Vec<bool>> = Vec::with_capacity(y_len);
    let mut anti_p2: Vec<Vec<bool>> = Vec::with_capacity(y_len);

    for y in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        let mut row_p2 = Vec::with_capacity(x_len);
        for x in 0..x_len {
            row.push(false);
            row_p2.push(false);
        }
        anti.push(row);
        anti_p2.push(row_p2);
    }

    for y in 0..y_len {
        for x in 0..x_len {
            let ch = lines[y][x];
            if ch != '.' && ch != '#' {
                let contains = antennas.contains_key(&ch);
                if contains {
                    let result = antennas.get_mut(&ch).unwrap();
                    for i in 0..result.len() {
                        let previous_antenna = result[i];
                        p1_res += calculate_anti(previous_antenna, y, x, &mut anti, y_len, x_len);
                        p2_res += calculate_anti_p2(previous_antenna, y, x, &mut anti_p2, y_len, x_len);
                    }
                    result.push([y, x]);
                } else {
                    antennas.insert(ch,vec!([y, x]));
                }
            }
        }
    }

    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}

fn calculate_anti(
    other: [usize; 2],
    y: usize,
    x: usize,
    anti: &mut Vec<Vec<bool>>,
    y_len: usize,
    x_len: usize,
) -> usize {
    let mut res = 0;
    let mut y_o = other[0];
    let mut x_o = other[1];

    let y_diff = y_o as isize - y as isize;
    let x_diff = x_o as isize - x as isize;

    {
        let a_y = y as isize - y_diff;
        let a_x = x  as isize - x_diff;
        if (0 <= a_x && a_x < x_len as isize ) && (0 <= a_y && a_y < y_len as isize ) {
            if anti[a_y as usize][a_x as usize] {
            } else {
                anti[a_y as usize][a_x as usize] = true;
                res += 1;
            }
        }
    }

    {
        let a_y = y as isize + (2 * y_diff);
        let a_x = x as isize + (2 * x_diff);
        if (0 <= a_x && a_x < x_len as isize ) && (0 <= a_y && a_y < y_len as isize) {
            if anti[a_y as usize][a_x as usize] {
            } else {
                anti[a_y as usize][a_x as usize] = true;
                res += 1;
            }
        }
    }

    return res;
}
fn calculate_anti_p2(
    other: [usize; 2],
    y: usize,
    x: usize,
    anti: &mut Vec<Vec<bool>>,
    y_len: usize,
    x_len: usize,
) -> usize {
    let mut res = 0;
    let mut y_o = other[0];
    let mut x_o = other[1];

    let y_diff = y_o as isize - y as isize;
    let x_diff = x_o as isize - x as isize;

    {
        if anti[y][x] {
        } else {
            anti[y][x] = true;
            res += 1;
        }

        if anti[y_o][x_o] {
        } else {
            anti[y_o][x_o] = true;
            res += 1;
        }
    }
    {
        let mut a_y = y as isize - y_diff;
        let mut a_x = x as isize - x_diff;
        while (0 <= a_x && a_x < x_len as isize) && (0 <= a_y && a_y < y_len as isize) {
            if anti[a_y as usize][a_x as usize] {
            } else {
                anti[a_y as usize][a_x as usize] = true;
                res += 1;
            }
            a_y = a_y - y_diff;
            a_x = a_x - x_diff;
        }
    }

    {
        let mut a_y = y as isize + (2 * y_diff);
        let mut a_x = x as isize + (2 * x_diff);
        while (0 <= a_x && a_x < x_len as isize) && (0 <= a_y && a_y < y_len as isize) {
            if anti[a_y as usize][a_x as usize] {
            } else {
                anti[a_y as usize][a_x as usize] = true;
                res += 1;
            }
            a_y = a_y + (y_diff);
            a_x = a_x + (x_diff);
        }
    }

    return res;
}
