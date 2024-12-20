use std::fs;

fn main() {
  let contents = fs::read_to_string("./src/day1/input.txt")
      .expect("Should have been able to read the file");
  println!("{}", problem1(contents.clone()));
  println!("{}", problem2(contents.clone()));
}

fn problem1(contents: String) -> i32 {
  let mut column1 = Vec::new();
  let mut column2 = Vec::new();
  for line in contents.split("\n") {
    let mut splitted = line.split_ascii_whitespace();
    column1.push(splitted.next().unwrap().parse::<i32>().unwrap());
    column2.push(splitted.next().unwrap().parse::<i32>().unwrap());
  }
  column1.sort();
  column2.sort();

  let mut solution = 0;

  for n in 0..column1.len() {
    solution += (column1[n] - column2[n]).abs();
  }

  solution
}

fn problem2(contents: String) -> i32 {
  let mut column1 = Vec::new();
  let mut column2 = Vec::new();
  for line in contents.split("\n") {
    let mut splitted = line.split_ascii_whitespace();
    column1.push(splitted.next().unwrap().parse::<i32>().unwrap());
    column2.push(splitted.next().unwrap().parse::<i32>().unwrap());
  }

  let mut solution = 0;

  for elem in column1.iter() {
    let mult = column2.iter().filter(|&n| *n==*elem ).count();
    solution += elem * mult as i32;
  }
  solution
}
