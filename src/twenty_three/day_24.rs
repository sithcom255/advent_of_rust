use crate::solve::Solve;
use clap::builder::Str;
use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::str::Split;

struct Solver {}
impl Solve for Solver {
    fn p1(input: &String) -> String {
        let rows = parse(&input);
        let mut colliding = 0;
        for i in 0..rows.len() {
            for j in 0..rows.len() {
                if i == j {
                    continue;
                }
                let a = rows[i];
                let b = rows[j];

                if intersection(a, b) {
                    colliding += 1;
                }
            }
        }

        (colliding / 2).to_string()
    }

    fn p2(input: &String) -> String {
        let rows = parse(&input);

        let mut by_y: HashMap<i128, Vec<(i128, i128, i128, i128, i128, i128)>> = HashMap::new();
        for i in 0..rows.len() {
            let y = rows[i].4;
            match by_y.get_mut(&y) {
                None => {
                    by_y.insert(y, vec![rows[i].clone()]);
                }
                Some(list) => list.push(rows[i]),
            }
        }

        let filter = by_y
            .values()
            .filter(|list| return list.len() >= 3)
            .into_iter()
            .collect::<Vec<&Vec<(i128, i128, i128, i128, i128, i128)>>>();

        let mut triplet = filter.get(0).unwrap();

        let s0: (i128, i128, i128) = (0, 0, 0);
        let v0: (i128, i128, i128) = (0, 0, 0);

        let s1: (i128, i128, i128) = (
            triplet[2].0 as i128 ,
            triplet[2].1 as i128 ,
            triplet[2].2 as i128 ,
        );

        let v1: (i128, i128, i128) = (
            triplet[2].3 as i128 ,
            triplet[2].4 as i128,
            triplet[2].5 as i128 ,
        );
        let mut triplet = filter.get(1).unwrap();


        let s2: (i128, i128, i128) = (
            triplet[1].0 as i128 ,
            triplet[1].1 as i128 ,
            triplet[1].2 as i128 ,
        );
        let v2: (i128, i128, i128) = (
            triplet[1].3 as i128 ,
            triplet[1].4 as i128 ,
            triplet[1].5 as i128 ,
        );

        let mut triplet = filter.get(2).unwrap();

        let s3: (i128, i128, i128) = (
            triplet[0].0 as i128 ,
            triplet[0].1 as i128 ,
            triplet[0].2 as i128 ,
        );
        let v3: (i128, i128, i128) = (
            triplet[0].3 as i128 ,
            triplet[0].4 as i128 ,
            triplet[0].5 as i128 ,
        );

        let mut triplet = filter.get(3).unwrap();

        let s4: (i128, i128, i128) = (
            triplet[0].0 as i128 ,
            triplet[0].1 as i128 ,
            triplet[0].2 as i128 ,
        );
        let v4: (i128, i128, i128) = (
            triplet[0].3 as i128 ,
            triplet[0].4 as i128 ,
            triplet[0].5 as i128 ,
        );

     

        /// s1 + t1 * v1 = p + t1 * w
        /// s1.0 + t1 * v1.0 = p.0 + t1 * w.0
        /// (s1.0 - p.0) / ( w.0 - v1.0)  = t1
        /// (s1.1 - p.1) / ( w.1 - v1.1)  = t1
        /// (s1.2 - p.2) / ( w.2 - v1.2)  = t1
        /// (s1.0 - p.0) / ( w.0 - v1.0) = (s1.1 - p.1) / ( w.1 - v1.1)
        /// (s1.0 - p.0) * ( w.1 - v1.1) = (s1.1 - p.1) * ( w.0 - v1.0)
        ///
        /// s1.0 * w.1 - p.0 * w.1 - v1.1 * s1.0 + p.0 * v1.1 = w.0 * s1.1 - w.0 * p.1 - v1.0 * s1.1 + v1.0 * p.1;
        /// s1.0 * w.2 - p.0 * w.2 - v1.2 * s1.0 + p.0 * v1.2 = w.0 * s1.2 - w.0 * p.2 - v1.0 * s1.2 + v1.0 * p.2;
        ///
        ///
        /// s2.0 * w.1 - p.0 * w.1 - v2.1 * s2.0 + p.0 * v2.1 = w.0 * s2.1 - w.0 * p.1 - v2.0 * s2.1 + v2.0 * p.1;
        /// s2.0 * w.2 - p.0 * w.2 - v2.2 * s2.0 + p.0 * v2.2 = w.0 * s2.2 - w.0 * p.2 - v2.0 * s2.2 + v2.0 * p.2;
        ///
        /// s1.0 * w.1 - p.0 * w.1 - v1.1 * s1.0 + p.0 * v1.1 = w.0 * s1.1 - w.0 * p.1 - v1.0 * s1.1 + v1.0 * p.1;
        /// s2.0 * w.1 - p.0 * w.1 - v2.1 * s2.0 + p.0 * v2.1 = w.0 * s2.1 - w.0 * p.1 - v2.0 * s2.1 + v2.0 * p.1;
        ///
        /// (s1.0 - s2.0) * w1 - v1.1 * s1.0 + v2.1 * s2.0 + p.0 *(v1.1 - v2.1)
        /// = w0 * (s1.1 - s2.1) - v1.0 * s1.1 + v2.0 * s2.1 + p.1 * (v1.0 - v2.0)
        ///
        /// p.0 * (v1.1 - v2.1) - p.1 * (v1.0 - v2.0) + 0 - w0 * (s1.1 - s2.1) - (s1.0 - s2.0) * w1 + 0 =  + v1.1 * s1.0 - v2.1 * s2.0 - v1.0 * s1.1 + v2.0 * s2.1
        /// p.0 * (v1.2 - v2.2) - p.2 * (v1.0 - v2.0) + 0 - w0 * (s1.2 - s2.2) + 0 - (s1.0 - s2.0) * w2 =  + v1.2 * s1.0 - v2.2 * s2.0 - v1.0 * s1.2 + v2.0 * s2.2
        ///
        {
            Self::get_m_rows(s1, v1, s2, v2);
            Self::get_m_rows(s3, v3, s1, v1);
            Self::get_m_rows(s1, v1, s4, v4);

        }

        "".to_string()

    }
}

