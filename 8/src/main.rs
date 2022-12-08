use core::hash::Hash;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
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

fn insert_once<K, V>(map: &mut HashMap<K, V>, key: K, value: V)
where
    K: Eq + Hash + Debug + Display,
    V: Debug,
{
    if map.contains_key(&key) {
        panic!("Already have {} for {:?}", key, map);
    }

    map.insert(key, value);
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
                    insert_once(transposition_map, "g".to_string(), g_digit.clone());
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
                    insert_once(transposition_map, "e".to_owned(), digit);
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

fn solve_digit_for_f(
    signal_patterns: &Vec<HashSet<String>>,
    transposition_map: &mut HashMap<String, String>,
    number_to_digits: &mut HashMap<i32, HashSet<String>>,
) {
    for pattern in signal_patterns {
        match pattern.len() {
            5 => {
                // can be 2, 3 or 5
                let mut digit_for_f = None;
                {
                    let digit_for_a = transposition_map.get("a").unwrap();
                    let digit_for_c = transposition_map.get("c").unwrap();
                    let digit_for_d = transposition_map.get("d").unwrap();
                    let digit_for_e = transposition_map.get("e").unwrap();
                    let digit_for_g = transposition_map.get("g").unwrap();

                    // it's a 2, let's insert it
                    if pattern.contains(digit_for_a)
                        && pattern.contains(digit_for_c)
                        && pattern.contains(digit_for_d)
                        && pattern.contains(digit_for_e)
                        && pattern.contains(digit_for_g)
                    {
                        insert_once(number_to_digits, 2, pattern.clone());
                    }

                    // it's a 3
                    if !pattern.contains(digit_for_e) && pattern.contains(digit_for_c) {
                        for digit in pattern {
                            // digit must be f
                            if digit != digit_for_a
                                && digit != digit_for_c
                                && digit != digit_for_d
                                && digit != digit_for_g
                            {
                                digit_for_f = Some(digit.to_owned());
                            }
                        }

                        insert_once(number_to_digits, 3, pattern.clone());
                    }
                };

                if let Some(digit_for_f) = digit_for_f {
                    insert_once(transposition_map, "f".to_owned(), digit_for_f);
                }
            }
            _ => {}
        }
    }

    if !transposition_map.contains_key("f") {
        panic!("Could not find f in 3: {:?}", signal_patterns);
    }
}

fn solve_digit_for_b(
    signal_patterns: &Vec<HashSet<String>>,
    transposition_map: &mut HashMap<String, String>,
    number_to_digits: &mut HashMap<i32, HashSet<String>>,
) {
    for pattern in signal_patterns {
        match pattern.len() {
            5 => {
                // can be 2, 3 or 5
                let mut digit_for_b = None;
                {
                    let digit_for_a = transposition_map.get("a").unwrap();
                    let digit_for_c = transposition_map.get("c").unwrap();
                    let digit_for_d = transposition_map.get("d").unwrap();
                    let digit_for_e = transposition_map.get("e").unwrap();
                    let digit_for_f = transposition_map.get("f").unwrap();
                    let digit_for_g = transposition_map.get("g").unwrap();

                    // it's a 5
                    if !pattern.contains(digit_for_e) && !pattern.contains(digit_for_c) {
                        for digit in pattern {
                            // digit must be b
                            if digit != digit_for_a
                                && digit != digit_for_d
                                && digit != digit_for_f
                                && digit != digit_for_g
                            {
                                digit_for_b = Some(digit.to_owned());
                            }
                        }

                        insert_once(number_to_digits, 5, pattern.clone());
                    }
                };

                if let Some(digit_for_f) = digit_for_b {
                    insert_once(transposition_map, "b".to_owned(), digit_for_f);
                }
            }
            _ => {}
        }
    }

    if !transposition_map.contains_key("b") {
        panic!("Could not find b in 5: {:?}", signal_patterns);
    }
}

fn find_six(
    signal_patterns: &Vec<HashSet<String>>,
    number_to_digits: &mut HashMap<i32, HashSet<String>>,
) {
    for pattern in signal_patterns {
        match pattern.len() {
            6 => {
                let is_six = {
                    let mut is_six = false;
                    let digits_for_four = get_from_number_map(number_to_digits, 4, signal_patterns);
                    let digits_for_one = get_from_number_map(number_to_digits, 1, signal_patterns);
                    // get b & d from 4
                    let remaining_digits: Vec<&String> = digits_for_four
                        .iter()
                        .filter(|digit| !digits_for_one.contains(*digit))
                        .collect();

                    let mut is_not_nine = false;
                    // verify this number is not a 9
                    let digits_for_nine = get_from_number_map(number_to_digits, 9, signal_patterns);
                    for digit in pattern {
                        if !digits_for_nine.contains(digit) {
                            is_not_nine = true;
                        }
                    }

                    if is_not_nine {
                        let digit_contains_b_and_d = remaining_digits
                            .iter()
                            .fold(true, |result, digit| result && pattern.contains(*digit));

                        // is not 9, and contains b & d, so it must be six and not zero
                        if digit_contains_b_and_d {
                            is_six = true;
                        }
                    }

                    is_six
                };

                if is_six {
                    insert_once(number_to_digits, 6, pattern.clone());
                }
            }
            _ => {}
        }
    }
}

fn find_zero(
    signal_patterns: &Vec<HashSet<String>>,
    transposition_map: &mut HashMap<String, String>,
    number_to_digits: &mut HashMap<i32, HashSet<String>>,
) {
    for pattern in signal_patterns {
        match pattern.len() {
            6 => {
                let is_zero = {
                    let digits_for_nine = get_from_number_map(number_to_digits, 9, signal_patterns);
                    let digits_for_six = get_from_number_map(number_to_digits, 6, signal_patterns);

                    let is_nine = digits_for_nine
                        .iter()
                        .fold(true, |result, digit| result && pattern.contains(digit));

                    let is_six = digits_for_six
                        .iter()
                        .fold(true, |result, digit| result && pattern.contains(digit));

                    // must be zero
                    !is_six && !is_nine
                };

                if is_zero {
                    number_to_digits.insert(0, pattern.clone());

                    let digits_for_six = get_from_number_map(number_to_digits, 6, signal_patterns);

                    for digit in pattern {
                        // six doesnt contain a digit from zero, must be c
                        if !digits_for_six.contains(digit) {
                            insert_once(transposition_map, "c".to_owned(), digit.to_owned());
                        }
                    }

                    for digit in digits_for_six {
                        // zero doesnt contain a digit from 6, must be d
                        if !pattern.contains(digit) {
                            insert_once(transposition_map, "d".to_owned(), digit.to_owned());
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let text = read_text("8/input.txt")?;

    let mut p1_sum = 0;
    let mut p2_sum = 0;

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
        find_six(&signal_patterns, &mut number_to_digits);
        find_zero(
            &signal_patterns,
            &mut transposition_map,
            &mut number_to_digits,
        );
        solve_digit_for_f(
            &signal_patterns,
            &mut transposition_map,
            &mut number_to_digits,
        );
        solve_digit_for_b(
            &signal_patterns,
            &mut transposition_map,
            &mut number_to_digits,
        );

        let mut digit = Vec::new();
        for pattern in &output_patterns {
            let set = pattern_to_set(pattern);
            for (num, solved_pattern) in &number_to_digits {
                if set == *solved_pattern {
                    digit.push(num.to_string());
                }
            }
        }

        if digit.len() != 4 {
            panic!("Invalid digit set {:?}", output_patterns);
        }

        p2_sum += digit.join("").parse::<i32>().unwrap();
    }

    println!("{}", p1_sum);
    println!("{}", p2_sum);

    Ok(())
}
