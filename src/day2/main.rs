//use std::fs;

fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem1(contents.to_string()));
  //print!("{}\n", problem2(contents.to_string()));
}

fn problem1(contents: String) -> i32 {
    let mut count = 0;
    for line in contents.lines() {
        if is_incr_or_decr(line.to_string()) && elements_differentiate_by(line.to_string(), 3) {
            count = count+1;
        }
    }
    return count;
}

fn is_incr_or_decr(line: String) -> bool {
    let splitted = line.split(" ").map(|el| el.parse::<i32>().unwrap());
    return splitted.clone().is_sorted_by(|a, b| a > b) || splitted.clone().is_sorted_by(|a, b| a < b)
}

fn elements_differentiate_by(line: String, by: i32) -> bool {
    let mut splitted = line.split_ascii_whitespace();
    let mut prev = splitted.next().unwrap();
    for elem in splitted {
        let difference = (prev.parse::<i32>().unwrap() - elem.parse::<i32>().unwrap()).abs();
        prev = elem;
        if difference > by {
            return false
        }
    }
    return true
}
