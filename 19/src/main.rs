use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::f32;
use std::io::Result;

use itertools::Itertools;
use nalgebra::base::{Matrix3, Vector3};
use read_input::read_text;

// The threshold for number of overlapping probes was 12, this constitutes to n*(n-1)/2 egdes.
const ALIGNMENT_THRESHOLD: u32 = 12;
const EDGE_THRESHOLD: u32 = ALIGNMENT_THRESHOLD * (ALIGNMENT_THRESHOLD - 1) / 2;

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

fn align_scanner(s1: &mut Scanner, s2: &Scanner) -> bool {
    assert!(
        s2.position.is_some() && s2.orientation.is_some(),
        "'s2' must have a known orientation and position"
    );

    let mut s2_diffs: Vec<Vector3<i32>> = position_differences(&s2.beacons);
    s2_diffs.sort_by(|v1, v2| compare_vector(&v1, v2));

    // Step 1. Find correct configuration
    let mut rotational_alignment = false;
    let mut orientation = None;

    for rotation in possible_orientations() {
        // Apply rotation to beacon locations
        let mut beacons: Vec<Vector3<i32>> = s1
            .beacons
            .clone()
            .into_iter()
            .map(|v| rotation * v)
            .collect();
        beacons.sort_by(|v1, v2| compare_vector(&v1, v2));
        // Compute new differences between beacons
        let mut diffs = position_differences(&beacons);
        diffs.sort_by(|v1, v2| compare_vector(&v1, v2));

        let eq_diffs = equal_vector_count(&diffs, &s2_diffs);

        if eq_diffs >= EDGE_THRESHOLD {
            rotational_alignment = true;
            orientation = Some(s2.orientation.unwrap() * rotation);
            break;
        }
    }
    if !rotational_alignment {
        return false;
    }

    // Step 2. Find offset, which causes probes to overlap
    let mut positional_alignment = false;
    let mut position: Option<Vector3<i32>> = None;

    let mut s2_beacons: Vec<Vector3<i32>> = s2
        .beacons
        .clone()
        .into_iter()
        .map(|v| s2.orientation.unwrap() * v)
        .collect();
    s2_beacons.sort_by(|v1, v2| compare_vector(&v1, v2));

    let mut s1_beacons: Vec<Vector3<i32>> = s1
        .beacons
        .clone()
        .into_iter()
        .map(|v| orientation.unwrap() * v)
        .collect();
    s1_beacons.sort_by(|v1, v2| compare_vector(&v1, v2));

    let mut stack = s2_beacons.clone();
    'outer: while let Some(s2_beacon) = stack.pop() {
        // Use s1_beacon as reference point
        for s1_beacon in s1_beacons.iter() {
            // align s2_beacon with s1_beacon and check whether alignment is correct
            let offset = s2_beacon - s1_beacon; // offset + x1 = x2 (if correct)
            let mut aligned_beacons: Vec<Vector3<i32>> =
                s1_beacons.clone().into_iter().map(|v| offset + v).collect();
            aligned_beacons.sort_by(|x, y| compare_vector(&x, &y));

            let eq = equal_vector_count(&aligned_beacons, &s2_beacons);
            if eq >= ALIGNMENT_THRESHOLD {
                positional_alignment = true;
                position = Some(s2.position.unwrap() + offset);
                break 'outer;
            }
        }
    }
    if positional_alignment {
        s1.position = position;
        s1.orientation = orientation;
    }
    return positional_alignment;
}

fn align_scanners(scanners: Vec<Scanner>) -> HashMap<u32, Scanner> {
    let mut unaligned = HashMap::new();
    let mut visited = HashMap::new();

    for s in scanners.into_iter() {
        unaligned.insert(s.id, s);
    }

    // Remove first scanner and make it the base reference frame
    let mut s0 = unaligned.remove(&1).unwrap();
    s0.position = Some(Vector3::from_element(0));
    s0.orientation = Some(Matrix3::identity());

    let mut queue = Vec::new();
    queue.push(s0);

    // Align scanners through graph traversal
    while let Some(scanner_info) = queue.pop() {
        // Visit node and try to align neighbouring nodes
        let ids = potential_neighbouring_scanners(&scanner_info, unaligned.values().collect());
        // Remove potential candidates
        for id in ids {
            let mut s = unaligned.remove(&id).unwrap(); // temporariliy take ownership of scanner
            let success = align_scanner(&mut s, &scanner_info);

            // Check whether scanner was aligned successfully
            if success {
                queue.push(s); // scanner can be visited next
            } else {
                unaligned.insert(id, s);
            }
        }
        visited.insert(scanner_info.id, scanner_info);
    }
    assert_eq!(
        unaligned.len(),
        0,
        "There are still unaligned scanners left over"
    );
    visited
}

