

const OUTSIDE: char = '#';

fn main() {
  let contents = include_str!("input.txt");
  //print!("{}\n", problem1(contents.to_string()));
  println!("{}", problem2(contents.to_string()));
}

fn problem1(contents: String) -> u32 {
  let mut count = 0;
  
  let formatted_content: Vec<Vec<u32>> = surround_content(contents);
  
  for (y, line) in formatted_content.iter().enumerate() {
    for (x, character) in line.iter().enumerate() {
      if *character == 0 {
        count += count_n_trailheads(0, x, y, &formatted_content).len();
      }
    }
  }
  
  count as u32
}

fn problem2(contents: String) -> u32 {
  let mut count = 0;
  
  let formatted_content: Vec<Vec<u32>> = surround_content(contents);
  
  for (y, line) in formatted_content.iter().enumerate() {
    for (x, character) in line.iter().enumerate() {
      if *character == 0 {
        count += count_paths(0, x, y, &formatted_content);
      }
    }
  }
  
  count
}

fn count_paths(value: u32, x: usize, y: usize, table: &Vec<Vec<u32>>) -> u32 {
  let line_len = &table.len();
  let target = value+1;
  let mut count = 0;
  println!("{}, {} - searching {} ", x, y, target);
  if target == 10 { return 1 }
  if x>0 && table[y][x-1] == target { count += count_paths(target, x-1, y, table) }
  if x<line_len-1 && table[y][x+1] == target { count += count_paths(target, x+1, y, table) }
  if y>0 && table[y-1][x] == target { count += count_paths(target, x, y-1, table) }
  if y<line_len-1 && table[y+1][x] == target { count += count_paths(target, x, y+1, table) }

  count
}

fn count_n_trailheads(value: u32, x: usize, y: usize, table: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
  let line_len = &table.len();
  let target = value+1;
  println!("{}, {} - searching {} ", x, y, target);
  let mut to_return: Vec<(usize, usize)> = vec![];
  if target == 10 { return vec![(x, y)]; }
  if x>0 && table[y][x-1] == target {
    to_return.append(&mut count_n_trailheads(target, x-1, y, table));
  }
  if x<line_len-1 && table[y][x+1] == target {
    to_return.append(&mut count_n_trailheads(target, x+1, y, table))
  }
  if y>0 && table[y-1][x] == target {
    to_return.append(&mut count_n_trailheads(target, x, y-1, table))
  }
  if y<line_len-1 && table[y+1][x] == target {
    to_return.append(&mut count_n_trailheads(target, x, y+1, table))
  }

  to_return.sort();
  to_return.dedup();
  to_return
}

fn surround_content(contents: String) -> Vec<Vec<u32>> {
  let line_len = contents.lines().next().unwrap().len()+2;
  let only_surround_line: Vec<char> = (0..line_len).map(|_| OUTSIDE).collect();

  let mut final_content: Vec<Vec<u32>> = Vec::new();
  //final_content.push(only_surround_line.clone());
  for line in contents.lines() {
    final_content.push(line.to_string().chars().map(|c| c.to_digit(10).unwrap_or(100) ).collect());
  }
  //final_content.push(only_surround_line);
  final_content
}
