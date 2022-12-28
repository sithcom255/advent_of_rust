use regex::Regex;
use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut rows = contents.split("\n").collect::<Vec<&str>>();
    let mut result = 0;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 10];
    let mut i = 0;
    while !rows[i].contains("1") {
        let mut row = rows[i].chars();
        let lenght = rows[0].len() / 4;
        for l in 0..lenght {
            let x = row.next().unwrap();
            let y = row.next().unwrap();
            let z = row.next().unwrap();
            if x == '[' {
                let mut s = &mut stacks[l];
                s.push(y)
            }
            row.next();
        }
        let x = row.next().unwrap();
        let y = row.next().unwrap();
        let z = row.next().unwrap();
        if x == '[' {
            let s = &mut stacks[lenght];
            s.push(y)
        }
        i += 1;
    }
    i += 2;
    for i in i..rows.len() {
        let re = Regex::new(r"[A-Za-z]").unwrap();
        let row = re.replace_all(rows[i], "")
        .replace("  ", " ").trim()
        .split(" ").map(|elem| {println!("{}", elem); elem}).map( |elem|elem.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        for l in 0..row[0] {
            let x = stacks[row[1] as usize - 1].remove(0);
            stacks[row[2] as usize - 1].insert(0,x);
        }
    }
    println!("{:?}", stacks);

    for stack in stacks {
        print!("{}", stack.first().get_or_insert(&' '))
    }
}
