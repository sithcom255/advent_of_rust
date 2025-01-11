use std::fs;
use std::ptr::null;

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}

pub fn aoc_p1(contents: String) -> [String; 2] {
    let mut p1_res: String = "".to_owned();
    let mut p2_res: String = "".to_string();

    let mut rows: Vec<&str> = contents.split("\n").collect();

    let mut a_reg: usize = parse_reg(&mut rows, 0);
    let mut b_reg: usize = parse_reg(&mut rows, 1);
    let mut c_reg: usize = parse_reg(&mut rows, 2);

    let mut instructions: Vec<usize> = get_instructions(&mut rows);
    let mut out: Vec<usize> = Vec::new();
    {
        let mut pointer: usize = 0;

        while pointer < instructions.len() {
            let instruction = instructions[pointer];
            let mut combo = 0;
            if pointer + 1 < instructions.len() {
                combo = instructions[pointer + 1];
            }

            match instruction {
                // adv
                0 => {
                    if combo == 4 {
                        a_reg = a_reg / 2usize.pow(a_reg as u32);
                    } else if combo == 5 {
                        a_reg = a_reg / 2usize.pow(b_reg as u32);
                    } else if combo == 6 {
                        a_reg = a_reg / 2usize.pow(c_reg as u32);
                    } else {
                        a_reg = a_reg / 2usize.pow(combo as u32);
                    }
                    pointer += 2;
                }
                // bxl
                1 => {
                    b_reg = b_reg ^ combo;
                    // if 0 > combo || combo > 3 {
                    //     println!("error bxl")
                    // }
                    pointer += 2;
                }
                // bst
                2 => {
                    if combo == 4 {
                        b_reg = a_reg % 8;
                    } else if combo == 5 {
                        b_reg = b_reg % 8;
                    } else if combo == 6 {
                        b_reg = c_reg % 8;
                    }
                    if 4 > combo || combo > 6 {
                        println!("error bst")
                    }
                    pointer += 2;
                }
                // jnz
                3 => {
                    if a_reg != 0 {
                        pointer = combo;
                        if combo > 3 {
                            println!("error jnz")
                        }
                    } else {
                        pointer += 2;
                    }
                }
                // bxc
                4 => {
                    b_reg = c_reg ^ b_reg;
                    pointer += 2;
                }
                // out
                5 => {
                    if combo == 4 {
                        out.push(a_reg % 8);
                    } else if combo == 5 {
                        out.push(b_reg % 8);
                    } else if combo == 6 {
                        out.push(c_reg % 8);
                    } else {
                        out.push(combo);
                    }
                    pointer += 2;
                }
                // bdv
                6 => {
                    if combo == 4 {
                        b_reg = a_reg / 2usize.pow(a_reg as u32);
                    } else if combo == 5 {
                        b_reg = a_reg / 2usize.pow(b_reg as u32);
                    } else if combo == 6 {
                        b_reg = a_reg / 2usize.pow(c_reg as u32);
                    } else {
                        b_reg = a_reg / 2usize.pow(combo as u32);
                    }
                    pointer += 2;
                }
                7 => {
                    if combo == 4 {
                        c_reg = a_reg / 2usize.pow(a_reg as u32);
                    } else if combo == 5 {
                        c_reg = a_reg / 2usize.pow(b_reg as u32);
                    } else if combo == 6 {
                        c_reg = a_reg / 2usize.pow(c_reg as u32);
                    } else {
                        c_reg = a_reg / 2usize.pow(combo as u32);
                    }
                    pointer += 2;
                }
                _ => {}
            }
        }
    }

    p1_res = out
        .iter()
        .map(|elem| elem.to_string())
        .collect::<Vec<String>>()
        .join(",");

    p2_res = a_reg.to_string();

    [p1_res, p2_res]
}

