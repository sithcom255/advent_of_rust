use regex::Regex;
use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let mut p1_res = 0;
    let mut p1_res_f = 0f64;
    let mut p2_res: usize = 0;

    let regex = Regex::new(r"X\+([\d]+), Y\+([\d]+)").unwrap();
    let target_r = Regex::new(r"X=([\d]+), Y=([\d]+)").unwrap();

    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut rows: Vec<&str> = contents.split("\n").collect();
    let mut i = 0;
    while i < rows.len() {
        let value: &str = rows[i].split(":").collect::<Vec<&str>>()[1];
        let value_b: &str = rows[i + 1].split(":").collect::<Vec<&str>>()[1];
        let target: &str = rows[i + 2].split(":").collect::<Vec<&str>>()[1];

        let mut a = parse_button(&regex, value);
        let mut b = parse_button(&regex, value_b);
        let mut target = parse_target(&target_r, target);

        // let mut a_c = (target[0] as isize - ((a[0] as isize * target[1] as isize) / b[0] as isize))
        //     / ((-(b[1] as isize) * a[0] as isize) / a[0] as isize + b[0] as isize);
        let mut a_c = (target[0] as f64 - target[1] as f64 * a[0] as f64 / a[1] as f64)
            / ((-(b[1] as f64) * a[0] as f64 / a[1] as f64) + b[0] as f64);

        if !(a_c.fract() < 0.1f64 || a_c.fract() > 0.9f64) {
            i += 4;
            continue;
        }
        let mut b_c = target[1] as f64 / a[1] as f64 - ((a_c * b[1] as f64) / a[1] as f64);
        let result = a_c + b_c * 3f64;
        if result.fract() < 0.1f64 || result.fract() > 0.9f64 {
            p1_res_f = p1_res_f + result;
        }

        i += 4;
    }

    println!("P1 res {p1_res_f}");
    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}

// a * 3 + b * 1 = 0
// x * a[0] + y * b[0] = t[0]
// t[0] - t[1] * a[0] / a[1] / ( - ( b[2] * a[0] / a[1]) +  b[0] )

// x * a[1] + y * b[2] = t[1]
// x = t[1] / a[1] - (y * b[2] / a[1])

// a = 5400 / 22 - b * 67
// - b * 67 * 94 / 34 + b * 22 = (8400 - 5400 * 94 / 22) / ( - b * 67 * 94 / 22 + b * 22 )

fn is_solved(a: [usize; 2], b: [usize; 2], a_c: usize, b_c: usize, target: [usize; 2]) -> bool {
    return a_c * a[0] + b_c * b[0] == target[0] && a_c * a[1] + b_c * b[1] == target[1];
}

fn parse_button(regex: &Regex, value: &str) -> [usize; 2] {
    let matches = regex.captures_iter(value.trim_end().trim_start().as_ref());

    let mut a = [0, 0];
    matches.for_each(|capture| {
        a[0] = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
        a[1] = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
    });

    a
}

fn parse_target(regex: &Regex, value: &str) -> [usize; 2] {
    let matches = regex.captures_iter(value.trim_end().trim_start().as_ref());

    let mut a = [0, 0];
    matches.for_each(|capture| {
        a[0] = 10000000000000 + capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
        a[1] = 10000000000000 + capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
    });

    a
}
