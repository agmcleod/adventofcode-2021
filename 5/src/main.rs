use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn delta(&self) -> (i32, i32) {
        (self.end.0 - self.start.0, self.end.1 - self.start.1)
    }
}

fn get_number(value: Option<&str>) -> i32 {
    if let Some(value) = value {
        match value.parse::<i32>() {
            Ok(n) => n,
            Err(_err) => panic!("Could not parse number {:?}", value),
        }
    } else {
        panic!("Value is None when expected some");
    }
}

fn get_coords_from_point_str(point: Option<&str>) -> (i32, i32) {
    if let Some(point) = point {
        let mut axis = point.split(",");
        let x = get_number(axis.next());
        let y = get_number(axis.next());

        (x, y)
    } else {
        panic!("Point is missing section");
    }
}

fn get_insert_coord_value(delta: i32, start: i32, incr: i32) -> i32 {
    if delta < 0 {
        start - incr
    } else {
        start + incr
    }
}

fn get_line_segments(text: &String, include_diagonal: bool) -> Vec<Line> {
    let mut segments = Vec::new();

    for line in text.lines() {
        let mut points = line.split(" -> ");
        let coord_one = get_coords_from_point_str(points.next());
        let coord_two = get_coords_from_point_str(points.next());

        if include_diagonal || coord_one.0 == coord_two.0 || coord_one.1 == coord_two.1 {
            segments.push(Line {
                start: coord_one.into(),
                end: coord_two.into(),
            });
        }
    }

    segments
}

fn get_grid_of_line_coords(segments: &Vec<Line>) -> HashMap<(i32, i32), i32> {
    let mut intersection_points = HashMap::new();
    for line in segments {
        let delta = line.delta();
        let sx = line.start.0;
        let sy = line.start.1;
        let mut x_incr = 0;
        let mut y_incr = 0;
        let delta_x = delta.0.abs();
        let delta_y = delta.1.abs();
        loop {
            let insert_x = get_insert_coord_value(delta.0, sx, x_incr);
            let insert_y = get_insert_coord_value(delta.1, sy, y_incr);

            if intersection_points.contains_key(&(insert_x, insert_y)) {
                let count = intersection_points.get_mut(&(insert_x, insert_y)).unwrap();
                *count += 1;
            } else {
                intersection_points.insert((insert_x, insert_y), 1);
            }

            let mut did_increment = false;
            if delta_x > 0 && x_incr < delta_x {
                x_incr += 1;
                did_increment = true;
            }
            if delta_y > 0 && y_incr < delta_y {
                y_incr += 1;
                did_increment = true;
            }
            if !did_increment {
                break;
            }
        }
    }

    intersection_points
}

fn main() -> Result<()> {
    let text = read_text("5/input.txt")?;

    let segments = get_line_segments(&text, false);
    let intersection_points = get_grid_of_line_coords(&segments);

    println!(
        "{}",
        intersection_points
            .iter()
            .filter(|&(_coord, count)| { *count > 1 })
            .count()
    );

    let segments = get_line_segments(&text, true);
    let intersection_points = get_grid_of_line_coords(&segments);

    println!(
        "{}",
        intersection_points
            .iter()
            .filter(|&(_coord, count)| { *count > 1 })
            .count()
    );

    Ok(())
}
