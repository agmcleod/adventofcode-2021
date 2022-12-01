use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("8/input.txt")?;

    let mut segment_display_map = HashMap::new();
    segment_display_map.insert(0, HashSet::from(["a", "b", "c", "e", "f", "g"]));
    segment_display_map.insert(1, HashSet::from(["c", "f"]));
    segment_display_map.insert(2, HashSet::from(["a", "c", "d", "e", "g"]));
    segment_display_map.insert(3, HashSet::from(["a", "c", "d", "f", "g"]));
    segment_display_map.insert(4, HashSet::from(["b", "c", "d", "f"]));
    segment_display_map.insert(5, HashSet::from(["a", "b", "d", "f", "g"]));
    segment_display_map.insert(6, HashSet::from(["a", "b", "d", "e", "f", "g"]));
    segment_display_map.insert(7, HashSet::from(["a", "c", "f"]));
    segment_display_map.insert(8, HashSet::from(["a", "b", "c", "d", "e", "f", "g"]));
    segment_display_map.insert(9, HashSet::from(["a", "b", "c", "d", "f", "g"]));

    for line in text.lines() {
        let mut parts = line.split(" | ");
        let signal_patterns = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        let output_value = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
    }

    Ok(())
}
