use std::collections::HashMap;

const FILL_CHAR: char = '.';

fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem1(contents.to_string()));
  //print!("{}\n", problem2(contents.to_string()));
}

fn problem1(contents: String) -> i32 {
  let mut count = 0;
  let mut frequencies_coords: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
  let mut antinodes: Vec<(i32, i32)> = vec![];
  let mut max_index: i32 = 0;
  for (y, line) in contents.lines().enumerate() {
    for (x, possible_antenna) in line.chars().enumerate() {
      if possible_antenna == FILL_CHAR { continue }

      frequencies_coords.entry(possible_antenna).or_insert_with(||vec![]);
      frequencies_coords.entry(possible_antenna).and_modify( |antennas: &mut Vec<_>| {
        antennas.push((x as i32, y as i32));
      });
    }
  }
  print!("{:#?}\n", frequencies_coords);
  for (key, frequencies) in frequencies_coords.iter() {
    
    for (id, freq1) in frequencies.iter().enumerate() {
      for freq2 in &frequencies[id..] {
        let proj1 = calc_antinode(*freq1, *freq2);
        
        antinodes.push(proj1);

        let proj2 = calc_antinode(*freq2, *freq1);

        antinodes.push(proj2);
      }
    }
  }
  return count;
}

fn calc_antinode(freq1: (i32, i32), freq2: (i32, i32)) -> (i32, i32) {
  return (freq1.0 + (freq1.0 - freq2.0), freq1.1 + (freq1.1 - freq2.1));
}

