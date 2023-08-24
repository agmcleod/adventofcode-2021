use std::cmp::Ordering;
use std::collections::HashSet;
use std::{fmt::Display, io::Result};

use read_input::read_text;

struct Cube {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Cube {
    fn new(x_range: (i32, i32), y_range: (i32, i32), z_range: (i32, i32)) -> Self {
        Cube {
            x: x_range,
            y: y_range,
            z: z_range,
        }
    }

    fn get_axis(&self, axis: Axis) -> (i32, i32) {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    fn set_axis_value(&mut self, axis: Axis, axis_slot: AxisSlot, value: i32) {
        let axis_values = match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        };

        match axis_slot {
            AxisSlot::First => {
                axis_values.0 = value;
            }
            AxisSlot::Second => {
                axis_values.1 = value;
            }
        }
    }
}

enum Axis {
    X,
    Y,
    Z,
}

enum AxisSlot {
    First,
    Second,
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

fn get_range(segment: Option<&str>, line: &String, axis: Axis) -> (i32, i32) {
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

            (range[0], range[1])
        } else {
            panic!("Segment does not have {}=. was: {}", axis, segment);
        }
    } else {
        panic!("Missing segment from {}", line);
    }
}

fn cubes_intersect(cube_one: &Cube, cube_two: &Cube) -> bool {
    if cube_one.x.0 <= cube_two.x.1
        && cube_one.x.1 >= cube_two.x.0
        && cube_one.y.0 <= cube_two.y.1
        && cube_one.y.1 >= cube_two.y.0
        && cube_one.z.0 <= cube_two.z.1
        && cube_one.z.1 >= cube_two.z.0
    {
        return true;
    }

    false
}

fn handle_min_range_comparison(
    cube: &mut Cube,
    cube_two: &Cube,
    cubes_to_process: &mut Vec<Cube>,
    modified_cubes: &mut Vec<Cube>,
    axis: Axis,
) {
    let values_to_compare = match axis {
        Axis::X => (cube.x.0, cube_two.x.0),
        Axis::Y => (cube.y.0, cube_two.y.0),
        Axis::Z => (cube.z.0, cube_two.z.0),
    };

    match values_to_compare.0.cmp(&values_to_compare.1) {
        // the new cube extends outside of the axis
        Ordering::Less => {
            let mut axis_values = [cube.x, cube.y, cube.z];

            match axis {
                Axis::X => {
                    axis_values[0] = (cube.x.0, cube_two.x.0 - 1);
                }
                Axis::Y => {
                    axis_values[1] = (cube.y.0, cube_two.y.0 - 1);
                }
                Axis::Z => {
                    axis_values[2] = (cube.z.0, cube_two.z.0 - 1);
                }
            }

            cubes_to_process.push(Cube::new(axis_values[0], axis_values[1], axis_values[2]));
            cube.set_axis_value(axis, AxisSlot::First, cube_two.get_axis(axis).0);
        }
        // the new cube is inside of the axis
        Ordering::Greater => {
            let mut axis_values = [cube_two.x, cube_two.y, cube_two.z];

            match axis {
                Axis::X => {
                    axis_values[0] = (cube_two.x.0, cube.x.0 - 1);
                }
                Axis::Y => {
                    axis_values[1] = (cube_two.y.0, cube.y.0 - 1);
                }
                Axis::Z => {
                    axis_values[2] = (cube_two.z.0, cube.z.0 - 1);
                }
            }

            modified_cubes.push(Cube::new(axis_values[0], axis_values[1], axis_values[2]));

            // TODO: need to remove the inside cube now
        }
        _ => {}
    }
}

fn handle_max_range_comparison(
    cube: &mut Cube,
    cube_two: &Cube,
    cubes_to_process: &mut Vec<Cube>,
    modified_cubes: &mut Vec<Cube>,
    axis: Axis,
) {
    let values_to_compare = match axis {
        Axis::X => (cube.x.1, cube_two.x.1),
        Axis::Y => (cube.y.1, cube_two.y.1),
        Axis::Z => (cube.z.1, cube_two.z.1),
    };

    match values_to_compare.0.cmp(&values_to_compare.1) {
        // the new cube extends past the far edge of the current
        Ordering::Greater => {
            let mut axis_values = [cube.x, cube.y, cube.z];

            match axis {
                Axis::X => {
                    axis_values[0] = (cube_two.x.1 + 1, cube.x.1);
                }
                Axis::Y => axis_values[1] = (cube_two.y.1 + 1, cube.y.1),
                Axis::Z => axis_values[2] = (cube_two.z.1 + 1, cube.z.1),
            }

            cubes_to_process.push(Cube::new(axis_values[0], axis_values[1], axis_values[2]));

            cube.set_axis_value(axis, AxisSlot::Second, cube_two.get_axis(axis).1);
        }
        // the new cube is inside the far edge of the axis
        Ordering::Less => {
            let mut axis_values = [cube_two.x, cube_two.y, cube_two.z];

            match axis {
                Axis::X => {
                    axis_values[0] = (cube.x.1 + 1, cube_two.x.1);
                }
                Axis::Y => {
                    axis_values[1] = (cube.y.1 + 1, cube_two.y.1);
                }
                Axis::Z => {
                    axis_values[2] = (cube.z.1 + 1, cube_two.z.1);
                }
            }

            modified_cubes.push(Cube::new((cube.x.1, cube_two.x.1), cube_two.y, cube_two.z));

            // TODO: need to remove the inside cube now
        }
        _ => {}
    }
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

        let mut modified_cubes = Vec::new();

        let mut cubes_to_process = vec![Cube::new(x_range, y_range, z_range)];

        while let Some(mut cube) = cubes_to_process.pop() {
            while let Some(cube_two) = current_on_cubes.pop() {
                if cubes_intersect(&cube, &cube_two) {
                    if is_on {
                    } else {
                        // This will have overlapping areas between axis
                        // Can optimize later if needed
                        // Not sure of the right approach at this time
                        handle_min_range_comparison(
                            &mut cube,
                            &cube_two,
                            &mut cubes_to_process,
                            &mut modified_cubes,
                            Axis::X,
                        );
                        handle_max_range_comparison(
                            &mut cube,
                            &cube_two,
                            &mut cubes_to_process,
                            &mut modified_cubes,
                            Axis::X,
                        );

                        handle_min_range_comparison(
                            &mut cube,
                            &cube_two,
                            &mut cubes_to_process,
                            &mut modified_cubes,
                            Axis::Y,
                        );
                        handle_max_range_comparison(
                            &mut cube,
                            &cube_two,
                            &mut cubes_to_process,
                            &mut modified_cubes,
                            Axis::Y,
                        );

                        handle_min_range_comparison(
                            &mut cube,
                            &cube_two,
                            &mut cubes_to_process,
                            &mut modified_cubes,
                            Axis::Z,
                        );
                        handle_max_range_comparison(
                            &mut cube,
                            &cube_two,
                            &mut cubes_to_process,
                            &mut modified_cubes,
                            Axis::Z,
                        );
                    }
                } else {
                    modified_cubes.push(cube_two);
                }
            }
        }

        intersected_cubes = modified_cubes;
    }

    Ok(())
}
