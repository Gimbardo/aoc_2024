use regex::Regex;

fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem1(contents));
  print!("{}\n", problem2(contents));
}

fn problem1(contents: &str) -> i32 {
  return multiply_mul_and_sum(contents);
}

fn problem2(contents: &str) -> i32 {
  let mut count = 0;
  let arr = contents.split("do()");
  for split in arr {
    count += multiply_mul_and_sum(split.split("don't()").next().unwrap())
  }
  return count;

}

fn multiply_mul_and_sum(contents: &str) -> i32 {
  let mut count = 0;
  let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
  for (_, [first, second]) in re.captures_iter(contents).map(|c| c.extract()) {
    count += first.parse::<i32>().unwrap()*second.parse::<i32>().unwrap();
  }
  return count;
}