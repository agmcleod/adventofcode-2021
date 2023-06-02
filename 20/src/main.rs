use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn get_binary_number(grid: &HashMap<(i32, i32), String>, col: i32, row: i32) -> i32 {
    let mut numeric_values = Vec::new();

    for col_offset in -1..=1 {
        for row_offset in -1..=1 {
            let empty = ".".to_string();
            let value = grid
                .get(&(col + col_offset, row + row_offset))
                .unwrap_or(&empty);

            let n = match value.as_str() {
                "#" => 1,
                _ => 0,
            };

            numeric_values.push(n);
        }
    }

    numeric_values
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, digit)| acc + 2i32.pow(i as u32) * *digit)
}

fn main() -> Result<()> {
    let text = read_text("20/input.txt")?;

    let mut image_map = None;
    let mut row = 0;
    let mut grid = HashMap::new();

    let mut left_edge = 0;
    let mut right_edge = 0;

    for line in text.lines() {
        if image_map.is_none() {
            image_map = Some(line.chars().map(|c| c.to_string()).collect::<Vec<String>>());
        } else if !line.is_empty() {
            for (col, ch) in line.chars().enumerate() {
                grid.insert((col as i32, row), ch.to_string());
            }
            row += 1;
        }
    }

    right_edge = grid.len();

    for _ in 0..2 {
        left_edge -= 2;
        right_edge += 2;

        for col in left_edge..right_edge {
            for row in left_edge..right_edge {
                let index = get_binary_number(&grid, col, row);
            }
        }
    }

    Ok(())
}
