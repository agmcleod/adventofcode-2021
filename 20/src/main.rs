use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("20/input.txt")?;

    for line in text.lines() {}

    Ok(())
}
