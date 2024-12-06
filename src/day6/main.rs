use std::{collections::HashMap, hash::Hash, thread, time};

const GUARD: char = '^';
const OBSTACLE: char = '#';
const TILE: char = '.';
const OUTSIDE: char = '🍃';
const FIRE_EMOJII: char = '🔥';

fn main() {
  let contents = include_str!("input.txt");
  let mut guard_starting_pos: HashMap<char, usize> = HashMap::new();

  let vec_contents: Vec<Vec<char>> = surround_content(contents.to_string(), contents.split("\n").next().unwrap().len()+2);

  contents.lines().enumerate().for_each( |(y, line)| {
    let line: Vec<char> = line.chars().collect::<Vec<char>>();
    line.iter().enumerate().for_each(|(x, pos)| {
        if *pos==GUARD {
            guard_starting_pos = HashMap::from([
                ('x', x+1),
                ('y', y+1)
            ]);
        }
    });
  });
  
  print!("{:#?}\n", problem1_and_2(vec_contents.clone(), guard_starting_pos));
}

fn surround_content(contents: String, line_len: usize) -> Vec<Vec<char>> {
  let only_surround_line: Vec<char> = (0..line_len).map(|_| OUTSIDE).collect();

  let mut final_content: Vec<Vec<char>> = Vec::new();
  final_content.push(only_surround_line.clone());
  for line in contents.lines() {
    final_content.push(format!("{OUTSIDE}{line}{OUTSIDE}").chars().collect());
  }
  final_content.push(only_surround_line);
  return final_content;
}

fn problem1_and_2(mut map: Vec<Vec<char>>, mut guard_current_pos: HashMap<char, usize>) -> (u32, u32) {
    let mut directions_maps: Vec<HashMap<char, i32>> = Vec::new();
    let guard_starting_pos: HashMap<char, usize> = guard_current_pos.clone();
    directions_maps.push(HashMap::from([
        ('x', 0),
        ('y', -1)]
    ));
    directions_maps.push(HashMap::from([
        ('x', 1),
        ('y', 0)]
    ));
    directions_maps.push(HashMap::from([
        ('x', 0),
        ('y', 1)]
    ));
    directions_maps.push(HashMap::from([
        ('x', -1),
        ('y', 0)]
    ));

    let mut possible_path: Vec<HashMap<char, usize>> = vec![];
    for direction in directions_maps.clone().iter().cycle() {
        let mut new_count: u32 = 0;
        let mut exit: bool = false;
        (guard_current_pos, exit) = proceed_until_obstacle(&mut map, direction.clone(), guard_current_pos.clone(), &mut possible_path);
        if exit {
          break
        }
    }
    let mut unique_steps_count: u32 = 0;
    map.iter().for_each(|line| {
      unique_steps_count += (line.iter().filter(|n| **n == FIRE_EMOJII).count() as u32);
    });

    let mut obstacle_loop_count: u32 = 0;

    for possible_obstacle_coord in possible_path {
      print!("valutando ostacolo in posizone {}, {}\n", possible_obstacle_coord.get(&'y').unwrap(), possible_obstacle_coord.get(&'x').unwrap());
      let mut new_map: Vec<Vec<char>> = map.clone();
      new_map[*possible_obstacle_coord.get(&'y').unwrap()][*possible_obstacle_coord.get(&'x').unwrap()] = OBSTACLE;
      if contains_a_loop(new_map, guard_starting_pos.clone(), directions_maps.clone()) {
        print!("loop in posizione {}, {}\n\n", possible_obstacle_coord.get(&'y').unwrap(), possible_obstacle_coord.get(&'x').unwrap());
        obstacle_loop_count+=1;
      }
    }

    return (unique_steps_count, obstacle_loop_count);
}

