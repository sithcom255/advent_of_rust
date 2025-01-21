use crate::solve::Solve;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::fs;
struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let mut rows = parse(input);
        let mut hands: Vec<(HashMap<char, usize>, usize, &str)> = vec![];
        rows.iter().for_each(|(hand, value)| {
            let mut map: HashMap<char, usize> = HashMap::new();
            hand.chars().for_each(|ch| match map.get(&ch) {
                None => {
                    map.insert(ch, 1);
                }
                Some(previos) => {
                    map.insert(ch, previos + 1);
                }
            });
            hands.push((map, *value, hand));
        });

        hands.sort_by(|a, b| sort_hands_p1(&mut a.clone(), &mut b.clone()));

        let mut p1 = 0;
        for i in 0..hands.len() {
            p1 += (i + 1) * hands[i].1;
        }
        p1.to_string()
    }

    fn p2(input: &String) -> String {
        let mut rows = parse(input);
        let mut hands: Vec<(HashMap<char, usize>, usize, &str)> = vec![];
        rows.iter().for_each(|(hand, value)| {
            let mut map: HashMap<char, usize> = HashMap::new();
            let mut js = 0;
            hand.chars().for_each(|ch| match map.get(&ch) {
                None => {
                    if ch == 'J' {
                        js += 1;
                        return;
                    }
                    map.insert(ch, 1);
                }
                Some(previos) => {
                    map.insert(ch, previos + 1);
                }
            });
            if js != 0 {
                let (a_max, a_max_char) = find_highes_p1(&mut (map.clone(), *value, hand));
                map.insert(a_max_char, a_max + js);
            };
            hands.push((map, *value, hand));
        });

        hands.sort_by(|a, b| sort_hands_p2(&mut a.clone(), &mut b.clone()));

        let mut p2 = 0;
        for i in 0..hands.len() {
            p2 += (i + 1) * hands[i].1;
        }
        p2.to_string()
    }
}

fn sort_hands_p1(
    a: &mut (HashMap<char, usize>, usize, &str),
    b: &mut (HashMap<char, usize>, usize, &str),
) -> Ordering {
    if a.0.is_empty() && b.0.is_empty() {
        println!("SAME");
        return Equal;
    }

    let ord = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let (a_max, a_max_char) = find_highes_p1(a);
    let (b_max, b_max_char) = find_highes_p1(b);

    match a_max.cmp(&b_max) {
        Less => Less,
        Equal => {
            a.0.remove(&a_max_char).unwrap();
            b.0.remove(&b_max_char).unwrap();
            let mut ordering;
            if a_max == 3 || a_max == 2 {
                let (a_max_2, a_max_char_2) = find_highes_p1(a);
                let (b_max_2, b_max_char_2) = find_highes_p1(b);
                if a_max_2 == 2 || b_max_2 == 2 {
                    ordering = match a_max_2.cmp(&b_max_2) {
                        Less => Less,
                        Equal => sort_hands_by_order(a.2, b.2, &ord),
                        Greater => Greater,
                    }
                } else {
                    ordering = sort_hands_by_order(a.2, b.2, &ord);
                }
            } else {
                ordering = sort_hands_by_order(a.2, b.2, &ord);
            }
            a.0.insert(a_max_char, a_max);
            b.0.insert(b_max_char, b_max);
            ordering
        }
        Greater => Greater,
    }
}

fn sort_hands_p2(
    a: &mut (HashMap<char, usize>, usize, &str),
    b: &mut (HashMap<char, usize>, usize, &str),
) -> Ordering {
    if a.0.is_empty() && b.0.is_empty() {
        println!("SAME");
        return Equal;
    }

    let ord = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let (a_max, a_max_char) = find_highes_p1(a);
    let (b_max, b_max_char) = find_highes_p1(b);

    match a_max.cmp(&b_max) {
        Less => Less,
        Equal => {
            a.0.remove(&a_max_char).unwrap();
            b.0.remove(&b_max_char).unwrap();
            let mut ordering;
            if a_max == 3 || a_max == 2 {
                let (a_max_2, a_max_char_2) = find_highes_p1(a);
                let (b_max_2, b_max_char_2) = find_highes_p1(b);
                if a_max_2 == 2 || b_max_2 == 2 {
                    ordering = match a_max_2.cmp(&b_max_2) {
                        Less => Less,
                        Equal => sort_hands_by_order(a.2, b.2, &ord),
                        Greater => Greater,
                    }
                } else {
                    ordering = sort_hands_by_order(a.2, b.2, &ord);
                }
            } else {
                ordering = sort_hands_by_order(a.2, b.2, &ord);
            }
            a.0.insert(a_max_char, a_max);
            b.0.insert(b_max_char, b_max);
            ordering
        }
        Greater => Greater,
    }
}

pub fn sort_hands_by_order(a: &str, b: &str, ordering: &Vec<char>) -> Ordering {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    for i in 0..a.len() {
        let a_ch = a_chars[i];
        let b_ch = b_chars[i];
        match compare_card(a_ch, b_ch, ordering) {
            Less => return Less,
            Equal => continue,
            Greater => return Greater,
        }
    }
    Equal
}

fn find_highes_p1(b: &mut (HashMap<char, usize>, usize, &str)) -> (usize, char) {
    let mut b_max = 0usize;
    let mut b_max_char = ' ';
    b.0.iter().for_each(|(key, value)| {
        if value > &b_max {
            b_max = *value;
            b_max_char = *key;
        }
    });
    (b_max, b_max_char)
}

fn compare_card(a: char, b: char, ordering: &Vec<char>) -> Ordering {
    let mut a_index = 0;
    for i in 0..ordering.len() {
        if a == ordering[i] {
            a_index = i;
            break;
        }
    }
    let mut b_index = 0;
    for i in 0..ordering.len() {
        if b == ordering[i] {
            b_index = i;
            break;
        }
    }

    b_index.cmp(&a_index)
}

fn parse(input: &String) -> Vec<(&str, usize)> {
    let mut parsed = vec![];
    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split_whitespace().collect();
        let mut bid: usize = split[1].parse().unwrap();
        parsed.push((split[0], bid));
    });
    parsed
}

#[test]
fn solve_2023_7() {
    let contents = fs::read_to_string("./resources/twenty_three/day_7.txt")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_7 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_7 P1 {p2_res}");
}
