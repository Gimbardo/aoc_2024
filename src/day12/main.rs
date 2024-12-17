#![feature(let_chains)]

fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem(contents.to_string()));
}

fn problem(contents: String) -> u64 {
  let mut formatted_content: Vec<Vec<(char, bool)>> = vectorize_content(contents);
  let mut val: u64 = 0;

  for y in 0..formatted_content.len() {
    let line = &formatted_content[y];
    for x in 0..line.len() {
      if formatted_content[y][x].1 { continue; }
      let fence = fence_cost(&mut formatted_content, (y, x));
      val += fence.0 as u64 * fence.1 as u64;
    }
  }
  return val;
}

fn vectorize_content(contents: String) -> Vec<Vec<(char, bool)>> {
  let mut final_content: Vec<Vec<(char, bool)>> = Vec::new();
  for line in contents.lines() {
    let line_form: Vec<(char, bool)> = line.chars().collect::<Vec<char>>().iter().map(|el| (*el, false)).collect();
    final_content.push(line_form);
  }
  return final_content;
}

fn fence_cost(farm: &mut Vec<Vec<(char, bool)>>, coords: (usize, usize)) -> (usize, usize) {
  let mut count: (usize, usize) = (0, 1);
  let elem = farm[coords.0][coords.1];

  if elem.1 {
    return (0, 0);
  }

  farm[coords.0][coords.1].1 = true;

  let line = &farm.clone()[coords.0];

  if coords.1 != 0 && let Some(left_elem) = line.get(coords.1 - 1) {
    if left_elem.0 == elem.0 {
      let temp_count = fence_cost(farm, (coords.0, coords.1-1));
      count.0 += temp_count.0;
      count.1 += temp_count.1;
    } else {
      count.0 += 1
    }
  } else {
    count.0 += 1
  }

  if let Some(right_elem) = line.get(coords.1 + 1) {
    if right_elem.0 == elem.0 {
      let temp_count = fence_cost(farm, (coords.0, coords.1+1));
      count.0 += temp_count.0;
      count.1 += temp_count.1;
    } else {
      count.0 += 1
    }
  } else {
    count.0 += 1
  }

  if coords.0 != 0 && let Some(upper_line) = farm.get(coords.0 - 1) {
    if upper_line[coords.1].0 == elem.0 {
      let temp_count = fence_cost(farm, (coords.0-1, coords.1));
      count.0 += temp_count.0;
      count.1 += temp_count.1;
    } else {
      count.0 += 1
    }
  } else {
    count.0 += 1
  }

  if let Some(lower_line) = farm.get(coords.0 + 1) {
    if lower_line[coords.1].0 == elem.0 {
      let temp_count = fence_cost(farm, (coords.0+1, coords.1));
      count.0 += temp_count.0;
      count.1 += temp_count.1;
    } else {
      count.0 += 1
    }
  } else {
    count.0 += 1
  }

  return count;
}