fn equal_distance_count(scanner_1: &Scanner, scanner_2: &Scanner) -> i32 {
    let d1 = &scanner_1.internal_distances;
    let d2 = &scanner_2.internal_distances;
    let mut count = 0;
    let (mut i1, mut i2) = (0, 0);
    loop {
        if i1 >= d1.len() || i2 >= d2.len() {
            break;
        } // loop guard
        if d1[i1].distance == d2[i2].distance {
            // found an equal distance
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

fn equal_vector_count(s1: &Vec<Vector3<i32>>, s2: &Vec<Vector3<i32>>) -> u32 {
    let d1 = s1; // inner_distances are sorted
    let d2 = s2;
    let mut count = 0;
    let (mut i1, mut i2) = (0, 0);
    loop {
        if i1 >= d1.len() || i2 >= d2.len() {
            break;
        } // loop guard
        if compare_vector(&d1[i1], &d2[i2]) == Ordering::Equal {
            // found an equal distance
            count += 1;
            i1 += 1;
            i2 += 1;
        } else if compare_vector(&d1[i1], &d2[i2]) == Ordering::Greater {
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
    let minimal_eq_distance = (12 * 11) / 2;

    unaligned
        .iter()
        .filter(|scanner2| equal_distance_count(scanner, scanner2) > minimal_eq_distance)
        .map(|scanner| scanner.id)
        .collect()
}

fn position_differences(beacons: &Vec<Vector3<i32>>) -> Vec<Vector3<i32>> {
    let mut differences = Vec::new();
    let mut stack: Vec<&Vector3<i32>> = beacons.iter().collect();

    while stack.len() > 1 {
        let beacon1 = stack.pop().unwrap();
        for beacon2 in &stack {
            differences.push(beacon1 - *beacon2);
        }
    }

    differences.sort_by(|v1, v2| compare_vector(&v1, &v2));
    differences
}

fn compare_vector(v1: &Vector3<i32>, v2: &Vector3<i32>) -> Ordering {
    if v1[0] == v2[0] {
        if v1[1] == v2[1] {
            if v1[2] == v2[2] {
                Ordering::Equal
            } else if v1[2] > v2[2] {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if v1[1] > v2[1] {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else if v1[0] > v2[0] {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

/// Returns all possible orientations that the scanner could be in the form of rotation matrices
fn possible_orientations() -> Vec<Matrix3<i32>> {
    let matrices: Vec<Matrix3<i32>> = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]
        .into_iter()
        .permutations(3)
        .map(|e| Matrix3::from_iterator(e.concat().into_iter()))
        .map(|m| {
            let mut m2 = m.clone();
            multiply_row(&mut m2, 0, -1);
            vec![m, m2]
        })
        .flatten()
        .map(|m| {
            let mut m2 = m.clone();
            multiply_row(&mut m2, 1, -1);
            vec![m, m2]
        })
        .flatten()
        .map(|m| {
            let mut m2 = m.clone();
            multiply_row(&mut m2, 2, -1);
            vec![m, m2]
        })
        .flatten()
        .filter(|m| det(&m) == 1)
        .collect();
    matrices
}

fn multiply_row(matrix: &mut Matrix3<i32>, index: usize, scalar: i32) {
    for i in 0..3 {
        // column major matrix
        matrix[i * 3 + index] = matrix[i * 3 + index] * scalar;
    }
}

/// Determinant of 3x3 Matrix
fn det(m: &Matrix3<i32>) -> i32 {
    let mut d = m[0] * (m[3 * 1 + 1] * m[3 * 2 + 2] - m[3 * 2 + 1] * m[3 * 1 + 2]);
    d = d - m[3] * (m[1] * m[8] - m[7] * m[2]);
    d = d + m[6] * (m[1] * m[5] - m[4] * m[2]);
    d
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

    let aligned = align_scanners(scanners);

    let mut beacons = Vec::new();
    for scanner in aligned.values() {
        let mut bs: Vec<Vector3<i32>> = scanner
            .beacons
            .iter()
            .map(|v| scanner.orientation.unwrap() * v + scanner.position.unwrap())
            .collect();
        beacons.append(&mut bs);
    }
    beacons.sort_by(|x, y| compare_vector(&x, &y));
    beacons.dedup_by(|x, y| compare_vector(&x, &y) == Ordering::Equal);
    println!("{}", beacons.len().to_string());

    Ok(())
}
