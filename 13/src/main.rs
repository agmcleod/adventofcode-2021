use std::collections::HashMap;
use std::io::Result;

use read_input::read_text;

fn add_coord_to_map(map: &mut HashMap<usize, HashMap<usize, usize>>, key: usize, value: usize) {
    if map.contains_key(&key) {
        map.get_mut(&key).unwrap().insert(value, 1);
    } else {
        let mut set = HashMap::new();
        set.insert(value, 1);
        map.insert(key, set);
    }
}

fn add_value_to_coord(map: &mut HashMap<usize, usize>, key: usize) {
    if map.contains_key(&key) {
        *map.get_mut(&key).unwrap() += 1;
    } else {
        map.insert(key, 1);
    }
}

fn flip_on_axis(
    coords: &mut HashMap<usize, HashMap<usize, usize>>,
    opposite_axis_coords: &mut HashMap<usize, HashMap<usize, usize>>,
    flip_number: usize,
) {
    let mut transformations = Vec::new();
    for (coord, _sub_coords) in coords.iter() {
        if *coord > flip_number {
            let new_coord = flip_number - (*coord - flip_number);
            transformations.push((*coord, new_coord));
        }
    }

    for transformation in &transformations {
        let sub_coords = coords.remove(&transformation.0).unwrap();
        // scan the sub_coords, to update the opposite axis' coords
        for (sub_coord, _) in &sub_coords {
            if let Some(sub_of_opposite) = opposite_axis_coords.get_mut(sub_coord) {
                if sub_of_opposite.contains_key(&transformation.0) {
                    let count_for_opposite = sub_of_opposite.get_mut(&transformation.0).unwrap();
                    *count_for_opposite -= 1;
                    if *count_for_opposite == 0 {
                        sub_of_opposite.remove(&transformation.0).unwrap();
                    }
                    add_value_to_coord(sub_of_opposite, transformation.1);
                } else {
                    panic!(
                        "Could not remove: {} from {:?}",
                        transformation.0, sub_of_opposite
                    );
                }
            } else {
                panic!("Other did not contain digit: {}", sub_coord);
            }
        }
        coords.insert(transformation.1, sub_coords);
    }
}

fn count_coords(coords: &HashMap<usize, HashMap<usize, usize>>) -> usize {
    coords.iter().fold(0, |sum, (_, sub_coords)| {
        sum + sub_coords.values().sum::<usize>()
    })
}

fn main() -> Result<()> {
    let text = read_text("13/input.txt")?;

    let mut x_coords: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let mut y_coords: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    for line in text.lines() {
        if line.contains("fold along") {
            let fold_text = line.replace("fold along ", "");
            let mut iter = fold_text.split("=");
            let axis = iter.next().unwrap();
            let number: usize = iter.next().unwrap().parse().unwrap();

            match axis {
                "x" => {
                    flip_on_axis(&mut x_coords, &mut y_coords, number);
                }
                "y" => {
                    flip_on_axis(&mut y_coords, &mut x_coords, number);
                }
                _ => panic!("Invalid value for axis: {} in line: {}", axis, line),
            }
            let x_coords_count = count_coords(&x_coords);
            let y_coords_count = count_coords(&y_coords);

            if x_coords_count != y_coords_count {
                panic!(
                    "Coord sets should be equal\n{} == {}!\n\n{:?}\n\n{:?}",
                    x_coords_count, y_coords_count, x_coords, y_coords
                );
            }
            println!("{}", x_coords_count);
            break;
        } else {
            let mut iter = line.split(",");
            if line.len() == 0 {
                continue;
            }
            let x: usize = iter.next().unwrap().parse().unwrap();
            let y: usize = iter.next().unwrap().parse().unwrap();

            add_coord_to_map(&mut x_coords, x, y);
            add_coord_to_map(&mut y_coords, y, x);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flip_axis() {
        let mut x_coords = HashMap::new();
        // 0,0  0,2
        x_coords.insert(0, HashMap::from([(0, 1), (2, 1)]));
        // 2,0  2,1, 2,4
        x_coords.insert(2, HashMap::from([(0, 1), (1, 1), (4, 1)]));
        // 3,0  3,3
        x_coords.insert(3, HashMap::from([(0, 1), (3, 1)]));
        // 4,4
        x_coords.insert(4, HashMap::from([(4, 1)]));

        let mut y_coords = HashMap::new();
        // 0,0  2,0, 3,0
        y_coords.insert(0, HashMap::from([(0, 1), (2, 1), (3, 1)]));
        // 2,1
        y_coords.insert(1, HashMap::from([(2, 1)]));
        // 0,2
        y_coords.insert(2, HashMap::from([(0, 1)]));
        // 3,3
        y_coords.insert(3, HashMap::from([(3, 1)]));
        // 4,4  2,4
        y_coords.insert(4, HashMap::from([(4, 1), (2, 1)]));

        flip_on_axis(&mut y_coords, &mut x_coords, 2);

        let mut x_result = HashMap::new();
        // 0,0  0,2
        x_result.insert(0, HashMap::from([(0, 1), (2, 1)]));
        // 2,0  2,1
        x_result.insert(2, HashMap::from([(0, 2), (1, 1)]));
        // 3,0  3,1
        x_result.insert(3, HashMap::from([(0, 1), (1, 1)]));
        // 4,0
        x_result.insert(4, HashMap::from([(0, 1)]));
        assert_eq!(x_coords, x_result);

        let mut y_result = HashMap::new();
        // 0,0, 2,0  3,0  4,0
        y_result.insert(0, HashMap::from([(0, 1), (2, 2), (3, 1), (4, 1)]));
        // 2,1  3,1
        y_result.insert(1, HashMap::from([(2, 1), (3, 1)]));
        // 2,0
        y_result.insert(2, HashMap::from([(0, 1)]));
        assert_eq!(y_coords, y_result);

        assert_eq!(x_coords.len(), y_coords.len());
    }
}
