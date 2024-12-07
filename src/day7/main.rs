#[derive(Clone)]
struct Equation {
    result: u64,
    valueses: Vec<u64>,
    operations: Vec<Operation>
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Operation {
    Sum,
    Multiply,
    Concatenation
}

fn main() {
let contents = include_str!("input.txt");
// for problem 1 just remove concatenation from enum and func below
print!("{}\n", problem2(contents.to_string()));
}

fn problem2(contents: String) -> u64 {
    let mut count = 0;
    for line in contents.lines() {
        print!("evaluating {}\n", line);
        let split_line: Vec<&str> = line.split(": ").collect::<>();
        let valueses: Vec<u64> = split_line[1].split(" ").collect::<Vec<&str>>().iter().map(|el| el.parse::<u64>().unwrap()).collect();
        let eq = Equation {
            result: split_line[0].parse::<u64>().unwrap(),
            valueses: valueses.clone(),
            operations: vec![Operation::Sum;valueses.len()-1]
        };
        
        if is_valid(&mut eq.clone(), 0) {
            count += eq.result;
        }
    }
    return count;
}

fn is_valid(equation: &mut Equation, depth: usize) -> bool {
    if depth >= equation.operations.len() {
        return operate(equation) == equation.result;
    }

    equation.operations[depth] = Operation::Sum;
    if is_valid(equation,depth + 1) {
        return true;
    }
    equation.operations[depth] = Operation::Multiply;
    if is_valid(equation,depth + 1) {
        return true;
    }
    equation.operations[depth] = Operation::Concatenation;
    if is_valid(equation,depth + 1) {
        return true;
    }
    return false
}

fn operate(equation: &mut Equation) -> u64 {
    let mut result = equation.valueses[0];
    for (index, operation) in equation.operations.iter().enumerate() {
        if matches!(operation, Operation::Sum) {
            result += equation.valueses[index+1];
        } else if matches!(operation, Operation::Multiply) {
            result *= equation.valueses[index+1];
        } else if matches!(operation, Operation::Concatenation) {
            let value = equation.valueses[index+1];
            result = format!("{result}{value}").parse::<u64>().unwrap();
        }
    }
    return result;
}