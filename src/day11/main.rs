use std::{collections::HashMap, hash::Hash};

fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem(contents.to_string()));
}

fn problem(contents: String) -> u64 {
  let mut stones: Vec<String> = vec![];
  for line in contents.lines() {
    stones = line.split(" ").collect::<Vec<&str>>().iter().map(|&s| s.to_string()).collect();
  }

  print!("{:?}\n", stones);

  let mut count = 0;
  let mut cache_results: HashMap<(String, usize), u64> = HashMap::new();

  for stone in stones {
    print!("facendo sasso {}\n", stone);
    count += rock_and_stone(&stone, 75, &mut cache_results);
    print!("partial count: {}\n", count);
  }
  return count;
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

fn rock_and_stone(stone: &str, missing_steps: usize, cache: &mut HashMap<(String, usize), u64>) -> u64 {
  //if missing_steps == 1 { print!("{} ", stone); }
  if missing_steps <= 0 { return 1; }
  match cache.get(&(stone.to_string(), missing_steps)) {
    Some(stone_result) => {return *stone_result;},
    None => {
      if stone == "0" {
        let res = rock_and_stone(&"1", missing_steps-1, cache);
        let key = (stone.to_string(), missing_steps);
        cache.insert(key, res.clone());
        return res;
      }
      if stone.len() % 2 == 0 {
        let mid = stone.char_indices().count() / 2;
        let (first, mut second) = stone.split_at(mid);
        second = second.trim_start_matches('0');
        if second.len() == 0 { second = &"0" }

        let res = rock_and_stone(&first, missing_steps-1, cache) + rock_and_stone(&second, missing_steps-1, cache);
        cache.insert((stone.to_string(), missing_steps), res.clone());
        return res;
      }
      let res = rock_and_stone(&format!("{}", stone.parse::<u64>().unwrap() * 2024), missing_steps-1, cache);
      cache.insert((stone.to_string(), missing_steps), res.clone());
      return res;
    }
  }
}
