use std::fs;

pub fn advent_of_rust(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("This is the error message");
    let rows = contents.split("\n");
    let mut result = 0;
    rows.for_each(|row: &str| {
      let ranges = row.split(",").collect::<Vec<&str>>();
      let first = ranges[0].split("-").map(|elem| elem.parse::<u32>().unwrap()).collect::<Vec<u32>>();
      let second = ranges[1].split("-").map(|elem| elem.parse::<u32>().unwrap()).collect::<Vec<u32>>();
      if check_contains(&first, &second) || check_contains(&second,&first) {
        result += 1;
      }
    });

    
   
    print!("max {} \n",  result)
}

fn check_contains(first: &Vec<u32>, second: & Vec<u32>) -> bool {
    first[1] >= second[0] && first[1] <= second[1] ||
    first[0] >= second[0] && first[0] <= second[1] 
}
