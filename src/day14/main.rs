
#[derive(Clone)]
struct Robot {
  coords: (usize, usize),
  speed: (i32, i32)
}

const WIDE: i32 = 101;
const TALL: i32 = 103;

impl Robot {
  fn new(coords: (usize, usize), speed: (i32, i32)) -> Robot {
    Robot { coords: coords, speed: speed }
  }

  fn go(&mut self) {
    let mut new_x = self.coords.0 as i32 + self.speed.0;
    let mut new_y = self.coords.1 as i32 + self.speed.1;

    if new_x >= WIDE { new_x -= WIDE }
    else if new_x < 0 { new_x += WIDE }
    if new_y >= TALL { new_y -= TALL }
    else if new_y < 0 { new_y += TALL }
    self.coords.0 = new_x as usize;
    self.coords.1 = new_y as usize;
  }
}

fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem1(contents.to_string()));
  print!("{}\n", problem2(contents.to_string()));
}

fn problem1(contents: String) -> usize {
  let mut area: Vec<Vec<usize>> = vec![vec![0;WIDE as usize]; TALL as usize];
  let mut robots: Vec<Robot> = vec![];
  for line in contents.lines() {
    let mut split = line.split_ascii_whitespace();
    let coords = calc_coords(split.next().unwrap_or_else(|| ""));
    let speed = calc_speed(split.next().unwrap_or_else(|| ""));
    robots.push(Robot::new(coords, speed));
    area[coords.1][coords.0] += 1;
  }

  for _ in 0..100 {
    for (i, _) in robots.clone().iter().enumerate() {
      area[robots[i].coords.1][robots[i].coords.0] -= 1;
      robots[i].go();
      area[robots[i].coords.1][robots[i].coords.0] += 1;
    };
  }

  let mut counters: (usize, usize, usize, usize) = quadrants_counter(&area);

  return counters.0 * counters.1 * counters.2 * counters.3;
}

fn problem2(contents: String) -> usize {
  let mut area: Vec<Vec<usize>> = vec![vec![0;WIDE as usize]; TALL as usize];
  let mut robots: Vec<Robot> = vec![];
  for line in contents.lines() {
    let mut split = line.split_ascii_whitespace();
    let coords = calc_coords(split.next().unwrap_or_else(|| ""));
    let speed = calc_speed(split.next().unwrap_or_else(|| ""));
    robots.push(Robot::new(coords, speed));
    area[coords.1][coords.0] += 1;
  }

  for n in 0..10000 {
    for (i, _) in robots.clone().iter().enumerate() {
      area[robots[i].coords.1][robots[i].coords.0] -= 1;
      robots[i].go();
      area[robots[i].coords.1][robots[i].coords.0] += 1;
    };
    if contains_segments_of_length(4, 10, &area)
    {
      pretty_print(&area);
      return n+1
      //println!("Seconds passed: {} \nPress ENTER to continue...", n+1);
      //let mut s=String::new();
      //std::io::stdin().read_line(&mut s).expect("Did not enter a correct string");
    }
  }
  return 0;
}

fn contains_segments_of_length(n_segments: usize, length_segment: usize, area: &Vec<Vec<usize>>) -> bool {
  let mut found_segments = 0;
  for line in area.iter() {
    let mut line_count = 0;
    for robot_count in line.iter() {
      if *robot_count != 0 {
        line_count += 1;
      } else {
        line_count = 0;
      }
      if line_count >= length_segment {
        found_segments += 1;
      }
      if found_segments >= n_segments {
        return true
      }
    }
  }
  return false;
}

fn calc_coords(data: &str) -> (usize, usize) {

  if let Some((_, values)) = data.split_once("=") {
    if let Some((x, y)) = values.split_once(",") {
      return (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }
  }
  return (0, 0)
}

fn calc_speed(data: &str) -> (i32, i32) {
  if let Some((_, values)) = data.split_once("=") {
    if let Some((x, y)) = values.split_once(",") {
      return (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
    }
  }
  return (0, 0)
}

fn quadrants_counter(area: &Vec<Vec<usize>>) -> (usize, usize, usize, usize) {
  let mut counters: (usize, usize, usize, usize) = (0, 0, 0, 0);
  for (y, line) in area.iter().enumerate() {    
    for (x, bot_count) in line.iter().enumerate() {
      
      let high = y < (area.len()-1)/ 2;
      let low = y > (area.len()-1)/ 2;
      let left = x < (line.len()-1)/ 2;
      let right = x > (line.len()-1)/ 2;

      if left && high {
        counters.0 += bot_count;
      } else if right && high {
        counters.1 += bot_count;
      } else if left && low {
        counters.2 += bot_count;
      } else if right && low {
        counters.3 += bot_count;
      }
    }
  }
  return counters;
}

fn pretty_print(area: &Vec<Vec<usize>>) {
  for line in area.iter() {
    for count in line.iter() {
      let tree_char = if *count != 0 { "🟩" } else { "🟦" };
      print!("{} ", tree_char);
    }
    print!("\n");
  }
}