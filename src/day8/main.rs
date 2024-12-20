use std::collections::HashMap;

const FILL_CHAR: char = '.';

fn main() {
  let contents = include_str!("input.txt");
  println!("{}", problem1_and2(contents.to_string()));
}

fn problem1_and2(contents: String) -> usize {
  let mut antennas_coords: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
  let mut antinodes: Vec<(i32, i32)> = vec![];
  let max_index: i32 = (contents.lines().next().unwrap().len() - 1) as i32;
  for (y, line) in contents.lines().enumerate() {
    for (x, possible_antenna) in line.chars().enumerate() {
      if possible_antenna == FILL_CHAR { continue }

      antennas_coords.entry(possible_antenna).or_default();
      antennas_coords.entry(possible_antenna).and_modify( |antennas: &mut Vec<_>| {
        antennas.push((x as i32, y as i32));
      });
    }
  }
  for (key, antennas) in antennas_coords.iter() {
    
    for (id, freq1) in antennas.iter().enumerate() {
      for freq2 in &antennas[id+1..] {

        if !antinodes.contains(freq1) {
          antinodes.push(*freq1)
        }
        if !antinodes.contains(freq2) {
          antinodes.push(*freq2)
        }

        populate_antinodes(freq1, freq2, &mut antinodes, max_index);
        populate_antinodes(freq2, freq1, &mut antinodes, max_index);        
      }
    }
  }
  antinodes.len()
}

fn populate_antinodes(freq1: &(i32, i32), freq2: &(i32, i32), antinodes: &mut Vec<(i32, i32)>, max_index: i32) {
  let antinode = calc_antinode(*freq1, *freq2);

  if !is_valid_proj(antinode, max_index) {
    return
  }
  if !antinodes.contains(&antinode) {
    antinodes.push(antinode)
  }
  populate_antinodes(&antinode, freq1, antinodes, max_index);
}

fn calc_antinode(freq1: (i32, i32), freq2: (i32, i32)) -> (i32, i32) {
  (freq1.0 + (freq1.0 - freq2.0), freq1.1 + (freq1.1 - freq2.1))
}

fn is_valid_proj(proj: (i32, i32), max_index: i32) -> bool {
  if proj.0 < 0 || proj.1 < 0 || proj.0 > max_index || proj.1 > max_index {
    return false;    
  }
  true
}