fn proceed_until_obstacle(map: &mut Vec<Vec<char>>, direction: HashMap<char, i32>, guard_position: HashMap<char, usize>, possible_path: &mut Vec<HashMap<char, usize>>) -> (HashMap<char, usize>, bool) {
  //pretty_print(map);
  //thread::sleep(time::Duration::from_millis(200));
  let next_pos_coord: HashMap<char, usize> = HashMap::from([
      ('x', ((*guard_position.get(&'x').unwrap() as i32) + direction.get(&'x').unwrap()) as usize),
      ('y', ((*guard_position.get(&'y').unwrap() as i32) + direction.get(&'y').unwrap()) as usize)
  ]);
  let next_pos = map[*next_pos_coord.get(&'y').unwrap()][*next_pos_coord.get(&'x').unwrap()];
  map[*guard_position.get(&'y').unwrap()][*guard_position.get(&'x').unwrap()] = FIRE_EMOJII;
  if next_pos == OBSTACLE {
    return (guard_position, false);
  } else if next_pos == OUTSIDE {
    return (guard_position, true);
  } else {
    if next_pos != FIRE_EMOJII {
      possible_path.push(guard_position.clone());
    }
    return proceed_until_obstacle(map, direction, next_pos_coord, possible_path);
  }
}

fn contains_a_loop(mut map: Vec<Vec<char>>, mut guard_starting_pos: HashMap<char, usize>, directions_maps: Vec<HashMap<char, i32>>) -> bool {
  let mut past_path_with_direction: Vec<HashMap<char, usize>> = vec![];  
  let mut is_loop = false;
  for direction in directions_maps.clone().iter().cycle() {
    let mut exit: bool = false;
    (guard_starting_pos, exit, is_loop) = proceed_until_obstacle_search_loop(&mut map, direction.clone(), guard_starting_pos.clone(), &mut past_path_with_direction);
    if is_loop {
      return true;
    }
    if exit {
      break;
    }
  }
  return false;
}

fn proceed_until_obstacle_search_loop(map: &mut Vec<Vec<char>>, direction: HashMap<char, i32>, guard_position: HashMap<char, usize>, possible_path: &mut Vec<HashMap<char, usize>>) -> (HashMap<char, usize>, bool, bool) {
  let next_pos_coord: HashMap<char, usize> = HashMap::from([
      ('x', ((*guard_position.get(&'x').unwrap() as i32) + direction.get(&'x').unwrap()) as usize),
      ('y', ((*guard_position.get(&'y').unwrap() as i32) + direction.get(&'y').unwrap()) as usize)
  ]);
  let next_pos = map[*next_pos_coord.get(&'y').unwrap()][*next_pos_coord.get(&'x').unwrap()];
  map[*guard_position.get(&'y').unwrap()][*guard_position.get(&'x').unwrap()] = FIRE_EMOJII;

  let mut guard_position_with_direction = guard_position.clone();
  guard_position_with_direction.insert('d', direction_to_usize(direction.clone()));

  for past_path in possible_path.iter()
  {
    if *guard_position.get(&'x').unwrap() == *past_path.get(&'x').unwrap() &&
       *guard_position.get(&'y').unwrap() == *past_path.get(&'y').unwrap() &&
       direction_to_usize(direction.clone()) == *past_path.get(&'d').unwrap() {
        return (guard_position, true, true);
    }
  }
  possible_path.push(guard_position_with_direction);

  if next_pos == OBSTACLE {
    return (guard_position, false, false);
  } else if next_pos == OUTSIDE {
    return (guard_position, true, false);
  } else {
    return proceed_until_obstacle_search_loop(map, direction, next_pos_coord, possible_path);
  }
}

fn direction_to_usize(direction: HashMap<char, i32>) -> usize {
  if *direction.get(&'x').unwrap() == 0 {
    if *direction.get(&'y').unwrap() == 1 {
      return 0; // DOWN
    } else {
      return 1; // UP
    }
  } else if *direction.get(&'x').unwrap() == 1 {
    return 2; // RIGHT
  } else {
    return 3; // LEFT
  }
}

fn pretty_print(fake_map: &Vec<Vec<char>>) {
  for row in fake_map {
      let row: String = row.clone().iter().collect();
      println!("{row}");
  }
  println!();
}