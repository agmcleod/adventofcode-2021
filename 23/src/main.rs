use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Result;

use read_input::read_text;

type Map = HashMap<(usize, usize), Tile>;

#[derive(Clone, PartialEq)]
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
    height: usize,
    width: usize,
    map: Map,
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

    fn is_target_room_available(&self, tile: &Tile) -> bool {
        let coords = tile.get_target_coords();
        let top_spot = self.map.get(&coords[0]).unwrap();
        if *top_spot != Tile::Empty {
            return false;
        }

        let bottom_spot = self.map.get(&coords[1]).unwrap();
        *bottom_spot != Tile::Empty
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
    println!("{}", state);

    Ok(())
}
