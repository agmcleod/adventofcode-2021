use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

type Coord = (i32, i32, i32);

struct Scanner {
    coords: Vec<Coord>,
    internal_distances: HashMap<(i32, i32, i32, i32, i32, i32), i32>,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            coords: Vec::new(),
            internal_distances: HashMap::new(),
        }
    }

    fn compute_internal_distances(&mut self) {
        for (i, coord1) in self.coords.iter().enumerate() {
            for coord2 in self.coords.iter().skip(i + 1) {
                let distance = get_distance(coord1, coord2);
                self.internal_distances.insert(
                    (coord1.0, coord1.1, coord1.2, coord2.0, coord2.1, coord2.2),
                    distance,
                );
            }
        }
    }
}

fn get_distance(coord1: &Coord, coord2: &Coord) -> i32 {
    (coord1.0 - coord2.0).abs() + (coord1.1 - coord2.1).abs() + (coord1.2 - coord2.2).abs()
}

fn get_common_coords_from_distance(scanner1: &Scanner, scanner2: &Scanner) -> HashSet<Coord> {
    let mut overlapped_coordinates = HashSet::new();

    for distance1 in &scanner1.internal_distances {
        for distance2 in &scanner2.internal_distances {
            if distance1.1 == distance2.1 {
                let coord1: Coord = (distance1.0 .0, distance1.0 .1, distance1.0 .2);
                overlapped_coordinates.insert(coord1);
            }
        }
    }

    overlapped_coordinates
}

fn main() -> Result<()> {
    let text = read_text("19/input.txt")?;

    let mut scanners = Vec::new();
    let mut scanner = None;

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("---") {
            if scanner.is_some() {
                scanners.push(scanner.unwrap());
            }

            scanner = Some(Scanner::new());
        } else {
            let coords = line
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>();

            if coords.len() != 3 {
                panic!("Invalid coords line {}", line);
            }

            let scanner = scanner.as_mut().unwrap();
            scanner.coords.push((coords[0], coords[1], coords[2]));
        }
    }

    scanners.push(scanner.unwrap());

    for scanner in &mut scanners {
        scanner.compute_internal_distances();
    }

    let set = get_common_coords_from_distance(&scanners[0], &scanners[1]);
    for value in &set {
        println!("{:?}", value);
    }

    Ok(())
}
