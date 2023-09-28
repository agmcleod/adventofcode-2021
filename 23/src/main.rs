use std::collections::{hash_map, HashMap, HashSet};
use std::fmt::Display;
use std::io::Result;
use std::iter;

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
                Tile::C => "D",
                Tile::D => "E",
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
            .filter(|(_, tile)| **tile != Tile::Empty && self.locations_solved.contains(*tile))
    }
}

fn manhatten_distance(coord1: &(usize, usize), coord2: &(usize, usize)) -> usize {
    coord1.0 + coord1.1 + coord2.0 + coord2.1
}

fn move_ampiphod_out_of_slot(
    mut state: State,
    coord: &(usize, usize),
    tile: &Tile,
    number_of_moves: usize,
    total_energy_cost: &mut usize,
) {
    state.energy += tile.get_energy_cost() * number_of_moves;
    // move the current letter out
    state.map.insert(*coord, Tile::Empty);
    let mut second_state = state.clone();

    // move up and then left
    let up_left_coord = (coord.0 - 1, coord.1 - 1);
    if *state.map.get(&up_left_coord).unwrap() == Tile::Empty {
        state.map.insert((coord.0 - 1, coord.1 - 1), tile.clone());
        *total_energy_cost = (*total_energy_cost).min(next_moves(state));
    }

    // move up and then right
    let up_right_coord = (coord.0 + 1, coord.1 - 1);
    if *second_state.map.get(&up_right_coord).unwrap() == Tile::Empty {
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
            continue;
        } else if (*coord == target_coords[0] && state.map.get(&target_coords[0]).unwrap() != tile)
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
            // TODO: check if there's other letters on coord.1 in the way
            // both empty, can move into bottom spot
            if *first_tile == Tile::Empty && *second_tile == Tile::Empty {
                let mut state = state.clone();
                state.map.insert(*coord, Tile::Empty);
                state.energy +=
                    tile.get_energy_cost() * manhatten_distance(coord, &target_coords[1]);
                energy_cost = energy_cost.min(next_moves(state));
            } else if *first_tile == Tile::Empty && *second_tile == *tile {
                // top spot is empty, and bottom spot has the same tile value as our current letter
                let mut state = state.clone();
                state.map.insert(*coord, Tile::Empty);
                state.energy +=
                    tile.get_energy_cost() * manhatten_distance(coord, &target_coords[0]);
                state.locations_solved.insert(tile.clone());
                energy_cost = energy_cost.min(next_moves(state));
            }
        }
    }

    energy_cost
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
