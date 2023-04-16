use std::collections::{HashMap, HashSet};
use std::io::Result;

use itertools::Itertools;
use nalgebra::base::{Matrix3, Vector3};
use read_input::read_text;

struct Distance {
    left: Vector3<i32>,
    right: Vector3<i32>,
    distance: i32,
}

impl Distance {
    fn new(left: Vector3<i32>, right: Vector3<i32>, distance: i32) -> Distance {
        Distance {
            left,
            right,
            distance,
        }
    }
}

struct Scanner {
    id: u32,
    beacons: Vec<Vector3<i32>>,
    internal_distances: Vec<Distance>,
    position: Option<Vector3<i32>>,
    orientation: Option<Matrix3<i32>>,
}

impl Scanner {
    fn new(id: u32) -> Self {
        Scanner {
            id: id,
            beacons: Vec::new(),
            internal_distances: Vec::new(),
            position: None,
            orientation: None,
        }
    }

    fn compute_internal_distances(&mut self) {
        for (i, coord1) in self.beacons.iter().enumerate() {
            for coord2 in self.beacons.iter().skip(i + 1) {
                let distance = get_distance(coord1, coord2);
                self.internal_distances.push(Distance::new(
                    coord1.to_owned(),
                    coord2.to_owned(),
                    distance,
                ));
            }
        }

        self.internal_distances
            .sort_by(|a, b| a.distance.cmp(&b.distance));
    }
}

fn get_distance(coord1: &Vector3<i32>, coord2: &Vector3<i32>) -> i32 {
    (coord1.x - coord2.x).abs() + (coord1.y - coord2.y).abs() + (coord1.z - coord2.z).abs()
}

fn align_scanners(scanners: Vec<Scanner>) {
    let mut unaligned = HashMap::new();
    let mut visited = HashMap::new();

    for s in scanners {
        unaligned.insert(s.id, s);
    }

    // remove first scanner to make it the reference point
    let mut scanner_0 = unaligned.remove(&1).unwrap();
    scanner_0.position = Some(Vector3::from_element(0));
    scanner_0.orientation = Some(Matrix3::identity());

    let mut queue = Vec::new();
    queue.push(scanner_0);

    while let Some(scanner) = queue.pop() {
        let ids = potential_neighbouring_scanners(&scanner, unaligned.values().collect());

        for id in ids {}
    }
}

fn equal_distance_count(scanner_1: &Scanner, scanner_2: &Scanner) -> i32 {
    let d1 = &scanner_1.internal_distances;
    let d2 = &scanner_2.internal_distances;
    let mut count = 0;
    let mut i1 = 0;
    let mut i2 = 0;
    loop {
        if i1 >= d1.len() || i2 >= d2.len() {
            break;
        }

        // found an equal distance
        if d1[i1].distance == d2[i2].distance {
            count += 1;
            i1 += 1;
            i2 += 1;
        } else if d1[i1].distance > d2[i2].distance {
            i2 += 1;
        } else {
            i1 += 1;
        }
    }

    count
}

// Uses the distances between beacons to find potential other scanners, which overlap
// their regions with the scanner
fn potential_neighbouring_scanners(scanner: &Scanner, unaligned: Vec<&Scanner>) -> Vec<u32> {
    let mut minimal_eq_distance = (12 * 11) / 2;

    unaligned
        .iter()
        .filter(|scanner2| equal_distance_count(scanner, scanner2) >= minimal_eq_distance)
        .map(|scanner| scanner.id)
        .collect()
}

fn main() -> Result<()> {
    let text = read_text("19/input.txt")?;

    let mut scanners = Vec::new();
    let mut scanner = None;

    let mut id = 1;

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("---") {
            if scanner.is_some() {
                scanners.push(scanner.unwrap());
            }

            scanner = Some(Scanner::new(id));
            id += 1;
        } else {
            let coords = line
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>();

            if coords.len() != 3 {
                panic!("Invalid coords line {}", line);
            }

            let scanner = scanner.as_mut().unwrap();
            scanner
                .beacons
                .push(Vector3::new(coords[0], coords[1], coords[2]));
        }
    }

    scanners.push(scanner.unwrap());

    for scanner in &mut scanners {
        scanner.compute_internal_distances();
    }

    Ok(())
}
