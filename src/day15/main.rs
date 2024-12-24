#[derive(PartialEq)]
enum Direction {
  UP, DOWN, LEFT, RIGHT
}

#[derive(PartialEq)]
#[derive(Clone)]
enum ObjectType {
  ROBOT, WALL, BOX, AIR
}

struct Object {
  coords: (usize, usize),
  type_of: ObjectType
}


impl Object {
  fn new(coords: (usize, usize), type_of: ObjectType) -> Self {
    Object { coords, type_of }
  }

  fn identifier(&self) -> char { 
    match self.type_of {
      ObjectType::ROBOT => '@',
      ObjectType::WALL => '#',
      ObjectType::BOX => 'O',
      ObjectType::AIR => '.',
    }
  }

  fn can_move_over(&self) -> bool {
    if self.type_of == ObjectType::AIR { return true }
    return false
  }

  fn can_be_moved(&self) -> bool {
    if self.type_of == ObjectType::WALL { return false }
    return true
  }
}

fn main() {
  let contents = include_str!("input.txt");
  println!("{}", problem1(contents.to_string()));
  //println!("{}", problem2(contents.to_string()));
}

fn problem1(contents: String) -> usize {
  let mut instructions: Vec<Direction> = vec![];
  let mut room: Vec<Vec<Object>> = vec![];
  let mut is_instructions = false;
  let mut robot_coords = (0, 0);
  for (y, line) in contents.lines().enumerate() {
    if line == "" { is_instructions = true; continue; }
    if !is_instructions {
      let line_objects: Vec<Object> = line.chars()
        .collect::<Vec<char>>()
        .iter().enumerate()
        .map( |(x, el)| {
          if char_to_object_type(*el) == ObjectType::ROBOT  { robot_coords = (x, y) };
          return Object::new((x, y),char_to_object_type(*el));
        }).collect();
      room.push(line_objects);
    } else {
      for elem in line.chars() {
        instructions.push(char_to_direction(elem))
      }
    }
  }

  pretty_print(&room);



  for instruction in instructions {
    let old_coords = robot_coords.clone();
    let is_moved = move_target_direction(&mut robot_coords, &mut room, instruction);
    if is_moved { room[old_coords.1][old_coords.0] = Object::new(old_coords, ObjectType::AIR) }
    //println!("Press ENTER to continue...");
    //let mut s=String::new();
    //std::io::stdin().read_line(&mut s).expect("Did not enter a correct string");
    //pretty_print(&room);
  }

  let mut count = 0;

  for (y, line) in room.iter().enumerate() {
    for (x, elem) in line.iter().enumerate() {
      if elem.type_of == ObjectType::BOX {
        count += (y*100) + x;
      }
    }
  }
  return count;
}


fn move_target_direction(robot_coords: &mut (usize, usize), area: &mut Vec<Vec<Object>>, direction: Direction) -> bool {
  let coords_diff = direction_to_coords_diff(&direction);
    
  let new_x = robot_coords.0 as i32 + coords_diff.0;
  let new_y = robot_coords.1 as i32 + coords_diff.1;
  
  if new_x < 0 || new_y < 0 || new_y >= area.len() as i32 || new_x >= area[new_y as usize].len() as i32 {
    return false;
  }

  let mut movable = true;

  if !area[new_y as usize][new_x as usize].can_be_moved() {
    return false;
  }

  if !area[new_y as usize][new_x as usize].can_move_over() {
    movable = move_target_direction(&mut (new_x as usize, new_y as usize), area, direction);
  }

  if movable {
    area[new_y as usize][new_x as usize] = Object::new((new_x as usize, new_y as usize), area[robot_coords.1][robot_coords.0].type_of.clone());
    *robot_coords = (new_x as usize, new_y as usize);
  }

  return movable;
}


fn problem2(contents: String) -> usize {
  return 1;
}

fn char_to_direction(elem: char) -> Direction {
  match elem {
    '^' => return Direction::UP,
    '>' => return Direction::RIGHT,
    'v' => return Direction::DOWN,
    '<' => return Direction::LEFT,
    _ => todo!()
  }
}

fn char_to_object_type(character: char) -> ObjectType { 
  match character {
    '@' => ObjectType::ROBOT ,
    '#' => ObjectType::WALL ,
    'O' => ObjectType::BOX ,
    '.' => ObjectType::AIR ,
    _ => todo!(),
  }
}

fn direction_to_coords_diff(direction: &Direction) -> (i32, i32){
  return match direction {
    Direction::UP => (0, -1),
    Direction::RIGHT => (1, 0),
    Direction::DOWN => (0, 1),
    Direction::LEFT => (-1, 0)
  };
}


fn pretty_print(area: &Vec<Vec<Object>>) {
  for line in area.iter() {
    for obj in line.iter() {
      print!("{} ", obj.identifier());
    }
    println!();
  }
  println!();
}