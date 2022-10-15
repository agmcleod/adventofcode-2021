use std::collections::HashSet;
use std::io::Result;

use geo::{
    line_intersection::{line_intersection, LineIntersection},
    CoordsIter, Line,
};
use read_input::read_text;

fn get_number(value: Option<&str>) -> f32 {
    if let Some(value) = value {
        match value.parse::<f32>() {
            Ok(n) => n,
            Err(_err) => panic!("Could not parse number {:?}", value),
        }
    } else {
        panic!("Value is None when expected some");
    }
}

fn get_coords_from_point_str(point: Option<&str>) -> (f32, f32) {
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

fn get_line_segments(text: &String, include_diagonal: bool) -> Vec<Line<f32>> {
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

fn get_intersection_points(segments: &Vec<Line<f32>>) -> HashSet<(i32, i32)> {
    let mut intersection_points = HashSet::new();
    for (i, line) in segments.iter().enumerate() {
        for line_2 in segments.iter().skip(i + 1) {
            let overlap = line_intersection(*line, *line_2);
            if let Some(overlap) = overlap {
                match overlap {
                    LineIntersection::SinglePoint {
                        intersection,
                        is_proper,
                    } => {
                        intersection_points.insert((intersection.x as i32, intersection.y as i32));
                    }
                    LineIntersection::Collinear { intersection } => {
                        let delta = intersection.delta();
                        let sx = intersection.start.x as i32;
                        let sy = intersection.start.y as i32;
                        let mut x_incr = 0;
                        let mut y_incr = 0;
                        let delta_x = delta.x.abs() as i32;
                        let delta_y = delta.y.abs() as i32;
                        loop {
                            let insert_x = get_insert_coord_value(delta.x as i32, sx, x_incr);
                            let insert_y = get_insert_coord_value(delta.y as i32, sy, y_incr);

                            intersection_points.insert((insert_x, insert_y));

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
                }
            }
        }
    }

    intersection_points
}

fn main() -> Result<()> {
    let text = read_text("5/input.txt")?;

    let segments = get_line_segments(&text, false);
    let intersection_points = get_intersection_points(&segments);

    println!("{}", intersection_points.len());

    let segments = get_line_segments(&text, true);
    let intersection_points = get_intersection_points(&segments);

    println!("{}", intersection_points.len());

    Ok(())
}

#[cfg(test)]
mod test {
    use std::iter::Inspect;

    use geo::Line;

    use crate::get_intersection_points;

    #[test]
    fn test_line_intersections_return_correct_points() {
        let segments = vec![
            Line {
                start: (1.0, 1.0).into(),
                end: (5.0, 5.0).into(),
            },
            Line {
                start: (2.0, 2.0).into(),
                end: (8.0, 8.0).into(),
            },
        ];

        let intersection_points = get_intersection_points(&segments);
        assert_eq!(intersection_points.len(), 4);
        assert!(intersection_points.contains(&(2, 2)));
        assert!(intersection_points.contains(&(3, 3)));
        assert!(intersection_points.contains(&(4, 4)));
        assert!(intersection_points.contains(&(5, 5)));

        let segments = vec![
            Line {
                start: (1.0, 1.0).into(),
                end: (5.0, 5.0).into(),
            },
            Line {
                start: (8.0, 2.0).into(),
                end: (3.0, 7.0).into(),
            },
        ];

        let intersection_points = get_intersection_points(&segments);
        assert_eq!(intersection_points.len(), 1);
        assert!(intersection_points.contains(&(5, 5)));
    }

    #[test]
    fn test_line_intersections_return_correct_points_when_reverse_delta() {
        let segments = vec![
            Line {
                start: (5.0, 5.0).into(),
                end: (1.0, 1.0).into(),
            },
            Line {
                start: (8.0, 8.0).into(),
                end: (2.0, 2.0).into(),
            },
        ];

        let intersection_points = get_intersection_points(&segments);
        assert_eq!(intersection_points.len(), 4);
        assert!(intersection_points.contains(&(2, 2)));
        assert!(intersection_points.contains(&(3, 3)));
        assert!(intersection_points.contains(&(4, 4)));
        assert!(intersection_points.contains(&(5, 5)));

        let segments = vec![
            Line {
                start: (5.0, 5.0).into(),
                end: (1.0, 1.0).into(),
            },
            Line {
                start: (3.0, 7.0).into(),
                end: (8.0, 2.0).into(),
            },
        ];

        let intersection_points = get_intersection_points(&segments);
        assert_eq!(intersection_points.len(), 1);
        assert!(intersection_points.contains(&(5, 5)));
    }

    #[test]
    fn test_diagonal_intersects_with_horiontal() {
        let segments = vec![
            Line {
                start: (5.0, 5.0).into(),
                end: (1.0, 1.0).into(),
            },
            Line {
                start: (1.0, 3.0).into(),
                end: (7.0, 3.0).into(),
            },
        ];

        let intersection_points = get_intersection_points(&segments);
        assert_eq!(intersection_points.len(), 1);
        assert!(intersection_points.contains(&(3, 3)));
    }
}
