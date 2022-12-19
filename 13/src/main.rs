use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

fn add_coord_to_map(map: &mut HashMap<usize, HashSet<usize>>, key: usize, value: usize) {
    if map.contains_key(&key) {
        map.get_mut(&key).unwrap().insert(value);
    } else {
        let mut set = HashSet::new();
        set.insert(value);
        map.insert(key, set);
    }
}

fn main() -> Result<()> {
    let text = read_text("13/input.txt")?;

    let mut columns: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut rows: HashMap<usize, HashSet<usize>> = HashMap::new();
    for line in text.lines() {
        if line.contains("fold along") {
            break;
        } else {
            let mut iter = line.split(",");
            let x: usize = iter.next().unwrap().parse().unwrap();
            let y: usize = iter.next().unwrap().parse().unwrap();

            add_coord_to_map(&mut columns, x, y);
            add_coord_to_map(&mut rows, y, x);
        }
    }

    Ok(())
}
