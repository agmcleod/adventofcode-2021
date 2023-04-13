use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("19/input.txt")?;

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
    }

    Ok(())
}
