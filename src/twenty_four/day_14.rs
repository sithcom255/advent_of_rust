use regex::Regex;
use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let mut p1_res = 0;
    let mut p2_res: usize = 0;

    let regex = Regex::new(r"p=([\d]+),([\d]+) v=(-?[\d]+),(-?[\d]+)").unwrap();

    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut rows: Vec<&str> = contents.split("\n").collect();

    let y_len = 103;
    let x_len = 101;
    let mut final_pos: Vec<Vec<usize>> = Vec::with_capacity(y_len);
    setup(y_len, x_len, &mut final_pos);

    for i in 0..10000 {
        rows.iter().for_each(|line| {
            let target = parse_target(&regex, line);

            let iter = get_pos_after_iter(
                target[1],
                target[0],
                target[3],
                target[2],
                y_len as isize,
                x_len as isize,
                i,
            );
            final_pos[iter[0]][iter[1]] += 1;
        });
        p1_res = get_security_2(&final_pos, y_len, x_len, i as usize);

        final_pos.clear();
        setup(y_len, x_len, &mut final_pos);
    }

    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}

fn setup(y_len: usize, x_len: usize, final_pos: &mut Vec<Vec<usize>>) {
    for i in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        for j in 0..x_len {
            row.push(0);
        }
        final_pos.push(row);
    }
}

pub fn get_pos_after_iter(
    y: isize,
    x: isize,
    y_one_step: isize,
    x_one_step: isize,
    y_len: isize,
    x_len: isize,
    n: isize,
) -> [usize; 2] {
    let mut y_c = (y + (n * y_one_step)) % y_len;
    let mut x_c = (x + (n * x_one_step)) % x_len;

    if y_c < 0 {
        y_c = y_len + y_c;
    }

    if x_c < 0 {
        x_c = x_len + x_c;
    }
    [y_c as usize, x_c as usize]
}

fn get_security(result: &Vec<Vec<usize>>, y_len: usize, x_len: usize, n: usize) -> usize {
    let x_start = 0;
    let x_end = x_len / 2;

    let y_start = 0;
    let y_end = y_len / 2;

    let mut a = get_count(result, x_start, x_end, y_start, y_end);
    let mut b = get_count(result, x_end + 1, x_len, y_start, y_end);
    let mut c = get_count(result, x_start, x_end, y_end + 1, y_len);
    let mut d = get_count(result, x_end + 1, x_len, y_end + 1, y_len);

    if a == b && c == d {
        println!("Same {n}");
        print(&result);
        println!("\n \n");
    }
    return a;
}

fn get_security_2(result: &Vec<Vec<usize>>, y_len: usize, x_len: usize, n: usize) -> usize {
    let x_start = 0;
    let x_end = x_len / 2;

    let y_start = 0;
    let y_end = y_len / 2;

    let mut a = get_count(result, x_start, x_end, y_start, y_len);
    let mut b = get_count(result, x_end + 1, x_len, y_start, y_len);

    let mut c = get_count(result, x_start, x_len, y_start, y_end);
    let mut d = get_count(result, x_start, x_len, y_end + 1, y_len);

    if print(&result) {
        println!("Same {n}");
    };

    return a;
}

fn get_count(
    result: &Vec<Vec<usize>>,
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
) -> usize {
    let mut count = 0;
    for y in y_start..y_end {
        for x in x_start..x_end {
            count += result[y][x];
        }
    }
    count
}

fn parse_target(regex: &Regex, value: &str) -> [isize; 4] {
    let matches = regex.captures_iter(value.trim_end().trim_start().as_ref());

    let mut a = [0, 0, 0, 0];
    matches.for_each(|capture| {
        a[0] = capture.get(1).unwrap().as_str().parse::<isize>().unwrap();
        a[1] = capture.get(2).unwrap().as_str().parse::<isize>().unwrap();
        a[2] = capture.get(3).unwrap().as_str().parse::<isize>().unwrap();
        a[3] = capture.get(4).unwrap().as_str().parse::<isize>().unwrap();
    });

    a
}

fn print(rows: &Vec<Vec<usize>>) -> bool {
    let mut print = false;
    let mut str: Vec<String> = Vec::with_capacity(110);
    for i in 0..rows.len() {
        let row = &rows[i];
        let mut s = "".to_owned();
        for j in 0..row.len() {
            if row[j] == 0 {
                s.push_str(".")
            } else {
                s.push_str("#")
            }
        }
        print |= s.contains("#######");
        str.push(s);
    }
    if print {
        str.iter().for_each(|line| println!("{line}"))
    }
    return print;
}