impl Solver {
    /// p.0 * (v1.1 - v2.1) + p.1 * (v1.0 - v2.0) + 0 - w0 * (s1.1 - s2.1) - (s1.0 - s2.0) * w1 + 0 =  + v1.1 * s1.0 - v2.1 * s2.0 - v1.0 * s1.1 + v2.0 * s2.1
    /// p.0 * (v1.2 - v2.2) - p.2 * (v1.0 - v2.0) + 0 - w0 * (s1.2 - s2.2) + 0 - (s1.0 - s2.0) * w2 =  + v1.2 * s1.0 - v2.2 * s2.0 - v1.0 * s1.2 + v2.0 * s2.2
    fn get_m_rows(s1: (i128, i128, i128), v1: (i128, i128, i128), s2: (i128, i128, i128), v2: (i128, i128, i128)) {
        {
            let p_0 = v2.1 - v1.1;
            let p_1 = (v1.0 - v2.0);
            let w_0 = -(s1.1 - s2.1);
            let w_1 = -(s1.0 - s2.0);
            let res = v1.1 * s1.0 - v2.1 * s2.0 - v1.0 * s1.1 + v2.0 * s2.1;
            let x = vec![p_0, p_1, 0i128, w_0, w_1, 0i128, res];
            println!("{:?}", x);
        }

        {
            let p_0 = (v2.2 - v1.2);
            let p_1 = 0;
            let p_2 = (v1.0 - v2.0);
            let w_0 = -(s1.2 - s2.2);
            let w_1 = -0;
            let w_2 = -(s1.0 - s2.0);
            let res = v1.2 * s1.0 - v2.2 * s2.0 - v1.0 * s1.2 + v2.0 * s2.2;
            let x = vec![p_0, p_1, p_2, w_0, w_1, w_2, res];
            println!("{:?}", x);
        }
    }
}

fn get_vector() {}

fn get_common_vector(a: (i128, i128, i128), b: (i128, i128, i128)) -> (bool, i128, i128, i128) {
    let s = b.0 * a.1 - a.0 * b.1;
    let t = (b.1 * s) / a.1;

    let x = a.0 / t;
    let y = a.1 / t;
    let z = a.2 / t;

    let ok = x * t == a.0;
    (ok, x, y, z)
}

fn intersection(
    a: (i128, i128, i128, i128, i128, i128),
    b: (i128, i128, i128, i128, i128, i128),
) -> bool {
    let s = ((b.1 - a.1) * a.3 - (b.0 - a.0) * a.4) as i128 / (b.3 * a.4 - b.4 * a.3) as i128;
    let t = ((b.1 - a.1) as i128 + s * b.4 as i128) / a.4 as i128;

    if s < 0i128 || t < 0i128 {
        return false;
    }

    let _min: i128 = 200_000_000_000_000i128;
    let _max: i128 = 400_000_000_000_000i128;

    let a_x = a.0 as i128 + t * a.3 as i128;
    let a_y = a.1 as i128 + t * a.4 as i128;

    let b_x = b.0 as i128 + s * b.3 as i128;
    let b_y = b.1 as i128 + s * b.4 as i128;

    return _min <= a_x
        && _max >= a_x
        && _min <= a_y
        && _max >= a_y
        && _min <= b_x
        && _max >= b_x
        && _min <= b_y
        && _max >= b_y;
}

fn parse(input: &String) -> Vec<(i128, i128, i128, i128, i128, i128)> {
    let mut rows: Vec<(i128, i128, i128, i128, i128, i128)> = vec![];
    input.lines().for_each(|line| {
        let r: Vec<i128> = line
            .split("@")
            .flat_map(|part| {
                part.split(",")
                    .map(|number| number.trim_end().trim_start().parse::<i128>().unwrap())
                    .into_iter()
            })
            .collect();
        rows.push((r[0], r[1], r[2], r[3], r[4], r[5]));
    });
    rows
}

#[test]
fn test() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    let p1_res = Solver::p1(&contents);
    println!("2023_23 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_23 P2 {p2_res}");
}

#[test]
fn solve_2023_24() {
    let contents = fs::read_to_string("./resources/twenty_three/day_24.txt")
        .expect("This is the error message");

    // let p1_res = Solver::p1(&contents);
    // println!("2023_24 P1 {p1_res}");

    let p2_res = Solver::p2(&contents);
    println!("2023_24 P2 {p2_res}");
}
