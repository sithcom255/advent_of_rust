use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let mut p1_res = 0;
    let mut p2_res: usize = 0;

    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut rows: Vec<Vec<char>> = contents
        .split("\n")
        .map(|line| {
            return line.chars().collect();
        })
        .collect();

    let y_len = rows.len();
    let x_len = rows[0].len();
    let mut visited = Vec::with_capacity(rows.len());
    rows.iter().for_each(|row| {
        let mut memo_row = Vec::with_capacity(row.len());
        for i in 0..row.len() {
            memo_row.push(false);
        }
        visited.push(memo_row);
    });

    for y in 0..y_len {
        for x in 0..x_len {
            let ch = rows[y][x];
            {
                let area = expand_grid_area(
                    &mut rows,
                    &mut visited,
                    ch,
                    y as isize,
                    x as isize,
                    y_len,
                    x_len,
                );
                p1_res += area[0] * area[1];
            }
        }
    }

    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}

pub fn bound_check(y: isize, x: isize, y_len: usize, x_len: usize) -> bool {
    (0 <= y && y < y_len as isize) && (0 <= x && x < x_len as isize)
}
pub fn expand_grid_area(
    rows: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    ch: char,
    y: isize,
    x: isize,
    y_len: usize,
    x_len: usize,
) -> [usize; 2] {
    if !bound_check(y, x, y_len, x_len) {
        return [0, 0];
    }

    let y_safe = y as usize;
    let x_safe = x as usize;

    let value = rows[y_safe][x_safe];

    if value != ch {
        return [0, 0];
    }

    if visited[y_safe][x_safe] {
        return [0, 0];
    }

    visited[y_safe][x_safe] = true;

    let mut result = [1, 0];
    {
        result[1] += expand_corners(rows, ch, y - 1, x, y, x + 1, y - 1, x + 1, y_len, x_len);
        result[1] += expand_corners(rows, ch, y + 1, x, y, x + 1, y + 1, x + 1, y_len, x_len);
        result[1] += expand_corners(rows, ch, y - 1, x, y, x - 1, y - 1, x - 1, y_len, x_len);
        result[1] += expand_corners(rows, ch, y + 1, x, y, x - 1, y + 1, x - 1, y_len, x_len);
    }

    {
        let area = expand_grid_area(rows, visited, ch, y + 1, x, y_len, x_len);
        result[0] += area[0];
        result[1] += area[1];
    }

    {
        let area = expand_grid_area(rows, visited, ch, y - 1, x, y_len, x_len);
        result[0] += area[0];
        result[1] += area[1];
    }

    {
        let area = expand_grid_area(rows, visited, ch, y, x + 1, y_len, x_len);
        result[0] += area[0];
        result[1] += area[1];
    }

    {
        let area = expand_grid_area(rows, visited, ch, y, x - 1, y_len, x_len);
        result[0] += area[0];
        result[1] += area[1];
    }

    result
}

pub fn expand_corners(
    rows: &mut Vec<Vec<char>>,
    ch: char,
    y: isize,
    x: isize,
    y_2: isize,
    x_2: isize,
    y_3: isize,
    x_3: isize,
    y_len: usize,
    x_len: usize,
) -> usize {
    if !bound_check(y, x, y_len, x_len) && !bound_check(y_2, x_2, y_len, x_len) {
        return 1;
    }

    if !bound_check(y, x, y_len, x_len) && bound_check(y_2, x_2, y_len, x_len) {
        let y2_safe = y_2 as usize;
        let x2_safe = x_2 as usize;

        let value_2 = rows[y2_safe][x2_safe];
        if value_2 != ch {
            return 1;
        }
        return 0;
    }

    if bound_check(y, x, y_len, x_len) && !bound_check(y_2, x_2, y_len, x_len) {
        let y_safe = y as usize;
        let x_safe = x as usize;

        let value = rows[y_safe][x_safe];
        if value != ch {
            return 1;
        }
        return 0;
    }

    let y_safe = y as usize;
    let x_safe = x as usize;

    let y2_safe = y_2 as usize;
    let x2_safe = x_2 as usize;

    let y3_safe = y_3 as usize;
    let x3_safe = x_3 as usize;

    let value = rows[y_safe][x_safe];
    let value_2 = rows[y2_safe][x2_safe];
    let value_3 = rows[y3_safe][x3_safe];

    if value != ch && value_2 != ch {
        return 1;
    }

    if value == ch && value_2 == ch && value_3 != ch {
        return 1;
    }

    return 0;
}
