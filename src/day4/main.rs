use std::collections::HashMap;

fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem1(contents.to_string()));
  print!("{}\n", problem2(contents.to_string()));
}

fn problem1(contents: String) -> u32 {
  let line_len: usize = contents.split("\n").next().unwrap().len()+2;
  let formatted_content: Vec<Vec<char>> = surround_content(contents, line_len);
  let x_indexes: Vec<HashMap<String, usize>> = find_char_indexes(formatted_content.clone(), "X".chars().next().unwrap());
  let directions_hash: Vec<HashMap<String, i32>> = calculate_directions(formatted_content.clone(), x_indexes);
  let count = count_xmas(formatted_content, directions_hash);
  return count;
}

fn problem2(contents: String) -> u32 {
  let line_len: usize = contents.split("\n").next().unwrap().len()+2;
  let formatted_content: Vec<Vec<char>> = surround_content(contents, line_len);
  let m_indexes: Vec<HashMap<String, usize>> = find_char_indexes(formatted_content.clone(), "A".chars().next().unwrap());
  let x_shapes_count = count_x_shapes(formatted_content.clone(), m_indexes);
  return x_shapes_count;
}

fn surround_content(contents: String, line_len: usize) -> Vec<Vec<char>> {
  let surround_char = "0".chars().next().unwrap();
  let only_surround_line: Vec<char> = (0..line_len).map(|_| surround_char).collect();

  let mut final_content: Vec<Vec<char>> = Vec::new();
  final_content.push(only_surround_line.clone());
  for line in contents.lines() {
    final_content.push(format!("{surround_char}{line}{surround_char}").chars().collect());
  }
  final_content.push(only_surround_line);
  return final_content;
}

fn find_char_indexes(contents: Vec<Vec<char>>, to_find: char) -> Vec<HashMap<String, usize>> {

  let mut result: Vec<HashMap<String, usize>> = Vec::new();
  
  for (y, row) in contents.iter().enumerate() {
    for (x, character) in row.iter().enumerate() {
      if *character==to_find {
        
        result.push(HashMap::from([
          ("x".into(), x),
          ("y".into(), y)
        ]))
      }
    }
  }

  return result;

}

fn calculate_directions(contents: Vec<Vec<char>>, x_indexes: Vec<HashMap<String, usize>>) -> Vec<HashMap<String, i32>> {

  let mut result: Vec<HashMap<String, i32>> = Vec::new();
  let mut directions_maps: Vec<HashMap<String, i32>> = Vec::new();
  for y in -1..=1 {
    for x in -1..=1 {
      directions_maps.push(HashMap::from([("x".into(), x), ("y".into(), y)]));
    }
  }

  for x_from_xmas in x_indexes.iter() {
    let y_index = *x_from_xmas.get("y").unwrap();
    let x_index = *x_from_xmas.get("x").unwrap();
    for (direction, direction_coords) in directions_maps.iter().enumerate() {
      let m_y_index = y_index as i32 + *direction_coords.get("y").unwrap();
      let m_x_index = x_index as i32 + *direction_coords.get("x").unwrap();
      if contents[m_y_index as usize][m_x_index as usize] == "M".chars().next().unwrap() {
        result.push(HashMap::from([
          ("m_x".into(), m_x_index), ("m_y".into(), m_y_index),
          ("direction_x".into(), *direction_coords.get("x").unwrap() as i32),
          ("direction_y".into(), *direction_coords.get("y").unwrap() as i32)        
        ]));
      }
    }
  }
  return result;
}

fn count_xmas(contents: Vec<Vec<char>>, directions_hash: Vec<HashMap<String, i32>>) -> u32 {
  let mut count: u32 = 0;
  for direction in directions_hash.iter() {
    let mut char_to_search = "A".chars().next().unwrap();
    let mut s_direction: HashMap<String, i32> = HashMap::new();

    let mut x_char_index = direction.get("m_x").unwrap() + direction.get("direction_x").unwrap();
    let mut y_char_index = direction.get("m_y").unwrap() + direction.get("direction_y").unwrap();
    if contents[y_char_index as usize][x_char_index as usize] == char_to_search {
      s_direction = HashMap::from([
        ("a_x".into(), x_char_index), ("a_y".into(), y_char_index),     
      ]);
    } else {
      continue;
    }

    char_to_search = "S".chars().next().unwrap();
    
    x_char_index = s_direction.get("a_x").unwrap() + direction.get("direction_x").unwrap();
    y_char_index = s_direction.get("a_y").unwrap() + direction.get("direction_y").unwrap();
    if contents[y_char_index as usize][x_char_index as usize] == char_to_search {
     count += 1; 
    }
  }

  return count;
}

fn count_x_shapes(contents: Vec<Vec<char>>, m_indexes: Vec<HashMap<String, usize>>) -> u32 {
    let mut count = 0;
    for m_index in m_indexes.iter() {
      let m_x = *m_index.get("x").unwrap() as u32;
      let m_y = *m_index.get("y").unwrap() as u32;
      let x_shape_values = vec! [
        contents[(m_y + 1) as usize][(m_x + 1) as usize],
        contents[(m_y - 1) as usize][(m_x + 1) as usize],
        contents[(m_y + 1) as usize][(m_x - 1) as usize],
        contents[(m_y - 1) as usize][(m_x - 1) as usize],
      ];
      let m_count = x_shape_values.iter().filter(|el| **el == "M".chars().next().unwrap()).count();
      let s_count = x_shape_values.iter().filter(|el| **el == "S".chars().next().unwrap()).count();
      if m_count == 2 && s_count == 2 && (x_shape_values[0] != x_shape_values[3]) {
        count +=1;
      }
    }
    return count;
}