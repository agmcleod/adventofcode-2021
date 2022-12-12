use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn get_surrounding_octopi(
    octopi: &HashMap<(usize, usize), u32>,
    center: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut surrounding = Vec::new();

    for offset_x in -1i32..=1i32 {
        for offset_y in -1i32..=1i32 {
            if offset_x == 0 && offset_y == 0 {
                continue;
            }

            let x = (center.0 as i32 + offset_x) as usize;
            let y = (center.1 as i32 + offset_y) as usize;
            if let Some(_octopus) = octopi.get(&(x, y)) {
                surrounding.push((x, y));
            }
        }
    }

    surrounding
}

fn main() -> Result<()> {
    let text = read_text("11/input.txt")?;

    let mut grid = HashMap::new();

    for (row, line) in text.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.insert((col, row), ch.to_digit(10).unwrap());
        }
    }

    let mut flash_count = 0;
    let mut step_count = 0;
    loop {
        for (_, octopus) in &mut grid {
            *octopus += 1;
        }

        loop {
            let mut next_grid = grid.clone();
            let mut should_continue = false;
            for (coord, octopus) in &grid {
                if *octopus > 9 {
                    // condition for p1
                    if step_count < 100 {
                        flash_count += 1;
                    }
                    // mark this one as flashed
                    next_grid.insert(coord.to_owned(), 0);
                    for neighbour_coord in &get_surrounding_octopi(&grid, coord) {
                        if let Some(octopus) = next_grid.get_mut(neighbour_coord) {
                            if *octopus > 0 {
                                *octopus += 1;
                                if *octopus > 9 {
                                    should_continue = true;
                                }
                            }
                        }
                    }
                }
            }

            grid = next_grid;
            if !should_continue {
                break;
            }
        }
        if step_count == 99 {
            println!("p1 {}", flash_count);
        }

        if grid
            .iter()
            .filter(|&(_coord, octopus)| *octopus == 0)
            .count()
            == grid.len()
        {
            println!("p2 {}", step_count + 1);
            break;
        }

        step_count += 1;
    }

    Ok(())
}
