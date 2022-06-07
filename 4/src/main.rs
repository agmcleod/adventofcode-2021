use std::collections::HashSet;
use std::io::{self};
use std::str::Lines;

use read_input::read_text;
use regex::Regex;

#[derive(Clone)]
struct Board {
    called_numbers: HashSet<String>,
    numbers: HashSet<String>,
    grid: Vec<Vec<String>>,
}

fn parse_boards(lines: Lines) -> Vec<Board> {
    let mut current_board: Option<Board> = None;
    let mut boards: Vec<Board> = Vec::new();
    let spaces_regex = Regex::new(r"\s+").unwrap();
    for line in lines {
        if line == "" {
            if current_board.is_some() {
                boards.push(current_board.unwrap().clone());
            }
            current_board = None;
        } else {
            if current_board.is_none() {
                current_board = Some(Board {
                    called_numbers: HashSet::new(),
                    numbers: HashSet::new(),
                    grid: Vec::new(),
                });
            }

            let board = current_board.as_mut().unwrap();

            let row_numbers: Vec<&str> = spaces_regex.split(line.trim()).collect();
            board.grid.push(Vec::new());
            for n in row_numbers {
                board.numbers.insert(n.to_owned());
                board.grid.last_mut().unwrap().push(n.to_owned());
            }
        }
    }

    boards
}

fn check_if_board_has_won(board: &Board) -> bool {
    for row in &board.grid {
        if row
            .iter()
            .filter(|col| board.called_numbers.contains(*col))
            .count()
            == board.grid.len()
        {
            return true;
        }
    }

    // maybe inefficient as we go through the board twice
    for col in 0..board.grid.len() {
        if (0..board.grid.len())
            .filter(|row| board.called_numbers.contains(&board.grid[*row][col]))
            .count()
            == board.grid.len()
        {
            return true;
        }
    }

    false
}

fn parse_num(num: &str) -> i32 {
    match num.parse::<i32>() {
        Ok(n) => n,
        Err(_) => panic!("Could not parse {}", num),
    }
}

fn main() -> io::Result<()> {
    let text = read_text("4/input.txt")?;

    let mut lines = text.lines();
    let callout_numbers: Vec<&str> = lines.next().unwrap().split(",").collect();
    let mut boards = parse_boards(lines);

    'outer: for number in callout_numbers {
        for board in &mut boards {
            if board.numbers.contains(number) {
                board.called_numbers.insert(number.to_owned());
            }

            // possible this may need to happen as a second loop
            if check_if_board_has_won(board) {
                let mut sum = 0;
                for row in &board.grid {
                    for col in row {
                        if !board.called_numbers.contains(col) {
                            sum += parse_num(col);
                        }
                    }
                }
                println!("{}", sum * parse_num(number));
                break 'outer;
            }
        }
    }

    Ok(())
}
