use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("20/input.txt")?;

    let mut image_map = None;
    let mut row = 0;
    let mut grid = HashMap::new();

    for line in text.lines() {
        if image_map.is_none() {
            image_map = Some(line.to_owned());
        } else if !line.is_empty() {
            for (col, ch) in line.chars().enumerate() {
                grid.insert((col, row), ch.to_string());
            }
            row += 1;
        }
    }

    Ok(())
}
