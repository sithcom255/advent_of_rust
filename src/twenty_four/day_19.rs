use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("This is the error message")
}

pub fn aoc_p1(contents: String) -> usize {
    eprintln!("aoc_p1");
    let mut p1 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    let towels: HashSet<&str> = input[0]
        .split(",")
        .map(|elem| elem.trim_end().trim_start())
        .collect();

    let mut trie = TrieNode::new();

    for towel in towels {
        trie.insert(towel, 0, 1);
    }

    for i in 2..input.len() {
        let to_solve = input[i];
        if can_solve(to_solve, &mut trie) {
            p1 += 1;
        }
    }

    p1
}

pub fn can_solve(composed: &str, trie: &mut TrieNode) -> bool {
    let mut start = 0;
    let vec;
    {
        vec = trie.search(composed);
    }

    let mut solvable = false;
    for (remove, ways) in vec {
        let candi = &composed[start + remove.len()..composed.len()];

        if candi.is_empty() {
            return true;
        }

        solvable |= can_solve(candi, trie);
        if solvable {
            break;
        }
    }
    solvable
}

pub fn aoc_p2(contents: String) -> usize {
    eprintln!("aoc_p1");
    let mut p1 = 0;
    let mut p2 = 0;

    let mut input: Vec<&str> = contents.split("\n").collect();

    let mut towels: HashSet<&str> = input[0]
        .split(",")
        .map(|elem| elem.trim_end().trim_start())
        .collect();

    let mut trie = TrieNode::new();

    let mut max = 0;
    for towel in towels.iter() {
        let c = towel.len();
        if c > max {
            max = c;
        }
    }

    let mut sorted = Vec::with_capacity(towels.len());

    for l in 0..max + 1 {
        for towel in towels.iter() {
            let c = towel.len();
            if c == l {
                sorted.push(towel);
            }
        }
    }

    for towel in sorted {
        let mut vec = Vec::with_capacity(towel.len());
        for j in 0..towel.len() {
            vec.push(-1);
        }
        let mut solved = get_solution_count(towel, &mut trie, &mut vec);
        if solved > 0 {
            trie.insert(towel, 0, 1);
        } else {
            trie.insert(towel, 0, 1);
        }
    }

    for i in 2..input.len() {
        let to_solve = input[i];

        let mut cache = Vec::with_capacity(to_solve.len());
        for j in 0..to_solve.len() {
            cache.push(-1);
        }

        let i1 = get_solution_count(to_solve, &mut trie, &mut cache);
        if i1 != 0 {
            p1 += 1;
        }
        p2 += i1;
    }

    println!("P! {p1}");
    p2
}

pub fn get_solution_count<'a>(
    composed: &'a str,
    trie: &mut TrieNode<'a>,
    cache: &mut Vec<isize>,
) -> usize {
    let cache_index = cache.len() - composed.len();
    if cache[cache_index] != -1 {
        return cache[cache_index] as usize;
    }

    let mut vec;
    {
        vec = trie.search(composed);
        vec.reverse()
    }

    let mut count = 0;
    for (remove, ways) in vec.iter() {
        let candi = &composed[remove.len()..composed.len()];

        if candi.is_empty() {
            count += ways;
            continue;
        }

        let i = get_solution_count(candi, trie, cache);
        count += i;
    }

    cache[cache_index] = count as isize;

    count
}

struct TrieNode<'a> {
    full: Option<&'a str>,
    ways: usize,
    chars: Vec<Option<RefCell<TrieNode<'a>>>>,
}

impl<'a> TrieNode<'a> {
    fn new() -> TrieNode<'a> {
        let chars = vec![None, None, None, None, None];
        TrieNode {
            full: None,
            ways: 0,
            chars,
        }
    }

    pub fn search(&mut self, pattern: &str) -> Vec<(String, usize)> {
        let mut result = vec![];
        self.search_internal(pattern, 0, &mut result);
        result
    }

    fn search_internal(&mut self, pattern: &str, position: usize, all: &mut Vec<(String, usize)>) {
        if position <= pattern.len() {
            match self.full {
                None => {}
                Some(so_far) => {
                    all.push((so_far.to_owned(), self.ways));
                }
            }
        }

        if position < pattern.len() {
            let ch = pattern.as_bytes()[position] as char;
            let i = get_index(ch);
            match &mut self.chars[i] {
                None => {}
                Some(trie) => {
                    trie.borrow_mut()
                        .search_internal(pattern, position + 1, all);
                }
            }
        }
    }

    pub fn insert(&mut self, pattern: &'a str, position: usize, ways: usize) {
        if position == pattern.len() {
            self.full = Some(pattern);
            self.ways = ways;
            return;
        }

        let ch = pattern.as_bytes()[position] as char;
        let i = get_index(ch);

        let option = &mut self.chars[i];
        match option {
            None => {
                let mut new = TrieNode::new();
                new.insert(pattern, position + 1, ways);
                let cell = RefCell::new(new);
                self.chars[i] = Some(cell)
            }
            Some(trie) => {
                trie.borrow_mut().insert(pattern, position + 1, ways);
            }
        }
    }
}

fn get_index(ch: char) -> usize {
    match ch {
        'u' => 1,
        'r' => 2,
        'g' => 3,
        'w' => 0,
        'b' => 4,
        _ => 999,
    }
}

#[test]
fn example() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/test")
        .expect("This is the error message");

    assert_eq!(6, aoc_p1(contents.to_owned()));
    assert_eq!(16, aoc_p2(contents.to_owned()));
}

#[test]
fn solve_() {
    let contents = fs::read_to_string("/home/jan/Documents/advent_of_rust/src/day_19.txt")
        .expect("This is the error message");

    let p1_res = aoc_p1(contents.to_owned());
    println!("RES P1 {p1_res}");

    let p2_res = aoc_p2(contents.to_owned());
    println!("RES P2 {p2_res}");
}

fn setup(y_len: usize, x_len: usize, final_pos: &mut Vec<Vec<bool>>) {
    for i in 0..y_len {
        let mut row = Vec::with_capacity(x_len);
        for j in 0..x_len {
            row.push(false);
        }
        final_pos.push(row);
    }
}
