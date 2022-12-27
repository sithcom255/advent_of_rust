use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let rows = contents.split("\n").collect::<Vec<&str>>();
    let mut result = 0;

    let mut row_iter = rows.iter().peekable();

    while row_iter.peek().is_some() {
        let f = row_iter.next().unwrap();
        let s = row_iter.next().unwrap();
        let t = row_iter.next().unwrap();

        let mut root: Option<TreeNode<char>> = None;
        let mut second_root: Option<TreeNode<char>> = None;
        f.chars().for_each(|c: char| {
            if root.is_none() {
                root = Some(TreeNode::new(c));
            } else {
                root.as_mut().unwrap().add(c)
            }
        });

        s.chars()
            .filter(|c: &char| root.as_ref().unwrap().find(*c))
            .for_each(|c: char| {
                if second_root.is_none() {
                    second_root = Some(TreeNode::new(c));
                } else {
                    second_root.as_mut().unwrap().add(c)
                }
            });

        t.chars().find(|c: &char| {
            if second_root.as_ref().unwrap().find(*c) {
                result += get_prio(&c);
                return true;
            }
            return false;
        });
    }
    println!("{}", result);
}

pub fn get_prio(char: &char) -> u32 {
    match char {
        char if char.is_uppercase() => char.clone() as u32 - 64 + 26,
        char if char.is_lowercase() => char.clone() as u32 - 96,
        _ => 0,
    }
}

struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Into<u32> + Copy> TreeNode<T> {
    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, value: T) {
        let insert = TreeNode::new(value);
        let inserted: u32 = value.into();
        let root: u32 = self.value.into();
        if inserted < root {
            if self.left.is_some() {
                self.left.as_mut().unwrap().add(value);
            } else {
                let _ = self.left.insert(Box::new(insert));
            }
        } else {
            if self.right.is_some() {
                self.right.as_mut().unwrap().add(value);
            } else {
                let _ = self.right.insert(Box::new(insert));
            }
        }
    }

    pub fn find(&self, value: T) -> bool {
        let val: u32 = value.clone().into();
        let root: u32 = self.value.into();
        if root == val {
            true
        } else if val < root {
            self.left.is_some() && self.left.as_ref().unwrap().find(value)
        } else {
            self.right.is_some() && self.right.as_ref().unwrap().find(value)
        }
    }
}
