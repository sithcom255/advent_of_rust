use crate::solve::Solve;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let (commands, parts) = parse(input);
        let mut p1 = 0;

        for part in parts {
            let mut todo = commands.get(&"in".to_owned()).unwrap();

            'outer: loop {
                for command in todo {
                    let predicate = &command.predicate;
                    if predicate(&part) {
                        if command.is_final {
                            if command.accepted {
                                p1 += part.sum();
                            }
                            break 'outer;
                        }

                        todo = commands.get(&command.next.to_owned()).unwrap();
                        continue 'outer;
                    }
                }
                unreachable!()
            }
        }

        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let (commands, parts) = parse(input);
        let mut p1 = 0;

        let mut todo = commands.get(&"in".to_owned()).unwrap();
        let i = walk(
            &commands,
            todo,
            RangePart {
                x_min: 1,
                x_max: 4000,
                m_min: 1,
                m_max: 4000,
                a_min: 1,
                a_max: 4000,
                s_min: 1,
                s_max: 4000,
            },
        );
        i.to_string()
    }
}

fn walk(commands: &HashMap<String, Vec<Command>>, todo: &Vec<Command>, point: RangePart) -> usize {
    let mut res = 0;
    let mut current: RangePart = point;
    for command in todo {
        if command.is_final {
            if command.value == 5000 {
                if command.accepted {
                    res += current.combinations();
                }
            } else {
                if command.accepted {
                    let part = current.with(&command.left, command.value, command.less_then, false);
                    res += part.combinations();
                }
                current = current.with(&command.left, command.value, !command.less_then, true);
            }
        } else {
            let part = current.with(&command.left, command.value, command.less_then, false);
            res += walk(commands, &commands.get(&command.next).unwrap(), part);
            current = current.with(&command.left, command.value, !command.less_then, true);
        }
    }
    res
}

fn parse(input: &String) -> (HashMap<String, Vec<Command>>, Vec<Part>) {
    let mut parts: Vec<Part> = vec![];
    let mut commands: HashMap<String, Vec<Command>> = HashMap::new();
    let mut has_parts = false;

    input.lines().for_each(|line| {
        if line.is_empty() {
            has_parts = true;
            return;
        }

        if !has_parts {
            let id_split: Vec<&str> = line.split("{").collect();
            let raw_commands: Vec<&str> =
                id_split[1].strip_suffix("}").unwrap().split(",").collect();

            let mut todo = vec![];
            for command in raw_commands {
                if command.contains(">") || command.contains("<") {
                    let one: Vec<&str> = command.split(":").collect();
                    let mut is_final: bool = one[1] == "A" || one[1] == "R";
                    let mut accepted: bool = one[1] == "A";
                    if command.contains(">") {
                        let sides: Vec<&str> = one[0].split(">").collect();
                        let value = sides[1].parse::<usize>().unwrap();
                        let predicate: Box<dyn Fn(&Part) -> bool> = match sides[0] {
                            "x" => Box::new(move |part| part.x > value),
                            "m" => Box::new(move |part| part.m > value),
                            "a" => Box::new(move |part| part.a > value),
                            "s" => Box::new(move |part| part.s > value),
                            &_ => {
                                unreachable!()
                            }
                        };
                        todo.push(Command {
                            predicate,
                            is_final,
                            accepted,
                            value,
                            left: sides[0].to_owned(),
                            less_then: false,
                            next: one[1].to_owned(),
                        });
                    } else {
                        let sides: Vec<&str> = one[0].split("<").collect();
                        let value = sides[1].parse::<usize>().unwrap();
                        let predicate: Box<dyn Fn(&Part) -> bool> = match sides[0] {
                            "x" => Box::new(move |part| part.x < value),
                            "m" => Box::new(move |part| part.m < value),
                            "a" => Box::new(move |part| part.a < value),
                            "s" => Box::new(move |part| part.s < value),
                            &_ => {
                                unreachable!()
                            }
                        };
                        todo.push(Command {
                            predicate,
                            is_final,
                            accepted,
                            value,
                            left: sides[0].to_owned(),
                            less_then: true,
                            next: one[1].to_owned(),
                        });
                    }
                } else {
                    let mut is_final: bool = command == "A" || command == "R";
                    let mut accepted: bool = command == "A";
                    todo.push(Command {
                        predicate: Box::new(|_x| true),
                        is_final,
                        accepted,
                        value: 5000,
                        left: "K".to_owned(),
                        less_then: true,
                        next: command.to_owned(),
                    });
                }
            }
            commands.insert(id_split[0].to_owned(), todo);
        } else {
            let raw = line.strip_suffix("}").unwrap().strip_prefix("{").unwrap();
            let nums: Vec<usize> = raw
                .split(",")
                .map(|elem| {
                    let x: Vec<&str> = elem.split("=").collect();
                    return x[1].parse::<usize>().unwrap();
                })
                .collect();
            parts.push(Part {
                x: nums[0],
                m: nums[1],
                a: nums[2],
                s: nums[3],
            })
        }
    });
    (commands, parts)
}

