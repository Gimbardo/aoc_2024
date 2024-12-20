use std::collections::HashMap;

fn main() {
  let contents = include_str!("input.txt");

  let mut rules: HashMap<u32, Vec<u32>> = HashMap::new(); // You know the rules, and so do I
  let mut updates: Vec<Vec<u32>> = vec![];
  (rules, updates) = setup_data(contents.to_string());

  println!("{}", problem1(&rules, &updates));
  println!("{}", problem2(&rules, &updates));
}

fn problem1(rules: &HashMap<u32, Vec<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    let count: u32 = updates.iter().map(|line| {
        if is_line_good(line, rules){
            middle_of_line(line)
        } else {
            0
        }
    }).sum();
    count
}

fn problem2(rules: &HashMap<u32, Vec<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    let count: u32 = updates.iter().map(|line| {
        if is_line_good(line, rules){
            0
        } else {
            let line_ordered: Vec<u32> = order_line(line.clone(), rules);
            middle_of_line(&line_ordered)
        }
    }).sum();

    count
}

fn setup_data(contents: String) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut reached_updates: bool = false;
    for line in contents.lines() {
        if !reached_updates && !line.is_empty() {
            let line_split: Vec<&str> = line.split("|").collect();
            let key = line_split[1].parse::<u32>().unwrap();
            let value = line_split[0].parse::<u32>().unwrap();
            rules.entry(key).or_default();
            rules.entry(key).and_modify(|rule| rule.push(value));
        } else if line.is_empty() {
            reached_updates = true;
        } else {
            updates.push(line.split(",").collect::<Vec<_>>().iter().map(|el| el.parse::<u32>().unwrap()).collect());
        }
    }
    (rules, updates)
}

fn is_line_good(line: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut banned_valueses: Vec<u32> = vec![];
    for el in line.iter() {
        if banned_valueses.contains(el) {
            return false;
        }
        banned_valueses.append(
            &mut rules.get(el).cloned().unwrap_or_else(Vec::new)
        );
    }
    true
}

fn middle_of_line(line: &[u32]) -> u32 {
    *line.get((line.len()-1)/2).unwrap()
}

fn order_line(mut line: Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    while !is_line_good(&line, rules) {
        let mut new_line = line.clone();
        let mut banned_valueses: HashMap<usize, Vec<u32>> = HashMap::new();
        for (index, el) in line.iter().enumerate() {
            if banned_valueses.values().flatten().collect::<Vec<_>>().contains(&el) {
                let mut id: usize = 0;
                for (id_arr, values) in banned_valueses.iter() {
                    if values.contains(el) {
                        id = *id_arr;
                        continue;
                    }
                }
                new_line.swap(id, index)
            } else {

                banned_valueses.entry(index).or_default();
                banned_valueses.entry(index).and_modify( |ban: &mut Vec<u32>| {
                    if let Some(values) = rules.get(el) {
                        ban.extend(values);
                    }
                });
            }
        }
        line = new_line
    }
    line
}
