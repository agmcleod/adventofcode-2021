use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("18/input.txt")?;

    for line in text.lines() {
        let mut characters = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        loop {
            let mut this_iteration = characters.clone();

            let mut depth = 0;
            let mut pair = (0, 0);
            for (i, ch) in characters.iter().enumerate() {
                if ch == "[" {
                    depth += 1;
                    pair = (0, 0);
                } else if ch == "]" {
                    depth -= 1;
                    pair = (0, 0);
                } else if ch != "," {
                    if pair.0 == 0 {
                        pair.0 = ch.parse().unwrap();
                    } else {
                        pair.1 = ch.parse().unwrap();
                    }
                }
            }

            characters = this_iteration;
        }
    }

    Ok(())
}
