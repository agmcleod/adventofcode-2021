use std::io::Result;

use read_input::read_text;

fn p1(text: &String) -> i32 {
    let mut depth = 0;
    let mut horizontal_position = 0;
    for line in text.lines() {
        let mut words = line.split(" ");
        let cmd = words.next().unwrap();
        match cmd {
            "forward" => {
                let v: i32 = words.next().unwrap().parse().unwrap();
                horizontal_position += v;
            }
            "up" => {
                let v: i32 = words.next().unwrap().parse().unwrap();
                depth -= v;
            }
            "down" => {
                let v: i32 = words.next().unwrap().parse().unwrap();
                depth += v;
            }
            _ => panic!("Unrecognized command {}", cmd),
        }
    }

    horizontal_position * depth
}

fn p2(text: &String) -> i32 {
    let mut depth = 0;
    let mut horizontal_position = 0;
    let mut aim = 0;
    for line in text.lines() {
        let mut words = line.split(" ");
        let cmd = words.next().unwrap();
        match cmd {
            "forward" => {
                let v: i32 = words.next().unwrap().parse().unwrap();
                horizontal_position += v;
                depth += aim * v;
            }
            "up" => {
                let v: i32 = words.next().unwrap().parse().unwrap();
                aim -= v;
            }
            "down" => {
                let v: i32 = words.next().unwrap().parse().unwrap();
                aim += v;
            }
            _ => panic!("Unrecognized command {}", cmd),
        }
    }

    horizontal_position * depth
}

fn main() -> Result<()> {
    let text = read_text("2/input.txt")?;

    println!("{}", p1(&text));
    println!("{}", p2(&text));

    Ok(())
}
