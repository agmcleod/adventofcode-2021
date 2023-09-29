use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Result;

use read_input::read_text;

type Coord = (usize, usize);
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
    fn get_target_coords(&self) -> [Coord; 2] {
        match *self {
            Tile::A => [(3, 2), (3, 3)],
            Tile::B => [(5, 2), (5, 3)],
            Tile::C => [(7, 2), (7, 3)],
            Tile::D => [(9, 2), (9, 3)],
            _ => panic!("Invalid location empty"),
        }
    }

    fn get_energy_cost(&self) -> usize {
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
    energy: usize,
    map: Map,
    width: usize,
    height: usize,
    locations_solved: HashSet<Tile>,
}

impl State {
    fn new(map: Map, width: usize, height: usize) -> Self {
        Self {
            energy: 0,
            map,
            width,
            height,
            locations_solved: HashSet::new(),
        }
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
                if let Some(location) = self.map.get(&(c, r)) {
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

fn manhatten_distance(coord1: &Coord, coord2: &Coord) -> usize {
    ((coord1.0 + coord1.1) as i32 - (coord2.0 + coord2.1) as i32).unsigned_abs() as usize
}

fn move_ampiphod_out_of_slot(
    mut state: State,
    old_state: State,
    coord: &Coord,
    tile: &Tile,
    number_of_moves: usize,
    total_energy_cost: &mut usize,
) {
    // move the current letter out
    state.map.insert(*coord, Tile::Empty);
    let mut second_state = state.clone();

    // move up and then left
    let up_left_coord = (coord.0 - 1, coord.1 - (number_of_moves - 1));
    if *state.map.get(&up_left_coord).unwrap() == Tile::Empty {
        state.energy += tile.get_energy_cost() * number_of_moves;
        state.map.insert(up_left_coord, tile.clone());
        *total_energy_cost = (*total_energy_cost).min(next_moves(old_state.clone(), state));
    }

    // move up and then right
    let up_right_coord = (coord.0 + 1, coord.1 - (number_of_moves - 1));
    if *second_state.map.get(&up_right_coord).unwrap() == Tile::Empty {
        second_state.energy += tile.get_energy_cost() * number_of_moves;
        second_state.map.insert(up_right_coord, tile.clone());
        *total_energy_cost = (*total_energy_cost).min(next_moves(old_state, second_state));
    }
}

fn next_moves(old_state: State, state: State) -> usize {
    // println!("old {}", old_state);
    // println!("new {}", state);

    if state.locations_solved.len() == 4 {
        return state.energy;
    }

    let old_state = state.clone();

    let mut energy_cost = usize::MAX;

    for (coord, tile) in state.get_nonsolved_tiles() {
        let target_coords = tile.get_target_coords();
        // letter is in the bottom of desired spot
        if *coord == target_coords[1] {
            // if letter in top spot also matches, mark this one as complete
            if state.map.get(&target_coords[0]).unwrap() == tile {
                let mut state = state.clone();
                state.locations_solved.insert(tile.clone());
                next_moves(old_state.clone(), state);
            }
            continue;
        } else if (*coord == target_coords[0] && state.map.get(&target_coords[1]).unwrap() != tile)
            || coord.1 == 2
            || coord.1 == 3
        {
            // letter is right spot but needs to move out so the letter below it can move
            // OR its in the wrong slot and needs to move out
            for to_coord in &state.get_possible_hallway_tiles(coord) {
                let mut state = state.clone();
                state.map.insert(*coord, Tile::Empty);
                state.map.insert(*to_coord, tile.clone());
                state.energy += tile.get_energy_cost() * manhatten_distance(coord, to_coord);
                energy_cost = energy_cost.min(next_moves(old_state.clone(), state));
            }
        } else if coord.1 == 1 {
            // y coord is 1, so is out of slot
            let first_tile = state.map.get(&target_coords[0]).unwrap();
            let second_tile = state.map.get(&target_coords[1]).unwrap();
            let can_move_into_bottom_spot =
                *first_tile == Tile::Empty && *second_tile == Tile::Empty;
            let can_move_into_top_spot = *first_tile == Tile::Empty && *second_tile == *tile;

            // because of the first two boolean checks, we can just use the first target coord as the endpoint safely
            if (can_move_into_bottom_spot || can_move_into_top_spot)
                && state.path_is_clear(coord, &target_coords[0])
            {
                // both empty, can move into bottom spot
                if can_move_into_bottom_spot {
                    let mut state = state.clone();
                    state.map.insert(*coord, Tile::Empty);
                    state.energy +=
                        tile.get_energy_cost() * manhatten_distance(coord, &target_coords[1]);
                    state.map.insert(target_coords[1], tile.clone());
                    // println!("Moving into bottom {} {}", tile, state);
                    energy_cost = energy_cost.min(next_moves(old_state.clone(), state));
                } else if can_move_into_top_spot {
                    // top spot is empty, and bottom spot has the same tile value as our current letter
                    let mut state = state.clone();
                    state.map.insert(*coord, Tile::Empty);
                    state.energy +=
                        tile.get_energy_cost() * manhatten_distance(coord, &target_coords[0]);
                    state.locations_solved.insert(tile.clone());
                    state.map.insert(target_coords[0], tile.clone());
                    // println!("Moving into top {} {}", tile, state);
                    energy_cost = energy_cost.min(next_moves(old_state.clone(), state));
                }
            }
        }
    }

    energy_cost
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

            map.insert((c, r), ch.into());
            width = width.max(c);
        }
        height += 1;
    }

    let state = State::new(map, width, height);
    println!("{}", next_moves(state.clone(), state));

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
    }
}
