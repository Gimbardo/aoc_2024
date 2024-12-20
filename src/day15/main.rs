
enum Direction {
  UP, DOWN, LEFT, RIGHT
}

#[derive(Clone)]
struct Robot<'a> {
  coords: (usize, usize),
  instructions: &'a [Direction],
}

#[derive(Clone)]
struct Box {
  coords: (usize, usize),
}

#[derive(Clone)]
struct Wall {
  coords: (usize, usize)
}

impl<'a> Robot<'a> {
  fn new(coords: (usize, usize), instructions: &'a [Direction]) -> Self {
    Robot { coords, instructions }
  }

  fn identifier() -> char { return '@' }
}

impl<'a> IntoIterator for &'a Robot<'a> {
  type Item = &'a Direction;
  type IntoIter = std::slice::Iter<'a, Direction>;

  fn into_iter(self) -> Self::IntoIter {
      self.instructions.iter()
  }
}

impl Box {
  fn new(coords: (usize, usize)) -> Box {
    Box { coords }
  }

  fn identifier() -> char { return 'O' }
}

impl Wall {
  fn new(coords: (usize, usize)) -> Wall {
    Wall { coords }
  }

  fn identifier() -> char { return '#' }
}

fn main() {
  let contents = include_str!("input.txt");
  println!("{}", problem1(contents.to_string()));
  //println!("{}", problem2(contents.to_string()));
}

fn problem1(contents: String) -> usize {
  let mut boxes: Vec<Box> = vec![];
  let mut walls: Vec<Wall> = vec![];
  let mut instructions: Vec<Direction> = vec![];
  let mut robot_coords: (usize, usize) = (0, 0);
  let mut is_instructions = false;
  for (y, line) in contents.lines().enumerate() {

    if line == "" { is_instructions = true; continue; }

    if !is_instructions {
      for (x, elem) in line.chars().enumerate() {
        if elem == Robot::identifier() {
          robot_coords = (x, y);
        } else if elem == Box::identifier() {
          boxes.push(Box::new((x,y)))
        } else if elem == Wall::identifier() {
          walls.push(Wall::new((x,y)))
        }
      }
    } else {
      for (x, elem) in line.chars().enumerate() {
        instructions.push(char_to_direction(elem))
      }
    }
  }
  let mut robot = Robot::new(robot_coords, &instructions);
  return 1;
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
