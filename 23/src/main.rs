use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Result;

use read_input::read_text;

type Map = HashMap<(usize, usize), Tile>;

#[derive(Clone, Eq, Hash, PartialEq)]
enum Tile {
    Empty,
    A,
    B,
    C,
    D,
}

impl Tile {
    fn get_target_coords(&self) -> [(usize, usize); 2] {
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

    fn get_nonsolved_tiles(&self) -> impl Iterator<Item = (&(usize, usize), &Tile)> {
        self.map
            .iter()
            .filter(|(_, tile)| **tile != Tile::Empty && !self.locations_solved.contains(*tile))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

fn manhatten_distance(coord1: &(usize, usize), coord2: &(usize, usize)) -> usize {
    ((coord1.0 + coord1.1) as i32 - (coord2.0 + coord2.1) as i32).unsigned_abs() as usize
}

fn move_ampiphod_out_of_slot(
    mut state: State,
    coord: &(usize, usize),
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
        state.map.insert((coord.0 - 1, coord.1 - 1), tile.clone());
        *total_energy_cost = (*total_energy_cost).min(next_moves(state));
    }

    // move up and then right
    let up_right_coord = (coord.0 + 1, coord.1 - (number_of_moves - 1));
    if *second_state.map.get(&up_right_coord).unwrap() == Tile::Empty {
        second_state.energy += tile.get_energy_cost() * number_of_moves;
        second_state.map.insert(up_right_coord, tile.clone());
        *total_energy_cost = (*total_energy_cost).min(next_moves(second_state));
    }
}

fn next_moves(state: State) -> usize {
    // loop through each letter
    // if letter is in desired spot
    //   is there a letter below it not in desired spot?
    //     move letter above it out of spot
    //   else
    //     leave as is
    // elseif is letter in a slot
    //   move letter out of slot (if space available)
    // elseif is letter out of slot
    //   can it move into slot?
    //     move into slot
    //   else
    //     do nothing

    if state.locations_solved.len() == 4 {
        return state.energy;
    }

    let mut energy_cost = usize::MAX;

    for (coord, tile) in state.get_nonsolved_tiles() {
        let target_coords = tile.get_target_coords();
        // letter is in the bottom of desired spot
        if *coord == target_coords[1] {
            // if letter in top spot also matches, mark this one as complete
            if state.map.get(&target_coords[0]).unwrap() == tile {
                let mut state = state.clone();
                state.locations_solved.insert(tile.clone());
                next_moves(state);
            }
            continue;
        } else if (*coord == target_coords[0] && state.map.get(&target_coords[1]).unwrap() != tile)
            || coord.1 == 2
        {
            // letter is in desired top spot, but has a different letter below, or it is in top spot and needs to be elsewhere
            move_ampiphod_out_of_slot(state.clone(), coord, tile, 2, &mut energy_cost);
        } else if coord.1 == 3 {
            if *state.map.get(&(coord.0, coord.1 - 1)).unwrap() == Tile::Empty {
                move_ampiphod_out_of_slot(state.clone(), coord, tile, 3, &mut energy_cost);
            }
        } else if coord.1 == 1 {
            // y coord is 1, so is out of slot
            let first_tile = state.map.get(&target_coords[1]).unwrap();
            let second_tile = state.map.get(&target_coords[1]).unwrap();
            let can_move_into_bottom_spot =
                *first_tile == Tile::Empty && *second_tile == Tile::Empty;
            let can_move_into_top_spot = *first_tile == Tile::Empty && *second_tile == *tile;

            let mut ampiphods_in_the_way = false;
            if can_move_into_bottom_spot || can_move_into_top_spot {
                let min_x = coord.0.min(target_coords[0].0);
                let max_x = coord.0.max(target_coords[0].0);

                for x in (min_x + 1)..max_x {
                    if *state.map.get(&(x, coord.1)).unwrap() != Tile::Empty {
                        ampiphods_in_the_way = true;
                    }
                }
            }

            if !ampiphods_in_the_way {
                // both empty, can move into bottom spot
                if can_move_into_bottom_spot {
                    let mut state = state.clone();
                    state.map.insert(*coord, Tile::Empty);
                    state.energy +=
                        tile.get_energy_cost() * manhatten_distance(coord, &target_coords[1]);
                    state.map.insert(target_coords[1], tile.clone());
                    // println!("Moving into bottom {} {}", tile, state);
                    energy_cost = energy_cost.min(next_moves(state));
                } else if can_move_into_top_spot {
                    // top spot is empty, and bottom spot has the same tile value as our current letter
                    let mut state = state.clone();
                    state.map.insert(*coord, Tile::Empty);
                    state.energy +=
                        tile.get_energy_cost() * manhatten_distance(coord, &target_coords[0]);
                    state.locations_solved.insert(tile.clone());
                    state.map.insert(target_coords[0], tile.clone());
                    // println!("Moving into top {} {}", tile, state);
                    energy_cost = energy_cost.min(next_moves(state));
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
    println!("{}", next_moves(state));

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
