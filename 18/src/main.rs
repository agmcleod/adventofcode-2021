use std::fmt;
use std::io::Result;
use std::slice::Iter;

use read_input::read_text;

#[derive(PartialEq)]
enum Pair {
    None,
    Pair(Box<(Pair, Pair)>),
    Value(usize),
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pair::None => {
                write!(f, "")
            }
            Pair::Pair(pair) => {
                write!(f, "[{},{}]", pair.0, pair.1)
            }
            Pair::Value(n) => write!(f, "{}", n),
        }
    }
}

fn explode(mut this_iteration: Vec<String>, pos: usize) -> Vec<String> {
    // add 1 since the position is that of the left bracket
    let left_digit: usize = this_iteration[pos + 1].parse().unwrap();
    let right_digit: usize = this_iteration[pos + 3].parse().unwrap();
    // println!(
    //     "explode {} {}, digits: {},{}",
    //     pos,
    //     this_iteration.join(""),
    //     left_digit,
    //     right_digit
    // );
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
    for i in pos + 4..this_iteration.len() {
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

    // println!("explode result {} {}", pos, this_iteration.join(""));

    this_iteration
}

fn split(mut this_iteration: Vec<String>, pos: usize, digits: &str, depth: usize) -> Vec<String> {
    // println!(
    //     "Split i: {}, num: {} {}",
    //     pos,
    //     digits,
    //     this_iteration.join("")
    // );
    this_iteration.remove(pos);

    // println!("Removed digits {}", this_iteration.join(""));

    let number_to_split: usize = digits.parse().unwrap();
    let div_two = number_to_split / 2;
    let mut right = div_two;

    if number_to_split % 2 != 0 {
        right += 1;
    }

    this_iteration.insert(pos, "[".to_string());
    this_iteration.insert(pos + 1, div_two.to_string());
    this_iteration.insert(pos + 2, ",".to_string());
    this_iteration.insert(pos + 3, right.to_string());
    this_iteration.insert(pos + 4, "]".to_string());

    if depth == 4 {
        return explode(this_iteration, pos);
    }

    // println!("Inserted after split {} {}", pos, this_iteration.join(""));

    this_iteration
}

fn create_recursive_pairs(iter: &mut Iter<String>, mut pair: Pair) -> Pair {
    loop {
        let ch = iter.next();
        if ch.is_none() {
            break;
        }
        let ch = ch.unwrap();

        if ch == "[" {
            match create_recursive_pairs(iter, Pair::Pair(Box::new((Pair::None, Pair::None)))) {
                Pair::None => {
                    panic!("Returned None after a left bracket.");
                }
                Pair::Pair(returned_pair) => match &mut pair {
                    Pair::Pair(pair) => {
                        if pair.0 == Pair::None {
                            pair.0 = Pair::Pair(returned_pair);
                        } else if pair.1 == Pair::None {
                            pair.1 = Pair::Pair(returned_pair);
                        } else {
                            panic!(
                                "Pair already populated for trying to populate returned pair from sub level"
                            );
                        }
                    }
                    Pair::None => {
                        pair = Pair::Pair(returned_pair);
                    }
                    _ => panic!("unexpected non-pair type for this level's pair value"),
                },
                Pair::Value(_value) => {
                    panic!("Should not have returned single value");
                }
            }
        } else if ch == "]" {
            return pair;
        } else if ch != "," {
            let n: usize = ch.parse().unwrap();

            match &mut pair {
                Pair::Pair(pair) => {
                    if pair.0 == Pair::None {
                        pair.0 = Pair::Value(n);
                    } else if pair.1 == Pair::None {
                        pair.1 = Pair::Value(n);
                    } else {
                        panic!("Pair already populated for trying to populate number {}", n);
                    }
                }
                _ => panic!("unexpected non-pair type for this level's pair value"),
            }
        }
    }

    pair
}

fn calculate_magnitude(sf_number: Pair) -> usize {
    match sf_number {
        Pair::Pair(pair) => 3 * calculate_magnitude(pair.0) + 2 * calculate_magnitude(pair.1),
        Pair::Value(n) => n,
        _ => panic!("unexpected None"),
    }
}

fn main() -> Result<()> {
    let text = read_text("18/input.txt")?;

    let mut added_result = Vec::<String>::new();

    for (i, line) in text.lines().enumerate() {
        let mut characters = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        if added_result.is_empty() {
            added_result.append(&mut characters);
            continue;
        } else {
            added_result.insert(0, "[".to_string());
            added_result.push(",".to_string());
            added_result.append(&mut characters);
            added_result.push("]".to_string());
        }

        // println!("line {} {}", i + 1, added_result.join(""));

        loop {
            let mut this_iteration = added_result.clone();

            let mut depth = 0;
            let mut operation_occurred = false;
            for (i, ch) in added_result.iter().enumerate() {
                if ch == "[" {
                    depth += 1;
                    if depth == 5 {
                        this_iteration = explode(this_iteration, i);
                        operation_occurred = true;
                        break;
                    }
                } else if ch == "]" {
                    depth -= 1;
                }
            }

            added_result = this_iteration;

            if !operation_occurred {
                break;
            }
        }

        loop {
            let mut this_iteration = added_result.clone();
            let mut operation_occurred = false;
            let mut depth = 0;
            for (i, ch) in added_result.iter().enumerate() {
                if ch == "[" {
                    depth += 1;
                } else if ch == "]" {
                    depth -= 1;
                } else if ch != "," && ch.len() > 1 {
                    this_iteration = split(this_iteration, i, ch, depth);
                    operation_occurred = true;
                    break;
                }
            }

            added_result = this_iteration;

            if !operation_occurred {
                break;
            }
        }

        // println!("{}", added_result.join(""));
    }

    // println!("{}", added_result.join(""));
    let mut iter = added_result.iter();
    let pair = create_recursive_pairs(&mut iter, Pair::None);
    // println!("{}", pair);
    println!("{}", calculate_magnitude(pair));

    Ok(())
}
