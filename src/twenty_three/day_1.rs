use crate::solve::Solve;
use std::fs;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut p1 = 0;
        input.split("\n").for_each(|line| {
            let mut has_first = false;
            let mut first = 0;
            let mut last = 0;
            line.chars().for_each(|ch| {
                if ch.is_digit(10) {
                    if has_first {
                        last = ch.to_digit(10).unwrap();
                    } else {
                        first = ch.to_digit(10).unwrap();
                        last = ch.to_digit(10).unwrap();
                        has_first = true;
                    }
                }
            });
            p1 += first * 10 + last;
        });
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let mut p2 = 0;
        input.split("\n").for_each(|line| {
            let mut has_first = false;
            let mut first = 0;
            let mut last = 0;

            let chars: Vec<char> = line.chars().collect();
            for i in 0..chars.len() {
                let ch = chars[i];
                if ch.is_digit(10) {
                    if has_first {
                        last = ch.to_digit(10).unwrap();
                    } else {
                        first = ch.to_digit(10).unwrap();
                        last = ch.to_digit(10).unwrap();
                        has_first = true;
                    }
                } else {
                    let x = &line[i..chars.len()];
                    let mut numb = -1i32;
                    if x.starts_with("one") {
                        numb = 1;
                    } else if x.starts_with("two") {
                        numb = 2;
                    } else if x.starts_with("three") {
                        numb = 3;
                    } else if x.starts_with("four") {
                        numb = 4;
                    } else if x.starts_with("five") {
                        numb = 5;
                    } else if x.starts_with("six") {
                        numb = 6;
                    } else if x.starts_with("seven") {
                        numb = 7;
                    } else if x.starts_with("eight") {
                        numb = 8;
                    } else if x.starts_with("nine") {
                        numb = 9;
                    }

                    if numb != -1 {
                        if has_first {
                            last = numb as u32;
                        } else {
                            first = numb as u32;
                            last = numb as u32;
                            has_first = true;
                        }
                    }
                }
            }
            p2 += first * 10 + last;
        });
        p2.to_string()
    }
}

#[test]
fn solve_2023_1() {
    let contents = fs::read_to_string("./resources/twenty_three/day_1.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_1 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_2 P2 {p2_res}");
}
