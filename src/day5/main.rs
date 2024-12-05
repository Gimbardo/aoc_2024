use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
  let contents = include_str!("input.txt");

  let mut rules: HashMap<u32, Vec<u32>> = HashMap::new(); // You know the rules, and so do I
  let mut updates: Vec<Vec<u32>> = vec![];
  (rules, updates) = setup_data(contents.to_string());

  //print!("{}\n", problem1(&rules, &updates));
  print!("{}\n", problem2(&rules, &updates));
}

fn problem1(rules: &HashMap<u32, Vec<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    let count: u32 = updates.iter().map(|line| {
        if is_line_good(&line, &rules){
            return middle_of_line(&line);
        } else {
            return 0;
        }
    }).sum();
    return count;
}

fn problem2(rules: &HashMap<u32, Vec<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    let count: u32 = updates.iter().map(|line| {
        if is_line_good(&line, &rules){
            return 0;
        } else {
            // FIND A BETTER AND NICEST WAY
            let line_ordered: Vec<u32> = order_line(line.clone(), &rules);
            return middle_of_line(&line_ordered);
        }
    }).sum();

    return count;
}

fn setup_data(contents: String) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut reached_updates: bool = false;
    for line in contents.lines() {
        if !reached_updates && line.len() != 0 {
            let line_split: Vec<&str> = line.split("|").collect();
            let key = line_split[1].parse::<u32>().unwrap();
            let value = line_split[0].parse::<u32>().unwrap();
            rules.entry(key).or_insert_with(||vec![]);
            rules.entry(key).and_modify(|rule| rule.push(value));
        } else if line.len() == 0 {
            reached_updates = true;
        } else {
            updates.push(line.split(",").collect::<Vec<_>>().iter().map(|el| el.parse::<u32>().unwrap()).collect());
        }
    }
    return (rules, updates);
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
    return true;
}

fn middle_of_line(line: &[u32]) -> u32 {
    return *line.get((line.len()-1)/2).unwrap()
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

                banned_valueses.entry(index).or_insert_with(||vec![]);
                banned_valueses.entry(index).and_modify( |ban: &mut Vec<u32>| {
                    if let Some(values) = rules.get(el) {
                        ban.extend(values);
                    }
                });
            }
        }
        line = new_line
    }
    return line;
}
