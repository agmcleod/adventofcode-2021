use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("6/input.txt")?;

    let mut lantern_fish: Vec<i32> = text
        .split(",")
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    for _ in 0..256 {
        let mut next_to_spawn = Vec::new();

        for fish in &mut lantern_fish {
            if *fish == 0 {
                *fish = 6;
                next_to_spawn.push(8);
            } else {
                *fish -= 1;
            }
        }

        if next_to_spawn.len() > 0 {
            lantern_fish.append(&mut next_to_spawn);
        }
    }

    println!("{}", lantern_fish.len());

    Ok(())
}
