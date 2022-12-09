use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Result;

use read_input::read_text;

type Pos = (i32, i32);

fn get_adjacents(height_map: &HashMap<Pos, u32>, pos: &Pos) -> Vec<Pos> {
    let mut adjacents = Vec::new();

    if height_map.contains_key(&(pos.0 - 1, pos.1)) {
        adjacents.push((pos.0 - 1, pos.1));
    }
    if height_map.contains_key(&(pos.0, pos.1 - 1)) {
        adjacents.push((pos.0, pos.1 - 1));
    }
    if height_map.contains_key(&(pos.0 + 1, pos.1)) {
        adjacents.push((pos.0 + 1, pos.1));
    }
    if height_map.contains_key(&(pos.0, pos.1 + 1)) {
        adjacents.push((pos.0, pos.1 + 1));
    }

    adjacents
}

fn get_coords_for_basin(
    height_map: &HashMap<Pos, u32>,
    crawled_basins: &mut HashSet<(i32, i32)>,
    pos: &Pos,
) -> usize {
    let mut crawled_positions_for_basin = HashSet::new();

    let mut positions = VecDeque::from([pos.to_owned()]);
    while positions.len() > 0 {
        let pos = positions.pop_front().unwrap();
        crawled_positions_for_basin.insert(pos.clone());
        let adjacents = get_adjacents(height_map, &pos)
            .iter()
            .filter(|pos| {
                !crawled_positions_for_basin.contains(pos)
                    && !crawled_basins.contains(pos)
                    && *height_map.get(pos).unwrap() != 9
            })
            .cloned()
            .collect::<Vec<Pos>>();

        positions.append(&mut VecDeque::from(adjacents));
    }

    for pos in &crawled_positions_for_basin {
        crawled_basins.insert(pos.to_owned());
    }
    crawled_positions_for_basin.len()
}

fn main() -> Result<()> {
    let text = read_text("9/input.txt")?;

    let mut height_map = HashMap::new();
    let mut max_row: i32 = 0;
    let mut max_col: i32 = 0;
    for (row, line) in text.lines().enumerate() {
        max_row = cmp::max(max_row, row as i32);
        for (col, ch) in line.chars().enumerate() {
            max_col = cmp::max(max_col, col as i32);
            height_map.insert((col as i32, row as i32), ch.to_digit(10).unwrap());
        }
    }

    let mut risk_sum = 0;
    let mut basin_sizes = Vec::new();
    let mut crawled_basins = HashSet::new();
    for col in 0..=max_col {
        for row in 0..=max_row {
            let height = height_map.get(&(col, row)).unwrap();
            if *height == 9 {
                continue;
            }

            let point = height_map.get(&(col, row)).unwrap();
            let adjacents = get_adjacents(&height_map, &(col, row));
            let is_low_point = adjacents.iter().fold(true, |result, pos| {
                result && *height_map.get(pos).unwrap() > *point
            });

            if is_low_point {
                risk_sum += *point + 1;
            }

            if !crawled_basins.contains(&(col, row)) {
                let size = get_coords_for_basin(&height_map, &mut crawled_basins, &(col, row));
                basin_sizes.push(size);
            }
        }
    }

    basin_sizes.sort_by(|a, b| {
        if a > b {
            cmp::Ordering::Less
        } else if a < b {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Equal
        }
    });
    println!("{}", risk_sum);
    println!(
        "{}",
        basin_sizes[0..3].iter().fold(1, |prod, size| prod * size)
    );

    Ok(())
}
