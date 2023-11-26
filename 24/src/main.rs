use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

use read_input::read_text;

type Variables = HashMap<String, i32>;

#[derive(Debug, PartialEq, Eq)]
struct ParseValueError;

#[derive(Debug)]
enum Value {
    Variable(String),
    Numerical(i32),
}

impl FromStr for Value {
    type Err = ParseValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<i32>();
        if let Ok(value) = value {
            Ok(Self::Numerical(value))
        } else {
            Ok(Self::Variable(s.to_owned()))
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Inp(String),
    Add(String, Value),
    Mul(String, Value),
    Div(String, Value),
    Mod(String, Value),
    Eql(String, Value),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError(String);

impl From<ParseValueError> for ParseInstructionError {
    fn from(err: ParseValueError) -> Self {
        ParseInstructionError(format!("Value somehow is wrong: {:?}", err))
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("inp") {
            Ok(Self::Inp(s.replace("inp ", "")))
        } else if s.starts_with("add") {
            let values: Vec<String> = get_value_tokens_from_instruction_line(s, "add");

            Ok(Self::Add(values[0].clone(), values[1].parse()?))
        } else if s.starts_with("mul") {
            let values: Vec<String> = get_value_tokens_from_instruction_line(s, "mul");

            Ok(Self::Mul(values[0].clone(), values[1].parse()?))
        } else if s.starts_with("div") {
            let values: Vec<String> = get_value_tokens_from_instruction_line(s, "div");

            Ok(Self::Div(values[0].clone(), values[1].parse()?))
        } else if s.starts_with("mod") {
            let values: Vec<String> = get_value_tokens_from_instruction_line(s, "mod");

            Ok(Self::Mod(values[0].clone(), values[1].parse()?))
        } else if s.starts_with("eql") {
            let values: Vec<String> = get_value_tokens_from_instruction_line(s, "eql");

            Ok(Self::Eql(values[0].clone(), values[1].parse()?))
        } else {
            Err(ParseInstructionError(format!(
                "Unrecognized instruction {}",
                s
            )))
        }
    }
}

fn get_value_tokens_from_instruction_line(line: &str, instruction_name: &str) -> Vec<String> {
    line.replace(&format!("{} ", instruction_name), "")
        .split(' ')
        .map(|s| s.to_owned())
        .collect()
}

fn check_variable(variables: &HashMap<String, i32>, variable: &String) {
    if !variables.contains_key(variable) {
        panic!("Invalid variable {}", variable);
    }
}

fn get_value(variables: &Variables, value: &Value) -> i32 {
    match value {
        Value::Numerical(n) => *n,
        Value::Variable(variable) => *variables.get(variable).unwrap(),
    }
}

fn run_program(
    instructions: &Vec<Instruction>,
    variables: &mut Variables,
    input: &Vec<i32>,
) -> i32 {
    let mut input_iter = input.iter();

    for instr in instructions {
        match &instr {
            Instruction::Inp(variable) => {
                check_variable(variables, variable);

                if let Some(input) = input_iter.next() {
                    variables.insert(variable.to_owned(), *input);
                } else {
                    panic!("Ran out of input {:?}", input);
                }
            }
            Instruction::Add(variable, value) => {
                check_variable(variables, variable);

                let value = get_value(variables, value);
                variables.insert(
                    variable.to_owned(),
                    *variables.get(variable).unwrap() + value,
                );
            }
            Instruction::Mul(variable, value) => {
                check_variable(variables, variable);

                let value = get_value(variables, value);
                variables.insert(
                    variable.to_owned(),
                    *variables.get(variable).unwrap() * value,
                );
            }
            Instruction::Div(variable, value) => {
                check_variable(variables, variable);

                let value = get_value(variables, value);
                if value != 0 {
                    variables.insert(
                        variable.to_owned(),
                        *variables.get(variable).unwrap() / value,
                    );
                }
            }
            Instruction::Mod(variable, value) => {
                check_variable(variables, variable);

                let value = get_value(variables, value);
                let existing_value = *variables.get(variable).unwrap();
                if existing_value >= 0 && value > 0 {
                    variables.insert(variable.to_owned(), existing_value % value);
                }
            }
            Instruction::Eql(variable, value) => {
                check_variable(variables, variable);

                let value = get_value(variables, value);
                variables.insert(
                    variable.to_owned(),
                    i32::from(*variables.get(variable).unwrap() == value),
                );
            }
        }
    }

    *variables.get("z").unwrap()
}

fn decrement_number(number_digits: &mut Vec<i32>) {
    if number_digits.len() != 14 {
        panic!(
            "Invalid length of digits, should be 14. was {}",
            number_digits.len()
        );
    }
    let mut index_to_decrement = 13;
    loop {
        let digit_value = number_digits.get_mut(index_to_decrement as usize).unwrap();
        *digit_value -= 1;
        if *digit_value == 0 {
            *digit_value = 9;
            index_to_decrement -= 1;
            if index_to_decrement < 0 {
                break;
            }
        } else {
            break;
        }
    }
}

fn main() -> io::Result<()> {
    let text = read_text("24/input.txt")?;

    let mut instructions = Vec::new();
    let mut subset = Vec::new();
    for line in text.lines() {
        let instruction: Result<Instruction, ParseInstructionError> = line.parse();
        if let Ok(instruction) = instruction {
            match instruction {
                Instruction::Inp(_) => {
                    if subset.len() > 0 {
                        instructions.push(subset);
                    }
                    subset = vec![instruction];
                }
                _ => {
                    subset.push(instruction);
                }
            }
        } else {
            panic!("Could not parse line: {}", line);
        }
    }

    if subset.len() > 0 {
        instructions.push(subset);
    }

    let mut desired_z_outcomes = HashSet::new();
    let mut list_of_valid_z_outcomes = Vec::new();

    for subset in instructions.iter().rev() {
        // first time capturing z outcomes, so set it to zero
        if desired_z_outcomes.len() == 0 {
            desired_z_outcomes.insert(0);
        }

        let mut next_outcomes = HashSet::new();
        for z in 0..1_000_000 {
            for input in 1..=9 {
                let mut variables: Variables = HashMap::new();
                variables.insert("w".to_string(), 0);
                variables.insert("x".to_string(), 0);
                variables.insert("y".to_string(), 0);
                variables.insert("z".to_string(), z);

                if desired_z_outcomes.contains(&run_program(subset, &mut variables, &vec![input])) {
                    next_outcomes.insert(z);
                }
            }
        }

        list_of_valid_z_outcomes.push(next_outcomes.iter().cloned().collect::<Vec<i32>>());
        desired_z_outcomes = next_outcomes;
    }

    list_of_valid_z_outcomes.reverse();

    let mut sol = vec![];
    let mut variables: Variables = HashMap::new();
    variables.insert("w".to_string(), 0);
    variables.insert("x".to_string(), 0);
    variables.insert("y".to_string(), 0);
    variables.insert("z".to_string(), 0);
    for (subset_i, subset) in instructions.iter().enumerate() {
        for input in (1..=9).rev() {
            let result = run_program(subset, &mut variables, &vec![input]);
            if list_of_valid_z_outcomes[subset_i].contains(&result) {
                sol.push(input);
                break;
            }
        }
    }

    println!(
        "{} {}",
        sol.iter().map(|n| n.to_string()).collect::<String>(),
        sol.len()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn number_as_vec(num: &str) -> Vec<i32> {
        num.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
    }

    #[test]
    fn test_decrement_number() {
        let mut test_num = number_as_vec("13579246899999");
        decrement_number(&mut test_num);
        assert_eq!(test_num, number_as_vec("13579246899998"));

        test_num = number_as_vec("21111111111111");
        decrement_number(&mut test_num);
        assert_eq!(test_num, number_as_vec("19999999999999"));
    }
}
