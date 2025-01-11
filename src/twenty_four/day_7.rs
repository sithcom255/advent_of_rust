use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut lines: Vec<&str> = contents.split("\n").collect();
    let mut p1_res = 0;
    let mut p2_res = 0;

    for y in 0..lines.len() {
        let line = lines[y];
        let parts: Vec<&str> = line.split(":").collect();
        let total: usize = parts[0].parse().expect("Did not manage to parse");
        let numbers: Vec<usize> = parts[1]
            .trim_start()
            .trim_end()
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        if try_calculate(&numbers, 1, numbers[0], total) {
            p1_res += total;
        }
    }

    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}

fn try_calculate(numbers: &Vec<usize>, i: usize, so_far: usize, total: usize) -> bool {
    if i == numbers.len() {
        return so_far == total;
    }
    let number = numbers[i];
    return try_calculate(numbers, i + 1, so_far + number, total)
        || try_calculate(numbers, i + 1, so_far * number, total)
        || try_calculate(numbers, i + 1, (so_far.to_string() + &*number.to_string()).parse().unwrap(), total);
}
