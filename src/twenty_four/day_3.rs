use regex::Regex;
use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let regex = Regex::new(r"mul[(]{1,1}(?<a>[\d]{1,3}),(?<b>[\d]{1,3})[)]{1,1}").unwrap();
    {
        let matches = regex.captures_iter(contents.as_ref());

        let mut valid = 0;
        matches.for_each(|capture| {
            let a: i32 = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b: i32 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
            valid += a * b;
        });
        println!("Are you winning {valid}");
    }


    {
        let mut valid_do = 0;
        let split = contents.split("do()");
        split.for_each(|do_start| {
            let first_valid = do_start.split("don't");
            let donts: Vec<&str> = first_valid.collect();
            if donts.len() > 0 {
                let x = donts[0];
                let matches = regex.captures_iter(x);

                matches.for_each(|capture| {
                    let a: i32 = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let b: i32 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    valid_do += a * b;
                });
            }

        });
        println!("Are you winning {valid_do}");
    }
}
