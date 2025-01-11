use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut lines: Vec<&str> = contents.split("\n").collect();
    let mut characters: Vec<Vec<char>> = Vec::with_capacity(lines.len());

    lines.iter().for_each(|line| {
        characters.push(line.chars().collect());
    });

    // println!("Are you winning son {:?}", characters);
    let mut valid: i32 = 0;

    for x in 0..characters.len() {
        for y in 0..characters[0].len() {
            valid += check_xmas(&characters, x, y);
        }
    }
    println!("Number of valid {valid}");

    let mut x_max_valid = 0;
    for x in 0..characters.len() {
        for y in 0..characters[0].len() {
            x_max_valid += check_x_mas(&characters, x, y);
        }
    }
    println!("Number of x_max_valid {x_max_valid}");
}

pub fn check_xmas(characters: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut valid: i32 = 0;
    if characters[x][y] == 'X' {
        for x_inc in [-1, 0, 1] {
            for y_inc in [-1, 0, 1] {
                if check_with_inc(characters, x, y, x_inc as isize, y_inc as isize) {
                    valid += 1;
                }
            }
        }
    }
    valid
}

pub fn check_with_inc(
    characters: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    x_inc: isize,
    y_inc: isize,
) -> bool {
    let chars: Vec<char> = vec!['X', 'M', 'A', 'S'];
    for i in 0isize..4 {
        let expected: char = chars[i as usize];
        let x_final = (x as isize + (i * x_inc)) as usize;
        let y_final = (y as isize + (i * y_inc)) as usize;
        if !(0 <= x_final && x_final < characters[0].len())
            || !(0 <= y_final && y_final < characters.len())
        {
            return false;
        }
        if expected != characters[x_final][y_final] {
            return false;
        }
    }
    true
}

pub fn check_x_mas(characters: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut valid: i32 = 0;
    if characters[x][y] == 'A' {
        if (check_with_inc_x_mas(characters, x, y, -1, -1) || check_with_inc_x_mas(characters, x, y, 1, 1))
            && (check_with_inc_x_mas(characters, x, y, -1, 1) ||check_with_inc_x_mas(characters, x, y, 1, -1) )
        {
            valid += 1;
        }
    }
    valid
}

pub fn check_with_inc_x_mas(
    characters: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    x_inc: isize,
    y_inc: isize,
) -> bool {
    let chars: Vec<char> = vec!['M', 'A', 'S'];
    for i in -1isize..2 {
        let expected: char = chars[(i + 1) as usize];
        let x_final = (x as isize + (i * x_inc)) as usize;
        let y_final = (y as isize + (i * y_inc)) as usize;
        if !(0 <= x_final && x_final < characters[0].len())
            || !(0 <= y_final && y_final < characters.len())
        {
            return false;
        }
        if expected != characters[x_final][y_final] {
            return false;
        }
    }
    true
}
