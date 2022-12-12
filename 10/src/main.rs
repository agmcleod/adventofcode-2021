use std::collections::VecDeque;
use std::io::Result;

use read_input::read_text;

fn get_closing_for_opening(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => {
            panic!("Unrecognized character: {}", opening);
        }
    }
}

fn closes_the_opening_bracket(opening: char, closing: char) -> bool {
    get_closing_for_opening(opening) == closing
}

fn main() -> Result<()> {
    let text = read_text("10/input.txt")?;

    let mut illegal_chars = Vec::new();
    let mut p2_scores = Vec::new();
    for line in text.lines() {
        let mut open_brackets = VecDeque::new();
        let mut is_illegal = false;
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    open_brackets.push_back(ch);
                }
                ')' | ']' | '}' | '>' => {
                    let opening = open_brackets.pop_back().unwrap();
                    if !closes_the_opening_bracket(opening, ch) {
                        is_illegal = true;
                        illegal_chars.push(ch);
                        break;
                    }
                }
                _ => {
                    panic!("Unrecognized character: {}", ch);
                }
            }
        }

        if !is_illegal && open_brackets.len() > 0 {
            let mut score: usize = 0;
            while open_brackets.len() > 0 {
                let opening = open_brackets.pop_back().unwrap();
                let closing = get_closing_for_opening(opening);
                score *= 5;
                score += match closing {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => {
                        panic!("Unrecognized character: {}", closing);
                    }
                };
            }

            p2_scores.push(score);
        }
    }

    let p1_score = illegal_chars.iter().fold(0, |sum, ch| match ch {
        ')' => sum + 3,
        ']' => sum + 57,
        '}' => sum + 1197,
        '>' => sum + 25137,
        _ => {
            panic!("Unrecognized character: {}", ch);
        }
    });
    println!("{}", p1_score);

    p2_scores.sort();
    println!("{:?}", p2_scores.get(p2_scores.len() / 2));

    Ok(())
}
