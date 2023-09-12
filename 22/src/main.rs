use std::fmt::Display;
use std::io::Result;
use std::ops::RangeInclusive;

use read_input::read_text;

#[derive(Clone, Debug)]
struct Cube {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Cube {
    fn new(
        x_range: RangeInclusive<i32>,
        y_range: RangeInclusive<i32>,
        z_range: RangeInclusive<i32>,
    ) -> Self {
        Cube {
            x: x_range,
            y: y_range,
            z: z_range,
        }
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    fn volume(&self) -> usize {
        self.x.to_owned().count() * self.y.to_owned().count() * self.z.to_owned().count()
    }

    fn x_min(&self) -> i32 {
        self.x.start().to_owned()
    }

    fn x_max(&self) -> i32 {
        self.x.end().to_owned()
    }

    fn y_min(&self) -> i32 {
        self.y.start().to_owned()
    }

    fn y_max(&self) -> i32 {
        self.y.end().to_owned()
    }

    fn z_min(&self) -> i32 {
        self.z.start().to_owned()
    }

    fn z_max(&self) -> i32 {
        self.z.end().to_owned()
    }

    fn subtract(&self, other: &Cube) -> Vec<Cube> {
        if !cubes_intersect(self, other) {
            vec![self.to_owned()]
        } else {
            [
                Cube::new(
                    self.x_min()..=other.x_min() - 1,
                    self.y.clone(),
                    self.z.clone(),
                ),
                Cube::new(
                    other.x_max() + 1..=self.x_max(),
                    self.y.clone(),
                    self.z.clone(),
                ),
                Cube::new(
                    self.x_min().max(other.x_min())..=self.x_max().min(other.x_max()),
                    self.y_min()..=other.y_min() - 1,
                    self.z.clone(),
                ),
                Cube::new(
                    self.x_min().max(other.x_min())..=self.x_max().min(other.x_max()),
                    other.y_max() + 1..=self.y_max(),
                    self.z.clone(),
                ),
                Cube::new(
                    self.x_min().max(other.x_min())..=self.x_max().min(other.x_max()),
                    self.y_min().max(other.y_min())..=self.y_max().min(other.y_max()),
                    self.z_min()..=other.z_min() - 1,
                ),
                Cube::new(
                    self.x_min().max(other.x_min())..=self.x_max().min(other.x_max()),
                    self.y_min().max(other.y_min())..=self.y_max().min(other.y_max()),
                    other.z_max() + 1..=self.z_max(),
                ),
            ]
            .into_iter()
            .filter(|c| !c.is_empty())
            .collect()
        }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

enum Axis {
    X,
    Y,
    Z,
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_value = match *self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        };
        write!(f, "{}", string_value)
    }
}

fn get_range(segment: Option<&str>, line: &String, axis: Axis) -> RangeInclusive<i32> {
    let axis_str = format!("{}=", axis);
    if let Some(segment) = segment {
        if segment.starts_with(&axis_str) {
            let segment = segment.replace(&axis_str, "");
            let range = segment
                .split("..")
                .map(|n| n.parse().expect("Could not parse number"))
                .collect::<Vec<i32>>();

            if range.len() != 2 {
                panic!("Range not the expected values {:?}", range);
            }

            range[0]..=range[1]
        } else {
            panic!("Segment does not have {}=. was: {}", axis, segment);
        }
    } else {
        panic!("Missing segment from {}", line);
    }
}

fn cubes_intersect(cube_one: &Cube, cube_two: &Cube) -> bool {
    if cube_one.x_min() <= cube_two.x_max()
        && cube_one.x_max() >= cube_two.x_min()
        && cube_one.y_min() <= cube_two.y_max()
        && cube_one.y_max() >= cube_two.y_min()
        && cube_one.z_min() <= cube_two.z_max()
        && cube_one.z_max() >= cube_two.z_min()
    {
        return true;
    }

    false
}

fn main() -> Result<()> {
    let text = read_text("22/input.txt")?;

    let mut current_on_cubes = Vec::new();

    for line in text.lines() {
        let mut line_copy = line.to_string();
        let is_on = if line_copy.starts_with("on") {
            line_copy = line_copy.replace("on ", "");
            true
        } else if line.starts_with("off") {
            line_copy = line_copy.replace("off ", "");
            false
        } else {
            panic!("Did not understand line: {}", line);
        };

        let mut iter = line_copy.split(',');

        let segment = iter.next();
        let x_range = get_range(segment, &line_copy, Axis::X);

        let segment = iter.next();
        let y_range = get_range(segment, &line_copy, Axis::Y);

        let segment = iter.next();
        let z_range = get_range(segment, &line_copy, Axis::Z);

        let mut cubes_to_process = vec![Cube::new(x_range, y_range, z_range)];

        while let Some(cube_to_switch) = cubes_to_process.pop() {
            if is_on {
                let mut modified_cubes = vec![cube_to_switch];

                for existing_cube in &current_on_cubes {
                    let mut new_modified_cubes = Vec::new();
                    for cube in &modified_cubes {
                        new_modified_cubes.extend(cube.subtract(existing_cube));
                    }
                    modified_cubes = new_modified_cubes;
                }

                current_on_cubes.extend(modified_cubes);
            } else {
                let mut modified_cubes = Vec::new();
                while let Some(existing_cube) = current_on_cubes.pop() {
                    modified_cubes.extend(existing_cube.subtract(&cube_to_switch));
                }

                current_on_cubes = modified_cubes;
            }
        }
    }

    println!(
        "{}",
        current_on_cubes
            .iter()
            // limit by p1 range
            .map(|cube| {
                Cube::new(
                    cube.x_min().max(-50)..=cube.x_max().min(50),
                    cube.y_min().max(-50)..=cube.y_max().min(50),
                    cube.z_min().max(-50)..=cube.z_max().min(50),
                )
            })
            .map(|cube| cube.volume())
            .sum::<usize>()
    );

    println!(
        "{}",
        current_on_cubes
            .iter()
            .map(|cube| cube.volume())
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_subtraction() {
        let cube_one = Cube::new(1..=10, 1..=10, 1..=10);
        let cube_two = Cube::new(2..=9, 2..=9, 2..=9);

        let cubes = cube_one.subtract(&cube_two);

        assert_eq!(cubes.len(), 6);
        assert_eq!(cubes[0], Cube::new(1..=1, 1..=10, 1..=10));
        assert_eq!(cubes[1], Cube::new(10..=10, 1..=10, 1..=10));
        assert_eq!(cubes[2], Cube::new(2..=9, 1..=1, 1..=10));
        assert_eq!(cubes[3], Cube::new(2..=9, 10..=10, 1..=10));
        assert_eq!(cubes[4], Cube::new(2..=9, 2..=9, 1..=1));
        assert_eq!(cubes[5], Cube::new(2..=9, 2..=9, 10..=10));
    }
}
