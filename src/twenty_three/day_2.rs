use crate::solve::Solve;
use std::collections::HashMap;
use std::fs;

struct Solver {}

impl Solve for Solver {
    fn p1(input: &String) -> String {
        let map = parse(input);
        let mut p1 = 0;

        let r_max = 12;
        let g_max = 13;
        let b_max = 14;

        map.iter().for_each(|(key, value)| {
            let mut valid = true;

            value.iter().for_each(|rgb: &RGB| {
                if rgb.r > r_max || rgb.b > b_max || rgb.g > g_max {
                    valid = false;
                }
            });

            if valid {
                p1 += key;
            }
        });

        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let map = parse(input);
        let mut p2 = 0;

        map.iter().for_each(|(key, value)| {
            let mut r_max = 0;
            let mut g_max = 0;
            let mut b_max = 0;

            value.iter().for_each(|rgb: &RGB| {
                if rgb.r > r_max {
                    r_max = rgb.r;
                }
                if rgb.b > b_max {
                    b_max = rgb.b;
                }
                if rgb.g > g_max {
                    g_max = rgb.g;
                }
            });
            p2 += r_max * g_max * b_max;
        });

        p2.to_string()
    }
}

struct RGB {
    r: usize,
    g: usize,
    b: usize,
}

fn parse(input: &String) -> HashMap<usize, Vec<RGB>> {
    let mut map: HashMap<usize, Vec<RGB>> = HashMap::new();
    input.split("\n").for_each(|line| {
        let split_line: Vec<&str> = line.split(":").collect();
        let first: Vec<&str> = split_line[0].split(" ").collect();
        let key: usize = first[1].parse().unwrap();

        let mut val = vec![];
        let games = split_line[1].split(";");

        games.for_each(|game| {
            let colors: Vec<&str> = game.split(",").collect();
            let mut rgb = RGB { r: 0, g: 0, b: 0 };
            colors.iter().for_each(|color| {
                let value: Vec<&str> = color.trim_end().trim_start().split(" ").collect();
                match value[1] {
                    "blue" => {
                        rgb.b = value[0].parse().unwrap();
                    }
                    "green" => {
                        rgb.g = value[0].parse().unwrap();
                    }
                    "red" => {
                        rgb.r = value[0].parse().unwrap();
                    }
                    _ => {}
                }
            });
            val.push(rgb);
        });
        map.insert(key, val);
    });
    map
}

#[test]
fn solve_2023_2() {
    let contents = fs::read_to_string("./resources/twenty_three/day_2.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_2 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_2 P2 {p2_res}");
}
#[test]
fn test() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_2 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_2 P2 {p2_res}");
}
