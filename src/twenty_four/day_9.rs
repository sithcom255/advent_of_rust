use std::collections::HashSet;
use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let mut p1_res = 0;
    let mut p2_res: usize = 0;

    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let mut lines: Vec<Vec<usize>> = contents
        .split("\n")
        .map(|line| line.chars().map(|ch| ch as usize - 0x30usize).collect())
        .collect();

    {
        let numbers = &lines[0];
        let mut ids: Vec<usize> = Vec::new();
        for i in 0..numbers.len() {
            if i % 2 == 0 {
                ids.push(numbers[i]);
            }
        }

        let mut disk_i: usize = 0;
        for i in 0..numbers.len() {
            let number = numbers[i];
            let file_index = i / 2;
            if i % 2 == 0 {
                let todo = ids[file_index];
                for j in 0..todo {
                    p1_res += file_index * disk_i;
                    disk_i += 1;
                }
                ids[file_index] = 0;
            } else {
                if file_index >= ids.len() - 1 {
                    break;
                }

                let mut file_todo = 0;

                for j in 0..number {
                    if file_todo == 0 {
                        let length = ids.len() - 1;
                        file_todo = ids[length];
                    }

                    p1_res += (ids.len() - 1) * disk_i;
                    disk_i += 1;

                    file_todo -= 1;

                    if file_todo == 0 {
                        ids.pop();
                    } else {
                        let length = ids.len() - 1;
                        ids[length] = file_todo;
                    }
                }
            }
        }
        println!("{:?}", numbers.iter().sum::<usize>());
    }
    {
        let mut numbers = &mut lines[0];
        let mut disk_index: Vec<usize> = Vec::new();
        {
            let mut di = 0;
            for i in 0..numbers.len() {
                if i % 2 == 0 {
                    disk_index.push(di);
                    di += 1;
                } else {
                    disk_index.push(0);
                }
            }
        }
        println!("D I {:?}", disk_index.iter().sum::<usize>());

        {
            let mut i = numbers.len() - 1;

            let mut moved: HashSet<usize> = HashSet::new();
            loop {
                println!(" {} {:?}",i, numbers.iter().sum::<usize>());
                let number = numbers[i];
                let mut k = 0;

                let mut no_update: bool = false;

                if i % 2 == 0 {
                    while k <= i {
                        if k % 2 == 1 {
                            let free = numbers[k];
                            if free >= number {
                                let mut add: bool = false;
                                {
                                    let id = disk_index[i];
                                    if moved.contains(&id) {
                                        i -= 1;
                                        continue
                                    }
                                    moved.insert(disk_index[i]);
                                }


                                let removed_number = numbers.remove(i);
                                let mut removed_empty = 0;
                                if i < numbers.len() {
                                    removed_empty = numbers.remove(i);
                                } else {
                                    add = true
                                }

                                numbers[i - 1] += removed_number + removed_empty;
                                let fi = numbers[k];
                                numbers[k] = 0;
                                numbers.insert(k + 1, number);
                                numbers.insert(k + 2, fi - number);

                                let removed = disk_index.remove(i);
                                if i < disk_index.len() {
                                    disk_index.remove(i);
                                }
                                disk_index.insert(k + 1, removed);
                                disk_index.insert(k + 2, 0);

                                if add {
                                    i += 1;
                                }
                                no_update = true;
                                break;
                            }
                        }
                        k += 1;
                    }
                }

                if no_update {
                    i +=1;
                }

                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }

        let mut di = 0;
        for i in 0..numbers.len() {
            let number = numbers[i];
            if i % 2 == 0 {
                for x in 0..number {
                    p2_res += disk_index[i] * di;
                    di += 1;
                }
            } else {
                di += number;
            }
        }
        println!("{:?}", numbers.iter().sum::<usize>());
        println!("D I {:?}", disk_index.iter().sum::<usize>());
    }

    println!("P1 res {p1_res}");
    println!("P2 res {p2_res}");
}
