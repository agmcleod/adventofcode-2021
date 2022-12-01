use std::io::Result;

use bitflags::bitflags;
use read_input::read_text;

bitflags! {
    struct Flags: u16 {
        const A = 1;
        const B = 2;
        const C = 4;
        const D = 8;
        const E = 16;
        const F = 32;
        const G = 64;
    }
}

fn main() -> Result<()> {
    let text = read_text("8/input.txt")?;

    for line in text.lines() {
        let mut parts = line.split(" | ");
        let signal_patterns = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
        let output_value = parts.next().unwrap().split(" ").collect::<Vec<&str>>();
    }

    Ok(())
}
