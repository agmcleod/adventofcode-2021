use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn get_binary_number(grid: &HashMap<(i32, i32), String>, col: i32, row: i32) -> usize {
    let mut numeric_values = Vec::new();

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            let value = grid.get(&(col + col_offset, row + row_offset));
            if value.is_none() {
                println!(
                    "coord: {:?}, finding {:?}",
                    (col, row),
                    (col + col_offset, row + row_offset)
                );
            }

            let n = match value.unwrap().as_ref() {
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

fn display_grid(grid: &HashMap<(i32, i32), String>, top_left_edge: i32, bottom_right_edge: i32) {
    for row in top_left_edge..=bottom_right_edge {
        for col in top_left_edge..=bottom_right_edge {
            if grid.contains_key(&(col, row)) {
                print!("{}", grid.get(&(col, row)).unwrap());
            } else {
                print!("-");
            }
        }
        println!();
    }
}

fn pad_grid_with_range(
    grid: &mut HashMap<(i32, i32), String>,
    image_map: &[String],
    step: i32,
    from: (i32, i32),
    to: (i32, i32),
    force: bool,
) {
    let v = if step % 2 == 0 {
        ".".to_string()
    } else {
        image_map.get(0).unwrap().clone()
    };
    for r in from.1..=to.1 {
        for c in from.0..=to.0 {
            if force {
                grid.insert((c, r), v.clone());
            } else {
                grid.entry((c, r)).or_insert_with(|| v.clone());
            }
        }
    }
}

fn pad_grid(
    grid: &mut HashMap<(i32, i32), String>,
    image_map: &[String],
    step: i32,
    top_left_edge: i32,
    bottom_right_edge: i32,
) {
    // top two rows
    pad_grid_with_range(
        grid,
        image_map,
        step,
        (top_left_edge, top_left_edge),
        (bottom_right_edge, top_left_edge + 1),
        false,
    );

    // bottom two rows
    pad_grid_with_range(
        grid,
        image_map,
        step,
        (top_left_edge, bottom_right_edge - 1),
        (bottom_right_edge, bottom_right_edge),
        false,
    );

    // left two rows
    pad_grid_with_range(
        grid,
        image_map,
        step,
        (top_left_edge, top_left_edge),
        (top_left_edge + 1, bottom_right_edge),
        false,
    );

    // right two rows
    pad_grid_with_range(
        grid,
        image_map,
        step,
        (bottom_right_edge - 1, top_left_edge),
        (bottom_right_edge, bottom_right_edge),
        false,
    );

    // println!("finished padding:");
    // display_grid(grid, top_left_edge, bottom_right_edge);
}

fn main() -> Result<()> {
    let text = read_text("20/input.txt")?;

    let mut image_map = None;
    let mut row = 0;
    let mut grid = HashMap::new();

    let mut top_left_edge: i32 = 0;
    let mut bottom_right_edge: i32 = 0;

    for line in text.lines() {
        if image_map.is_none() {
            image_map = Some(line.chars().map(|c| c.to_string()).collect::<Vec<String>>());
        } else if !line.is_empty() {
            for (col, ch) in line.chars().enumerate() {
                bottom_right_edge = bottom_right_edge.max(col as i32);
                grid.insert((col as i32, row), ch.to_string());
            }
            row += 1;
        }
    }

    top_left_edge -= 2;
    bottom_right_edge += 2;

    let image_map = image_map.as_ref().unwrap();

    pad_grid(&mut grid, image_map, 0, top_left_edge, bottom_right_edge);

    for step in 0..50 {
        let mut next_grid = grid.clone();
        for row in (top_left_edge + 1)..=(bottom_right_edge - 1) {
            for col in (top_left_edge + 1)..=(bottom_right_edge - 1) {
                let index = get_binary_number(&grid, col, row);

                let resulting_value = image_map.get(index).unwrap().to_owned();

                next_grid.insert((col, row), resulting_value);
            }
        }

        grid = next_grid;

        let outer_edge_step = if step % 2 == 0 { 1 } else { 0 };

        // replace outer edges with next step of padding
        pad_grid_with_range(
            &mut grid,
            image_map,
            outer_edge_step,
            (top_left_edge, top_left_edge),
            (bottom_right_edge, top_left_edge),
            true,
        );
        pad_grid_with_range(
            &mut grid,
            image_map,
            outer_edge_step,
            (top_left_edge, top_left_edge),
            (top_left_edge, bottom_right_edge),
            true,
        );
        pad_grid_with_range(
            &mut grid,
            image_map,
            outer_edge_step,
            (top_left_edge, bottom_right_edge),
            (bottom_right_edge, bottom_right_edge),
            true,
        );
        pad_grid_with_range(
            &mut grid,
            image_map,
            outer_edge_step,
            (bottom_right_edge, top_left_edge),
            (bottom_right_edge, bottom_right_edge),
            true,
        );

        // if step % 2 == 0 {
        top_left_edge -= 2;
        bottom_right_edge += 2;

        pad_grid(
            &mut grid,
            image_map,
            step + 1,
            top_left_edge,
            bottom_right_edge,
        );
        // }
    }

    // println!("result");
    // display_grid(&grid, top_left_edge, bottom_right_edge);

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

        assert_eq!(get_binary_number(&mut grid, 2, 2), 34);
    }
}
