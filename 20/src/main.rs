use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn get_binary_number(grid: &HashMap<(i32, i32), String>, col: i32, row: i32) -> usize {
    let mut numeric_values = Vec::new();

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
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
        .fold(0, |acc, (i, digit)| acc + 2i32.pow(i as u32) * *digit) as usize
}

fn main() -> Result<()> {
    let text = read_text("20/input.txt")?;

    let mut image_map = None;
    let mut row = 0;
    let mut grid = HashMap::new();

    let mut left_edge: i32 = 0;

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

    let mut right_edge: i32 = grid.len() as i32;

    for _ in 0..2 {
        left_edge -= 2;
        right_edge += 2;

        let mut next_grid = grid.clone();
        for row in left_edge..right_edge {
            for col in left_edge..right_edge {
                let index = get_binary_number(&grid, col, row);
                next_grid.insert(
                    (col, row),
                    image_map.as_ref().unwrap().get(index).unwrap().to_owned(),
                );
            }
        }

        grid = next_grid;
    }

    println!(
        "{}",
        grid.iter().fold(0, |sum, (_, v)| {
            if v == "#" {
                sum + 1
            } else {
                sum
            }
        })
    );

    // for row in left_edge..right_edge {
    //     for col in left_edge..right_edge {
    //         print!("{}", grid.get(&(col, row)).unwrap());
    //     }
    //     println!();
    // }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_binary_number() {
        let mut grid = HashMap::new();

        grid.insert((0, 0), "#".to_string());
        grid.insert((1, 0), ".".to_string());
        grid.insert((2, 0), ".".to_string());
        grid.insert((3, 0), "#".to_string());
        grid.insert((4, 0), ".".to_string());

        grid.insert((0, 1), "#".to_string());
        grid.insert((1, 1), ".".to_string());
        grid.insert((2, 1), ".".to_string());
        grid.insert((3, 1), ".".to_string());
        grid.insert((4, 1), ".".to_string());

        grid.insert((0, 2), "#".to_string());
        grid.insert((1, 2), "#".to_string());
        grid.insert((2, 2), ".".to_string());
        grid.insert((3, 2), ".".to_string());
        grid.insert((4, 2), "#".to_string());

        grid.insert((0, 3), ".".to_string());
        grid.insert((1, 3), ".".to_string());
        grid.insert((2, 3), "#".to_string());
        grid.insert((3, 3), ".".to_string());
        grid.insert((4, 3), ".".to_string());

        grid.insert((0, 4), ".".to_string());
        grid.insert((1, 4), ".".to_string());
        grid.insert((2, 4), "#".to_string());
        grid.insert((3, 4), "#".to_string());
        grid.insert((4, 4), "#".to_string());

        assert_eq!(get_binary_number(&grid, 2, 2), 34);
    }
}
