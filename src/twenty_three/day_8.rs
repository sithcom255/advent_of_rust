use crate::solve::Solve;
use std::collections::HashMap;
use std::fs;
use std::net::Incoming;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (steps, maps) = parse(input);
        let mut p1 = 1;

        let mut current = maps.get("AAA").unwrap();
        'outer: loop {
            for ch in steps.chars() {
                if ch == 'L' {
                    if current.0 == "ZZZ" {
                        break 'outer;
                    }
                    current = maps.get(current.0).unwrap();
                } else {
                    if current.1 == "ZZZ" {
                        break 'outer;
                    }
                    current = maps.get(current.1).unwrap();
                }
                p1 += 1;
            }
        }

        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let (dirs, maps) = parse(input);
        let mut steps = 1;

        let mut steppers: Vec<Stepper> = vec![];
let mut i = 0;
        for (kev, value) in &maps {
            if kev.ends_with("A") {
                steppers.push(Stepper {
                    id: i,
                    chars: dirs.to_owned().to_string().chars().collect(),
                    index: 0,
                    me: kev,
                    current: &value,
                });
                i+=1;
            }
        }
        let size = *&steppers.len();
        loop {
            let mut end = true;

            for stepper in steppers.iter_mut() {
                let res = stepper.next(&maps);
                if res {
                    // check out of this and do lcm
                    println!("{} steps {} \n",stepper.id, steps);
                }
                end = res & end;
            }

            if end {
                break;
            }

            if(steps == 100000) {
                break
            }

            steps += 1;
        }

        steps.to_string()
    }
}

struct Stepper<'a> {
    id:usize,
    chars: Vec<char>,
    me: &'a str,
    index: usize,
    current: &'a (&'a str, &'a str),
}

impl<'a> Stepper<'a> {
    fn next(&'_ mut self, map: &'a HashMap<&'a str, (&'a str, &'a str)>) -> bool {
        let ch = self.chars[self.index];
        if ch == 'L' {
            self.me = self.current.0;
            self.current = map.get(self.current.0).unwrap();
        } else {
            self.me = self.current.1;
            self.current = map.get(self.current.1).unwrap();
        }

        self.index += 1;
        if self.index == self.chars.len() {
            self.index = 0;
        }

        self.me.ends_with("Z")
    }
}

fn parse(input: &String) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut map = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();

    for i in 2..lines.len() {
        let line = lines[i];
        let lr: Vec<&str> = line.split("=").collect();
        let lef_right: Vec<&str> = lr[1]
            .trim_end()
            .trim_start()
            .strip_suffix(")")
            .unwrap()
            .strip_prefix("(")
            .unwrap()
            .split(",")
            .map(|elem| elem.trim_end().trim_start())
            .collect();
        map.insert(lr[0].trim_end().trim_start(), (lef_right[0], lef_right[1]));
    }
    (lines[0], map)
}

#[test]
fn solve_2023_8() {
    let contents = fs::read_to_string("./resources/twenty_three/day_8.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_8 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_8 P2 {p2_res}");
}
