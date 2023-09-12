use std::collections::HashMap;
use std::fmt::Display;
use std::io::Result;

use read_input::read_text;

type Map = HashMap<(usize, usize), Location>;

enum Location {
    Empty,
    A,
    B,
    C,
    D,
}

impl From<char> for Location {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Location::Empty,
            'A' => Location::A,
            'B' => Location::B,
            'C' => Location::C,
            'D' => Location::D,
            _ => panic!("unrecognized character {}", ch),
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Location::A => "A",
                Location::B => "B",
                Location::C => "D",
                Location::D => "E",
                Location::Empty => ".",
            }
        )
    }
}

fn draw_map(map: &Map, width: usize, height: usize) {
    for r in 0..=height {
        for c in 0..=width {
            if let Some(location) = map.get(&(c, r)) {
                print!("{}", location);
            } else {
                print!(" ");
            }
        }
        println!();
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

    draw_map(&map, width, height);

    Ok(())
}
