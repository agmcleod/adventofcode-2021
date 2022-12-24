use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::Result;

use read_input::read_text;

type Grid = Vec<Vec<i32>>;
type Position = (i32, i32);

#[derive(Eq)]
struct Step {
    pos: Position,
    score: i32,
    path: Vec<Position>,
    crossed_points: HashSet<Position>,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Step) -> Ordering {
        self.score.cmp(&other.score)
    }
}

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

    let mut work: BinaryHeap<Step> = BinaryHeap::new();
    let mut crossed_points = HashSet::new();
    crossed_points.insert((0, 0));
    work.push(Step {
        pos: (0, 0),
        score: 0,
        path: Vec::new(),
        crossed_points,
    });

    while let Some(next_step) = work.pop() {
        if next_step.pos == (9, 9) {
            println!(
                "{}",
                next_step
                    .path
                    .iter()
                    .fold(0, |sum, pos| { sum + grid[pos.1 as usize][pos.0 as usize] })
            );
            break;
        }
        let adjacents = get_adjacents(&grid, &next_step.pos);
        for pos in &adjacents {
            if next_step.crossed_points.contains(pos) {
                continue;
            }
            let mut path = next_step.path.clone();
            path.push(pos.to_owned());
            let mut crossed_points = next_step.crossed_points.clone();
            crossed_points.insert(pos.to_owned());
            work.push(Step {
                pos: pos.to_owned(),
                score: grid[pos.1 as usize][pos.0 as usize],
                path,
                crossed_points,
            });
        }
    }

    Ok(())
}
