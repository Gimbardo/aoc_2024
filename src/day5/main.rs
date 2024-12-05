use std::collections::HashMap;

fn main() {
  let contents = include_str!("input.txt");
  print!("{}\n", problem1(contents.to_string()));
  //print!("{}\n", problem2(contents.to_string()));
}

fn problem1(contents: String) -> u32 {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new(); // You know the rules, and so do I
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut reached_updates: bool = false;
    for line in contents.lines() {
        if !reached_updates && line.len() != 0 {
            line.split("|")
        } else if line.len() == 0 {
            print!("\n\n REACHED UPDATES \n\n");
            reached_updates = true;
        } else {
            print!("{:#?}\n", line);
        }
    }
    return 1;
}
