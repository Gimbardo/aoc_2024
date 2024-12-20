//use std::fs;

fn main() {
  let contents = include_str!("input.txt");
  println!("{}", problem1(contents.to_string()));
  println!("{}", problem2(contents.to_string()));
}

fn problem1(contents: String) -> i32 {
    let mut count = 0;
    for line in contents.lines() {
        let arr_line: Vec<i32> = line.split_ascii_whitespace().map(|el| el.parse::<i32>().unwrap()).collect();
        if is_incr_or_decr(arr_line.clone()) && elements_differentiate_by(arr_line.clone(), 3) {
            count += 1;
        }
    }
    count
}

fn problem2(contents: String) -> i32 {
    let mut count = 0;
    for line in contents.lines() {
        let arr_line: Vec<i32> = line.split_ascii_whitespace().map(|el| el.parse::<i32>().unwrap()).collect();
        if is_line_ok(arr_line) {
            count += 1
        }
    }
    count
}

fn is_line_ok(line: Vec<i32>) -> bool {
    for n in 0..line.len() {
        let mut new_line = line.clone();
        new_line.remove(n);
        if is_incr_or_decr(new_line.clone()) && elements_differentiate_by(new_line.clone(), 3) {
            return true
        }
    }
    false
}

fn is_incr_or_decr(line: Vec<i32>) -> bool {
    line.clone().is_sorted_by(|a, b| a > b) || line.clone().is_sorted_by(|a, b| a < b)
}

fn elements_differentiate_by(line: Vec<i32>, by: i32) -> bool {
    let mut splitted = line.iter();
    let mut prev = splitted.next().unwrap();
    for elem in splitted {
        let difference = (prev - elem).abs();
        prev = elem;
        if difference > by {
            return false
        }
    }
    true
}
