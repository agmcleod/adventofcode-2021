use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Result;

use read_input::read_text;

type Coord = (i32, i32);
type Map = HashMap<Coord, Tile>;

#[derive(Clone, Eq, Hash, PartialEq)]
enum Tile {
    Empty,
    A,
    B,
    C,
    D,
}

impl Tile {
    fn get_target_coords(&self) -> [Coord; 4] {
        match *self {
            Tile::A => [(3, 2), (3, 3), (3, 4), (3, 5)],
            Tile::B => [(5, 2), (5, 3), (5, 4), (5, 5)],
            Tile::C => [(7, 2), (7, 3), (7, 4), (7, 5)],
            Tile::D => [(9, 2), (9, 3), (9, 4), (9, 5)],
            _ => panic!("Invalid location empty"),
        }
    }

    fn get_energy_cost(&self) -> i32 {
        match *self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
            _ => panic!("Invalid type for energy"),
        }
    }
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Tile::Empty,
            'A' => Tile::A,
            'B' => Tile::B,
            'C' => Tile::C,
            'D' => Tile::D,
            _ => panic!("unrecognized character {}", ch),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::A => "A",
                Tile::B => "B",
                Tile::C => "C",
                Tile::D => "D",
                Tile::Empty => ".",
            }
        )
    }
}

#[derive(Clone)]
struct State {
    // mostly used for debugging
    old_state: Option<Box<State>>,
    energy: usize,
    map: Map,
    width: usize,
    height: usize,
    locations_solved: HashSet<Tile>,
}

impl State {
    fn new(map: Map, width: usize, height: usize) -> Self {
        Self {
            old_state: None,
            energy: 0,
            map,
            width,
            height,
            locations_solved: HashSet::new(),
        }
    }

    fn create_next(&self) -> Self {
        let mut next = self.clone();
        next.old_state = Some(Box::new(self.clone()));

        next
    }

    fn get_nonsolved_tiles(&self) -> impl Iterator<Item = (&Coord, &Tile)> {
        self.map
            .iter()
            .filter(|(_, tile)| **tile != Tile::Empty && !self.locations_solved.contains(*tile))
    }

