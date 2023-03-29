use std::io::Result;

use read_input::read_text;

fn explode(mut this_iteration: Vec<String>, pos: usize) -> Vec<String> {
    let left_digit: usize = this_iteration[pos + 1].parse().unwrap();
    let right_digit: usize = this_iteration[pos + 3].parse().unwrap();
    // going left
    for i in (0..pos).rev() {
        let ch = this_iteration.get(i).unwrap();
        if ch != "[" && ch != "]" && ch != "," {
            let n: usize = ch.parse().unwrap();
            this_iteration[i] = format!("{}", n + left_digit);
            break;
        }
    }

    // going right
    for i in pos + 3..this_iteration.len() {
        let ch = this_iteration.get(i).unwrap();
        if ch != "[" && ch != "]" && ch != "," {
            let n: usize = ch.parse().unwrap();
            this_iteration[i] = format!("{}", n + right_digit);
            break;
        }
    }

    for _ in 0..5 {
        this_iteration.remove(pos);
    }

    this_iteration.insert(pos, "0".to_string());

    this_iteration
}

fn split(mut this_iteration: Vec<String>, pos: usize, digits: &Vec<String>) -> Vec<String> {
    for _ in 0..digits.len() {
        this_iteration.remove(pos);
    }

    let number_to_split: usize = digits.join("").parse().unwrap();
    let div_two = number_to_split / 2;
    let mut right = div_two;

    if number_to_split % 2 != 0 {
        right += 1;
    }

    this_iteration.insert(pos, "[".to_string());
    this_iteration.insert(pos, div_two.to_string());
    this_iteration.insert(pos, ",".to_string());
    this_iteration.insert(pos, right.to_string());
    this_iteration.insert(pos, "]".to_string());

    this_iteration
}

fn main() -> Result<()> {
    let text = read_text("18/input.txt")?;

    for line in text.lines() {
        let mut characters = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        loop {
            let mut this_iteration = characters.clone();

            let mut depth = 0;
            let mut digits = Vec::new();
            for (i, ch) in characters.iter().enumerate() {
                if ch == "[" {
                    digits.clear();
                    depth += 1;
                    if depth == 5 {
                        this_iteration = explode(this_iteration, i);
                        break;
                    }
                } else if ch == "]" {
                    depth -= 1;
                    digits.clear();
                } else if ch == "," {
                    digits.clear();
                } else {
                    digits.push(ch.to_owned());
                    if digits.len() > 1 {
                        this_iteration = split(this_iteration, i, &digits);
                        break;
                    }
                }
            }

            characters = this_iteration;
        }
    }

    Ok(())
}
