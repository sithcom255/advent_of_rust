use std::env;
use std::fs;
use std::str::FromStr;

pub fn day_two(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let rows = contents.split("\n");
    let mut final_res = 0;
    rows.for_each(|row: &str| {
        let vec = row.split(" ").collect::<Vec<&str>>();
        let other = Other::from_str(vec[0]).unwrap();
        let response = Response::from_str(vec[1]).unwrap();
        final_res += evaluate(other, response);
    });
    print!("{}", final_res);
}

fn evaluate(other: Other, response: Response) -> u32 {
    let mut res = 0;

    if other == Other::A {
        if response == Response::Y {
            res += 1;
        } else if response == Response::X {
            res += 3;
        } else if response == Response::Z {
            res += 2;
        }
    }
    if other == Other::B {
        if response == Response::Y {
            res += 2;
        } else if response == Response::X {
            res += 1;
        } else if response == Response::Z {
            res += 3;
        }
    }

    if other == Other::C {
        if response == Response::Y {
            res += 3;
        } else if response == Response::X {
            res += 2;
        } else if response == Response::Z {
            res += 1;
        }
    }

    if response == Response::X {
        res += 0;
    } else if response == Response::Y {
        res += 3;
    } else {
        res += 6;
    }

    return res;
}

#[derive(PartialEq)]
enum Response {
    // rock
    X,
    // paper
    Y,
    // scissors
    Z,
}

impl FromStr for Response {
    type Err = ();

    fn from_str(s: &str) -> Result<Response, Self::Err> {
        match s {
            "X" => Ok(Response::X),
            "Y" => Ok(Response::Y),
            "Z" => Ok(Response::Z),
            _ => Err(()),
        }
    }
}
#[derive(PartialEq)]
enum Other {
    // rock
    A,
    // paper
    B,
    // scissors
    C,
}

impl FromStr for Other {
    type Err = ();

    fn from_str(s: &str) -> Result<Other, Self::Err> {
        match s {
            "A" => Ok(Other::A),
            "B" => Ok(Other::B),
            "C" => Ok(Other::C),
            _ => Err(()),
        }
    }
}
