use std::collections::{HashMap, HashSet};
use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut lines: Vec<&str> = contents.split("\n").collect();
    let mut valid_result = 0;
    let mut fixed_result = 0;
    let mut rules_finished = false;

    let mut reverse_rules: HashMap<isize, HashSet<isize>> = HashMap::new();
    let mut rules: HashMap<isize, HashSet<isize>> = HashMap::new();

    lines.iter().for_each(|line| {
        if line.is_empty() {
            rules_finished = true;
            return;
        }

        if rules_finished {
            let numbers: Vec<isize> = line.split(",").map(|num| num.parse().unwrap()).collect();

            let mut valid = true;
            {
                let mut todo: HashSet<isize> = numbers.iter().map(|e| *e).collect();
                let mut done: HashMap<isize, bool> = HashMap::new();

                if numbers.len() != todo.len() {
                    return;
                }

                for i in 0..numbers.len() {
                    valid &= validate_number(numbers[i], &reverse_rules, &mut todo, &mut done);
                    todo.remove(&numbers[i]);
                }
            }

            if valid {
                valid_result += numbers[numbers.len() / 2];
            } else {
                let mut todo: HashSet<isize> = numbers.iter().map(|e| *e).collect();
                let mut done: HashSet<isize> = HashSet::new();
                let mut correct_order: Vec<isize> = Vec::with_capacity(numbers.len());

                for i in 0..numbers.len() {
                    let number = numbers[i];
                    resolve_following(number, &rules, &mut todo, &mut done, &mut correct_order);
                }
                correct_order.reverse();
                println!("Fixed{:?}", correct_order);
                fixed_result += correct_order[correct_order.len() / 2];
            }
        } else {
            let split: Vec<isize> = line.split("|").map(|num| num.parse().unwrap()).collect();
            {
                let contains = reverse_rules.get_mut(&split[1]);

                if contains.is_none() {
                    let mut new_set = HashSet::new();
                    new_set.insert(split[0]);
                    reverse_rules.insert(split[1], new_set);
                } else {
                    let mut set = contains.unwrap();
                    set.insert(split[0]);
                }
            }
            {
                let contains = rules.get_mut(&split[0]);

                if contains.is_none() {
                    let mut new_set = HashSet::new();
                    new_set.insert(split[1]);
                    rules.insert(split[0], new_set);
                } else {
                    let mut set = contains.unwrap();
                    set.insert(split[1]);
                }
            }
        }
    });
    println!("Result {}", valid_result);
    println!("Fixed_Result {}", fixed_result);
}

fn resolve_following(
    number: isize,
    rules: &HashMap<isize, HashSet<isize>>,
    todo: &mut HashSet<isize>,
    done: &mut HashSet<isize>,
    correct_order_rev: &mut Vec<isize>,
) {
    if !todo.contains(&number) {
        return;
    }

    if done.contains(&number) {
        return;
    }

    let option = rules.get(&number);

    if option.is_none() {
        correct_order_rev.push(number);
        done.insert(number);
        return;
    }

    option.unwrap().iter().for_each(|following| {
        resolve_following(*following, rules, todo, done, correct_order_rev);
    });

    correct_order_rev.push(number);

    done.insert(number);
}

fn validate_number(
    number: isize,
    rules: &HashMap<isize, HashSet<isize>>,
    todo: &mut HashSet<isize>,
    done: &mut HashMap<isize, bool>,
) -> bool {
    if done.contains_key(&number) {
        return *done.get(&number).unwrap();
    }

    let number_rules = rules.get(&number);

    if number_rules.is_none() {
        done.insert(number, true);
        return true;
    }

    let mut this_valid = true;
    number_rules.unwrap().iter().for_each(|prerequisite| {
        this_valid &= !todo.contains(prerequisite);
    });

    done.insert(number, this_valid);

    return this_valid;
}
