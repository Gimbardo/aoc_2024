// pile of shame:
// too low 143193697865

fn main() {
  let contents = include_str!("input.txt");

  let mut disk_map_extended: Vec<i32> = vec![];
  let mut is_space = false;
  let mut id = 0;

  for character in contents.chars() {
    let char_as_int = character.to_string().parse::<usize>().unwrap();
    if is_space {
      let mut to_add = vec![-1; char_as_int];
      disk_map_extended.append(&mut to_add);
    } else {
      let mut to_add = vec![id; char_as_int];
      disk_map_extended.append(&mut to_add);
      id +=1;
    }
    is_space = !is_space;
  }

  //print!("{}\n", problem1(disk_map_extended.clone()));
  print!("{}\n", problem2(disk_map_extended.clone()));
}

fn problem1(mut disk_map_extended: Vec<i32>) -> u64 {

  for elem in (0..disk_map_extended.len()).rev() {
    if disk_map_extended[elem] != -1 {
      if let Some(id) = disk_map_extended.iter().position( |&el| el == -1) {
        if id >= elem { break; }
        disk_map_extended.swap(id, elem);
        disk_map_extended.remove(elem);
      } else {
        break;
      }
    }
  }

  disk_map_extended.retain( |v| *v != -1  );

  print!("calculating result\n");

  let mut result: u64 = 0;

  for (mult, elem) in disk_map_extended.iter().enumerate() {
    result += (elem * (mult as i32)) as u64;
  }

  return result;
}

fn problem2(mut disk_map_extended: Vec<i32>) -> u64{
  
  let grouped = group_contingent(&disk_map_extended);

  let mut string_format: String = disk_map_extended.iter().map({ |el|
    if *el != -1 {
      return el.to_string();
    } else {
      return ".".to_string();
    }
  }).collect();

  // operate on this mf
  for (index, to_move_before) in grouped.clone().iter().enumerate().rev() {
    if to_move_before.0 == -1 { continue; }
    let  index_to_move = calculate_index_string(grouped.clone(), index);

    if index_to_move < to_move_before.1 { break; }

    let to_replace= std::iter::repeat(".").take(to_move_before.1).collect::<String>();
    let to_place= std::iter::repeat((to_move_before.0).to_string()).take(to_move_before.1).collect::<String>();
    
    if let Some(id_to_replace) = string_format.find(&to_replace) {
      if id_to_replace-1 > index_to_move { continue; }
      let mut new_string_format = string_format.replacen(&to_replace, &to_place, 1);
      new_string_format.replace_range(index_to_move..index_to_move + to_replace.len(), &to_replace);
      string_format = new_string_format.clone();
    }
  }

  let mut count: u64 = 0;

  for (i, c) in string_format.chars().enumerate() {
    if let Some(char_as_int) = c.to_digit(10) {
      count += char_as_int as u64 * i as u64;
    }
  }

  return count;
}

fn calculate_index_string(grouped: Vec<(i32, usize)>, index_limit: usize) -> usize {
  let mut index: usize = 0;
  for (id, el) in grouped.iter().enumerate() {
    if id == index_limit { break; }
    index += el.1;
  }
  return index
}

fn group_contingent(vector: &[i32]) -> Vec<(i32, usize)> {
  let mut result = Vec::new();
  
  if let Some(&first) = vector.first() {
    let mut prev_value = first;
    let mut count = 1;

    for &current_value in vector[1..].iter() {
      if prev_value == current_value {
        count += 1;
      } else {
        result.push((prev_value, count));
        prev_value = current_value;
        count = 1;
      }
    }
    result.push((prev_value, count));
  }
  
  return result;
}