pub fn aoc_p2(contents: String) -> [String; 2] {
    let mut p1_res: String = "".to_owned();
    let mut p2_res: String = "".to_string();

    let mut rows: Vec<&str> = contents.split("\n").collect();

    let mut a_reg: usize = parse_reg(&mut rows, 0);
    let mut b_reg: usize = parse_reg(&mut rows, 1);
    let mut c_reg: usize = parse_reg(&mut rows, 2);

    let mut instructions: Vec<usize> = get_instructions(&mut rows);
    let mut out: Vec<usize> = Vec::new();

    p1_res = out
        .iter()
        .map(|elem| elem.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // {
    //     let mut reverse_pointer = (instructions.len() - 2) as isize;
    //     let mut out_pointer = (instructions.len() - 1) as isize;
    //     // p2
    //     while true {
    //         let instruction = instructions[reverse_pointer as usize];
    //         let mut combo = 0;
    //         if reverse_pointer + 1 < instructions.len() as isize {
    //             combo = instructions[reverse_pointer as usize + 1];
    //         }
    //
    //         match instruction {
    //             // adv
    //             0 => {
    //                 if combo == 4 {
    //                     a_reg = a_reg * 2usize.pow(a_reg as u32 % 127)
    //                 } else if combo == 5 {
    //                     a_reg = a_reg * 2usize.pow(b_reg as u32 % 127);
    //                 } else if combo == 6 {
    //                     a_reg = a_reg * 2usize.pow(c_reg as u32 % 127);
    //                 } else {
    //                     a_reg = (a_reg as u64)
    //                         .overflowing_mul(2usize.pow(combo as u32 % 64) as u64)
    //                         .0 as usize;
    //                 }
    //                 reverse_pointer -= 2;
    //             }
    //             // bxl
    //             1 => {
    //                 b_reg = b_reg ^ combo;
    //                 // if 0 > combo || combo > 3 {
    //                 //     println!("error bxl")
    //                 // }
    //                 reverse_pointer -= 2;
    //             }
    //             // bst
    //             2 => {
    //                 if combo == 4 {
    //                     while a_reg % 8 > 0 && a_reg % 8 != b_reg % 8 {
    //                         a_reg = a_reg << 1;
    //                     }
    //                     a_reg = a_reg | b_reg % 8;
    //                 } else if combo == 5 {
    //                     b_reg = b_reg - (b_reg % 8) + b_reg % 8;
    //                 } else if combo == 6 {
    //                     c_reg = c_reg - (c_reg % 8) + b_reg % 8;
    //                 }
    //                 if 4 > combo || combo > 6 {
    //                     println!("error bst")
    //                 }
    //                 reverse_pointer -= 2;
    //             }
    //             // jnz
    //             3 => {
    //                 // pass
    //                 reverse_pointer -= 2;
    //             }
    //             // bxc
    //             4 => {
    //                 b_reg = c_reg ^ b_reg;
    //                 reverse_pointer -= 2;
    //             }
    //             // out
    //             5 => {
    //                 let out = instructions[out_pointer as usize];
    //                 out_pointer -= 1;
    //                 if combo == 4 {
    //                     while a_reg % 8 > 0 && a_reg % 8 != out {
    //                         a_reg = a_reg << 1;
    //                     }
    //                     a_reg = a_reg | out;
    //                 } else if combo == 5 {
    //                     while b_reg % 8 > 0 && b_reg % 8 != out {
    //                         b_reg = b_reg << 1;
    //                     }
    //                     b_reg = b_reg | out;
    //                 } else if combo == 6 {
    //                     while c_reg % 8 > 0 && c_reg % 8 != out {
    //                         c_reg = c_reg << 1;
    //                     }
    //                     c_reg = c_reg | out;
    //                 } else {
    //                     println!("error out")
    //                 }
    //                 reverse_pointer -= 2;
    //             }
    //             // bdv
    //             6 => {
    //                 if combo == 4 {
    //                     b_reg = a_reg * 2usize.pow(a_reg as u32 % 127);
    //                 } else if combo == 5 {
    //                     b_reg = a_reg * 2usize.pow(b_reg as u32 % 127);
    //                 } else if combo == 6 {
    //                     b_reg = a_reg * 2usize.pow(c_reg as u32 % 127);
    //                 } else {
    //                     b_reg = a_reg * 2usize.pow(combo as u32 % 127);
    //                 }
    //                 reverse_pointer -= 2;
    //             }
    //             7 => {
    //                 if combo == 4 {
    //                     c_reg = a_reg * 2usize.pow(a_reg as u32 % 127);
    //                 } else if combo == 5 {
    //                     c_reg = (a_reg as u64)
    //                         .overflowing_mul(2usize.pow(b_reg as u32 % 64) as u64)
    //                         .0 as usize;
    //                 } else if combo == 6 {
    //                     c_reg = a_reg * 2usize.pow(c_reg as u32 % 127);
    //                 } else {
    //                     c_reg = a_reg * 2usize.pow(combo as u32 % 127);
    //                 }
    //                 reverse_pointer -= 2;
    //             }
    //             _ => {}
    //         }
    //
    //         if reverse_pointer == -2 {
    //             if out_pointer >= 0 {
    //                 reverse_pointer = (instructions.len() - 2) as isize;
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    // }

    let mut res = Vec::new();
    p2_brute_by_num_day_3(&instructions,0,(&instructions.len() -1) as isize,&mut res,true);
    println!("{:?}", res);
    p2_res = a_reg.to_string();

    [p1_res, p2_res]
}

fn p2_brute_by_num(
    instructions: &Vec<usize>,
    current: usize,
    position: isize,
    res: &mut Vec<usize>,
    first:bool,
) {
    if position == -1 {
        res.push(current);
        return;
    }

    let inst = instructions[position as usize];

    let target = inst ^ 4;

    // this is the b ^ c possibilities
    for i in 0..8 {
        let b_c = 8 - i;
        let c_c = i;

        if target != b_c ^ c_c {
            continue;
        }

        let  candi = current | (b_c ^ 7) << (position * 3);
        let next_o = c_c << (position  * 3);
        if candi == (candi | next_o) || first {
            p2_brute_by_num(instructions, candi, position - 1, res,false);
        }
    }
}

fn p2_brute_by_num_day_3(
    instructions: &Vec<usize>,
    current: usize,
    position: isize,
    res: &mut Vec<usize>,
    first:bool,
) {
    if position == -1 {
        res.push(current);
        println!("RES {current}");
        return;
    }

    let shifted_current = current << 3;

    let inst = instructions[position as usize];
    let target = inst ;

    {
        // let initial_a_reg = shifted_current / 2usize.pow((shifted_current % 64) as u32);
        for i in 0..8 {
            // 2,4
            let mut mod_8 = i;
            // 1,7
            let b_reg = mod_8 ^ 7usize;
            // 7,5
            let c_reg = (shifted_current | mod_8 )/ 2usize.pow(b_reg as u32);
            // 4,1
            let xored_b_reg = b_reg ^ c_reg;
            // 1,4
            let final_xor = xored_b_reg ^ 4;
            // 5,5
            let out = final_xor % 8;

            if out == target {
                p2_brute_by_num_day_3(instructions, shifted_current | mod_8, position - 1, res, false);
            }
        }
    }
}

fn get_instructions(rows: &mut Vec<&str>) -> Vec<usize> {
    rows[4].split(":").collect::<Vec<&str>>()[1]
        .trim_start()
        .trim_end()
        .split(",")
        .map(|ch| ch.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_reg(rows: &mut Vec<&str>, i: usize) -> usize {
    rows[i].split(":").collect::<Vec<&str>>()[1]
        .trim_start()
        .trim_end()
        .parse::<usize>()
        .unwrap()
}

#[test]
fn a() {
    let template = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    let result = aoc_p1(template.to_string());

    let a_reg = result[1].parse::<usize>().unwrap();

    assert_eq!(result[0], "4,6,3,5,6,3,5,2,1,0");

    println!("A_reg {}", a_reg);

    let modified = aoc_p1(format!(
        "Register A: 29328
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
    ));
    assert_eq!(modified[0], "0,1,5,4,3,0")
}

#[test]
fn b() {
    let template = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    let result = aoc_p1(template.to_string());

    let a_reg = result[1].parse::<usize>().unwrap();

    println!("A_reg {}", a_reg);

    let modified = aoc_p1(format!(
        "Register A: {a_reg}
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
    ));
    assert_eq!(modified[0], "0,3,5,4,3,0")
}

#[test]
fn test_replication() {
    let result = aoc_p1(
        "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            .to_string(),
    );
    println!("{:?}", result)
}

#[test]
fn solve() {
    let template = "Register A: 53437164
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0";
    let result = aoc_p1(template.to_string());

    let mut a_reg = result[1].parse::<usize>().unwrap();

    println!("A_reg {}", a_reg);

    let modified = aoc_p1(format!(
        "Register A: {a_reg}
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0"
    ));
    assert_eq!(modified[0], "2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0");
}

#[test]
fn solve_() {
    let mut a_reg="1".to_owned() ;
    let mut string_check ="null()".to_owned() ;

    {
        let modified = aoc_p2(format!(
"Register A: {a_reg}
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0"
        ));

        a_reg = modified[1].to_owned();
    }

    {
        let modified = aoc_p1(format!(
            "Register A: {a_reg}
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0"
        ));

        string_check = modified[0].to_owned();
    }

    assert_eq!(string_check, "2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0")
}
