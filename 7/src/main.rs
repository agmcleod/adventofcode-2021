use std::cmp;
use std::io::Result;

use read_input::read_text;

fn calc_fuel(positions: &Vec<i32>, target: i32) -> i32 {
    positions.iter().fold(0, |sum, n| sum + (*n - target).abs())
}

fn sum_steps(start: i32, end: i32) -> i32 {
    (1..=(start - end).abs()).fold(0, |sum, n| sum + n)
}

fn calc_fuel_p2(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .fold(0, |sum, n| sum + sum_steps(target, *n))
}

fn main() -> Result<()> {
    let text = read_text("7/input.txt")?;

    let positions: Vec<i32> = text.split(",").map(|n| n.parse::<i32>().unwrap()).collect();

    let avg = positions.iter().fold(0, |sum, n| sum + *n as i32) / positions.len() as i32;

    let mut lowest_fuel = i32::MAX;
    let count = 800;
    let mut start_count = -count / 2;
    if avg + start_count < 0 {
        start_count = 0;
    }
    for i in start_count..count {
        let fuel = calc_fuel(&positions, i);
        lowest_fuel = cmp::min(lowest_fuel, fuel);
    }
    println!("{}", lowest_fuel);

    lowest_fuel = i32::MAX;
    for i in start_count..count {
        let fuel = calc_fuel_p2(&positions, i);
        lowest_fuel = cmp::min(lowest_fuel, fuel);
    }

    println!("{}", lowest_fuel);

    Ok(())
}
