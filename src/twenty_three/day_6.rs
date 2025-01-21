use crate::solve::Solve;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (mut times, mut distances) = parse(input);
        let mut p1 = 1;

        for i in 0..times.len(){
            let time = times[i];
            let distance = distances[i];
            let mut wins = 0;
            for x in 1..time -1{
                let speed = x;
                let time_remaining = time - x;
                let traveled = speed * time_remaining;
                if traveled > distance {
                    wins += 1;
                }
            }
            if wins > 0 {
                p1 *= wins;
            }
        }

        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let (mut times, mut distances) = parse_p2(input);
        let mut p1 = 1;

        for i in 0..times.len(){
            let time = times[i];
            let distance = distances[i];
            let mut wins = 0;
            for x in 1..time -1{
                let speed = x;
                let time_remaining = time - x;
                let traveled = speed * time_remaining;
                if traveled > distance {
                    wins += 1;
                }
            }
            if wins > 0 {
                p1 *= wins;
            }
        }

        p1.to_string()
    }
}

fn parse(input: &String) -> (Vec<usize>, Vec<usize>) {
    let mut time: Vec<usize> = vec![];
    let mut distance: Vec<usize> = vec![];
    let lines: Vec<&str> = input.lines().collect();
    time = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim_end()
        .trim_start()
        .split_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
    distance = lines[1].split(":").collect::<Vec<&str>>()[1]
        .trim_end()
        .trim_start()
        .split_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
    (time, distance)
}


fn parse_p2(input: &String) -> (Vec<usize>, Vec<usize>) {
    let mut time: Vec<usize> = vec![];
    let mut distance: Vec<usize> = vec![];
    let lines: Vec<&str> = input.lines().collect();
    time = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim_end()
        .trim_start()
        .replace(" ","")
        .split_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
    distance = lines[1].split(":").collect::<Vec<&str>>()[1]
        .trim_end()
        .trim_start()
        .replace(" ","")
        .split_whitespace()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
    (time, distance)
}

#[test]
fn solve_2023_6() {
    let contents = fs::read_to_string("./resources/twenty_three/day_6.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_6 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_6 {p2_res}");
}
