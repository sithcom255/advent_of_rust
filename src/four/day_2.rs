use std::fs;
pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut rows = contents.split("\n");

    let mut valid = 0;
    let mut valid_two2 = 0;

    rows.for_each(|line| {
        let numbers: Vec<isize> = line
            .split_whitespace()
            .map(|elem| return elem.parse::<isize>().unwrap())
            .collect();

        if numbers.len() <= 1 {
            valid += 1;
            return;
        }


        if check_valid(&numbers) {
            valid_two2 += 1;
        } else {
            for remove in 0..numbers.len()  {
                let vec_without_i = get_vec_without(&numbers, &remove);
                if (check_valid(&vec_without_i)) {
                    valid_two2 += 1;
                    break;
                }
            }
        }

        valid += 1;
    });

    println!("Valid lines {valid} and {valid_two2}")
}

fn get_vec_without(numbers: &Vec<isize>, i: &usize) -> Vec<isize> {
    let mut vec: Vec<isize> = Vec::with_capacity(numbers.len());
    for elem in 0..numbers.len() {
        let i1 = numbers[elem];
        vec.push(i1);
    }

    vec.remove(*i);
    vec
}

fn check_valid(numbers: &Vec<isize>) -> bool {
    let mut is_increasing: bool = numbers[0] < numbers[1];
    for i in 0..(numbers.len() - 1) {
        if (is_increasing && numbers[i] >= numbers[i + 1])
            || (!is_increasing && numbers[i] <= numbers[i + 1])
        {
            return false;
        }

        let mut distance = numbers[i] - numbers[i + 1];
        if distance < 0 {
            distance = -distance;
        }
        if !(1 <= distance && distance <= 3) {
            return false;
        }
    }
    true
}
