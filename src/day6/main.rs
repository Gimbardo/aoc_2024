use std::{collections::HashMap, thread, time};

const GUARD: char = '^';
const OBSTACLE: char = '#';
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
  
  print!("{}\n", problem1(vec_contents.clone(), guard_starting_pos));
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

fn problem1(mut map: Vec<Vec<char>>, mut guard_current_pos: HashMap<char, usize>) -> u32 {
    let mut directions_maps: Vec<HashMap<char, i32>> = Vec::new();
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
    let mut count = 0;
    for direction in directions_maps.iter().cycle() {
        let mut new_count: u32 = 0;
        let mut exit: bool = false;
        (guard_current_pos, new_count, exit) = proceed_until_obstacle(&mut map, direction.clone(), guard_current_pos.clone(), 0);
        count += new_count;
        if exit {
          break
        }
    }
    let mut unique_steps_count: u32 = 0;
    map.iter().for_each(|line| {
      unique_steps_count += (line.iter().filter(|n| **n == FIRE_EMOJII).count() as u32);
    });

    return unique_steps_count;
}

fn proceed_until_obstacle(map: &mut Vec<Vec<char>>, direction: HashMap<char, i32>, guard_position: HashMap<char, usize>, count: u32) -> (HashMap<char, usize>, u32, bool) {
  //pretty_print(map);
  //thread::sleep(time::Duration::from_millis(200));
  let next_pos_coord: HashMap<char, usize> = HashMap::from([
      ('x', ((*guard_position.get(&'x').unwrap() as i32) + direction.get(&'x').unwrap()) as usize),
      ('y', ((*guard_position.get(&'y').unwrap() as i32) + direction.get(&'y').unwrap()) as usize)
  ]);
  let next_pos = map[*next_pos_coord.get(&'y').unwrap()][*next_pos_coord.get(&'x').unwrap()];
  map[*guard_position.get(&'y').unwrap()][*guard_position.get(&'x').unwrap()] = FIRE_EMOJII;
  if next_pos == OBSTACLE {
    return (guard_position, count, false);
  } else if next_pos == OUTSIDE {
    return (guard_position, count, true);
  } else {
    return proceed_until_obstacle(map, direction, next_pos_coord, count+1);
  }
}

fn pretty_print(fake_map: &Vec<Vec<char>>) {
  for row in fake_map {
      let row: String = row.clone().iter().collect();
      println!("{row}");
  }
  println!();
}