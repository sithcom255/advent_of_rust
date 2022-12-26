use std::env;
use std::fs;

pub fn day_one(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let numbers = contents.split("\n\n");
    let mut max = [0, 0, 0];
    numbers.for_each(|elem: &str| {
        let each_number = elem.split("\n");
        let result: u32 = each_number
            .map(|number: &str| number.parse::<u32>().unwrap())
            .sum();
        for i in 0..3 {
            if max[i] < result {
                max[i] = result;
                break
            }
        }
    });
    let res = max.iter().sum::<u32>() as u32;
    print!("max {} \n",  res)
}