    fn get_possible_hallway_tiles(&self, from: &Coord) -> Vec<Coord> {
        let possible_hallway_tiles = [(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];

        possible_hallway_tiles
            .iter()
            .filter(|coord| {
                *self.map.get(coord).unwrap() == Tile::Empty && self.path_is_clear(from, coord)
            })
            .cloned()
            .collect()
    }

    fn path_is_clear(&self, from: &Coord, to: &Coord) -> bool {
        // since we're dealing with a fairly strict map, we can check y axis more deliberately
        if from.1 != to.1 {
            let mut y_sorted = [from, to];
            y_sorted.sort_by(|a, b| a.1.cmp(&b.1));

            for y in y_sorted[0].1..=y_sorted[1].1 {
                // we use second coord's x axis, because that's the Y vertical we're checking
                let coord = (y_sorted[1].0, y);
                // if not the from spot, and a tile along the y axis is filled, return false
                if coord != *from && *self.map.get(&coord).unwrap() != Tile::Empty {
                    return false;
                }
            }
        }

        // now just check x axis

        let min_x = from.0.min(to.0);
        let max_x = from.0.max(to.0);

        for x in min_x..=max_x {
            let coord = (x, from.1.min(to.1));
            if coord != *from && *self.map.get(&coord).unwrap() != Tile::Empty {
                return false;
            }
        }

        true
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Energy: {}", self.energy)?;
        for r in 0..=self.height {
            for c in 0..=self.width {
                if let Some(location) = self.map.get(&(c as i32, r as i32)) {
                    write!(f, "{}", location)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn manhatten_distance(coord1: &Coord, coord2: &Coord) -> i32 {
    (coord1.0 - coord2.0).unsigned_abs() as i32 + (coord1.1 - coord2.1).unsigned_abs() as i32
}

fn move_letter_out_of_way(
    state: &State,
    coord: &Coord,
    tile: &Tile,
    current_best_score: &mut usize,
) {
    for to_coord in &state.get_possible_hallway_tiles(coord) {
        let mut state = state.create_next();
        state.map.insert(*coord, Tile::Empty);
        state.map.insert(*to_coord, tile.clone());
        state.energy += (tile.get_energy_cost() * manhatten_distance(coord, to_coord)) as usize;
        next_moves(state, current_best_score);
    }
}

fn next_moves(state: State, current_best_score: &mut usize) {
    if state.energy > *current_best_score {
        return;
    }
    if state.locations_solved.len() == 4 {
        // let mut old_state = state.old_state.as_ref();
        // println!("printing solved");
        // println!("{}", state);
        // while old_state.is_some() {
        //     println!("{}", old_state.unwrap());
        //     old_state = old_state.as_ref().unwrap().old_state.as_ref();
        // }
        *current_best_score = (*current_best_score).min(state.energy);
        return;
    }

    for (coord, tile) in state.get_nonsolved_tiles() {
        let target_coords = tile.get_target_coords();
        // if letters are solved, mark this one as complete
        if target_coords
            .iter()
            .filter(|c| state.map.get(c).unwrap() == tile)
            .count()
            == 4
        {
            let mut state = state.create_next();
            state.locations_solved.insert(tile.clone());
            next_moves(state, current_best_score);
            continue;
        }

        if (*coord == target_coords[0]
            && (state.map.get(&target_coords[1]).unwrap() != tile
                || state.map.get(&target_coords[2]).unwrap() != tile
                || state.map.get(&target_coords[3]).unwrap() != tile))
            || (*coord == target_coords[1]
                && *state.map.get(&target_coords[0]).unwrap() == Tile::Empty
                && (state.map.get(&target_coords[2]).unwrap() != tile
                    || state.map.get(&target_coords[3]).unwrap() != tile))
            || (*coord == target_coords[2]
                && *state.map.get(&target_coords[0]).unwrap() == Tile::Empty
                && *state.map.get(&target_coords[1]).unwrap() == Tile::Empty
                && state.map.get(&target_coords[3]).unwrap() != tile)
            || (coord.1 == target_coords[3].1
                && coord.0 != target_coords[3].0
                && *state.map.get(&target_coords[0]).unwrap() == Tile::Empty
                && *state.map.get(&target_coords[1]).unwrap() == Tile::Empty
                && *state.map.get(&target_coords[2]).unwrap() == Tile::Empty)
            || (coord.1 >= 2 && coord.0 != target_coords[0].0)
        {
            move_letter_out_of_way(&state, coord, tile, current_best_score);
        } else if coord.1 == 1 {
            // y coord is 1, so is out of slot
            // definitely an ugly solution here, my p1 answer did not scale :)
            let first_tile = state.map.get(&target_coords[0]).unwrap();
            let second_tile = state.map.get(&target_coords[1]).unwrap();
            let third_tile = state.map.get(&target_coords[2]).unwrap();
            let fourth_tile = state.map.get(&target_coords[3]).unwrap();

            let can_move_into_fourth_spot = *first_tile == Tile::Empty
                && *second_tile == Tile::Empty
                && *third_tile == Tile::Empty
                && *fourth_tile == Tile::Empty;
            let can_move_into_third_spot = *first_tile == Tile::Empty
                && *second_tile == Tile::Empty
                && *third_tile == Tile::Empty
                && *fourth_tile == *tile;
            let can_move_into_second_spot = *first_tile == Tile::Empty
                && *second_tile == Tile::Empty
                && *third_tile == *tile
                && *fourth_tile == *tile;
            let can_move_into_top_spot = *first_tile == Tile::Empty
                && *second_tile == *tile
                && *third_tile == *tile
                && *fourth_tile == *tile;

            // because of the first boolean check, we can just use the first target coord as the endpoint safely
            if (can_move_into_second_spot
                || can_move_into_top_spot
                || can_move_into_fourth_spot
                || can_move_into_third_spot)
                && state.path_is_clear(coord, &target_coords[0])
            {
                let resulting_coord = if can_move_into_top_spot {
                    target_coords[0]
                } else if can_move_into_second_spot {
                    target_coords[1]
                } else if can_move_into_third_spot {
                    target_coords[2]
                } else {
                    target_coords[3]
                };

                let mut state = state.create_next();
                state.map.insert(*coord, Tile::Empty);
                state.energy +=
                    (tile.get_energy_cost() * manhatten_distance(coord, &resulting_coord)) as usize;
                state.map.insert(resulting_coord, tile.clone());
                next_moves(state, current_best_score);
            }
        }
    }
}

fn main() -> Result<()> {
    let text = read_text("23/input.txt")?;

    let mut map: Map = HashMap::new();

    let mut width = 0;
    let mut height = 0;

    for (r, line) in text.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == ' ' || ch == '#' {
                continue;
            }

            map.insert((c as i32, r as i32), ch.into());
            width = width.max(c);
        }
        height += 1;
    }

    let state = State::new(map, width, height);
    let mut current_best_score = usize::MAX;
    next_moves(state, &mut current_best_score);
    println!("{}", current_best_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_manhatten_distance() {
        assert_eq!(manhatten_distance(&(1, 1), &(1, 2)), 1);
        assert_eq!(manhatten_distance(&(1, 1), &(2, 2)), 2);
        assert_eq!(manhatten_distance(&(1, 1), &(6, 2)), 6);
        assert_eq!(manhatten_distance(&(6, 2), &(1, 1)), 6);
        assert_eq!(manhatten_distance(&(10, 1), &(9, 2)), 2);
    }
}
