use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

type Grid = Vec<Vec<i32>>;
type Position = (i32, i32);

#[derive(Eq)]
struct Location {
    pos: Position,
    score: i32,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        other.score.cmp(&self.score)
    }
}

// fn distance_to_target(location: &Position, target: &Position) -> i32 {
//     let mut x_diff = location.1 - target.1;
//     let mut y_diff = location.0 - target.0;
//     if x_diff < 0 {
//         x_diff *= -1;
//     }
//     if y_diff < 0 {
//         y_diff *= -1;
//     }

//     x_diff + y_diff
// }

fn get_adjacents(grid: &Grid, pos: &Position) -> Vec<Position> {
    let mut adjancents = Vec::new();
    if pos.0 - 1 >= 0 {
        adjancents.push((pos.0 - 1, pos.1));
    }
    if pos.0 + 1 < grid.len() as i32 {
        adjancents.push((pos.0 + 1, pos.1));
    }
    if pos.1 - 1 >= 0 {
        adjancents.push((pos.0, pos.1 - 1));
    }
    if pos.1 + 1 < grid.len() as i32 {
        adjancents.push((pos.0, pos.1 + 1));
    }

    adjancents
}

fn main() -> Result<()> {
    let text = read_text("15/input.txt")?;

    let mut grid = Vec::with_capacity(text.lines().count());

    for line in text.lines() {
        let mut row = Vec::with_capacity(line.chars().count());
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }

    let mut heap: BinaryHeap<Location> = BinaryHeap::new();
    heap.push(Location {
        pos: (0, 0),
        score: 0,
    });
    let mut costs = HashMap::new();
    costs.insert((0, 0), 0);
    let mut closed = HashMap::new();

    let mut tracked_positions = Vec::new();

    while let Some(location) = heap.pop() {
        if location.pos == (9, 9) {
            let mut pos: &(i32, i32) = closed.get(&location.pos).unwrap();
            tracked_positions.push(location.pos);
            loop {
                if let Some(p) = closed.get(pos) {
                    tracked_positions.push(p.to_owned());
                    pos = p;
                } else {
                    break;
                }
            }
            break;
        }
        let adjacents = get_adjacents(&grid, &location.pos);
        for pos in &adjacents {
            let new_cost = costs.get(&location.pos).unwrap() + grid[pos.1 as usize][pos.0 as usize];
            if !costs.contains_key(pos) || new_cost < *costs.get(pos).unwrap() {
                heap.push(Location {
                    pos: pos.to_owned(),
                    score: new_cost, // + distance_to_target(pos, &location.pos),
                });
                closed.insert(pos.to_owned(), location.pos);
                costs.insert(pos.to_owned(), new_cost);
            }
        }
    }

    tracked_positions.reverse();
    println!(
        "{:?}",
        tracked_positions
            .iter()
            .fold(0, |sum, pos| sum + grid[pos.1 as usize][pos.0 as usize])
    );

    Ok(())
}
