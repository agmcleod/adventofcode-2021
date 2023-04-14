use std::io::Result;

use read_input::read_text;

type Coord = (i32, i32, i32);

struct Scanner {
    coords: Vec<Coord>,
}

impl Scanner {
    fn new() -> Self {
        Scanner { coords: Vec::new() }
    }
}

fn main() -> Result<()> {
    let text = read_text("19/input.txt")?;

    let mut scanners = Vec::new();
    let mut scanner = None;

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("---") {
            if scanner.is_some() {
                scanners.push(scanner.unwrap());
            }

            scanner = Some(Scanner::new());
        } else {
            let coords = line
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>();

            if coords.len() != 3 {
                panic!("Invalid coords line {}", line);
            }

            let scanner = scanner.as_mut().unwrap();
            scanner.coords.push((coords[0], coords[1], coords[2]));
        }
    }

    Ok(())
}
