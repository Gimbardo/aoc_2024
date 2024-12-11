
fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem1(contents.to_string()));
  //print!("{}\n", problem2(contents.to_string()));
}

fn problem1(contents: String) -> usize {
  let mut stones: Vec<String> = vec![];
  for line in contents.lines() {
    stones = line.split(" ").collect::<Vec<&str>>().iter().map(|&s| s.to_string()).collect();
  }

  print!("{:?}\n", stones);


  for i in 0..75 {
    print!("{}", i);
    stones = blink(stones);
  }
  return stones.len();
}

fn blink(stones: Vec<String>) -> Vec<String> {
  let mut new_stones: Vec<String> = vec![];
  stones.iter().enumerate().for_each( |(id, stone)| {
    if *stone == "0" {
      new_stones.push("1".to_string());
      return;
    }
    if stone.len() % 2 == 0 {
      let mid = stone.char_indices().count() / 2;
      let (first, second) = stone.split_at(mid);
      new_stones.push(format!("{}", first.parse::<i32>().unwrap()));
      new_stones.push(format!("{}", second.parse::<i32>().unwrap()));
      return;
    }
    new_stones.push( format!("{}", stone.parse::<u64>().unwrap() * 2024));
  });
  return new_stones;
}