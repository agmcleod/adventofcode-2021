use std::io::Result;

use read_input::read_text;

fn main() -> Result<()> {
    let text = read_text("1/input.txt")?;

    let mut last_value = None;
    let mut increases = 0;
    let mut windows = (0, 0, 0);
    for (i, line) in text.lines().enumerate() {
        let n: usize = line
            .parse()
            .map_err(|err| {
                panic!("Could not parse number: {}", line);
            })
            .unwrap();
        if last_value.is_some() && last_value.unwrap() < n {
            increases += 1;
        }

        if i % 3 == 0 {}

        last_value = Some(n);
    }

    println!("p1 {}", increases);

    Ok(())
}
