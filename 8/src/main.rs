use core::num;
use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

fn pattern_to_set(pattern: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    for ch in pattern.chars() {
        set.insert(ch.to_string());
    }

    set
}

fn get_from_number_map<'a>(
    map: &'a HashMap<i32, HashSet<String>>,
    key: i32,
    patterns: &Vec<HashSet<String>>,
) -> &'a HashSet<String> {
    if let Some(pattern) = map.get(&key) {
        pattern
    } else {
        panic!("Line: {:?} didn't have a pattern for 4", patterns);
    }
}

fn solve_digit_for_g(
    signal_patterns: &Vec<HashSet<String>>,
    transposition_map: &mut HashMap<String, String>,
    number_to_digits: &mut HashMap<i32, HashSet<String>>,
) {
    for pattern in signal_patterns {
        match pattern.len() {
            6 => {
                // can be a 0 a 6 or a 9
                let mut count = 0;
                let mut digit_not_in_the_pattern = None;
                for digit in pattern {
                    let digits_for_four = get_from_number_map(number_to_digits, 4, signal_patterns);
                    let digit_for_a = transposition_map.get("a").unwrap();
                    if digit == digit_for_a || digits_for_four.contains(digit) {
                        count += 1;
                    } else {
                        digit_not_in_the_pattern = Some(digit.to_owned());
                    }
                }

                // we must have a 9
                if count == 5 {
                    let g_digit = digit_not_in_the_pattern.unwrap();
                    transposition_map.insert("g".to_string(), g_digit.clone());
                    let mut new_pattern = pattern.clone();
                    new_pattern.insert(g_digit);
                    number_to_digits.insert(9, new_pattern);
                }
            }
            _ => {}
        }
    }
}

fn solve_digit_for_e(
    signal_patterns: &Vec<HashSet<String>>,
    transposition_map: &mut HashMap<String, String>,
    number_to_digits: &mut HashMap<i32, HashSet<String>>,
) {
    for pattern in signal_patterns {
        match pattern.len() {
            6 => {
                // can be a 0 a 6 or a 9
                let mut digit_not_in_the_pattern = None;
                for digit in pattern {
                    let digits_for_nine = get_from_number_map(number_to_digits, 9, signal_patterns);
                    if !digits_for_nine.contains(digit) {
                        digit_not_in_the_pattern = Some(digit.to_owned());
                        break;
                    }
                }

                if let Some(digit) = digit_not_in_the_pattern {
                    transposition_map.insert("e".to_owned(), digit);
                    break;
                }
            }
            _ => {}
        }
    }

    if !transposition_map.contains_key("e") {
        panic!("Did not find e in 6 or 0 when it should have");
    }
}

fn main() -> Result<()> {
    let text = read_text("8/input.txt")?;

    let mut segment_display_map = HashMap::new();
    segment_display_map.insert(0, HashSet::from(["a", "b", "c", "e", "f", "g"]));
    segment_display_map.insert(1, HashSet::from(["c", "f"]));
    segment_display_map.insert(2, HashSet::from(["a", "c", "d", "e", "g"]));
    segment_display_map.insert(3, HashSet::from(["a", "c", "d", "f", "g"]));
    segment_display_map.insert(4, HashSet::from(["b", "c", "d", "f"]));
    segment_display_map.insert(5, HashSet::from(["a", "b", "d", "f", "g"]));
    segment_display_map.insert(6, HashSet::from(["a", "b", "d", "e", "f", "g"]));
    segment_display_map.insert(7, HashSet::from(["a", "c", "f"]));
    segment_display_map.insert(8, HashSet::from(["a", "b", "c", "d", "e", "f", "g"]));
    segment_display_map.insert(9, HashSet::from(["a", "b", "c", "d", "f", "g"]));

    let mut p1_sum = 0;

    for line in text.lines() {
        let mut parts = line.split(" | ");
        let signal_patterns = parts.next().unwrap().split(" ").collect::<Vec<&str>>();

        let output_patterns = parts.next().unwrap().split(" ").collect::<Vec<&str>>();

        // p1 stuff
        for pattern in &output_patterns {
            match pattern.len() {
                2 => {
                    // it's a 1
                    p1_sum += 1;
                }
                3 => {
                    // a 7
                    p1_sum += 1;
                }
                4 => {
                    // a 4
                    p1_sum += 1;
                }
                7 => {
                    // an 8
                    p1_sum += 1;
                }
                _ => {}
            }
        }

        // the map where we store character substitutions
        let mut transposition_map = HashMap::new();
        // pass through all singular patterns in here to determine the numbers
        let mut number_to_digits = HashMap::new();
        // change the pattern values into hash sets so we can search & iterate more easily
        let signal_patterns: Vec<HashSet<String>> = signal_patterns
            .iter()
            .map(|pattern| pattern_to_set(pattern))
            .collect();
        for pattern in &signal_patterns {
            match pattern.len() {
                2 => {
                    number_to_digits.insert(1, pattern.clone());
                }
                3 => {
                    number_to_digits.insert(7, pattern.clone());
                }
                4 => {
                    number_to_digits.insert(4, pattern.clone());
                }
                7 => {
                    number_to_digits.insert(8, pattern.clone());
                }
                _ => {}
            }
        }

        let digits_for_one = get_from_number_map(&number_to_digits, 1, &signal_patterns);
        // find what the transpoition is for a
        for digit in get_from_number_map(&number_to_digits, 7, &signal_patterns) {
            if !digits_for_one.contains(digit) {
                transposition_map.insert("a".to_string(), digit.to_owned());
            }
        }

        solve_digit_for_g(
            &signal_patterns,
            &mut transposition_map,
            &mut number_to_digits,
        );
        solve_digit_for_e(
            &signal_patterns,
            &mut transposition_map,
            &mut number_to_digits,
        );
    }

    println!("{}", p1_sum);

    Ok(())
}
