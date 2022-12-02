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

    let mut p1_sum = 0;

    for line in text.lines() {
        let mut parts = line.split(" | ");
        let _signal_patterns = parts.next().unwrap().split(" ").collect::<Vec<&str>>();

        let output_patterns = parts.next().unwrap().split(" ").collect::<Vec<&str>>();

        for pattern in &output_patterns {
            if pattern.len() == 2 {
                // it's a 1
                p1_sum += 1;
            } else if pattern.len() == 3 {
                // a 7
                p1_sum += 1;
            } else if pattern.len() == 4 {
                // a 4
                p1_sum += 1;
            } else if pattern.len() == 7 {
                // an 8
                p1_sum += 1;
            }
        }
    }

    println!("{}", p1_sum);

    Ok(())
}