struct Command {
    predicate: Box<dyn Fn(&Part) -> bool>,
    is_final: bool,
    accepted: bool,
    value: usize,
    less_then: bool,
    left: String,
    next: String,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct RangePart {
    x_min: usize,
    x_max: usize,
    m_min: usize,
    m_max: usize,
    a_min: usize,
    a_max: usize,
    s_min: usize,
    s_max: usize,
}

impl Part {
    fn sum(&self) -> usize {
        return self.x + self.m + self.a + self.s;
    }
}

impl RangePart {
    fn combinations(&self) -> usize {
        if !(self.x_max > self.x_min
            && self.a_max > self.a_min
            && self.m_max > self.m_min
            && self.s_max > self.s_min)
        {
            return 0;
        }
        (self.x_max - self.x_min + 1)
            * (self.m_max - self.m_min + 1)
            * (self.a_max - self.a_min + 1)
            * (self.s_max - self.s_min + 1)
    }
    fn with(&self, att: &String, par: usize, less_then: bool, remainder: bool) -> RangePart {

        let less = less_then;
        let mut value = 0;
        if less {
            if remainder {
                value = par;
            } else {
                value = par - 1;
            }
        } else {
            if remainder {
                value = par;
            } else {
                value = par + 1;
            }
        }

        match att.as_str() {
            "x" => {
                if less {
                    RangePart {
                        x_min: self.x_min,
                        x_max: min(self.x_max, value),
                        m_min: self.m_min,
                        m_max: self.m_max,
                        a_min: self.a_min,
                        a_max: self.a_max,
                        s_min: self.s_min,
                        s_max: self.s_max,
                    }
                } else {
                    RangePart {
                        x_min: max(self.x_min, value),
                        x_max: self.x_max,
                        m_min: self.m_min,
                        m_max: self.m_max,
                        a_min: self.a_min,
                        a_max: self.a_max,
                        s_min: self.s_min,
                        s_max: self.s_max,
                    }
                }
            }
            "m" => {
                if less {
                    RangePart {
                        x_min: self.x_min,
                        x_max: self.x_max,
                        m_min: self.m_min,
                        m_max: min(self.m_max, value),
                        a_min: self.a_min,
                        a_max: self.a_max,
                        s_min: self.s_min,
                        s_max: self.s_max,
                    }
                } else {
                    RangePart {
                        x_min: self.x_min,
                        x_max: self.x_max,
                        m_min: max(self.m_min, value),
                        m_max: self.m_max,
                        a_min: self.a_min,
                        a_max: self.a_max,
                        s_min: self.s_min,
                        s_max: self.s_max,
                    }
                }
            }
            "a" => {
                if less {
                    RangePart {
                        x_min: self.x_min,
                        x_max: self.x_max,
                        m_min: self.m_min,
                        m_max: self.m_max,
                        a_min: self.a_min,
                        a_max: min(self.a_max, value),
                        s_min: self.s_min,
                        s_max: self.s_max,
                    }
                } else {
                    RangePart {
                        x_min: self.x_min,
                        x_max: self.x_max,
                        m_min: self.m_min,
                        m_max: self.m_max,
                        a_min: max(self.a_min   , value),
                        a_max: self.a_max,
                        s_min: self.s_min,
                        s_max: self.s_max,
                    }
                }
            }
            "s" => {
                if less {
                    RangePart {
                        x_min: self.x_min,
                        x_max: self.x_max,
                        m_min: self.m_min,
                        m_max: self.m_max,
                        a_min: self.a_min,
                        a_max: self.a_max,
                        s_min: self.s_min,
                        s_max: min(self.s_max, value),
                    }
                } else {
                    RangePart {
                        x_min: self.x_min,
                        x_max: self.x_max,
                        m_min: self.m_min,
                        m_max: self.m_max,
                        a_min: self.a_min,
                        a_max: self.a_max,
                        s_min: max(self.s_min, value),
                        s_max: self.s_max,
                    }
                }
            }
            "K" => RangePart {
                x_min: self.x_min,
                x_max: self.x_max,
                m_min: self.m_min,
                m_max: self.m_max,
                a_min: self.a_min,
                a_max: self.a_max,
                s_min: self.s_min,
                s_max: self.s_max,
            },
            &_ => {
                unreachable!("{}", att)
            }
        }
    }
}



#[test]
fn solve_2023_19() {
    let contents = fs::read_to_string("./resources/twenty_three/day_19.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_19 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_19 {p2_res}");
}
