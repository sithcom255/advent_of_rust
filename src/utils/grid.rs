pub fn bound_check(y: isize, x: isize, y_len: usize, x_len: usize) -> bool {
    (0 <= y && y < y_len as isize) && (0 <= x && x < x_len as isize)
}

pub fn get_next(dir: usize, y: usize, x: usize) -> (isize, isize) {
    match dir {
        0 => return (y as isize - 1, x as isize),
        1 => return (y as isize, x as isize + 1),
        2 => return (y as isize + 1, x as isize),
        3 => return (y as isize, x as isize - 1),
        _ => {
            unreachable!("Dir {} - Y: {} X: {}", dir, y, x)
        }
    }
}
