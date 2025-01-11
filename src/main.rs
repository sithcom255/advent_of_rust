use crate::four::day_16::advent_of_rust;

mod four;

fn main() {

    if !check_res(advent_of_rust("/home/jan/Documents/advent_of_rust/src/test"), 11048, 64) {
        return;
    }
    let res = advent_of_rust("/home/jan/Documents/advent_of_rust/src/input");
    println!("{:?}", res);
}

fn check_res(small: [usize;2], sp1_res: usize, sp2_res: usize) -> bool {
    let p1_res = small[0];
    let p2_res = small[1];
    if p1_res != sp1_res {
        println!("Expected {sp1_res}, actual {p1_res}");
        return false;
    }

    if sp2_res != p2_res {
        println!("Expected {sp2_res}, actual {p2_res}");
        return false;
    }
    println!("Passing");
    true
}
